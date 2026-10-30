//! B原14099ca60：自动clutter的真实父子映射、拾取、描边和开始移动。
//!
//! 原13组SystemParam按PDB恢复；signifier流因现有安全所有权API使用ResMut。
//! 父映射每帧由真实组件查询重建，最新右键目标同时校验Entity世代与删除标记。
//! 原source四路跳转分别显示装饰轮廓、木门顶边、共享点列或无来源；
//! 激活后共同显示圆形轮廓与link图标，拖拽写完整StartPlace事件和历史通知。
//! 原相机朝向、描边工厂及音效由相同canonical资源/消费者接续；
//! 未完成上游AutoClutterConvertable producer的业务不能由人工索引表代替。
use crate::{
    components::{
        Ui2dImage, Ui2dWsExtent, WoodenGate,
        drawable::{
            AutoClutter, AutoClutterConvertableToDecoClutter, AutoClutterDestination,
            AutoClutterSource, NaniSolidTexture,
        },
    },
    resources::{
        clutter_meshes::{
            ClutterMarkedForDeletion, ClutterMesh, ClutterMeshes, DecoratorAutoClutterChildren,
            clutter_mesh_ty_data,
        },
        history::History,
        mouse_terrain_raycast::CameraCursorRay,
        onboarding::NotifyHintSystemClutterWasMoved,
        roofs::Roof,
        walls::{
            cached_decorator_transforms::CachedDecoratorTransforms,
            decorator::{DecoratorId, DecoratorRank},
            decorator_storage::DecoratorStorage,
            decorator_subtype::{DecoratorLibKey, DerivedDecoratorInfoResources},
            public_walls::PublicWalls,
        },
    },
    systems::{
        audio::PlayDecoratorSoundCmd,
        camera_rig::MainCamera,
        collision::world::{RaycastWorld, RaycastWorldResult},
        ui::{
            resources::{
                ui_state::UiState,
                widget_params::{WidgetFlags, WidgetGeometry, WidgetParams},
            },
            signifiers::{
                SignifierEntityBuilder, SignifierFactory, UiSignifierStream,
                signifier_factory::SignifierMesh,
            },
        },
    },
    utils::{
        components::transform::Transform,
        debug_viz::{DebugColor, DebugLine},
    },
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::Arc;

use crate::systems::mode_manager::{AppMode, EraserLayer};

/// 原B事件只携带世代完整的Entity；EventId由Events单独拥有。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct EvRightClickedAutoClutter {
    pub entity: u64,
}

/// B1408b83d0/2748B: exact original name parser, shared by the actual
/// parent/RMB consumers. This does not search caller-modified mesh catalogs.
pub(super) fn clutter_mesh_type_from_name(
    name: &str,
) -> crate::systems::mode_manager::ClutterMeshTy {
    let tag = match name {
        "clutter/plant_pot_v1" => 0,
        "clutter/ladder" => 1,
        "clutter/wheel" => 2,
        "clutter/pitchfork" => 3,
        "clutter/shovel" => 4,
        "clutter/box_w_stuff" => 5,
        "clutter/watering_can" => 6,
        "clutter/firewood" => 7,
        "clutter/bucket" => 8,
        "clutter/birdnest" => 9,
        "clutter/bench" => 10,
        "clutter/bench_w_basket" => 11,
        "clutter/bench_short" => 12,
        "clutter/barrel" => 13,
        "clutter/haystack" => 14,
        "clutter/plant_pot_v2" => 15,
        "clutter/birdhouse" => 16,
        "clutter/basket" => 17,
        "clutter/basket_w_food" => 18,
        "clutter/barrel_w_food_basket" => 19,
        "clutter/potato_sack" => 20,
        "clutter/road_sign" => 21,
        "clutter/sunflower" => 22,
        "clutter/cart" => 23,
        "clutter/cart2" => 24,
        "clutter/crates_w_tools" => 25,
        "clutter/crates_w_flowers" => 26,
        "clutter/window_flowerpot" => 27,
        "clutter/window_shutter" => 28,
        "clutter/reed" => 30,
        "clutter/lilypad" => 31,
        "fallen_leaves" => 32,
        "fallen_petals" => 33,
        "olden_glade_rock" => 34,
        "tree_log" => 35,
        "tree_stump" => 36,
        "clutter/pumpkin" => 37,
        "clutter/pumpkin2" => 38,
        "clutter/pumpkin_w_face" => 39,
        "clutter/cart_w_pumpkins" => 40,
        "clutter/crates_w_pumpkins" => 41,
        "clutter/barrel_w_candles" => 43,
        "clutter/anvil" => 44,
        "clutter/fabric_rolls" => 45,
        "clutter/stall_bread" => 46,
        "clutter/stall_veggies" => 47,
        "clutter/crate_fish" => 48,
        "clutter/rack_brooms" => 49,
        "clutter/flowerbox" => 50,
        "clutter/door_bell" => 51,
        "clutter/door_krans" => 52,
        "clutter/door_krans_autumn" => 53,
        "clutter/cart_w_pumpkins_regular" => 54,
        "clutter/crates_w_pumpkins_regular" => 55,
        _ => unreachable!(),
    };
    crate::systems::mode_manager::ClutterMeshTy::from_native_tag(tag)
        .expect("original ClutterMeshTy name arm has a valid tag")
}

/// Original B InstaCreateDeco: an actual creation command, not the right-click
/// input. Native EventId and inactive extra/padding bytes are not business data.
#[derive(Event, Debug)]
pub struct InstaCreateDeco {
    /// Existing source component's 24-byte destination union. Semantic decoding
    /// remains with placement; never synthesize it from an auto-clutter ID.
    pub destination: [u64; 3],
    pub extra: Option<crate::resources::walls::decorator::DecoratorExtraInfo>,
    pub rotation: f32,
    pub position: glam::Vec3,
    pub ty: crate::systems::mode_manager::ClutterMeshTy,
}

/// Original B fixed-size stair creation business value. Its typed native Events
/// destructor releases only two 120-byte-stride buffers, with no element/Arc/Vec
/// destruction. No native pointer is converted to a Rust reference or owner.
/// Destination enums and seven color bytes still need semantic recovery; the
/// original placement consumer is not reconstructed by this transport type.
/// No Default/Clone/Copy is supplied for unknown active application data.
#[derive(Event, Debug)]
pub struct InstaCreateStairPoint {
    observed: [u8; 0x70],
}
impl InstaCreateStairPoint {
    /// Accept a complete already observed payload, excluding its EventId.
    /// The writer always supplies kind0x3f, also checked by original placement.
    pub fn from_observed(observed: [u8; 0x70]) -> Self {
        assert_eq!(observed[0x68], 0x3f, "original stair creation kind");
        Self { observed }
    }
    pub fn observed(&self) -> &[u8; 0x70] {
        &self.observed
    }
}

/// 原141327570复制96B业务值，EventInstance另有EventId且步长112B。
/// rotation@0、widget@16、完整碰撞值@24、mesh type@88；Rust采用语义值，
/// 不承诺重构后的碰撞结构与原私有enum的布局相同，不拷贝未激活分支或padding。
/// 原1407af340读取此事件启动放置，末尾unit事件属于引导通知，不能混为一谈。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct StartPlaceDecoClutterInteraction {
    pub relative_rotation: glam::Quat,
    pub widget: crate::systems::ui::resources::ui_state::WidgetId,
    pub hit: crate::systems::collision::world::RaycastWorldResult,
    pub ty: crate::systems::mode_manager::ClutterMeshTy,
}

/// DisplayChildAutoClutterInfo（O：0x60 B；`grow_one<DisplayChildAutoClutterInfo>`）。
#[derive(Clone, Debug)]
pub struct DisplayChildAutoClutterInfo {
    pub mesh: ClutterMesh,
    pub nani_name: &'static str,
    pub solid_texture: Option<NaniSolidTexture>,
    pub transform: Transform,
    pub progress: f32,
}

/// `ui_convert_autoclutter_to_decoclutter` @0x14099ca60。
pub fn ui_convert_autoclutter_to_decoclutter<S: Send + Sync + 'static>(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
    raycasts: Res<RaycastWorld>,
    ray: Res<CameraCursorRay>,
    (mut starts, mut right_clicks, mut creates): (
        EventWriter<StartPlaceDecoClutterInteraction>,
        EventReader<EvRightClickedAutoClutter>,
        EventWriter<InstaCreateDeco>,
    ),
    (mut deleted, mut parents, query, meshes): (
        ResMut<super::DeletedAutoClutter>,
        ResMut<DecoratorAutoClutterChildren>,
        Query<
            (
                Entity,
                &AutoClutter,
                &AutoClutterConvertableToDecoClutter,
                &Transform,
            ),
            Without<ClutterMarkedForDeletion>,
        >,
        Res<ClutterMeshes>,
    ),
    decorators: Res<DecoratorStorage>,
    transforms: Res<CachedDecoratorTransforms>,
    derived: DerivedDecoratorInfoResources,
    (walls, mut history, mut records, roofs, mut hints, camera): (
        Res<PublicWalls>,
        ResMut<History<S>>,
        EventWriter<super::defered_autoclutter_history::RecordAutoClutterHistory>,
        Query<&Roof>,
        EventWriter<NotifyHintSystemClutterWasMoved>,
        Res<MainCamera>,
    ),
    gates: Query<&WoodenGate>,
    mut sounds: EventWriter<PlayDecoratorSoundCmd>,
) {
    // 原99cdd1..d646每帧清空后遍历真实Query，不依赖点击或镜像实体表。
    parents.0.clear();
    for (_, clutter, source, transform) in &query {
        if let AutoClutterSource::Id {
            tag, native_u32_10, ..
        } = &source.source
            && tag.encoded() < 3
        {
            let data = clutter_mesh_ty_data(clutter_mesh_type_from_name(source.mesh_name), &meshes);
            parents.maintain(
                *native_u32_10,
                DisplayChildAutoClutterInfo {
                    mesh: data.mesh.clone(),
                    nani_name: data.nani_name,
                    solid_texture: data.solid_texture,
                    transform: *transform,
                    progress: clutter.progress,
                },
            );
        }
    }

    // 原只取最新事件；Query::get同时验证世代、必需组件和删除过滤。
    if let Some(entity) = right_clicks
        .read()
        .last()
        .and_then(|click| Entity::try_from_bits(click.entity).ok())
        && let Ok((_, clutter, source, transform)) = query.get(entity)
    {
        super::rmb_autoclutter::handle_rmb_autoclutter::<S>(
            &mode,
            clutter,
            source,
            transform,
            &walls,
            &roofs,
            &mut history,
            &mut deleted,
            &mut creates,
        );
        return;
    }

    let disc = mode.native_tag();
    if disc != 0 && !matches!(*mode, AppMode::Eraser(EraserLayer::Bulldoze)) {
        return;
    }
    // 原直接Rapier mask=0x8f3，不附加terrain求交或simple predicate。
    let hit = raycasts.raycast(ray.origin, ray.dir, 0x8f3);
    if hit.is_miss() || hit.handle[0] as u8 != 12 {
        return;
    }
    let Ok(entity) = Entity::try_from_bits(hit.handle[1]) else {
        return;
    };
    let Ok((_, clutter, source, transform)) = query.get(entity) else {
        return;
    };
    let delta = ray.origin - hit.position;
    let distance = (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt();
    let mut params =
        WidgetParams::default().geometry(WidgetGeometry::TerrainCanvas { distance, layer: 0 });
    params.flags = WidgetFlags(3);
    let context = ui.add(&format!("auto_clutter_{entity}"), params);
    if !context.is_active() {
        return;
    }

    match &source.source {
        AutoClutterSource::Id {
            tag,
            native_u64_08,
            native_u32_10,
        } if tag.encoded() < 3 => {
            let id = DecoratorId(*native_u32_10);
            if let Some(info) = decorators.decorator_info(id) {
                let transform = transforms.transform(id.0);
                let derived = derived.as_view();
                let subtype = derived.subtype(id);
                derived
                    .mesh_library()
                    .get_outline(DecoratorLibKey::new(subtype.0, info.rank))
                    .display(
                        &glam::Affine3A::from_scale_rotation_translation(
                            transform.scale,
                            transform.rotation,
                            transform.translation,
                        ),
                        &mut stream,
                        id,
                        DecoratorRank(info.rank),
                        ((info.u20 >> 16) as u8) < 2,
                        true,
                    );
            }
            let _ = native_u64_08; // 此原分支仅以DecoratorId查storage。
        }
        AutoClutterSource::Id {
            native_u64_08,
            native_u32_10,
            ..
        } => {
            for gate in &gates {
                if gate.wall.0 == *native_u64_08 && gate.seed == *native_u32_10 {
                    let mut points: Vec<Vec3> = gate
                        .top_positions
                        .iter()
                        .copied()
                        .map(Vec3::from_array)
                        .collect();
                    let first = *points.first().expect("original gate outline has a point");
                    points.push(first);
                    show_source_line(points, true, &mut stream);
                }
            }
        }
        AutoClutterSource::Points(points) => {
            show_source_line(points.to_vec(), true, &mut stream);
        }
        AutoClutterSource::None => {}
    }
    let metadata = clutter_mesh_ty_data(clutter_mesh_type_from_name(source.mesh_name), &meshes);
    let footprint = metadata.mesh.bounds.footprint.size;
    let radius = (footprint.x + footprint.y) * 0.5 * 0.5 + 0.1;
    let circle = ::utils::geometry::Circle2d::new(
        Vec2::new(transform.translation.x, transform.translation.z),
        radius,
    );
    show_source_line(
        circle.as_points3(0.1, transform.translation.y).collect(),
        false,
        &mut stream,
    );
    let size = if radius + -0.05 < 0.5 {
        radius + -0.05
    } else {
        0.5
    };
    let position = transform.translation;
    let factory = SignifierFactory::from_closure(
        &format!("move_auto_clutter{entity:?}{position:?}{size:?}"),
        Arc::new(move |meshes, shaders, commands| {
            let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
                .custom_shader("vertex_color_shader")
                .animation_duration(0.0)
                .build(meshes, shaders, commands);
            commands.entity(entity).insert((
                Ui2dWsExtent {
                    extent: Vec2::splat(size),
                    position,
                },
                Ui2dImage::new("link"),
            ));
            vec![entity]
        }),
    );
    stream.write_factory(&factory);
    if context.dragged() != 3 {
        return;
    }
    history
        .start_input_recording(mode.native_tag())
        .expect("original non-Replay recording invariant");
    records.write(super::defered_autoclutter_history::RecordAutoClutterHistory(clutter.key));
    let ty = clutter_mesh_type_from_name(source.mesh_name);
    if matches!(*mode, AppMode::Eraser(EraserLayer::Bulldoze)) {
        let mut sound = PlayDecoratorSoundCmd::at(0x20, transform.translation);
        sound.selector = ty as u8;
        sounds.write(sound);
        return;
    }
    sounds.write(PlayDecoratorSoundCmd::at(0x25, transform.translation));
    starts.write(StartPlaceDecoClutterInteraction {
        relative_rotation: relative_clutter_rotation(
            camera.xz_projective_rotation(),
            transform.rotation,
        ),
        widget: context.id,
        hit: starting_hit(source.destination, transform, &raycasts),
        ty,
    });
    hints.write(NotifyHintSystemClutterWasMoved);
}

fn show_source_line(points: Vec<Vec3>, dashed: bool, stream: &mut UiSignifierStream) {
    DebugLine {
        points,
        colors: None,
        color: DebugColor::White,
        line_width: 2.0,
        ui_pass: true,
        dashed,
    }
    .show(stream);
}

/// 原140976ca0按Quat内存xyzw逐lane读取，不能按旧adapter的误导字段名重排。
pub(super) fn original_heading(q: glam::Quat) -> f32 {
    let [y, x] = original_heading_inputs(q);
    y.atan2(x)
}

/// 原ui_autoclutter源码403行的屋顶相对朝向，保留两次atan2和半角重建。
pub(super) fn get_clutter_rot_in_roof_space(rotation: glam::Quat, roof: &Roof) -> f32 {
    let delta = (original_heading(rotation) - roof.rotation_yaw_radians()) % core::f32::consts::TAU;
    let sin = (delta * 0.5).sin();
    let cos = (delta * 0.5).cos();
    original_heading(glam::Quat::from_xyzw(0.0, sin, 0.0, cos))
}

fn relative_clutter_rotation(camera: glam::Quat, clutter: glam::Quat) -> glam::Quat {
    clutter * camera.conjugate()
}

fn original_heading_inputs(q: glam::Quat) -> [f32; 2] {
    let [x, y, z, w] = q.to_array();
    let squared = z * z + (y * y + x * x);
    let term = z * -1.0 + (y * -0.0 + x * -0.0);
    let twice = term + term;
    [
        -((w + w) * (y * -1.0 - z * -0.0) + (twice * x + (w * w - squared) * -0.0)),
        -((w + w) * (x * -0.0 - y * -0.0) + (twice * z + (w * w - squared) * -1.0)),
    ]
}

fn starting_hit(
    destination: AutoClutterDestination,
    transform: &Transform,
    raycasts: &RaycastWorld,
) -> RaycastWorldResult {
    let (handle, normal, discriminant, extra) = match destination {
        AutoClutterDestination::Terrain => ([11, 0, 0], Vec3::Y, 0, [0.0; 2]),
        AutoClutterDestination::OnBuilding {
            native_u32_04: 1,
            wall,
            ..
        } => ([0, wall.0, 0], transform.rotation * Vec3::Z, 0, [0.0; 2]),
        AutoClutterDestination::OnBuilding { wall, .. } => {
            let hit = raycasts.raycast(
                transform.translation + Vec3::new(0.0, 0.05, 0.0),
                Vec3::new(-0.0, -1.0, -0.0),
                2,
            );
            if !hit.is_miss() && hit.discriminant & 1 != 0 && hit.handle[0] as u8 == 2 {
                (hit.handle, Vec3::Y, 1, hit.extra)
            } else {
                ([2 | (1 << 8), wall.0, 0], Vec3::Y, 1, [0.5, 0.5])
            }
        }
    };
    RaycastWorldResult {
        handle,
        position: transform.translation,
        normal,
        toi: 0.0,
        discriminant,
        extra,
    }
}

#[cfg(test)]
mod lifecycle_integration_tests {
    use super::*;
    use crate::resources::{
        history::{EditType, auto_clutter_edit::AutoClutterEdit},
        walls::WallId,
    };
    use crate::{
        components::drawable::AutoClutterSourceIdTag,
        resources::walls::decorator_subtype::{DecoratorMeshLibrary, DerivedDecoratorSubtypes},
        systems::{
            history::recording::{history_finish_recording_system, register_recording_events},
            mode_manager::ClutterMeshTy,
            ui_clutter::registration::register_ui_clutter_resources,
        },
    };

    fn world() -> World {
        let mut world = World::new();
        register_ui_clutter_resources(&mut world);
        register_recording_events(&mut world);
        world.insert_resource(AppMode::Path);
        world.insert_resource(History::<()>::empty_glade());
        world.insert_resource(UiState::at_startup());
        world.insert_resource(UiSignifierStream::default());
        world.insert_resource(RaycastWorld::default());
        world.insert_resource(CameraCursorRay::default());
        world.insert_resource(CachedDecoratorTransforms::default());
        world.insert_resource(DerivedDecoratorSubtypes::default());
        world.insert_resource(DecoratorMeshLibrary::default());
        world.insert_resource(PublicWalls::default());
        world.insert_resource(MainCamera::default());
        world.init_resource::<Events<NotifyHintSystemClutterWasMoved>>();
        world.init_resource::<Events<PlayDecoratorSoundCmd>>();
        let metadata = std::array::from_fn(|_| {
            let mut mesh = ClutterMesh::omitted_in_demo("clutter/bench", 2, None, false, false);
            mesh.bounds =
                ::utils::geometry::Rectangle3d::new(Vec3::ZERO, Vec3::new(2.0, 3.0, 4.0), Vec2::X);
            mesh
        });
        world.insert_resource(ClutterMeshes::from_parts(metadata, Vec::new()));
        world
    }

    fn spawn(world: &mut World, key: u64, source: AutoClutterSource) -> Entity {
        let bounds = world
            .resource::<ClutterMeshes>()
            .mesh(ClutterMeshTy::Bench)
            .bounds;
        world
            .spawn((
                AutoClutter {
                    key,
                    timer: 0.0,
                    progress: 0.75,
                    target_scale: 1.0,
                    native_group: 2,
                    native_flag_15: false,
                },
                AutoClutterConvertableToDecoClutter {
                    source,
                    destination: AutoClutterDestination::Terrain,
                    mesh_name: "clutter/bench",
                    bounds,
                },
                Transform::from_translation(Vec3::new(1.0, 2.0, 3.0)),
            ))
            .id()
    }

    fn schedule() -> Schedule {
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_convert_autoclutter_to_decoclutter::<()>,
                super::super::defered_autoclutter_history::defered_autoclutter_history::<()>,
                history_finish_recording_system::<()>,
            )
                .chain(),
        );
        schedule
    }

    #[test]
    fn original_sse_heading_inputs_match_all_native_words_without_float_tolerance() {
        let cases: &[([u32; 4], [u32; 2])] = include!(
            "../../../../../reports/module-recovery-2026-10-23/m01-ui-convert-heading-native-cases.rs"
        );
        assert_eq!(cases.len(), 515);
        for (q, expected) in cases {
            let q = glam::Quat::from_array(q.map(f32::from_bits));
            assert_eq!(
                original_heading_inputs(q).map(f32::to_bits),
                *expected,
                "original Quat lanes {:?}",
                q.to_array().map(f32::to_bits)
            );
        }
    }

    #[test]
    fn full_original_vector_leaf_and_relative_quaternion_match_native_words() {
        type Normal = ([u32; 4], [u32; 3]);
        type Rotation = ([u32; 4], [u32; 4], [u32; 4]);
        let (normals, rotations): (&[Normal], &[Rotation]) = include!(
            "../../../../../reports/module-recovery-2026-10-23/m01-ui-convert-pose-native-cases.rs"
        );
        assert_eq!((normals.len(), rotations.len()), (515, 515));
        let world = RaycastWorld::default();
        for (q, expected) in normals {
            let transform = Transform {
                rotation: glam::Quat::from_array(q.map(f32::from_bits)),
                translation: Vec3::new(1.0, 2.0, 3.0),
                scale: Vec3::ONE,
            };
            let hit = starting_hit(
                AutoClutterDestination::OnBuilding {
                    native_u32_04: 1,
                    coordinates: Vec2::new(0.3, 0.4),
                    wall: WallId(9),
                },
                &transform,
                &world,
            );
            assert_eq!(
                hit.normal.to_array().map(f32::to_bits),
                *expected,
                "原14099c420 quaternion {q:?}"
            );
        }
        for (q, camera, expected) in rotations {
            assert_eq!(
                relative_clutter_rotation(
                    glam::Quat::from_array(camera.map(f32::from_bits)),
                    glam::Quat::from_array(q.map(f32::from_bits))
                )
                .to_array()
                .map(f32::to_bits),
                *expected,
                "原14099e61d quaternion {q:?}, camera {camera:?}"
            );
        }
    }

    #[test]
    fn actual_query_rebuilds_parent_metadata_each_frame_without_any_click() {
        let mut world = world();
        let mut schedule = schedule();
        let child = spawn(
            &mut world,
            73,
            AutoClutterSource::Id {
                tag: AutoClutterSourceIdTag::Native2,
                native_u64_08: 5,
                native_u32_10: 9,
            },
        );
        let gate = spawn(
            &mut world,
            74,
            AutoClutterSource::Id {
                tag: AutoClutterSourceIdTag::Native3,
                native_u64_08: 5,
                native_u32_10: 11,
            },
        );
        schedule.run(&mut world);
        let parents = world.resource::<DecoratorAutoClutterChildren>();
        let child_info = &parents.parent_of(9).unwrap()[0];
        assert_eq!(child_info.mesh.mesh_name, "clutter/bench");
        assert_eq!(child_info.nani_name, "nani_clutter_SolidAlbedoMap");
        assert_eq!(child_info.progress, 0.75);
        assert_eq!(child_info.transform.translation, Vec3::new(1.0, 2.0, 3.0));
        assert!(parents.parent_of(11).is_none());
        world.get_mut::<Transform>(child).unwrap().translation.x = 7.0;
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource::<DecoratorAutoClutterChildren>()
                .parent_of(9)
                .unwrap()[0]
                .transform
                .translation
                .x,
            7.0
        );
        world.entity_mut(child).insert(ClutterMarkedForDeletion);
        schedule.run(&mut world);
        assert!(
            world
                .resource::<DecoratorAutoClutterChildren>()
                .0
                .is_empty()
        );
        world.despawn(child);
        world.despawn(gate);
        schedule.run(&mut world);
        assert!(
            world
                .resource::<DecoratorAutoClutterChildren>()
                .0
                .is_empty()
        );
    }

    #[test]
    fn actual_latest_entity_converts_source_id_and_commits_real_history() {
        let mut world = world();
        let mut schedule = schedule();
        let entity = spawn(&mut world, 73, AutoClutterSource::None);
        world.send_event(EvRightClickedAutoClutter {
            entity: entity.to_bits(),
        });
        schedule.run(&mut world);
        let events = world.resource::<Events<InstaCreateDeco>>();
        let mut cursor = events.get_cursor();
        let output = cursor.read(events).next().unwrap();
        assert_eq!(output.destination, [2, 0, 0]);
        assert_eq!(output.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(output.rotation.to_bits(), (-0.0f32).to_bits());
        assert_eq!(output.ty, ClutterMeshTy::Bench);
        assert!(
            world
                .resource::<super::super::DeletedAutoClutter>()
                .is_deleted(73)
        );
        let history = world.resource::<History<()>>();
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(matches!(&history.ledger().entries[0].edits[..],
            [EditType::AutoClutter(edit)] if **edit == AutoClutterEdit::Delete { id: 73 }));
    }

    #[test]
    fn non_unit_snapshot_uses_only_its_real_history_owner_through_rmb_and_commit() {
        struct SnapshotWithPayload {
            _generation: u64,
        }
        let mut world = world();
        assert!(world.remove_resource::<History<()>>().is_some());
        world.insert_resource(History::<SnapshotWithPayload>::empty_glade());
        let entity = spawn(&mut world, 73, AutoClutterSource::None);
        world.send_event(EvRightClickedAutoClutter {
            entity: entity.to_bits(),
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_convert_autoclutter_to_decoclutter::<SnapshotWithPayload>,
                super::super::defered_autoclutter_history::defered_autoclutter_history::<
                    SnapshotWithPayload,
                >,
                history_finish_recording_system::<SnapshotWithPayload>,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert!(!world.contains_resource::<History<()>>());
        assert_eq!(world.resource::<Events<InstaCreateDeco>>().len(), 1);
        let history = world.resource::<History<SnapshotWithPayload>>();
        assert!(!history.is_recording());
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(matches!(&history.ledger().entries[0].edits[..],
            [EditType::AutoClutter(edit)] if **edit == AutoClutterEdit::Delete { id: 73 }));
    }

    #[test]
    fn actual_building_rmb_resolves_generation_bearing_roof_and_preserves_creation_context() {
        let mut world = world();
        let entity = spawn(&mut world, 73, AutoClutterSource::None);
        world
            .get_mut::<AutoClutterConvertableToDecoClutter>(entity)
            .unwrap()
            .destination = AutoClutterDestination::OnBuilding {
            native_u32_04: 1,
            coordinates: Vec2::new(0.25, 0.75),
            wall: WallId(9),
        };
        let half = f32::from_bits(0x3f3504f3);
        world.get_mut::<Transform>(entity).unwrap().rotation =
            glam::Quat::from_xyzw(0.0, half, 0.0, half);
        let mut shape = [0u8; 28];
        shape[0] = 1;
        shape[4..8].copy_from_slice(&1.0f32.to_le_bytes());
        let roof = Roof::new(
            9u64.to_le_bytes(),
            [0; 28],
            shape,
            0.0f32.to_le_bytes(),
            0u32.to_le_bytes(),
            0,
            [0; 12],
        );
        let roof_entity = world.spawn(roof).id();
        world
            .resource_mut::<PublicWalls>()
            .associate_roof(WallId(9), roof_entity);
        world.send_event(EvRightClickedAutoClutter {
            entity: entity.to_bits(),
        });
        schedule().run(&mut world);
        let events = world.resource::<Events<InstaCreateDeco>>();
        let mut cursor = events.get_cursor();
        let create = cursor.read(events).next().unwrap();
        assert_eq!(
            create.destination,
            [
                1u64 << 32,
                u64::from(0.25f32.to_bits()) | (u64::from(0.75f32.to_bits()) << 32),
                9
            ]
        );
        assert_eq!(
            create.rotation.to_bits(),
            core::f32::consts::FRAC_PI_2.to_bits()
        );
        assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
    }

    #[test]
    fn building_rmb_keeps_original_missing_roof_failure_before_mutation() {
        let mut world = world();
        let entity = spawn(&mut world, 73, AutoClutterSource::None);
        world
            .get_mut::<AutoClutterConvertableToDecoClutter>(entity)
            .unwrap()
            .destination = AutoClutterDestination::OnBuilding {
            native_u32_04: 1,
            coordinates: Vec2::ZERO,
            wall: WallId(9),
        };
        world.send_event(EvRightClickedAutoClutter {
            entity: entity.to_bits(),
        });
        let mut schedule = schedule();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)))
                .is_err()
        );
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        assert!(
            !world
                .resource::<super::super::DeletedAutoClutter>()
                .is_deleted(73)
        );
        assert!(!world.resource::<History<()>>().is_recording());
    }

    #[test]
    fn latest_stale_generation_suppresses_previous_valid_and_does_not_replay() {
        let mut world = world();
        let mut schedule = schedule();
        let stale = spawn(&mut world, 71, AutoClutterSource::None);
        world.despawn(stale);
        let live = spawn(&mut world, 73, AutoClutterSource::None);
        world.send_event(EvRightClickedAutoClutter {
            entity: live.to_bits(),
        });
        world.send_event(EvRightClickedAutoClutter {
            entity: stale.to_bits(),
        });
        schedule.run(&mut world);
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        schedule.run(&mut world);
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        world.send_event(EvRightClickedAutoClutter {
            entity: live.to_bits(),
        });
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<InstaCreateDeco>>().len(), 1);
    }

    #[test]
    fn deletion_marker_excludes_even_a_live_generation_bearing_entity() {
        let mut world = world();
        let mut schedule = schedule();
        let entity = spawn(&mut world, 73, AutoClutterSource::None);
        world.entity_mut(entity).insert(ClutterMarkedForDeletion);
        world.send_event(EvRightClickedAutoClutter {
            entity: entity.to_bits(),
        });
        schedule.run(&mut world);
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        assert!(world.resource::<History<()>>().ledger().entries.is_empty());
    }

    #[cfg(feature = "native-physics")]
    fn picked_world(source: AutoClutterSource, phase: u8) -> (World, Entity) {
        use crate::systems::{
            collision::{
                query_policy::RaycastGeoSourceProjection,
                world::{RaycastMeshData, upsert_collider},
            },
            ui::resources::ui_state::{UiInteraction, WidgetId},
        };
        let mut world = world();
        world.insert_resource(AppMode::default());
        let entity = spawn(&mut world, 73, source);
        let mut owners = std::collections::HashMap::new();
        {
            let mut raycasts = world.resource_mut::<RaycastWorld>();
            let mut source = [0u8; 24];
            source[0] = 12;
            source[8..16].copy_from_slice(&entity.to_bits().to_le_bytes());
            upsert_collider(
                &mut raycasts,
                &mut owners,
                73,
                RaycastGeoSourceProjection(source),
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO)),
                Vec3::new(1.0, 2.0, 3.0),
                glam::Quat::IDENTITY,
            );
            raycasts.flush_colliders();
        }
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(1.0, 8.0, 3.0),
            dir: Vec3::NEG_Y,
        });
        let widget = WidgetId::from_name(&format!("auto_clutter_{entity}"));
        let mut ui = world.resource_mut::<UiState>();
        ui.replace_interaction(UiInteraction {
            native_tag: 2,
            native_phase: phase,
            widget,
        });
        ui.set_intersection_tag(0);
        (world, entity)
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn every_original_source_arm_reaches_its_real_outline_and_owned_factory() {
        use crate::resources::walls::{
            cached_decorator_transforms::CachedDecoratorTransform,
            decorator::DecoratorInfo,
            decorator_storage::DecoratorStorageAddr,
            decorator_subtype::{DecoratorOutline, DecoratorSubtype},
        };
        use crate::systems::ui::signifiers::UiSignifierEntry;
        for kind in 0..4 {
            let points: Arc<[Vec3]> = Arc::from([
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.0, 6.0),
                Vec3::new(7.0, 8.0, 9.0),
            ]);
            let weak = Arc::downgrade(&points);
            let source = match kind {
                0 => AutoClutterSource::Id {
                    tag: AutoClutterSourceIdTag::Native0,
                    native_u64_08: 5,
                    native_u32_10: 100,
                },
                1 => AutoClutterSource::Id {
                    tag: AutoClutterSourceIdTag::Native3,
                    native_u64_08: 5,
                    native_u32_10: 17,
                },
                2 => AutoClutterSource::Points(Arc::clone(&points)),
                _ => AutoClutterSource::None,
            };
            let (mut world, entity) = picked_world(source, 2);
            if kind == 0 {
                let id = DecoratorId(100);
                let address = DecoratorStorageAddr::Terrain;
                let mut storage = world.resource_mut::<DecoratorStorage>();
                storage.get_or_insert_default(address).insert(
                    id,
                    DecoratorInfo {
                        rank: 2,
                        ..Default::default()
                    },
                );
                storage.decorator_to_storage.insert(id, address);
                world
                    .resource_mut::<DerivedDecoratorSubtypes>()
                    .insert(100, DecoratorSubtype(7));
                world.resource_mut::<DecoratorMeshLibrary>().outline.insert(
                    DecoratorLibKey::new(7, 2),
                    DecoratorOutline::Lines(points.to_vec()),
                );
                world.resource_mut::<CachedDecoratorTransforms>().insert(
                    100,
                    CachedDecoratorTransform {
                        rotation: glam::Quat::IDENTITY,
                        translation: Vec3::ZERO,
                        scale: Vec3::ONE,
                    },
                );
            } else if kind == 1 {
                world.spawn(WoodenGate {
                    top_positions: points
                        .iter()
                        .copied()
                        .map(|point| point.to_array())
                        .collect(),
                    wall: WallId(5),
                    endpoint_directions: [[1.0, 0.0]; 2],
                    seed: 17,
                    clutter_version: 0,
                });
            }
            let mut schedule = schedule();
            schedule.run(&mut world);
            let entries = world.resource_mut::<UiSignifierStream>().take_entries();
            let names: Vec<_> = entries
                .iter()
                .map(|entry| {
                    let UiSignifierEntry::Factory(factory) = entry else {
                        panic!("原source必须提交真实工厂");
                    };
                    factory.name().unwrap().to_owned()
                })
                .collect();
            assert_eq!(names.len(), if kind == 3 { 2 } else { 3 });
            assert!(
                names
                    .iter()
                    .any(|name| name.starts_with(&format!("move_auto_clutter{entity:?}")))
            );
            assert!(
                world
                    .resource::<Events<StartPlaceDecoClutterInteraction>>()
                    .is_empty()
            );
            if kind == 2 {
                assert_eq!(Arc::strong_count(&points), 2);
                world.despawn(entity);
                drop(points);
                assert!(
                    weak.upgrade().is_none(),
                    "工厂持有Vec值，不能偷偷保留来源Arc"
                );
            }
        }
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn bulldoze_started_keeps_real_mesh_selector_without_starting_placement() {
        let (mut world, _) = picked_world(AutoClutterSource::None, 3);
        world.insert_resource(AppMode::Eraser(EraserLayer::Bulldoze));
        schedule().run(&mut world);
        let events = world.resource::<Events<PlayDecoratorSoundCmd>>();
        let mut cursor = events.get_cursor();
        let sound = cursor.read(events).next().unwrap();
        assert_eq!(
            (sound.kind, sound.selector),
            (0x20, ClutterMeshTy::Bench as u8)
        );
        assert_eq!(sound.position, Vec3::new(1.0, 2.0, 3.0));
        assert!(
            world
                .resource::<Events<StartPlaceDecoClutterInteraction>>()
                .is_empty()
        );
        assert!(
            world
                .resource::<Events<NotifyHintSystemClutterWasMoved>>()
                .is_empty()
        );
        assert!(world.resource::<History<()>>().is_recording());
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn picked_link_factory_builds_actual_ui_image_then_reclaims_transient_mesh() {
        use crate::{
            asset_libraries::{
                mesh_library::AssetMeshLibrary,
                shader_library::AssetShaderLibrary,
                transient_mesh::{TransientMesh, delete_dropped_transient_meshes},
                vao_library::AssetVAOLibrary,
            },
            render::{mesh::Mesh, shader::RasterShaderProgram, test_device::TestDevice},
            systems::ui::signifiers::UiSignifierEntry,
        };
        use bevy_ecs::world::CommandQueue;
        let (mut world, _) = picked_world(AutoClutterSource::None, 2);
        schedule().run(&mut world);
        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        let factory = entries
            .iter()
            .find_map(|entry| match entry {
                UiSignifierEntry::Factory(factory)
                    if factory.name().unwrap().starts_with("move_auto_clutter") =>
                {
                    Some(factory)
                }
                _ => None,
            })
            .unwrap();
        let mut meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &TestDevice(0),
            ),
            false,
        );
        let mut queue = CommandQueue::default();
        let entities = factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
        queue.apply(&mut world);
        assert_eq!(entities.len(), 1);
        let entity = entities[0];
        assert_eq!(world.get::<Ui2dImage>(entity).unwrap().name, "link");
        let extent = world.get::<Ui2dWsExtent>(entity).unwrap();
        assert_eq!(extent.extent, Vec2::splat(0.5));
        assert_eq!(extent.position, Vec3::new(1.0, 2.0, 3.0));
        assert!(world.get::<TransientMesh>(entity).is_some());
        let handle = *world
            .get::<crate::asset_libraries::Handle<Mesh>>(entity)
            .unwrap();
        assert!(meshes.get(handle).is_some());
        world.despawn(entity);
        delete_dropped_transient_meshes(&TestDevice(0), &mut meshes, &mut AssetVAOLibrary::new());
        assert!(meshes.get(handle).is_none());
    }

    #[cfg(feature = "native-physics")]
    #[test]
    fn real_ray_and_mouse_widget_start_drag_then_history_and_entity_unload() {
        use crate::systems::{
            collision::{
                query_policy::RaycastGeoSourceProjection,
                world::{RaycastMeshData, dispose_collider_mesh_instance, upsert_collider},
            },
            ui::resources::ui_state::{UiInteraction, WidgetId},
        };
        let mut world = world();
        let mut schedule = schedule();
        world.insert_resource(AppMode::default());
        let entity = spawn(&mut world, 73, AutoClutterSource::None);
        let mut sources = std::collections::HashMap::new();
        {
            let mut raycasts = world.resource_mut::<RaycastWorld>();
            let mut source = [0u8; 24];
            source[0] = 12;
            source[8..16].copy_from_slice(&entity.to_bits().to_le_bytes());
            upsert_collider(
                &mut raycasts,
                &mut sources,
                73,
                RaycastGeoSourceProjection(source),
                Arc::new(RaycastMeshData::new_cuboid_at(Vec3::splat(2.0), Vec3::ZERO)),
                Vec3::new(1.0, 2.0, 3.0),
                glam::Quat::IDENTITY,
            );
            raycasts.flush_colliders();
        }
        world.insert_resource(CameraCursorRay {
            origin: Vec3::new(1.0, 8.0, 3.0),
            dir: Vec3::NEG_Y,
        });
        let widget = WidgetId::from_name(&format!("auto_clutter_{entity}"));
        // 非活动输入仍注册真实距离volume，但不请求描边或开始移动。
        schedule.run(&mut world);
        assert!(world.resource::<UiState>().widget(widget).is_some());
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert!(
            world
                .resource::<Events<StartPlaceDecoClutterInteraction>>()
                .is_empty()
        );
        {
            let mut ui = world.resource_mut::<UiState>();
            ui.replace_interaction(UiInteraction {
                native_tag: 2,
                native_phase: 3,
                widget,
            });
            ui.set_intersection_tag(0);
        }
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 2);
        let events = world.resource::<Events<StartPlaceDecoClutterInteraction>>();
        let mut cursor = events.get_cursor();
        let start = cursor.read(events).next().unwrap();
        assert_eq!(start.widget, widget);
        assert_eq!(start.hit.handle[0], 11);
        assert_eq!(start.hit.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(start.hit.normal, Vec3::Y);
        assert_eq!(start.ty, ClutterMeshTy::Bench);
        assert_eq!(
            world
                .resource::<Events<NotifyHintSystemClutterWasMoved>>()
                .len(),
            1
        );
        assert!(world.resource::<History<()>>().is_recording());
        assert!(world.resource::<History<()>>().ledger().entries.is_empty());
        {
            let mut ui = world.resource_mut::<UiState>();
            let params = ui.widget(widget).unwrap();
            ui.replace_interaction_full(
                UiInteraction {
                    native_tag: 2,
                    native_phase: 1,
                    widget,
                },
                Some(params),
                [0; 56],
            );
        }
        schedule.run(&mut world);
        assert_eq!(world.resource::<History<()>>().ledger().entries.len(), 1);
        // 世代与物理资源分别回收；失效碰撞不能再指向已卸载组件。
        world.despawn(entity);
        dispose_collider_mesh_instance(
            &mut world.resource_mut::<RaycastWorld>(),
            &sources.remove(&73).unwrap(),
        );
        world.resource_mut::<RaycastWorld>().flush_colliders();
        world.resource_mut::<UiSignifierStream>().take_entries();
        schedule.run(&mut world);
        assert_eq!(world.resource::<UiSignifierStream>().committed_len(), 0);
        assert_eq!(
            world
                .resource::<Events<StartPlaceDecoClutterInteraction>>()
                .len(),
            1
        );
    }
}

#[cfg(test)]
mod original_name_lookup_tests {
    use super::clutter_mesh_type_from_name;

    #[test]
    fn original_fallthrough_arms_and_duplicate_stump_name_keep_their_types() {
        for (name, tag) in [
            ("tree_log", 35),
            ("tree_stump", 36),
            ("clutter/cart_w_pumpkins", 40),
            ("clutter/crates_w_pumpkins", 41),
            ("clutter/stall_veggies", 47),
            ("clutter/crate_fish", 48),
            ("clutter/door_krans_autumn", 53),
            ("clutter/cart_w_pumpkins_regular", 54),
        ] {
            assert_eq!(clutter_mesh_type_from_name(name) as u8, tag);
        }
        assert_ne!(clutter_mesh_type_from_name("tree_stump") as u8, 42);
    }

    #[test]
    fn unknown_or_normalized_names_keep_the_original_unreachable_failure() {
        for name in [
            "",
            "clutter/tree_stump",
            "TREE_STUMP",
            "tree_stump_v2",
            "clutter/window_flowerbeds",
            "caller_defined_mesh",
        ] {
            assert!(
                std::panic::catch_unwind(|| clutter_mesh_type_from_name(name)).is_err(),
                "unknown original name unexpectedly parsed: {name}"
            );
        }
    }
}
