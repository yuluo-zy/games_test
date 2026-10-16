//! duck_splash_particle（B 主基线；0x142322650，源码 22–）。
//!
//! 证据（skeleton PDB 依赖注解）：Commands::spawn_empty / EntityCommands::despawn /
//! option::unwrap_failed、AssetMeshLibrary::get_handle_by_name / AssetShaderLibrary::
//! get_raster_handle_by_name（"duck_shader" 同款资产取句柄）。
//! 语义：鸭子入水（水面穿越）→ **生成**水花粒子实体（spawn_empty + 资产句柄）；
//! 粒子生命周期结束 → despawn。GPU 粒子更新归宿主 render 域（GPU 后置）。

use bevy_ecs::prelude::*;
use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::asset_libraries::shader_library::AssetShaderLibrary;
use country_core::resources::time::GameTime;

/// 资产库包装（core AssetMeshLibrary 非 Resource；装配层构造）。
#[derive(Resource, Default)]
pub struct MeshLibraryHandle(pub Option<AssetMeshLibrary>);

/// 资产库包装（core AssetShaderLibrary 非 Resource；装配层构造）。
#[derive(Resource, Default)]
pub struct ShaderLibraryHandle(pub Option<AssetShaderLibrary>);
use country_core::resources::world_raster::WorldRaster;
use glam::Vec3;

/// 水花粒子（组件；生命周期 + 位置）。
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SplashParticle {
    pub pos: Vec3,
    pub age: f32,
    /// 总生命周期（O 语义：入水瞬间 0.5s 级）。
    pub lifetime: f32,
}

/// 鸭子入水水花请求（鸭落水检测系统写；本系统消费）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct DuckSplash {
    pub entity: Entity,
    pub pos: Vec3,
}

/// duck_splash_particle（0x142322650）：处理新水花请求（spawn 粒子）+ 推进/清理粒子。
/// 证据：`AssetMeshLibrary::get_handle_by_name` / `AssetShaderLibrary::
/// get_raster_handle_by_name`（"duck_shader"）取句柄 → spawn_empty 建粒子实体。
pub fn duck_splash_particle(
    time: Res<GameTime>,
    _water: Res<WorldRaster<512>>,
    mesh_library: Option<Res<MeshLibraryHandle>>,
    shader_library: Option<Res<ShaderLibraryHandle>>,
    mut commands: Commands,
    mut events: EventReader<DuckSplash>,
    mut query: Query<(Entity, &mut SplashParticle)>,
) {
    let dt = time.delta;
    // 推进现有粒子；到期 despawn。
    let mut done = Vec::new();
    for (entity, mut particle) in &mut query {
        particle.age += dt;
        if particle.age >= particle.lifetime {
            done.push(entity);
        }
    }
    for entity in done {
        commands.entity(entity).despawn();
    }
    // 新请求：取资产句柄（O："duck_shader"）+ spawn 水花粒子实体。
    for event in events.read() {
        let mesh = mesh_library
            .as_ref()
            .and_then(|lib| lib.0.as_ref())
            .map(|lib| lib.get_handle_by_name("duck_shader"));
        let shader = shader_library
            .as_ref()
            .and_then(|lib| lib.0.as_ref())
            .and_then(|lib| lib.get_raster_handle_by_name("duck_shader"));
        let _ = (mesh, shader);
        commands.spawn(SplashParticle {
            pos: event.pos,
            age: 0.0,
            lifetime: 0.5,
        });
    }
}
