use rspack_core::LoaderRunnerContext;
use rspack_error::Result;
use rspack_loader_runner::{Identifiable, Identifier, Loader, LoaderContext};

#[derive(Debug, Default)]
pub struct MiniCssLoader {}

pub static MINI_CSS_EXTRACT_LOADER: &str = "builtin:mini-css-extract-rspack-plugin";

impl Identifiable for MiniCssLoader {
  fn identifier(&self) -> Identifier {
    Identifier::from(MINI_CSS_EXTRACT_LOADER)
  }
}

#[async_trait::async_trait]
impl Loader<LoaderRunnerContext> for MiniCssLoader {
  async fn run(&self, loader_context: &mut LoaderContext<'_, LoaderRunnerContext>) -> Result<()> {
    let original_module = loader_context.context.module;
    let original_module_context = loader_context.context.module_context.clone();
    let res = loader_context
      .context
      .import_module(
        "/Users/bytedance/libs/rspack/examples/basic/src/answer.js".into(),
        "".into(),
        original_module,
        original_module_context,
      )
      .await;

    dbg!(res);

    Ok(())
  }
}
