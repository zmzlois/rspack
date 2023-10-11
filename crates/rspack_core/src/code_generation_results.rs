use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use anymap::CloneAny;
use rspack_error::{internal_error, Result};
use rspack_hash::{HashDigest, HashFunction, HashSalt, RspackHash, RspackHashDigest};
use rspack_identifier::IdentifierMap;
use rspack_sources::BoxSource;
use rustc_hash::FxHashMap as HashMap;

use crate::{
  AssetInfo, ChunkInitFragments, ModuleIdentifier, RuntimeGlobals, RuntimeMode, RuntimeSpec,
  RuntimeSpecMap, SourceType,
};

#[derive(Clone, Debug)]
pub struct CodeGenerationDataUrl {
  inner: String,
}

impl CodeGenerationDataUrl {
  pub fn new(inner: String) -> Self {
    Self { inner }
  }

  pub fn inner(&self) -> &str {
    &self.inner
  }
}

#[derive(Clone, Debug)]
pub struct CodeGenerationDataFilename {
  inner: String,
}

impl CodeGenerationDataFilename {
  pub fn new(inner: String) -> Self {
    Self { inner }
  }

  pub fn inner(&self) -> &str {
    &self.inner
  }
}

#[derive(Clone, Debug)]
pub struct CodeGenerationDataAssetInfo {
  inner: AssetInfo,
}

impl CodeGenerationDataAssetInfo {
  pub fn new(inner: AssetInfo) -> Self {
    Self { inner }
  }

  pub fn inner(&self) -> &AssetInfo {
    &self.inner
  }
}

#[derive(Debug, Default, Clone)]
pub struct CodeGenerationData {
  inner: anymap::Map<dyn CloneAny + Send + Sync>,
}

impl Deref for CodeGenerationData {
  type Target = anymap::Map<dyn CloneAny + Send + Sync>;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl DerefMut for CodeGenerationData {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

#[derive(Debug, Default, Clone)]
pub struct CodeGenerationResult {
  inner: HashMap<SourceType, BoxSource>,
  /// [definition in webpack](https://github.com/webpack/webpack/blob/4b4ca3bb53f36a5b8fc6bc1bd976ed7af161bd80/lib/Module.js#L75)
  pub data: CodeGenerationData,
  pub chunk_init_fragments: ChunkInitFragments,
  pub runtime_requirements: RuntimeGlobals,
  pub hash: Option<RspackHashDigest>,
}

impl CodeGenerationResult {
  pub fn with_javascript(mut self, generation_result: BoxSource) -> Self {
    self.inner.insert(SourceType::JavaScript, generation_result);
    self
  }

  pub fn with_css(mut self, generation_result: BoxSource) -> Self {
    self.inner.insert(SourceType::Css, generation_result);
    self
  }

  pub fn with_asset(mut self, generation_result: BoxSource) -> Self {
    self.inner.insert(SourceType::Asset, generation_result);
    self
  }

  pub fn inner(&self) -> &HashMap<SourceType, BoxSource> {
    &self.inner
  }

  pub fn get(&self, source_type: &SourceType) -> Option<&BoxSource> {
    self.inner.get(source_type)
  }

  pub fn add(&mut self, source_type: SourceType, generation_result: BoxSource) {
    let result = self.inner.insert(source_type, generation_result);
    debug_assert!(result.is_none());
  }

  pub fn set_hash(
    &mut self,
    hash_function: &HashFunction,
    hash_digest: &HashDigest,
    hash_salt: &HashSalt,
  ) {
    let mut hasher = RspackHash::with_salt(hash_function, hash_salt);
    for (source_type, source) in &self.inner {
      source_type.hash(&mut hasher);
      source.hash(&mut hasher);
    }
    self.chunk_init_fragments.hash(&mut hasher);
    self.hash = Some(hasher.digest(hash_digest));
  }
}

#[derive(Debug, Default)]
pub struct CodeGenerationResults {
  // TODO: remove Arc after we finished runtime optimization.
  map: IdentifierMap<RuntimeSpecMap<Arc<CodeGenerationResult>>>,
}

impl CodeGenerationResults {
  pub fn get_one(&self, module_identifier: &ModuleIdentifier) -> Option<&CodeGenerationResult> {
    self
      .map
      .get(module_identifier)
      .and_then(|spec| match spec.mode {
        RuntimeMode::Empty => None,
        RuntimeMode::SingleEntry => spec.single_value.as_deref(),
        RuntimeMode::Map => spec.map.values().next().map(|v| &**v),
      })
  }

  pub fn clear_entry(
    &mut self,
    module_identifier: &ModuleIdentifier,
  ) -> Option<(ModuleIdentifier, RuntimeSpecMap<Arc<CodeGenerationResult>>)> {
    self.map.remove_entry(module_identifier)
  }

  pub fn get(
    &self,
    module_identifier: &ModuleIdentifier,
    runtime: Option<&RuntimeSpec>,
  ) -> Result<&CodeGenerationResult> {
    if let Some(entry) = self.map.get(module_identifier) {
      if let Some(runtime) = runtime {
        entry
          .get(runtime)
          .map(|v| &**v)
          .ok_or_else(|| {
            internal_error!(
              "Failed to code generation result for {module_identifier} with runtime {runtime:?} \n {entry:?}"
            )
          })
      } else {
        if entry.size() > 1 {
          let results = entry.get_values();
          if results.len() != 1 {
            return Err(internal_error!(
              "No unique code generation entry for unspecified runtime for {module_identifier} ",
            ));
          }

          return results
            .first()
            .map(|r| &***r)
            .ok_or_else(|| internal_error!("Expected value exists"));
        }

        entry
          .get_values()
          .first()
          .map(|r| &***r)
          .ok_or_else(|| internal_error!("Expected value exists"))
      }
    } else {
      Err(internal_error!(
        "No code generation entry for {} (existing entries: {:?})",
        module_identifier,
        self.map.keys().collect::<Vec<_>>()
      ))
    }
  }

  pub fn add(
    &mut self,
    module_identifier: ModuleIdentifier,
    runtime: RuntimeSpec,
    result: Arc<CodeGenerationResult>,
  ) {
    match self.map.entry(module_identifier) {
      Entry::Occupied(mut record) => {
        record.get_mut().set(runtime, result);
      }
      Entry::Vacant(record) => {
        let mut spec_map = RuntimeSpecMap::default();
        spec_map.set(runtime, result);
        record.insert(spec_map);
      }
    };
  }

  pub fn get_runtime_requirements(
    &self,
    module_identifier: &ModuleIdentifier,
    runtime: Option<&RuntimeSpec>,
  ) -> RuntimeGlobals {
    match self.get(module_identifier, runtime) {
      Ok(result) => result.runtime_requirements,
      Err(_) => {
        eprint!("Failed to get runtime requirements for {module_identifier}");
        Default::default()
      }
    }
  }

  #[allow(clippy::unwrap_in_result)]
  pub fn get_hash(
    &self,
    module_identifier: &ModuleIdentifier,
    runtime: Option<&RuntimeSpec>,
  ) -> Option<&RspackHashDigest> {
    let code_generation_result = self
      .get(module_identifier, runtime)
      .expect("should have code generation result");

    code_generation_result.hash.as_ref()
  }
}
