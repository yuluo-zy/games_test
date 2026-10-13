//! startup_quad.rs — `stars` / `clouds` / `billboard_forest` 三个 startup 系统共用的“单面片网格 + shader + spawn”骨架。
//!
//! 三个 `startup_*`（0x140903ca0 / 0x14085b8b0 / 0x140ae8080，各 662 B）逐条相同，只差常量：
//!
//! 1. 索引：`Vec<u32>::from_iter(FlatMap<TupleWindows<RangeInclusive<u32>, (u32,u32)>, [u32;6], closure$0>)`
//!    （单态化体 0x1412fa8e0，ICF 后三个 closure 共用一份）。范围初值 `0..=1`（O：`[rbp+0x2c] = 0x100000000` 即
//!    start = 0、end = 1，`exhausted = 0`），所以只有一个窗口 `(0, 1)`；`closure$0((a, b))` =
//!    `[2a, 2a+1, 2b, 2b, 2a+1, 2b+1]`（O：栈上依次写 `edx+edx, 2*rdx+1, eax+eax, eax+eax, 2*rdx+1, 2*rax+1`）。
//!    结果 `[0, 1, 2, 2, 1, 3]`（两个三角形拼成的四边形）。
//! 2. 网格：索引 Vec + 其余顶点属性为 `None`（`i64::MIN` 作 niche，O：`movabs rcx, 0x8000000000000000`）+ 空
//!    `HashMap<_, _, RandomState>`（`std::hash::random` 的 KEYS 线程局部）+ 一个 32 B 的常量（`f32` 半径 + `u32 = 1` +
//!    零，O：`0x142aa9130` 等；三处半径不同，见各文件）+ 名字 `&str`。→ `load_mesh_into_library(mesh, name)`。
//! 3. shader：`load_shader_into_library<&str>(path, name)`（路径形如 "/star.raster.hlsl"，名形如 "star_shader"）。
//! 4. spawn：`Commands::spawn_empty()` + 一次 `insert(bundle)`：`Transform::IDENTITY`（旋转 `(0,0,0,1)`、平移 0、
//!    缩放 1）、网格句柄、shader 句柄、`DrawableMeshBundle` 里的一个 `bool = false`（`+0x58` / billboard 里 `+0x48`）；
//!    stars / clouds 另有一个 `u32 = 0x10`（`DesignatedRenderPass`，I）。`DontCastShadow` / `KeepOnLevelUnload` 是零大小标记，
//!    机器码里没有数据。
//!
//! 外部依赖（`load_*_into_library`、`Commands`、`SubApp::add_systems`）都经 [`StartupHost`] / [`AddStartupSystem`] 边界表达。

use glam::{Quat, Vec3};
use itertools::Itertools;

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
    /// 32 B 常量的第一个字（`f32`）。
    pub far_param: f32,
    /// 32 B 常量的第二个字（`u32`，O：三处都是 1）。
    pub kind: u32,
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
    /// 捆绑里的 `bool`，三处都是 `false`（语义 U）。
    pub flag: bool,
}

/// 外部调用边界。
pub trait StartupHost {
    type Mesh;
    type Shader;
    /// `country_core::startup::load_mesh_into_library`（0x140b1aca0）：`library_name` 是传给库的名字。
    fn load_mesh_into_library(&mut self, library_name: &str, mesh: StartupMesh) -> Self::Mesh;
    /// `country_core::startup::load_shader_into_library<&str>`（0x140b3fe40）。
    fn load_shader_into_library(&mut self, path: &str, name: &str) -> Self::Shader;
    /// `Commands::spawn_empty` + `insert`。
    fn spawn(&mut self, bundle: StartupBundle<Self::Mesh, Self::Shader>);
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
    pub far_param: f32,
    pub shader_path: &'static str,
    pub shader_name: &'static str,
    pub designated_pass: Option<u32>,
}

/// 三个 `startup_*` 的共同体。
pub fn run_startup<H: StartupHost>(spec: &QuadSpec, host: &mut H) {
    let mesh = StartupMesh {
        name: spec.mesh_name,
        indices: quad_indices(),
        has_vertex_attributes: false,
        far_param: spec.far_param,
        kind: 1,
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
    });
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
            far_param: 1.5,
            shader_path: "/p.hlsl",
            shader_name: "p_shader",
            designated_pass: Some(0x10),
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
        assert_eq!(h.meshes[0].1.kind, 1);
        assert!(!h.meshes[0].1.has_vertex_attributes);
    }
}
