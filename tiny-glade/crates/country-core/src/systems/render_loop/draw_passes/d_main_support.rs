//! `d_main_support.rs` — draw_passes 共用的 pass 宿主层与“列表驱动”运行器（d_main 新增，无独立 PDB 过程）。
//!
//! 原版里每个 pass 都是一个 bevy 系统风格的自由函数：先 `RasterPass::new_impl(name, 颜色…, 深度, 视口, &World, &ctx)`，
//! 再 `Components::get_resource_id(Time)` 取 `Duration::as_secs_f32` 作 `CommonDrawParams.time`，然后逐元素
//! `DrawPacket::new → DrawBuilder::new → bind_common → 状态位改写 → 回调`，最后 `RasterPass::render`。
//! 这些步骤在 wireframe / ui / loading / main / oc2 / late 里是同一段内联代码，差别只在下面 [`ListPass`] 里的参数。
//!
//! A4 的 `RasterPass` 需要一个 `Box<dyn PassHost>`（对应 `PassResources::new_from(world)` + 图录制器）；
//! 本层用 [`DrawPassCtx`] 的工厂闭包按 pass 生成宿主。

use core::time::Duration;

use alloc::boxed::Box;
use bevy_ecs::world::World;

use crate::asset_libraries::color_library::ColorLibrarySsbo;
use crate::render::pass_builder::draw_builder::{CommonDrawFlag, CommonDrawParams, DrawBuilder};
use crate::render::pass_builder::draw_stream::{PassHost, RasterPass};
use crate::render::pass_builder::framebuffer_binding::{
    AttachmentTexture, ColorAttachment, DepthAttachment, DepthClear,
};
use crate::render::shader::RasterShaderProgram;
use crate::rhapsody_contract::Handle;
use crate::systems::render_loop::draw_passes::draw_lists::DrawEntry;

/// `ColorAttachment` 的 tag（O：`RasterPass::new_impl` 读取的 dword）。A4 的枚举只有 tag0/tag1，
/// `main_pass` 的首个 `G-buffer` pass 用了 **tag 2**（O：`mov dword [rbp+0x988], 2`），下面用 [`ColorKind::Tag2`] 表达并在建 pass 后补丁。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorKind {
    /// tag0：访问 0x36。
    Tag0,
    /// tag1：访问 0x2e，附带 16 B 视图字（O：全 0）。
    Tag1,
    /// tag2：访问 0x2e（`(tag == 0) * 8 + 0x2e`），引用在 `+8`。
    Tag2,
}

/// 一个颜色附件说明。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorSpec {
    pub tex: AttachmentTexture,
    pub kind: ColorKind,
}

impl ColorSpec {
    pub fn tag0(tex: AttachmentTexture) -> ColorSpec {
        ColorSpec {
            tex,
            kind: ColorKind::Tag0,
        }
    }
    pub fn tag1(tex: AttachmentTexture) -> ColorSpec {
        ColorSpec {
            tex,
            kind: ColorKind::Tag1,
        }
    }
    pub fn tag2(tex: AttachmentTexture) -> ColorSpec {
        ColorSpec {
            tex,
            kind: ColorKind::Tag2,
        }
    }
}

/// 深度附件说明（O：`DepthAttachment` tag 0 可写 / tag 1 只读 / tag 2 无）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DepthSpec {
    /// tag2。
    None,
    /// tag0；`clear` = `+0x10` 非 0 时的清除值（`+0x14`）。
    Write {
        tex: AttachmentTexture,
        layer: u32,
        clear: Option<f32>,
    },
    /// tag1。
    ReadOnly { tex: AttachmentTexture },
}

impl DepthSpec {
    fn attachment(&self) -> Option<DepthAttachment> {
        match *self {
            DepthSpec::None => None,
            DepthSpec::Write { tex, layer, clear } => Some(DepthAttachment {
                writable: true,
                layer,
                texture: tex,
                clear: clear.map_or(DepthClear::Load, DepthClear::Clear),
            }),
            DepthSpec::ReadOnly { tex } => Some(DepthAttachment {
                writable: false,
                layer: 0,
                texture: tex,
                clear: DepthClear::Load,
            }),
        }
    }
}

/// `AssetShaderLibrary::get_raster_handle_by_name(&str) -> Option<Handle<RasterShaderProgram>>`（0x140937aa0，O：返回 `al` 命中标志 +
/// `rdx` 句柄）。库本体属 `asset_libraries::shader_library`（不在本范围），这里只取调用面。
pub trait ShaderLookup {
    fn raster_handle_by_name(&self, name: &str) -> Option<Handle<RasterShaderProgram>>;
}

/// 空查找表（任何名字都未命中）。
pub struct NoShaders;
impl ShaderLookup for NoShaders {
    fn raster_handle_by_name(&self, _name: &str) -> Option<Handle<RasterShaderProgram>> {
        None
    }
}

/// pass 运行环境：World、`Res<Time>` 的已流逝时间、以及按 pass 生成宿主的工厂。
pub struct DrawPassCtx<'a> {
    pub world: &'a mut World,
    /// `Res<Time>`（TypeId 0x4bc362195b80f729）`+0` 的 `Duration`（secs u64 @0、nanos u32 @8）。
    pub elapsed: Duration,
    new_host: &'a mut dyn FnMut() -> Box<dyn PassHost>,
    /// `Res<AssetShaderLibrary>` 的按名查找（`get_raster_handle_by_name`）。
    pub shaders: &'a dyn ShaderLookup,
}

impl<'a> DrawPassCtx<'a> {
    pub fn new(
        world: &'a mut World,
        elapsed: Duration,
        new_host: &'a mut dyn FnMut() -> Box<dyn PassHost>,
        shaders: &'a dyn ShaderLookup,
    ) -> DrawPassCtx<'a> {
        DrawPassCtx {
            world,
            elapsed,
            new_host,
            shaders,
        }
    }

    /// `get_raster_handle_by_name(name)`，未命中 → `Option::expect` 失败（各调用点文案见各 pass）。
    pub fn shader(&self, name: &str) -> Option<Handle<RasterShaderProgram>> {
        self.shaders.raster_handle_by_name(name)
    }

    /// 取一个新的 `PassHost`（对应 `RasterPass::new_impl` 的 `&World` + `&ctx` 参数）。
    pub fn host(&mut self) -> Box<dyn PassHost> {
        (self.new_host)()
    }

    /// `CommonDrawParams::new`（inline）：`Duration::as_secs_f32` + 布尔标志。
    pub fn common_params(&self, flag: impl Into<CommonDrawFlag>) -> CommonDrawParams {
        CommonDrawParams::new(self.elapsed, flag)
    }

    /// 建 `RasterPass`（`new_impl`）：颜色/深度/视口；tag2 颜色补丁访问类型。
    pub fn raster_pass(
        &mut self,
        name: &'static str,
        colors: &[ColorSpec],
        depth: DepthSpec,
        viewport: AttachmentTexture,
    ) -> RasterPass {
        let atts: Vec<ColorAttachment> = colors
            .iter()
            .map(|c| match c.kind {
                ColorKind::Tag1 => ColorAttachment::View {
                    view: [0; 4],
                    texture: c.tex,
                },
                _ => ColorAttachment::Texture(c.tex),
            })
            .collect();
        let host = self.host();
        let d = depth.attachment();
        let mut pass = RasterPass::new(name, &atts, d.as_ref(), host)
            .with_viewport([viewport.extent.0, viewport.extent.1]);
        for (i, c) in colors.iter().enumerate() {
            if c.kind == ColorKind::Tag2 {
                // O：tag2 的访问 = 0x2e，描述里的 `+4` 覆盖为 2。
                pass.colors[i].tag = 2;
                pass.accesses[i].access =
                    crate::render::pass_builder::framebuffer_binding::COLOR_ACCESS_VIEW;
            }
        }
        pass
    }
}

/// 逐元素的状态位改写模式。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemMode {
    /// 无状态改写。
    Plain,
    /// `ui_pass` 0x140837ee2 / `loading_screen_pass` 0x140838292：
    /// `bind_common` 后 `state.byte3 = (byte3 & 0x9f) | 0x40`。
    /// Premultiplied alpha blending; all other state bits are preserved.
    Ui,
    /// `bind_common` 之后：`state.byte3 |= 0x80`（`set_polygon_mode`）、`state.byte0 &= 0x7f`（`set_depth_test_enable(false)`）、
    /// 混合 1.0（`wireframe_pass`）。
    Wireframe,
    /// `DrawBuilder::new` 之后读取 `stencil_ref`（`Option::unwrap`，None → panic）写入 `packet.+0xb1`，
    /// `bind_common` 之后 `state.byte1 = (byte1 & 0x8e) | 0x21`（`with_stencil_write`）。
    Stencil,
    /// 在构造包**之前**把状态改成 `(state & 0xffff_ff8f) | 0x20`（`Velocity` / `Velocity OC/2`）。
    Velocity,
    /// `shadow_pass`：`bind_common` 的标志字节是 `CommonDrawFlag`（O：`movzx r14d, byte [params+4]`），这里传 `cascade + 2`
    /// （`ShadowCascade0..2`，A4 已改 u8 枚举，不再需要覆盖写 uniform）；之后 `state.byte0 &= 0xcf`（O：`and byte [..], -0x31`）。
    Shadow { flag: u8 },
    /// 透明 pass：`bind_common` + 额外绑定之后 `state.byte0 &= 0xbf`（`and al, -0x41`）；`byte3_field2` 为真时再
    /// `byte3 = (byte3 & 0x9f) | 0x40`（`pre/post_taa` 有，`refraction_copy` 没有）。
    Transparent { byte3_field2: bool },
}

/// 额外的 bind 步骤（`bind_common` 之后、状态改写之前）。
pub type ExtraBind<'s> = &'s (dyn for<'p> Fn(DrawBuilder<'p>) -> DrawBuilder<'p> + Sync);

/// `bind_common` 里 `write_uniform<u32>(…, flag)` 的 uniform 名哈希（O：`movabs -0x3a5498c04f79ae0b`）。
pub const BIND_COMMON_FLAG_INT_UNIFORM: u64 = 0xc5ab_673f_b086_51f5;

/// `Option::unwrap` 失败文案（原 `unwrap_failed` 的固定文本）。
pub const UNWRAP_NONE: &str = "called `Option::unwrap()` on a `None` value";

/// `with_stencil_write`（main_pass.rs:22）：`set_stencil_test_enable` + `set_stencil_pass_op`，
/// 机器码为 `byte1 = (byte1 & 0x8e) | 0x21`（O：`and al, -0x72; or al, 0x21`）。
pub fn with_stencil_write(state: u32) -> u32 {
    let b1 = (state >> 8) & 0xff;
    let nb1 = (b1 & 0x8e) | 0x21;
    (state & !0xff00) | (nb1 << 8)
}

/// `Velocity` 状态改写（O：`and esi, -0x71; or esi, 0x20`）。
pub fn velocity_state(state: u32) -> u32 {
    (state & 0xffff_ff8f) | 0x20
}

/// `wireframe_pass` 状态改写（O：`or byte [+0xab], 0x80; and byte [+0xa8], 0x7f`）。
pub fn wireframe_state(state: u32) -> u32 {
    (state | 0x8000_0000) & !0x80
}

/// 一次“列表驱动 pass”的全部参数。
pub struct ListPass<'s> {
    pub name: &'static str,
    pub colors: &'s [ColorSpec],
    pub depth: DepthSpec,
    /// 视口取自哪张纹理的 `extent_2d`。
    pub viewport: AttachmentTexture,
    /// `CommonDrawParams` 的布尔标志（O：`mov byte [..+0x89c], 1/0`）。
    pub flag: bool,
    pub mode: ItemMode,
    /// `DrawBuilder::new` 的着色器变体（O：`xor r9d, r9d` = 0；`shadow_pass` 为 `mov r9b, 1`）。
    pub variant: u8,
    /// `bind_common` 之后的额外纹理绑定（`refraction_copy` / `pre_taa`）。
    pub extra: Option<ExtraBind<'s>>,
}

/// 运行一个列表驱动 pass（O：见模块头）。
pub fn run_list_pass(ctx: &mut DrawPassCtx<'_>, spec: &ListPass<'_>, items: &[DrawEntry]) {
    let mut pass = ctx.raster_pass(spec.name, spec.colors, spec.depth, spec.viewport);
    let mut params = ctx.common_params(spec.flag);
    if let ItemMode::Shadow { flag } = spec.mode {
        params.flag = CommonDrawFlag::from_byte(flag).expect("shadow cascade flag out of range");
    }
    let loop_started = crate::render::declare_probe::start();
    collect_list(&mut pass, spec, items, &params, ctx.world);
    crate::render::declare_probe::list_loop(loop_started, items.len());
    pass.render();
}

/// Collect an explicitly independent CPU layer in the shared task pool. Graph
/// mutation and GPU dependency insertion remain on the owner in stable order.
/// Each task owns one whole pass: there are no nested pool waits.
pub fn run_independent_list_passes(ctx: &mut DrawPassCtx<'_>, specs: &[ListPass<'_>], items: &[DrawEntry]) {
    let layer: Vec<_> = specs.iter().map(|spec| (spec, items)).collect();
    run_list_layer(ctx, &layer);
}

/// CPU-independent declarations may have GPU write/read dependencies: those
/// remain in the original graph because Commit never reorders the layer.
pub fn run_list_layer(ctx: &mut DrawPassCtx<'_>, layer: &[(&ListPass<'_>, &[DrawEntry])]) {
    if !collect_layer(ctx, layer) {
        for (spec, items) in layer { run_list_pass(ctx, spec, items); }
    }
}

fn collect_layer(ctx: &mut DrawPassCtx<'_>, layer: &[(&ListPass<'_>, &[DrawEntry])]) -> bool {
    let draws = layer.iter().fold(0usize, |n, (_, items)| n.saturating_add(items.len()));
    if !(2..=8).contains(&layer.len()) || draws == 0 || draws > COLLECT_MAX_DRAWS { return false; }
    let mut jobs = smallvec::SmallVec::<[_; 8]>::with_capacity(layer.len());
    let mut bytes = 0usize;
    let prepare_started = crate::render::declare_probe::start();
    // Probe admission before preparing any expensive reflection metadata.
    let host = ctx.host();
    if draws < host.collect_chunk_draws().max(1).saturating_mul(2) { return false; }
    let mut constant_plan = if host.collect_constant_plan_enabled() {
        let mut plan = match host.checkout_draw_scratch() {
            Ok(plan) => plan,
            Err(error) => return reject_collect_layer(ctx, layer[0].0, error),
        };
        if let Err(error) = crate::render::pass_builder::draw_commit_scratch::prepare_constant_plan(&mut plan, draws) {
            drop(plan);
            return reject_collect_layer(ctx, layer[0].0, error);
        }
        Some(plan)
    } else { None };
    for &(spec, items) in layer {
        let plan_start = constant_plan.as_ref().map_or(0, |plan| plan.value().constant_sizes.len());
        let host = ctx.host();
        host.begin_collect_scope();
        for (item, _) in items {
            let shader = Handle(crate::rhapsody_contract::ManagedHandle(item.shader), core::marker::PhantomData);
            let Some(reflection) = host.raster_shader(&shader, spec.variant) else { return false; };
            bytes = bytes.saturating_add(reflection.uniform_block_size as usize);
            if bytes > COLLECT_MAX_CONSTANT_BYTES { return false; }
            if let Some(plan) = &mut constant_plan { plan.with_prepared(|plan| plan.constant_sizes.push(reflection.uniform_block_size)); }
        }
        let mut params = ctx.common_params(spec.flag);
        if let ItemMode::Shadow { flag } = spec.mode {
            params.flag = CommonDrawFlag::from_byte(flag).expect("shadow cascade flag out of range");
        }
        let factory = host.collect_factory();
        jobs.push((factory, [spec.viewport.extent.0, spec.viewport.extent.1], params, plan_start));
    }
    crate::render::declare_probe::collect_stage(0, prepare_started);
    let started = crate::render::declare_probe::start();
    let world: &World = ctx.world;
    let chunk_draws = host.collect_chunk_draws().max(1);
    let mut pages = Vec::new();
    for (index, (_, items)) in layer.iter().enumerate() {
        for chunk in items.chunks(chunk_draws) { pages.push((index, chunk)); }
    }
    let mut batches: Vec<Vec<_>> = (0..layer.len()).map(|_| Vec::new()).collect();
    let mut positions = [0usize; 8];
    for wave in pages.chunks_mut(8) {
        #[cfg(feature = "cpu-probe")]
        let count_wave = crate::render::declare_probe::collect_count_wave(true);
        let mut offsets = [0usize; 8];
        if constant_plan.is_some() {
            for (slot, (index, items)) in wave.iter().enumerate() {
                offsets[slot] = jobs[*index].3 + positions[*index];
                positions[*index] += items.len();
            }
        }
        let sizes = constant_plan.as_ref().map(|plan| &plan.value().constant_sizes);
        let results = rhapsody::rg::map_chunks(wave, |slot, (index, items)| {
            let (factory, viewport, params, _) = &jobs[*index];
            let mut local = RasterPass::collector(factory(), *viewport);
            if let Some(sizes) = sizes {
                local.host.prepare_collect_constants(&sizes[offsets[slot]..offsets[slot] + items.len()]);
            }
            declare_items(&mut local, layer[*index].0, items, params, world);
            (*index, local.into_draw_batch())
        });
        for (index, batch) in results { batches[index].push(batch); }
        #[cfg(feature = "cpu-probe")]
        count_wave.complete(wave.len());
    }
    crate::render::declare_probe::collect_stage(1, started);
    drop(constant_plan); // All scoped workers joined; Commit may reuse this slot.
    let commit_started = crate::render::declare_probe::start();
    for (&(spec, items), batches) in layer.iter().zip(batches) {
        let mut pass = ctx.raster_pass(spec.name, spec.colors, spec.depth, spec.viewport);
        let task_count = batches.len();
        for batch in batches { pass.commit_draw_batch(batch); }
        crate::render::declare_probe::collected(task_count);
        crate::render::declare_probe::list_loop(None, items.len());
        pass.render();
    }
    crate::render::declare_probe::cross_pass_collected(pages.len());
    crate::render::declare_probe::collect_stage(2, commit_started);
    true
}

fn reject_collect_layer(ctx: &mut DrawPassCtx<'_>, spec: &ListPass<'_>, error: rhapsody::Error) -> bool {
    #[cfg(feature = "cpu-probe")]
    crate::render::declare_probe::collect_counts_failed();
    let mut pass = ctx.raster_pass(spec.name, spec.colors, spec.depth, spec.viewport);
    pass.host.record_collect_error(error);
    pass.render();
    true
}

fn declare_items(pass: &mut RasterPass, spec: &ListPass<'_>, items: &[DrawEntry], params: &CommonDrawParams, world: &World) {
    for (item, func) in items {
        let mut state = pass.host.default_raster_state();
        if spec.mode == ItemMode::Velocity {
            state.0 = velocity_state(state.0);
        }
        let packet = item.to_draw_packet(state);
        let mut builder = DrawBuilder::new(pass, packet, spec.variant);
        if spec.mode == ItemMode::Stencil {
            builder.packet_mut().unknown_b1 = item.stencil_ref.expect(UNWRAP_NONE);
        }
        let mut builder = builder.bind_common::<ColorLibrarySsbo>(params, &item.params.transform);
        if let Some(extra) = spec.extra {
            builder = extra(builder);
        }
        match spec.mode {
            ItemMode::Ui => {
                let p = builder.packet_mut();
                p.state = p.state.set_byte3_field_2();
            }
            ItemMode::Transparent { byte3_field2 } => {
                let p = builder.packet_mut();
                p.state.0 &= !0x40;
                if byte3_field2 {
                    p.state = p.state.set_byte3_field_2();
                }
            }
            ItemMode::Wireframe => {
                let p = builder.packet_mut();
                p.state.0 = wireframe_state(p.state.0);
                p.blend = 1.0;
            }
            ItemMode::Stencil => {
                let p = builder.packet_mut();
                p.state.0 = with_stencil_write(p.state.0);
            }
            ItemMode::Shadow { .. } => {
                let p = builder.packet_mut();
                p.state.0 &= !0x30;
            }
            _ => {}
        }
        func(builder, &item.params, world);
    }
}

/// Bounded worker-owned Collect pages followed by stable owner Commit.
/// Large passes use the same path in successive pages; no declaration is replayed.
pub(crate) const COLLECT_MAX_DRAWS: usize = 4096;
pub(crate) const COLLECT_MAX_CONSTANT_BYTES: usize = 32 * 1024 * 1024;

/// Shared admission rules for list, generated and surface draws. Reflection
/// is resolved by the owner before a worker receives its immutable input.
pub(crate) fn draw_constant_bytes(host: &dyn PassHost, shader: u64, variant: u8) -> usize {
    let shader = Handle(crate::rhapsody_contract::ManagedHandle(shader), core::marker::PhantomData);
    host.raster_shader(&shader, variant).expect("draw shader reflection missing").uniform_block_size as usize
}

pub(crate) fn collect_draw_pages<T: Sync, R: Send + 'static>(
    pass: &mut RasterPass, items: &[T],
    constant_bytes: impl Fn(&dyn PassHost, &T) -> usize,
    declare: impl Fn(&mut RasterPass, &[T]) -> R + Send + Sync,
    mut commit: impl FnMut(R),
) {
    if items.is_empty() { return; }
    let chunk_draws = pass.host.collect_chunk_draws().max(1);
    let viewport = pass.viewport;
    let mut offset = 0;
    while offset < items.len() {
        let prepare_started = crate::render::declare_probe::start();
        pass.host.begin_collect_scope();
        let count = (items.len() - offset).min(COLLECT_MAX_DRAWS);
        let mut constant_plan = if pass.host.collect_constant_plan_enabled() {
            let mut plan = match pass.host.checkout_draw_scratch() {
                Ok(plan) => plan,
                Err(error) => {
                    #[cfg(feature = "cpu-probe")]
                    crate::render::declare_probe::collect_counts_failed();
                    pass.host.record_collect_error(error); return;
                }
            };
            if let Err(error) = crate::render::pass_builder::draw_commit_scratch::prepare_constant_plan(&mut plan, count) {
                drop(plan);
                #[cfg(feature = "cpu-probe")]
                crate::render::declare_probe::collect_counts_failed();
                pass.host.record_collect_error(error);
                return;
            }
            Some(plan)
        } else { None };
        let mut bytes = 0usize;
        let mut end = offset;
        for item in &items[offset..items.len().min(offset + COLLECT_MAX_DRAWS)] {
            let size = constant_bytes(pass.host.as_ref(), item);
            assert!(size <= COLLECT_MAX_CONSTANT_BYTES, "draw constants exceed Collect page budget");
            if bytes.saturating_add(size) > COLLECT_MAX_CONSTANT_BYTES { break; }
            bytes += size;
            if let Some(plan) = &mut constant_plan { plan.with_prepared(|plan| plan.constant_sizes.push(size as u32)); }
            end += 1;
        }
        crate::render::declare_probe::collect_stage(0, prepare_started);
        let factory = pass.host.collect_factory();
        let sizes = constant_plan.as_ref().map(|plan| &plan.value().constant_sizes);
        let mut plan_offset = 0usize;
        // At most eight in-flight tasks; each receives the configured 64 draws
        // by default. Completed pages are committed before accepting more.
        for wave in items[offset..end].chunks(chunk_draws.saturating_mul(8)) {
            #[cfg(feature = "cpu-probe")]
            let count_wave = crate::render::declare_probe::collect_count_wave(false);
            let started = crate::render::declare_probe::start();
            let mut chunks = wave.chunks(chunk_draws).collect::<smallvec::SmallVec<[_; 8]>>();
            let batches = rhapsody::rg::map_chunks(&mut chunks, |slot, chunk| {
                let mut local = RasterPass::collector(factory(), viewport);
                if let Some(sizes) = sizes {
                    let start = plan_offset + slot * chunk_draws;
                    local.host.prepare_collect_constants(&sizes[start..start + chunk.len()]);
                }
                let result = declare(&mut local, chunk);
                (local.into_draw_batch(), result)
            });
            crate::render::declare_probe::collect_stage(1, started);
            let started = crate::render::declare_probe::start();
            for (batch, result) in batches { pass.commit_draw_batch(batch); commit(result); }
            crate::render::declare_probe::collect_stage(2, started);
            crate::render::declare_probe::collected(chunks.len());
            #[cfg(feature = "cpu-probe")]
            count_wave.complete(chunks.len());
            plan_offset += wave.len();
        }
        offset = end;
    }
}

fn collect_list(pass: &mut RasterPass, spec: &ListPass<'_>, items: &[DrawEntry], params: &CommonDrawParams, world: &World) {
    collect_draw_pages(pass, items,
        |host, (item, _)| draw_constant_bytes(host, item.shader, spec.variant),
        |local, chunk| declare_items(local, spec, chunk, params, world), |_| {});
}


#[cfg(test)]
mod collect_page_tests {
    use super::*;
    #[test]
    fn pages_bound_worker_inputs_and_merge_uneven_results_without_owner_execution() {
        use crate::render::pass_builder::draw_stream::test_host::MockHost;
        let owner = std::thread::current().id();
        for count in [0usize, 1, 63, 64, 65, 513, 4097] {
            let (host, _) = MockHost::new();
            let mut pass = RasterPass::collector(host.boxed(), [1280, 720]);
            let items: Vec<_> = (0..count).collect();
            let mut observed = Vec::new();
            #[cfg(feature = "cpu-probe")]
            assert!(crate::render::declare_probe::collect_counts_begin());
            collect_draw_pages(&mut pass, &items, |_, _| 0,
                |_, chunk| {
                    if chunk[0] % 2 == 0 { std::thread::yield_now(); }
                    (chunk.to_vec(), std::thread::current().id())
                }, |(chunk, thread)| {
                    assert!(chunk.len() <= 64);
                    assert_ne!(thread, owner, "Collect must occur on the task pool");
                    observed.extend(chunk);
                });
            assert_eq!(observed, items);
            #[cfg(feature = "cpu-probe")]
            {
                let counts = crate::render::declare_probe::collect_counts_end();
                let tasks = count.div_ceil(64);
                let waves = count.div_ceil(512);
                assert_eq!((counts.tasks, counts.waves), (tasks as u64, waves as u64));
                assert_eq!((counts.cross_pass_tasks, counts.cross_pass_waves), (0, 0));
                assert!(counts.valid && !counts.overflow);
            }
            drop(pass.into_draw_batch());
        }
    }
    #[test]
    fn owner_plan_budget_refusal_stops_workers_and_off_mode_never_rents_plan_storage() {
        use crate::render::pass_builder::draw_stream::test_host::MockHost;
        use std::sync::atomic::{AtomicUsize, Ordering};
        for enabled in [true, false] {
            let (mut host, log) = MockHost::new();
            host.constant_plan_enabled = enabled;
            let budget = host.constant_budget_for_test();
            let usage = budget.cpu_memory_usage().total();
            budget.set_host_budget(usage).unwrap();
            let recorded = host.constant_plans.clone();
            let mut pass = RasterPass::collector(host.boxed(), [1280, 720]);
            let workers = AtomicUsize::new(0);
            #[cfg(feature = "cpu-probe")]
            assert!(crate::render::declare_probe::collect_counts_begin());
            collect_draw_pages(&mut pass, &[1u32; 64], |_, _| 96,
                |_, _| { workers.fetch_add(1, Ordering::Relaxed); }, |_| {});
            pass.render();
            #[cfg(feature = "cpu-probe")]
            {
                let counts = crate::render::declare_probe::collect_counts_end();
                assert_eq!((counts.tasks, counts.waves), if enabled { (0, 0) } else { (1, 1) });
                assert_eq!(counts.valid, !enabled, "admission refusal invalidates without inventing jobs");
            }
            let log = log.borrow();
            if enabled {
                assert_eq!(workers.load(Ordering::Relaxed), 0);
                assert_eq!(log.collect_errors, 1);
                assert_eq!(log.render_pass_cache_calls, 0);
                assert!(log.created_passes.is_empty() && log.submissions.is_empty());
            } else {
                // Empty declaration/Commit needs no backing: the old path is
                // valid under this same cap, so a hidden plan rental would fail.
                assert_eq!(workers.load(Ordering::Relaxed), 1);
                assert_eq!(log.collect_errors, 0);
                assert_eq!(log.created_passes.len(), 1);
                assert_eq!(log.submissions.len(), 1);
            }
            assert!(recorded.lock().unwrap().is_empty());
            budget.set_host_budget(0).unwrap();
        }
    }
    #[test]
    fn task_plans_reuse_single_owner_admission_and_preserve_multiwave_size_offsets() {
        use crate::render::pass_builder::draw_stream::test_host::MockHost;
        for count in [1usize, 65, 513, 4097] {
            let (host, _) = MockHost::new();
            let recorded = host.constant_plans.clone();
            let mut pass = RasterPass::collector(host.boxed(), [1280, 720]);
            let items: Vec<_> = (1..=count).collect();
            let calls = core::cell::Cell::new(0usize);
            collect_draw_pages(&mut pass, &items, |_, item| {
                calls.set(calls.get() + 1);
                *item
            }, |_, chunk| chunk.len(), |_| {});
            assert_eq!(calls.get(), count);
            let mut plans = recorded.lock().unwrap().clone();
            plans.sort_by_key(|plan| plan[0]);
            let expected: Vec<Vec<u32>> = items.chunks(64)
                .map(|chunk| chunk.iter().map(|&size| size as u32).collect()).collect();
            assert_eq!(plans, expected);
            drop(pass.into_draw_batch());
        }
    }
    #[test]
    fn cross_pass_constant_plans_preserve_tagged_slices_across_job_and_wave_boundaries() {
        use crate::render::pass_builder::draw_builder::DrawBuilder;
        use crate::render::pass_builder::draw_stream::{ShaderReflection, test_host::{Log, MockHost}};
        use crate::render::pass_builder::pass_resource_access::SsboType;
        use crate::rhapsody_contract::ManagedHandle;
        use crate::systems::render_loop::draw_passes::draw_lists::{DrawItem, EntityDrawParams};
        use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

        fn draw(builder: DrawBuilder<'_>, _: &EntityDrawParams, _: &World) {
            builder.draw_no_vao(3);
        }

        // A contributes seven tasks. Wave 1's last task starts B; wave 2 must
        // resume B at offset 64 before advancing to C's non-aligned plan_start.
        let lengths = [385usize, 129, 65];
        let tags = [1000u32, 2000, 3000];
        let names = ["plan-a", "plan-b", "plan-c"];
        let sizes: Vec<Vec<u32>> = lengths.iter().zip(tags).map(|(&len, tag)| {
            (0..len).map(|index| tag + index as u32).collect()
        }).collect();
        let inputs: Vec<Vec<DrawEntry>> = sizes.iter().map(|sizes| {
            sizes.iter().enumerate().map(|(index, &size)| {
                (DrawItem::new(u64::from(size), index as i32), draw as _)
            }).collect()
        }).collect();
        let owner_log = Rc::new(RefCell::new(Log::default()));
        let plans = Arc::new(Mutex::new(Vec::<Vec<u32>>::new()));
        let mut factory = || {
            let (mut host, _) = MockHost::new();
            host.log = owner_log.clone();
            host.constant_plans = plans.clone();
            for &size in sizes.iter().flatten() {
                host.shaders.insert(u64::from(size), Arc::new(ShaderReflection {
                    // The plan is an admission hint. No uniform writes are
                    // needed to test its routing through the real Collect path.
                    uniform_block_size: size,
                    ..Default::default()
                }));
            }
            host.rg_buffers.insert(ColorLibrarySsbo::rg_buffer_hash(), 1);
            // bind_common resolves the preset before consulting the shader's
            // binding map, even when this plan-only shader has no uniforms.
            // Match the existing d_main Rig's complete common-bind inputs.
            host.presets.insert(0x11, AttachmentTexture {
                id: 900, extent: (4, 4), format: 44, sample_count: 1,
            });
            host.presets.insert(0x12, AttachmentTexture {
                id: 901, extent: (4, 4), format: 44, sample_count: 1,
            });
            host.boxed()
        };
        let viewport = AttachmentTexture { id: 7, extent: (1280, 720), format: 44, sample_count: 1 };
        let specs = names.map(|name| ListPass {
            name, colors: &[], depth: DepthSpec::None, viewport,
            flag: false, mode: ItemMode::Plain, variant: 0, extra: None,
        });
        let layer: Vec<_> = specs.iter().zip(&inputs).map(|(spec, input)| (spec, input.as_slice())).collect();
        let mut world = World::new();
        let mut ctx = DrawPassCtx::new(&mut world, Duration::ZERO, &mut factory, &NoShaders);
        #[cfg(feature = "cpu-probe")]
        assert!(crate::render::declare_probe::collect_counts_begin());
        run_list_layer(&mut ctx, &layer);
        #[cfg(feature = "cpu-probe")]
        {
            let counts = crate::render::declare_probe::collect_counts_end();
            assert_eq!((counts.tasks, counts.waves), (12, 2));
            assert_eq!((counts.cross_pass_tasks, counts.cross_pass_waves), (12, 2));
            assert!(counts.valid && !counts.overflow);
        }

        let mut actual = plans.lock().unwrap().clone();
        actual.sort_by_key(|plan| plan[0]); // Worker completion order is arbitrary.
        let expected: Vec<Vec<u32>> = sizes.iter().flat_map(|sizes| sizes.chunks(64).map(|chunk| chunk.to_vec())).collect();
        assert_eq!(actual.len(), 12, "must exercise the real cross-pass Collect path");
        assert_eq!(actual, expected, "exact per-task sizes: no cross-pass alias or wrong resumed offset");
        let log = owner_log.borrow();
        assert_eq!(log.raster_reflection_queries, lengths.iter().sum::<usize>(),
            "one owner admission query per draw, with worker queries in their own logs");
        assert_eq!(log.submissions.iter().map(|pass| pass.name).collect::<Vec<_>>(), names);
        for (pass, expected_sizes) in log.submissions.iter().zip(&sizes) {
            assert_eq!(pass.draws.iter().map(|draw| draw.shader.0.clone()).collect::<Vec<_>>(),
                expected_sizes.iter().map(|&size| ManagedHandle(u64::from(size))).collect::<Vec<_>>(),
                "Commit retains pass and draw order independently of plan recording order");
        }
    }

    #[test]
    fn constant_budget_splits_a_page_before_dispatch() {
        use crate::render::pass_builder::draw_stream::test_host::MockHost;
        let (host, _) = MockHost::new();
        let mut pass = RasterPass::collector(host.boxed(), [1280, 720]);
        let mut page_lengths = Vec::new();
        collect_draw_pages(&mut pass, &[1, 2, 3, 4, 5], |_, _| COLLECT_MAX_CONSTANT_BYTES / 2,
            |_, chunk| chunk.len(), |len| page_lengths.push(len));
        assert_eq!(page_lengths, [2, 2, 1]);
        drop(pass.into_draw_batch());
    }
}
