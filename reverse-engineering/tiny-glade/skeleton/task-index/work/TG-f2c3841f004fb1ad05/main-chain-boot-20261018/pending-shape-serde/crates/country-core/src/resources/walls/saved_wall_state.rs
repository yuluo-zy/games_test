//! Original-B owned WallState input used by save/history consumers.
//! Premade shapes reuse the canonical named shape serializer and owned geometry.
use crate::{
    resources::{
        history::wall_edit::{WallAuthoritySnapshot, WallHistoryMetadata},
        roofs::geometry::surface::Curve2,
        walls::{
            inner_wall_state::WallState, publication_fields::WallShapeSnapshot,
            publication_source::RawWallPublicationSource,
        },
    },
    systems::wall::valid_enclosure::{IsEnclosed, ShapeParameters},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Native curve2_serialized_as_curve3 @141285770 keeps the serialized
/// coordinates and cached u/length; it does not recompute a new curve.
#[derive(Deserialize, Serialize)]
struct SavedCurve3 {
    points: Vec<[f32; 3]>,
    points_u: Vec<f32>,
    length: f32,
}

#[derive(Default, Deserialize, Serialize)]
enum SavedTerrainPolicy {
    #[default]
    FollowTerrain,
    ConstantMaxHeight,
}
impl SavedTerrainPolicy {
    fn native_tag(self) -> u8 {
        match self {
            Self::FollowTerrain => 0,
            Self::ConstantMaxHeight => 1,
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
enum SavedStructureType {
    #[default]
    Wall,
    Fence,
}
impl SavedStructureType {
    fn native_tag(self) -> u8 {
        match self {
            Self::Wall => 0,
            Self::Fence => 1,
        }
    }
}

fn default_arch_height() -> f32 {
    f32::from_bits(0x4033_3333)
}

/// Field-specific original @14085ebc0, distinct from the public WallStyle enum
/// derive: accepts bool and exactly these three string spellings.
fn deserialize_saved_style<'de, D: Deserializer<'de>>(d: D) -> Result<u8, D::Error> {
    struct Visitor;
    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = u8;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a boolean or Stone, StoneWithSkirts, HalfTimbered")
        }
        fn visit_bool<E: serde::de::Error>(self, value: bool) -> Result<u8, E> {
            Ok(u8::from(value))
        }
        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<u8, E> {
            match value {
                "Stone" => Ok(0),
                "StoneWithSkirts" => Ok(1),
                "HalfTimbered" => Ok(2),
                _ => Err(E::invalid_value(serde::de::Unexpected::Str(value), &self)),
            }
        }
    }
    d.deserialize_any(Visitor)
}

/// Required fields and defaults are from original object visitor
/// @1415bb1c0, default gates @1415bbba9..bbc0c. Serde retains the original
/// duplicate-field check when both an alias and canonical name are provided.
#[derive(Deserialize, Serialize)]
struct SavedWallState {
    curve: SavedCurve3,
    height: f32,
    #[serde(alias = "is_valid_enclosure")]
    is_enclosed: IsEnclosed,
    #[serde(default)]
    elevation: f32,
    #[serde(default = "default_arch_height")]
    arch_height: f32,
    #[serde(default)]
    arch_ty: SavedTerrainPolicy,
    #[serde(
        default,
        alias = "has_skirts",
        deserialize_with = "deserialize_saved_style",
        serialize_with = "serialize_saved_style"
    )]
    style: u8,
    #[serde(default)]
    structure_ty: SavedStructureType,
    #[serde(default)]
    clutter_version: u8,
    #[serde(default)]
    freehand_ty: SavedTerrainPolicy,
    #[serde(default)]
    plaster_peel: f32,
    #[serde(default)]
    removed_supports: bool,
}

fn serialize_saved_style<S: Serializer>(style: &u8, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(match style {
        0 => "Stone",
        1 => "StoneWithSkirts",
        2 => "HalfTimbered",
        _ => return Err(serde::ser::Error::custom("unrestored wall style tag")),
    })
}

/// Inverse owned-state projection. It is semantic JSON, not a dump of native
/// Vec pointers. Original curve2-as-curve3 writes zero in the middle channel.
pub fn serialize_wall_state<S: Serializer>(wall: &WallState, s: S) -> Result<S::Ok, S::Error> {
    let state = wall.snapshot();
    let policy = |tag| -> Result<SavedTerrainPolicy, S::Error> {
        match tag {
            0 => Ok(SavedTerrainPolicy::FollowTerrain),
            1 => Ok(SavedTerrainPolicy::ConstantMaxHeight),
            _ => Err(serde::ser::Error::custom("unrestored wall terrain policy")),
        }
    };
    let source = &state.source;
    let is_enclosed = match source.shape.tag() {
        2 => IsEnclosed::No,
        3 => IsEnclosed::Yes,
        0 | 1 => IsEnclosed::PremadeShape(ShapeParameters::from_snapshot(&source.shape)),
        _ => return Err(serde::ser::Error::custom("invalid wall shape tag")),
    };
    SavedWallState {
        curve: SavedCurve3 {
            points: source
                .curve
                .points
                .iter()
                .map(|&[x, z]| [x, 0.0, z])
                .collect(),
            points_u: source.curve.points_u.clone(),
            length: source.curve.length,
        },
        height: source.extra,
        is_enclosed,
        elevation: source.terrain_floor,
        arch_height: source.observed_path_scalar,
        arch_ty: policy(source.observed_source_flag_64)?,
        style: source.style,
        structure_ty: match source.clutter_version {
            0 => SavedStructureType::Wall,
            1 => SavedStructureType::Fence,
            _ => return Err(serde::ser::Error::custom("unrestored StructureType tag")),
        },
        clutter_version: state.metadata.source_69,
        freehand_ty: policy(
            state
                .metadata
                .source_66
                .ok_or_else(|| serde::ser::Error::custom("wall source+66 missing"))?,
        )?,
        plaster_peel: f32::from_bits(state.metadata.source_60_bits),
        removed_supports: state.metadata.source_67 != 0,
    }
    .serialize(s)
}

/// Produces the existing canonical owned wall, never a record containing
/// fabricated native Vec pointers. A caller can use this as WallState's serde
/// implementation once the original compatibility surface is accepted.
pub fn deserialize_wall_state<'de, D: Deserializer<'de>>(d: D) -> Result<WallState, D::Error> {
    let saved = SavedWallState::deserialize(d)?;
    let shape = match saved.is_enclosed {
        IsEnclosed::No | IsEnclosed::Yes => {
            let mut bytes = [0; 28];
            let tag: u32 = if matches!(saved.is_enclosed, IsEnclosed::No) {
                2
            } else {
                3
            };
            bytes[..4].copy_from_slice(&tag.to_le_bytes());
            WallShapeSnapshot::from_observed(bytes)
        }
        IsEnclosed::PremadeShape(params) => params.to_snapshot(),
    };
    let structure_ty = saved.structure_ty.native_tag();
    let freehand_ty = saved.freehand_ty.native_tag();
    Ok(WallState::from_snapshot(
        WallAuthoritySnapshot {
            source: RawWallPublicationSource {
                curve: Curve2 {
                    points: saved
                        .curve
                        .points
                        .into_iter()
                        .map(|[x, _, z]| [x, z])
                        .collect(),
                    points_u: saved.curve.points_u,
                    length: saved.curve.length,
                },
                curve_kind_tag: shape.tag(),
                shape,
                terrain_floor: saved.elevation,
                extra: saved.height,
                style: saved.style,
                observed_path_scalar: saved.arch_height,
                observed_source_flag_64: saved.arch_ty.native_tag(),
                // Existing canonical field name is wrong: this stores native +65
                // StructureType. Keep the proven bytes pending a coordinated rename.
                clutter_version: structure_ty,
                special: structure_ty != 0,
            },
            metadata: WallHistoryMetadata {
                source_60_bits: saved.plaster_peel.to_bits(),
                source_66: Some(freehand_ty),
                source_67: u8::from(saved.removed_supports),
                source_69: saved.clutter_version,
            },
        },
        freehand_ty,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_intro_wall_states_keep_owned_curves_and_legacy_style() {
        let rows: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-intro-wall-states-b.json"
        ))
        .unwrap();
        for row in rows.as_array().unwrap() {
            let state = deserialize_wall_state(row["state"].clone()).unwrap();
            assert!(!state.source().curve.points.is_empty());
            assert_eq!(
                state.source().curve.points.len(),
                state.source().curve.points_u.len()
            );
            assert_eq!(state.style(), 0);
            assert_eq!(state.freehand_ty(), 0);
            assert!(!state.is_special());
            assert_eq!(state.snapshot().metadata.source_69, 0);
        }
        let first = deserialize_wall_state(rows[0]["state"].clone()).unwrap();
        assert_eq!(first.source().curve.points.len(), 2);
        assert_eq!(
            first.source().curve.length.to_bits(),
            0.20000006258487701_f32.to_bits()
        );
        assert_eq!(
            first.source().curve.points[0],
            [-0.5528736114501953, -9.726827621459961]
        );
        assert_eq!(first.source().curve.points_u, vec![0.0, 1.0]);
        assert_eq!(first.height().to_bits(), 3.9000000953674316_f32.to_bits());
    }

    #[test]
    fn alias_and_canonical_style_are_duplicate_fields() {
        let mut rows: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/original-intro-wall-states-b.json"
        ))
        .unwrap();
        let mut value = rows[0]["state"].take();
        value["style"] = serde_json::json!("Stone");
        assert!(
            deserialize_wall_state(value)
                .unwrap_err()
                .to_string()
                .contains("duplicate field")
        );
    }
}
