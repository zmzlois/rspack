use std::cell::RefCell;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use napi::bindgen_prelude::{Either3, FromNapiValue, Null, Undefined, Unknown};
use napi::{sys, JsUndefined};
use napi_derive::napi;
use rspack_binding_values::{JsModule, ToJsModule};
use rspack_napi::regexp::{JsRegExp, JsRegExpExt};
use rspack_napi::threadsafe_function::ThreadsafeFunction;
use rspack_napi::NapiTypeRef;
use rspack_plugin_split_chunks::{CacheGroupTest, CacheGroupTestFnCtx};
use tokio::runtime::Handle;

thread_local! {
  pub static NAPI_ENV: RefCell<Option<sys::napi_env>> = RefCell::new(None);
}

pub(super) type RawCacheGroupTest =
  Either3<String, Unknown, ThreadsafeFunction<RawCacheGroupTestCtx, Option<bool>>>;

#[napi(object)]
pub struct RawCacheGroupTestCtx {
  pub module: JsModule,
}

impl<'a> From<CacheGroupTestFnCtx<'a>> for RawCacheGroupTestCtx {
  fn from(value: CacheGroupTestFnCtx<'a>) -> Self {
    RawCacheGroupTestCtx {
      module: value
        .module
        .to_js_module()
        .expect("should convert js module success"),
    }
  }
}

pub static COUNT: AtomicUsize = AtomicUsize::new(0);

pub(super) fn normalize_raw_cache_group_test(raw: RawCacheGroupTest) -> CacheGroupTest {
  let handle = Handle::current();
  match raw {
    Either3::A(str) => CacheGroupTest::String(str),
    Either3::B(regexp) => {
      JsRegExp::from_unknown(regexp).expect("");

      // CacheGroupTest::RegExp(regexp.to_rspack_regex())
      // let mut t = 0;
      // let r = unsafe { sys::napi_typeof(regexp.0.env, regexp.0.value, &mut t) };
      // assert_eq!(r, 0);
      // let a = NapiTypeRef::from_unknown(regexp).expect("");
      // let b = a.get_type();
      // println!(
      //   "type {b}, count {}, typeof {t:?}",
      //   COUNT.load(std::sync::atomic::Ordering::Relaxed),
      // );
      unreachable!()
    }
    Either3::C(v) => CacheGroupTest::Fn(Arc::new(move |ctx| {
      handle
        .block_on(v.call(ctx.into()))
        .expect("failed to load cache group test")
    })),
  }
}

#[inline]
pub(super) fn default_cache_group_test() -> CacheGroupTest {
  CacheGroupTest::Enabled
}
