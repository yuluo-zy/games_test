//! duck_splash_particle（B 主基线；0x142322650，fresh 反编译 + 符号实证 2026-10-17）。
//!
//! 系统签名（`SpawnSplashParticle` 注册符号 0x140f94300 与 0x142322650 反编译参数面）：
//! `fn(Res<GameTime>, Query<&mut SplashParticle>, Query<&Transform>, EventReader<
//! SpawnSplashParticle>, Res<AssetMeshLibrary>, Res<AssetShaderLibrary>, Commands)`。
//!
//! 业务（fresh 反编译 142322650 逐语句）：
//! 1. **粒子推进**：每个 SplashParticle：`age += dt / 2.5`（0x40200000）。
//!    `age <= 1.0` → `age = clamp(age, 0, 1)`（MAXSS/MINSS 展开，NaN 保留）；
//!    `age > 1.0`（含 unordered：NaN 落到 else）→ `EntityCommands::despawn`。
//! 2. **新水花**：`EventReader<SpawnSplashParticle>`（Bevy EventInstance 16B = event_id
//!    + `Entity` 载荷；move_ducks 在状态 0→1 且 t>0.25 时写入）。逐事件：
//!    - 用 `Query<&Transform>` 取事件实体变换（查不到跳过）；
//!    - `AssetMeshLibrary::get_handle_by_name("splash_particle")` +
//!      `AssetShaderLibrary::get_raster_handle_by_name("splash_particle")`（后者
//!      `Option`，B 侧 `unwrap`）；
//!    - `spawn<(DrawableMeshBundle, SplashParticle, DontCastShadow)>`：
//!      bundle.transform = 事件实体变换；SplashParticle 初值 0。
//!
//! 组件布局：B `components::SplashParticle` 的消费者只读 `f32 @ +8`（render_loop
//! `SplashParticleView`），故本类型以两个 u32 前缀 + age 保持偏移。
//! GPU 粒子更新归宿主 render 域（GPU 后置）。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::asset_libraries::shader_library::AssetShaderLibrary;
use country_core::components::drawable::DrawableMeshBundle;
use country_core::resources::time::GameTime;
use country_core::systems::render_loop::draw_passes::draw_lists::DontCastShadow;

use crate::components::Transform;

/// 资产库包装（core AssetMeshLibrary 非 Resource；装配层构造）。
#[derive(Resource, Default)]
pub struct MeshLibraryHandle(pub Option<AssetMeshLibrary>);

/// 资产库包装（core AssetShaderLibrary 非 Resource；装配层构造）。
#[derive(Resource, Default)]
pub struct ShaderLibraryHandle(pub Option<AssetShaderLibrary>);

/// 水花粒子（组件；B 只读 f32@+8 = age）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SplashParticle {
    /// +0：B insert 载荷首 dword 置 0（语义 U；保留布局）。
    pub pad0: u32,
    /// +4：填充（保持 age 在 +8）。
    pub pad1: u32,
    /// +8：age（`+= dt/2.5`；>1.0 despawn）。
    pub age: f32,
}

/// `SpawnSplashParticle`（B 事件；move_ducks EventWriter 载荷 = 鸭实体）。
/// Bevy 事件缓冲的 `EventInstance<E>` 为 16B（event_id u64 + E）；E = Entity（8B）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpawnSplashParticle(pub Entity);

/// duck_splash_particle（0x142322650）。
pub fn duck_splash_particle(
    time: Res<GameTime>,
    mesh_library: Option<Res<MeshLibraryHandle>>,
    shader_library: Option<Res<ShaderLibraryHandle>>,
    mut commands: Commands,
    mut events: EventReader<SpawnSplashParticle>,
    mut query: Query<(Entity, &mut SplashParticle)>,
    transforms: Query<&Transform>,
) {
    let dt = time.delta;

    // 1) 推进/清理（O：age += dt/2.5；<=1.0 clamp 否则 despawn）。
    let mut done: Vec<Entity> = Vec::new();
    for (entity, mut particle) in &mut query {
        particle.age += dt / 2.5;
        if particle.age <= 1.0 {
            // MAXSS 0 / MINSS 1 展开：NaN 输入保持 NaN。
            let lo = if 0.0 <= particle.age { particle.age } else { 0.0 };
            particle.age = if lo <= 1.0 { lo } else { 1.0 };
        } else {
            done.push(entity);
        }
    }
    for entity in done {
        commands.entity(entity).despawn();
    }

    // 2) 新水花事件（O：事件实体 → Transform → splash 资产 → spawn bundle）。
    for SpawnSplashParticle(target) in events.read().copied() {
        let Ok(transform) = transforms.get(target) else {
            continue;
        };
        let Some(mesh_lib) = mesh_library.as_ref().and_then(|l| l.0.as_ref()) else {
            continue;
        };
        let Some(shader_lib) = shader_library.as_ref().and_then(|l| l.0.as_ref()) else {
            continue;
        };
        let mesh = mesh_lib.get_handle_by_name("splash_particle");
        // O：`if (extraout_RAX_00 & 1) == 0 { unwrap_failed() }` —— 缺失即 panic。
        let shader = shader_lib
            .get_raster_handle_by_name("splash_particle")
            .expect("splash_particle raster shader");
        commands.spawn((
            DrawableMeshBundle {
                mesh,
                shader,
                transform: *transform,
            },
            SplashParticle::default(),
            DontCastShadow,
        ));
    }
}
