//! 调试图元请求与折线实体工厂。使用类型化网格和现有资产库，不再打包裸指针。

use crate::{
    asset_libraries::{mesh_library::AssetMeshLibrary, shader_library::AssetShaderLibrary},
    components::DashedLineLength,
    render::pass_builder::draw_list::DrawTopology,
    systems::ui::signifiers::{
        signifier_create::UiSignifierStream,
        signifier_factory::{SignifierEntityBuilder, SignifierFactory, SignifierMesh},
    },
};
use bevy_ecs::prelude::Commands;
use glam::Vec3;
use std::sync::Arc;

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
#[derive(Clone, Copy, PartialEq)]
pub struct DebugSphere {
    pub color: DebugColor,
    pub radius: f32,
    pub center: Vec3,
    /// 原字段 `line_width`；保留现有 Rust 调用面的字段名。
    pub alpha: f32,
    /// 原字段 `filled_in`；为假时把 `alpha` 作为线宽传给 DrawTopology。
    pub solid: bool,
}

impl std::fmt::Debug for DebugSphere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 140b05b60 的原字段名参与工厂身份，不能用重构后的别名替代。
        f.debug_struct("DebugSphere")
            .field("color", &self.color)
            .field("radius", &self.radius)
            .field("position", &self.center)
            .field("line_width", &self.alpha)
            .field("filled_in", &self.solid)
            .finish()
    }
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

impl DebugRectangle3d {
    /// 原 140b02ed0：上下两个闭合轮廓及四条竖边各自拥有工厂身份和生命周期。
    pub fn show(&self, stream: &mut UiSignifierStream) {
        let rect = ::utils::geometry::Rectangle2d {
            axis: glam::Vec2::new(self.rect[0], self.rect[1]),
            center: glam::Vec2::new(self.rect[2], self.rect[3]),
            size: glam::Vec2::new(self.rect[4], self.rect[5]),
        };
        let [bottom, top] = [self.rect[6], self.rect[7]];
        for height in [bottom, top] {
            let name = format!("{height:?}{rect:?}{:?}{}", self.color, self.dashed);
            self.append_line(
                stream,
                &name,
                rect.as_curve3(height).points,
                rect.perimeter(),
            );
        }
        for index in 0..4 {
            let point = rect.get_point(index);
            let from = Vec3::new(point.x, bottom, point.y);
            let to = Vec3::new(point.x, top, point.y);
            let name = format!("{from:?}{to:?}{:?}{}", self.color, self.dashed);
            // 原顺序先 dy²+dx²，再加 dz²，不能使用 SIMD dot 的不同归约次序。
            let delta = to - from;
            let length = (delta.z * delta.z + (delta.y * delta.y + delta.x * delta.x)).sqrt();
            self.append_line(stream, &name, vec![from, to], length);
        }
    }

    fn append_line(
        &self,
        stream: &mut UiSignifierStream,
        name: &str,
        points: Vec<Vec3>,
        length: f32,
    ) {
        let color = self.color.rgb();
        let line_width = self.line_width;
        let dashed = self.dashed;
        let factory = SignifierFactory::from_closure(
            name,
            Arc::new(
                move |meshes: &AssetMeshLibrary,
                      shaders: &AssetShaderLibrary,
                      commands: &mut Commands| {
                    let entity = SignifierEntityBuilder::new(SignifierMesh::PointsWithColors {
                        colors: vec![color; points.len()],
                        points: points.clone(),
                    })
                    .animation_duration(0.0)
                    .custom_shader(if dashed {
                        "dashed_line"
                    } else {
                        "vertex_color_shader"
                    })
                    .build(meshes, shaders, commands);
                    commands
                        .entity(entity)
                        .insert(DrawTopology::Alt(line_width));
                    // 两个原闭包 140b03a50/140b04040 均乘 6，区别于 DebugLine 的乘 4。
                    if dashed {
                        commands
                            .entity(entity)
                            .insert(DashedLineLength(length * 6.0));
                    }
                    vec![entity]
                },
            ),
        );
        stream.write_factory(&factory);
    }
}

impl DebugSphere {
    /// 原 140b02520/140b027d0：提交具名工厂，按真实共享构建器生成球体。
    pub fn show(&self, stream: &mut UiSignifierStream) {
        let name = format!("{self:?}");
        let mesh_name = self.color.sphere_mesh_name();
        let center = self.center;
        let radius = self.radius;
        let line_width = self.alpha;
        let filled_in = self.solid;
        let factory = SignifierFactory::from_closure(
            &name,
            Arc::new(
                move |meshes: &AssetMeshLibrary,
                      shaders: &AssetShaderLibrary,
                      commands: &mut Commands| {
                    let entity = SignifierEntityBuilder::from_mesh_name(mesh_name)
                        .position(center)
                        .scale(radius)
                        .animation_duration(0.0)
                        .build(meshes, shaders, commands);
                    if !filled_in {
                        commands
                            .entity(entity)
                            .insert(DrawTopology::Alt(line_width));
                    }
                    vec![entity]
                },
            ),
        );
        stream.write_factory(&factory);
    }
}

/// 二维矩形请求（O：`DebugRectangle2d`，0x140b04640 `show`）。布局 `{ color: DebugColor @0
/// （16 B：tag + Vec3 载荷）, line_width: f32 @0x10, rect: Rectangle2d @0x14（24 B）,
/// height: f32 @0x2c }`；`show` 先 `Rectangle2d::as_curve3(height)`，再按曲线折线提交。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DebugRectangle2d {
    pub color: DebugColor,
    pub line_width: f32,
    pub rect: ::utils::geometry::Rectangle2d,
    pub height: f32,
}

impl DebugRectangle2d {
    /// 原版 `DebugRectangle2d::show`：矩形闭合折线（4 角 + 回到起点，高度 `height`）经 `DebugLine`
    /// 工厂路径提交。原版闭包本体（0x140b04640 之后的工厂）未单独反编译（未闭合），这里复用
    /// 已有折线工厂，保持颜色/线宽/世界空间点语义。
    pub fn show(&self, stream: &mut UiSignifierStream) {
        let curve = self.rect.as_curve3(self.height);
        DebugLine::new(curve.points.clone())
            .color(self.color)
            .line_width(self.line_width)
            .show(stream);
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

    /// 原 140b04f70：不滤除空点列或非有限值；资源与几何前置条件在唤醒时检查。
    pub fn show(&self, stream: &mut UiSignifierStream) {
        // 原四个格式参数之间没有文字片段；绘制通道和虚线标志不参与名字。
        let name = format!(
            "{:?}{}{:?}{:?}",
            self.points,
            self.colors.is_some(),
            self.color,
            self.line_width
        );
        let points = self.points.clone();
        let colors = self
            .colors
            .clone()
            .unwrap_or_else(|| vec![self.color.rgb(); points.len()]);
        let length = original_line_length(&points);
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
                    let entity = SignifierEntityBuilder::new(SignifierMesh::PointsWithColors {
                        points: points.clone(),
                        colors: colors.clone(),
                    })
                    .animation_duration(0.0)
                    .set_ui_pass(u8::from(ui_pass))
                    .custom_shader(shader_name)
                    .build(meshes, shaders, commands);
                    // 原虚线参数先写入，线宽后写入；不将部分写入失败回滚为默认成功。
                    if dashed {
                        commands
                            .entity(entity)
                            .insert(DashedLineLength(length * 4.0));
                    }
                    commands
                        .entity(entity)
                        .insert(DrawTopology::Alt(line_width));
                    vec![entity]
                },
            ),
        );
        stream.write_factory(&factory);
    }

    /// 现有显式校验扩展；原业务的 show 不经过这个额外输入过滤器。
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
        anyhow::ensure!(
            (original_line_length(&self.points) * 4.0).is_finite(),
            "调试折线长度过大"
        );
        self.show(stream);
        Ok(())
    }
}

/// 原 SSE 顺序为先 dy²+dx²，再 dz² 加此前和，按相邻线段顺序累加。
fn original_line_length(points: &[Vec3]) -> f32 {
    let mut length = 0.0;
    for segment in points.windows(2) {
        let delta = segment[0] - segment[1];
        let xy = delta.y * delta.y + delta.x * delta.x;
        let segment_length = (delta.z * delta.z + xy).sqrt();
        length = segment_length + length;
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        asset_libraries::transient_mesh::TransientMesh,
        systems::render_loop::draw_passes::draw_lists::UiPass,
        utils::components::transform::Transform,
    };

    #[test]
    fn rectangle_wakes_six_original_factories_and_uses_six_times_dash_length() {
        use crate::systems::ui::signifiers::signifier_create::UiSignifierEntry;
        use bevy_ecs::{
            prelude::{Res, ResMut},
            system::RunSystemOnce,
            world::World,
        };
        let mut stream = UiSignifierStream::default();
        DebugRectangle3d::new([1., 0., 0., 0., 2., 4., 3., 8.])
            .color(DebugColor::Green)
            .line_width(2.)
            .dashed_line()
            .show(&mut stream);
        assert_eq!(stream.committed_len(), 6);
        let entries = stream.take_entries();
        let names: Vec<_> = entries
            .iter()
            .map(|e| match e {
                UiSignifierEntry::Factory(f) => f.name().unwrap(),
                _ => panic!(),
            })
            .collect();
        assert_eq!(
            names.iter().collect::<std::collections::HashSet<_>>().len(),
            6
        );
        assert!(names[0].starts_with("3.0Rectangle2d"));
        assert!(names[1].starts_with("8.0Rectangle2d"));
        assert!(names.iter().all(|n| n.ends_with("Greentrue")));
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
        for entry in entries {
            if let UiSignifierEntry::Factory(f) = entry {
                world.resource_mut::<UiSignifierStream>().write_factory(&f);
            }
        }
        fn wake(
            mut commands: Commands,
            meshes: Res<AssetMeshLibrary>,
            shaders: Res<AssetShaderLibrary>,
            mut stream: ResMut<UiSignifierStream>,
        ) {
            for entry in stream.take_entries() {
                if let UiSignifierEntry::Factory(f) = entry {
                    assert_eq!(f.wake(&meshes, &shaders, &mut commands).len(), 1);
                }
            }
        }
        world.run_system_once(wake).unwrap();
        let mut query = world.query::<(
            &DashedLineLength,
            &DrawTopology,
            &UiPass,
            &crate::systems::ui::signifiers::animation::SignifierAnimation,
            &TransientMesh,
        )>();
        let mut lengths = Vec::new();
        for (dash, topology, _, animation, _) in query.iter(&world) {
            lengths.push(dash.0);
            assert_eq!(*topology, DrawTopology::Alt(2.));
            assert_eq!(animation.duration, 0.);
        }
        lengths.sort_by(f32::total_cmp);
        assert_eq!(lengths, [30., 30., 30., 30., 72., 72.]);
    }

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
    fn sphere_and_rectangle2d_show_commit_one_slot_each_with_distinct_names() {
        use crate::systems::ui::signifiers::UiSignifierStream;
        use crate::systems::ui::signifiers::signifier_create::UiSignifierEntry;

        let mut stream = UiSignifierStream::default();
        DebugSphere::new(Vec3::ONE)
            .color(DebugColor::Red)
            .show(&mut stream);
        DebugSphere::new(Vec3::ONE)
            .color(DebugColor::Blue)
            .show(&mut stream);
        DebugRectangle2d {
            color: DebugColor::Green,
            line_width: 2.0,
            rect: ::utils::geometry::Rectangle2d::new_axis_aligned(
                glam::Vec2::ZERO,
                glam::Vec2::new(2.0, 4.0),
            ),
            height: 0.0,
        }
        .show(&mut stream);
        assert_eq!(stream.committed_len(), 3);
        let names: Vec<_> = stream
            .take_entries()
            .into_iter()
            .map(|entry| match entry {
                UiSignifierEntry::Factory(factory) => factory.name().unwrap().to_string(),
                other => panic!("unexpected {other:?}"),
            })
            .collect();
        assert_ne!(names[0], names[1], "colour participates in the identity");
        assert!(names[0].starts_with("DebugSphere { color: Red,"));
        assert!(names[2].ends_with("falseGreen2.0"));
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
        assert_eq!(
            name,
            "[Vec3(0.0, 0.0, 0.0), Vec3(3.0, 4.0, 0.0)]falseRed2.5"
        );

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
        assert_eq!(transform.scale, Vec3::ZERO);
        let animation = world
            .get::<crate::systems::ui::signifiers::animation::SignifierAnimation>(entity)
            .unwrap();
        assert_eq!(animation.duration.to_bits(), 0.0_f32.to_bits());
        assert_eq!(animation.flag_8, 0);
        assert_eq!(animation.value, 1.0);
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
    fn explicit_validation_is_separate_from_original_factory_identity() {
        fn name(line: &DebugLine) -> String {
            let mut stream = UiSignifierStream::default();
            line.show(&mut stream);
            let crate::systems::ui::signifiers::signifier_create::UiSignifierEntry::Factory(
                factory,
            ) = stream.take_entries().pop().unwrap()
            else {
                panic!("factory expected")
            };
            factory.name().unwrap().to_owned()
        }
        let mut line = DebugLine::new(vec![Vec3::ZERO, Vec3::X]);
        line.colors = Some(vec![Vec3::ONE]);
        let mut stream = UiSignifierStream::default();
        assert!(line.try_show(&mut stream).is_err());
        assert_eq!(stream.committed_len(), 0);
        // 原 show 接收原点列及原颜色；不会通过额外的校验扩展。
        line.show(&mut stream);
        assert_eq!(stream.committed_len(), 1);
        line.colors = None;
        let identity = name(&line);
        assert_eq!(identity, name(&line.clone().dashed()));
        assert_eq!(identity, name(&line.clone().set_ui_pass(false)));
        line.colors = Some(vec![Vec3::X; 2]);
        let red = name(&line);
        line.colors = Some(vec![Vec3::Y; 2]);
        assert_eq!(red, name(&line), "原名只记录颜色表是否存在");
        assert_ne!(identity, red);
    }

    fn shaders_for_debug() -> AssetShaderLibrary {
        let shaders = AssetShaderLibrary::new();
        for name in ["vertex_color_shader", "dashed_line"] {
            shaders.add_raster(
                crate::render::shader::RasterShaderProgram::new_draw(
                    name,
                    Default::default(),
                    &crate::render::test_device::TestDevice(1),
                ),
                false,
            );
        }
        shaders
    }

    fn wake_one(
        stream: &mut UiSignifierStream,
        meshes: &AssetMeshLibrary,
        shaders: &AssetShaderLibrary,
        world: &mut bevy_ecs::world::World,
    ) -> bevy_ecs::entity::Entity {
        use crate::systems::ui::signifiers::signifier_create::UiSignifierEntry;
        let UiSignifierEntry::Factory(factory) = stream.take_entries().pop().unwrap() else {
            panic!("factory expected")
        };
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let entities = factory.wake(meshes, shaders, &mut Commands::new(&mut queue, world));
        queue.apply(world);
        assert_eq!(entities.len(), 1);
        entities[0]
    }

    #[test]
    fn original_show_keeps_empty_and_nonfinite_payloads_until_factory_wake() {
        let mut stream = UiSignifierStream::default();
        DebugLine::new(Vec::new()).show(&mut stream);
        assert_eq!(stream.committed_len(), 1);
        let mut world = bevy_ecs::world::World::new();
        let meshes = AssetMeshLibrary::new();
        let shaders = shaders_for_debug();
        // 原空点列先进入工厂，再在共享 mesh_from_pts 的真实前置条件处失败。
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                wake_one(&mut stream, &meshes, &shaders, &mut world)
            }))
            .is_err()
        );
        let mut line = DebugLine::new(vec![Vec3::new(f32::NAN, f32::INFINITY, -0.0)]);
        line.line_width = f32::NAN;
        line.dashed = true;
        line.show(&mut stream);
        let entity = wake_one(&mut stream, &meshes, &shaders, &mut world);
        let mesh = meshes
            .get(
                *world
                    .get::<crate::asset_libraries::Handle<crate::render::mesh::Mesh>>(entity)
                    .unwrap(),
            )
            .unwrap();
        assert!(mesh.positions_without_indices()[0].x.is_nan());
        assert_eq!(mesh.positions_without_indices()[0].y, f32::INFINITY);
        assert_eq!(
            mesh.positions_without_indices()[0].z.to_bits(),
            (-0.0_f32).to_bits()
        );
        assert_eq!(world.get::<DashedLineLength>(entity).unwrap().0, 0.0);
        let DrawTopology::Alt(width) = world.get::<DrawTopology>(entity).unwrap() else {
            panic!("line topology expected")
        };
        assert!(width.is_nan());
    }

    #[test]
    fn spheres_keep_native_identity_wire_width_and_named_mesh_ownership() {
        use crate::asset_libraries::mesh_library::MeshAsset;
        use crate::render::mesh::Mesh;
        use crate::systems::ui::signifiers::animation::SignifierAnimation;
        let meshes = AssetMeshLibrary::new();
        let handle = meshes.add(MeshAsset::new(
            Mesh::new("sphere"),
            Some("debug_sphere_red".into()),
        ));
        let shaders = shaders_for_debug();
        let mut world = bevy_ecs::world::World::new();
        let sphere = DebugSphere {
            color: DebugColor::Red,
            radius: 2.0,
            center: Vec3::new(1.0, 2.0, 3.0),
            alpha: 3.5,
            solid: false,
        };
        assert_eq!(
            format!("{sphere:?}"),
            "DebugSphere { color: Red, radius: 2.0, position: Vec3(1.0, 2.0, 3.0), line_width: 3.5, filled_in: false }"
        );
        let mut stream = UiSignifierStream::default();
        sphere.show(&mut stream);
        let wire = wake_one(&mut stream, &meshes, &shaders, &mut world);
        assert_eq!(
            *world
                .get::<crate::asset_libraries::Handle<Mesh>>(wire)
                .unwrap(),
            handle
        );
        assert_eq!(
            *world.get::<DrawTopology>(wire).unwrap(),
            DrawTopology::Alt(3.5)
        );
        assert!(world.get::<TransientMesh>(wire).is_none());
        assert!(world.get::<UiPass>(wire).is_some());
        assert_eq!(
            world.get::<Transform>(wire).unwrap().translation,
            sphere.center
        );
        assert_eq!(world.get::<Transform>(wire).unwrap().scale, Vec3::ZERO);
        let animation = world.get::<SignifierAnimation>(wire).unwrap();
        assert_eq!(
            (animation.value, animation.duration, animation.flag_8),
            (2.0, 0.0, 0)
        );
        DebugSphere {
            solid: true,
            ..sphere
        }
        .show(&mut stream);
        let filled = wake_one(&mut stream, &meshes, &shaders, &mut world);
        assert!(world.get::<DrawTopology>(filled).is_none());
        assert!(world.get::<TransientMesh>(filled).is_none());
    }

    #[test]
    fn missing_named_mesh_and_shader_fail_instead_of_returning_an_empty_factory_result() {
        let mut stream = UiSignifierStream::default();
        let mut world = bevy_ecs::world::World::new();
        let meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        DebugSphere::new(Vec3::ZERO).show(&mut stream);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                wake_one(&mut stream, &meshes, &shaders, &mut world)
            }))
            .is_err()
        );
        DebugLine::new(vec![Vec3::ZERO, Vec3::X]).show(&mut stream);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                wake_one(&mut stream, &meshes, &shaders, &mut world)
            }))
            .is_err()
        );
    }

    #[test]
    fn adjacent_line_lengths_keep_native_f32_order_and_nonfinite_results() {
        assert_eq!(original_line_length(&[]).to_bits(), 0.0_f32.to_bits());
        let points = [
            Vec3::ZERO,
            Vec3::new(3.0, 4.0, 0.0),
            Vec3::new(3.0, 4.0, 12.0),
        ];
        assert_eq!(original_line_length(&points), 17.0);
        assert!(original_line_length(&[Vec3::ZERO, Vec3::splat(f32::MAX)]).is_infinite());
        assert!(original_line_length(&[Vec3::ZERO, Vec3::splat(f32::NAN)]).is_nan());
    }
}
