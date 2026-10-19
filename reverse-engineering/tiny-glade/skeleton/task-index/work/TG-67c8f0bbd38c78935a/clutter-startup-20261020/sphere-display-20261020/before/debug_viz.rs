//! 调试图元请求与折线实体工厂。使用类型化网格和现有资产库，不再打包裸指针。

use crate::{
    asset_libraries::{
        mesh_library::{AssetMeshLibrary, MeshAsset},
        shader_library::AssetShaderLibrary,
        transient_mesh::TransientMesh,
    },
    components::{DashedLineLength, drawable::DrawableMeshBundle},
    render::{mesh::ATTR_COLOR, pass_builder::draw_list::DrawTopology},
    systems::{
        render_loop::draw_passes::draw_lists::{DontCastShadow, UiPass},
        ui::signifiers::{
            animation::SignifierAnimation,
            signifier_create::{SignifierId, UiSignifierStream},
            signifier_factory::SignifierFactory,
        },
    },
    utils::{components::transform::Transform, geo::mesh_from_pts},
};
use bevy_ecs::prelude::Commands;
use glam::Vec3;
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

/// 调试图元的颜色。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DebugColor {
    #[default]
    White,
    Red,
    Blue,
    Green,
    Grey,
    Custom(Vec3),
}

impl DebugColor {
    pub fn tag(self) -> u32 {
        match self {
            Self::White => 0,
            Self::Red => 1,
            Self::Blue => 2,
            Self::Green => 3,
            Self::Grey => 4,
            Self::Custom(_) => 5,
        }
    }

    pub fn rgb(self) -> Vec3 {
        match self {
            Self::White => Vec3::new(1.0, 1.0, 1.0),
            Self::Red => Vec3::new(1.0, 0.0, 0.0),
            Self::Blue => Vec3::new(0.0, 0.0, 1.0),
            Self::Green => Vec3::new(0.0, 1.0, 0.0),
            Self::Grey => Vec3::new(0.1, 0.1, 0.1),
            Self::Custom(rgb) => rgb,
        }
    }

    pub fn as_array(self) -> [f32; 3] {
        let rgb = self.rgb();
        [rgb.x, rgb.y, rgb.z]
    }

    pub fn sphere_mesh_name(self) -> &'static str {
        match self {
            Self::White => "debug_sphere_white",
            Self::Red => "debug_sphere_red",
            Self::Blue => "debug_sphere_blue",
            Self::Green => "debug_sphere_green",
            Self::Grey => "debug_sphere_grey",
            Self::Custom(_) => "debug_sphere_custom",
        }
    }
}

/// 球体请求参数。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugSphere {
    pub color: DebugColor,
    pub radius: f32,
    pub center: Vec3,
    pub alpha: f32,
    pub solid: bool,
}

impl DebugSphere {
    pub fn new(center: Vec3) -> Self {
        Self {
            color: DebugColor::White,
            radius: 0.1,
            center,
            alpha: 1.0,
            solid: false,
        }
    }

    pub fn color(mut self, color: DebugColor) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn solid(mut self) -> Self {
        self.solid = true;
        self
    }
}

/// 三维矩形请求参数；rect 的解释由调用方约定。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugRectangle3d {
    pub color: DebugColor,
    pub line_width: f32,
    pub rect: [f32; 8],
    pub dashed: bool,
}

impl DebugRectangle3d {
    pub fn new(rect: [f32; 8]) -> Self {
        Self {
            color: DebugColor::White,
            line_width: 1.0,
            rect,
            dashed: false,
        }
    }

    pub fn color(mut self, color: DebugColor) -> Self {
        self.color = color;
        self
    }

    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn dashed_line(mut self) -> Self {
        self.dashed = true;
        self
    }
}

/// 折线请求；颜色可按点提供，也可使用统一颜色。
#[derive(Clone, Debug, PartialEq)]
pub struct DebugLine {
    pub points: Vec<Vec3>,
    pub colors: Option<Vec<Vec3>>,
    pub color: DebugColor,
    pub line_width: f32,
    pub ui_pass: bool,
    pub dashed: bool,
}

impl DebugLine {
    pub fn new(points: Vec<Vec3>) -> Self {
        Self {
            points,
            colors: None,
            color: DebugColor::White,
            line_width: 1.0,
            ui_pass: true,
            dashed: false,
        }
    }
    pub fn color(mut self, color: DebugColor) -> Self {
        self.color = color;
        self
    }
    pub fn set_ui_pass(mut self, ui_pass: bool) -> Self {
        self.ui_pass = ui_pass;
        self
    }
    pub fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.line_width = line_width;
        self
    }

    /// 无效调试输入只记录告警，不中断主编辑流程。
    pub fn show(&self, stream: &mut UiSignifierStream) {
        if let Err(error) = self.try_show(stream) {
            log::warn!("调试折线未提交：{error}");
        }
    }

    /// 校验成功才提交工厂。名字包含实际颜色、绘制通道和虚线模式，避免错误去重。
    pub fn try_show(&self, stream: &mut UiSignifierStream) -> anyhow::Result<()> {
        anyhow::ensure!(self.points.len() >= 2, "调试折线至少需要两个点");
        anyhow::ensure!(
            self.points.iter().all(|point| point.is_finite()),
            "调试折线包含非有限位置"
        );
        anyhow::ensure!(
            self.line_width.is_finite() && self.line_width > 0.0,
            "调试线宽必须为有限正数"
        );
        anyhow::ensure!(self.color.rgb().is_finite(), "调试颜色必须有限");
        if let Some(colors) = &self.colors {
            anyhow::ensure!(
                colors.len() == self.points.len() && colors.iter().all(|color| color.is_finite()),
                "逐点颜色数量必须匹配位置且全部有限"
            );
        }
        let length: f32 = self
            .points
            .windows(2)
            .map(|segment| segment[0].distance(segment[1]))
            .sum();
        anyhow::ensure!((length * 4.0).is_finite(), "调试折线长度过大");
        let name = format!("debug_line:{:016x}", self.identity_hash());
        let points = self.points.clone();
        let colors = self
            .colors
            .clone()
            .unwrap_or_else(|| vec![self.color.rgb(); points.len()]);
        let line_width = self.line_width;
        let ui_pass = self.ui_pass;
        let dashed = self.dashed;
        let factory = SignifierFactory::from_closure(
            &name,
            Arc::new(
                move |meshes: &AssetMeshLibrary,
                      shaders: &AssetShaderLibrary,
                      commands: &mut Commands| {
                    let shader_name = if dashed {
                        "dashed_line"
                    } else {
                        "vertex_color_shader"
                    };
                    let Some(shader) = shaders.get_raster_handle_by_name(shader_name) else {
                        log::warn!("调试折线着色器未注册：{shader_name}");
                        return Vec::new();
                    };
                    let mut mesh = mesh_from_pts("debug_line", &points);
                    mesh.set_attribute(ATTR_COLOR, colors.clone());
                    let mesh = meshes.add(MeshAsset::new(mesh, None));
                    let entity = commands
                        .spawn((
                            DrawableMeshBundle {
                                mesh,
                                shader,
                                transform: Transform::identity(),
                            },
                            TransientMesh::new_with_registry(mesh, meshes.transient_meshes()),
                            DrawTopology::Alt(line_width),
                            DontCastShadow,
                            SignifierId(0),
                            // 点已经位于世界空间；不让通用指示符动画把线缩放到零。
                            SignifierAnimation {
                                flag_8: 1,
                                duration: 0.0,
                                value: 1.0,
                                t: 1.0,
                                eased: 1.0,
                                ..Default::default()
                            },
                        ))
                        .id();
                    if ui_pass {
                        commands.entity(entity).insert(UiPass);
                    }
                    if dashed {
                        commands
                            .entity(entity)
                            .insert(DashedLineLength(length * 4.0));
                    }
                    vec![entity]
                },
            ),
        );
        stream.write_factory(&factory);
        Ok(())
    }

    /// 哈希浮点位模式，不创建整条点列的格式化字符串；分隔长度防止不同点列拼接混淆。
    fn identity_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.points.len().hash(&mut hasher);
        for point in &self.points {
            point.to_array().map(f32::to_bits).hash(&mut hasher);
        }
        self.colors.is_some().hash(&mut hasher);
        if let Some(colors) = &self.colors {
            colors.len().hash(&mut hasher);
            for color in colors {
                color.to_array().map(f32::to_bits).hash(&mut hasher);
            }
        } else {
            self.color.as_array().map(f32::to_bits).hash(&mut hasher);
        }
        self.line_width.to_bits().hash(&mut hasher);
        self.ui_pass.hash(&mut hasher);
        self.dashed.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colour_tags_follow_the_original_table_order() {
        let all = [
            DebugColor::White,
            DebugColor::Red,
            DebugColor::Blue,
            DebugColor::Green,
            DebugColor::Grey,
            DebugColor::Custom(Vec3::new(0.2, 0.4, 0.6)),
        ];
        assert_eq!(all.map(DebugColor::tag), [0, 1, 2, 3, 4, 5]);
        assert_eq!(DebugColor::default(), DebugColor::White);
        assert_eq!(DebugColor::Blue.rgb(), Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(DebugColor::Green.rgb(), Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(DebugColor::Grey.rgb(), Vec3::splat(0.1));
        assert_eq!(all[5].rgb(), Vec3::new(0.2, 0.4, 0.6));
    }

    #[test]
    fn sphere_mesh_names_have_the_original_lengths() {
        let lens = [0x12, 0x10, 0x11, 0x12, 0x11, 0x13];
        let colours = [
            DebugColor::White,
            DebugColor::Red,
            DebugColor::Blue,
            DebugColor::Green,
            DebugColor::Grey,
            DebugColor::Custom(Vec3::ZERO),
        ];
        for (colour, len) in colours.into_iter().zip(lens) {
            assert_eq!(colour.sphere_mesh_name().len(), len, "{colour:?}");
        }
    }

    #[test]
    fn sphere_defaults_match_the_constructor_bytes() {
        let sphere = DebugSphere::new(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(sphere.radius.to_bits(), 0x3dcccccd);
        assert_eq!(sphere.alpha, 1.0);
        assert!(!sphere.solid);
        let sphere = sphere.radius(2.0).solid().color(DebugColor::Red);
        assert_eq!(
            (sphere.radius, sphere.solid, sphere.color),
            (2.0, true, DebugColor::Red)
        );
    }

    #[test]
    fn rectangle_and_line_defaults() {
        let rect = DebugRectangle3d::new([0.0; 8]);
        assert_eq!((rect.line_width, rect.dashed), (1.0, false));
        assert!(rect.dashed_line().dashed);
        let line = DebugLine::new(vec![Vec3::ZERO]);
        assert_eq!(
            (line.line_width, line.ui_pass, line.dashed),
            (1.0, true, false)
        );
        assert!(line.colors.is_none());
        let line = line.set_ui_pass(false).dashed().line_width(3.0);
        assert_eq!(
            (line.ui_pass, line.dashed, line.line_width),
            (false, true, 3.0)
        );
    }

    #[test]
    fn show_packs_the_record_and_wakes_the_dashed_line_closure() {
        use bevy_ecs::prelude::{Commands, Res, ResMut};
        use bevy_ecs::system::RunSystemOnce;
        use bevy_ecs::world::World;

        use crate::components::DashedLineLength;
        use crate::render::pass_builder::draw_list::DrawTopology;
        use crate::systems::ui::signifiers::UiSignifierStream;
        use crate::systems::ui::signifiers::signifier_create::UiSignifierEntry;

        let mut stream = UiSignifierStream::default();
        let line = DebugLine::new(vec![Vec3::ZERO, Vec3::new(3.0, 4.0, 0.0)])
            .color(DebugColor::Red)
            .line_width(2.5)
            .set_ui_pass(false)
            .dashed();
        line.show(&mut stream);
        assert_eq!(stream.committed_len(), 1);

        let entries = stream.take_entries();
        let UiSignifierEntry::Factory(factory) = &entries[0] else {
            panic!("DebugLine::show must push a native factory slot");
        };
        assert!(factory.has_closure());
        let name = factory.name().expect("factory name is utf-8");
        // 工厂使用紧凑哈希名，完整绘制参数参与去重。
        assert!(name.starts_with("debug_line:"));

        // 唤醒：等价于 signifier_create 的 vtable 调用槽。
        let mut world = World::new();
        world.init_resource::<AssetMeshLibrary>();
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            crate::render::shader::RasterShaderProgram::new_draw(
                "dashed_line",
                crate::rhapsody_types::RasterPipelineDesc::default(),
                &crate::render::test_device::TestDevice(1),
            ),
            false,
        );
        world.insert_resource(shaders);
        world.init_resource::<UiSignifierStream>();
        world
            .resource_mut::<UiSignifierStream>()
            .write_factory(factory);

        fn wake(
            mut commands: Commands,
            mesh: Res<AssetMeshLibrary>,
            shader: Res<AssetShaderLibrary>,
            mut stream: ResMut<UiSignifierStream>,
        ) {
            let mut spawned = 0usize;
            for entry in stream.take_entries() {
                if let UiSignifierEntry::Factory(factory) = entry {
                    spawned += factory.wake(&mesh, &shader, &mut commands).len();
                }
            }
            assert_eq!(spawned, 1);
        }
        world.run_system_once(wake).unwrap();

        let mut drawables = world.query::<(
            bevy_ecs::entity::Entity,
            &crate::asset_libraries::Handle<crate::render::mesh::Mesh>,
            &crate::asset_libraries::Handle<crate::render::shader::RasterShaderProgram>,
            &Transform,
            &TransientMesh,
        )>();
        let (entity, mesh_handle, shader_handle, transform, transient) =
            drawables.single(&world).unwrap();
        assert_eq!(transform.scale, Vec3::ONE);
        assert_eq!(transient.mesh_handle(), *mesh_handle);
        assert_eq!(
            world
                .resource::<AssetShaderLibrary>()
                .get_raster(*shader_handle)
                .unwrap()
                .name,
            "dashed_line"
        );
        assert!(world.get::<UiPass>(entity).is_none());
        let mesh_handle = *mesh_handle;
        assert_eq!(
            world
                .resource::<AssetMeshLibrary>()
                .get(mesh_handle)
                .unwrap()
                .positions_without_indices(),
            &[Vec3::ZERO, Vec3::new(3.0, 4.0, 0.0)]
        );
        let mut dashed = world.query::<&DashedLineLength>();
        let lengths: Vec<f32> = dashed.iter(&world).map(|d| d.0).collect();
        assert_eq!(lengths, vec![20.0], "|(3,4,0)| = 5.0, x4.0");
        // 走真实绘制回调，防止创建端和消费端再次定义不同身份的组件。
        let draw_input =
            crate::systems::render_loop::draw_passes::draw_callbacks_a::read_env57(&world, entity);
        assert_eq!(draw_input.dashed_len, 20.0);
        assert_eq!(
            draw_input.index_count,
            world
                .resource::<AssetMeshLibrary>()
                .get(mesh_handle)
                .unwrap()
                .indices
                .len() as u64
        );

        let mut topology = world.query::<&DrawTopology>();
        let topologies: Vec<DrawTopology> = topology.iter(&world).copied().collect();
        assert_eq!(topologies, vec![DrawTopology::Alt(2.5)]);
        let registry = Arc::clone(world.resource::<AssetMeshLibrary>().transient_meshes());
        world.despawn(entity);
        assert_eq!(*registry.lock().unwrap(), vec![mesh_handle]);
    }

    #[test]
    fn external_factory_shape_matches_water_closure_zero() {
        use crate::systems::ui::signifiers::UiSignifierStream;
        use crate::systems::ui::signifiers::signifier_create::UiSignifierEntry;

        let mut stream = UiSignifierStream::default();
        let factory = SignifierFactory::from_closure(
            "brush_circle_scribble",
            Arc::new(|_mesh, _shader, _commands| Vec::new()),
        );
        stream.write_factory(&factory);
        let entries = stream.take_entries();
        let UiSignifierEntry::Factory(slot) = &entries[0] else {
            panic!("closure factory must use the native slot");
        };
        assert_eq!(slot.name(), Some("brush_circle_scribble"));
    }

    #[test]
    fn invalid_colors_are_not_committed_and_render_modes_have_distinct_ids() {
        let mut line = DebugLine::new(vec![Vec3::ZERO, Vec3::X]);
        line.colors = Some(vec![Vec3::ONE]);
        let mut stream = UiSignifierStream::default();
        assert!(line.try_show(&mut stream).is_err());
        assert_eq!(stream.committed_len(), 0);
        line.colors = None;
        let identity = line.identity_hash();
        assert_ne!(identity, line.clone().dashed().identity_hash());
        assert_ne!(identity, line.clone().set_ui_pass(false).identity_hash());
        line.colors = Some(vec![Vec3::X; 2]);
        let red = line.identity_hash();
        line.colors = Some(vec![Vec3::Y; 2]);
        assert_ne!(red, line.identity_hash());
    }
}
