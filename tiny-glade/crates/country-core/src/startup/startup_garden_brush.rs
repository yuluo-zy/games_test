//! 花园笔刷资源：石头与花朵按 mesh、shader、实体顺序初始化。
//! ECS 入口只负责网格；完整渲染生成复用 GardenBrushHost，花朵关闭投影。
use crate::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use crate::utils::load_json::load_json_as_mesh;
use bevy_ecs::prelude::*;

/// 笔刷网格路径与库内名称；花朵资源路径和注册名有意不同。
pub const GARDEN_BRUSH_MESHES: [(&str, &str); 2] = [
    ("assets/meshes/garden_stone.json", "garden_stone"),
    ("assets/meshes/lowpoly_flower.json", "garden_flower_03"),
];

/// 与网格表同序的着色器路径和业务名称。
pub const GARDEN_BRUSH_SHADERS: [(&str, &str); 2] = [
    (
        "/solid_vertex_color_garden_stone.raster.hlsl",
        "solid_vertex_color_garden_stone",
    ),
    (
        "/garden_lowpoly_flower.raster.hlsl",
        "garden_lowpoly_flower",
    ),
];

/// 读取并注册一个网格。资产库内部同步，共享借用即可完成注册。
pub fn register_brush_mesh(
    meshes: &AssetMeshLibrary,
    path: &str,
    name: &str,
) -> crate::asset_libraries::Handle<crate::render::mesh::Mesh> {
    let mesh = load_json_as_mesh(path);
    meshes.add(MeshAsset::new(mesh, Some(name.to_string())))
}

/// 一次性网格初始化；着色器和实体由 run_garden_brush 的宿主接口完成。
pub fn startup_garden_brush(meshes: Res<AssetMeshLibrary>) {
    for (path, name) in GARDEN_BRUSH_MESHES {
        register_brush_mesh(&meshes, path, name);
    }
}

/// 宿主提供网格、着色器和实体创建；本模块只负责顺序与数据传递。
pub trait GardenBrushHost {
    /// 注册后的网格句柄类型。
    type Mesh;
    /// 注册后的着色器句柄类型。
    type Shader;

    /// 注册已经读取的网格。
    fn load_mesh_into_library(
        &mut self,
        name: &'static str,
        mesh: crate::render::mesh::Mesh,
    ) -> Self::Mesh;

    /// 根据路径加载并注册着色器。
    fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) -> Self::Shader;

    /// 创建绘制实体，按参数决定是否关闭投影。
    fn spawn_empty_insert(
        &mut self,
        mesh: Self::Mesh,
        shader: Self::Shader,
        dont_cast_shadow: bool,
    );
}

/// 每项依次加载网格、加载着色器、创建实体。第二项花朵禁用投影。
/// 网格读取函数可注入，测试不需要访问实际游戏安装目录。
pub fn run_garden_brush<H: GardenBrushHost>(
    host: &mut H,
    mut load_mesh: impl FnMut(&str) -> crate::render::mesh::Mesh,
) {
    for (brush_index, (mesh_path, mesh_name)) in GARDEN_BRUSH_MESHES.iter().enumerate() {
        let mesh = load_mesh(mesh_path);
        let mesh_handle = host.load_mesh_into_library(mesh_name, mesh);
        let (shader_path, shader_name) = GARDEN_BRUSH_SHADERS[brush_index];
        let shader = host.load_shader_into_library(shader_path, shader_name);
        host.spawn_empty_insert(mesh_handle, shader, brush_index == 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        meshes: Vec<(&'static str, u64)>,
        shaders: Vec<(&'static str, &'static str)>,
        casts: Vec<bool>,
        order: Vec<&'static str>,
    }

    impl GardenBrushHost for Rec {
        type Mesh = u64;
        type Shader = u64;
        fn load_mesh_into_library(
            &mut self,
            name: &'static str,
            mesh: crate::render::mesh::Mesh,
        ) -> u64 {
            self.order.push("mesh");
            self.meshes.push((name, mesh.indices.len() as u64));
            100 + self.meshes.len() as u64
        }
        fn load_shader_into_library(&mut self, path: &'static str, name: &'static str) -> u64 {
            self.order.push("shader");
            self.shaders.push((path, name));
            200 + self.shaders.len() as u64
        }
        fn spawn_empty_insert(&mut self, mesh: u64, shader: u64, dont_cast_shadow: bool) {
            self.order.push("spawn");
            assert!(mesh >= 101 && shader >= 201, "handles flow into the bundle");
            self.casts.push(dont_cast_shadow);
        }
    }

    #[test]
    fn two_iterations_run_mesh_shader_spawn_and_second_insert_casts_no_shadow() {
        let mut h = Rec::default();
        run_garden_brush(&mut h, |_path| crate::render::mesh::Mesh::new("brush"));
        assert_eq!(
            h.order,
            vec!["mesh", "shader", "spawn", "mesh", "shader", "spawn"]
        );
        assert_eq!(h.meshes[0].0, "garden_stone");
        assert_eq!(h.meshes[1].0, "garden_flower_03");
        assert_eq!(
            h.shaders,
            vec![
                (
                    "/solid_vertex_color_garden_stone.raster.hlsl",
                    "solid_vertex_color_garden_stone"
                ),
                (
                    "/garden_lowpoly_flower.raster.hlsl",
                    "garden_lowpoly_flower"
                ),
            ]
        );
        assert_eq!(
            h.casts,
            vec![false, true],
            "2nd insert carries DontCastShadow"
        );
    }
}
