//! 天空/太阳资源载入（B `startup_sky_and_sun` @0x140864430）。
//!
//! 证据（expansion-b2 decomp + 字串池实读）：
//! * 路径 `assets/meshes/skydome.json`（0x142a963b0，len 0x1a）；注册名 `skydome`。
//! * shader 路径 `/skydome.raster.hlsl`（0x142a9641f，len 0x14）；名
//!   `skydome_shader`（0x142a96433，len 0xe）。
//! * 调用顺序：文件式 mesh → shader → `Commands::spawn` 一个实体。
//!
//! 实体剩余字段（原作者 bundle 的 +0x00 fn 指针、+0x18 常量、+0x20/-0x24 字、
//! +0x58 qword）没有类型级证据；本实现只落已证实的
//! `DrawableMeshBundle { mesh, shader, Transform }`，其余显式 U。
use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use super::{StartupAssetHost, register_mesh_file};
use bevy_ecs::prelude::Res;

pub const SKYDOME_MESH: (&str, &str) = ("assets/meshes/skydome.json", "skydome");
pub const SKYDOME_SHADER: (&str, &str) = ("/skydome.raster.hlsl", "skydome_shader");

/// 原作三件套顺序：mesh、shader、spawn。
pub fn startup_sky_and_sun<H: StartupAssetHost>(host: &mut H) {
    let mesh = host.load_mesh(SKYDOME_MESH.0, Some(SKYDOME_MESH.1));
    let shader = host.load_shader(SKYDOME_SHADER.0, SKYDOME_SHADER.1);
    host.spawn_drawable(mesh, shader);
}

/// ECS 网格产者：只写入真实 `AssetMeshLibrary`。shader 与实体需要宿主设备/Commands，
/// 由 [`startup_sky_and_sun`] 在真实主机装配。
pub fn startup_sky_and_sun_meshes(meshes: Res<AssetMeshLibrary>) {
    register_mesh_file(&meshes, SKYDOME_MESH.0, Some(SKYDOME_MESH.1));
    // U: skydome bundle 的非 DrawableMeshBundle 字段（sky 状态/参数）未恢复。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        calls: Vec<String>,
        mesh: Option<(&'static str, Option<&'static str>)>,
        shader: Option<(&'static str, &'static str)>,
        spawned: usize,
    }

    impl StartupAssetHost for Rec {
        type Mesh = u64;
        type Shader = u64;
        fn load_mesh(&mut self, path: &'static str, name: Option<&'static str>) -> u64 {
            self.calls.push("mesh".into());
            self.mesh = Some((path, name));
            1
        }
        fn load_shader(&mut self, path: &'static str, name: &'static str) -> u64 {
            self.calls.push("shader".into());
            self.shader = Some((path, name));
            2
        }
        fn spawn_drawable(&mut self, mesh: u64, shader: u64) {
            assert_eq!((mesh, shader), (1, 2));
            self.calls.push("spawn".into());
            self.spawned += 1;
        }
    }

    #[test]
    fn skydome_uses_the_exact_mesh_and_shader_strings_in_order() {
        let mut host = Rec::default();
        startup_sky_and_sun(&mut host);
        assert_eq!(host.calls, ["mesh", "shader", "spawn"]);
        assert_eq!(
            host.mesh,
            Some((SKYDOME_MESH.0, Some(SKYDOME_MESH.1)))
        );
        assert_eq!(host.shader, Some(SKYDOME_SHADER));
        assert_eq!(host.spawned, 1);
    }
}
