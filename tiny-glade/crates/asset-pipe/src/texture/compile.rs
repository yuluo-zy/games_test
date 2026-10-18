use super::{
    TextureAssetLibraryMeta, TextureAssetMeta,
    cache::{PIPELINE_REVISION, source_stamps},
    cache_is_current,
    output::replace_file,
    process_texture_asset,
};
use anyhow::{Context, Result};
use asset::texture::desc::{TextureAssetDesc, validate_logical_name};
use relative_path::{RelativePath, RelativePathBuf};
use std::{collections::HashMap, path::Path, sync::Mutex};
use unicode_normalization::UnicodeNormalization;

// CPU count alone can spawn hundreds of simultaneous decoders on large hosts.
const MAX_COMPILE_WORKERS: usize = 8;

/// Compile one texture or reuse a complete, validated cache entry.
/// Invalid cache files are disposable; source and publication errors are reported.
pub fn compile_texture_asset(
    name: &RelativePath,
    descriptor: TextureAssetDesc,
    output_root: &Path,
    previous: Option<&TextureAssetMeta>,
) -> Result<TextureAssetMeta> {
    // RelativePath permits parent traversal and native separators; reject them
    // before any filesystem work so a public call cannot publish outside its root.
    validate_logical_name(name)?;
    let sources = source_stamps(&descriptor)?;
    let modified = std::fs::metadata(&descriptor.path)
        .with_context(|| format!("Reading metadata for {}", descriptor.path.display()))?
        .modified()?;
    let output = name.with_extension("texture").to_path(output_root);
    let mut metadata = TextureAssetMeta {
        pipeline_revision: PIPELINE_REVISION,
        output_hash: None,
        desc: descriptor,
        src_mtime: modified,
        sources,
    };
    let dependencies_match = previous.is_some_and(|previous| {
        previous.desc == metadata.desc
            && previous.src_mtime == metadata.src_mtime
            && previous.sources == metadata.sources
            && !metadata.sources.is_empty()
    });
    if dependencies_match
        && let Ok(bytes) = std::fs::read(&output)
        && cache_is_current(
            &metadata.desc,
            modified,
            &metadata.sources,
            previous,
            &bytes,
        )
    {
        metadata.output_hash = previous.and_then(|previous| previous.output_hash);
        return Ok(metadata);
    }
    let blob = process_texture_asset(&metadata.desc)
        .with_context(|| format!("Processing texture {name}"))?;
    let bytes = blob.to_file_bytes()?;
    replace_file(&output, &bytes)?;
    metadata.output_hash = Some(xxhash_rust::xxh3::xxh3_64(&bytes));
    Ok(metadata)
}

/// Compile with bounded parallelism and publish indices only after every job succeeds.
/// Individual outputs are replaced atomically, but a batch is not a multi-file transaction.
/// A failed batch may leave new textures beside the previous indices; the next run retries.
pub fn compile_texture_assets(
    root: &Path,
) -> Result<(Vec<RelativePathBuf>, TextureAssetLibraryMeta)> {
    let metadata_path = root.join("tmp/textures.meta.ron");
    let previous: TextureAssetLibraryMeta = std::fs::read_to_string(&metadata_path)
        .ok()
        .and_then(|text| ron::from_str(&text).ok())
        .unwrap_or_default();
    let descriptors = asset::texture::scan::scan_asset_tree(&root.join("assets-src/textures"))?;
    let output_root = root.join("compiled-assets/textures");
    let mut jobs: Vec<_> = descriptors.into_iter().collect();
    jobs.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    validate_names(jobs.iter().map(|(name, _)| name.as_relative_path()))?;
    let names: Vec<_> = jobs.iter().map(|(name, _)| name.clone()).collect();
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(MAX_COMPILE_WORKERS)
        .min(jobs.len());
    let mut results = run_jobs(jobs, workers, |(name, descriptor)| {
        let result =
            compile_texture_asset(&name, descriptor, &output_root, previous.assets.get(&name));
        (name, result)
    });
    // Completion order depends on worker scheduling; error selection should not.
    results.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    let mut metadata = TextureAssetLibraryMeta::default();
    for (name, result) in results {
        metadata.assets.insert(name, result?);
    }
    let metadata_bytes = ron::to_string(&metadata)?;
    let list_bytes = ron::to_string(&names)?;
    replace_file(&metadata_path, metadata_bytes.as_bytes())?;
    replace_file(
        &root.join("compiled-assets/textures/list.ron"),
        list_bytes.as_bytes(),
    )?;
    Ok((names, metadata))
}

// Validate both source scans and shipped indices before any caller uses their paths.
fn validate_names<'a>(names: impl IntoIterator<Item = &'a RelativePath>) -> Result<()> {
    let mut outputs = HashMap::new();
    for name in names {
        validate_logical_name(name)?;
        // Extension replacement and filesystem case/Unicode folding can collapse
        // distinct logical names. Keep the existing runtime path convention.
        let output_name = name.with_extension("texture");
        let key: String = output_name.as_str().to_uppercase().nfd().collect();
        if let Some(previous) = outputs.insert(key, name) {
            anyhow::bail!("Texture names {previous:?} and {name:?} map to the same output file");
        }
    }
    Ok(())
}

// Each worker owns its results; the lock only covers taking the next descriptor.
// All workers finish before any job error is returned to the caller.
fn run_jobs<Job: Send, Output: Send>(
    jobs: Vec<Job>,
    workers: usize,
    process: impl Fn(Job) -> Output + Sync,
) -> Vec<Output> {
    if workers <= 1 {
        return jobs.into_iter().map(process).collect();
    }
    let pending = Mutex::new(jobs.into_iter());
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut results = Vec::new();
                    loop {
                        let job = pending
                            .lock()
                            .unwrap_or_else(|poisoned| poisoned.into_inner())
                            .next();
                        let Some(job) = job else { break };
                        results.push(process(job));
                    }
                    results
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().expect("texture worker panicked"))
            .collect()
    })
}

/// Compile available sources when requested; otherwise load the shipped index.
/// A new source tree does not require a pre-existing compiled index.
pub fn compile_or_load_texture_assets(
    root: &Path,
    allow_compile: bool,
) -> Result<Vec<RelativePathBuf>> {
    if allow_compile && root.join("assets-src/textures").is_dir() {
        return Ok(compile_texture_assets(root)?.0);
    }
    let path = root.join("compiled-assets/textures/list.ron");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Reading texture index {}", path.display()))?;
    let names: Vec<RelativePathBuf> = ron::from_str(&text)
        .with_context(|| format!("Parsing texture index {}", path.display()))?;
    validate_names(names.iter().map(|name| name.as_relative_path()))
        .with_context(|| format!("Validating texture index {}", path.display()))?;
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Barrier,
        atomic::{AtomicUsize, Ordering},
    };

    #[test]
    fn workers_are_bounded_and_join_every_result_including_errors() {
        let active = AtomicUsize::new(0);
        let peak = AtomicUsize::new(0);
        let barrier = Barrier::new(3);
        let results = run_jobs((0..24).collect(), 3, |job| {
            let count = active.fetch_add(1, Ordering::SeqCst) + 1;
            peak.fetch_max(count, Ordering::SeqCst);
            if job < 3 {
                barrier.wait();
            }
            active.fetch_sub(1, Ordering::SeqCst);
            if job == 0 { Err(job) } else { Ok(job) }
        });
        assert_eq!(results.len(), 24);
        assert_eq!(results.iter().filter(|result| result.is_err()).count(), 1);
        assert_eq!(active.load(Ordering::SeqCst), 0);
        assert_eq!(peak.load(Ordering::SeqCst), 3);
    }
}
