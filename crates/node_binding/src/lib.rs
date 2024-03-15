#![recursion_limit = "256"]
#![feature(let_chains)]
#![feature(try_blocks)]
#[macro_use]
extern crate napi_derive;
extern crate rspack_allocator;

use std::sync::atomic::AtomicI32;
use std::sync::{Arc, Mutex};
use std::{pin::Pin, sync::atomic::AtomicU32};

use compiler::{Compiler, CompilerState, CompilerStateGuard};
use napi::bindgen_prelude::*;
use rspack_binding_options::BuiltinPlugin;
use rspack_core::PluginExt;
use rspack_error::Diagnostic;
use rspack_fs_node::{AsyncNodeWritableFileSystem, ThreadsafeNodeFS};

mod compiler;
mod hook;
mod loader;
mod panic;
mod plugins;

use hook::*;
pub use loader::run_builtin_loader;
use plugins::*;
use rspack_binding_options::*;
use rspack_binding_values::*;
use rspack_tracing::chrome::FlushGuard;

#[napi(custom_finalize)]
pub struct Rspack {
  id: u32,
  js_plugin: JsHooksAdapterPlugin,
  compiler: Pin<Box<Compiler>>,
  finalize_callbacks: *mut dyn FnOnce(),
  ref_count: Arc<AtomicI32>,
  state: CompilerState,
}

static COMPILER_ID: AtomicU32 = AtomicU32::new(1);

impl ObjectFinalize for Rspack {
  fn finalize(mut self, _env: Env) -> Result<()> {
    // println!(
    //   "finalizing compiler {id} with ref count {count} before",
    //   id = self.id,
    //   count = self.ref_count.load(std::sync::atomic::Ordering::Relaxed)
    // );
    // let finalize_callbacks = unsafe { Box::from_raw(self.finalize_callbacks) };
    // finalize_callbacks();
    // println!(
    //   "finalizing compiler {id} with ref count {count} after",
    //   id = self.id,
    //   count = self.ref_count.load(std::sync::atomic::Ordering::Relaxed)
    // );
    Ok(())
  }
}

impl Drop for Rspack {
  fn drop(&mut self) {
    println!(
      "dropping compiler {id} with ref count {count} before",
      id = self.id,
      count = self.ref_count.load(std::sync::atomic::Ordering::Relaxed)
    );
    let finalize_callbacks = unsafe { Box::from_raw(self.finalize_callbacks) };
    finalize_callbacks();
    println!(
      "dropping compiler {id} with ref count {count} after",
      id = self.id,
      count = self.ref_count.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!("dropping {}", self.id);
  }
}

#[napi]
impl Rspack {
  #[napi(constructor)]
  pub fn new(
    env: Env,
    options: RawOptions,
    builtin_plugins: Vec<BuiltinPlugin>,
    js_hooks: JsHooks,
    register_js_taps: RegisterJsTaps,
    output_filesystem: ThreadsafeNodeFS,
  ) -> Result<Self> {
    tracing::info!("raw_options: {:#?}", &options);

    let disabled_hooks: DisabledHooks = Default::default();
    let mut plugins = Vec::new();
    let js_plugin =
      JsHooksAdapterPlugin::from_js_hooks(env, js_hooks, disabled_hooks, register_js_taps)?;
    plugins.push(js_plugin.clone().boxed());
    for bp in builtin_plugins {
      bp.append_to(&mut plugins)
        .map_err(|e| Error::from_reason(format!("{e}")))?;
    }

    let compiler_options = options
      .apply(&mut plugins)
      .map_err(|e| Error::from_reason(format!("{e}")))?;

    tracing::info!("normalized_options: {:#?}", &compiler_options);

    let rspack = rspack_core::Compiler::new(
      compiler_options,
      plugins,
      AsyncNodeWritableFileSystem::new(output_filesystem)
        .map_err(|e| Error::from_reason(format!("Failed to create writable filesystem: {e}",)))?,
    );

    Ok(Self {
      id: COMPILER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
      compiler: Box::pin(Compiler::from(rspack)),
      state: CompilerState::init(),
      finalize_callbacks: Box::into_raw(Box::new(|| {})),
      ref_count: Arc::new(AtomicI32::new(1)),
      js_plugin,
    })
  }

  #[napi]
  pub fn set_disabled_hooks(&self, hooks: Vec<String>) {
    self.js_plugin.set_disabled_hooks(hooks)
  }

  /// Build with the given option passed to the constructor
  #[napi(ts_args_type = "callback: (err: null | Error) => void")]
  pub fn build(&mut self, env: Env, reference: Reference<Rspack>, f: JsFunction) -> Result<()> {
    unsafe {
      self.run(env, reference, |compiler, _guard| {
        callbackify(env, f, async move {
          compiler.build().await.map_err(|e| {
            Error::new(
              napi::Status::GenericFailure,
              print_error_diagnostic(e, compiler.options.stats.colors),
            )
          })?;
          tracing::info!("build ok");
          drop(_guard);
          Ok(())
        })
      })
    }
  }

  /// Rebuild with the given option passed to the constructor
  #[napi(
    ts_args_type = "changed_files: string[], removed_files: string[], callback: (err: null | Error) => void"
  )]
  pub fn rebuild(
    &mut self,
    env: Env,
    reference: Reference<Rspack>,
    changed_files: Vec<String>,
    removed_files: Vec<String>,
    f: JsFunction,
  ) -> Result<()> {
    use std::collections::HashSet;

    unsafe {
      self.run(env, reference, |compiler, _guard| {
        callbackify(env, f, async move {
          compiler
            .rebuild(
              HashSet::from_iter(changed_files.into_iter()),
              HashSet::from_iter(removed_files.into_iter()),
            )
            .await
            .map_err(|e| {
              Error::new(
                napi::Status::GenericFailure,
                print_error_diagnostic(e, compiler.options.stats.colors),
              )
            })?;
          tracing::info!("rebuild ok");
          drop(_guard);
          Ok(())
        })
      })
    }
  }
}

impl Rspack {
  /// Run the given function with the compiler.
  ///
  /// ## Safety
  /// 1. The caller must ensure that the `Compiler` is not moved or dropped during the lifetime of the function.
  /// 2. `CompilerStateGuard` should only be dropped so soon as each `build` or `rebuild` session is finished.
  ///    Otherwise, this would lead to potential race condition for `Compiler`, especially when `build` or `rebuild`
  ///    was called on JS side and its previous `build` or `rebuild` was yet to finish.
  unsafe fn run<R>(
    &mut self,
    env: Env,
    reference: Reference<Rspack>,
    f: impl FnOnce(&'static mut Compiler, CompilerStateGuard) -> Result<R>,
  ) -> Result<R> {
    if self.state.running() {
      return Err(concurrent_compiler_error());
    }
    let _guard = self.state.enter();

    // Leak the `SharedReference`.
    // `SharedReference` was originally designed to be leaked if `Deref` or `DerefMut` is called.
    // This means the original `SharedReference` will not likely to be dropped. The wrapped native
    // instance is also not to be dropped until the `Reference` is dropped, which requires the
    // `SharedReference` to be dropped, which contains the `Reference`.
    // See [`napi::bindgen_prelude::Reference`]
    let compiler_ref = Box::leak(Box::new(
      reference.share_with(env, |s| Ok(&mut s.compiler))?,
    ));
    let ref_ptr = compiler_ref as *mut _;
    self
      .ref_count
      .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    let rc = self.ref_count.clone();
    let id = self.id;

    let prev_drop_fn = unsafe { Box::from_raw(self.finalize_callbacks) };
    let drop_fn = Box::new(move || {
      let r = rc.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
      println!("decreasing compiler {id} by 1, current {}", r - 1);
      drop(unsafe { Box::from_raw(ref_ptr) });
      prev_drop_fn();
    });
    self.finalize_callbacks = Box::into_raw(drop_fn);

    // let mut compiler_ref = reference.share_with(env, |s| Ok(&mut s.compiler))?;

    f(
      unsafe {
        // SAFETY:
        // 1. The mutable reference to `Compiler` is exclusive. It's guaranteed by the running state guard.
        // 2. `Compiler` will not be moved, as it's stored on the heap.
        // 3. `Compiler` is valid through the lifetime before it's closed by calling `Compiler.close()` or gc-ed.
        // 4. Caution: This **does** not guarantee that `Compiler` will not be moved in other crates.
        std::mem::transmute::<&mut Compiler, &'static mut Compiler>(
          compiler_ref.as_mut().get_unchecked_mut(),
        )
      },
      _guard,
    )
  }
}

fn concurrent_compiler_error() -> Error {
  Error::new(
    napi::Status::GenericFailure,
    "ConcurrentCompilationError: You ran rspack twice. Each instance only supports a single concurrent compilation at a time.",
  )
}

#[derive(Default)]
enum TraceState {
  On(Option<FlushGuard>),
  #[default]
  Off,
}

#[ctor]
fn init() {
  panic::install_panic_handler();
}

fn print_error_diagnostic(e: rspack_error::Error, colored: bool) -> String {
  Diagnostic::from(e)
    .render_report(colored)
    .expect("should print diagnostics")
}

static GLOBAL_TRACE_STATE: Mutex<TraceState> = Mutex::new(TraceState::Off);

/**
 * Some code is modified based on
 * https://github.com/swc-project/swc/blob/d1d0607158ab40463d1b123fed52cc526eba8385/bindings/binding_core_node/src/util.rs#L29-L58
 * Apache-2.0 licensed
 * Author Donny/강동윤
 * Copyright (c)
 */
#[napi]
pub fn register_global_trace(
  filter: String,
  #[napi(ts_arg_type = "\"chrome\" | \"logger\"")] layer: String,
  output: String,
) {
  let mut state = GLOBAL_TRACE_STATE
    .lock()
    .expect("Failed to lock GLOBAL_TRACE_STATE");
  if matches!(&*state, TraceState::Off) {
    let guard = match layer.as_str() {
      "chrome" => rspack_tracing::enable_tracing_by_env_with_chrome_layer(&filter, &output),
      "logger" => {
        rspack_tracing::enable_tracing_by_env(&filter, &output);
        None
      }
      _ => panic!("not supported layer type:{layer}"),
    };
    let new_state = TraceState::On(guard);
    *state = new_state;
  }
}

#[napi]
pub fn cleanup_global_trace() {
  let mut state = GLOBAL_TRACE_STATE
    .lock()
    .expect("Failed to lock GLOBAL_TRACE_STATE");
  if let TraceState::On(guard) = &mut *state
    && let Some(g) = guard.take()
  {
    g.flush();
    drop(g);
    let new_state = TraceState::Off;
    *state = new_state;
  }
}
