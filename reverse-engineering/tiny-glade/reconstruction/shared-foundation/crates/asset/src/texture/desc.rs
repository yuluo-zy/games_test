use crate::inheritable::Inherit;
use serde::{Deserialize, Serialize};

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
/// B derived serialization names and field types are independently present in the PDB.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextureAssetDesc {
    pub path: std::path::PathBuf,
    pub opts: TextureOpts,
    pub layers: Vec<TextureAssetDesc>,
}
impl TextureAssetDesc {
    pub fn source_files(&self) -> Vec<&std::path::Path> {
        if self.layers.is_empty() {
            vec![self.path.as_path()]
        } else {
            self.layers.iter().flat_map(Self::source_files).collect()
        }
    }
}
