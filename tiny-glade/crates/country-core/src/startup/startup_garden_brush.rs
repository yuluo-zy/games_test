//! 花园笔刷资源：石头与花朵按 mesh、shader、实体顺序初始化。
//! 原 B `startup_garden_brush` @0x1415f87c0；花朵关闭投影。
use std::path::Path;

use crate::{
    asset_libraries::{
        Handle,
        mesh_library::{AssetMeshLibrary, MeshAsset},
        meshwatch::MeshWatch,
        shader_library::AssetShaderLibrary,
    },
    components::drawable::DrawableMeshBundle,
    render::{
        mesh::Mesh,
        shader::{RasterShaderProgram, boundary::ShaderBackend},
    },
    systems::render_loop::draw_passes::draw_lists::DontCastShadow,
    utils::{components::transform::Transform, load_json::try_load_json_as_mesh},
};
use bevy_ecs::system::Commands;

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

/// 由设备宿主调用；失败立即返回，调用方必须处理启动失败。
pub fn startup_garden_brush(
    root: &Path,
    meshes: &AssetMeshLibrary,
    watch: &MeshWatch,
    shaders: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    commands: &mut Commands,
) -> anyhow::Result<()> {
    let mut host = GardenBrushAssetHost {
        meshes,
        watch,
        shaders,
        backend,
        commands,
    };
    run_garden_brush(&mut host, |path| {
        let full = root.join(path);
        let path = full
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("garden mesh path is not UTF-8: {}", full.display()))?;
        try_load_json_as_mesh(path)
    })
}

struct GardenBrushAssetHost<'a, 'w, 's> {
    meshes: &'a AssetMeshLibrary,
    watch: &'a MeshWatch,
    shaders: &'a AssetShaderLibrary,
    backend: &'a dyn ShaderBackend,
    commands: &'a mut Commands<'w, 's>,
}

impl GardenBrushHost for GardenBrushAssetHost<'_, '_, '_> {
    type Mesh = Handle<Mesh>;
    type Shader = Handle<RasterShaderProgram>;

    fn load_mesh_into_library(
        &mut self,
        name: &'static str,
        mesh: Mesh,
    ) -> anyhow::Result<Self::Mesh> {
        self.watch.watch(&mesh);
        Ok(self.meshes.add(MeshAsset::new(mesh, Some(name.to_owned()))))
    }

    fn load_shader_into_library(
        &mut self,
        path: &'static str,
        name: &'static str,
    ) -> anyhow::Result<Self::Shader> {
        super::load_shader_into_library(
            name,
            crate::rhapsody_types::RasterPipelineDesc {
                path: path.to_owned(),
                entry: String::new(),
                defines: Vec::new(),
            },
            self.backend,
            self.shaders,
        )
    }

    fn spawn_empty_insert(
        &mut self,
        mesh: Self::Mesh,
        shader: Self::Shader,
        dont_cast_shadow: bool,
    ) {
        let bundle = DrawableMeshBundle {
            mesh,
            shader,
            transform: Transform::identity(),
        };
        if dont_cast_shadow {
            self.commands.spawn((bundle, DontCastShadow));
        } else {
            self.commands.spawn(bundle);
        }
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
    ) -> anyhow::Result<Self::Mesh>;

    /// 根据路径加载并注册着色器。
    fn load_shader_into_library(
        &mut self,
        path: &'static str,
        name: &'static str,
    ) -> anyhow::Result<Self::Shader>;

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
    mut load_mesh: impl FnMut(&str) -> anyhow::Result<crate::render::mesh::Mesh>,
) -> anyhow::Result<()> {
    for (brush_index, (mesh_path, mesh_name)) in GARDEN_BRUSH_MESHES.iter().enumerate() {
        let mesh = load_mesh(mesh_path)?;
        let mesh_handle = host.load_mesh_into_library(mesh_name, mesh)?;
        let (shader_path, shader_name) = GARDEN_BRUSH_SHADERS[brush_index];
        let shader = host.load_shader_into_library(shader_path, shader_name)?;
        host.spawn_empty_insert(mesh_handle, shader, brush_index == 1);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::test_device::TestDevice;
    use bevy_ecs::world::{CommandQueue, World};

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
        ) -> anyhow::Result<u64> {
            self.order.push("mesh");
            self.meshes.push((name, mesh.indices.len() as u64));
            Ok(100 + self.meshes.len() as u64)
        }
        fn load_shader_into_library(
            &mut self,
            path: &'static str,
            name: &'static str,
        ) -> anyhow::Result<u64> {
            self.order.push("shader");
            self.shaders.push((path, name));
            Ok(200 + self.shaders.len() as u64)
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
        run_garden_brush(&mut h, |_path| Ok(crate::render::mesh::Mesh::new("brush"))).unwrap();
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

    #[test]
    fn mesh_failure_stops_before_registration_and_later_drawables() {
        let mut h = Rec::default();
        let result = run_garden_brush(&mut h, |path| {
            if path == GARDEN_BRUSH_MESHES[1].0 {
                anyhow::bail!("missing flower mesh");
            }
            Ok(crate::render::mesh::Mesh::new("stone"))
        });
        assert!(result.is_err());
        assert_eq!(h.order, ["mesh", "shader", "spawn"]);
    }

    #[test]
    fn shipped_garden_meshes_create_real_library_entries_and_drawables() {
        let root = std::env::var_os("TG_INSTALL_ROOT")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../../ljxsj_92385/Tiny Glade")
            });
        let meshes = AssetMeshLibrary::new();
        let watch = MeshWatch::new(false);
        let shaders = AssetShaderLibrary::new();
        let mut world = World::new();
        let mut queue = CommandQueue::default();
        startup_garden_brush(
            &root,
            &meshes,
            &watch,
            &shaders,
            &TestDevice::default(),
            &mut Commands::new(&mut queue, &world),
        )
        .unwrap();
        queue.apply(&mut world);

        let stone = meshes.get_handle_by_name("garden_stone");
        let flower = meshes.get_handle_by_name("garden_flower_03");
        assert!(!meshes.get(stone).unwrap().indices.is_empty());
        assert!(!meshes.get(flower).unwrap().indices.is_empty());
        let stone_shader = shaders
            .get_raster_handle_by_name("solid_vertex_color_garden_stone")
            .unwrap();
        let flower_shader = shaders
            .get_raster_handle_by_name("garden_lowpoly_flower")
            .unwrap();
        let mut query = world.query::<(
            &Handle<Mesh>,
            &Handle<RasterShaderProgram>,
            Option<&DontCastShadow>,
        )>();
        let drawables: Vec<_> = query.iter(&world).collect();
        assert_eq!(drawables.len(), 2);
        assert!(drawables.iter().any(|(mesh, shader, no_shadow)| {
            **mesh == stone && **shader == stone_shader && no_shadow.is_none()
        }));
        assert!(drawables.iter().any(|(mesh, shader, no_shadow)| {
            **mesh == flower && **shader == flower_shader && no_shadow.is_some()
        }));
    }
}
