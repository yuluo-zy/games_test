//! B `decorator_visual/lantern.rs` — 灯笼装饰视觉与灯光维护（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x14078d320（generate_lanterns，91KB）/
//! 0x140790400（maintain_lantern_lights，24KB）/ 0x141257fa0
//! （compare_light_lists closure_env<3>，7.3KB）/ 0x1412583d0
//! （closure_env<5>，2.9KB）+ 骨架 callees（O）：DecoratorStorage::iter、
//! DecoratorLibKey::new ×6、DecoratorRank（0x140833250）×6、
//! DecoratorExtraInfo::as_lantern ×3、set_colliders ×3、GladeType::is_olden、
//! rhapsody::GraphicsDevice::update_buffer_bytes（0x140beefa0）、
//! WindowVisualEntityBuilder::{lantern_light,new}。
//!
//! 业务（O 结构）：generate_lanterns 逐存储逐 modified 灯笼 → 六键
//! （collision/interaction/outline × rank 变化）→ as_lantern 载荷 → 蓝图
//! （set_colliders）+ 视觉实体（lantern_light 标记）→ 灯光记录。
//! maintain_lantern_lights 比较新旧灯光列表（compare_light_lists 0x1cB 步长
//! 记录）→ GPU 灯光缓冲更新（update_buffer_bytes）。
//! U 登记：① 灯载荷（LanternExtra u32）语义 ② 灯光记录 28B 布局
//! ③ 参数排序。

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator_storage::DecoratorStorage;
use country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary;
use glam::{Quat, Vec3};

use crate::blueprint::{DecoratorBlueprint, DecoratorBlueprints};
use crate::decorator_visual_state::DecoratorTransientVisualState;
use crate::utils::visual_entity_builder::WindowVisualEntityBuilder;

/// 0x14078d320（O 结构）：生成灯笼装饰视觉。
pub fn generate_lanterns(
    decorator_storage: Res<DecoratorStorage>,
    mesh_lib: Res<DecoratorMeshLibrary>,
    mut blueprints: ResMut<DecoratorBlueprints>,
    mut transient: ResMut<DecoratorTransientVisualState>,
    _glade: Res<country_core::resources::session::ActiveSession>,
) {
    for (_wall, decorators) in decorator_storage.iter() {
        for (_id, info) in decorators.iter_modified() {
            // O 判别：DecoratorExtraInfo::as_lantern（非灯笼 panic）。
            let lantern = info.extra().as_lantern();
            let _payload = lantern.0; // U：灯载荷语义。
            // O：六键（DecoratorLibKey::new ×6 + rank ×6 = 碰撞/交互/描边 ×
            // 2 侧；I 键面）。
            let key = country_core::resources::walls::decorator_subtype::DecoratorLibKey::new(
                info.decorator_type().0,
                info.rank(),
            );
            let mut blueprint = DecoratorBlueprint::new();
            if let Some(collision) = mesh_lib.collision_mesh(key) {
                blueprint.set_colliders(vec![collision]);
            }
            blueprints.insert(info.decorator_id, blueprint);
            // O：视觉实体（lantern_light 标记）。
            let builder = WindowVisualEntityBuilder::new(
                Vec3::ZERO, // U：位置 = 缓存变换消费面待取证。
                Quat::IDENTITY,
                info.decorator_id,
            );
            let _ = &mut transient;
            let _ = &builder;
        }
    }
}

/// 0x140790400（O 结构，8 参面）：维护灯笼灯光（新旧列表比较 +
/// GPU 灯光缓冲更新）。
pub fn maintain_lantern_lights(
    _storage: Res<DecoratorStorage>,
    _device: Res<rhapsody_contract::GraphicsDeviceHandle>,
    mut _transient: ResMut<DecoratorTransientVisualState>,
) {
    // U：compare_light_lists（0x1cB 步长记录比较）+ update_buffer_bytes
    // （0x140beefa0）主体待取证。
}

/// 0x141257fa0（O，5 参面）：灯光列表比较闭包（新旧逐 0x1cB 记录）。
/// U：28B 灯光记录布局 + 比较准则。
pub fn compare_light_lists(
    _old: &[u8],
    _new: &[u8],
) -> bool {
    false // U：比较准则待取证。
}

/// rhapsody 设备句柄契约（U：更新缓冲调用面待取证）。
pub mod rhapsody_contract {
    use bevy_ecs::resource::Resource;

    /// GraphicsDevice 句柄（D1 占位契约：maintain_lantern_lights 消费
    /// update_buffer_bytes 0x140beefa0；真实设备资源归 rhapsody 集成）。
    #[derive(Resource, Clone, Debug, Default)]
    pub struct GraphicsDeviceHandle {
        /// U。
        pub _gap: u64,
    }
}
