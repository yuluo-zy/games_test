//! startup_quad.rs — `stars` / `clouds` / `billboard_forest` 三个 startup 系统共用的“单面片网格 + shader + spawn”骨架。
//!
//! 三个 `startup_*`（0x140903ca0 / 0x14085b8b0 / 0x140ae8080，各 662 B）逐条相同，只差常量：
//!
//! 1. 索引：`Vec<u32>::from_iter(FlatMap<TupleWindows<RangeInclusive<u32>, (u32,u32)>, [u32;6], closure$0>)`
//!    （单态化体 0x1412fa8e0，ICF 后三个 closure 共用一份）。范围初值 `0..=1`（O：`[rbp+0x2c] = 0x100000000` 即
//!    start = 0、end = 1，`exhausted = 0`），所以只有一个窗口 `(0, 1)`；`closure$0((a, b))` =
//!    `[2a, 2a+1, 2b, 2b, 2a+1, 2b+1]`（O：栈上依次写 `edx+edx, 2*rdx+1, eax+eax, eax+eax, 2*rdx+1, 2*rax+1`）。
//!    结果 `[0, 1, 2, 2, 1, 3]`（两个三角形拼成的四边形）。
//! 2. 网格：0x78 B 的 `render::mesh::Mesh`（O：栈上 `[rbp-0x40, rbp+0x38)`，`load_mesh_into_library` 0x140b1aca0 把它按
//!    `0x78 + name(0x18)` 整体搬进 `MeshAsset`）：`+0x00` 索引 Vec；`+0x18` `source_file = None`（`i64::MIN` niche，O：
//!    `movabs rcx, 0x8000000000000000`）；`+0x38..+0x58` 为一个 32 B 常量；`+0x58..+0x68` 为 `RandomState`
//!    （`std::hash::random` 的 KEYS 线程局部取出的 `(k0, k1+1)`）；`+0x68` 为标签 `&str`。→ `load_mesh_into_library(mesh, name)`。
//!
//!    **32 B 常量不是“半径 + kind”**：它是空 `hashbrown::RawTable` 的四个字 `{ctrl, bucket_mask = 0, growth_left = 0,
//!    items = 0}`。`ctrl` 指向 `.rdata` 里 16 个 `0xff` 的静态空组（O：`0x142a94930` 起 16 B 全 `ff`；常量字节
//!    `30 49 a9 42 01 00 00 00 …`，即 64 位指针 `0x1_42a94930`）。旧版把指针低 32 位当 `f32` 半径（`84.64`）、高 32 位当
//!    `u32 = 1`，是误读。所以 `Mesh` 没有、也不需要对应的字段：它就是“空属性表”，`Mesh::new` 的 `attributes`。
//! 3. shader：`load_shader_into_library<&str>(path, name)`（路径形如 "/star.raster.hlsl"，名形如 "star_shader"）。
//! 4. spawn：`Commands::spawn_empty()` + 一次 `insert(bundle)`：`Transform::IDENTITY`（旋转 `(0,0,0,1)`、平移 0、
//!    缩放 1）、网格句柄、shader 句柄、`DrawableMeshBundle` 里的一个 `bool = false`（`+0x58` / billboard 里 `+0x48`）；
//!    stars / clouds 另有一个 `u32 = 0x10`（`DesignatedRenderPass`，I）。`DontCastShadow` / `KeepOnLevelUnload` 是零大小标记，
//!    机器码里没有数据。
//!
//! 外部依赖（`load_*_into_library`、`Commands`、`SubApp::add_systems`）都经 [`StartupHost`] / [`AddStartupSystem`] 边界表达；
//! 生产实现见 [`WorldStartupHost`]（真实 `AssetMeshLibrary` / `MeshWatch` / `AssetShaderLibrary` / `Commands`）。

use bevy_ecs::system::Commands;
use glam::{Quat, Vec3};
use itertools::Itertools;

use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
    },
    components::{KeepOnLevelUnload, drawable::DrawableMeshBundle},
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::ShaderBackend},
    },
    startup::load_shaders::load_shader_into_library,
    systems::render_loop::draw_passes::draw_lists::{DesignatedRenderPass, DontCastShadow},
    utils::components::transform::Transform,
};

/// `closure$0`（O：stars / clouds / billboard_forest 三份同体）。
pub fn quad_window_indices((a, b): (u32, u32)) -> [u32; 6] {
    [2 * a, 2 * a + 1, 2 * b, 2 * b, 2 * a + 1, 2 * b + 1]
}

/// 三个 startup 共用的索引（范围 `0..=1`）。
pub fn quad_indices() -> Vec<u32> {
    (0u32..=1)
        .tuple_windows::<(u32, u32)>()
        .flat_map(quad_window_indices)
        .collect()
}

/// 传给 `load_mesh_into_library` 的网格（只含被机器码写入的字段）。
#[derive(Clone, Debug, PartialEq)]
pub struct StartupMesh {
    /// 网格自带的名字字段（`[rbp+0x28]` / `[rbp+0x30]`）。
    pub name: &'static str,
    pub indices: Vec<u32>,
    /// 其余顶点属性：`None`（`i64::MIN` niche）。
    pub has_vertex_attributes: bool,
}

impl StartupMesh {
    /// 机器码里栈上组装出的 `Mesh`：索引 + 空属性表 + `source_file = None` + 标签（`Mesh::new(label)` 同形）。
    pub fn into_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(self.name);
        mesh.set_indices(self.indices);
        mesh
    }
}

/// `Commands::spawn_empty().insert(bundle)` 的载荷。
#[derive(Clone, Debug, PartialEq)]
pub struct StartupBundle<M, S> {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: Vec3,
    pub mesh: M,
    pub shader: S,
    /// 紧随句柄的 `u32 = 0x10`（stars / clouds）；billboard_forest 没有。
    pub designated_pass: Option<u32>,
    /// 捆绑里的 `bool`，三处都是 `false`（语义 U）。注：命令闭包的 `+0x58` 字节属于外层
    /// `Commands` 命令状态而不是组件（`DrawableMeshBundle` 的 bundle 载荷里没有 bool）。
    pub flag: bool,
    /// 元组里是否带零大小标记 `KeepOnLevelUnload`（O：命令函数 `Bundle::component_ids` 里第 4 个组件
    /// 注册 0x1416e2ac0 = 该标记的 TypeId `0x3ab90f28eb4f1da3_66031cdac51d559b`）。
    /// stars / clouds / billboard_forest 带；snow_particles / loading_screen 不带。
    pub keep_on_level_unload: bool,
}

/// 外部调用边界。
pub trait StartupHost {
    type Mesh;
    type Shader;
    /// `country_core::startup::load_mesh_into_library`（0x140b1aca0）：`library_name` 是传给库的名字。
    fn load_mesh_into_library(&mut self, library_name: &str, mesh: StartupMesh) -> Self::Mesh;
    /// `country_core::startup::load_shader_into_library<&str>`（0x140b3fe40）。
    fn load_shader_into_library(&mut self, path: &str, name: &'static str) -> Self::Shader;
    /// `Commands::spawn_empty` + `insert`。
    fn spawn(&mut self, bundle: StartupBundle<Self::Mesh, Self::Shader>);
}

/// 生产宿主：真实资源库 + `Commands`。
///
/// * 网格：`load_mesh_into_library` 0x140b1aca0 先 `MeshWatch::watch` 再 `AssetMeshLibrary::add`
///   （`MeshAsset { asset: Mesh, name: Some(library_name) }`）；这里的网格 `source_file = None`，watch 为合法空操作。
/// * 着色器：`load_shader_into_library<&str>` 0x140b3fe40，经宿主的 `ShaderBackend` 适配编译。
/// * 实体：原版 `Commands::spawn_empty().insert(..)` 的元组身份由命令函数 0x1412d8820 / 0x1412d8620 的
///   `Bundle::component_ids` 取证：stars / clouds = `(DrawableMeshBundle, DontCastShadow, KeepOnLevelUnload,
///   DesignatedRenderPass)`（6 个组件），billboard_forest = `(DrawableMeshBundle, DontCastShadow, KeepOnLevelUnload)`。
pub struct WorldStartupHost<'a, 'w, 's> {
    pub meshes: &'a AssetMeshLibrary,
    pub watch: &'a MeshWatch,
    pub shaders: &'a AssetShaderLibrary,
    pub backend: &'a dyn ShaderBackend,
    pub commands: &'a mut Commands<'w, 's>,
}

impl StartupHost for WorldStartupHost<'_, '_, '_> {
    type Mesh = Handle<Mesh>;
    type Shader = Handle<RasterShaderProgram>;

    fn load_mesh_into_library(&mut self, library_name: &str, mesh: StartupMesh) -> Self::Mesh {
        let mesh = mesh.into_mesh();
        self.watch.watch(&mesh);
        self.meshes
            .add(MeshAsset::new(mesh, Some(library_name.to_owned())))
    }

    fn load_shader_into_library(&mut self, path: &str, name: &'static str) -> Self::Shader {
        load_shader_into_library(self.shaders, self.backend, path, name)
    }

    fn spawn(&mut self, bundle: StartupBundle<Self::Mesh, Self::Shader>) {
        let drawable = DrawableMeshBundle {
            mesh: bundle.mesh,
            shader: bundle.shader,
            transform: Transform {
                rotation: bundle.rotation,
                translation: bundle.translation,
                scale: bundle.scale,
            },
        };
        match (bundle.designated_pass, bundle.keep_on_level_unload) {
            (Some(pass), true) => self.commands.spawn((
                drawable,
                DontCastShadow,
                KeepOnLevelUnload,
                DesignatedRenderPass(pass),
            )),
            (Some(pass), false) => self
                .commands
                .spawn((drawable, DontCastShadow, DesignatedRenderPass(pass))),
            (None, true) => self
                .commands
                .spawn((drawable, DontCastShadow, KeepOnLevelUnload)),
            (None, false) => self.commands.spawn((drawable, DontCastShadow)),
        };
    }
}

/// `SubApp::add_systems<Startup, S>`（三个 `Plugin::build` 各 8 B：一次尾调用）。
pub trait AddStartupSystem {
    fn add_startup_system(&mut self, name: &'static str);
}

/// 一个面片 startup 的常量。
#[derive(Clone, Copy, Debug)]
pub struct QuadSpec {
    pub mesh_name: &'static str,
    pub library_name: &'static str,
    pub shader_path: &'static str,
    pub shader_name: &'static str,
    pub designated_pass: Option<u32>,
    pub keep_on_level_unload: bool,
}

/// `startup_*` 的共同体（stars / clouds / billboard_forest / snow_particles / loading_screen 同一骨架）。
pub fn run_startup<H: StartupHost>(spec: &QuadSpec, host: &mut H) {
    let mesh = StartupMesh {
        name: spec.mesh_name,
        indices: quad_indices(),
        has_vertex_attributes: false,
    };
    let mesh = host.load_mesh_into_library(spec.library_name, mesh);
    let shader = host.load_shader_into_library(spec.shader_path, spec.shader_name);
    host.spawn(StartupBundle {
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
        scale: Vec3::ONE,
        mesh,
        shader,
        designated_pass: spec.designated_pass,
        flag: false,
        keep_on_level_unload: spec.keep_on_level_unload,
    });
}

/// `load_glade_tod_and_weather::load_snow_particles`（O 0x140ae7d90，同一骨架的第 4 个实例）。
///
/// * 门：`Res<ActiveSession>` 的 glade 字节（`+0x58`）`== 4`（Winter），否则整个函数不做任何事（O：`cmp byte ptr
///   [rax+0x58],4; jne ret`）。
/// * 网格标签 "snow particles"（长 0xe），**库名却是 "cloud"**（长 5，O：`0x142ae35d6`，与 clouds 的库名同一个字符串，
///   原作的复制粘贴，如实保留）；shader "/snow_particles.raster.hlsl" / "snow_particles_shader"（长 0x1b / 0x15）。
/// * 随后原版再 `AssetShaderLibrary::get_raster_handle_by_name("snow_particles_shader").unwrap()` 取回刚注册的句柄
///   （O 0x140ae7f1b；同名唯一，等于 `load_shader_into_library` 的返回值）。
/// * 实体：`(DrawableMeshBundle, DontCastShadow, DesignatedRenderPass(0xc))`（命令 0x1412dc470，无 KeepOnLevelUnload）。
pub const SNOW_PARTICLES_SPEC: QuadSpec = QuadSpec {
    mesh_name: "snow particles",
    library_name: "cloud",
    shader_path: "/snow_particles.raster.hlsl",
    shader_name: "snow_particles_shader",
    designated_pass: Some(0xc),
    keep_on_level_unload: false,
};

pub fn load_snow_particles<H: StartupHost>(glade: crate::resources::session::GladeType, host: &mut H) {
    if glade.is_winter() {
        run_startup(&SNOW_PARTICLES_SPEC, host);
    }
}

/// `resources::loading_screen::startup_loading_screen`（O 0x140a1a1e0，同一骨架的第 5 个实例）：无门。
///
/// 网格标签与库名都是 "loading screen"/"loading_screen"（长 0xe），shader "/loading_screen.raster.hlsl" /
/// "loading_screen"（长 0x1b / 0xe）；实体 `(DrawableMeshBundle, DesignatedRenderPass(0x12), DontCastShadow)`
/// （命令 0x1412da5d0，组件集与 snow 相同、仅元组顺序不同，无 KeepOnLevelUnload）。
pub const LOADING_SCREEN_SPEC: QuadSpec = QuadSpec {
    mesh_name: "loading screen",
    library_name: "loading_screen",
    shader_path: "/loading_screen.raster.hlsl",
    shader_name: "loading_screen",
    designated_pass: Some(0x12),
    keep_on_level_unload: false,
};

pub fn startup_loading_screen<H: StartupHost>(host: &mut H) {
    run_startup(&LOADING_SCREEN_SPEC, host);
}

#[cfg(test)]
pub(crate) mod test_host {
    use super::*;

    #[derive(Default)]
    pub struct Rec {
        pub meshes: Vec<(String, StartupMesh)>,
        pub shaders: Vec<(String, String)>,
        pub spawned: Vec<StartupBundle<u32, u32>>,
        pub order: Vec<&'static str>,
    }
    impl StartupHost for Rec {
        type Mesh = u32;
        type Shader = u32;
        fn load_mesh_into_library(&mut self, n: &str, m: StartupMesh) -> u32 {
            self.order.push("mesh");
            self.meshes.push((n.into(), m));
            100
        }
        fn load_shader_into_library(&mut self, p: &str, n: &str) -> u32 {
            self.order.push("shader");
            self.shaders.push((p.into(), n.into()));
            200
        }
        fn spawn(&mut self, b: StartupBundle<u32, u32>) {
            self.order.push("spawn");
            self.spawned.push(b);
        }
    }

    #[derive(Default)]
    pub struct Systems(pub Vec<&'static str>);
    impl AddStartupSystem for Systems {
        fn add_startup_system(&mut self, name: &'static str) {
            self.0.push(name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_host::Rec;
    use super::*;

    #[test]
    fn closure_matches_the_stack_stores() {
        assert_eq!(quad_window_indices((0, 1)), [0, 1, 2, 2, 1, 3]);
        assert_eq!(quad_window_indices((3, 4)), [6, 7, 8, 8, 7, 9]);
    }

    #[test]
    fn range_0_to_1_gives_a_single_quad() {
        assert_eq!(quad_indices(), vec![0, 1, 2, 2, 1, 3]);
    }

    #[test]
    fn startup_order_is_mesh_shader_spawn() {
        let spec = QuadSpec {
            mesh_name: "m",
            library_name: "lib",
            shader_path: "/p.hlsl",
            shader_name: "p_shader",
            designated_pass: Some(0x10),
            keep_on_level_unload: true,
        };
        let mut h = Rec::default();
        run_startup(&spec, &mut h);
        assert_eq!(h.order, vec!["mesh", "shader", "spawn"]);
        let b = &h.spawned[0];
        assert_eq!((b.mesh, b.shader), (100, 200), "句柄原样进捆绑");
        assert_eq!(
            (b.rotation, b.translation, b.scale),
            (Quat::IDENTITY, Vec3::ZERO, Vec3::ONE)
        );
        assert_eq!(b.designated_pass, Some(0x10));
        assert!(!b.flag);
        assert!(!h.meshes[0].1.has_vertex_attributes);
    }

    #[test]
    fn snow_is_gated_on_winter_and_keeps_the_original_cloud_library_name() {
        use crate::resources::session::GladeType;
        for glade in GladeType::ALL {
            let mut h = Rec::default();
            load_snow_particles(glade, &mut h);
            assert_eq!(h.spawned.len(), usize::from(glade.is_winter()), "{glade:?}");
        }
        let mut h = Rec::default();
        load_snow_particles(GladeType::Winter, &mut h);
        assert_eq!(h.meshes[0].0, "cloud");
        assert_eq!(h.meshes[0].1.name, "snow particles");
        assert_eq!(h.spawned[0].designated_pass, Some(0xc));
        assert!(!h.spawned[0].keep_on_level_unload);
    }

    #[test]
    fn loading_screen_has_no_gate_and_pass_0x12() {
        let mut h = Rec::default();
        startup_loading_screen(&mut h);
        assert_eq!(h.spawned[0].designated_pass, Some(0x12));
        assert_eq!(h.shaders[0].1, "loading_screen");
        assert_eq!(LOADING_SCREEN_SPEC.mesh_name.len(), 0xe);
    }
}
