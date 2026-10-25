//! Original B140862270: legacy SpawnPreplacedTrees event → owned assets,
//! tree identity/components → the existing full prefab spawn lifecycle.
use crate::{
    components::{ConvertedPreplacedTree, TreeAnimation, TreeComponent, TreeMoveBend},
    resources::{
        history::tree_edit::SpawnPreplacedTrees,
        placement_data::PlacementDataSsbo,
        session::ActiveSession,
        tree_storage::{TreeStorage, TreeStorageEntry, TreeStorageIndex},
    },
    systems::{audio::tree_audio::TreeAmbience, prefab::SpawnPrefabCmd},
};
use asset::prefab::PrefabName;
use bevy_ecs::prelude::*;
use std::path::{Path, PathBuf};

pub const TREES_FILE_NAME: &str = "trees.json";
fn one() -> f32 {
    1.0
}
/// Original clearing::PlacementConfigEntry visitor141416520: four fields.
#[derive(Clone, Debug, serde::Deserialize)]
pub struct PreplacedTreeGroup {
    pub prefab: String,
    pub placements: String,
    #[serde(default)]
    pub particle_emitter: Option<String>,
    #[serde(default = "one")]
    pub billboard_expansion_scale: f32,
}
/// Use the full original48B placement decoder, including both angle aliases
/// and packed animation defaults. No reduced parallel asset record exists.
pub type TreePlacement = PlacementDataSsbo;
pub fn trees_json_path(assets_root: impl AsRef<Path>, theme: &str) -> PathBuf {
    assets_root
        .as_ref()
        .join("assets")
        .join("glade")
        .join(theme)
        .join(TREES_FILE_NAME)
}
pub fn load_tree_groups(path: impl AsRef<Path>) -> anyhow::Result<Vec<PreplacedTreeGroup>> {
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}
pub fn load_tree_placements(
    glade_dir: impl AsRef<Path>,
    group: &PreplacedTreeGroup,
) -> anyhow::Result<Vec<TreePlacement>> {
    // Original RelativePath::to_path, not platform-dependent raw Path::join.
    let path = relative_path::RelativePath::new(&group.placements).to_path(glade_dir);
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}

/// Update + in_state(Game), registered by original add_save_load_systems.
/// Both readers of SpawnPreplacedTrees see the same complete event batch.
pub fn spawn_preplaced_trees(
    session: Res<ActiveSession>,
    mut commands: Commands,
    mut requests: EventReader<SpawnPreplacedTrees>,
    mut storage: ResMut<TreeStorage>,
    mut prefabs: EventWriter<SpawnPrefabCmd>,
) -> bevy_ecs::error::Result {
    if requests.read().count() == 0 {
        return Ok(());
    }
    log::info!("Converting pre-placed trees");
    // ActiveSession::assets_root already returns assets/glade/<theme>.
    // Native builds that path once (14086243e..455), then joins trees.json.
    let glade_dir = session.assets_root();
    let groups = load_tree_groups(glade_dir.join(TREES_FILE_NAME))?;
    let mut accepted = 0u64;
    for group in groups {
        // Native exactly compares the18-byte prefab name before opening its
        // placement file; bushes stay in the original forest asset system.
        if group.prefab == "clearing/tree/bush" {
            continue;
        }
        let placements = load_tree_placements(&glade_dir, &group)?;
        let name = PrefabName::new(&group.prefab);
        for placement in placements {
            if !(placement.position.x.abs() < 65.0 && placement.position.z.abs() < 65.0) {
                continue;
            }
            let id = 100001u64.wrapping_add(accepted);
            let entity = commands
                .spawn((
                    TreeComponent {
                        id,
                        is_preplaced: true,
                    },
                    ConvertedPreplacedTree(name.clone()),
                    TreeAnimation::preplaced(),
                    TreeMoveBend {
                        position: glam::Vec2::new(placement.position.x, placement.position.z),
                    },
                    TreeAmbience {
                        position: placement.position + glam::Vec3::new(0.0, 10.0, 0.0),
                    },
                ))
                .id();
            let mut flags = [0u8; 16];
            flags[0..4].copy_from_slice(&placement.custom1.to_le_bytes());
            flags[4..8].copy_from_slice(&placement.anim_t_packed.to_le_bytes());
            flags[8..12].copy_from_slice(&placement.bend_vector.x.to_le_bytes());
            flags[12..16].copy_from_slice(&placement.bend_vector.y.to_le_bytes());
            prefabs.write(SpawnPrefabCmd {
                prefab_name: Some(crate::resources::render::mesh_atlas_library::PrefabName::new(
                    &group.prefab,
                )),
                entity: Some(entity),
                position: placement.position,
                scale: placement.scale,
                rotation: placement.rotation_angle,
                variant: placement.id as u32,
                // Original overrides placement+18 with group billboard scale.
                one: group.billboard_expansion_scale,
                flags,
                mode: placement.pad0 as u32,
            });
            assert!(
                storage
                    .insert(
                        TreeStorageIndex(id),
                        TreeStorageEntry {
                            entity,
                            position: placement.position
                        }
                    )
                    .is_none(),
                "pre-placed tree identity already exists"
            );
            accepted = accepted.wrapping_add(1);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_placement_preserves_full_asset_fields_and_group_scale() {
        let group: PreplacedTreeGroup = serde_json::from_str(r#"{"prefab":"clearing/tree/mid1","placements":"trees/a.json","billboard_expansion_scale":2.2}"#).unwrap();
        assert_eq!(group.billboard_expansion_scale, 2.2);
        let p: TreePlacement = serde_json::from_str(r#"{"position":[1,2,3],"scale":1.06,"rotation_degrees":110.3,"id":52,"custom0":7,"custom1":9,"bend_vector":[0.5,1.5]}"#).unwrap();
        assert_eq!(
            (p.id, p.custom0, p.custom1, p.anim_t_packed),
            (52, 7, 9, 0x3c003c00)
        );
        assert_eq!(p.bend_vector, glam::Vec2::new(0.5, 1.5));
    }
}
