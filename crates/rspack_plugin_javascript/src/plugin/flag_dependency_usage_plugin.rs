#[allow(unused)]
use std::collections::VecDeque;

use rspack_core::tree_shaking::visitor::ModuleIdOrDepId;
use rspack_core::{
  Compilation, Dependency, DependencyId, ModuleGraph, ModuleIdentifier, RuntimeSpec,
};

struct FlagDependencyUsagePlugin<'a> {
  global: bool,
  compilation: &'a mut Compilation,
}

impl<'a> FlagDependencyUsagePlugin<'a> {
  pub fn new(global: bool, compilation: &'a mut Compilation) -> Self {
    Self {
      global,
      compilation,
    }
  }

  fn apply(&mut self) {
    for (name, entry_data) in self.compilation.entries.iter() {}
  }

  fn process_entry_dependency(&mut self, dep: DependencyId, runtime: &RuntimeSpec) {
    if let Some(module) = self
      .compilation
      .module_graph
      .module_graph_module_by_dependency_id(&dep)
    {}
  }

  fn process_referenced_module(
    &mut self,
    module_id: ModuleIdentifier,
    used_exports: Vec<ReferencedExport>,
  ) {
  }
}
