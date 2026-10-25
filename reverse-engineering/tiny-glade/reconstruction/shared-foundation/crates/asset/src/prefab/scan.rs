use super::{PrefabName, config::PrefabConfig};
use crate::inheritable::Inherit;
use anyhow::{Context, Result, bail};
use relative_path::RelativePathBuf;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Default, Debug)]
pub struct PrefabDependencies {
    pub by_file: HashMap<PathBuf, Vec<PrefabName>>,
}
impl PrefabDependencies {
    /// Original pushes a name into Vec; duplicate registrations are preserved.
    pub fn add_file_to_prefab_dependency(&mut self, file: &Path, name: PrefabName) {
        self.by_file.entry(file.to_owned()).or_default().push(name);
    }
}
#[derive(Default, Debug)]
pub struct PrefabScan {
    pub prefabs: HashMap<PrefabName, PrefabConfig>,
    pub dependencies: PrefabDependencies,
}
pub fn load_prefab_config(path: &Path) -> Result<PrefabConfig> {
    ron::from_str(&std::fs::read_to_string(path).with_context(|| format!("Reading {path:?}"))?)
        .with_context(|| format!("Parsing {path:?}"))
}
pub fn scan_asset_tree(root: &Path) -> Result<PrefabScan> {
    let mut result = PrefabScan::default();
    scan_asset_tree_walker(
        root,
        &RelativePathBuf::new(),
        &PrefabConfig::default(),
        &mut vec![],
        &mut result,
    )?;
    Ok(result)
}
pub fn scan_asset_tree_walker(
    dir: &Path,
    prefix: &RelativePathBuf,
    parent: &PrefabConfig,
    dependencies: &mut Vec<PathBuf>,
    result: &mut PrefabScan,
) -> Result<()> {
    let start = dependencies.len();
    let default_path = dir.join("default.ron");
    let mut inherited = parent.clone();
    if default_path.is_file() {
        inherited = load_prefab_config(&default_path)?;
        inherited.inherit_from(parent);
        dependencies.push(default_path);
    }
    for entry in std::fs::read_dir(dir).with_context(|| format!("Reading {dir:?}"))? {
        let entry = entry?;
        let path = entry.path();
        let filename = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("Non-UTF8 prefab name"))?;
        if entry.file_type()?.is_dir() {
            scan_asset_tree_walker(
                &path,
                &prefix.join(filename),
                &inherited,
                dependencies,
                result,
            )?;
        } else {
            if filename == "default.ron" {
                continue;
            }
            match path.extension().and_then(|x| x.to_str()) {
                Some("asset-filter") => continue,
                Some("ron") => {}
                ext => bail!("Unsupported prefab file extension: {ext:?}"),
            }
            let name = PrefabName::new(
                prefix
                    .join(
                        path.file_stem()
                            .and_then(|x| x.to_str())
                            .context("Missing prefab stem")?,
                    )
                    .as_str(),
            );
            let mut config = load_prefab_config(&path)?;
            config.inherit_from(&inherited);
            result.prefabs.insert(name.clone(), config);
            for file in dependencies.iter().chain(std::iter::once(&path)) {
                result
                    .dependencies
                    .add_file_to_prefab_dependency(file, name.clone());
            }
        }
    }
    dependencies.truncate(start);
    Ok(())
}
