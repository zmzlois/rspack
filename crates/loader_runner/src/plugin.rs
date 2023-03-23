use rspack_error::Result;

use crate::{Content, LoaderContext};

#[async_trait::async_trait]
pub trait LoaderRunnerPlugin<T, U>: Send + Sync {
  fn name(&self) -> &'static str {
    "unknown"
  }

  async fn process_resource(
    &self,
    loader_context: &mut LoaderContext<'_, '_, T, U>,
  ) -> Result<Option<Content>>;
}
