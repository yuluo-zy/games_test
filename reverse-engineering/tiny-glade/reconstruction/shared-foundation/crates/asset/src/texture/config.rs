use super::desc::{CompressionMethod, Gamma, MipChain, TextureOpts};
use crate::inheritable::{Inherit, Inheritable, deserialize_inheritable_option};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct InheritableTextureOpts {
    #[serde(deserialize_with = "deserialize_inheritable_option")]
    pub resize: Inheritable<Option<[u32; 2]>>,
    pub gamma: Inheritable<Gamma>,
    pub mips: Inheritable<MipChain>,
    pub compression: Inheritable<CompressionMethod>,
}
impl Inherit for InheritableTextureOpts {
    fn inherit_from(&mut self, parent: &Self) {
        self.resize.inherit_shallow(&parent.resize);
        self.gamma.inherit_shallow(&parent.gamma);
        self.mips.inherit_shallow(&parent.mips);
        self.compression.inherit_shallow(&parent.compression);
    }
}
impl InheritableTextureOpts {
    /// Resolved root defaults, unlike `Default`'s missing-input markers.
    /// B inlinee 0x1a65f at 0x1420d8c24 writes all four concrete zero tags.
    pub fn concrete_default() -> Self {
        Self::from_concrete(TextureOpts {
            resize: None,
            gamma: Gamma::Linear,
            mips: MipChain::None,
            compression: CompressionMethod::Color,
        })
    }

    pub fn from_concrete(value: TextureOpts) -> Self {
        Self {
            resize: Inheritable::Value(value.resize),
            gamma: Inheritable::Value(value.gamma),
            mips: Inheritable::Value(value.mips),
            compression: Inheritable::Value(value.compression),
        }
    }
    /// B 0x142456250: fields resolved independently; unknown defaults panic, no invented fallbacks.
    pub fn into_concrete(self) -> TextureOpts {
        TextureOpts {
            resize: self.resize.into_concrete(),
            gamma: self.gamma.into_concrete(),
            mips: self.mips.into_concrete(),
            compression: self.compression.into_concrete(),
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct TextureGroupConfig {
    pub default: InheritableTextureOpts,
    pub assets: HashMap<String, InheritableTextureOpts>,
}
