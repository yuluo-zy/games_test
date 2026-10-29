//! 原 B 1409ac910：左侧投影轮廓选择、真实旋转手柄、装饰/屋顶通知和Concurrent历史。
use super::super::to_authority_curve;
use crate::{
    resources::{
        events::OnCircleRotated,
        history::{
            EditType, History,
            wall_edit::{MorphPremadeShapeEdit, WallCompoundHistoryEdit, WallHistoryEdit},
        },
        mouse_terrain_raycast::CameraCursorRay,
        onboarding::NotifyHintSystemShapeEditWasPerformed,
        roofs::commands::RecalculateRoofCmd,
        time::GameTime,
        walls::{
            InnerWalls, WallId,
            decorator_archivist::{DecoratorArchivalResources, LiveDecoratorArchivist},
            decorator_storage::{DecoratorStorage, DecoratorStorageAddr},
            public_walls::PublicWalls,
            publication_fields::{WallId as RecordedWallId, WallShapeSnapshot},
        },
        window::WindowSize,
    },
    systems::{
        audio::PlayBuildingAudio,
        camera_rig::MainCamera,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{PlayUiSound, WidgetParams},
            },
            signifiers::{SignifierEntityBuilder, SignifierFactory, UiSignifierStream},
        },
        ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter,
        wall::{
            shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume,
            valid_enclosure::{Shape, ShapeParameters},
        },
    },
    utils::{components::transform::Transform, debug_viz::DebugLine},
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use utils::{circular_f32::CircularF32, geometry::Circle2d};

/// 原局部抓取记录28B；前三个角与交点持续到释放及回位音效结束。
#[derive(Default)]
pub struct RotateCircleCache {
    angle_offset: f32,
    initial_cursor_angle: f32,
    initial_rotation: f32,
    anchor: Vec3,
    return_sound: u8,
}
#[derive(Default)]
pub struct CurrentAngleGizmo(Option<CircularF32>);

pub fn rotate_circle_wall(inner: &mut InnerWalls, wall: WallId, angle: f32) -> bool {
    let Some(state) = inner.get(wall) else {
        return false;
    };
    let Some(params) = state.get_enclosure_params() else {
        return false;
    };
    let mut circle = ShapeParameters::from_snapshot(params).as_circle();
    circle.rotation = angle;
    inner.update_wall_state_curve(wall, to_authority_curve(&circle.as_curve2(0.5)));
    inner.update_wall_state_enclosure(
        wall,
        WallShapeSnapshot::from_observed(
            ShapeParameters::from_shape(Shape::Circle(circle)).to_record(),
        ),
    );
    true
}

/// 保留原两级矩阵乘与最小屏幕X比较；不以世界X或圆解析极值代替轮廓采样。
fn leftmost_outline(points: &[[f32; 3]], camera: &MainCamera, width: u32) -> Vec3 {
    assert!(!points.is_empty(), "圆旋转轮廓不能为空");
    let view = camera.read_matrix(0x80);
    let projection = camera.read_matrix(0);
    let transform = |matrix: glam::Mat4, point: Vec3| {
        ((matrix.x_axis * point.x + matrix.y_axis * point.y) + matrix.z_axis * point.z)
            + matrix.w_axis
    };
    let mut chosen = Vec3::from_array(points[0]);
    let mut minimum = 10_000_000.0;
    for &point in points {
        let point = Vec3::from_array(point);
        let clip = transform(projection, transform(view, point).truncate());
        let screen_x = ((clip.x / clip.w) * 0.5 + 0.5) * width as f32;
        if screen_x < minimum {
            chosen = point;
        }
        minimum = if minimum <= screen_x {
            minimum
        } else {
            screen_x
        };
    }
    chosen
}
fn angle(point: Vec3, center: Vec2) -> f32 {
    (point.z - center.y).atan2(point.x - center.x)
}
fn clamp_audio(value: f32) -> f32 {
    let lower = if 0.0 <= value { value } else { 0.0 };
    if lower <= 1.0 { lower } else { 1.0 }
}

/// 原16组/21展开SystemParam；事件与装饰/屋顶consumer共享身份。
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn ui_rotate_circle(
    mode: Res<AppMode>,
    mut inner: ResMut<InnerWalls>,
    public: Res<PublicWalls>,
    (archival, mut archivist, storage): (
        DecoratorArchivalResources,
        ResMut<LiveDecoratorArchivist>,
        ResMut<DecoratorStorage>,
    ),
    (ui, mut stream): (Res<UiState>, ResMut<UiSignifierStream>),
    (window, camera, ray): (Res<WindowSize>, Res<MainCamera>, Res<CameraCursorRay>),
    mut history: ResMut<History>,
    mut rotated: EventWriter<OnCircleRotated>,
    mut roofs: EventWriter<RecalculateRoofCmd>,
    mut pins: EventWriter<ShowPinnedDecoClutter>,
    mut hints: EventWriter<NotifyHintSystemShapeEditWasPerformed>,
    mut building_audio: EventWriter<PlayBuildingAudio>,
    mut ui_audio: EventWriter<PlayUiSound>,
    mut cache: Local<RotateCircleCache>,
    mut gizmo: Local<CurrentAngleGizmo>,
    time: Res<GameTime>,
) {
    let wall = match *mode {
        AppMode::Editing(EditSubject::Wall(wall) | EditSubject::Roof(wall)) => WallId(wall),
        _ => {
            gizmo.0 = None;
            return;
        }
    };
    let state = public.get(wall).expect("圆旋转目标公共墙不存在");
    let Some(circle) = state.is_circle() else {
        gizmo.0 = None;
        return;
    };
    // 原公共状态+0x40是地形映射原轮廓，不是+0x88的路径切分轮廓。
    let left = leftmost_outline(&state.geometry.outline, &camera, window.width);
    let relative = Vec2::new(left.x, left.z) - circle.center;
    let inverse = 1.0 / (relative.y * relative.y + relative.x * relative.x).sqrt();
    let normal = relative * inverse;
    let rotation = utils::xz_rotation_bad(normal);
    let outward = Vec3::new(normal.x, 0.0, normal.y);
    let handle = left + outward;
    let trigger = handle + outward * 0.3;
    let transform = Transform {
        rotation,
        translation: trigger,
        scale: Vec3::ONE,
    };
    let params = WidgetParams::default()
        .mesh_volume(box_trigger_volume(3.5, 0.5, 1.5, &transform))
        .occlude_by_world(false);
    let context = ui.add(&format!("rotate_circle_{}", wall.0), params);
    let phase = context.dragged();
    let mut immediate_angle = None;
    match phase {
        3 => {
            archivist.archive_current_decorators_at_with_resources(
                DecoratorStorageAddr::Wall(wall),
                &storage,
                &archival,
                "circle rotate",
            );
            archivist.decorator_cull_allow_wall(wall, &storage);
            let hit = ray.intersect_with_xz_plane(trigger.y);
            cache.initial_cursor_angle = angle(hit, circle.center);
            cache.angle_offset = angle(trigger, circle.center) - cache.initial_cursor_angle;
            cache.initial_rotation = CircularF32::new(circle.rotation).as_f32();
            cache.anchor = hit;
            cache.return_sound = 0;
        }
        4 => {
            let hit = ray.intersect_with_xz_plane(cache.anchor.y);
            let cursor_angle = angle(hit, circle.center);
            let wanted = CircularF32::new(
                (cache.initial_rotation - cache.initial_cursor_angle) + cursor_angle,
            )
            .as_f32();
            if wanted != circle.rotation {
                let position = Vec3::new(circle.center.x, trigger.y, circle.center.y);
                building_audio.write(PlayBuildingAudio {
                    id: wall.0,
                    position,
                    editing_speed: clamp_audio(utils::scalar::inverse_lerp(
                        0.0,
                        15.0,
                        CircularF32::new(wanted).distance(circle.rotation) / time.delta,
                    )),
                    kind: 2,
                });
                assert!(rotate_circle_wall(&mut inner, wall, wanted));
                if let Some(entity) = public.roof_entity(wall) {
                    roofs.write(RecalculateRoofCmd {
                        entity,
                        wall,
                        kind: 2,
                    });
                }
            }
            DebugLine::new(vec![
                hit,
                Vec3::new(circle.center.x, trigger.y, circle.center.y),
            ])
            .show(&mut stream);
            rotated.write(OnCircleRotated(wall));
            pins.write(ShowPinnedDecoClutter::for_circle(
                circle,
                state.authority.elevation(),
                state.max_y(),
                Some(wall),
            ));
            immediate_angle = Some(CircularF32::new(cursor_angle + cache.angle_offset));
        }
        2 => {}
        _ => {
            rotated.write(OnCircleRotated(wall));
            let before = Circle2d {
                rotation: CircularF32::new(cache.initial_rotation).as_f32(),
                ..circle
            };
            let edit = WallCompoundHistoryEdit {
                groups: vec![vec![WallHistoryEdit::MorphPremadeShape(
                    MorphPremadeShapeEdit::new(
                        RecordedWallId(wall.0),
                        ShapeParameters::from_shape(Shape::Circle(before)),
                        ShapeParameters::from_shape(Shape::Circle(circle)),
                    ),
                )]],
            };
            history
                .add(EditType::Wall(std::sync::Arc::new(edit)))
                .expect("圆旋转释放时历史事务不可写");
            hints.write(NotifyHintSystemShapeEditWasPerformed);
            cache.return_sound = 1;
        }
    }
    let target = CircularF32::new(angle(handle, circle.center));
    let previous = gizmo.0;
    let displayed = if let Some(immediate) = immediate_angle {
        immediate
    } else if let Some(previous) = previous {
        previous.lerp(
            target.as_f32(),
            utils::time_continuous_lerp_factor(time.delta, 10.0),
        )
    } else {
        target
    };
    gizmo.0 = Some(displayed);
    let theta = displayed.as_f32();
    let direction = Vec2::new(theta.cos(), theta.sin());
    let point = direction * (circle.radius + 1.0) + circle.center;
    let position = Vec3::new(point.x, trigger.y, point.y);
    if let Some(previous) = previous.filter(|_| cache.return_sound == 1) {
        let distance = previous.distance(displayed.as_f32());
        if distance <= 0.01 {
            ui_audio.write(PlayUiSound {
                kind: 35,
                ..Default::default()
            });
            cache.return_sound = 2;
        } else {
            let mut payload = [0u8; 20];
            payload[..4].copy_from_slice(
                &clamp_audio(utils::scalar::inverse_lerp(0.4, 0.0, distance)).to_le_bytes(),
            );
            for (slot, component) in position.to_array().into_iter().enumerate() {
                payload[4 + slot * 4..8 + slot * 4].copy_from_slice(&component.to_le_bytes());
            }
            ui_audio.write(PlayUiSound {
                kind: 34,
                _payload: payload,
            });
        }
    }
    let rotation = utils::xz_rotation_bad(direction);
    let scale = if context.is_active() { 1.0 } else { 0.8 };
    let name = format!("circle_{wall:?}_rotate_{position:?}{scale}");
    for (mesh, pass) in [("corner_arrow", 0), ("corner_arrow_outline", 1)] {
        let factory = SignifierFactory::from_closure(
            &name,
            std::sync::Arc::new(move |meshes, shaders, commands| {
                vec![
                    SignifierEntityBuilder::from_mesh_name(mesh)
                        .set_ui_pass(pass)
                        .rotation(rotation)
                        .position(position)
                        .scale(scale)
                        .animation_duration(0.0)
                        .build(meshes, shaders, commands),
                ]
            }),
        );
        stream.write_factory(&factory);
    }
}

#[cfg(test)]
mod native_gizmo_tests {
    use super::*;
    #[test]
    fn circle_rotate_projected_choice_and_trigger_match_original_instructions() {
        let report: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../reports/module-recovery-2026-10-23/m01-circle-rotate-gizmo-native.json"
        )))
        .unwrap();
        assert_eq!(
            report["binary_sha256"],
            "f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03"
        );
        assert_eq!(report["rows"].as_array().unwrap().len(), 24);
        for (case, row) in report["rows"].as_array().unwrap().iter().enumerate() {
            let input = &row["input"];
            let points: Vec<[f32; 3]> = serde_json::from_value(input["points"].clone()).unwrap();
            let center: [f32; 2] = serde_json::from_value(input["center"].clone()).unwrap();
            let mut camera = MainCamera::new(1.0);
            for (name, offset) in [("view", 0x80), ("projection", 0)] {
                let values: [f32; 16] = serde_json::from_value(input[name].clone()).unwrap();
                camera.write_matrix(offset, &glam::Mat4::from_cols_array(&values));
            }
            let left = leftmost_outline(&points, &camera, input["width"].as_u64().unwrap() as u32);
            let relative = Vec2::new(left.x, left.z) - Vec2::from_array(center);
            let normal =
                relative * (1.0 / (relative.y * relative.y + relative.x * relative.x).sqrt());
            let outward = Vec3::new(normal.x, 0.0, normal.y);
            let rotation = utils::xz_rotation_bad(normal);
            let position = (left + outward) + outward * 0.3;
            let actual = rotation
                .to_array()
                .into_iter()
                .chain(position.to_array())
                .chain(Vec3::ONE.to_array())
                .map(f32::to_bits)
                .collect::<Vec<_>>();
            let expected: Vec<u32> = serde_json::from_value(row["transform"].clone()).unwrap();
            assert_eq!(actual, expected, "原圆旋转几何case{case}");
        }
    }
}
