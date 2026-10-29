//! B 140ac98e0：逐控制点编辑自由墙，保持闭合端点、最小邻边与附着参数。
use std::sync::Arc;

use ::utils::curve::CurveInternalCoord;
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};

use crate::{
    resources::{
        cursor_terrain_raycast::CursorTerrainRaycast,
        glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder},
        history::History,
        roofs::geometry::surface::Curve2,
        terrain_height::{TerrainHeightSsbo, TerrainHeightsData},
        tree_storage::HideTreesNextToWallCmd,
        walls::{
            InnerWalls, WallId,
            decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
            decorator_storage::DecoratorStorage,
            public_walls::PublicWalls,
        },
        window::AudioCursorMoved,
    },
    systems::{
        audio::RequestWallMaterialAudio,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{WidgetParams, WidgetVolumeShape},
            },
            signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
        },
        ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
        ui_move_freehand_walls::{anchor_xz, archive_freehand, finish_curve_history, wall_anchor},
    },
};

/// 原12B元素，第三字为段内t；墙空间y仍来自装饰自身，不能额外伪造第四字段。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FreehandWallEdit {
    pub id: u32,
    pub segment: u32,
    pub t: f32,
}

#[derive(Event, Clone, Debug)]
pub struct OnFreehandWallEdited {
    pub affected: Vec<FreehandWallEdit>,
    pub wall: WallId,
}

#[derive(Default)]
pub struct EditFreehandWallCache {
    pub affected: Vec<FreehandWallEdit>,
    pub before: Option<Curve2>,
    pub cursor_offset: Vec2,
    pub grabbed_point: Option<Vec2>,
}

/// 原140acaae0..af39：闭合墙首尾同改，再按完整原曲线算法重算弧长参数。
fn edit_point(curve: &Curve2, index: usize, target: Vec2, shape_tag: u32) -> Curve2 {
    let mut points = curve.points.clone();
    points[index] = target.to_array();
    if shape_tag != 2 {
        let last = points.len() - 1;
        if index == 0 {
            points[last] = target.to_array();
        } else if index == last {
            points[0] = target.to_array();
        }
    }
    Curve2::try_new(points, false).expect("原自由墙曲线重建失败")
}

/// 原只检查index±1；首点仅有后一项，越过末点的项跳过。
fn adjacent_edges_valid(curve: &Curve2, index: usize, target: Vec2) -> bool {
    let valid = |neighbor: usize| {
        curve.points.get(neighbor).is_none_or(|point| {
            let dx = point[0] - target.x;
            let dy = point[1] - target.y;
            // UCOMISS/J A：NaN不是“小于0.1”，保持原无序比较语义。
            !((dy * dy + dx * dx).sqrt() < 0.1)
        })
    };
    valid(index + 1) && (index == 0 || valid(index - 1))
}

/// 原15组资源合同；手柄使用真实球拾取体，禁止外部预制交互事件代替。
pub fn ui_edit_freehand_walls(
    mut cache: Local<EditFreehandWallCache>,
    mode: Res<AppMode>,
    public: Res<PublicWalls>,
    mut inner: ResMut<InnerWalls>,
    (cursor, mut audio_moved): (Res<CursorTerrainRaycast>, EventReader<AudioCursorMoved>),
    storage: Res<DecoratorStorage>,
    mut archivist: ResMut<LiveDecoratorArchivist>,
    terrain: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    archival: DecoratorArchivalResources,
    ui: Res<UiState>,
    mut history: ResMut<History>,
    mut audio: EventWriter<RequestWallMaterialAudio>,
    mut edited: EventWriter<OnFreehandWallEdited>,
    mut stream: ResMut<UiSignifierStream>,
    (border, mut display, mut feedback, mut pins, mut trees): (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
        EventWriter<ShowPinnedDecoClutter>,
        EventWriter<HideTreesNextToWallCmd>,
    ),
) {
    let AppMode::Editing(EditSubject::Wall(id)) = *mode else {
        return;
    };
    let wall = WallId(id);
    let published = public.get(wall).expect("编辑自由墙的公开墙不存在");
    let shape_tag = published.shape().tag();
    if shape_tag < 2 {
        return;
    }
    let curve = published.simplified_curve_2d();
    for (index, point) in curve.points.iter().enumerate() {
        let xz = Vec2::from_array(*point);
        let ground = terrain.sample_at_world_xz(xz.x, xz.y);
        let position = Vec3::new(xz.x, ground, xz.y);
        let widget = ui.add(
            &format!("edit_wall_{wall}_{index}"),
            WidgetParams::new(WidgetVolumeShape::sphere(position, 0.75), [0; 2])
                .occlude_by_world(false),
        );
        let scale = if widget.is_active() { 0.3 } else { 0.15 };
        // 原工厂按位置+尺度命名，避免活动半径变化复用旧实体。
        stream.write_factory(&SignifierFactory::from_closure(
            &format!("wall_edit_pt_{position:?}{scale}"),
            Arc::new(move |meshes, shaders, commands| {
                vec![
                    SignifierEntityBuilder::from_mesh_name("temp_sphere")
                        .position(position)
                        .scale(scale)
                        .animation_duration(0.0)
                        .build(meshes, shaders, commands),
                ]
            }),
        ));
        match widget.dragged() {
            2 => continue,
            3 => {
                archive_freehand(
                    &mut archivist,
                    &storage,
                    &archival,
                    wall,
                    "edit_freehand_wall",
                );
                cache.affected.clear();
                if let Some(bucket) = storage.wall_decorators(wall) {
                    for (&id, info) in bucket.decorators() {
                        if wall_anchor(info) {
                            let CurveInternalCoord { segment, t } =
                                ::utils::curve::approx_curve_coord(
                                    &curve.points,
                                    anchor_xz(info),
                                    |point| Vec2::from_array(*point),
                                );
                            cache.affected.push(FreehandWallEdit {
                                id,
                                segment: u32::try_from(segment).expect("原曲线段索引必须能容纳u32"),
                                t,
                            });
                        }
                    }
                }
                cache.before = Some(curve.clone());
                cache.grabbed_point = Some(xz);
                let cursor_position = cursor.position();
                cache.cursor_offset = Vec2::new(cursor_position.x, cursor_position.z) - xz;
            }
            4 => {
                let cursor_position = cursor.position();
                let target = Vec2::new(
                    cursor_position.x - cache.cursor_offset.x,
                    cursor_position.z - cache.cursor_offset.y,
                );
                let current = &inner
                    .get(wall)
                    .expect("编辑自由墙的权威墙不存在")
                    .source()
                    .curve;
                let next = edit_point(current, index, target, shape_tag);
                let inside = next
                    .points
                    .iter()
                    .all(|point| border.is_pos_inside(Vec2::from_array(*point)));
                let valid = adjacent_edges_valid(&next, index, target);
                if !inside {
                    display.write(DisplayOutOfBorder);
                    feedback.write(FeedbackInputOutOfBorder);
                }
                if inside && valid {
                    edited.write(OnFreehandWallEdited {
                        affected: cache.affected.clone(),
                        wall,
                    });
                    inner.update_wall_state_curve(wall, next);
                }
                trees.write(HideTreesNextToWallCmd {
                    wall,
                    finished: false,
                });
                pins.write(ShowPinnedDecoClutter::for_wall(wall));
                if !inside || !valid {
                    continue;
                }
            }
            _ => {
                edited.write(OnFreehandWallEdited {
                    affected: cache.affected.clone(),
                    wall,
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
                kind: 0,
                flag: published.authority.source_flag_65().wrapping_mul(2),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_neighbor_threshold_fixture_matches_branch() {
        let cases: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-freehand-native.json"
        )))
        .unwrap();
        let pair = |p: &serde_json::Value| {
            [
                f32::from_bits(p[0].as_u64().unwrap() as u32),
                f32::from_bits(p[1].as_u64().unwrap() as u32),
            ]
        };
        for case in cases["edges"].as_array().unwrap() {
            let target = Vec2::from_array(pair(&case["target"]));
            let curve = Curve2 {
                points: vec![target.to_array(), pair(&case["neighbor"])],
                points_u: vec![0., 1.],
                length: 1.,
            };
            assert_eq!(
                !adjacent_edges_valid(&curve, 0, target),
                case["rejected"].as_bool().unwrap()
            );
        }
    }
    #[test]
    fn closed_endpoints_and_neighbor_minimum_follow_original_branches() {
        let source = Curve2::try_new(vec![[0., 0.], [2., 0.], [2., 2.], [0., 0.]], false).unwrap();
        let first = edit_point(&source, 0, Vec2::new(1., 1.), 3);
        assert_eq!(first.points[0], first.points[3]);
        let last = edit_point(&source, 3, Vec2::new(1., 1.), 3);
        assert_eq!(last.points[0], last.points[3]);
        assert!(adjacent_edges_valid(&source, 0, Vec2::new(1.9, 0.)));
        assert!(!adjacent_edges_valid(&source, 0, Vec2::new(1.95, 0.)));
        let open = edit_point(&source, 0, Vec2::new(1., 1.), 2);
        assert_eq!(open.points[3], source.points[3]);
    }
}
