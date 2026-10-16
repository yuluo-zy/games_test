//! ColorId newtype observed in B components and the render consumers.
#[derive(
    bevy_ecs::component::Component,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(transparent)]
pub struct ColorId(pub u32);

impl ColorId {
    /// B 0x14099ca50.
    pub fn default_value() -> Self {
        Self(0)
    }
    pub fn is_default(&self) -> bool {
        self.0 == 0
    }
}
