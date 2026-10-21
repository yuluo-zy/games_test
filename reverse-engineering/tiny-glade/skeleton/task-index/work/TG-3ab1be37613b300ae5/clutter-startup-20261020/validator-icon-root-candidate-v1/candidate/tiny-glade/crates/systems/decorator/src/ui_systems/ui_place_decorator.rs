//! B `ui_systems/ui_place_decorator.rs` — 装饰放置交互（0x1407a9c10，O 结构；
//! 90KB fresh 反编译）。
//!
//! 主要观察（O）：`EvInitPlaceDecoratorInteractionState` 事件（读+写）初始化
//! 交互状态；`TransientDecoratorIdGen::new/generate`（0x1408339c0/0x1408339e0）
//! 分配瞬时 id；`convert_raycast_hit_to_decorator_dst`（0x14127c990）命中转
//! 目的；`calculate_decorator_interaction_intent` + `find_stair_placement_candidate`
//! （0x1407a7e80）楼梯候选；`show_decorator_icon`（0x1407adb20）/`decorator_cursor_icon`
//! （0x1407adcb0）；`apply_decorator_op`（0x141276e30）；dst 跳表 switch
//! （case 3..0x14 观察）；'?' 类型（info+0x69，I：0x70B 包裹记录类型字节）时
//! 遮罩 0x23 射线（排除装饰碰撞体）；事件面：InstaCreateStairPoint/InstaCreateDeco
//! （读）、RequestAppModeChange/PlayStairsAudioCmd/NotifyHintSystemStairsPlaced/
//! FeedbackInputOutOfBorder/DisplayOutOfBorder（写）。
//! `Local<PlayedConnectSoundLastFrame>`：连接音播放节流（I）。
use bevy_ecs::prelude::*;
use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder};
use country_core::resources::stairs::stairs_state::StairsState;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{DecoratorId, TransientDecoratorIdGen};
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::window::WindowSize;
use country_core::systems::audio::PlayStairsAudioCmd;
use country_core::systems::mode_manager::RequestAppModeChange;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

/// 初始化放置交互状态事件（lib.rs 符号；O：读+写对）。
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct EvInitPlaceDecoratorInteractionState;

/// `Local<PlayedConnectSoundLastFrame>`（I：连接音节流）。
#[derive(Default)]
pub struct PlayedConnectSoundLastFrame(pub bool);

/// 放置交互状态（I：当前放置 id/位置）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct PlaceDecoratorInteractionState {
    pub id: Option<DecoratorId>,
}

/// 瞬时装饰 id 基数（I：`TransientDecoratorIdGen` 的共享 base 所有权；
/// 原版 base 由宿主提供，Rust 侧以资源持有避免空指针解引用）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct TransientDecoratorIdBase(pub u32);

/// B 0x1407a9c10（O 结构）：放置主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_place_decorator(
    _ui: Res<UiState>,
    mut stream: ResMut<UiSignifierStream>,
    _dispatcher: Res<super::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>,
    _storage: ResMut<DecoratorStorage>,
    _transforms: Res<CachedDecoratorTransforms>,
    _stairs: Res<StairsState>,
    _window: Res<WindowSize>,
    mut _inits: EventReader<EvInitPlaceDecoratorInteractionState>,
    mut _init_w: EventWriter<EvInitPlaceDecoratorInteractionState>,
    mut _app_mode: EventWriter<RequestAppModeChange>,
    mut _stairs_audio: EventWriter<PlayStairsAudioCmd>,
    // NotifyHintSystemStairsPlaced（onboarding 域）core 未落——登记待接线。
    mut _border_feedback: EventWriter<FeedbackInputOutOfBorder>,
    mut _border_display: EventWriter<DisplayOutOfBorder>,
    // InstaCreateStairPoint/InstaCreateDeco 为早期编造事件（fresh 0x1407adb20 无此
    // 事件面；core ui_autoclutter 已删除）——不伪造参数，登记待真事件取证。
    mut _local: Local<PlayedConnectSoundLastFrame>,
    mut id_base: ResMut<TransientDecoratorIdBase>,
) {
    // O 结构：init 事件 → 瞬时 id 生成器 + 交互状态初始化；随后命中转换 +
    // 意图计算 + 楼梯候选 + 指示符 + 释放 → apply。精确流程 I（90KB 反编译
    // 的相位/跳表细节登记于 RESTORE.md；本实现承载结构面）。
    for _ev in _inits.read() {
        _init_w.write(EvInitPlaceDecoratorInteractionState);
        // 0x1408339e0 非直写模式：offset 递增 + base 值参与 id。base 由本资源
        // 持有（原宿主所有权面），不再用空指针（旧实现 UAF/P0）。
        let mut id_gen = TransientDecoratorIdGen::new(&mut id_base.0, false);
        let _transient_id = id_gen.generate();
    }
    let _ = &mut stream;
}

/// 原 1407adb20：以屏幕位置命名的真实图标工厂，不携带装饰 ID。
pub fn show_decorator_icon(icon: u8, pos: glam::Vec2, stream: &mut UiSignifierStream) {
    use country_core::systems::ui::signifiers::signifier_factory::SignifierFactory;
    let name = format!("decorator_{pos:?}");
    let factory = SignifierFactory::from_closure(
        &name,
        std::sync::Arc::new(move |meshes, shaders, commands| {
            vec![decorator_cursor_icon(pos, icon, meshes, shaders, commands)]
        }),
    );
    stream.write_factory(&factory);
}

/// 原 1407adcb0/492B：解析原类型图标表，交给现有二维 UI 组件与绘制消费方。
pub fn decorator_cursor_icon(
    pos: glam::Vec2,
    icon: u8,
    meshes: &country_core::asset_libraries::mesh_library::AssetMeshLibrary,
    shaders: &country_core::asset_libraries::shader_library::AssetShaderLibrary,
    commands: &mut Commands,
) -> Entity {
    use country_core::components::{Ui2DFollowMouse, Ui2dExtent, Ui2dImage, UiInactive};
    use country_core::systems::ui::signifiers::signifier_factory::{
        SignifierEntityBuilder, SignifierMesh,
    };
    let entity = SignifierEntityBuilder::new(SignifierMesh::Empty)
        .animation_duration(0.0)
        .build(meshes, shaders, commands);
    commands.entity(entity).insert((
        Ui2dExtent {
            pos,
            size: glam::Vec2::new(34.5, 58.0),
        },
        Ui2dImage::new(decorator_icon_name(icon)),
        UiInactive,
    ));
    commands.entity(entity).insert(Ui2DFollowMouse);
    entity
}

fn decorator_icon_name(icon: u8) -> &'static str {
    match icon.wrapping_sub(0x38) {
        0..=2 => "decorator_cursor_icon/window",
        3 => "decorator_cursor_icon/chimney",
        4 => "decorator_cursor_icon/lantern",
        5 => "decorator_cursor_icon/flag",
        // 原范围外类型也落 slot6；原 slot6/7 的两名字均为 clutter。
        _ => "decorator_cursor_icon/clutter",
    }
}

#[cfg(test)]
mod icon_factory_tests {
    use super::*;
    use bevy_ecs::world::{CommandQueue, World};
    use country_core::{
        asset_libraries::{
            mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary,
            transient_mesh::TransientMesh,
        },
        components::{Ui2DFollowMouse, Ui2dExtent, Ui2dImage, UiInactive},
        systems::ui::signifiers::{
            animation::SignifierAnimation, signifier_create::UiSignifierEntry,
        },
    };

    #[test]
    fn original_type_table_keeps_every_byte_tag_and_duplicate_clutter_names() {
        for ty in 0..=u8::MAX {
            let expected = match ty {
                0x38..=0x3a => "decorator_cursor_icon/window",
                0x3b => "decorator_cursor_icon/chimney",
                0x3c => "decorator_cursor_icon/lantern",
                0x3d => "decorator_cursor_icon/flag",
                _ => "decorator_cursor_icon/clutter",
            };
            assert_eq!(decorator_icon_name(ty), expected);
        }
    }

    #[test]
    fn real_factory_has_position_identity_and_wakes_one_actual_ui_entity() {
        let mut stream = UiSignifierStream::default();
        let pos = glam::Vec2::new(-4.5, 12.0);
        show_decorator_icon(0x3c, pos, &mut stream);
        let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else {
            panic!("factory expected")
        };
        assert_eq!(factory.name(), Some("decorator_Vec2(-4.5, 12.0)"));
        let meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            country_core::render::shader::RasterShaderProgram::new_draw(
                "vertex_color_shader",
                Default::default(),
                &country_core::render::test_device::TestDevice(1),
            ),
            false,
        );
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        let entities = factory.wake(&meshes, &shaders, &mut Commands::new(&mut queue, &world));
        queue.apply(&mut world);
        assert_eq!(entities.len(), 1);
        let id = entities[0];
        assert_eq!(world.get::<Ui2dExtent>(id).unwrap().pos, pos);
        assert_eq!(
            world.get::<Ui2dExtent>(id).unwrap().size,
            glam::Vec2::new(34.5, 58.0)
        );
        assert_eq!(
            world.get::<Ui2dImage>(id).unwrap().name,
            "decorator_cursor_icon/lantern"
        );
        assert_eq!(world.get::<Ui2dImage>(id).unwrap().layer, 0);
        assert!(world.get::<UiInactive>(id).is_some());
        assert!(world.get::<Ui2DFollowMouse>(id).is_some());
        assert!(world.get::<TransientMesh>(id).is_some());
        assert_eq!(world.get::<SignifierAnimation>(id).unwrap().duration, 0.0);
        assert_eq!(world.get::<SignifierAnimation>(id).unwrap().flag_8, 0);
        let handle = *world
            .get::<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>>(id)
            .unwrap();
        assert!(
            meshes
                .get(handle)
                .unwrap()
                .positions_without_indices()
                .is_empty()
        );
        let registry = std::sync::Arc::clone(meshes.transient_meshes());
        world.despawn(id);
        assert_eq!(*registry.lock().unwrap(), vec![handle]);
    }

    #[test]
    fn original_identity_omits_icon_type_and_retains_nonfinite_position() {
        fn name(icon: u8, pos: glam::Vec2) -> String {
            let mut stream = UiSignifierStream::default();
            show_decorator_icon(icon, pos, &mut stream);
            let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else {
                panic!("factory expected")
            };
            factory.name().unwrap().to_owned()
        }
        assert_eq!(name(0x38, glam::Vec2::ZERO), name(0x3d, glam::Vec2::ZERO));
        assert!(name(0x38, glam::Vec2::new(f32::NAN, f32::INFINITY)).contains("NaN"));
    }
}
