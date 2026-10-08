//! Validated art configuration. Domain state never stores file paths or Handles.
use garden_generation::mesh::{GeometryProfile, MaterialKey};
use serde::Deserialize;
use std::{collections::BTreeMap, fmt};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ThemeFile {
    schema_version: u32,
    theme_id: String,
    revision: u32,
    geometry: GeometryFile,
    materials: BTreeMap<String, MaterialDefinition>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeometryFile {
    wall_thickness: f64,
    frame_width: f64,
    eave: f64,
    roof_pitch_degrees: f64,
    max_roof_rise: f64,
    uv_meters_per_tile: f64,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialDefinition {
    pub base_color_srgb: [u8; 3],
    pub roughness: f32,
}

#[derive(Debug, Clone)]
pub struct ArtCatalog {
    theme_id: String,
    revision: u32,
    geometry: GeometryProfile,
    materials: BTreeMap<MaterialKey, MaterialDefinition>,
}
#[derive(Debug)]
pub struct CatalogError(pub String);
impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for CatalogError {}

impl ArtCatalog {
    pub fn parse(json: &str) -> Result<Self, CatalogError> {
        let file: ThemeFile =
            serde_json::from_str(json).map_err(|e| CatalogError(e.to_string()))?;
        if file.schema_version != 1
            || file.revision == 0
            || file.theme_id.is_empty()
            || !file
                .theme_id
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(CatalogError("unsupported theme/schema/revision".into()));
        }
        let p = file.geometry;
        let geometry = GeometryProfile {
            wall_thickness: p.wall_thickness,
            frame_width: p.frame_width,
            eave: p.eave,
            roof_pitch_degrees: p.roof_pitch_degrees,
            max_roof_rise: p.max_roof_rise,
            uv_meters_per_tile: p.uv_meters_per_tile,
        };
        geometry
            .validate()
            .map_err(|e| CatalogError(e.to_string()))?;
        let mut materials = BTreeMap::new();
        for key in MaterialKey::ALL {
            let material = file
                .materials
                .get(key.name())
                .ok_or_else(|| CatalogError(format!("missing material: {}", key.name())))?;
            if !material.roughness.is_finite() || !(0.0..=1.0).contains(&material.roughness) {
                return Err(CatalogError(format!("invalid roughness: {}", key.name())));
            }
            materials.insert(key, material.clone());
        }
        if file.materials.len() != MaterialKey::ALL.len() {
            return Err(CatalogError("unknown material role".into()));
        }
        Ok(Self {
            theme_id: file.theme_id,
            revision: file.revision,
            geometry,
            materials,
        })
    }
    pub fn warm_stone() -> Self {
        Self::parse(include_str!("../../../assets/themes/warm-stone/theme.json"))
            .expect("embedded theme must pass validation")
    }
    pub fn theme_id(&self) -> &str {
        &self.theme_id
    }
    pub fn revision(&self) -> u32 {
        self.revision
    }
    pub fn geometry(&self) -> GeometryProfile {
        self.geometry
    }
    pub fn material(&self, key: MaterialKey) -> &MaterialDefinition {
        &self.materials[&key]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const JSON: &str = include_str!("../../../assets/themes/warm-stone/theme.json");
    #[test]
    fn shipped_palette_and_profile_are_valid() {
        let art = ArtCatalog::parse(JSON).unwrap();
        assert_eq!(art.theme_id(), "warm-stone");
        assert_eq!(
            art.material(MaterialKey::Stone).base_color_srgb,
            [216, 204, 180]
        );
    }
    #[test]
    fn bad_theme_cannot_reach_runtime() {
        let mut value: serde_json::Value = serde_json::from_str(JSON).unwrap();
        value["geometry"]["wall_thickness"] = 0.into();
        assert!(ArtCatalog::parse(&value.to_string()).is_err());
        let mut value: serde_json::Value = serde_json::from_str(JSON).unwrap();
        value["materials"].as_object_mut().unwrap().remove("roof");
        assert!(ArtCatalog::parse(&value.to_string()).is_err());
    }
}
