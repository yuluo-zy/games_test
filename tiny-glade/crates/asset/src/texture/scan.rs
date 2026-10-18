use super::config::{InheritableTextureOpts, TextureGroupConfig};
use super::desc::{TextureAssetDesc, validate_logical_name};
use crate::inheritable::Inherit;
use anyhow::{Context, Result, bail};
use relative_path::RelativePathBuf;
use std::{
    collections::{HashMap, hash_map::Entry},
    path::Path,
};

pub fn scan_asset_tree(root: &Path) -> Result<HashMap<RelativePathBuf, TextureAssetDesc>> {
    let mut output = HashMap::new();
    scan_asset_tree_walker(
        root,
        &RelativePathBuf::new(),
        &InheritableTextureOpts::concrete_default(),
        &mut output,
    )?;
    Ok(output)
}
/// Scan lower-case PNG, JPEG and DDS files, inheriting each directory's options.
/// Logical names are unique across ordinary textures and `array_` directories.
pub fn scan_asset_tree_walker(
    dir: &Path,
    prefix: &RelativePathBuf,
    parent: &InheritableTextureOpts,
    out: &mut HashMap<RelativePathBuf, TextureAssetDesc>,
) -> Result<()> {
    let config_path = dir.join("assets.ron");
    let mut config: TextureGroupConfig = if config_path.is_file() {
        ron::from_str(
            &std::fs::read_to_string(&config_path)
                .with_context(|| format!("Reading {config_path:?}"))?,
        )
        .with_context(|| format!("Parsing {config_path:?}"))?
    } else {
        TextureGroupConfig::default()
    };
    config.default.inherit_from(parent);
    for entry in std::fs::read_dir(dir).with_context(|| format!("Reading {dir:?}"))? {
        let entry = entry?;
        let path = entry.path();
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("Non-UTF8 asset file name in {dir:?}"))?;
        if entry.file_type()?.is_dir() {
            if let Some(array_name) = name.strip_prefix("array_") {
                let mut layer_map = HashMap::new();
                scan_asset_tree_walker(
                    &path,
                    &RelativePathBuf::new(),
                    &config.default,
                    &mut layer_map,
                )?;
                if layer_map.is_empty() {
                    continue;
                }
                let mut layers = Vec::with_capacity(layer_map.len());
                for n in 0..layer_map.len() {
                    layers.push(
                        layer_map
                            .remove(&RelativePathBuf::from(n.to_string()))
                            .with_context(|| format!("Array {path:?} is missing layer {n}"))?,
                    );
                }
                let opts = config
                    .default
                    .clone()
                    .try_into_concrete()
                    .with_context(|| format!("Resolving texture options for {path:?}"))?;
                insert_unique(
                    out,
                    prefix.join(array_name),
                    TextureAssetDesc { path, opts, layers },
                )?;
            } else {
                scan_asset_tree_walker(&path, &prefix.join(&name), &config.default, out)?;
            }
        } else {
            if name == "assets.ron" {
                continue;
            }
            let ext = path.extension().and_then(|s| s.to_str());
            match ext {
                Some("txt" | "asset-filter") => continue,
                Some("png" | "jpg" | "dds") => {}
                _ => bail!("Unsupported image file extension {ext:?}: {path:?}"),
            }
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .context("Missing texture stem")?;
            let mut opts = config.assets.get(stem).cloned().unwrap_or_default();
            opts.inherit_from(&config.default);
            let opts = opts
                .try_into_concrete()
                .with_context(|| format!("Resolving texture options for {path:?}"))?;
            insert_unique(
                out,
                prefix.join(stem),
                TextureAssetDesc {
                    path,
                    opts,
                    layers: Vec::new(),
                },
            )?;
        }
    }
    Ok(())
}

fn insert_unique(
    output: &mut HashMap<RelativePathBuf, TextureAssetDesc>,
    name: RelativePathBuf,
    texture: TextureAssetDesc,
) -> Result<()> {
    validate_logical_name(&name).with_context(|| format!("Naming texture {:?}", texture.path))?;
    match output.entry(name) {
        Entry::Vacant(entry) => {
            entry.insert(texture);
            Ok(())
        }
        Entry::Occupied(entry) => bail!(
            "Duplicate texture name {:?}: {:?} and {:?}",
            entry.key(),
            entry.get().path,
            texture.path
        ),
    }
}
