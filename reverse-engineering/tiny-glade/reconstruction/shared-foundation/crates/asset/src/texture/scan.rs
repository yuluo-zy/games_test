use super::config::{InheritableTextureOpts, TextureGroupConfig};
use super::desc::TextureAssetDesc;
use crate::inheritable::Inherit;
use anyhow::{Context, Result, bail};
use relative_path::RelativePathBuf;
use std::{collections::HashMap, path::Path};

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
/// B only supports lower-case png/jpg/dds, explicitly ignores txt and asset-filter.
pub fn scan_asset_tree_walker(
    dir: &Path,
    prefix: &RelativePathBuf,
    parent: &InheritableTextureOpts,
    out: &mut HashMap<RelativePathBuf, TextureAssetDesc>,
) -> Result<()> {
    let config_path = dir.join("assets.ron");
    let mut config: TextureGroupConfig = if config_path.is_file() {
        ron::from_str(&std::fs::read_to_string(&config_path)?)
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
            .map_err(|_| anyhow::anyhow!("Non-UTF8 asset file name"))?;
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
                let opts = config.default.clone().into_concrete();
                out.insert(
                    prefix.join(array_name),
                    TextureAssetDesc { path, opts, layers },
                );
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
                _ => bail!("Unsupported image file extension: {ext:?}"),
            }
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .context("Missing texture stem")?;
            let mut opts = config.assets.remove(stem).unwrap_or_default();
            opts.inherit_from(&config.default);
            out.insert(
                prefix.join(stem),
                TextureAssetDesc {
                    path,
                    opts: opts.into_concrete(),
                    layers: vec![],
                },
            );
        }
    }
    Ok(())
}
