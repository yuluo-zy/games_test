//! 游戏启动与主题资源加载。调度分为一次性启动、会话请求、每帧更新。
//! 各资产加载器也可以由宿主独立调用，设备资源必须来自实际渲染上下文。

pub mod clearing;
pub mod init_color_library;
pub mod init_photomode_state;
pub mod init_populate_minimal_startup_ssbo_library;
pub mod init_populate_ssbo_library;
pub mod load_audio;
pub mod load_glade_audio;
pub mod load_glade_grading;
pub mod load_glade_tod_and_weather;
pub mod load_mesh_atlases;
pub mod load_prefabs;
pub mod load_shaders;
pub mod load_textures;
pub mod run_game;
pub mod sheep_assets;
pub mod spawn_brush_previews;
pub mod spawn_human_ref;
pub mod spawn_preplaced_trees;
pub mod startup_blit;
pub mod startup_critters;
pub mod startup_deferred_raster;
pub mod startup_garden_brush;
pub mod startup_grass;
pub mod startup_ivy;
pub mod startup_meadow;
pub mod startup_nani;
pub mod startup_roof;
pub mod startup_session;
pub mod startup_sheep;
pub mod startup_sky_and_sun;
pub mod startup_terrain;
pub mod startup_time_of_day;
pub mod startup_walls;
pub mod startup_water;

/// Original B `country_core::startup::debug::debug` @1408efd40 (5683 B): a Startup-tuple member (add_game_startup_systems
/// 0x140941110 @14094117f -> add_systems<Startup> 0x140b67b50 -> tuple 0x14099b8b0 -> into_configs 0x1417650e0, system vtable
/// 0x142cd6be8) that registers the debug/editor meshes and, crucially for shipped rendering, the `sparkles_shader` and
/// `leaf_particle` raster programs plus the sparkle / leaf-particle / test-cube entities. There is no gate.
///
/// Machine-code order (O): sparkles shader -> `sparkle.json` -> mesh "sparkle" -> spawn(Drawable, KeepOnLevelUnload,
/// DontCastShadow) [command 0x1412d9bf0] -> decorator meshes -> "cube"/"cube_grey" -> "stair_step" -> spawn(KeepOnLevelUnload,
/// Drawable{cube, solid_vertex_color, rot(-.5,.5,-.5,.5) pos(-0,-.5,-0) scale 0}, TestCube) [0x1412dcb80] -> "leaf_particle_mesh"
/// -> `leaf_particle` shader -> spawn(Drawable) [0x1412d7c70] -> `texture_test_shader` (registered only) -> five debug spheres.
pub mod debug {
    use std::path::Path;

    use bevy_ecs::system::Commands;
    use glam::{Quat, Vec3};

    use crate::{
        asset_libraries::{
            Handle,
            mesh_library::{AssetMeshLibrary, MeshAsset},
            meshwatch::MeshWatch,
            shader_library::AssetShaderLibrary,
        },
        components::{KeepOnLevelUnload, TestCube, drawable::DrawableMeshBundle},
        geometry::{
            cube::{CubeBox, cube_mesh},
            sphere::Sphere,
        },
        render::{mesh::Mesh, shader::boundary::ShaderBackend},
        systems::render_loop::draw_passes::draw_lists::DontCastShadow,
        utils::{components::transform::Transform, load_json::try_load_json_as_mesh},
    };

    pub const SPARKLES_SHADER: (&str, &str) = ("/sparkles.raster.hlsl", "sparkles_shader");
    pub const LEAF_PARTICLE_SHADER: (&str, &str) = ("/leaf_particle.raster.hlsl", "leaf_particle");
    pub const TEXTURE_TEST_SHADER: (&str, &str) =
        ("/texture_test.raster.hlsl", "texture_test_shader");
    /// (file, library name, optional `add_color_self` colour) in the original call order, between the sparkle spawn and
    /// the cube meshes. Colours are the constants stored before each `Mesh::add_color_self` call.
    pub const DECORATOR_MESHES: [(&str, &str, Option<Vec3>); 11] = [
        (
            "assets/meshes/decorator_delete_scribble.json",
            "decorator_delete_scribble",
            Some(Vec3::new(1.0, 1.0, 1.0)),
        ),
        ("assets/meshes/decorator_test.json", "decorator_test", None),
        (
            "assets/meshes/decorator_test_turret.json",
            "decorator_test_turret",
            None,
        ),
        ("assets/meshes/scissors.json", "ui_scissors", None),
        ("assets/meshes/wooden_gate/ladder.json", "ladder", None),
        (
            "assets/meshes/decorator_test_door.json",
            "decorator_test_door",
            None,
        ),
        (
            "assets/meshes/decorator_test_cupola.json",
            "decorator_test_cupola",
            None,
        ),
        (
            "assets/meshes/decorator_test_dormer.json",
            "decorator_test_dormer",
            None,
        ),
        (
            "assets/meshes/decorator_outline.json",
            "decorator_outline",
            None,
        ),
        (
            "assets/meshes/decorator_outline.json",
            "decorator_outline_red",
            Some(Vec3::new(0.8, 0.0, 0.0)),
        ),
        (
            "assets/meshes/decorator_trigger_volume.json",
            "decorator_trigger_volume",
            None,
        ),
    ];
    /// Unit cube (O: `[-0.5, 0.5]` on all axes, 0x142a9f3d0 / 0x142a9f3e0).
    pub const UNIT_CUBE: CubeBox = CubeBox {
        x: [-0.5, 0.5],
        y: [-0.5, 0.5],
        z: [-0.5, 0.5],
    };
    /// `cube_grey` colour (O: 0x142aa68a0 = `(0.07, 0.06)`, third word `0x3d75c28f` = 0.06).
    pub const CUBE_GREY: Vec3 = Vec3::new(0.07, 0.06, 0.06);
    /// Debug spheres: `Sphere { subdivisions: 0, radius: 1.0 }` coloured per name (O: `edx = 0`, `xmm2 = 1.0`).
    pub const DEBUG_SPHERES: [(&str, Vec3); 5] = [
        ("debug_sphere_white", Vec3::new(1.0, 1.0, 1.0)),
        ("debug_sphere_red", Vec3::new(1.0, 0.0, 0.0)),
        ("debug_sphere_blue", Vec3::new(0.0, 0.0, 1.0)),
        ("debug_sphere_green", Vec3::new(0.0, 1.0, 0.0)),
        ("debug_sphere_grey", Vec3::new(0.1, 0.1, 0.1)),
    ];
    /// Transform stored by the TestCube spawn (O: 0x1408f092c..0x1408f0953).
    pub fn test_cube_transform() -> Transform {
        Transform {
            rotation: Quat::from_xyzw(-0.5, 0.5, -0.5, 0.5),
            translation: Vec3::new(-0.0, -0.5, -0.0),
            scale: Vec3::ZERO,
        }
    }

    /// Common original `load_mesh_into_library` @140b1aca0: watch first, add second.
    fn add_mesh(
        meshes: &AssetMeshLibrary,
        watch: &MeshWatch,
        mesh: Mesh,
        name: &str,
    ) -> Handle<Mesh> {
        watch.watch(&mesh);
        meshes.add(MeshAsset::new(mesh, Some(name.to_owned())))
    }

    fn load_json(root: &Path, path: &str) -> anyhow::Result<Mesh> {
        let full = root.join(path);
        let full = full
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("debug mesh path is not UTF-8: {}", full.display()))?;
        try_load_json_as_mesh(full)
    }

    pub fn debug(
        root: &Path,
        meshes: &AssetMeshLibrary,
        watch: &MeshWatch,
        shaders: &AssetShaderLibrary,
        backend: &dyn ShaderBackend,
        commands: &mut Commands,
    ) -> anyhow::Result<()> {
        let load_shader = |(path, name): (&str, &'static str)| {
            super::load_shader_into_library(
                name,
                crate::rhapsody_types::RasterPipelineDesc {
                    path: path.to_owned(),
                    entry: String::new(),
                    defines: Vec::new(),
                },
                backend,
                shaders,
            )
        };
        let sparkles = load_shader(SPARKLES_SHADER)?;
        let mesh = add_mesh(
            meshes,
            watch,
            load_json(root, "assets/meshes/sparkle.json")?,
            "sparkle",
        );
        commands.spawn((
            DrawableMeshBundle {
                mesh,
                shader: sparkles,
                transform: Transform::identity(),
            },
            KeepOnLevelUnload,
            DontCastShadow,
        ));
        for (path, name, color) in DECORATOR_MESHES {
            let mut mesh = load_json(root, path)?;
            if let Some(color) = color {
                mesh = mesh.add_color_self(color);
            }
            add_mesh(meshes, watch, mesh, name);
        }
        let cube = add_mesh(meshes, watch, cube_mesh(&UNIT_CUBE), "cube");
        add_mesh(
            meshes,
            watch,
            cube_mesh(&UNIT_CUBE).add_color_self(CUBE_GREY),
            "cube_grey",
        );
        add_mesh(
            meshes,
            watch,
            load_json(root, "assets/meshes/stair_step.json")?,
            "stair_step",
        );
        let solid = shaders
            .get_raster_handle_by_name("solid_vertex_color")
            .ok_or_else(|| anyhow::anyhow!("solid_vertex_color shader is not registered"))?;
        commands.spawn((
            KeepOnLevelUnload,
            DrawableMeshBundle {
                mesh: cube,
                shader: solid,
                transform: test_cube_transform(),
            },
            TestCube,
        ));
        let particle_mesh = add_mesh(
            meshes,
            watch,
            load_json(root, "assets/meshes/particle_plane.json")?,
            "leaf_particle_mesh",
        );
        let leaf = load_shader(LEAF_PARTICLE_SHADER)?;
        commands.spawn(DrawableMeshBundle {
            mesh: particle_mesh,
            shader: leaf,
            transform: Transform::identity(),
        });
        load_shader(TEXTURE_TEST_SHADER)?;
        for (name, color) in DEBUG_SPHERES {
            let sphere = Mesh::try_from(Sphere::new(0, 1.0))
                .map_err(|e| anyhow::anyhow!("debug sphere {name}: {e}"))?;
            add_mesh(meshes, watch, sphere.add_color_self(color), name);
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cube_is_the_original_degenerate_marker() {
            let t = test_cube_transform();
            assert_eq!(t.scale, Vec3::ZERO);
            assert_eq!(t.translation.y, -0.5);
            assert_eq!(t.rotation, Quat::from_xyzw(-0.5, 0.5, -0.5, 0.5));
        }

        #[test]
        fn debug_asset_name_lengths_match_the_machine_code() {
            // R8D at each load_mesh_into_library call site.
            let lens: Vec<usize> = DECORATOR_MESHES.iter().map(|m| m.1.len()).collect();
            assert_eq!(
                lens,
                [0x19, 0xe, 0x15, 0xb, 6, 0x13, 0x15, 0x15, 0x11, 0x15, 0x18]
            );
            let sphere: Vec<usize> = DEBUG_SPHERES.iter().map(|m| m.0.len()).collect();
            assert_eq!(sphere, [0x12, 0x10, 0x11, 0x12, 0x11]);
            assert_eq!(SPARKLES_SHADER.0.len(), 0x15);
            assert_eq!(LEAF_PARTICLE_SHADER.0.len(), 0x1a);
            assert_eq!(TEXTURE_TEST_SHADER.0.len(), 0x19);
        }

        #[test]
        fn debug_spheres_build_from_the_original_parameters() {
            let mesh = Mesh::try_from(Sphere::new(0, 1.0)).unwrap();
            assert!(!mesh.indices.is_empty());
        }
    }
}

#[cfg(test)]
mod tests;

use crate::render::shader::compile_shader_variants;
use crate::{
    asset_libraries::{Handle, shader_library::AssetShaderLibrary},
    render::shader::{RasterShaderProgram, boundary::ShaderBackend},
    rhapsody_types::RasterPipelineDesc,
};

/// 编译两个光栅变体后注册；编译失败返回错误，避免 Result 接口内部隐藏 panic。
pub fn load_shader_into_library(
    name: &'static str,
    desc: RasterPipelineDesc,
    device: &dyn ShaderBackend,
    library: &AssetShaderLibrary,
) -> anyhow::Result<Handle<RasterShaderProgram>> {
    let variants = compile_shader_variants(&desc, device)?;
    Ok(library.add_raster(
        RasterShaderProgram {
            name,
            desc,
            variants,
        },
        false,
    ))
}

/// 启动资产宿主：与原作 `load_mesh_into_library` / `load_shader_into_library` /
/// `Commands::spawn` 三件套一一对应的最小接口。
///
/// 设备与命令所有者属于宿主 App；这些模块只复现原作的资源表、调用顺序和实体
/// 组合，不合成 shader、不伪造句柄。测试使用记录器实现同一接口。
pub trait StartupAssetHost {
    /// 注册后的网格句柄类型。
    type Mesh: Copy;
    /// 注册后的着色器句柄类型。
    type Shader: Copy;

    /// 注册一个网格。`name` 为 `None` 时对应原作的匿名（仅路径）加载。
    fn load_mesh(&mut self, path: &'static str, name: Option<&'static str>) -> Self::Mesh;

    /// 编译并注册一个光栅着色器。
    fn load_shader(&mut self, path: &'static str, name: &'static str) -> Self::Shader;

    /// 按名查已导入的网格（mesh atlas/prefab 通道）。名式加载不得伪造文件路径；
    /// 查不到时返回 `None`，调用方保留 U。
    fn lookup_mesh(&mut self, _name: &str) -> Option<Self::Mesh> {
        None
    }

    /// 按名查已注册的着色器；查不到时返回 `None`。
    fn lookup_shader(&mut self, _name: &str) -> Option<Self::Shader> {
        None
    }

    /// 生成一个绘制实体。
    fn spawn_drawable(&mut self, mesh: Self::Mesh, shader: Self::Shader);
}

/// 装配 CPU 启动、会话和更新阶段；设备资产由实际宿主注册。
pub fn add_game_startup_systems<B, S>(
    world: &mut bevy_ecs::world::World,
    startup_schedule: &mut bevy_ecs::schedule::Schedule,
    session_schedule: &mut bevy_ecs::schedule::Schedule,
    update_schedule: &mut bevy_ecs::schedule::Schedule,
) where
    B: crate::systems::audio::music::music_player::MusicBackend + bevy_ecs::resource::Resource,
    S: crate::resources::audio_resources::Studio + Send + Sync + 'static,
{
    run_game::register_core_startup::<B, S>(
        world,
        startup_schedule,
        session_schedule,
        update_schedule,
    );
    // Ivy and garden brush require the device host's root, shader backend and MeshWatch.
}

/// Complete theme-specific clearing producers.
pub mod glade_themes;
