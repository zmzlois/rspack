use std::sync::Arc;

use rspack_error::{Error, Result};
pub use rspack_loader_runner::{run_loaders, Content, Loader, LoaderContext};
use tokio::sync::mpsc::unbounded_channel;

use crate::{CompilerOptions, Context, ModuleIdentifier, ResolverFactory, TaskResult, TaskSender};

#[derive(Debug, Clone)]
pub struct CompilerContext {
  pub options: Arc<CompilerOptions>,
  pub resolver_factory: Arc<ResolverFactory>,
  pub module: Option<ModuleIdentifier>,     // current module
  pub module_context: Option<Box<Context>>, // current module context
  pub task_sender: Option<TaskSender>,
}

impl CompilerContext {
  pub async fn import_module(
    &self,
    request: String,
    public_path: String,
    original_module: Option<ModuleIdentifier>,
    original_module_context: Option<Box<Context>>,
  ) -> Result<String> {
    let (tx, mut rx) = unbounded_channel();
    self
      .task_sender
      .as_ref()
      .unwrap()
      .send(Ok(TaskResult::ImportModule(Box::new(
        crate::ImportModuleResult {
          request,
          sender: tx,
          original_module,
          original_module_context,
          options: crate::ImportModuleOption { public_path },
        },
      ))))
      .unwrap();

    let res = rx.recv().await;

    match res {
      Some(Ok(res)) => Ok(res),
      Some(Err(e)) => Err(e),
      None => Err(Error::InternalError(rspack_error::InternalError::new(
        "Failed to call importModule".into(),
        rspack_error::Severity::Error,
      ))),
    }
  }
}

pub type LoaderRunnerContext = CompilerContext;

pub type BoxLoader = Arc<dyn Loader<LoaderRunnerContext>>;
