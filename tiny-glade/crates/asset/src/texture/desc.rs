use crate::inheritable::Inherit;
use anyhow::{Result, ensure};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};

/// Check an asset's portable, slash-separated path within an output directory.
///
/// Empty components, traversal components and platform-specific separators or
/// drive prefixes cannot be represented as logical asset names.
pub fn validate_logical_name(name: &RelativePath) -> Result<()> {
    let name = name.as_str();
    ensure!(
        !name.contains(['\\', ':'])
            && name
                .split('/')
                .all(|component| !matches!(component, "" | "." | "..")),
        "Invalid texture logical name {name:?}"
    );
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gamma {
    #[default]
    Linear,
    #[serde(rename = "sRGB")]
    Srgb,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum MipChain {
    #[default]
    None,
    Full,
    Limited(u32),
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionMethod {
    #[default]
    Color,
    Grayscale,
    None,
}
impl Inherit for Gamma {
    fn inherit_from(&mut self, _: &Self) {}
}
impl Inherit for MipChain {
    fn inherit_from(&mut self, _: &Self) {}
}
impl Inherit for CompressionMethod {
    fn inherit_from(&mut self, _: &Self) {}
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextureOpts {
    pub resize: Option<[u32; 2]>,
    pub gamma: Gamma,
    pub mips: MipChain,
    pub compression: CompressionMethod,
}
/// A single texture or an array whose source images are its leaf descriptors.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextureAssetDesc {
    pub path: std::path::PathBuf,
    pub opts: TextureOpts,
    pub layers: Vec<TextureAssetDesc>,
}
impl TextureAssetDesc {
    pub fn source_files(&self) -> Vec<&std::path::Path> {
        let mut files = Vec::new();
        self.visit_source_files(&mut |path| files.push(path));
        files
    }

    /// Visit leaf source paths in layer order without allocating intermediate vectors.
    pub fn visit_source_files<'a>(&'a self, visitor: &mut impl FnMut(&'a std::path::Path)) {
        if self.layers.is_empty() {
            visitor(self.path.as_path());
        } else {
            for layer in &self.layers {
                layer.visit_source_files(visitor);
            }
        }
    }
}
