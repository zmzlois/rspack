use futures::Future;
use indexmap::IndexMap;
use rspack_database::Database;
use rspack_error::Result;
use rustc_hash::FxHashMap as HashMap;
use tracing::instrument;

use crate::{
  build_chunk_graph::CodeSplitter, Chunk, ChunkGraph, ChunkGroup, ChunkGroupUkey, ChunkUkey,
  Compilation,
};

#[derive(Debug, Default)]
pub struct CodeSplittingCache {
  chunk_by_ukey: Database<Chunk>,
  chunk_graph: ChunkGraph,
  chunk_group_by_ukey: Database<ChunkGroup>,
  entrypoints: IndexMap<String, ChunkGroupUkey>,
  async_entrypoints: Vec<ChunkGroupUkey>,
  named_chunk_groups: HashMap<String, ChunkGroupUkey>,
  named_chunks: HashMap<String, ChunkUkey>,
  code_splitter: CodeSplitter,
}

#[instrument(skip_all)]
pub(crate) async fn use_code_splitting_cache<'a, T, F>(
  compilation: &'a mut Compilation,
  task: T,
) -> Result<()>
where
  T: Fn(&'a mut Compilation, Option<CodeSplitter>) -> F,
  F: Future<Output = Result<(&'a mut Compilation, CodeSplitter)>>,
{
  let is_incremental_rebuild = compilation.options.is_incremental_rebuild_make_enabled();
  if !is_incremental_rebuild {
    task(compilation, None).await?;
    return Ok(());
  }

  if !compilation.has_module_import_export_change {
    let cache = &mut compilation.code_splitting_cache;
    rayon::scope(|s| {
      s.spawn(|_| compilation.chunk_by_ukey = cache.chunk_by_ukey.clone());
      s.spawn(|_| compilation.chunk_graph = cache.chunk_graph.clone());
      s.spawn(|_| compilation.chunk_group_by_ukey = cache.chunk_group_by_ukey.clone());
      s.spawn(|_| compilation.entrypoints = cache.entrypoints.clone());
      s.spawn(|_| compilation.async_entrypoints = cache.async_entrypoints.clone());
      s.spawn(|_| compilation.named_chunk_groups = cache.named_chunk_groups.clone());
      s.spawn(|_| compilation.named_chunks = cache.named_chunks.clone());
    });

    return Ok(());
  }

  let cache = &mut compilation.code_splitting_cache;
  let code_splitter = cache.code_splitter.clone();
  let (compilation, code_splitter) = task(compilation, Some(code_splitter)).await?;
  let cache = &mut compilation.code_splitting_cache;
  rayon::scope(|s| {
    s.spawn(|_| cache.chunk_by_ukey = compilation.chunk_by_ukey.clone());
    s.spawn(|_| cache.chunk_graph = compilation.chunk_graph.clone());
    s.spawn(|_| cache.chunk_group_by_ukey = compilation.chunk_group_by_ukey.clone());
    s.spawn(|_| cache.entrypoints = compilation.entrypoints.clone());
    s.spawn(|_| cache.async_entrypoints = compilation.async_entrypoints.clone());
    s.spawn(|_| cache.named_chunk_groups = compilation.named_chunk_groups.clone());
    s.spawn(|_| cache.named_chunks = compilation.named_chunks.clone());
    s.spawn(|_| cache.code_splitter = code_splitter);
  });
  Ok(())
}
