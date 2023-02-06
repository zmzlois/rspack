use bitflags;
use rspack_database::Ukey;
use rspack_symbol::{IndirectTopLevelSymbol, Symbol};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

use self::visitor::TreeShakingResult;
use crate::{Chunk, IdentifierMap, IdentifierSet};

pub mod utils;
pub mod visitor;
#[derive(Debug)]
pub struct OptimizeDependencyResult {
  pub used_symbol: HashSet<Symbol>,
  pub used_indirect_symbol: HashSet<IndirectTopLevelSymbol>,
  pub analyze_results: IdentifierMap<TreeShakingResult>,
  pub bail_out_module_identifiers: IdentifierMap<BailoutFlog>,
  pub chunk_key_to_used_modules_map: HashMap<Ukey<Chunk>, IdentifierSet>,
}
const ANALYZE_LOGGING: bool = true;
pub static CARED_MODULE_ID: &[&str] = &[];

pub fn debug_care_module_id<T: AsRef<str>>(id: T) -> bool {
  if !ANALYZE_LOGGING {
    return false;
  }
  if CARED_MODULE_ID.is_empty() {
    return true;
  }
  CARED_MODULE_ID
    .iter()
    .any(|module_id| id.as_ref().contains(module_id))
}

bitflags::bitflags! {
  pub struct BailoutFlog: u8 {
      const HELPER = 1 << 0;
      const COMMONJS_REQUIRE = 1 << 1;
      const COMMONJS_EXPORTS = 1 << 2;
      const DYNAMIC_IMPORT = 1 << 3;
  }
}

#[derive(Debug, Clone, Copy)]
pub enum SideEffect {
  Configuration(bool),
  Analyze(bool),
}
