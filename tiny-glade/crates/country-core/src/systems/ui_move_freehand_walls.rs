//! B 1409781d0：自由墙移动。保留真实手柄、源曲线缓存与逐帧副作用。
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use crate::{
    input::action_set::ActionSetStates,
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        history::{
            EditType, History,
            wall_edit::{CompoundEditBuilder, WallCurveHistory, WallHistoryEdit, WallValueEdit},
        },
        roofs::geometry::surface::Curve2,
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        tree_storage::HideTreesNextToWallCmd,
        walls::{
            InnerWalls, WallId,
            decorator::DecoratorInfo,
            decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            public_walls::PublicWalls,
        },
        window::AudioCursorMoved,
    },
    systems::{
        audio::RequestWallMaterialAudio,
        camera_rig::MainCamera,
        mode_manager::{AppMode, EditSubject},
        ui::{resources::ui_state::UiState, signifiers::UiSignifierStream},
        ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
        ui_move_shapes::move_gizmo_and_trigger_vol,
    },
};

/// 原 Vec 元素12B：装饰ID及相对手柄中心的水平偏移。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveShapeOffset {
    pub id: u32,
    pub offset: Vec2,
}

/// 原事件40B；EventId由Bevy管理，不是业务kind。
#[derive(Event, Clone, Debug)]
pub struct OnShapeMoved {
    pub affected: Vec<MoveShapeOffset>,
    pub wall: WallId,
    pub offset: Vec2,
}

/// B Local：装饰锚点、拖拽前完整曲线及光标抓取偏移。
#[derive(Default)]
pub struct MoveWallCache {
    pub affected: Vec<MoveShapeOffset>,
    pub before: Option<Curve2>,
    pub cursor_offset: Vec2,
}

pub(crate) fn archive_freehand(
    archivist: &mut LiveDecoratorArchivist,
    storage: &DecoratorStorage,
    resources: &DecoratorArchivalResources<'_>,
    wall: WallId,
    reason: &str,
) {
    // 原归档数组同时包含墙和地面；必须经过活碰撞体归档入口。
    archivist.archive_current_decorators_in_storages_with_resources(
        &[
            DecoratorStorageAddr::Wall(wall),
            DecoratorStorageAddr::Terrain,
        ],
        storage,
        resources,
        reason,
    );
    archivist.decorator_cull_allow_wall(wall, storage);
    if let Some(terrain) = storage.get(DecoratorStorageAddr::Terrain) {
        archivist
            .culling
            .allowed_ids
            .extend(terrain.decorators().keys().copied());
    }
}

pub(crate) fn anchor_xz(info: &DecoratorInfo) -> Vec2 {
    Vec2::new(
        f32::from_bits((info.dst_payload >> 32) as u32),
        f32::from_bits((info.u18 >> 32) as u32),
    )
}

pub(crate) fn wall_anchor(info: &DecoratorInfo) -> bool {
    match info.dst_tag() {
        2 => false,
        3 => panic!("自由墙装饰不能附着楼梯"),
        4 => unreachable!("自由墙装饰目的地无效"),
        _ => true,
    }
}

pub(crate) fn finish_curve_history(
    history: &mut History,
    wall: WallId,
    before: Curve2,
    after: &Curve2,
) {
    let saved = |curve: &Curve2| WallCurveHistory {
        points: curve.points.clone(),
        points_u: curve.points_u.clone(),
        length: curve.length,
    };
    let mut builder = CompoundEditBuilder::default();
    // 原完整保留每个ConcurrentEdits组，不按几何相等过滤release。
    builder.new_edit_sequence();
    builder.next(WallHistoryEdit::MorphFreehandWall(WallValueEdit {
        wall_index: wall.0,
        before: saved(&before),
        after: saved(after),
    }));
    history
        .add(EditType::Wall(Arc::new(builder.build())))
        .unwrap_or_else(|error| panic!("自由墙历史记录失败: {error:?}"));
}

fn cursor_xz(cursor: &CursorTerrainRaycast) -> Vec2 {
    let p = cursor.position();
    Vec2::new(p.x, p.z)
}

/// 原14097842a..53e：曲线中点与面向相机的单位法线，含原表达式顺序。
fn move_anchor(curve: &Curve2, eye: Vec3) -> (Vec2, Vec2, Vec2) {
    let midpoint = Vec2::from_array(curve.pos_at_u(0.5));
    let (segment, _) = curve.coord_at_u(0.5);
    let first = Vec2::from_array(curve.points[segment]);
    let next = Vec2::from_array(curve.points[segment + 1]);
    let tangent = next - first;
    let inverse = 1.0 / (tangent.y * tangent.y + tangent.x * tangent.x).sqrt();
    let normal = Vec2::new(-tangent.y * inverse, tangent.x * inverse);
    let inverse_eye = 1.0 / (eye.z * eye.z + eye.x * eye.x).sqrt();
    let dot = inverse_eye * eye.z * normal.y + inverse_eye * eye.x * normal.x;
    let offset = if dot < 0.0 { normal } else { -normal };
    (midpoint, midpoint + offset, normal)
}

/// 对源曲线做平移时，原版保留points_u/length缓存，不重新参数化。
fn translate_curve(curve: &Curve2, cursor: Vec2, offset: Vec2) -> Curve2 {
    let first = curve.points.first().expect("自由墙曲线不能为空");
    let dx = (cursor.x - offset.x) - first[0];
    let dz = (cursor.y - offset.y) - first[1];
    let mut result = curve.clone();
    translate_points(&mut result.points, Vec2::new(dx, dz));
    result
}

fn translate_points(points: &mut [[f32; 2]], delta: Vec2) {
    for point in points {
        point[0] += delta.x;
        point[1] += delta.y;
    }
}

/// 使用原16组SystemParam，展开后的22个资源/事件仍由Bevy管理。
pub fn ui_move_freehand_walls(
    mut cache: Local<MoveWallCache>,
    mode: Res<AppMode>,
    public: Res<PublicWalls>,
    mut inner: ResMut<InnerWalls>,
    (cursor, mut audio_moved): (Res<CursorTerrainRaycast>, EventReader<AudioCursorMoved>),
    (mut storage, mut archivist): (ResMut<DecoratorStorage>, ResMut<LiveDecoratorArchivist>),
    ui: Res<UiState>,
    mut history: ResMut<History>,
    mut audio: EventWriter<RequestWallMaterialAudio>,
    archival: DecoratorArchivalResources,
    actions: Res<ActionSetStates>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    camera: Res<MainCamera>,
    mut stream: ResMut<UiSignifierStream>,
    (border, mut display, mut feedback): (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
    (mut moved, mut trees, mut pins): (
        EventWriter<OnShapeMoved>,
        EventWriter<HideTreesNextToWallCmd>,
        EventWriter<ShowPinnedDecoClutter>,
    ),
) {
    let AppMode::Editing(EditSubject::Wall(id)) = *mode else {
        return;
    };
    let wall = WallId(id);
    let published = public.get(wall).expect("移动自由墙的公开墙不存在");
    if published.shape().tag() < 2 {
        return;
    }
    let curve = published.simplified_curve_2d();
    let (midpoint, anchor, normal) = move_anchor(curve, camera.eye_forward());
    let ground = terrain.sample_at_world_xz(anchor.x, anchor.y);
    let elevation = published.authority.elevation();
    // MAXSS后显式修复地形NaN，与原140978563..579相同。
    let height = if ground.is_nan() || elevation > ground {
        elevation
    } else {
        ground
    };
    let position = Vec3::new(anchor.x, height, anchor.y);
    let rotation = ::utils::xz_projective_rotation_bad(Vec3::new(normal.x, 0.0, normal.y));
    let widget = move_gizmo_and_trigger_vol(
        &position,
        &rotation,
        &actions,
        &ui,
        wall,
        published,
        ground,
        &mut stream,
    );
    match widget.dragged() {
        2 => return,
        3 => {
            archive_freehand(
                &mut archivist,
                &storage,
                &archival,
                wall,
                "move_freehand_wall",
            );
            cache.affected.clear();
            if let Some(bucket) = storage.wall_decorators_mut(wall) {
                for (&id, info) in bucket.decorators() {
                    if wall_anchor(info) {
                        cache.affected.push(MoveShapeOffset {
                            id,
                            offset: anchor_xz(info) - midpoint,
                        });
                    }
                }
            }
            cache.before = Some(curve.clone());
            cache.cursor_offset = cursor_xz(&cursor) - Vec2::from_array(curve.points[0]);
        }
        4 => {
            let source = inner.get(wall).expect("移动自由墙的权威墙不存在").source();
            let next = translate_curve(&source.curve, cursor_xz(&cursor), cache.cursor_offset);
            let inside = next
                .points
                .iter()
                .all(|point| border.is_pos_inside(Vec2::from_array(*point)));
            if inside {
                moved.write(OnShapeMoved {
                    affected: cache.affected.clone(),
                    wall,
                    offset: midpoint,
                });
                inner.update_wall_state_curve(wall, next);
            } else {
                display.write(DisplayOutOfBorder);
                feedback.write(FeedbackInputOutOfBorder);
            }
            trees.write(HideTreesNextToWallCmd {
                wall,
                finished: false,
            });
            pins.write(ShowPinnedDecoClutter::for_wall(wall));
            if !inside {
                return;
            }
        }
        _ => {
            moved.write(OnShapeMoved {
                affected: cache.affected.clone(),
                wall,
                offset: midpoint,
            });
            if let Some(before) = cache.before.take() {
                finish_curve_history(&mut history, wall, before, curve);
            }
            trees.write(HideTreesNextToWallCmd {
                wall,
                finished: true,
            });
        }
    }
    if audio_moved.read().last().is_some() {
        audio.write(RequestWallMaterialAudio {
            id: wall.0,
            position,
            kind: 1,
            flag: 0,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_scalar_translation_fixture_matches_bits() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-freehand-native.json"
        )))
        .unwrap();
        let read_pair = |pair: &serde_json::Value| {
            [
                f32::from_bits(pair[0].as_u64().unwrap() as u32),
                f32::from_bits(pair[1].as_u64().unwrap() as u32),
            ]
        };
        for case in cases["translations"].as_array().unwrap() {
            let mut points: Vec<[f32; 2]> = case["points"]
                .as_array()
                .unwrap()
                .iter()
                .map(read_pair)
                .collect();
            translate_points(&mut points, Vec2::from_array(read_pair(&case["delta"])));
            let bits: Vec<u32> = points.iter().flatten().map(|x| x.to_bits()).collect();
            let expected: Vec<u32> = case["output"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap() as u32)
                .collect();
            assert_eq!(bits, expected);
        }
    }
    #[test]
    fn translation_keeps_original_cache_and_grab_offset() {
        let source = Curve2 {
            points: vec![[1., 2.], [3., 5.], [8., 7.]],
            points_u: vec![0., 0.271, 1.],
            length: 123.4,
        };
        let result = translate_curve(&source, Vec2::new(6., 9.), Vec2::new(2., 3.));
        assert_eq!(result.points, vec![[4., 6.], [6., 9.], [11., 11.]]);
        assert_eq!(result.points_u, source.points_u);
        assert_eq!(result.length.to_bits(), source.length.to_bits());
    }

    #[test]
    fn real_world_move_edit_border_release_and_unload() {
        use crate::{
            resources::{
                history::MaybePendingSnapshotId,
                walls::{
                    WallState, public_walls::WallPublicationDirty,
                    publication_source::TerrainHeightGrid512,
                },
            },
            systems::{
                ui::resources::ui_state::{UiInteraction, WidgetId},
                ui_edit_freehand_walls::{OnFreehandWallEdited, ui_edit_freehand_walls},
                wall::wall_state::wall_spatial_hash::WallSpatialHash,
            },
        };
        let source = Curve2::try_new(vec![[0., 0.], [2., 0.], [3., 2.]], false).unwrap();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(source.clone(), 4., 0, 0, false));
        let mut world = World::new();
        world.insert_resource(inner);
        world.init_resource::<PublicWalls>();
        fn publish(world: &mut World) {
            let mut inner = world.remove_resource::<InnerWalls>().unwrap();
            let terrain = TerrainHeightGrid512 {
                encoded: vec![0u16; 512 * 512].into(),
                minimum_y: 0.,
                maximum_y: 0.,
                sampling_fix: false,
            };
            world.resource_mut::<PublicWalls>().consume_with_paths(
                &mut inner,
                &terrain,
                None,
                &WallPublicationDirty::default(),
                &WallSpatialHash::default(),
            );
            world.insert_resource(inner);
        }
        publish(&mut world);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::ZERO,
            ..Default::default()
        });
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<crate::systems::collision::world::RaycastWorld>();
        #[cfg(feature="native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        let mut history = History::<MaybePendingSnapshotId>::empty_glade();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        let mut heights = TerrainHeightsData::<TerrainHeightSsbo>::new();
        heights.set_height_range(0., 0.);
        world.insert_resource(heights);
        world.insert_resource(MainCamera::new(1.));
        world.init_resource::<ActionSetStates>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(GladeBorder(
            crate::resources::glade_border::BorderShape::Rectangle(
                ::utils::geometry::Rectangle2d::new_centered_axis_aligned(Vec2::splat(130.)),
            ),
        ));
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<DisplayOutOfBorder>>();
        world.init_resource::<Events<FeedbackInputOutOfBorder>>();
        world.init_resource::<Events<OnShapeMoved>>();
        world.init_resource::<Events<OnFreehandWallEdited>>();
        world.init_resource::<Events<HideTreesNextToWallCmd>>();
        world.init_resource::<Events<ShowPinnedDecoClutter>>();
        let widget = WidgetId::from_name(&format!("move_shape{wall}"));
        let interact = |world: &mut World, phase, widget| {
            let mut ui = UiState::new(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget,
            });
            ui.set_intersection_tag(0);
            world.insert_resource(ui);
        };
        let mut movement = Schedule::default();
        movement.add_systems(ui_move_freehand_walls);
        interact(&mut world, 3, widget);
        movement.run(&mut world);
        let registered = world.resource::<UiState>().widget(widget).unwrap();
        assert!(matches!(
            registered.geometry,
            Some(crate::systems::ui::resources::widget_params::WidgetGeometry::TriangleMesh(_))
        ));
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(4., 0., 2.);
        interact(&mut world, 4, widget);
        movement.run(&mut world);
        let translated = world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .source()
            .curve
            .clone();
        assert_eq!(translated.points, vec![[4., 2.], [6., 2.], [7., 4.]]);
        assert_eq!(translated.points_u, source.points_u);
        publish(&mut world);
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(200., 0., 200.);
        interact(&mut world, 4, widget);
        movement.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points,
            translated.points
        );
        assert_eq!(
            world.resource::<Events<FeedbackInputOutOfBorder>>().len(),
            1
        );
        interact(&mut world, 0, widget);
        movement.run(&mut world);
        assert_eq!(
            world.resource::<History>().pending().unwrap().edits.len(),
            1
        );
        // 同一World转入逐点编辑；Local跨帧持有真实起点与曲线，而不是注入编辑命令。
        let point_widget = WidgetId::from_name(&format!("edit_wall_{wall}_1"));
        let mut editing = Schedule::default();
        editing.add_systems(ui_edit_freehand_walls);
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(6., 0., 2.);
        interact(&mut world, 3, point_widget);
        editing.run(&mut world);
        assert_eq!(
            world
                .resource::<UiState>()
                .widget(point_widget)
                .unwrap()
                .volume
                .size
                .x,
            0.75
        );
        world.resource_mut::<CursorTerrainRaycast>().pos = Vec3::new(7., 0., 3.);
        interact(&mut world, 4, point_widget);
        editing.run(&mut world);
        assert_eq!(
            world
                .resource::<InnerWalls>()
                .get(wall)
                .unwrap()
                .source()
                .curve
                .points[1],
            [7., 3.]
        );
        publish(&mut world);
        interact(&mut world, 1, point_widget);
        editing.run(&mut world);
        let pending = world.resource::<History>().pending().unwrap();
        assert_eq!(pending.edits.len(), 2);
        let saved = serde_json::to_vec(&pending.edits).unwrap();
        let restored: Vec<EditType> = serde_json::from_slice(&saved).unwrap();
        assert_eq!(restored.len(), 2);
        let EditType::Wall(restored_wall) = &restored[1] else {
            panic!("自由墙历史恢复必须保留业务类型");
        };
        let WallHistoryEdit::MorphFreehandWall(change) = &restored_wall.groups[0][0] else {
            panic!("自由墙历史恢复必须保留曲线编辑");
        };
        let current_curve = &world
            .resource::<InnerWalls>()
            .get(wall)
            .unwrap()
            .source()
            .curve;
        assert_eq!(change.after.points, current_curve.points);
        assert_eq!(change.after.points_u, current_curve.points_u);
        assert_eq!(
            change.after.length.to_bits(),
            current_curve.length.to_bits()
        );
        for edit in &pending.edits {
            let EditType::Wall(edit) = edit else {
                panic!("必须记录墙复合编辑");
            };
            assert_eq!(edit.groups.len(), 1);
            assert!(matches!(
                edit.groups[0][0],
                WallHistoryEdit::MorphFreehandWall(_)
            ));
        }
        assert_eq!(world.resource::<Events<OnShapeMoved>>().len(), 2);
        assert_eq!(world.resource::<Events<OnFreehandWallEdited>>().len(), 2);
        let tree: Vec<_> = world
            .resource_mut::<Events<HideTreesNextToWallCmd>>()
            .drain()
            .collect();
        assert_eq!(tree.iter().filter(|e| e.finished).count(), 2);
        world.resource_mut::<UiSignifierStream>().take_entries();
        // 卸载World和两个Schedule会销毁Local曲线/闭包/归档，无裸指针借用残留。
        drop(editing);
        drop(movement);
        drop(world);
    }
}
