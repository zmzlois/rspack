use rspack_error::Result;
use rspack_loader_runner::{Content, LoaderContext, LoaderRunnerPlugin, ResourceData};

use crate::SharedPluginDriver;

pub struct LoaderRunnerPluginProcessResource<T, U> {
  plugin_driver: SharedPluginDriver<T, U>,
}

impl<T, U> LoaderRunnerPluginProcessResource<T, U> {
  pub fn new(plugin_driver: SharedPluginDriver<T, U>) -> Self {
    Self { plugin_driver }
  }
}

#[async_trait::async_trait]
impl<T, U> LoaderRunnerPlugin<T, U> for LoaderRunnerPluginProcessResource<T, U> {
  fn name(&self) -> &'static str {
    "process-resource"
  }

  async fn process_resource(
    &self,
    context: &mut LoaderContext<'_, '_, T, U>,
  ) -> Result<Option<Content>> {
    let result = self
      .plugin_driver
      .read()
      .await
      .read_resource(resource_data)
      .await?;
    if result.is_some() {
      return Ok(result);
    }

    Ok(None)
  }
}
