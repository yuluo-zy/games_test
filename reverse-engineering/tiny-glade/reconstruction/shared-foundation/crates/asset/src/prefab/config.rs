use crate::inheritable::{Inherit, Inheritable, deserialize_inheritable_option};
use serde::Deserialize;
use std::collections::HashMap;
/// Original prefab vector fields use RON sequence syntax, unlike texture resize tuples.
fn deserialize_float3<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Inheritable<[f32; 3]>, D::Error> {
    let value = Vec::<f32>::deserialize(d)?;
    let value: [f32; 3] = value.try_into().map_err(|v: Vec<f32>| {
        serde::de::Error::invalid_length(v.len(), &"three float components")
    })?;
    Ok(Inheritable::Value(value))
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct PrefabConfig {
    pub parts: HashMap<String, PrefabPartConfig>,
}
impl Inherit for PrefabConfig {
    fn inherit_from(&mut self, parent: &Self) {
        self.parts.inherit_from(&parent.parts);
        // B 0x142424d90 performs a second default-resolution pass over the child map.
        for part in self.parts.values_mut() {
            part.inherit_from(&PrefabPartConfig::resolved_defaults());
        }
    }
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct PrefabPartConfig {
    pub delete: Inheritable<bool>,
    pub render: Inheritable<PrefabRenderConfig>,
    #[serde(deserialize_with = "deserialize_inheritable_option")]
    pub rt: Inheritable<Option<PrefabRtConfig>>,
    #[serde(deserialize_with = "deserialize_inheritable_option")]
    pub audio: Inheritable<Option<PrefabAudioConfig>>,
}
impl PrefabPartConfig {
    pub fn resolved_defaults() -> Self {
        Self {
            delete: Inheritable::Value(false),
            render: Inheritable::Value(PrefabRenderConfig::concrete_default()),
            rt: Inheritable::Value(None),
            audio: Inheritable::Value(None),
        }
    }
}
impl Inherit for PrefabPartConfig {
    fn inherit_from(&mut self, p: &Self) {
        self.delete.inherit_from(&p.delete);
        self.render.inherit_from(&p.render);
        self.rt.inherit_from(&p.rt);
        self.audio.inherit_from(&p.audio);
    }
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct PrefabRenderConfig {
    pub mesh: Inheritable<String>,
    #[serde(deserialize_with = "deserialize_inheritable_option")]
    pub mesh_processor: Inheritable<Option<MeshProcessorConfig>>,
    pub shader: Inheritable<String>,
}
impl PrefabRenderConfig {
    pub fn concrete_default()->Self {
        Self {mesh:Inheritable::MissingDefault,shader:Inheritable::MissingDefault,mesh_processor:Inheritable::Value(None)}
    }
}
impl Inherit for PrefabRenderConfig {
    fn inherit_from(&mut self, p: &Self) {
        self.mesh.inherit_from(&p.mesh);
        self.mesh_processor.inherit_from(&p.mesh_processor);
        self.shader.inherit_from(&p.shader);
    }
}
#[derive(Clone, Debug, Deserialize)]
pub enum MeshProcessorConfig {
    Code { name: String },
}
impl Inherit for MeshProcessorConfig {
    fn inherit_from(&mut self, _: &Self) {}
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum RtMesh {
    Aabb,
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum AudioTriggerShape {
    Aabb,
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum RtMeshSemantic {
    None,
    Canopy,
}
impl Inherit for RtMesh {
    fn inherit_from(&mut self, _: &Self) {}
}
impl Inherit for AudioTriggerShape {
    fn inherit_from(&mut self, _: &Self) {}
}
impl Inherit for RtMeshSemantic {
    fn inherit_from(&mut self, _: &Self) {}
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct PrefabRtConfig {
    pub mesh: Inheritable<RtMesh>,
    #[serde(deserialize_with = "deserialize_float3")]
    pub mesh_scale: Inheritable<[f32; 3]>,
    #[serde(deserialize_with = "deserialize_float3")]
    pub summer_albedo: Inheritable<[f32; 3]>,
    #[serde(deserialize_with = "deserialize_float3")]
    pub autumn_albedo: Inheritable<[f32; 3]>,
    #[serde(deserialize_with = "deserialize_float3")]
    pub winter_albedo: Inheritable<[f32; 3]>,
    pub semantic: Inheritable<RtMeshSemantic>,
}
impl PrefabRtConfig {
    /// Original option finalizer 0x14268a42f..0x14268a4d7. Mesh is required;
    /// scale and all three seasonal albedos default to one, semantic to None.
    pub fn concrete_default()->Self {
        Self {
            mesh:Inheritable::MissingDefault,
            mesh_scale:Inheritable::Value([1.0;3]),
            summer_albedo:Inheritable::Value([1.0;3]),
            autumn_albedo:Inheritable::Value([1.0;3]),
            winter_albedo:Inheritable::Value([1.0;3]),
            semantic:Inheritable::Value(RtMeshSemantic::None),
        }
    }
}
impl Inherit for PrefabRtConfig {
    fn inherit_from(&mut self, p: &Self) {
        self.mesh.inherit_from(&p.mesh);
        self.mesh_scale.inherit_from(&p.mesh_scale);
        self.summer_albedo.inherit_from(&p.summer_albedo);
        self.autumn_albedo.inherit_from(&p.autumn_albedo);
        self.winter_albedo.inherit_from(&p.winter_albedo);
        self.semantic.inherit_from(&p.semantic);
    }
    fn finish_inheritance(&mut self) {self.inherit_from(&Self::concrete_default());}
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct PrefabAudioConfig {
    pub trigger: Inheritable<AudioTriggerConfig>,
}
impl PrefabAudioConfig {
    pub fn concrete_default()->Self {
        Self {trigger:Inheritable::Value(AudioTriggerConfig::concrete_default())}
    }
}
impl Inherit for PrefabAudioConfig {
    fn inherit_from(&mut self, p: &Self) {
        self.trigger.inherit_from(&p.trigger);
    }
    fn finish_inheritance(&mut self) {
        // Original trigger wrapper rejects an already exhausted outer object
        // marker; required fields inside a concrete trigger remain unresolved
        // until extraction, which is a different state.
        assert!(!matches!(self.trigger,Inheritable::MissingDefault),"cannot finalize an exhausted audio trigger");
        self.inherit_from(&Self::concrete_default());
    }
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct AudioTriggerConfig {
    pub shape: Inheritable<AudioTriggerShape>,
    #[serde(deserialize_with = "deserialize_float3")]
    pub shape_scale: Inheritable<[f32; 3]>,
    pub kind: Inheritable<String>,
}
impl AudioTriggerConfig {
    /// Original inline 0x14268a790: required shape/kind markers, unit scale.
    pub fn concrete_default()->Self {
        Self {shape:Inheritable::MissingDefault,shape_scale:Inheritable::Value([1.0;3]),kind:Inheritable::MissingDefault}
    }
}
impl Inherit for AudioTriggerConfig {
    fn inherit_from(&mut self, p: &Self) {
        self.shape.inherit_from(&p.shape);
        self.shape_scale.inherit_from(&p.shape_scale);
        self.kind.inherit_from(&p.kind);
    }
}

#[cfg(test)]
mod completion_tests {
    include!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../evidence/foundation-completion-2026-10-13/base-assets/prefab_default_tests.rs"));
}
