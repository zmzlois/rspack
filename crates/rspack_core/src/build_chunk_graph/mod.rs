// use rspack_core::Bundle;
// use rspack_core::ChunkGraph;

use rustc_hash::FxHashSet;
use tracing::instrument;

pub(crate) use self::code_splitter::CodeSplitter;
use crate::{Compilation, MakeParam, ModuleIdentifier};

mod code_splitter;

#[instrument(skip_all)]
pub(crate) fn build_chunk_graph(
  compilation: &mut Compilation,
  params: &Vec<MakeParam>,
  code_splitter: Option<CodeSplitter>,
) -> rspack_error::Result<CodeSplitter> {
  let mut modify_modules: Option<FxHashSet<ModuleIdentifier>> = None;

  for param in params {
    match param {
      MakeParam::ModifiedFiles(param_modify_files) => {
        let param_modify_modules =
          compilation
            .module_graph
            .modules()
            .keys()
            .filter(|module_identifier| {
              compilation
                .module_graph
                .is_module_invalidated(module_identifier, &param_modify_files)
            });

        match &mut modify_modules {
          Some(modify_files) => {
            modify_files.extend(param_modify_modules);
          }
          None => modify_modules = Some(param_modify_modules.cloned().collect()),
        }
      }
      _ => {
        modify_modules = None;
        break;
      }
    }
  }

  let mut code_spliter =
    code_splitter::CodeSplitter::new(compilation, modify_modules, code_splitter);
  code_spliter.split(compilation)?;
  Ok(code_spliter)
}
