//! 原 B 140b2fc00：发布墙的高度控件入口；与聚焦预制形状的球形控件分工。
//! PDB 的十一项参数与原四项跳表逐一对应，拖拽副作用交给原消费者。
use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::Vec3;
use utils::curve::Curve3;

use super::{
    UiWallHeight, signifier_animation::WallHeightSignifier,
    ui_adjust_wall_height_mode::AdjustWallHeightUi,
};
use crate::{
    render::pass_builder::draw_list::DrawTopology,
    resources::{
        events::{FinishAdjWallHeightMode, InitAdjWallHeightMode},
        roofs::{Roof, RoofAnimation},
        time::GameTime,
        walls::{
            WallId,
            public_walls::{PublicWallState, PublicWalls},
        },
    },
    systems::{
        camera_rig::main_camera::MainCameraMoved,
        mode_manager::{AppMode, EditSubject},
        ui::{
            resources::{
                ui_state::{UiState, UiWidgetContext},
                widget_params::{UiSounds, WidgetGeometry, WidgetParams},
            },
            signifiers::{
                signifier_create::UiSignifierStream,
                signifier_factory::{SignifierEntityBuilder, SignifierFactory, SignifierMesh},
            },
        },
    },
};

impl UiWallHeight {
    /// 原PublicWallState以借用传入；不为只读发布数据额外克隆两个数组。
    fn published_points(&self, record: &PublicWallState, rim: bool) -> Vec<Vec3> {
        let lift = if self.mode == 0 {
            Self::FLAT_LIFT
        } else if rim {
            Self::STACKED_LIFT
        } else {
            0.0
        };
        record
            .height_outline()
            .iter()
            .zip(record.arch_segment_heights().unwrap_or(&[]))
            .map(|(p, y)| Vec3::new(p[0], *y + lift, p[2]))
            .collect()
    }

    /// 原1409566d0：逐点云体积保留首尾重复点；相机移动只静音，不清空控件。
    fn widget<'a>(
        &self,
        record: &PublicWallState,
        ui: &'a UiState,
        premade: bool,
        camera_moved: bool,
        flat_with_rim: bool,
    ) -> UiWidgetContext<'a> {
        let radius = if self.mode == 0 || premade { 0.5 } else { 0.7 };
        let mut sounds = UiSounds::empty();
        if !camera_moved {
            for (slot, kind) in sounds.slots.iter_mut().zip([1u32, 2, 3, 3]) {
                slot[..4].copy_from_slice(&kind.to_le_bytes());
            }
        }
        let mut params = WidgetParams::default()
            .geometry(WidgetGeometry::SphereCloud {
                centers: self.published_points(record, flat_with_rim),
                radius,
            })
            .sounds(sounds)
            .occlude_by_world(self.flag == 0);
        // 原+a78字节02/02：带偏移的第二类brush反馈，零Vec3偏移仍保留tag。
        params.extra_78 = [0x202, 0];
        ui.add(&self.widget_name(), params)
    }

    /// 原140956cd0及两个真实闭包140957390/1409576d0。
    /// 普通选择用专属动画组件展开曲线；聚焦时每帧新建静态预览，旧实体由统一流回收。
    fn signifier(
        &self,
        record: &PublicWallState,
        context: &UiWidgetContext<'_>,
        stream: &mut UiSignifierStream,
        frame: u32,
    ) {
        if self.mode == 1 && self.flag == 0 && !context.is_active() {
            return;
        }
        let name = self.signifier_name(frame);
        if self.mode == 1 {
            let wall = self.wall;
            let duration = self.signifier_env_scalar();
            stream.write_factory(&SignifierFactory::from_closure(
                &name,
                Arc::new(move |meshes, shaders, commands| {
                    let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
                        .disable_default_animation()
                        .animation_duration(duration)
                        .build(meshes, shaders, commands);
                    commands.entity(entity).insert((
                        DrawTopology::Alt(3.0),
                        WallHeightSignifier {
                            // 原Option<Curve3>尚未初始化；动画系统先重建再读取。
                            curve: Curve3::from_cached_parts(Vec::new(), Vec::new(), 0.0),
                            anchor_u: None,
                            latched_u: None,
                            cursor: None,
                            wall,
                        },
                    ));
                    vec![entity]
                }),
            ));
        } else {
            let points = self.published_points(record, false);
            stream.write_factory(&SignifierFactory::from_closure(
                &name,
                Arc::new(move |meshes, shaders, commands| {
                    let entity = SignifierEntityBuilder::from_points(points.clone())
                        .disable_default_animation()
                        .animation_duration(0.0)
                        .build(meshes, shaders, commands);
                    commands.entity(entity).insert(DrawTopology::Alt(3.0));
                    vec![entity]
                }),
            ));
        }
    }
}

/// 原系统未接触持久状态、音频和装饰物；它们经Adjust/Init/Finish进入各自原系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_adjust_wall_height(
    mode: Res<AppMode>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    ui: Res<UiState>,
    mut finish: EventWriter<FinishAdjWallHeightMode>,
    mut adjust: EventWriter<AdjustWallHeightUi>,
    mut init: EventWriter<InitAdjWallHeightMode>,
    animations: Query<&RoofAnimation>,
    mut stream: ResMut<UiSignifierStream>,
    mut camera_moved: EventReader<MainCameraMoved>,
    time: Res<GameTime>,
) {
    // 外层tag>=6直接返回，不能提前消费相机事件。
    if mode.native_tag() >= 6 {
        return;
    }
    let moved = camera_moved.read().count() != 0;
    for (wall, record) in public.all() {
        if let Some(entity) = public.roof_entity(wall) {
            let animation = animations
                .get(entity)
                .expect("屋顶关联必须包含RoofAnimation");
            // UCOMISS + JNE/JP：NaN同样阻止建立高度控件。
            if animation.values()[0] != 1.0 {
                continue;
            }
        }
        // 原循环再做PublicWalls::get验证；被推土机删除的记录不参与。
        record.assert_valid();
        if record.is_being_bulldozed() {
            continue;
        }
        let focused = match *mode {
            AppMode::Editing(EditSubject::Wall(id) | EditSubject::Roof(id)) => {
                if id != wall.0 || record.is_premade_shape() {
                    continue;
                }
                true
            }
            AppMode::Editing(EditSubject::None) => false,
            _ => return,
        };
        let rim = public
            .roof_component(wall, |entity| roofs.get(entity).ok())
            .is_some_and(|roof| roof.is_flat() && roof.observed()[0x3d] != 0);
        let height = UiWallHeight {
            wall,
            flag: u8::from(focused),
            mode: u8::from(!focused),
            ..Default::default()
        };
        let widget = height.widget(record, &ui, record.is_premade_shape(), moved, rim);
        // 普通模式遇相机运动隐藏新悬停，但已经捕获的拖拽必须持续到松手。
        if !focused && moved && widget.dragged() == 2 {
            continue;
        }
        height.signifier(record, &widget, &mut stream, time.frame_count);
        match widget.dragged().wrapping_sub(2).min(3) {
            0 => {}
            1 => {
                adjust.write(AdjustWallHeightUi(wall));
                init.write(InitAdjWallHeightMode);
            }
            2 => {
                adjust.write(AdjustWallHeightUi(wall));
            }
            _ => {
                finish.write(FinishAdjWallHeightMode(wall));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::{
        InnerWalls, WallState, publication_source::TerrainHeightGrid512,
    };
    use crate::systems::ui::resources::ui_state::{UiInteraction, WidgetId};
    use bevy_ecs::system::RunSystemOnce;

    fn fixture() -> (World, WallId) {
        let mut world = World::new();
        let mut inner = InnerWalls::new();
        let wall = inner.new_wall(WallState::new(
            Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap(),
            2.0,
            0,
            0,
            false,
        ));
        let mut public = PublicWalls::default();
        let paths =
            crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
                std::array::from_fn(|_| vec![0; 512 * 512].into()),
                false,
            )
            .unwrap();
        public.consume_with_paths(
            &mut inner,
            &TerrainHeightGrid512 {
                encoded: vec![0; 512 * 512].into(),
                minimum_y: 0.0,
                maximum_y: 0.0,
                sampling_fix: false,
            },
            Some(&paths),
            &Default::default(),
            &Default::default(),
        );
        world.insert_resource(public);
        world.insert_resource(AppMode::default());
        world.insert_resource(UiState::at_startup());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<GameTime>();
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<InitAdjWallHeightMode>>();
        (world, wall)
    }

    /// 160例来自独立执行原RNE：模式、聚焦、相机运动、屋顶边、形状和奇偶长度全组合。
    /// 比较原机器码真实输出，非拿Rust表达式自身充当oracle。
    #[test]
    fn native_widget_160_cases_match_points_radius_sounds_and_feedback() {
        let oracle: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../../../reports/module-recovery-2026-10-23/m01-height-widget-native.json"
        ))
        .unwrap();
        let (world, wall) = fixture();
        for row in oracle["cases"].as_array().unwrap() {
            let word = |name: &str| row[name].as_u64().unwrap() as u32;
            let bits = |name: &str| {
                row[name]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_u64().unwrap() as u32)
                    .collect::<Vec<_>>()
            };
            let mut record = world.resource::<PublicWalls>().get(wall).unwrap().clone();
            record.geometry.outline = Arc::new(
                bits("input_points")
                    .chunks_exact(3)
                    .map(|p| {
                        [
                            f32::from_bits(p[0]),
                            f32::from_bits(p[1]),
                            f32::from_bits(p[2]),
                        ]
                    })
                    .collect(),
            );
            record.prepared_path = None;
            record
                .freeform_heights
                .as_mut()
                .unwrap()
                .arch_segment_heights = bits("input_heights")
                .into_iter()
                .map(f32::from_bits)
                .collect();
            let height = UiWallHeight {
                wall,
                flag: word("flag") as u8,
                mode: word("mode") as u8,
                ..Default::default()
            };
            let ui = UiState::at_startup();
            let widget = height.widget(
                &record,
                &ui,
                word("shape") < 2,
                word("moved") != 0,
                word("rim") != 0,
            );
            let params = ui.widget(widget.id).unwrap();
            let WidgetGeometry::SphereCloud { centers, radius } = params.geometry.as_ref().unwrap()
            else {
                panic!("原高度widget必须是SphereCloud")
            };
            let actual: Vec<_> = centers
                .iter()
                .flat_map(|p| p.to_array().map(f32::to_bits))
                .collect();
            assert_eq!(actual, bits("points"), "{row}");
            assert_eq!(radius.to_bits(), word("radius"), "{row}");
            for (i, slot) in params.sounds.slots.iter().enumerate() {
                assert_eq!(
                    u32::from_le_bytes(slot[..4].try_into().unwrap()),
                    row["sound_tags"][i].as_u64().unwrap() as u32
                );
                assert_eq!(slot[4..], [0; 20]);
            }
            assert_eq!(params.extra_78, [0x202, 0]);
            assert_eq!(u32::from(params.occlude_by_world), word("occlude"));
            assert_eq!(u32::from(params.flags.0), word("flags"));
        }
    }

    fn phase(world: &mut World, wall: WallId, phase: u8) {
        world.resource::<UiState>().clear_widgets();
        world.resource_mut::<UiState>().set_intersection_tag(0);
        world
            .resource_mut::<UiState>()
            .replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: phase,
                widget: WidgetId::from_name(&format!("edit_height_{wall}")),
            });
    }

    #[test]
    fn real_world_dispatches_start_ongoing_release_and_camera_capture() {
        let (mut world, wall) = fixture();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui_adjust_wall_height);
        let mut adjust = world.resource::<Events<AdjustWallHeightUi>>().get_cursor();
        let mut init = world
            .resource::<Events<InitAdjWallHeightMode>>()
            .get_cursor();
        let mut finish = world
            .resource::<Events<FinishAdjWallHeightMode>>()
            .get_cursor();
        for (p, a, i, f) in [
            (2, 0, 0, 0),
            (3, 1, 1, 0),
            (4, 1, 0, 0),
            (1, 0, 0, 1),
            (0, 0, 0, 1),
        ] {
            phase(&mut world, wall, p);
            world
                .resource_mut::<Events<MainCameraMoved>>()
                .send(MainCameraMoved);
            schedule.run(&mut world);
            assert_eq!(adjust.read(world.resource()).count(), a);
            assert_eq!(init.read(world.resource()).count(), i);
            assert_eq!(finish.read(world.resource()).count(), f);
        }
        // 无相机移动时保留真实悬停的动态标记工厂。
        phase(&mut world, wall, 2);
        world.resource_mut::<UiSignifierStream>().take_entries();
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiState>().registry_snapshot().len(), 1);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 1);
    }

    #[test]
    fn mode_gates_preserve_pending_camera_and_only_focus_selected_freehand_wall() {
        let (mut world, wall) = fixture();
        *world.resource_mut::<AppMode>() = AppMode::Photomode;
        world
            .resource_mut::<Events<MainCameraMoved>>()
            .send(MainCameraMoved);
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert!(world.resource::<UiState>().registry_snapshot().is_empty());
        *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::Wall(wall.0));
        world.resource_mut::<GameTime>().frame_count = 513;
        world.run_system_once(ui_adjust_wall_height).unwrap();
        assert_eq!(world.resource::<UiState>().registry_snapshot().len(), 1);
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let crate::systems::ui::signifiers::signifier_create::UiSignifierEntry::Factory(factory) =
            &entries[0]
        else {
            panic!("高度预览必须使用真实工厂")
        };
        assert_eq!(
            factory.name(),
            Some(format!("wall_height_edit_{wall}513").as_str())
        );
        assert!(factory.has_closure());
    }

    #[test]
    fn actual_height_factories_animate_and_release_transient_meshes() {
        use crate::{
            asset_libraries::{
                mesh_library::AssetMeshLibrary,
                shader_library::{AssetShaderLibrary, SharedAssetShaderLibrary},
                transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
                vao_library::AssetVAOLibrary,
            },
            render::{shader::RasterShaderProgram, test_device::TestDevice},
            systems::ui::signifiers::{
                animation::{SignifierAnimation, signifier_animation_t},
                signifier_create::{SignifierId, signifier_create},
                signifier_mark_for_delete::{despawn_marked, signfier_mark_for_delete},
            },
        };
        for focused in [false, true] {
            let (mut world, wall) = fixture();
            if focused {
                *world.resource_mut::<AppMode>() = AppMode::Editing(EditSubject::Wall(wall.0));
            }
            world.resource_mut::<GameTime>().delta = 0.25;
            phase(&mut world, wall, 3);
            world.run_system_once(ui_adjust_wall_height).unwrap();
            let shaders = AssetShaderLibrary::new();
            shaders.add_raster(
                RasterShaderProgram::new_draw(
                    "vertex_color_shader",
                    Default::default(),
                    &TestDevice(1),
                ),
                false,
            );
            world.insert_resource(AssetMeshLibrary::new());
            world.insert_resource(SharedAssetShaderLibrary(Arc::new(shaders)));
            let widget = WidgetId::from_name(&format!("edit_height_{wall}"));
            let params = world.resource::<UiState>().widget(widget).unwrap();
            world.resource::<UiState>().clear_widgets();
            world.resource_mut::<UiState>().replace_interaction_full(
                UiInteraction {
                    native_tag: 2,
                    native_phase: 3,
                    widget,
                },
                Some(params),
                [0; 56],
            );
            world.run_system_once(signifier_create).unwrap();
            let entities = world
                .query_filtered::<Entity, With<SignifierId>>()
                .iter(&world)
                .collect::<Vec<_>>();
            assert_eq!(entities.len(), 1);
            let entity = entities[0];
            assert_eq!(
                world.get::<DrawTopology>(entity),
                Some(&DrawTopology::Alt(3.0))
            );
            let mesh = world.get::<TransientMesh>(entity).unwrap().mesh_handle();
            assert_eq!(world.get::<WallHeightSignifier>(entity).is_some(), !focused);
            let anim = world.get::<SignifierAnimation>(entity).unwrap();
            assert_eq!(anim.flag_8, 1);
            assert_eq!(anim.duration, if focused { 0.0 } else { 0.2 });
            world.run_system_once(signifier_animation_t).unwrap();
            world
                .run_system_once(
                    super::super::signifier_animation::ui_adjust_wall_height_signifier_animation,
                )
                .unwrap();
            assert!(
                !world
                    .resource::<AssetMeshLibrary>()
                    .get(mesh)
                    .unwrap()
                    .positions_without_indices()
                    .is_empty()
            );
            let owner = Arc::downgrade(&world.resource::<UiState>().active_params().unwrap());
            world.insert_resource(UiState::at_startup());
            assert!(owner.upgrade().is_none());
            world.run_system_once(signfier_mark_for_delete).unwrap();
            world.run_system_once(signifier_animation_t).unwrap();
            world.run_system_once(despawn_marked).unwrap();
            assert!(world.get_entity(entity).is_err());
            delete_dropped_transient_meshes(
                &TestDevice(1),
                &mut world.resource_mut::<AssetMeshLibrary>(),
                &mut AssetVAOLibrary::new(),
            );
            assert!(world.resource::<AssetMeshLibrary>().get(mesh).is_none());
        }
    }

    #[test]
    fn roof_animation_gate_and_flat_rim_use_published_association() {
        for value in [0.0f32, 0.5, 1.0, f32::NAN] {
            let (mut world, wall) = fixture();
            let mut roof = [0; 88];
            roof[0x30..0x34].copy_from_slice(&0.1f32.to_le_bytes());
            roof[0x3d] = 1;
            let mut animation = [0; 8];
            animation[..4].copy_from_slice(&value.to_le_bytes());
            let entity = world
                .spawn((
                    Roof::from_observed(roof),
                    RoofAnimation::from_inner(
                        crate::resources::roofs::components::RoofAnimation::from_observed(
                            animation,
                        ),
                    ),
                ))
                .id();
            world
                .resource_mut::<PublicWalls>()
                .associate_roof(wall, entity);
            world.run_system_once(ui_adjust_wall_height).unwrap();
            let widget = world
                .resource::<UiState>()
                .widget(WidgetId::from_name(&format!("edit_height_{wall}")));
            assert_eq!(widget.is_some(), value == 1.0);
            if let Some(widget) = widget {
                let Some(WidgetGeometry::SphereCloud { centers, .. }) = &widget.geometry else {
                    panic!("应保持原SphereCloud")
                };
                let record = world.resource::<PublicWalls>().get(wall).unwrap();
                for (point, height) in centers.iter().zip(record.arch_segment_heights().unwrap()) {
                    assert_eq!(
                        point.y.to_bits(),
                        (*height + UiWallHeight::STACKED_LIFT).to_bits()
                    );
                }
            }
        }
    }
}

// 保留远端新增的真实输入适配→高度历史→undo/redo整合回归。
#[cfg(test)]
mod physical_chain {
    use super::*;
    use bevy_input::mouse::MouseButton;
    use glam::{Mat4, Vec2};

    use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
    use crate::resources::events::{EscapeMenuJustOpened, ShutdownGameCmd};
    use crate::resources::mouse_capture::OsCursorVisibilityCmd;
    use crate::resources::mouse_terrain_raycast::CursorPositionSS;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::public_walls::{WallPublicationDirty, WallSpatialHash};
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::resources::walls::{InnerWalls, WallState};
    use crate::resources::window::{WindowFocusEvent, WindowSize};
    use crate::systems::backend::input::button_input::MouseButtonInput;
    use crate::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved;
    use crate::systems::backend::input::{self, InputSystemSet};
    use crate::systems::brush_preview::OverwriteBrushPreview;
    use crate::systems::camera_rig::main_camera::MainCamera;
    use crate::systems::ui::hover_intersection::hover_intersection;
    use crate::systems::ui::resources::widget_params::PlayUiSound;
    use crate::systems::ui::ui_update::ui_update;

    fn flat_terrain() -> TerrainHeightGrid512 {
        TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 0.0,
            sampling_fix: true,
        }
    }

    fn setup() -> (World, Schedule) {
        bevy_tasks::ComputeTaskPool::get_or_init(|| {
            bevy_tasks::TaskPoolBuilder::new().num_threads(2).build()
        });
        let mut world = World::new();
        // A flat wall at the world origin with zero height so the lifted widget points sit
        // at y = 0.6 after the original focused-widget lift; the identity camera maps this to y = 20 pixels.
        let mut inner = InnerWalls::new();
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let wall = inner.new_wall_from_curve(WallState::new(curve, 0.0, 0, 0, false));
        let mut public = PublicWalls::default();
        // 与生产发布一样准备路径高度；None只提供未准备的几何，不能替代真实控件输入。
        let paths = crate::systems::wall::wall_state::path_preparation::PathRasters512::from_encoded(
            std::array::from_fn(|_| vec![0; 512 * 512].into()), false,
        ).unwrap();
        public.consume_with_paths(
            &mut inner,
            &flat_terrain(),
            Some(&paths),
            &WallPublicationDirty::default(),
            &WallSpatialHash::default(),
        );
        world.insert_resource(inner);
        world.insert_resource(public);
        world.insert_resource(AppMode::Editing(EditSubject::Wall(wall.0)));
        let mut camera = MainCamera::new(1.0);
        for offset in [0x40usize, 0x80, 0xc0] {
            camera.write_matrix(offset, &Mat4::IDENTITY);
        }
        world.insert_resource(camera);
        world.insert_resource(CursorTerrainRaycast::default());
        world.insert_resource(CursorPositionSS(Vec2::splat(50.0)));
        world.insert_resource(WindowSize::new(100, 100));
        world.insert_resource(UiState::at_startup());
        world.insert_resource(UiSignifierStream::default());
        input::register_input_events(&mut world);
        // The input installer's control_input_replay needs the country-core Toolbar resource.
        world.insert_resource(crate::resources::toolbar::Toolbar::default());
        world.init_resource::<crate::resources::mouse_capture::CursorCapturedPosition>();
        world.init_resource::<crate::resources::shader_refactoring_state::ShaderRefactoringState>();
        world.init_resource::<crate::resources::gui::EscMenuState>();
        world.init_resource::<crate::game_profile::GameProfile>();
        world.init_resource::<crate::game_config::GameConfig>();
        world.init_resource::<Events<MainCameraMoved>>();
        world.init_resource::<Events<FinishAdjWallHeightMode>>();
        world.init_resource::<Events<AdjustWallHeightUi>>();
        world.init_resource::<Events<InitAdjWallHeightMode>>();
        world.init_resource::<Events<WindowFocusEvent>>();
        world.init_resource::<Events<EscapeMenuJustOpened>>();
        world.init_resource::<Events<ShutdownGameCmd>>();
        world.init_resource::<Events<OverwriteBrushPreview>>();
        world.init_resource::<Events<OsCursorVisibilityCmd>>();
        world.init_resource::<Events<PlayUiSound>>();

        let mut schedule = Schedule::default();
        input::add_input_cpu_consumers(&mut schedule);
        schedule.add_systems(
            (hover_intersection, ui_update, ui_adjust_wall_height)
                .chain()
                .in_set(InputSystemSet::Consume),
        );
        (world, schedule)
    }

    fn frame(world: &mut World, schedule: &mut Schedule, pressed: bool, position: Vec2) {
        {
            let mut mouse = world.resource_mut::<MouseButtonInput>();
            mouse.clear();
            if pressed {
                mouse.press_button(MouseButton::Left);
            } else {
                mouse.release_button(MouseButton::Left);
            }
        }
        world.send_event(InputCursorMoved::Absolute(position));
        world
            .resource_mut::<bevy_time::Time>()
            .advance_by(std::time::Duration::from_millis(16));
        schedule.run(world);
        world.flush();
    }

    fn adjust_n(world: &World) -> usize {
        let e = world.resource::<Events<AdjustWallHeightUi>>();
        e.get_cursor().read(e).count()
    }
    fn init_n(world: &World) -> usize {
        let e = world.resource::<Events<InitAdjWallHeightMode>>();
        e.get_cursor().read(e).count()
    }
    fn finish_n(world: &World) -> usize {
        let e = world.resource::<Events<FinishAdjWallHeightMode>>();
        e.get_cursor().read(e).count()
    }

    #[test]
    fn real_mouse_press_reaches_adjust_and_init_events_through_the_picker() {
        let (mut world, mut schedule) = setup();
        let start = Vec2::new(50.0, 20.0);
        frame(&mut world, &mut schedule, false, start); // register widget + ray
        frame(&mut world, &mut schedule, false, start); // hover the widget
        frame(&mut world, &mut schedule, true, start); // press → stage 3
        let ui = world.resource::<UiState>();
        assert!(
            ui.hover_byte() >= 2,
            "press should activate the height widget; hover={} widget={:?}",
            ui.hover_byte(),
            ui.active_widget()
        );
        assert!(adjust_n(&world) >= 1, "expected AdjustWallHeightUi on press");
        assert!(init_n(&world) >= 1, "expected InitAdjWallHeightMode on press");
        // Drag: move the cursor while pressed, then release.
        frame(&mut world, &mut schedule, true, Vec2::new(60.0, 20.0)); // drag continue
        frame(&mut world, &mut schedule, true, Vec2::new(70.0, 20.0)); // drag continue
        frame(&mut world, &mut schedule, false, Vec2::new(70.0, 20.0)); // release
        // Release wraps stage 0/1 → case 3 (native unsigned clamp), so Finish IS emitted.
        assert!(
            adjust_n(&world) >= 2,
            "drag continues should emit AdjustWallHeightUi (got {})",
            adjust_n(&world)
        );
        assert!(
            finish_n(&world) >= 1,
            "release should emit FinishAdjWallHeightMode via the wrapping clamp"
        );
    }

    fn setup_full() -> (World, Schedule, WallId) {
        use crate::input::action_set::MainActionSet;
        use crate::resources::mouse_terrain_raycast::{CameraCursorRay, camera_mouse_ray};
        use crate::resources::onboarding::NotifyHintSystemShapeEditWasPerformed;
        use crate::resources::walls::decorator_archivist::LiveDecoratorArchivist;
        use crate::resources::walls::decorator_storage::DecoratorStorage;
        use crate::resources::window::AudioCursorMoved;
        use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};
        use crate::systems::collision::world::RaycastWorld;
        use crate::systems::wall::adjust_wall_height::AdjustWallHeightCmd;
        use crate::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl;
        use crate::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::{
            AdjWallHeightModeCache, ui_adjust_wall_height_mode,
        };
        use crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain;

        let (mut world, mut schedule) = setup();
        let wall = world
            .resource::<PublicWalls>()
            .wall_ids()
            .next()
            .expect("setup built a wall");
        world.init_resource::<MainActionSet>();
        world.init_resource::<CameraCursorRay>();
        world.init_resource::<RaycastWorld>();
        #[cfg(feature = "native-physics")]
        world.init_resource::<crate::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
        world.init_resource::<LiveDecoratorArchivist>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<AdjWallHeightModeCache>();
        world.insert_resource(WallPublicationTerrain(flat_terrain()));
        world.init_resource::<Events<AdjustWallHeightCmd>>();
        world.init_resource::<Events<RequestWallMaterialAudio>>();
        world.init_resource::<Events<PlayBuildingAudio>>();
        world.init_resource::<Events<AudioCursorMoved>>();
        world.init_resource::<Events<NotifyHintSystemShapeEditWasPerformed>>();
        // History recording target for the finish leg.
        let mut history = crate::resources::history::History::<()>::after_observed_future_clear();
        history.start_input_recording(0).unwrap();
        world.insert_resource(history);
        // Undo/redo application pipeline: history_process → replay_wall → pop.
        world.init_resource::<crate::systems::history::PendingHistoryCommands>();
        world.insert_resource(crate::resources::history::ReplayBuffer::new(1.0));
        world.init_resource::<Events<crate::systems::mode_manager::HistoryCommand>>();
        world.init_resource::<Events<crate::systems::history::history_process::DebugHistoryMsg>>();
        world.init_resource::<Events<crate::systems::history::history_process::NewReplayCommandEvent>>();
        world.init_resource::<Events<crate::systems::audio::PlayReplayAudio>>();
        world.init_resource::<Events<crate::systems::timelapse_replay::InterruptTimelapse>>();
        world.init_resource::<Events<crate::systems::history::HistoryReplayFailed>>();
        world.init_resource::<Events<crate::systems::history::replay_wall::WallHistoryReplayAudio>>();
        world.init_resource::<Events<crate::systems::history::replay_wall::WallHistoryRestoreColor>>();
        world.init_resource::<Events<crate::systems::history::ReplayBucketFinished>>();
        world.init_resource::<crate::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>();

        schedule.add_systems(
            (
                camera_mouse_ray,
                ui_adjust_wall_height_mode,
                adjust_wall_height_impl,
                crate::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::history_record_adjust_wall_height::<()>,
                crate::systems::history::history_process_system::<()>,
                crate::systems::history::replay_wall::replay_wall,
                crate::systems::history::pop_empty_replay_bucket,
            )
                .chain()
                .in_set(InputSystemSet::Consume)
                .after(ui_adjust_wall_height),
        );
        (world, schedule, wall)
    }

    /// Real input → AdjustWallHeightUi/Finish → mode → AdjustWallHeightCmd → impl → `InnerWalls`
    /// height; Finish → history_record → `History`; then real undo/redo
    /// (`history_process` → `replay_wall`) restores and re-applies the height.
    #[test]
    fn real_input_reaches_history_and_undo_redo_applies_to_inner_walls() {
        use crate::resources::history::EditType;
        use crate::resources::history::wall_edit::WallHistoryEdit;
        let (mut world, mut schedule, wall) = setup_full();
        let start = Vec2::new(50.0, 20.0);
        frame(&mut world, &mut schedule, false, start); // register
        frame(&mut world, &mut schedule, false, start); // hover
        frame(&mut world, &mut schedule, true, start); // press
        frame(&mut world, &mut schedule, true, Vec2::new(50.0, 10.0)); // drag init
        frame(&mut world, &mut schedule, true, Vec2::new(50.0, 0.0)); // drag update
        frame(&mut world, &mut schedule, false, Vec2::new(50.0, 0.0)); // release → finish
        let height = world.resource::<InnerWalls>().get(wall).map(|s| s.height());
        assert!(
            height.is_some_and(|h| h != 0.0),
            "InnerWalls height should change from 0 via mode→impl; got {height:?}"
        );
        let (from, to) = {
            let history = world.resource::<crate::resources::history::History<()>>();
            let entry = history.pending().expect("pending entry");
            let EditType::Wall(compound) = &entry.edits[0] else {
                panic!("expected wall edit")
            };
            let WallHistoryEdit::AdjustHeight { from, to, .. } = &compound.groups[0][0] else {
                panic!("expected AdjustHeight edit")
            };
            (*from, *to)
        };
        world
            .resource_mut::<crate::resources::history::History<()>>()
            .finish_input_recording()
            .expect("history finish succeeds")
            .expect("committed entry");
        // Real undo via the canonical HistoryCommand::UNDO ([0,0,0,0]). No real producer sets
        // byte2 (the consumer's instant flag), so drive the animated replay over many frames
        // until the height settles at the pre-edit value.
        world.send_event(crate::systems::mode_manager::HistoryCommand::UNDO);
        for _ in 0..400 {
            tick(&mut world, &mut schedule);
        }
        let after_undo = world
            .resource::<InnerWalls>()
            .get(wall)
            .map(|s| s.height())
            .unwrap();
        assert!(
            (after_undo - from).abs() < 1e-3,
            "undo should restore pre-edit height {from}; got {after_undo}"
        );
        // Real redo via the canonical HistoryCommand::REDO ([0,1,0,1]), animated to completion.
        world.send_event(crate::systems::mode_manager::HistoryCommand::REDO);
        for _ in 0..400 {
            tick(&mut world, &mut schedule);
        }
        let after_redo = world
            .resource::<InnerWalls>()
            .get(wall)
            .map(|s| s.height())
            .unwrap();
        assert!(
            (after_redo - to).abs() < 1e-3,
            "redo should re-apply edited height {to}; got {after_redo}"
        );
    }

    fn tick(world: &mut World, schedule: &mut Schedule) {
        world
            .resource_mut::<bevy_time::Time>()
            .advance_by(std::time::Duration::from_millis(16));
        schedule.run(world);
        world.flush();
    }
}
