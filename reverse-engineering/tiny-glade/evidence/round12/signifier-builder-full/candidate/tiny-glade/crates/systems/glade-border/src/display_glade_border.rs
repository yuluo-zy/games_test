//! display_glade_border.rs — 谷地边界调试可视化（B 0x141be0ad0 + closure$0/closure$1，
//! 2026-10-17 fresh 反编译重建 + wave3 落库 closure$1）。
//!
//! 签名（O：PDB 候选接口 + 机器码寄存器映射，8 参）：
//! `(Res<AppMode>, Res<GladeBorder>, ResMut<UiSignifierStream>,
//!  Res<TerrainHeightsData<TerrainHeightSsbo>>, Res<GameTime>,
//!  EventReader<DisplayOutOfBorder>, EventWriter<ShowDemoBorderOverlay>, Res<GameConfig>)`。
//!
//! 控制流（O，逐段，disasm 0x141be0ad0..0x141be0eff）：
//! 1. 模式门：`dword[*param1] != 0x13`（AppMode::Photomode）才继续；
//! 2. 折线构造：`ShapeParameters::as_curve2`（0x1411ff2b0）→ 2D 轮廓，逐点
//!    `TerrainHeightsData::sample_at_world_xz`（0x141246fb0）+ `utils::swizzlers::xvy`
//!    （0x140c9cde0）抬升为 3D 折线（closure$0，行 46）；
//! 3. 事件门：EventReader 有未读 `DisplayOutOfBorder` 才画 `DebugLine`
//!    （new/color=Red/line_width=4.0/set_ui_pass=false/show，0x140b04e70/eb0/f40/ee0/f70）；
//!    随后进程全局 `*0x14362c098 == 1`（DEV_OVERRIDE_IS_DEMO_MODE，B 构建 = 0）才写
//!    `ShowDemoBorderOverlay`；
//! 4. perimeter 门：`GameConfig[+0x121] == 0` 时用 `perimeter`（0x1411ff640）+ `format!`
//!    （格式串 "glade_border" 12B 无空格，1 个 Display 参数 = GameTime+0x2c `frame_count`）
//!    构造 48B 记录 {tag=1, kind=1, 折线 Vec{ptr,len,cap}, perimeter f32@+0x28}，经
//!    `SignifierFactory::new`（0x14089d8b0）+ `UiSignifierStream::write`（0x140b2cf90）
//!    写边界 signifier（closure$1 @0x141be1080 虚线实体装配）。
//!
//! ## 消费者接线（2026-10-17 晚，restorer-consume-debugline）
//! - **分歧① 已关闭**：core `DebugLine::show`（0x140b04f70）落库后，本系统在事件门内
//!   直接 `line.show(&mut signifiers)`（fresh disasm 0x141be0ce9：`RCX=&DebugLine`、
//!   `RDX=*[RSI]`=流 → `CALL 0x140b04f70`）。DebugLine 构建链逐指令复核：color tag=1
//!   （`[RBP-0x50]=1` = Red）、line_width=`[0x142934694]`=4.0、set_ui_pass=false。
//! - **分歧② perimeter 唤醒已接线**：fresh 复核 `SignifierFactory::new`（0x14089d8b0）=
//!   `{name_len, name_ptr, name_len, inner, marker}`；`inner` = ArcInner 基址
//!   `{strong=1@0, weak=1@8, data@+0x10}`，data = `{points.cap@0, points.ptr@+8,
//!   points.len@+0x10, perimeter@+0x18}`（0x30 B alloc，vtable 0x142efa150，size 0x20）。
//!   crate 侧改以 `SignifierFactory::from_closure(&label, Arc::new(closure))` 表达同一
//!   记录，`signifier_create` 唤醒 `closure$1` —— 不再写 raw 5-qword 槽（raw 槽不会
//!   唤醒闭包、且原 `mem::forget` 会泄漏折线 Vec）。
//! - `closure$1`（0x141be1080）fresh 复核：env 读 `+8=ptr / +0x10=len / +0x18=length`；
//!   实体装配后返回 **1 元素 `Vec<Entity>`**（机器码写 `{1, ptr, 1}` sret），供
//!   `signifier_create` 插 `SignifierId`。原实现返回 `()`，本批已纠正。

use std::sync::Arc;

use bevy_ecs::prelude::*;
use glam::Vec3;
use utils::swizzlers::xvy;

use country_core::asset_libraries::mesh_library::AssetMeshLibrary;
use country_core::asset_libraries::shader_library::AssetShaderLibrary;
use country_core::components::DashedLineLength;
use country_core::game_config::GameConfig;
use country_core::render::pass_builder::draw_list::DrawTopology;
use country_core::resources::glade_border::{
    BorderShape, DisplayOutOfBorder, GladeBorder, ShowDemoBorderOverlay,
};
use country_core::resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData};
use country_core::resources::time::GameTime;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::signifiers::SignifierEntityBuilder;
use country_core::systems::ui::signifiers::SignifierFactory;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::utils::debug_viz::{DebugColor, DebugLine};

/// 边界曲线采样步长（O：`ShapeParameters::as_curve2` 0x1411ff2b0 圆路径 = 0.2，
/// country-core `Shape::as_curve2` 同源）。
const BORDER_CURVE_STEP: f32 = 0.2;

/// 进程全局门禁（O：`*0x14362c098 == 1` → `DEV_OVERRIDE_IS_DEMO_MODE`；B 构建
/// 0x143634d90 全 0，默认不触发）。建模同 water crate 门禁常量族。
pub fn border_overlay_enabled() -> bool {
    BORDER_OVERLAY_ENABLED
}
const BORDER_OVERLAY_ENABLED: bool = false;

/// `display_glade_border` @0x141be0ad0（O：控制流逐段对应；`DebugLine::show` 与
/// perimeter closure 唤醒均已接线）。
#[allow(clippy::too_many_arguments)]
pub fn display_glade_border(
    mode: Res<AppMode>,
    border: Res<GladeBorder>,
    mut signifiers: ResMut<UiSignifierStream>,
    heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    game_time: Res<GameTime>,
    mut display: EventReader<DisplayOutOfBorder>,
    mut overlay: EventWriter<ShowDemoBorderOverlay>,
    config: Res<GameConfig>,
) {
    // 1. 模式门（O：`CMP dword[RAX],0x13; JZ 尾部`）。
    if mode.native_tag() == AppMode::Photomode.native_tag() {
        return;
    }
    // 2. 折线构造（O：as_curve2 → Map(closure$0) → from_iter；closure$0 = 地形采样抬升）。
    let points: Vec<Vec3> = match border.0 {
        BorderShape::Circle(circle) => circle.as_curve2(BORDER_CURVE_STEP),
        BorderShape::Rectangle(rect) => rect.as_curve2(),
    }
    .points
    .into_iter()
    .map(|p| xvy(p, heights.sample_at_world_xz(p.x, p.y)))
    .collect();

    // 3. 事件门（O：`display.read().last().is_some()` 才进入绘制分支；光标推进到双缓冲末尾）。
    if display.read().last().is_some() {
        // O：DebugLine 构建链（0x140b04e70/eb0/f40/ee0/f70，fresh disasm
        //     0x141be0c80..0x141be0ce9）：
        //     color tag=1（`MOV dword [RBP-0x50],1` = Red）、line_width=`[0x142934694]`=4.0、
        //     set_ui_pass=false（R8D=0）。随后 `show(&self, &mut UiSignifierStream)`
        //     （RCX=&line、RDX=流），把折线以原生 40B `SignifierFactory` 记录压入流。
        let line = DebugLine::new(points.clone())
            .color(DebugColor::Red)
            .line_width(4.0)
            .set_ui_pass(false);
        line.show(&mut signifiers);
        // O：`*0x14362c098 == 1`（DEV_OVERRIDE_IS_DEMO_MODE）→ ShowDemoBorderOverlay。
        if border_overlay_enabled() {
            overlay.write(ShowDemoBorderOverlay);
        }
    }

    // 4. perimeter signifier 门（O：`CMP byte[*GameConfig+0x121],0; JZ 画`）。
    if !config.border_signifier_enabled {
        let perimeter = match border.0 {
            BorderShape::Circle(circle) => circle.perimeter(),
            BorderShape::Rectangle(rect) => rect.perimeter(),
        };
        // O：label = `format!("glade_border{}", game_time.frame_count)`（格式串 @0x142efa130
        // = "glade_border" 12B 无空格，1 个 Display 参数 = GameTime+0x2c frame_count，
        // 格式化器 0x140d8d560）。
        let label = format!("glade_border{}", game_time.frame_count);
        // O（fresh disasm 0x141be0e1e-0x141be0e97）：
        // 1. `alloc 0x30` 写 ArcInner：`{strong=1@0, weak=1@8, data@+0x10}`，
        //    data = `{points cap@+0x10, ptr@+0x18, len@+0x20, perimeter@+0x28}`
        //    （+0x80=cap / +0x88=ptr / +0x90=len 由 from_iter 输出与 12*len
        //    dealloc 实证）；
        // 2. `SignifierFactory::new`（0x14089d8b0）= `{name_len, name_ptr(拷贝), name_len,
        //    inner(=ArcInner 基址), marker(vtable 0x142efa150)}`；
        // 3. `UiSignifierStream::write`（0x140b2cf90）写 5×qword。
        //
        // crate 侧以 `from_closure` 表达同一记录：名字按原语义拷贝，闭包（= Arc vtable
        // 唤醒面）拥有折线 Vec/周长，`signifier_create` 唤醒 [`closure_1`]。原生格式串
        // 在 write 后 `__rust_dealloc`；`from_closure` 的临时 label 同样在作用域末释放。
        let factory = SignifierFactory::from_closure(
            &label,
            Arc::new(
                move |mesh_library: &AssetMeshLibrary,
                      shader_library: &AssetShaderLibrary,
                      commands: &mut Commands| {
                    // O：closure$1 的 env = ArcInner data：`{cap@0, ptr@+8, len@+0x10,
                    // length@+0x18}`。折线 Vec 由闭包捕获持有，指针在调用期间有效。
                    let env = GladeBorderSignifierEnv {
                        points_cap: points.capacity(),
                        points_ptr: points.as_ptr(),
                        points_len: points.len(),
                        length: perimeter,
                    };
                    closure_1(&env, mesh_library, shader_library, commands)
                },
            ),
        );
        signifiers.write_factory(&factory);
    }
}

/// perimeter signifier 的 ArcInner 数据面（O：closure$1 的 env，0x20 B）：
/// `{points.cap@0, points.ptr@+8, points.len@+0x10, length(perimeter)@+0x18}`。
///
/// 原生 ArcInner<closure_env$1> = `{strong=1@0, weak=1@8, 本结构@+0x10}`（alloc 0x30），
/// vtable 0x142efa150（drop 0x14014b880 / size 0x20 / align 8 / call_once 0x141be0a30）。
/// `cap` 只被 drop 使用（closure$1 不读）；此处显式保留以记录布局。
pub struct GladeBorderSignifierEnv {
    /// @+0：折线 Vec 容量（原生 drop 释放用）。
    pub points_cap: usize,
    /// @+8：折线数据指针。
    pub points_ptr: *const Vec3,
    /// @+0x10：折线点数。
    pub points_len: usize,
    /// @+0x18：折线总长 f32（→ DashedLineLength ×0.7）。
    pub length: f32,
}

/// `display_glade_border::closure$1` @0x141be1080（O，fresh 逐指令复核）。
///
/// env（in_RDX，见 [`GladeBorderSignifierEnv`]）读取：`+8=ptr`、`+0x10=len`、
/// `+0x18=length`。fresh disasm 0x141be1080 逐段：
/// 1. `alloc 12*len` + `memcpy` 复制折线（长度 0 时用 dangling，与 Rust `Vec` 同）；
/// 2. `SignifierEntityBuilder::new`（0x14089d9b0，载荷 = 6 qword：首 =
///    `0x8000000000000002`，余 = 折线 {cap(或len),ptr,len} + 2×栈未初始化 0）→
///    `set_ui_pass(0)`（0x14089dad0）→ `animation_duration(0.0)`（0x14089dc90）→
///    `custom_shader("dashed_line"@0x142efa180，len 0xb)`（0x14089db00）→
///    `build`（0x14089dd50，实参含 `&AssetMeshLibrary`/`&AssetShaderLibrary`/
///    `&mut Commands`）；
/// 3. `Commands` 队列 2 条 `insert`（按原顺序）：
///    - `insert(DrawTopology)`：值 `0x4000000000000001`（tag 1 = `Alt`，f32 2.0）；
///    - `insert(DashedLineLength)`：值 = env.length × 0.7（0x1429cd030）；
/// 4. 返回 1 元素 `Vec<Entity>`（sret 写 `{cap=1, ptr=heap(Entity), len=1}`）。
pub fn closure_1(
    env: &GladeBorderSignifierEnv,
    meshes: &AssetMeshLibrary,
    shaders: &AssetShaderLibrary,
    commands: &mut Commands,
) -> Vec<Entity> {
    // 1. O：复制折线（disasm 0x141be10ed-0x141be115e：alloc 12*len + memcpy）。
    let points: Vec<Vec3> = if env.points_ptr.is_null() || env.points_len == 0 {
        Vec::new()
    } else {
        // SAFETY：env.points_ptr/points_len 由调用方构造，指向 len 个 Vec3。
        unsafe { std::slice::from_raw_parts(env.points_ptr, env.points_len) }.to_vec()
    };
    let entity = SignifierEntityBuilder::from_points(points)
        .set_ui_pass(0)
        .animation_duration(0.0)
        .custom_shader("dashed_line")
        .build(meshes, shaders, commands);
    // 3. O：2× Commands insert（disasm 0x141be1214-0x141be1295）。
    //    DrawTopology = `0x4000000000000001` = {tag u32=1 → Alt, f32 2.0}。
    commands.entity(entity).insert(DrawTopology::Alt(2.0));
    //    DashedLineLength = env.length × 0.7（0x1429cd030）；插入真实组件
    //    `country_core::components::DashedLineLength`（TypeId 0x4782fcee8a8c578b，
    //    `f32 @+0`，与 `DebugLine::show` 同型）。渲染读取面
    //    `draw_callbacks_a::DashedLineLengthView` 仍是同 TypeId 的镜像类型，跨模块
    //    登记（本 crate 不再插 View，避免两个不同 ECS 类型）。
    commands
        .entity(entity)
        .insert(DashedLineLength(env.length * 0.7));
    // 4. O：返回 1 元素 Vec<Entity>（sret 写 {1, ptr, 1}）。
    vec![entity]
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::system::RunSystemOnce;
    use bevy_ecs::world::World;

    // The dependency crate's cfg(test) TestDevice is deliberately unavailable
    // here. This local fixture only admits CPU shader artifacts, never a GPU.
    struct CpuShaders;
    use country_core::render::shader::boundary::{
        CompiledShaderPipeline, ComputePipeline, ComputePipelineDesc, Error, RasterPipeline,
        RasterPipelineDesc, RasterPipelineKey, ShaderBackend,
    };
    impl ShaderBackend for CpuShaders {
        fn load_compiled_compute(
            &self,
            _: &ComputePipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            unreachable!()
        }
        fn load_compiled_raster(
            &self,
            _: &RasterPipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            Ok(CompiledShaderPipeline::default())
        }
        fn raster_shader_variants(&self, d: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
            [d.clone(), d.clone()]
        }
        fn create_compute_pipeline(
            &self,
            _: &str,
            _: std::sync::Arc<CompiledShaderPipeline>,
        ) -> Result<ComputePipeline, country_core::rhapsody_types::BackendError> {
            unreachable!()
        }
        fn create_raster_pipeline(
            &self,
            _: std::sync::Arc<CompiledShaderPipeline>,
            _: &RasterPipelineKey,
        ) -> Result<RasterPipeline, Error> {
            unreachable!()
        }
    }

    /// 接线验证：perimeter 记录携带的闭包（= vtable 0x142efa150 唤醒面）调用
    /// [`closure_1`] 后，按原顺序落 `DrawTopology::Alt(2.0)` 与
    /// `DashedLineLength(perimeter × 0.7)`，并返回 1 元素 `Vec<Entity>`。
    #[test]
    fn closure_1_wakes_and_inserts_dashed_border_components() {
        let mut world = World::new();
        let meshes = AssetMeshLibrary::new();
        let shaders = AssetShaderLibrary::new();
        shaders.add_raster(
            country_core::render::shader::RasterShaderProgram::new_draw(
                "dashed_line",
                Default::default(),
                &CpuShaders,
            ),
            false,
        );
        let points = vec![Vec3::ZERO, Vec3::new(3.0, 4.0, 0.0)];
        world
            .run_system_once(move |mut commands: Commands| {
                let env = GladeBorderSignifierEnv {
                    points_cap: points.capacity(),
                    points_ptr: points.as_ptr(),
                    points_len: points.len(),
                    length: 4.0,
                };
                let spawned = closure_1(&env, &meshes, &shaders, &mut commands);
                assert_eq!(spawned.len(), 1);
            })
            .unwrap();

        let mut topology = world.query::<&DrawTopology>();
        let all: Vec<DrawTopology> = topology.iter(&world).copied().collect();
        assert_eq!(all, vec![DrawTopology::Alt(2.0)]);

        let mut dashed = world.query::<&DashedLineLength>();
        let lens: Vec<f32> = dashed.iter(&world).map(|d| d.0).collect();
        assert_eq!(lens, vec![2.8], "4.0 * 0.7");
    }
}
