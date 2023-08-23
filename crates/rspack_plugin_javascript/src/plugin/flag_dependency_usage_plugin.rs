use std::collections::VecDeque;

use rspack_core::ModuleGraph;

struct FlagDependencyUsagePlugin<'a> {
  global: bool,
  module_graph: &'a mut ModuleGraph,
}

impl FlagDependencyUsagePlugin {
  pub fn new(global: bool, module_graph: &mut ModuleGraph) -> Self {
    Self {
      global,
      module_graph,
    }
  }

  fn apply() {
    let mut q = VecDeque::new();
    1
  }
}
