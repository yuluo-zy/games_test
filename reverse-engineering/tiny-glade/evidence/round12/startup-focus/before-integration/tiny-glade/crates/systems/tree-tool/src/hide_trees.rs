//! B hide_trees @14226a260: wall requests, then area requests. Each
//! resolved query clears the existing animation flag before checking a hit.
//! Placement comes from PrefabInstance; the name comes from PrefabInstanceData.
use bevy_ecs::prelude::*;
use country_core::components::{PrefabInstanceData, TreeAnimation, TreeComponent};
use country_core::resources::history::tree_edit::{DeleteTreeCmd, TreeEdit, TreeEditData};
use country_core::resources::history::{EditType, History};
use country_core::resources::prefab::PrefabInstance;
use country_core::resources::tree_storage::{
    HideTreesNextToWallCmd, HideTreesinAreaCmd, TreeHideArea, TreeStorage,
};
use country_core::resources::walls::{
    WallId,
    spatial_index::{GRID_SIDE, WORLD_SIZE, world_space_to_grid_coord_unclamped},
};
use country_core::systems::wall::wall_state::wall_spatial_hash::{
    WallSpatialHash, WallSpatialSegment,
};
use glam::Vec2;

const TREE_WALL_RADIUS: f32 = 0.4;

// B 1425df5e4..1425df66a. Preserve the ordered SSE operands, weighted
// interpolation, denominator floor and strict comparison, including NaNs.
fn segment_distance_to_tree(segment: &WallSpatialSegment, p: Vec2) -> f32 {
    let [a, b] = segment.endpoints;
    let d = [b[0] - a[0], b[1] - a[1]];
    let q = [p.x - a[0], p.y - a[1]];
    let dot = q[1] * d[1] + q[0] * d[0];
    let length_squared = d[1] * d[1] + d[0] * d[0];
    let denominator = if length_squared > 1e-20 {
        length_squared
    } else {
        1e-20
    };
    let ratio = dot / denominator;
    let t = if 0.0 > ratio { 0.0 } else { ratio };
    let t = if 1.0 < t { 1.0 } else { t };
    let x = p.x - (t * b[0] + (1.0 - t) * a[0]);
    let z = p.y - (t * b[1] + (1.0 - t) * a[1]);
    (z * z + x * x).sqrt()
}

fn wall_segment_hides_tree(segment: &WallSpatialSegment, p: Vec2) -> bool {
    TREE_WALL_RADIUS > segment_distance_to_tree(segment, p)
}

// This callback specialization walks grid cells directly. The generic
// query_with_circle sorts, deduplicates and uses <=; that is a different
// consumer contract and must not be used for the tree hide predicate.
fn wall_hides_tree(walls: &WallSpatialHash, wall: WallId, p: Vec2) -> bool {
    let half = WORLD_SIZE * 0.5;
    let min = [
        (p.x - TREE_WALL_RADIUS).max(-half),
        (p.y - TREE_WALL_RADIUS).max(-half),
    ];
    let max = [
        (p.x + TREE_WALL_RADIUS).min(half),
        (p.y + TREE_WALL_RADIUS).min(half),
    ];
    let lo = world_space_to_grid_coord_unclamped(min).map(|v| v.max(0));
    let hi = world_space_to_grid_coord_unclamped(max).map(|v| v.min(GRID_SIDE as i32 - 1));
    let cell_radius = ((WORLD_SIZE / GRID_SIDE as f32) * 0.5) * std::f32::consts::SQRT_2;
    for z in lo[1]..=hi[1] {
        for x in lo[0]..=hi[0] {
            let cell = z as usize * GRID_SIDE + x as usize;
            let center = walls.cell_center(cell);
            let dx = center[0] - p.x;
            let dz = center[1] - p.y;
            if !((TREE_WALL_RADIUS + cell_radius) > (dz * dz + dx * dx).sqrt()) {
                continue;
            }
            for segment in walls.segments_in_cell(cell) {
                if segment.wall == wall && wall_segment_hides_tree(segment, p) {
                    return true;
                }
            }
        }
    }
    false
}

fn area_hides_tree(command: &HideTreesinAreaCmd, p: glam::Vec3) -> bool {
    // UCOMISS/JA skip only for ordered > comparisons; NaNs reach geometry.
    if command.height_limit > 0.0 && command.height_limit - p.y > 4.0 {
        return false;
    }
    let p = Vec2::new(p.x, p.z);
    match command.area {
        TreeHideArea::Circle(circle) => circle
            .with_expanded_diameter(command.expansion * 2.0)
            .contains_point(p),
        TreeHideArea::Rectangle(rectangle) => rectangle
            .with_expanded_size(Vec2::splat(command.expansion))
            .contains_point(p),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn hide_trees<S: Send + Sync + 'static>(
    mut trees: Query<(
        &mut TreeAnimation,
        &PrefabInstanceData,
        &PrefabInstance,
        &TreeComponent,
    )>,
    storage: Res<TreeStorage>,
    mut area_cmds: EventReader<HideTreesinAreaCmd>,
    mut wall_cmds: EventReader<HideTreesNextToWallCmd>,
    walls: Res<WallSpatialHash>,
    mut deletes: EventWriter<DeleteTreeCmd>,
    mut history: ResMut<History<S>>,
) {
    for command in wall_cmds.read() {
        for (&index, entry) in storage.iter() {
            let Ok((mut animation, data, instance, tree)) = trees.get_mut(entry.entity) else {
                continue;
            };
            animation.hidden_this_frame = false;
            let p = instance.0.position;
            if wall_hides_tree(&walls, command.wall, Vec2::new(p.x, p.z)) {
                animation.hidden_this_frame = true;
                if command.finished {
                    deletes.write(DeleteTreeCmd { id: index.0 });
                    let edit = TreeEdit::Delete(TreeEditData::from_placement(
                        index,
                        &instance.0,
                        data.prefab_name.as_name_ref().to_string(),
                        tree.is_preplaced,
                    ));
                    history
                        .add(EditType::Tree(std::sync::Arc::new(edit)))
                        .expect("tree deletion requires an active history recording");
                }
            }
        }
    }
    for command in area_cmds.read() {
        for (&index, entry) in storage.iter() {
            let Ok((mut animation, data, instance, tree)) = trees.get_mut(entry.entity) else {
                continue;
            };
            animation.hidden_this_frame = false;
            if area_hides_tree(command, instance.0.position) {
                animation.hidden_this_frame = true;
                if command.finished {
                    deletes.write(DeleteTreeCmd { id: index.0 });
                    let edit = TreeEdit::Delete(TreeEditData::from_placement(
                        index,
                        &instance.0,
                        data.prefab_name.as_name_ref().to_string(),
                        tree.is_preplaced,
                    ));
                    history
                        .add(EditType::Tree(std::sync::Arc::new(edit)))
                        .expect("tree deletion requires an active history recording");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::event::Events;
    use country_core::resources::{
        history::transactions::ObservedHistoryLedger,
        placement_data::PlacementDataSsbo,
        render::mesh_atlas_library::PrefabName,
        tree_storage::{TreeStorageEntry, TreeStorageIndex},
    };
    use glam::Vec3;

    fn segment(a: [f32; 2], b: [f32; 2], wall: u64) -> WallSpatialSegment {
        WallSpatialSegment {
            endpoints: [a, b],
            source_u: [0., 1.],
            relative_heights: [0.; 2],
            terrain_heights: [0.; 2],
            wall: WallId(wall),
        }
    }

    // Original B callback arithmetic: 10 edge + 51 finite/tiny cases. The
    // independently reviewed raw probe/JSON is frozen in tree-wall-review.
    #[test]
    fn distance_matches_original_bits_and_ordered_threshold() {
        let fixtures: &[[u32; 7]] = &[
            [0xbf800000, 0x0, 0x3f800000, 0x0, 0x0, 0x0, 0x0],
            [
                0xbf800000, 0x0, 0x3f800000, 0x0, 0x0, 0x3ecccccc, 0x3ecccccc,
            ],
            [
                0xbf800000, 0x0, 0x3f800000, 0x0, 0x0, 0x3ecccccd, 0x3ecccccd,
            ],
            [
                0xbf800000, 0x0, 0x3f800000, 0x0, 0x0, 0x3eccccce, 0x3eccccce,
            ],
            [0x0, 0x0, 0x0, 0x0, 0x3dcccccd, 0x3dcccccd, 0x3e10d0c3],
            [0x0, 0x0, 0x0, 0x0, 0x3ecccccd, 0x0, 0x3ecccccd],
            [
                0x0, 0x0, 0x2b8cbccc, 0x0, 0x3dcccccd, 0x3dcccccd, 0x3e10d0c3,
            ],
            [
                0xbf800000, 0x0, 0x3f800000, 0x0, 0x7fc00000, 0x0, 0x7fc00000,
            ],
            [0x7fc00000, 0x0, 0x3f800000, 0x0, 0x0, 0x0, 0x7fc00000],
            [0xbf800000, 0x0, 0x7f800000, 0x0, 0x0, 0x0, 0xffc00000],
            [
                0xc1d3608d, 0x3e838244, 0x40f653df, 0xc095e80e, 0x412688c1, 0xbfc416ce, 0x40850c1b,
            ],
            [
                0x4009776a, 0xc20eedb6, 0xc23b02b8, 0xc1eb1a07, 0x42064fc4, 0xc263cdb3, 0x4217b023,
            ],
            [
                0xc18de684, 0xc1151245, 0x40cbc0a6, 0xc0ff4193, 0x422a98a4, 0x4251e4fb, 0x428d0154,
            ],
            [
                0x4103eb35, 0xc20ecefa, 0x42340627, 0x422ab71a, 0xc0894674, 0x420f05d1, 0x4226c3dc,
            ],
            [
                0x42388241, 0x4258710a, 0xc2021645, 0xc207b18a, 0x4211508d, 0xc1dbed7a, 0x423c3c45,
            ],
            [
                0xc1d89eaa, 0xc2588392, 0x41fbb7f5, 0xc1fc9324, 0xc21e5cf1, 0x4159bf90, 0x42876a0e,
            ],
            [
                0xc2478be6, 0x420883e2, 0xc248adb0, 0x42628ab5, 0xc230af13, 0xc22b924d, 0x429a7787,
            ],
            [
                0xc090eed5, 0x41ff303b, 0xbfc63c06, 0x41c84c36, 0xc1f7dbbc, 0x422bd814, 0x41e56282,
            ],
            [
                0xc20e9e67, 0xc12ca156, 0x42118abd, 0xc14dfa2f, 0xc253b0ed, 0x41d22f47, 0x42238d21,
            ],
            [
                0xc244e6df, 0xc12bcdc3, 0xc19691bb, 0xbfa3e163, 0xc22e78c5, 0x424b86f1, 0x4264b15e,
            ],
            [
                0x42160f78, 0x40b93b65, 0xc1bfaf6b, 0xc15aead3, 0xc1e76fe6, 0xc1317e31, 0x40b34bf2,
            ],
            [
                0xbfafc044, 0x41ef45a0, 0xc2422f39, 0x424fdea2, 0x423ebc62, 0x408694dc, 0x425d87f7,
            ],
            [
                0x40b6c68f, 0xc2449886, 0xc2071958, 0xc25a65c0, 0xc23b7ae9, 0x41d5ab76, 0x42a4b636,
            ],
            [
                0xc2110923, 0x41bafdc0, 0x4205a016, 0xc122ba83, 0x41acc19e, 0x412d6a9c, 0x415cdfba,
            ],
            [
                0x41645f73, 0x412e3777, 0x418a0ac4, 0xc22569b3, 0x426dab51, 0xc20d565d, 0x4229bf00,
            ],
            [
                0xc22fb59c, 0xc1faf5ed, 0x42187a00, 0x426556e3, 0x426aada0, 0xc22f53ac, 0x42a78f8a,
            ],
            [
                0xc15f5735, 0x41f34618, 0xc23b411a, 0xc1287e7b, 0x40fe1b57, 0xc1da50b1, 0x4254c547,
            ],
            [
                0xc1c0912c, 0xc042c284, 0xc22f7f36, 0x423ce55a, 0xc060352c, 0xc21e00ec, 0x42276fe1,
            ],
            [
                0xbf0597ce, 0xc182954c, 0x41829fb8, 0xc24fe0b4, 0xc17edec4, 0xc24e91e0, 0x41e83074,
            ],
            [
                0x4137d321, 0xbf8a3750, 0x420cc798, 0x417e0109, 0x4011a747, 0x41722bcc, 0x41946448,
            ],
            [
                0xc1ad4b52, 0x404da898, 0xc15a624f, 0xc0b25df9, 0xc1279233, 0xc26f9a6b, 0x4259ada5,
            ],
            [
                0xc20f9f6a, 0x4110925d, 0xc235c610, 0xc1785204, 0x4214c1c6, 0xbff94777, 0x4293d4b9,
            ],
            [
                0x420357b1, 0xc1a53426, 0xc26fed99, 0xc0db5e9d, 0x41f796e5, 0xc0036520, 0x4190f18d,
            ],
            [
                0xc186dd5c, 0x422c68c1, 0xc17fe45e, 0x424fe939, 0x41d87ced, 0xc2157c50, 0x42b75b43,
            ],
            [
                0x4183e480, 0x418f053d, 0x412c7861, 0x41b05a17, 0xc14b784d, 0x41eb3651, 0x41c4f863,
            ],
            [
                0x413c2878, 0x4207e899, 0x41faece9, 0x4146cd3e, 0x4070d573, 0xc2040998, 0x4254a5e3,
            ],
            [
                0xc242d2bf, 0xbfa3c3bd, 0xbf170586, 0xc175b5cd, 0xbed05359, 0x422d2598, 0x4261578d,
            ],
            [
                0x422e2d17, 0x423d5942, 0xc22fc22c, 0xc20a29f7, 0x422c06b6, 0x421d53f6, 0x40af46c0,
            ],
            [
                0xc20c5371, 0x4205fd7f, 0xc26071ed, 0xc150eca2, 0x4177d141, 0xc146ceeb, 0x4281fb7e,
            ],
            [
                0x422a10cd, 0xc209135a, 0xc2272a86, 0xc0f0b220, 0xc1dca138, 0x41e6e4d4, 0x421be979,
            ],
            [
                0x41b35d50, 0xc23a1444, 0xc21c5c62, 0xc25289f1, 0x425beb89, 0xc1f68c8c, 0x42109714,
            ],
            [
                0xc21bdeca, 0xc1c66235, 0xc100604a, 0xc223ab07, 0x41c551ad, 0x3f597cf0, 0x4254266a,
            ],
            [
                0xc196a716, 0x425e8753, 0x40b56aec, 0x41c2ad67, 0xc1a90f72, 0xc12a6616, 0x423048ca,
            ],
            [
                0x424baf3c, 0xc22096ed, 0x425dc989, 0xc1cd0df0, 0xc269de93, 0xc14886e4, 0x42e1a3f3,
            ],
            [
                0x4221675f, 0x414a1f44, 0xc09062d7, 0xc260b616, 0x40719f9c, 0x426f485a, 0x426ed02a,
            ],
            [
                0x421c6c5c, 0x42246605, 0xc255eb32, 0xc21dbb4e, 0xc1b92d96, 0x41f97771, 0x42057eeb,
            ],
            [
                0x410442df, 0xc1b75ee4, 0xc250c235, 0xc1bbd133, 0x418dadfb, 0x4159b03b, 0x4216e95b,
            ],
            [
                0x424befcf, 0x41389f0f, 0x41bfc64c, 0xc26f4fca, 0xc1b1b6b1, 0xc237d0a1, 0x4240e64f,
            ],
            [
                0xc1cb42b4, 0x42045a71, 0x4082c237, 0x41e926e1, 0x423cfd5b, 0xc15b503a, 0x427347f3,
            ],
            [
                0xc1287b68, 0xc2655431, 0x410c3ff2, 0x41834931, 0x42655731, 0xc1ada7e3, 0x42628c01,
            ],
            [
                0xc1ec14fe, 0x426f02f7, 0xc25b830b, 0xc1e8e8dc, 0x4173b35f, 0x425cd228, 0x4233e366,
            ],
            [
                0xc1f1c185, 0xc1138e42, 0x42050e72, 0xc22c5ab6, 0xc25e6089, 0xc1dfbd4e, 0x41fc61d8,
            ],
            [
                0x41ee2011, 0x3f204136, 0x423073ac, 0xc1e7569f, 0x41f013b4, 0x414929ef, 0x413f3010,
            ],
            [
                0x426887c0, 0x41f1e01c, 0x4153ad1b, 0xc2006098, 0xc100708a, 0xc2420bc9, 0x41d6dec7,
            ],
            [
                0xc23d97e1, 0x4183036e, 0x425ea608, 0x40c1a9aa, 0xc1cf494f, 0x41e1ef70, 0x415f2b45,
            ],
            [
                0x421196a6, 0x4264c366, 0x424ba68f, 0xc239d396, 0xc1648d4d, 0xc1f97b64, 0x4279cbc6,
            ],
            [
                0x4256adf7, 0xc2666c74, 0xc20745e7, 0x3ffbc050, 0xc1a2888c, 0x423a6fee, 0x423a8ee1,
            ],
            [
                0xc2329830, 0x40447f67, 0x423cb61b, 0xc16432dd, 0xc152272f, 0x4212df9f, 0x421ba34a,
            ],
            [
                0x0, 0x0, 0x2b8cbccc, 0x0, 0x2b8cbccc, 0x3dcccccd, 0x3dcccccd,
            ],
            [
                0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000, 0x0,
            ],
            [0x80000000, 0x0, 0x0, 0x80000000, 0x80000000, 0x0, 0x0],
        ];
        for bits in fixtures {
            let v = bits.map(f32::from_bits);
            let edge = segment([v[0], v[1]], [v[2], v[3]], 7);
            let point = Vec2::new(v[4], v[5]);
            let distance = segment_distance_to_tree(&edge, point);
            if v[6].is_nan() {
                assert!(distance.is_nan());
            } else {
                assert_eq!(distance.to_bits(), bits[6], "{bits:x?}");
            }
            assert_eq!(
                wall_segment_hides_tree(&edge, point),
                TREE_WALL_RADIUS > v[6]
            );
        }
    }

    fn world() -> (World, Schedule) {
        let mut world = World::new();
        world.init_resource::<TreeStorage>();
        world.init_resource::<WallSpatialHash>();
        world.init_resource::<Events<HideTreesinAreaCmd>>();
        world.init_resource::<Events<HideTreesNextToWallCmd>>();
        world.init_resource::<Events<DeleteTreeCmd>>();
        let mut history = History::<()>::from_observed(ObservedHistoryLedger {
            entries: vec![],
            cursor: -1,
            undo_anchor: -1,
            resolved_snapshots: vec![],
            observed_count_58: 0,
            mutation_nonce: 0,
        });
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        let mut schedule = Schedule::default();
        schedule.add_systems(hide_trees::<()>);
        (world, schedule)
    }

    fn tree(world: &mut World, index: u64, position: Vec3) -> Entity {
        let instance = PrefabInstance(PlacementDataSsbo {
            position,
            scale: 0.75,
            rotation_angle: 123.0,
            id: -31,
            custom0: 91,
            custom1: 71,
            ..Default::default()
        });
        let entity = world
            .spawn((
                TreeAnimation::preplaced(),
                instance,
                PrefabInstanceData {
                    prefab_name: PrefabName::new("test/tree"),
                    ..Default::default()
                },
                TreeComponent {
                    id: 999,
                    is_preplaced: true,
                },
            ))
            .id();
        // Deliberately stale cache: the original consumer queries placement.
        world.resource_mut::<TreeStorage>().insert(
            TreeStorageIndex(index),
            TreeStorageEntry {
                entity,
                position: Vec3::splat(60.),
            },
        );
        entity
    }

    #[test]
    fn actual_wall_drag_release_records_live_placement_and_wide_storage_index() {
        let (mut world, mut schedule) = world();
        let near = tree(&mut world, 0x1_0000_0011, Vec3::new(0., 3., 0.2));
        let boundary = tree(&mut world, 9, Vec3::new(0., 3., 0.4));
        world
            .get_mut::<TreeAnimation>(boundary)
            .unwrap()
            .hidden_this_frame = true;
        world
            .resource_mut::<WallSpatialHash>()
            .add_segment(segment([-1., 0.], [1., 0.], 7));
        world
            .resource_mut::<Events<HideTreesNextToWallCmd>>()
            .send(HideTreesNextToWallCmd {
                wall: WallId(7),
                finished: false,
            });
        schedule.run(&mut world);
        assert!(world.get::<TreeAnimation>(near).unwrap().hidden_this_frame);
        assert!(
            !world
                .get::<TreeAnimation>(boundary)
                .unwrap()
                .hidden_this_frame
        );
        assert!(
            world
                .resource::<History<()>>()
                .pending()
                .unwrap()
                .edits
                .is_empty()
        );
        assert!(world.resource::<Events<DeleteTreeCmd>>().is_empty());
        world
            .resource_mut::<Events<HideTreesNextToWallCmd>>()
            .send(HideTreesNextToWallCmd {
                wall: WallId(7),
                finished: true,
            });
        schedule.run(&mut world);
        let history = world.resource::<History<()>>();
        let edits = &history.pending().unwrap().edits;
        assert_eq!(edits.len(), 1);
        let EditType::Tree(edit) = &edits[0] else {
            panic!()
        };
        assert_eq!(edit.id(), 0x1_0000_0011);
        let TreeEdit::Delete(data) = &**edit else {
            panic!()
        };
        assert_eq!(data.position, Vec3::new(0., 3., 0.2));
        assert_eq!(data.scale, 0.75);
        assert_eq!(data.rotation_degrees, 123.0);
        assert_eq!(data.id, -31);
        assert_eq!(data.prefab_name, "test/tree");
        assert!(data.is_preplaced);
        let events = world.resource::<Events<DeleteTreeCmd>>();
        let mut cursor = events.get_cursor();
        assert_eq!(
            cursor.read(events).map(|e| e.id).collect::<Vec<_>>(),
            [0x1_0000_0011]
        );
    }

    #[test]
    fn area_follows_wall_and_each_resolved_command_clears_previous_hide() {
        let (mut world, mut schedule) = world();
        let entity = tree(&mut world, 4, Vec3::new(0., 0., 0.6));
        let area = HideTreesinAreaCmd {
            area: TreeHideArea::Circle(utils::geometry::Circle2d::new(Vec2::ZERO, 0.5)),
            height_limit: 4.,
            expansion: 0.2,
            finished: false,
        };
        world
            .resource_mut::<Events<HideTreesinAreaCmd>>()
            .send(area);
        schedule.run(&mut world);
        assert!(
            world
                .get::<TreeAnimation>(entity)
                .unwrap()
                .hidden_this_frame
        );
        world
            .resource_mut::<Events<HideTreesinAreaCmd>>()
            .send(HideTreesinAreaCmd {
                height_limit: 4.0001,
                ..area
            });
        schedule.run(&mut world);
        assert!(
            !world
                .get::<TreeAnimation>(entity)
                .unwrap()
                .hidden_this_frame
        );
        assert!(area_hides_tree(
            &HideTreesinAreaCmd {
                height_limit: f32::NAN,
                ..area
            },
            Vec3::new(0., 0., 0.6)
        ));
        assert!(!area_hides_tree(&area, Vec3::new(0., 0., f32::NAN)));
        // Final area command wins over a preceding wall request, regardless
        // of insertion order across the two event resources.
        world
            .resource_mut::<WallSpatialHash>()
            .add_segment(segment([-1., 0.6], [1., 0.6], 7));
        world
            .resource_mut::<Events<HideTreesNextToWallCmd>>()
            .send(HideTreesNextToWallCmd {
                wall: WallId(7),
                finished: false,
            });
        world
            .resource_mut::<Events<HideTreesinAreaCmd>>()
            .send(HideTreesinAreaCmd {
                height_limit: 4.0001,
                ..area
            });
        schedule.run(&mut world);
        assert!(
            !world
                .get::<TreeAnimation>(entity)
                .unwrap()
                .hidden_this_frame
        );
    }
    #[test]
    fn paths_clear_trees_outside_eraser_mode_and_consume_all_events_once() {
        use country_core::{
            resources::{
                cursor_terrain_raycast::CursorTerrainRaycast, path_stroke::PathDrawEvent,
                walls::EraserSizeMultiplier,
            },
            systems::{
                mode_manager::AppMode,
                ui::{
                    resources::ui_state::UiState, signifiers::signifier_create::UiSignifierStream,
                },
            },
        };
        let (mut world, _) = world();
        tree(&mut world, 81, Vec3::new(-3., 0., 0.));
        tree(&mut world, 82, Vec3::new(3., 0., 0.));
        world.insert_resource(AppMode::Path);
        world.insert_resource(UiState::at_startup());
        world.init_resource::<CursorTerrainRaycast>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(EraserSizeMultiplier(1.));
        world.init_resource::<Events<PathDrawEvent>>();
        for (x, observed_flag) in [(-3., false), (3., false), (-3., true)] {
            world
                .resource_mut::<Events<PathDrawEvent>>()
                .send(PathDrawEvent {
                    position: Vec3::new(x, 0., 0.),
                    radius: 0.5,
                    observed_flag,
                });
        }
        let mut schedule = Schedule::default();
        schedule.add_systems(crate::ui_delete_tree::ui_delete_tree::<()>);
        schedule.run(&mut world);
        let events = world.resource::<Events<DeleteTreeCmd>>();
        let mut cursor = events.get_cursor();
        assert_eq!(cursor.read(events).map(|e| e.id).collect::<Vec<_>>(), [82]);
        assert_eq!(
            world
                .resource::<History<()>>()
                .pending()
                .unwrap()
                .edits
                .len(),
            1
        );
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<History<()>>()
                .pending()
                .unwrap()
                .edits
                .len(),
            1
        );
    }
    #[test]
    #[should_panic(expected = "TreeStorage entry is missing the tree placement query components")]
    fn path_tree_deletion_preserves_original_query_failure() {
        use country_core::{
            resources::{
                cursor_terrain_raycast::CursorTerrainRaycast, path_stroke::PathDrawEvent,
                walls::EraserSizeMultiplier,
            },
            systems::{
                mode_manager::AppMode,
                ui::{
                    resources::ui_state::UiState, signifiers::signifier_create::UiSignifierStream,
                },
            },
        };
        let (mut world, _) = world();
        let invalid = world.spawn_empty().id();
        world.resource_mut::<TreeStorage>().insert(
            TreeStorageIndex(1),
            TreeStorageEntry {
                entity: invalid,
                position: Vec3::ZERO,
            },
        );
        world.insert_resource(AppMode::Path);
        world.insert_resource(UiState::at_startup());
        world.init_resource::<CursorTerrainRaycast>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(EraserSizeMultiplier(1.));
        world.init_resource::<Events<PathDrawEvent>>();
        world
            .resource_mut::<Events<PathDrawEvent>>()
            .send(PathDrawEvent {
                position: Vec3::ZERO,
                radius: 0.5,
                observed_flag: false,
            });
        let mut schedule = Schedule::default();
        schedule.add_systems(crate::ui_delete_tree::ui_delete_tree::<()>);
        schedule.run(&mut world);
    }
}
