use crate::inheritable::{Inherit, Inheritable, deserialize_inheritable_option};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
/// Prefab vectors use RON sequence syntax; texture resize uses tuples.
fn deserialize_float3<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> std::result::Result<Inheritable<[f32; 3]>, D::Error> {
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
        // Parent parts may still be incomplete directory defaults.
        for part in self.parts.values_mut() {
            part.inherit_from(&PrefabPartConfig::resolved_defaults());
        }
    }
}
impl PrefabConfig {
    /// Check required fields after inheritance, including enabled RT/audio objects.
    /// Deleted parts are removal directives and do not require render data.
    pub fn validate(&self) -> Result<()> {
        for (name, part) in &self.parts {
            validate_part(part).with_context(|| format!("part {name:?}"))?;
        }
        Ok(())
    }
}

fn required<'a, T>(field: &'a Inheritable<T>, name: &'static str) -> Result<&'a T> {
    field.try_as_concrete().with_context(|| name)
}

fn validate_part(part: &PrefabPartConfig) -> Result<()> {
    if *required(&part.delete, "delete")? {
        return Ok(());
    }
    let render = required(&part.render, "render")?;
    required(&render.mesh, "render.mesh")?;
    required(&render.shader, "render.shader")?;
    required(&render.mesh_processor, "render.mesh_processor")?;
    if let Some(rt) = required(&part.rt, "rt")? {
        required(&rt.mesh, "rt.mesh")?;
        required(&rt.mesh_scale, "rt.mesh_scale")?;
        required(&rt.summer_albedo, "rt.summer_albedo")?;
        required(&rt.autumn_albedo, "rt.autumn_albedo")?;
        required(&rt.winter_albedo, "rt.winter_albedo")?;
        required(&rt.semantic, "rt.semantic")?;
    }
    if let Some(audio) = required(&part.audio, "audio")? {
        let trigger = required(&audio.trigger, "audio.trigger")?;
        required(&trigger.shape, "audio.trigger.shape")?;
        required(&trigger.shape_scale, "audio.trigger.shape_scale")?;
        required(&trigger.kind, "audio.trigger.kind")?;
    }
    Ok(())
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
    fn inherit_from(&mut self, parent: &Self) {
        self.delete.inherit_from(&parent.delete);
        self.render.inherit_from(&parent.render);
        self.rt.inherit_from(&parent.rt);
        self.audio.inherit_from(&parent.audio);
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
    pub fn concrete_default() -> Self {
        Self {
            mesh: Inheritable::MissingDefault,
            shader: Inheritable::MissingDefault,
            mesh_processor: Inheritable::Value(None),
        }
    }
}
impl Inherit for PrefabRenderConfig {
    fn inherit_from(&mut self, parent: &Self) {
        self.mesh.inherit_from(&parent.mesh);
        self.mesh_processor.inherit_from(&parent.mesh_processor);
        self.shader.inherit_from(&parent.shader);
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
    /// Required mesh, unit scale/albedos and no special semantic.
    pub fn concrete_default() -> Self {
        Self {
            mesh: Inheritable::MissingDefault,
            mesh_scale: Inheritable::Value([1.0; 3]),
            summer_albedo: Inheritable::Value([1.0; 3]),
            autumn_albedo: Inheritable::Value([1.0; 3]),
            winter_albedo: Inheritable::Value([1.0; 3]),
            semantic: Inheritable::Value(RtMeshSemantic::None),
        }
    }
}
impl Inherit for PrefabRtConfig {
    fn inherit_from(&mut self, parent: &Self) {
        self.mesh.inherit_from(&parent.mesh);
        self.mesh_scale.inherit_from(&parent.mesh_scale);
        self.summer_albedo.inherit_from(&parent.summer_albedo);
        self.autumn_albedo.inherit_from(&parent.autumn_albedo);
        self.winter_albedo.inherit_from(&parent.winter_albedo);
        self.semantic.inherit_from(&parent.semantic);
    }
    fn finish_inheritance(&mut self) {
        self.inherit_from(&Self::concrete_default());
    }
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct PrefabAudioConfig {
    pub trigger: Inheritable<AudioTriggerConfig>,
}
impl PrefabAudioConfig {
    pub fn concrete_default() -> Self {
        Self {
            trigger: Inheritable::Value(AudioTriggerConfig::concrete_default()),
        }
    }
}
impl Inherit for PrefabAudioConfig {
    fn inherit_from(&mut self, parent: &Self) {
        self.trigger.inherit_from(&parent.trigger);
    }
    fn finish_inheritance(&mut self) {
        // Preserve unresolved required fields for the fallible validation boundary.
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
    /// Required shape/kind and unit scale.
    pub fn concrete_default() -> Self {
        Self {
            shape: Inheritable::MissingDefault,
            shape_scale: Inheritable::Value([1.0; 3]),
            kind: Inheritable::MissingDefault,
        }
    }
}
impl Inherit for AudioTriggerConfig {
    fn inherit_from(&mut self, parent: &Self) {
        self.shape.inherit_from(&parent.shape);
        self.shape_scale.inherit_from(&parent.shape_scale);
        self.kind.inherit_from(&parent.kind);
    }
}

#[cfg(test)]
mod completion_tests {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/oracle/foundation-completion-2026-10-13/base-assets/prefab_default_tests.rs"
    ));
}
