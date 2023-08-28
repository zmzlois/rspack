use std::collections::VecDeque;

use rspack_core::{
  DependencyId, ExportInfo, ExportsInfo, ExportsSpec, ModuleGraph, ModuleIdentifier, UsageState,
};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use swc_core::ecma::atoms::JsWord;

pub struct ProvidedExportsPlugin<'a> {
  mg: &'a mut ModuleGraph,
}

impl<'a> ProvidedExportsPlugin<'a> {
  pub fn apply(&mut self) {
    let mut dependencies: HashMap<ModuleIdentifier, HashSet<ModuleIdentifier>> = HashMap::default();
    let mut q = VecDeque::new();
    while let Some(module_id) = q.pop_back() {
      let mut changed = false;
      let exports_specs_from_dependencies: HashMap<DependencyId, ExportsSpec> = HashMap::default();
      self.process_dependencies_block(module_id);
      // for (const [dep, exportsSpec] of exportsSpecsFromDependencies) {
      // 	processExportsSpec(dep, exportsSpec);
      // }
    }
  }

  pub fn process_dependencies_block(&mut self, mi: ModuleIdentifier) -> Option<()> {
    None
  }
  pub fn process_exports_spec(
    &mut self,
    dep_id: DependencyId,
    exports_desc: ExportsSpec,
    exports_info: &mut ExportsInfo,
  ) {
    // const exports = exportDesc.exports;
    // const globalCanMangle = exportDesc.canMangle;
    // const globalFrom = exportDesc.from;
    // const globalPriority = exportDesc.priority;
    // const globalTerminalBinding =
    // 	exportDesc.terminalBinding || false;
    // const exportDeps = exportDesc.dependencies;
    // if (exportDesc.hideExports) {
    // 	for (const name of exportDesc.hideExports) {
    // 		const exportInfo = exportsInfo.getExportInfo(name);
    // 		exportInfo.unsetTarget(dep);
    // 	}
    // }
    let exports = &exports_desc.exports;
    let global_can_mangle = &exports_desc.can_mangle;
    let global_from = &exports_desc.from;
    let global_priority = &exports_desc.priority;
    let global_terminal_binding = exports_desc.terminal_binding.clone().unwrap_or(false);
    let export_dependencies = &exports_desc.dependencies;
    if !exports_desc.hide_export.is_empty() {
      for name in exports_desc.hide_export.iter() {
        // let export_info = exports_info.get_export_info_mut(name);

        let info = if let Some(info) = exports_info.exports.get_mut(name) {
          info
        } else if let Some(ref mut redirect_to) = exports_info.redirect_to {
          redirect_to.get_export_info_mut(name)
          // exports_info
          //   .redirect_to
          //   .as_mut()
          //   .unwrap()
          //   .get_export_info_mut(name)
        } else {
          let new_info = ExportInfo::new(
            name.clone(),
            UsageState::Unknown,
            Some(&exports_info.other_exports_info),
          );
          exports_info.exports.insert(name.clone(), new_info);
          exports_info._exports_are_ordered = false;
          // SAFETY: because we insert the export info above
          exports_info
            .exports
            .get_mut(name)
            .expect("This is unreachable")
        };
      }
    }
  }
}
