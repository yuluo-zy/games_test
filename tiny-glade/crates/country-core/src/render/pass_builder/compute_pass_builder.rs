//! compute_pass_builder.rs — ComputePass 声明层。
//!
//! 机器码锚点（逐函数反汇编核对，O）：
//! - `ComputePass::new<T>`            0x1405876a0（String 实例）/ 0x140587eb0（&str 实例）
//! - `read_rg_storage_buffer`          0x14096dac0   `write_rg_storage_buffer`  0x14096dd60
//! - `write_rg_storage_buffer_concurrent` 0x14096e000   `bind_external_storage_buffer` 0x14096e2a0
//! - `bind_external_raw`               0x14096e540   `dispatch_indirect_raw`     0x14096e880
//! - `dispatch_1d` 0x14096eaf0 / `dispatch_2d` 0x14096ec80 / `dispatch_groups` 0x14096ee10
//! - `dispatch_with<F>` 0x14160a2e0 / `dispatch_indirect_with<F>` 0x14160b850
//! - `ComputeDispatchApi::dispatch_3d` 0x14096efa0
//! - `impl$3::read_storage_buffer<T>`  0x140588650 起（6 个实例）；`write_storage_buffer<T>` 0x1416044c0 起（12+）
//! - `write_to_cpu_mapped_storage_buffer<T>` 0x141609770
//! - `impl$4::bind_texture_to_compute_pass<A>` 0x140905260（TexturePreset）等 8 个实例
//! - dispatch 渲染回调体 `boxed::call_once<…dispatch_impl::closure_env$0>` 0x14169af70
//!
//! 对象布局（O，`ComputePass` 400 B；方法按值消费 `self`，末尾 `memcpy 0x190` 回传）：
//! `+0x00 bindings: Vec<RenderPassBinding(0x44)>`、`+0x18 access_ranges: Vec<{u32 binding, u8 index}(8)>`、
//! `+0x30 external_bindings: Vec<(BindingIndex, PassResourceSlot)(0x48)>`、`+0x60 PassBuilder`、
//! `+0x110/+0x120/+0x136 UniformBinder`（`+0x136 == 8` 表示无 uniform 块）、`+0x138 &着色器反射（注册表）`、
//! `+0x140 &GraphicsDevice`、`+0x148 PassResources(56 B)`。
//!
//! 注册表（`+0x138` 的 `+0x98` 表）的键是**着色器绑定名哈希**（u64），值 = `(SetIndex, BindingIndex)`；
//! 每个访问都 `assert_eq!(set_index, 0)`；未命中 = 该着色器没有此绑定 → **静默跳过**（连 `add_resource_access`
//! 也不做；`read_*_storage_buffer<T>` 例外：先登记再查注册表，见各函数）。
//!
//! 偏离：rhapsody（`PassBuilder`/`GraphicsDevice`/`DynamicConstants`）以接缝表示（[`RecordingPassBuilder`]、
//! [`ComputePassDevice`]、[`UniformAllocator`]）；`PassBuilder::render(closure)` 改为返回 [`RecordedComputePass`]
//! （原函数返回 `()`，图录制器持有闭包；这里把同一份闭包状态交给调用方）。

// Rust 1.88 enables newer style suggestions; retain original nested branches and modulo expressions.
#![allow(clippy::collapsible_if)]
// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::err_expect, clippy::redundant_slicing, clippy::type_complexity)]
use std::collections::HashMap;
use std::sync::Arc;

use crate::asset_libraries::Handle;
use crate::asset_libraries::shader_library::ComputeShaderSourceHash;
use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::bind_resources::{
    self, BindGameResources, PassResourceSlot as SlotMarker,
};
use crate::render::pass_builder::pass_resource_access::{
    self, ACCESS_CPU_MAPPED_WRITE, ACCESS_INDIRECT_READ, ACCESS_READ, ACCESS_WRITE_CONCURRENT,
    PassBinding, RecordingPassBuilder, ResourceAccess, RgTexture, SYNC_GROUP_DEFAULT,
    SYNC_GROUP_READ, TextureAccess, TextureBinding, TextureCtx, TextureDevice,
    WriteTextureUnchecked, access_is_write, next_binding_index, write_access_buffer,
    write_access_buffer_concurrent,
};
use crate::render::pass_builder::pass_resources::{PassResources, ResourceLookup};
use crate::render::pass_builder::shader_uri::{AsComputeShaderSource, ComputePipelineDesc};
use crate::render::pass_builder::uniform_binder::{
    UniformAllocator, UniformBinder, UniformBlockInfo, UniformBytes, UniformReflection,
};
use crate::render::shader::ComputeShaderProgram;
use crate::render::shader::boundary::ShaderBackend;
use crate::resources::texture_presets::TextureFromDisk;
use crate::resources::texture_presets::TexturePreset;
use crate::rhapsody_contract::{
    self, Extent2D, Handle as RgHandle, ManagedHandle, PassBuilder, ReadOnlyHandle,
    RenderPassBinding, Texture,
};
use crate::rhapsody_types::TextureViewDesc;

/// 资源访问范围记录（8 B：u32 绑定号 + u8 绑定表下标）。
/// 字段名沿用旧版（`resource_index` 实为注册表值里的 **BindingIndex**，O：`mov [..+8*r14], ebx`）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AccessRange {
    pub resource_index: u32,
    pub access_index: u8,
}

/// 着色器反射（`ComputePass+0x138` 指向的 `CompiledShaderPipeline` 反射部分）。
#[derive(Clone, Debug, Default)]
pub struct ResourceRegistry {
    /// `+0x98` 表：名字哈希 → (SetIndex, BindingIndex)。
    pub bindings: HashMap<u64, (u32, u32)>,
    /// `+0xd8`/`+0x128`/`+0x130`：uniform 块（无 → `UniformBinder::new` 返回 `None`）。
    pub uniform_block: Option<UniformBlockInfo>,
    /// `+0xe8`：uniform 字段名哈希 → 偏移。
    pub uniform: UniformReflection,
}

impl ResourceRegistry {
    /// 哈希表查找（O：`cmp [reg+0xb0], 0 → 空表直接未命中`）；返回 `(set_index, binding_index)`。
    pub fn get(&self, hash: u64) -> Option<(u32, u32)> {
        if self.bindings.is_empty() {
            return None;
        }
        self.bindings.get(&hash).copied()
    }
}

/// `rg::resource::Handle<Buffer>`（RG 缓冲句柄）：资源号在 `+0x10`（O：`mov edi, [r9+0x10]`）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RgBuffer {
    pub resource: u32,
}

/// 外部（非 RG）绑定的槽（`PassResourceSlot` 的已观测变体，0x40 B）。
#[derive(Clone, Debug)]
pub enum ExternalSlot {
    /// Immutable original uniform bytes. The current declaration host uploads
    /// the shared table once through its existing DynamicConstants allocator,
    /// imports the actual physical range and keeps that owner through commands.
    /// This source adapter is not an original native enum discriminant.
    UniformBytes(Arc<[u8]>),
    /// 标签 5：`bind_external_storage_buffer` 构造 `{vk 缓冲, offset 0, range !0, 受管句柄克隆, 标志 0}`。
    Buffer {
        vk_buffer: u64,
        offset: u64,
        range: u64,
        handle: ManagedHandle,
        flag: bool,
    },
    /// 由调用方给的其他槽（`bind_external_raw` 原样收入，仅覆写绑定号）。
    Other(rhapsody_contract::PassResourceSlot),
    /// 判别 0（B 阶段补，i_api；O：`bind_texture_to_compute_pass` 受管/磁盘纹理分支 0x14090587d..0x1409058c8 构造的 0x48 B 记录）：
    /// `{binding: u32 @0, 判别 0 @8, sampler 0 @0x10, image_view @0x18, layout 5 @0x20, 句柄克隆 @0x28, 标志 0 @0x40}`。
    /// `layout == 5` 是 `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`（读，故该分支要求 `!access.is_write()`）。
    Image {
        sampler: u64,
        image_view: u64,
        layout: u32,
        handle: ManagedHandle,
        flag: bool,
    },
}

/// `ExternalSlot::Image` 的布局字（O：`mov qword ptr [rbp+0x30], 0x5`）= `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`。
pub const EXTERNAL_IMAGE_LAYOUT_READ_ONLY: u32 = 5;

/// 外部绑定记录（`(BindingIndex, PassResourceSlot)`，0x48 B）。
#[derive(Clone, Debug)]
pub struct ExternalBinding {
    pub binding: u32,
    pub slot: ExternalSlot,
}

/// `GraphicsDevice` 的 compute pass 调用点。
pub trait ComputePassDevice: TextureDevice {
    /// `GraphicsDevice::get_vk_buffer(handle, None)`（O 0x140bef470）；调用点 `Option::unwrap`。
    fn get_vk_buffer(&self, handle: &ManagedHandle) -> Option<u64>;
    /// 已编译管线 `Arc<CompiledShaderPipeline>` 的反射（`ComputePass::new` 里 `[arc+0x10]` 一族偏移）。
    fn pipeline_registry(&self, program: &ComputeShaderProgram) -> Arc<ResourceRegistry>;
}

/// `ComputePass::new` 的外部输入（world 资源、设备、着色器后端、uniform 池）。
pub struct ComputePassInit<'a> {
    /// `PassResources::new_from(world)` 的来源。
    pub world: &'a dyn WorldAccess,
    /// 着色器后端（`ComputeShaderProgram::new` 的 `device` 实参）。
    pub backend: &'a dyn ShaderBackend,
    /// `Mutex<DynamicConstants>` 守卫内容。
    pub uniforms: &'a mut dyn UniformAllocator,
}

/// world 访问：资源查找 + `GraphicsDeviceResource`。
pub trait WorldAccess: ResourceLookup {
    /// `UnsafeWorldCell::get_resource<GraphicsDeviceResource>` + `Deref`；缺失 → `Option::unwrap` panic。
    fn graphics_device(&self) -> Option<Arc<dyn ComputePassDevice>>;
    /// The original PassBuilder carries its enclosing graph (`+0x40`). A
    /// production WorldAccess returns that same live graph facade so library
    /// SSBOs are imported lazily by get_rg_buffer, rather than into a detached
    /// default recorder. Unit-test worlds can keep the CPU recorder default.
    fn resource_graph(&self) -> crate::rhapsody_types::RenderGraph {
        crate::rhapsody_types::RenderGraph::default()
    }
}

/// dispatch 模式（`closure_env_0 +0x50` 的判别，O）。
pub enum DispatchMode {
    /// 0：直接给线程组数（`dispatch_groups`）。
    Groups { x: u32, y: u32, z: u32 },
    /// 1：给线程总数，执行时按 local size 做 `div_ceil`（`dispatch_1d`/`dispatch_2d`，缺省维度 = 1）。
    Threads { x: u32, y: u32, z: u32 },
    /// 2：间接 dispatch，参数缓冲资源号 + 偏移（`dispatch_indirect_raw`）。
    IndirectRaw { buffer: u32, offset: u64 },
    /// 3：用户闭包拿到 `ComputeDispatchApi`（`dispatch_with`）。
    With(Box<dyn FnOnce(&mut dyn ComputeDispatchApi) + Send>),
    /// 4：先登记间接缓冲读访问，再调用户闭包（`dispatch_indirect_with`）。
    IndirectWith {
        buffer: u32,
        f: Box<dyn FnOnce(&mut dyn ComputeDispatchIndirectApi) + Send>,
    },
}

impl DispatchMode {
    /// 判别值（O：0/1/2/3/4）。
    pub fn discriminant(&self) -> u32 {
        match self {
            DispatchMode::Groups { .. } => 0,
            DispatchMode::Threads { .. } => 1,
            DispatchMode::IndirectRaw { .. } => 2,
            DispatchMode::With(_) => 3,
            DispatchMode::IndirectWith { .. } => 4,
        }
    }
}

/// 已声明完毕的 compute pass（原版交给 `PassBuilder::render` 的闭包状态）。
pub struct RecordedComputePass {
    pub name: &'static str,
    pub builder: RecordingPassBuilder,
    pub bindings: Vec<PassBinding>,
    pub access_ranges: Vec<AccessRange>,
    /// Binding-table indices with a proven uniformly disabled texel read.
    pub inactive_texture_bindings: Vec<u8>,
    /// Real reads whose original caller allows undefined initial contents.
    pub undefined_texture_bindings: Vec<u8>,
    /// Host content intent; native access codes and synchronization stay intact.
    pub preserving_buffers: Vec<u32>,
    /// Image binding indices whose writes retain previously produced texels.
    pub preserving_texture_bindings: Vec<u8>,
    pub external_bindings: Vec<ExternalBinding>,
    /// Immutable physical ranges read through shader device addresses.
    pub device_address_reads: Vec<rhapsody::resource::DynamicConstantsChunk>,
    pub uniform_binder: Option<UniformBinder>,
    pub push_constants: Vec<u8>,
    pub program: Option<Handle<ComputeShaderProgram>>,
    pub dispatch: DispatchMode,
}

/// 渲染回调里的命令面（`RenderPassApi` + `GraphicsDevice` 调用点）。
pub trait ComputeExec {
    /// `[device+0xaba]`：是否要求每次 dispatch 前全管线屏障。
    fn full_barrier_enabled(&self) -> bool;
    /// `GraphicsDevice::issue_full_pipeline_barrier`。
    fn issue_full_pipeline_barrier(&mut self);
    /// `lock inc [device+0xaa0]`：dispatch 计数。
    fn count_dispatch(&mut self);
    /// `vkCmdDispatch`（`[device+0x360]`）。
    fn cmd_dispatch(&mut self, x: u32, y: u32, z: u32);
    /// `vkCmdDispatchIndirect`（`[device+0x368]`），缓冲经 `ResourceRegistry::buffer(id)` 取得；缺失 → `None`。
    fn cmd_dispatch_indirect(&mut self, buffer: u32, offset: u64) -> Option<()>;
    /// 管线的 local size（`pipeline[+0x38/+0x3c/+0x40]`）。
    fn local_size(&self) -> [u32; 3];
    /// `RenderPassApi::push_constants`。
    fn push_constants(&mut self, bytes: &[u8]);
}

/// 立即/图执行 dispatch 接口（`ComputeDispatchApi`）。
pub trait ComputeDispatchApi {
    fn dispatch_1d(&mut self, x: u32);
    fn dispatch_2d(&mut self, x: u32, y: u32);
    /// `dispatch_3d`（0x14096efa0）：`threads` 为线程总数。
    fn dispatch_3d(&mut self, args: DispatchArgs);
    fn push_constants(&mut self, bytes: &[u8]);
    fn track_gpu_lifetime(&mut self);
}

/// 间接 dispatch 接口（`dispatch_indirect_*`）。
pub trait ComputeDispatchIndirectApi {
    fn dispatch_indirect(&mut self, byte_offset: u64);
    fn push_constants(&mut self, bytes: &[u8]);
}

/// dispatch 线程数 / 间接参数。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DispatchArgs {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    /// 间接参数缓冲的资源号。
    pub indirect_index: Option<u32>,
}

/// `ComputeDispatchApi::dispatch_3d` 的实现体（0x14096efa0）。
///
/// O：先检查 local size 三项非 0（`test r15d/r13d/ebx; je → panic_const_div_by_zero`，**在屏障之前**）；
/// `[device+0xaba] == 1` 则先发全管线屏障；`ceil(threads / local)` 逐维计算（x/y/z 顺序按寄存器 ecx/ebp/r14d 取参）；
/// `lock inc [device+0xaa0]`；调 `vkCmdDispatch(cmd, gx, gy, gz)`。
pub struct ImmediateDispatch<'a> {
    pub exec: &'a mut dyn ComputeExec,
}

impl ImmediateDispatch<'_> {
    /// 按线程数 dispatch。
    pub fn dispatch_threads(&mut self, threads: [u32; 3]) {
        let local = self.exec.local_size();
        for l in local {
            if l == 0 {
                panic!("attempt to divide by zero");
            }
        }
        if self.exec.full_barrier_enabled() {
            self.exec.issue_full_pipeline_barrier();
        }
        let gx = threads[0].div_ceil(local[0]);
        let gy = threads[1].div_ceil(local[1]);
        let gz = threads[2].div_ceil(local[2]);
        self.exec.count_dispatch();
        self.exec.cmd_dispatch(gx, gy, gz);
    }
}

impl ComputeDispatchApi for ImmediateDispatch<'_> {
    fn dispatch_1d(&mut self, x: u32) {
        self.dispatch_threads([x, 1, 1]);
    }
    fn dispatch_2d(&mut self, x: u32, y: u32) {
        self.dispatch_threads([x, y, 1]);
    }
    fn dispatch_3d(&mut self, args: DispatchArgs) {
        self.dispatch_threads([args.x, args.y, args.z]);
    }
    fn push_constants(&mut self, bytes: &[u8]) {
        self.exec.push_constants(bytes);
    }
    fn track_gpu_lifetime(&mut self) {}
}

/// The indirect callback is bound to the argument buffer declared by the pass.
/// Original callback families use ComputeDispatchIndirectApi, not the ordinary
/// thread-count dispatcher (PDB 14160bdf0 / FnOnce 1415fbc50).
struct ImmediateIndirectDispatch<'a> {
    exec: &'a mut dyn ComputeExec,
    buffer: u32,
}
impl ComputeDispatchIndirectApi for ImmediateIndirectDispatch<'_> {
    fn dispatch_indirect(&mut self, byte_offset: u64) {
        if self.exec.full_barrier_enabled() {
            self.exec.issue_full_pipeline_barrier();
        }
        self.exec.count_dispatch();
        self.exec
            .cmd_dispatch_indirect(self.buffer, byte_offset)
            .expect("Resource does not exist");
    }
    fn push_constants(&mut self, bytes: &[u8]) {
        self.exec.push_constants(bytes);
    }
}

/// 渲染回调里真正执行的部分（push constants + dispatch 模式）。不含构图用的 `RecordingPassBuilder`，
/// 因此是 `Send` 的，可以交给 rhapsody 的并行录制线程。
pub struct ComputeExecution {
    pub push_constants: Vec<u8>,
    pub dispatch: DispatchMode,
}

impl ComputeExecution {
    /// 渲染回调体（0x14169af70，按 `closure_env+0x60` 的判别跳转）：
    /// 非空 push constants 先推送；然后按模式 dispatch。
    ///
    /// 模式 0：`[dev+0xaba]==1` → 屏障；计数 +1；`vkCmdDispatch(x,y,z)`。
    /// 模式 1：三维 local size 逐个除（为 0 → `div_by_zero`），其余同 `dispatch_3d`。
    /// 模式 2：缓冲经 `ResourceRegistry::buffer`（缺失 → `Option::unwrap`，字符串 "Resource does not exist"），
    ///         `[dev+0xaba] != 0` → 屏障；计数 +1；`vkCmdDispatchIndirect`。
    /// 模式 3/4：调用户闭包（`call [vtable+0x18]`），闭包拿到 `ComputeDispatchApi`。
    pub fn execute(self, exec: &mut dyn ComputeExec) {
        if !self.push_constants.is_empty() {
            exec.push_constants(&self.push_constants);
        }
        match self.dispatch {
            DispatchMode::Groups { x, y, z } => {
                if exec.full_barrier_enabled() {
                    exec.issue_full_pipeline_barrier();
                }
                exec.count_dispatch();
                exec.cmd_dispatch(x, y, z);
            }
            DispatchMode::Threads { x, y, z } => {
                ImmediateDispatch { exec }.dispatch_threads([x, y, z])
            }
            DispatchMode::IndirectRaw { buffer, offset } => {
                if exec.full_barrier_enabled() {
                    exec.issue_full_pipeline_barrier();
                }
                exec.count_dispatch();
                exec.cmd_dispatch_indirect(buffer, offset)
                    .expect("Resource does not exist");
            }
            DispatchMode::With(f) => {
                f(&mut ImmediateDispatch { exec });
            }
            DispatchMode::IndirectWith { buffer, f } => {
                f(&mut ImmediateIndirectDispatch { exec, buffer });
            }
        }
    }
}

impl RecordedComputePass {
    /// Attach actual BDA upload allocations, never numeric lifetime tokens.
    /// The production graph imports their physical buffers/ranges and its
    /// existing command owner protocol retains them through GPU completion.
    pub fn with_device_address_reads(
        mut self,
        chunks: Vec<rhapsody::resource::DynamicConstantsChunk>,
    ) -> Self {
        self.device_address_reads.extend(chunks);
        self
    }

    /// 见 [`ComputeExecution::execute`]。
    pub fn execute(self, exec: &mut dyn ComputeExec) {
        assert!(
            self.device_address_reads.is_empty(),
            "BDA upload ranges require the production graph command owner path"
        );
        ComputeExecution {
            push_constants: self.push_constants,
            dispatch: self.dispatch,
        }
        .execute(exec)
    }
}

/// 计算 pass 声明结构。
pub struct ComputePass {
    /// `+0x00`：0x44 B `RenderPassBinding` 表。
    pub(crate) bindings: Vec<PassBinding>,
    /// `+0x18`：访问范围表（8 B/项）。
    pub(crate) access_ranges: Vec<AccessRange>,
    pub(crate) inactive_texture_bindings: Vec<u8>,
    pub(crate) undefined_texture_bindings: Vec<u8>,
    pub(crate) preserving_buffers: Vec<u32>,
    pub(crate) preserving_texture_bindings: Vec<u8>,
    /// `+0x30`：外部绑定表（0x48 B/项）。
    pub(crate) external_bindings: Vec<ExternalBinding>,
    /// `+0x60`：`PassBuilder`。
    pub(crate) builder: RecordingPassBuilder,
    /// `+0x110..+0x136`：uniform 绑定器（`None` ⇔ `+0x136 == 8`）。
    pub(crate) uniform_binder: Option<UniformBinder>,
    /// `+0x138`：着色器反射。
    pub(crate) registry: Arc<ResourceRegistry>,
    /// `+0x140`：设备。
    pub(crate) device: Arc<dyn ComputePassDevice>,
    /// `+0x148`：`PassResources`（56 B）。
    pub(crate) resources: PassResources,
    /// 着色器程序句柄（`FrozenMap<Handle<ComputeShaderProgram>,..>` 的键，`add_compute`/`get_compute_by_name` 的返回）。
    pub(crate) program: Option<Handle<ComputeShaderProgram>>,
    /// push constants 字节（I：回调里 `RenderPassApi::push_constants` 的数据）。
    pub(crate) push_constants: Vec<u8>,
    /// `BindGameResources` 的槽载体（ComputePass 机器码没有槽表，仅为满足 trait）。
    pub(crate) _slot: Option<SlotMarker>,
}

/// dispatch 之前的全部访问（测试/调用方观察用）。
impl ComputePass {
    pub fn bindings(&self) -> &[PassBinding] {
        &self.bindings
    }
    pub fn access_ranges(&self) -> &[AccessRange] {
        &self.access_ranges
    }
    pub fn external_bindings(&self) -> &[ExternalBinding] {
        &self.external_bindings
    }
    pub fn accesses(&self) -> &[ResourceAccess] {
        &self.builder.accesses
    }
    pub fn uniform_binder(&self) -> Option<&UniformBinder> {
        self.uniform_binder.as_ref()
    }
}

impl ComputePass {
    /// `new<T: AsComputeShaderSource>`（O 0x1405876a0 / 0x140587eb0）。
    ///
    /// 1. `hash = calculate_hash(&(source, &()))`（即 `source.source_hash()`）；
    /// 2. `PassResources::new_from(world)`；`get_resource<GraphicsDeviceResource>` 缺失 → `Option::unwrap`；
    /// 3. `AssetShaderLibrary::get_compute_by_name(hash)`：命中复用其管线；未命中 →
    ///    `as_compute_pipeline` → `ComputeShaderProgram::new(name, desc, device)` → `add_compute(program, hash)`；
    /// 4. 建 `PassBuilder`（pass 名 = `name`）；锁 `Mutex<DynamicConstants>`，`UniformBinder::new(device, 反射, dc)`；
    /// 5. 构造 400 B 对象，随即 `bind_color_library::<ComputePass>()`。
    pub fn new<T: AsComputeShaderSource>(
        name: &'static str,
        source: &T,
        init: ComputePassInit<'_>,
    ) -> ComputePass {
        Self::try_new(name, source, init).expect("compute pass creation failed")
    }

    pub fn try_new<T: AsComputeShaderSource>(
        name: &'static str,
        source: &T,
        init: ComputePassInit<'_>,
    ) -> rhapsody::Result<ComputePass> {
        let hash = source.source_hash();
        let resources = PassResources::new_from(init.world);
        let device = init.world.graphics_device().unwrap();
        let key = ComputeShaderSourceHash::from_hash(hash);
        let library = resources.shader_library();

        let (handle, registry) = match library.get_compute_by_name(&key) {
            Some((handle, program)) => (handle, device.pipeline_registry(program)),
            None => {
                let desc = compute_desc(source);
                let program = ComputeShaderProgram::try_new(name, desc, init.backend)?;
                let registry = device.pipeline_registry(&program);
                (library.add_compute(program, key), registry)
            }
        };

        let uniform_binder = UniformBinder::new(registry.uniform_block, init.uniforms);
        let pass = ComputePass {
            bindings: Vec::new(),
            access_ranges: Vec::new(),
            inactive_texture_bindings: Vec::new(),
            undefined_texture_bindings: Vec::new(),
            preserving_buffers: Vec::new(),
            preserving_texture_bindings: Vec::new(),
            external_bindings: Vec::new(),
            builder: RecordingPassBuilder {
                name,
                render_graph: init.world.resource_graph(),
                ..RecordingPassBuilder::default()
            },
            uniform_binder,
            registry,
            device,
            resources,
            program: Some(handle),
            push_constants: Vec::new(),
            _slot: None,
        };
        let mut pass = pass;
        BindGameResources::bind_color_library(&mut pass);
        Ok(pass)
    }

    /// 注册表解析：未命中 `None`；命中则 `assert_eq!(set_index, 0)`（`assert_failed<SetIndex>`）。
    fn resolve(&self, hash: u64) -> Option<u32> {
        let (set_index, binding) = self.registry.get(hash)?;
        assert_eq!(set_index, 0);
        Some(binding)
    }

    /// RG 缓冲访问的共同形体（O：三个 `*_rg_storage_buffer` 仅 access/group 不同）：
    /// 查注册表（未命中直接返回）→ `add_resource_access(buffer, access, group)` → 绑定数检查 →
    /// 追加 `Buffer` 绑定 → 追加 `AccessRange{binding, 绑定下标}`。
    fn declare_rg_buffer(&mut self, hash: u64, buffer: &RgBuffer, access: u8, group: u32) {
        let Some(binding) = self.resolve(hash) else {
            return;
        };
        self.builder
            .add_resource_access(buffer.resource, access, group);
        let index = next_binding_index(&self.bindings);
        self.bindings.push(PassBinding::Buffer {
            resource: buffer.resource,
        });
        self.access_ranges.push(AccessRange {
            resource_index: binding,
            access_index: index,
        });
    }

    /// `read_rg_storage_buffer`（0x14096dac0）：access 0x1a、同步组 1。
    pub fn read_rg_storage_buffer(mut self, hash: u64, buffer: &RgBuffer) -> ComputePass {
        self.declare_rg_buffer(hash, buffer, ACCESS_READ, SYNC_GROUP_READ);
        self
    }

    /// `write_rg_storage_buffer`（0x14096dd60）：access 0x37、同步组 0。
    pub fn write_rg_storage_buffer(mut self, hash: u64, buffer: &RgBuffer) -> ComputePass {
        self.declare_rg_buffer(
            hash,
            buffer,
            pass_resource_access::ACCESS_WRITE,
            SYNC_GROUP_DEFAULT,
        );
        self
    }

    /// `write_rg_storage_buffer_concurrent`（0x14096e000）：access 0x37，同步组由调用方给。
    pub fn write_rg_storage_buffer_concurrent(
        mut self,
        hash: u64,
        buffer: &RgBuffer,
        group: u32,
    ) -> ComputePass {
        self.declare_rg_buffer(
            hash,
            buffer,
            pass_resource_access::ACCESS_WRITE_CONCURRENT,
            group,
        );
        self
    }

    /// `bind_external_storage_buffer`（0x14096e2a0）：未命中静默返回；命中 → `assert_eq!(set, 0)` →
    /// `get_vk_buffer(handle)`（缺失 `Option::unwrap`）→ 追加 `(binding, Buffer{vk, 0, !0, handle.clone(), false})`。
    pub fn bind_external_storage_buffer(
        mut self,
        hash: u64,
        handle: &ManagedHandle,
    ) -> ComputePass {
        let Some(binding) = self.resolve(hash) else {
            return self;
        };
        let vk_buffer = self.device.get_vk_buffer(handle).unwrap();
        self.external_bindings.push(ExternalBinding {
            binding,
            slot: ExternalSlot::Buffer {
                vk_buffer,
                offset: 0,
                range: u64::MAX,
                handle: handle.clone(),
                flag: false,
            },
        });
        self
    }

    /// `bind_external_raw`（0x14096e540）：命中 → `assert_eq!(set, 0)`，收入调用方的槽并覆写绑定号；
    /// 未命中 → 槽被丢弃。
    pub fn bind_external_raw(mut self, hash: u64, slot: ExternalSlot) -> ComputePass {
        if let Some(binding) = self.resolve(hash) {
            self.external_bindings
                .push(ExternalBinding { binding, slot });
        }
        self
    }

    /// 以 SSBO 类型登记的访问（`read_storage_buffer<T>` / `write_storage_buffer<T>` 共同后半）：
    /// 先 `*_access_buffer<T>`（无条件，返回绑定下标），再查注册表命中才追加 `AccessRange`。
    fn push_ssbo_range(&mut self, hash: u64, index: u8) {
        if let Some(binding) = self.resolve(hash) {
            self.access_ranges.push(AccessRange {
                resource_index: binding,
                access_index: index,
            });
        }
    }

    /// `impl$3::read_storage_buffer<T>`（0x140588650 起）：`read_access_buffer<T>(…, access 0x1a)`。
    pub fn read_storage_buffer<T: LibrarySsbo>(mut self, hash: u64) -> ComputePass {
        self.read_ssbo_in_place::<T>(hash);
        self
    }

    /// `write_storage_buffer<T>`（0x1416044c0 起）：`write_access_buffer<T>(…, access 0x32)`。
    pub fn write_storage_buffer<T: LibrarySsbo>(mut self, hash: u64) -> ComputePass {
        let index = write_access_buffer::<T>(
            &self.resources,
            &mut self.builder,
            &mut self.bindings,
            ACCESS_CPU_MAPPED_WRITE,
        );
        self.push_ssbo_range(hash, index);
        self
    }

    /// `write_to_cpu_mapped_storage_buffer<T>`（0x141609770）：与 `write_storage_buffer<T>` 同形（access 0x32）。
    pub fn write_to_cpu_mapped_storage_buffer<T: LibrarySsbo>(self, hash: u64) -> ComputePass {
        self.write_storage_buffer::<T>(hash)
    }

    /// `write_storage_buffer_concurrent<T>`（O 0x141608450，`NaniDrawListSsbo` 实例；B 阶段 i_api 核对：之前以为“无实例”，
    /// 访问码按 `write_storage_buffer` 取 0x32）：`write_access_buffer_concurrent<T>(.., access 0x37, group)`——访问码是
    /// **0x37**（`mov byte ptr [rsp+0x20], 0x37`，同 `write_rg_storage_buffer_concurrent`），同步组由调用方给（`r9d` → `[rsp+0x28]`）。
    pub fn write_storage_buffer_concurrent<T: LibrarySsbo>(
        mut self,
        hash: u64,
        group: u32,
    ) -> ComputePass {
        let index = write_access_buffer_concurrent::<T>(
            &self.resources,
            &mut self.builder,
            &mut self.bindings,
            ACCESS_WRITE_CONCURRENT,
            group,
        );
        self.push_ssbo_range(hash, index);
        self
    }

    /// `impl$3::uniform<T>`（`write_uniform<closure_env<T>, T>` 0x14096b570..）：无 uniform 块时静默跳过。
    pub fn uniform<T: UniformBytes>(mut self, name_hash: u64, value: T) -> ComputePass {
        self.write_uniform_in_place(name_hash, value);
        self
    }

    /// push constants（I：回调里经 `RenderPassApi::push_constants` 推送）。
    pub fn push_constants(mut self, bytes: &[u8]) -> ComputePass {
        self.push_constants = bytes.to_vec();
        self
    }

    /// 维度 uniform（`write_optional_uniform<…bind_texture_to_compute_pass::closure_env$3>`）：
    /// 键 = `BindingNameHash::with_suffix_dims(name)` = `name + 1`（O 0x140c87650：`lea rax,[rcx+1]`）；
    /// 无 uniform 块（`+0x136 == 8`）则跳过；值 = 纹理 extent（8 B）。
    fn write_dims_uniform(&mut self, name_hash: u64, extent: impl FnOnce() -> Extent2D) {
        let key = with_suffix_dims(name_hash);
        if let Some(binder) = self.uniform_binder.as_mut() {
            binder.write_optional_uniform(&self.registry.uniform, key, || {
                let e = extent();
                [e.width, e.height]
            });
        }
    }

    /// Current-host metadata for a bound texture whose configured shader
    /// branch cannot fetch texels. The binding and native access stay intact.
    pub fn inactive_texture_contents(mut self, hash: u64) -> Self {
        let Some(binding) = self.resolve(hash) else {
            return self;
        };
        let range = self
            .access_ranges
            .iter()
            .rev()
            .find(|range| range.resource_index == binding)
            .expect("inactive texture contents requires an existing graph binding");
        assert!(
            matches!(
                self.bindings.get(range.access_index as usize),
                Some(PassBinding::ImageView { write: false, .. })
            ),
            "inactive texture contents requires a non-writing image binding"
        );
        if !self.inactive_texture_bindings.contains(&range.access_index) {
            self.inactive_texture_bindings.push(range.access_index);
        }
        self
    }

    /// A confirmed shader branch performs no fetch from this storage buffer.
    /// Reuse the existing per-binding inactive-content capability; its legacy
    /// `inactive_texture_bindings` field name is retained for API compatibility.
    /// The descriptor, native access, synchronization and owner remain present.
    pub fn inactive_storage_buffer_contents(mut self, hash: u64) -> Self {
        let Some(binding) = self.resolve(hash) else {
            return self;
        };
        let range = self
            .access_ranges
            .iter()
            .rev()
            .find(|range| range.resource_index == binding)
            .expect("inactive storage contents requires an existing graph binding");
        // `read_storage_buffer` 产生 BufferRead；`read_rg_storage_buffer` 产生 Buffer，
        // 此时要求该资源在本 pass 里没有任何写访问。
        let non_writing = match self.bindings.get(range.access_index as usize) {
            Some(PassBinding::BufferRead { .. }) => true,
            Some(PassBinding::Buffer { resource }) => self
                .builder
                .accesses
                .iter()
                .filter(|access| access.resource == *resource)
                .all(|access| !pass_resource_access::access_is_write(access.access)),
            _ => false,
        };
        assert!(
            non_writing,
            "inactive storage contents requires a non-writing buffer binding"
        );
        if !self.inactive_texture_bindings.contains(&range.access_index) {
            self.inactive_texture_bindings.push(range.access_index);
        }
        self
    }

    /// Current-host original-caller contract: the texture is genuinely read
    /// but its initial contents may be undefined. Does not mark it initialized,
    /// remove native binding/sync, or suppress an available producer.
    pub fn allow_undefined_texture_contents(mut self, hash: u64) -> Self {
        let Some(binding) = self.resolve(hash) else {
            return self;
        };
        let range = self
            .access_ranges
            .iter()
            .rev()
            .find(|range| range.resource_index == binding)
            .expect("undefined texture contents requires an existing graph binding");
        assert!(
            matches!(
                self.bindings.get(range.access_index as usize),
                Some(PassBinding::ImageView { write: false, .. })
            ),
            "undefined texture contents requires a non-writing image binding"
        );
        assert!(
            !self.inactive_texture_bindings.contains(&range.access_index),
            "undefined content read is distinct from an inactive binding"
        );
        if !self
            .undefined_texture_bindings
            .contains(&range.access_index)
        {
            self.undefined_texture_bindings.push(range.access_index);
        }
        self
    }

    /// Current-host content intent for a confirmed read/modify operation or a
    /// partial write whose previously produced regions must survive. Call only
    /// after the actual storage write binding has been declared.
    pub fn preserve_storage_buffer_contents(mut self, hash: u64) -> Self {
        let Some(binding) = self.resolve(hash) else {
            return self;
        };
        let range = self
            .access_ranges
            .iter()
            .rev()
            .find(|range| range.resource_index == binding)
            .expect("preserving buffer contents requires a declared binding");
        let resource = match self.bindings.get(range.access_index as usize) {
            Some(PassBinding::Buffer { resource }) => *resource,
            _ => panic!("preserving contents requires a graph buffer binding"),
        };
        assert!(
            self.builder
                .accesses
                .iter()
                .any(|access| access.resource == resource
                    && matches!(
                        access.access,
                        ACCESS_CPU_MAPPED_WRITE | ACCESS_WRITE_CONCURRENT
                    )),
            "preserving contents requires a declared buffer write"
        );
        if !self.preserving_buffers.contains(&resource) {
            self.preserving_buffers.push(resource);
        }
        self
    }

    /// Preserve the actual previous texture contents for a confirmed partial
    /// image write. Uses the exact bound view, never every alias of its owner.
    pub fn preserve_storage_texture_contents(mut self, hash: u64) -> Self {
        let Some(binding) = self.resolve(hash) else {
            return self;
        };
        let range = self
            .access_ranges
            .iter()
            .rev()
            .find(|range| range.resource_index == binding)
            .expect("preserving texture contents requires a declared binding");
        assert!(
            matches!(
                self.bindings.get(range.access_index as usize),
                Some(PassBinding::ImageView { write: true, .. })
            ),
            "preserving texture contents requires a writing image binding"
        );
        if !self
            .preserving_texture_bindings
            .contains(&range.access_index)
        {
            self.preserving_texture_bindings.push(range.access_index);
        }
        self
    }

    /// `bind_texture_to_compute_pass<A>`（0x140905260 等）：无视图版。见 [`ComputePass::bind_texture_with_view`]。
    pub fn bind_texture<A: ComputeTexture>(
        self,
        name_hash: u64,
        texture: &A,
        access: u8,
    ) -> ComputePass {
        self.bind_texture_with_view(name_hash, texture, access, None)
    }

    /// `bind_texture_to_compute_pass<A>`（带视图的 3 标签实例 0x140363db0 / 0x140155400 / 0x1402af750 / 0x1408fcd40 与
    /// `TextureFromDisk` 0x1409056d0；第 5 个实参是 `&Option<TextureViewDesc>`，`None` 的编码是首 dword = 2）。
    ///
    /// O 顺序：
    ///  0. 只读类型的入口守卫（`TextureFromDisk`、`&Handle`、`&ReadOnlyHandle`）：`is_write(access)` →
    ///     `panic!("Trying to bind {:?} for writing, but it's not writable")`；
    ///  1. 注册表未命中（或表空）：只写维度 uniform（范围惰性求 extent），返回；
    ///  2. 命中：`assert_eq!(set, 0)`；`as_texture`：
    ///     * 标签 0（RG）：绑定数检查 → `add_resource_access(res, access, is_write ? 0 : 1)` → `Ref::bind_view(res, 视图)`
    ///       （写 0x140bd6c30 / 读 0x140bd6c00；视图 = 调用方给的描述，`None` → 缺省描述）→ 追加 `AccessRange` → 维度 uniform；
    ///     * 标签 1（受管）/ 2（按名字，先 `get_by_name`）：`assert!(!access.is_write())`（"assertion failed: !access.is_write()"，0x24 字节）→
    ///       视图必须为 `None`（否则 `panic!("TODO?")`）→ `get_vk_image_view` → 追加 [`ExternalSlot::Image`]（**不**登记 RG 访问、
    ///       **不**追加 `AccessRange`）→ 维度 uniform。
    pub fn bind_texture_with_view<A: ComputeTexture>(
        mut self,
        name_hash: u64,
        texture: &A,
        access: u8,
        view: Option<&TextureViewDesc>,
    ) -> ComputePass {
        let write = access_is_write(access);
        if write {
            if let Some(who) = texture.write_guard() {
                panic!("Trying to bind {who} for writing, but it's not writable");
            }
        }
        let device = Arc::clone(&self.device);
        let resources = self.resources;
        let ctx = TextureCtx {
            resources: &resources,
            device: &*device,
        };
        let Some(binding) = self.resolve(name_hash) else {
            self.write_dims_uniform(name_hash, || texture.dims(&ctx));
            return self;
        };
        match texture.target(&ctx) {
            ComputeTarget::Rg { resource, extent } => {
                let index = next_binding_index(&self.bindings);
                let group = if write {
                    SYNC_GROUP_DEFAULT
                } else {
                    SYNC_GROUP_READ
                };
                self.builder.add_resource_access(resource, access, group);
                self.bindings.push(PassBinding::ImageView {
                    resource,
                    write,
                    view: view.copied().unwrap_or_default(),
                });
                self.access_ranges.push(AccessRange {
                    resource_index: binding,
                    access_index: index,
                });
                self.write_dims_uniform(name_hash, || extent);
            }
            ComputeTarget::External {
                view: image_view,
                handle,
                extent,
            } => {
                assert!(!write, "assertion failed: !access.is_write()");
                if view.is_some() {
                    panic!("TODO?");
                }
                self.external_bindings.push(ExternalBinding {
                    binding,
                    slot: ExternalSlot::Image {
                        sampler: 0,
                        image_view,
                        layout: EXTERNAL_IMAGE_LAYOUT_READ_ONLY,
                        handle,
                        flag: false,
                    },
                });
                self.write_dims_uniform(name_hash, || extent);
            }
        }
        self
    }

    // ---- dispatch 家族：返回已声明完毕的 pass（原版把闭包交给 `PassBuilder::render`，返回 `()`）----

    fn finish(self, dispatch: DispatchMode) -> RecordedComputePass {
        RecordedComputePass {
            name: self.builder.name,
            builder: self.builder,
            bindings: self.bindings,
            access_ranges: self.access_ranges,
            inactive_texture_bindings: self.inactive_texture_bindings,
            undefined_texture_bindings: self.undefined_texture_bindings,
            preserving_buffers: self.preserving_buffers,
            preserving_texture_bindings: self.preserving_texture_bindings,
            external_bindings: self.external_bindings,
            device_address_reads: Vec::new(),
            uniform_binder: self.uniform_binder,
            push_constants: self.push_constants,
            program: self.program,
            dispatch,
        }
    }

    /// `dispatch_1d`（0x14096eaf0）：模式 1，`(x, 1, 1)`。
    pub fn dispatch_1d(self, x: u32) -> RecordedComputePass {
        self.finish(DispatchMode::Threads { x, y: 1, z: 1 })
    }

    /// `dispatch_2d`（0x14096ec80）：模式 1，`(x, y, 1)`。
    pub fn dispatch_2d(self, x: u32, y: u32) -> RecordedComputePass {
        self.finish(DispatchMode::Threads { x, y, z: 1 })
    }

    /// `dispatch_groups`（0x14096ee10）：模式 0，线程组数直给。
    pub fn dispatch_groups(self, x: u32, y: u32, z: u32) -> RecordedComputePass {
        self.finish(DispatchMode::Groups { x, y, z })
    }

    /// `dispatch_indirect_raw`（0x14096e880）：`add_resource_access(buffer, 0x02, 1)`，模式 2。
    pub fn dispatch_indirect_raw(mut self, buffer: &RgBuffer, offset: u64) -> RecordedComputePass {
        self.builder
            .add_resource_access(buffer.resource, ACCESS_INDIRECT_READ, SYNC_GROUP_READ);
        self.finish(DispatchMode::IndirectRaw {
            buffer: buffer.resource,
            offset,
        })
    }

    /// `dispatch_with<F>`（0x14160a2e0）：模式 3，闭包装箱（原版 `Box` 0x30 B）。
    pub fn dispatch_with(
        self,
        f: impl FnOnce(&mut dyn ComputeDispatchApi) + Send + 'static,
    ) -> RecordedComputePass {
        self.finish(DispatchMode::With(Box::new(f)))
    }

    /// `dispatch_indirect_with<F>`（0x14160b850）：`add_resource_access(buffer, 0x02, 1)`，模式 4。
    pub fn dispatch_indirect_with(
        mut self,
        buffer: &RgBuffer,
        f: impl FnOnce(&mut dyn ComputeDispatchIndirectApi) + Send + 'static,
    ) -> RecordedComputePass {
        self.builder
            .add_resource_access(buffer.resource, ACCESS_INDIRECT_READ, SYNC_GROUP_READ);
        self.finish(DispatchMode::IndirectWith {
            buffer: buffer.resource,
            f: Box::new(f),
        })
    }
}

/// `BindingNameHash::with_suffix_dims`（O 0x140c87650）：`hash + 1`（不回绕检查：`lea`）。
pub const fn with_suffix_dims(hash: u64) -> u64 {
    hash.wrapping_add(1)
}

/// `ComputeShaderSource` → `ComputePipelineDesc`（`as_compute_pipeline` 的结果）：path + entry + **defines**。
///
/// B 阶段（i_api）：`defines` 取自 [`AsComputeShaderSource::pipeline_defines`]（`PERM_*` 排列宏，如 `TgsrMain` 的
/// `PERM_wave32`/`PERM_sharpen`、`BinLights` 的 `PERM_write`、DoF 五个源的 `PERM_quality`、`GaussBlur`、`GenerateStrokes` 的
/// `PERM_photomode`）。原版 `as_compute_pipeline` 直接返回带 defines 的 `ComputePipelineDesc`，旧版这里恒置空。
fn compute_desc<T: AsComputeShaderSource>(source: &T) -> ComputePipelineDesc {
    let src = source.as_compute_pipeline();
    ComputePipelineDesc {
        path: src.uri.path.to_owned(),
        entry: String::new(),
        defines: source.pipeline_defines(),
    }
    .with_entry(src.uri.entry)
}

/// `bind_texture` 解析纹理后的两种去向（O：`as_texture` 的标签 0 → RG 访问；标签 1/2 → 外部绑定记录）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComputeTarget {
    /// RG 纹理：`add_resource_access` + 视图绑定 + `AccessRange`。
    Rg { resource: u32, extent: Extent2D },
    /// 受管/磁盘纹理：不进 RG 访问表，只追加 `ExternalBinding`（读-only，不得带视图）。
    External {
        view: u64,
        handle: ManagedHandle,
        extent: Extent2D,
    },
}

/// 可绑到 compute pass 的纹理：在 `TextureAccess` 之上给出去向。
///
/// B 阶段（i_api）：`target` / `write_guard` 是新增的**带缺省实现**方法（缺省 = 按 `as_texture` 标签分派：0 → `Rg`，
/// 1 → `External`，2 → 经纹理库 `get_by_name` 解析的 `External`），已有的 `rg_resource` 实现者不受影响；
/// `rg_resource` 本身改为带缺省（从 `target` 推出，`External` → `Option::unwrap` panic）。
pub trait ComputeTexture: TextureAccess {
    /// 入口处的写守卫（O：`TextureFromDisk` 0x1409056d0、`&Handle`/`&ReadOnlyHandle` 0x1408ee240/0x14096f090 在**注册表查找之前**
    /// `is_write(access)` → `panic!("Trying to bind {:?} for writing, but it's not writable")`）。返回 `Some(Debug 文本)` 表示该类型是只读的。
    fn write_guard(&self) -> Option<String> {
        None
    }
    /// 解析纹理去向（缺省：按 `as_texture`）。
    fn target(&self, ctx: &TextureCtx<'_>) -> ComputeTarget {
        match self.as_texture(ctx) {
            TextureBinding::Rg {
                resource, extent, ..
            } => ComputeTarget::Rg { resource, extent },
            TextureBinding::Managed {
                extent,
                view,
                handle,
            } => ComputeTarget::External {
                view,
                handle,
                extent,
            },
            TextureBinding::ByName { name_hash } => resolve_by_name(ctx, name_hash),
        }
    }
    fn rg_resource(&self, ctx: &TextureCtx<'_>) -> u32 {
        match self.target(ctx) {
            ComputeTarget::Rg { resource, .. } => resource,
            // 受管纹理不在 RG 内（经外部绑定路径），无 RG 资源号。
            ComputeTarget::External { .. } => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

/// `as_texture` 标签 2：`AssetTextureLibrary::get_by_name(name)` → `extent_2d` → `get_vk_image_view(tex, 0)`（`Err` → `Result::unwrap`
/// panic）→ `handle.clone()`（O 0x14090584f..0x1409058a8）。占位 `Texture` 的句柄取 `ManagedHandle(id)`（I）。
fn resolve_by_name(ctx: &TextureCtx<'_>, name_hash: u64) -> ComputeTarget {
    let t = ctx
        .resources
        .texture_library()
        .get_by_name(crate::name_hash::NameHash(name_hash));
    let extent = Extent2D {
        width: t.width,
        height: t.height,
    };
    let handle = ManagedHandle(t.id);
    let view = ctx
        .device
        .get_vk_image_view(&handle)
        .unwrap_or_else(|e| panic!("called `Result::unwrap()` on an `Err` value: {e:?}"));
    ComputeTarget::External {
        view,
        handle,
        extent,
    }
}

impl ComputeTexture for TexturePreset {}
impl ComputeTexture for &RgTexture {}
impl ComputeTexture for WriteTextureUnchecked {}
/// 磁盘纹理（O 0x1409056d0）：入口处写守卫（见 [`ComputeTexture::write_guard`]）；`as_texture` 标签 2 → 外部绑定。
impl ComputeTexture for TextureFromDisk {
    fn write_guard(&self) -> Option<String> {
        Some(format!("{self:?}"))
    }
}
/// 原生纹理（O 0x1408fcd40，带视图的 3 标签实例）：标签 1 → 外部绑定。
impl ComputeTexture for &Texture {}
/// RG 句柄（`Handle<Texture>`）：资源号取句柄本身（O：`handle()`）。
/// O（0x1408ee240）：共享引用不可写——入口 `is_write` → panic（`&mut` 版 0x1408edd00 没有该守卫，这里只有 `&`）。
impl ComputeTexture for &RgHandle<Texture> {
    fn write_guard(&self) -> Option<String> {
        Some(format!("{:?}", **self))
    }
    fn target(&self, ctx: &TextureCtx<'_>) -> ComputeTarget {
        ComputeTarget::Rg {
            resource: self.handle(),
            extent: self.dims(ctx),
        }
    }
}
impl ComputeTexture for &ReadOnlyHandle<Texture> {
    fn write_guard(&self) -> Option<String> {
        Some(format!("{:?}", **self))
    }
    fn target(&self, ctx: &TextureCtx<'_>) -> ComputeTarget {
        ComputeTarget::Rg {
            resource: self.handle(),
            extent: self.dims(ctx),
        }
    }
}

/// `BindGameResources`：ComputePass 的实现。`bind_color_library` / `bind_ray_tracing` 在 ComputePass 上
/// 没有槽表（O：DrawBuilder 版才有 `rustc_entry`），这里覆盖默认实现。
impl BindGameResources for ComputePass {
    fn resource_slot(&mut self, _name: &'static str) -> &mut Option<SlotMarker> {
        &mut self._slot
    }
    fn push_access_range(&mut self, range: AccessRange) {
        self.access_ranges.push(range);
    }
    fn read_wall_color_ids(&mut self) {}
    fn as_pass_builder(&mut self) -> PassBuilder<'_, PassResources> {
        PassBuilder::detached()
    }
    fn bindings_placeholder(&mut self) -> Vec<RenderPassBinding> {
        self.bindings.iter().map(PassBinding::to_contract).collect()
    }
    fn resolve_binding(&mut self, hash: u64) -> Option<u32> {
        self.resolve(hash)
    }
    fn bind_color_library(&mut self) {
        bind_resources::bind_color_library_compute(self)
    }
    fn bind_ray_tracing(&mut self) {
        bind_resources::bind_ray_tracing_compute::<
            bind_resources::RtTrianglesSsbo,
            bind_resources::RtTriangleMetaSsbo,
            bind_resources::RtBvhSsbo,
            bind_resources::WallColorIdSsbo,
        >(self)
    }
}

impl ComputePass {
    /// `read_access_buffer<T>`（access 0x1a）+ 注册表命中追加访问范围（`bind_resources` 与 `read_storage_buffer` 共用）。
    pub(crate) fn read_ssbo_in_place<T: LibrarySsbo>(&mut self, hash: u64) {
        let index = pass_resource_access::read_access_buffer::<T>(
            &self.resources,
            &mut self.builder,
            &mut self.bindings,
            ACCESS_READ,
        );
        self.push_ssbo_range(hash, index);
    }

    /// `write_uniform<_, u32>`（无 uniform 块时跳过）。
    pub(crate) fn write_uniform_in_place<T: UniformBytes>(&mut self, name_hash: u64, value: T) {
        if let Some(binder) = self.uniform_binder.as_mut() {
            binder.write_uniform(&self.registry.uniform, name_hash, value);
        }
    }

    pub(crate) fn resources(&self) -> &PassResources {
        &self.resources
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::color_library::ColorLibrarySsbo;
    use crate::asset_libraries::shader_library::AssetShaderLibrary;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::asset_libraries::texture_library::AssetTextureLibrary;
    use crate::name_hash::NameHash;
    use crate::render::pass_builder::bind_resources::*;
    use crate::render::pass_builder::pass_resources::{TypeId128, type_ids};
    use crate::render::pass_builder::uniform_binder::UniformReflection;
    use crate::render::shader::boundary::{
        CompiledShaderPipeline, ComputePipeline, RasterPipeline, RasterPipelineDesc,
        RasterPipelineKey,
    };
    use crate::render::test_device::TestDevice;
    use crate::rhapsody_types::{BackendError, Error, TextureDesc};
    use std::cell::{Cell, RefCell};

    // ---- 着色器反射：绑定名哈希 ----
    const H_BUF_A: u64 = 0xA001;
    const H_BUF_B: u64 = 0xA002;
    const H_BAD_SET: u64 = 0xA0FF;
    const H_TEX: u64 = 0x7E00;
    const H_UNIFORM: u64 = 0x5500;

    fn registry() -> ResourceRegistry {
        let mut r = ResourceRegistry::default();
        for (h, set, binding) in [
            (H_BUF_A, 0, 3),
            (H_BUF_B, 0, 4),
            (H_BAD_SET, 1, 0),
            (H_TEX, 0, 6),
            (COLOR_LIBRARY_BINDING_HASH, 0, 9),
            (RT_TRIANGLES_BINDING_HASH, 0, 10),
            (RT_TRIANGLE_META_BINDING_HASH, 0, 11),
            (RT_BVH_BINDING_HASH, 0, 12),
            (WALL_COLOR_ID_BINDING_HASH, 0, 13),
        ] {
            r.bindings.insert(h, (set, binding));
        }
        r.uniform_block = Some(UniformBlockInfo {
            size: 64,
            set_index: 1,
        });
        r.uniform = UniformReflection {
            valid: true,
            fields: HashMap::new(),
        };
        r.uniform.fields.insert(with_suffix_dims(H_TEX), 8);
        r.uniform.fields.insert(H_UNIFORM, 16);
        r.uniform.fields.insert(RT_TRIANGLES_UNIFORM_HASH, 24);
        r.uniform.fields.insert(RT_BVH_UNIFORM_HASH, 28);
        r
    }

    struct FakeDevice {
        registry: Arc<ResourceRegistry>,
        vk: HashMap<u64, u64>,
    }
    impl TextureDevice for FakeDevice {
        fn get_vk_image_view(&self, h: &ManagedHandle) -> Result<u64, BackendError> {
            Ok(h.0 + 1)
        }
        fn managed_texture_desc(&self, _: &ManagedHandle) -> TextureDesc {
            TextureDesc::new([32, 16, 1], 0, 0)
        }
    }
    impl ComputePassDevice for FakeDevice {
        fn get_vk_buffer(&self, h: &ManagedHandle) -> Option<u64> {
            self.vk.get(&h.0).copied()
        }
        fn pipeline_registry(&self, _p: &ComputeShaderProgram) -> Arc<ResourceRegistry> {
            Arc::clone(&self.registry)
        }
    }

    /// 计数的着色器后端（`ComputeShaderProgram::new` 的外部调用点）。
    #[derive(Default)]
    struct Backend {
        loads: Cell<u32>,
        last_path: RefCell<String>,
        last_defines: RefCell<Vec<(String, String)>>,
        failure: RefCell<Option<rhapsody::Error>>,
    }
    impl ShaderBackend for Backend {
        fn load_compiled_compute(
            &self,
            d: &ComputePipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            self.loads.set(self.loads.get() + 1);
            *self.last_path.borrow_mut() = format!("{}:{}", d.path, d.entry);
            *self.last_defines.borrow_mut() = d.defines.clone();
            Ok(CompiledShaderPipeline(vec![]))
        }
        fn load_compiled_raster(
            &self,
            _: &RasterPipelineDesc,
        ) -> Result<CompiledShaderPipeline, Error> {
            unreachable!()
        }
        fn raster_shader_variants(&self, _: &RasterPipelineDesc) -> [RasterPipelineDesc; 2] {
            unreachable!()
        }
        fn create_compute_pipeline(
            &self,
            _: &str,
            _: Arc<CompiledShaderPipeline>,
        ) -> rhapsody::Result<ComputePipeline> {
            if let Some(error) = self.failure.borrow_mut().take() {
                return Err(error);
            }
            Ok(ComputePipeline { id: 1 })
        }
        fn create_raster_pipeline(
            &self,
            _: Arc<CompiledShaderPipeline>,
            _: &RasterPipelineKey,
        ) -> Result<RasterPipeline, Error> {
            unreachable!()
        }
    }

    struct Pool(Vec<u32>);
    impl UniformAllocator for Pool {
        fn allocate_uniform(&mut self, size: u32) -> Vec<u8> {
            self.0.push(size);
            vec![0xEE; size as usize]
        }
    }

    /// 持有各库（保证裸指针有效）的测试环境。
    struct Env {
        _shaders: Box<AssetShaderLibrary>,
        textures: Box<AssetTextureLibrary>,
        ssbos: Box<AssetSsboLibrary>,
        map: HashMap<TypeId128, *const ()>,
        device: Arc<FakeDevice>,
        backend: Backend,
    }
    impl ResourceLookup for Env {
        fn get_resource_ptr(&self, id: TypeId128) -> Option<*const ()> {
            self.map.get(&id).copied()
        }
    }
    impl WorldAccess for Env {
        fn graphics_device(&self) -> Option<Arc<dyn ComputePassDevice>> {
            Some(self.device.clone() as Arc<dyn ComputePassDevice>)
        }
    }

    fn env_with(registry: ResourceRegistry, vk: &[(u64, u64)]) -> Env {
        let dev = TestDevice(1);
        let shaders = Box::new(AssetShaderLibrary::new());
        let mut textures = Box::new(AssetTextureLibrary::new(&dev));
        let mut ssbos = Box::new(AssetSsboLibrary::new());
        ssbos.add_new::<ColorLibrarySsbo>(&dev);
        ssbos.add_new::<RtTrianglesSsbo>(&dev);
        ssbos.add_new::<RtTriangleMetaSsbo>(&dev);
        ssbos.add_new::<RtBvhSsbo>(&dev);
        ssbos.add_new::<WallColorIdSsbo>(&dev);
        ssbos.get_mut::<RtTrianglesSsbo>().unwrap().instance_count = 17;
        ssbos.get_mut::<RtBvhSsbo>().unwrap().instance_count = 9;
        textures.create_preset_textures_with(&dev, |_| TextureDesc::new([8, 4, 1], 2, 0x25));
        textures.import_texture_presets_to_rg(&mut crate::rhapsody_types::RenderGraph::default());
        let mut map: HashMap<TypeId128, *const ()> = HashMap::new();
        let dummy: &'static u8 = Box::leak(Box::new(0u8));
        for id in [
            type_ids::RESOURCE_E,
            type_ids::RESOURCE_G,
            type_ids::DYNAMIC_CONSTANTS,
        ] {
            map.insert(id, dummy as *const u8 as *const ());
        }
        map.insert(type_ids::SHADER_LIBRARY, &*shaders as *const _ as *const ());
        map.insert(
            type_ids::TEXTURE_LIBRARY,
            &*textures as *const _ as *const (),
        );
        map.insert(type_ids::SSBO_LIBRARY, &*ssbos as *const _ as *const ());
        Env {
            _shaders: shaders,
            textures,
            ssbos,
            map,
            device: Arc::new(FakeDevice {
                registry: Arc::new(registry),
                vk: vk.iter().copied().collect(),
            }),
            backend: Backend::default(),
        }
    }

    fn env() -> Env {
        env_with(registry(), &[(77, 0xBEEF)])
    }

    fn new_pass(env: &Env) -> ComputePass {
        ComputePass::try_new(
            "test_pass",
            &"/shaders/foo.cs.hlsl:main",
            ComputePassInit {
                world: env,
                backend: &env.backend,
                uniforms: &mut Pool(vec![]),
            },
        )
        .unwrap()
    }

    fn rgb(resource: u32) -> RgBuffer {
        RgBuffer { resource }
    }

    // ---------------------------------------------------------------- new

    #[test]
    fn try_pass_does_not_publish_a_program_when_pipeline_admission_fails() {
        for error in [
            rhapsody::Error::Timeout,
            rhapsody::Error::OutOfHostMemory,
            rhapsody::Error::OutOfDeviceMemory,
            rhapsody::Error::DeviceLost,
        ] {
            let expected = core::mem::discriminant(&error);
            let env = env();
            *env.backend.failure.borrow_mut() = Some(error);
            let actual = ComputePass::try_new(
                "failure",
                &"/shaders/foo.cs.hlsl:main",
                ComputePassInit {
                    world: &env,
                    backend: &env.backend,
                    uniforms: &mut Pool(vec![]),
                },
            )
            .err()
            .unwrap();
            assert_eq!(core::mem::discriminant(&actual), expected);
            assert_eq!(env._shaders.compute_shaders.len(), 0);
            let pass = new_pass(&env);
            assert!(pass.program.is_some());
            assert_eq!(env._shaders.compute_shaders.len(), 1);
        }
    }

    #[test]
    fn new_compiles_once_per_source_hash_then_reuses_the_library_program() {
        let env = env();
        let a = new_pass(&env);
        assert_eq!(env.backend.loads.get(), 1);
        assert_eq!(*env.backend.last_path.borrow(), "/shaders/foo.cs.hlsl:main");
        let b = new_pass(&env);
        assert_eq!(
            env.backend.loads.get(),
            1,
            "second pass must hit get_compute_by_name"
        );
        assert_eq!(a.program.unwrap(), b.program.unwrap());
        assert_eq!(a.builder.name, "test_pass");
    }

    #[test]
    fn new_binds_color_library_read_access_and_range() {
        let env = env();
        let pass = new_pass(&env);
        assert_eq!(pass.accesses().len(), 1);
        let a = pass.accesses()[0];
        assert_eq!((a.access, a.group), (0x1a, 1));
        assert!(matches!(pass.bindings(), [PassBinding::BufferRead { .. }]));
        assert_eq!(
            pass.access_ranges(),
            [AccessRange {
                resource_index: 9,
                access_index: 0
            }]
        );
    }

    #[test]
    fn new_without_color_binding_still_registers_access_but_no_range() {
        let mut r = registry();
        r.bindings.remove(&COLOR_LIBRARY_BINDING_HASH);
        let env = env_with(r, &[]);
        let pass = new_pass(&env);
        assert_eq!(pass.accesses().len(), 1);
        assert!(pass.access_ranges().is_empty());
    }

    #[test]
    fn new_with_empty_registry_skips_range_lookup_entirely() {
        let env = env_with(ResourceRegistry::default(), &[]);
        let pass = new_pass(&env);
        assert_eq!(pass.bindings().len(), 1);
        assert!(pass.access_ranges().is_empty());
        assert!(
            pass.uniform_binder().is_none(),
            "no uniform block => UniformBinder::new returns None"
        );
    }

    #[test]
    fn new_allocates_uniform_block_of_reflected_size() {
        let env = env();
        let mut pool = Pool(vec![]);
        let pass = ComputePass::new(
            "p",
            &"/a.cs.hlsl",
            ComputePassInit {
                world: &env,
                backend: &env.backend,
                uniforms: &mut pool,
            },
        );
        assert_eq!(pool.0, [64]);
        assert_eq!(pass.uniform_binder().unwrap().data().len(), 64);
        assert_eq!(
            *env.backend.last_path.borrow(),
            "/a.cs.hlsl:cs_main",
            "default entry cs_main"
        );
    }

    // ---------------------------------------------------------------- rg storage buffers

    #[test]
    fn rg_read_write_concurrent_constants_groups_and_tags() {
        let env = env();
        let pass = new_pass(&env)
            .read_rg_storage_buffer(H_BUF_A, &rgb(100))
            .write_rg_storage_buffer(H_BUF_B, &rgb(101))
            .write_rg_storage_buffer_concurrent(H_BUF_A, &rgb(102), 7);
        let acc: Vec<_> = pass.accesses()[1..]
            .iter()
            .map(|a| (a.resource, a.access, a.group))
            .collect();
        assert_eq!(acc, [(100, 0x1a, 1), (101, 0x37, 0), (102, 0x37, 7)]);
        // 前面还有 bind_color_library 的 1 个绑定 → 下标从 1 起。
        let ranges = &pass.access_ranges()[1..];
        assert_eq!(
            ranges,
            [
                AccessRange {
                    resource_index: 3,
                    access_index: 1
                },
                AccessRange {
                    resource_index: 4,
                    access_index: 2
                },
                AccessRange {
                    resource_index: 3,
                    access_index: 3
                },
            ]
        );
        assert!(
            pass.bindings()[1..].iter().all(|b| b.tag() == 4),
            "all three use tag 4"
        );
    }

    #[test]
    fn explicit_buffer_content_intent_preserves_only_the_named_write() {
        let env = env();
        let recorded = new_pass(&env)
            .write_rg_storage_buffer(H_BUF_A, &rgb(100))
            .write_rg_storage_buffer(H_BUF_B, &rgb(101))
            .preserve_storage_buffer_contents(H_BUF_A)
            .preserve_storage_buffer_contents(H_BUF_A)
            .preserve_storage_buffer_contents(0xDEAD)
            .dispatch_1d(1);
        assert_eq!(recorded.preserving_buffers, [100]);
        assert_eq!(
            recorded
                .builder
                .accesses
                .iter()
                .filter(|a| a.resource == 100 || a.resource == 101)
                .map(|a| (a.resource, a.access))
                .collect::<Vec<_>>(),
            [(100, 0x37), (101, 0x37)]
        );
    }

    #[test]
    #[should_panic(expected = "declared buffer write")]
    fn content_intent_does_not_turn_a_read_into_a_write() {
        let env = env();
        new_pass(&env)
            .read_rg_storage_buffer(H_BUF_A, &rgb(100))
            .preserve_storage_buffer_contents(H_BUF_A);
    }

    #[test]
    fn rg_unknown_hash_is_a_silent_noop() {
        let env = env();
        let pass = new_pass(&env);
        let (nb, na, nr) = (
            pass.bindings().len(),
            pass.accesses().len(),
            pass.access_ranges().len(),
        );
        let pass = pass
            .read_rg_storage_buffer(0xDEAD, &rgb(1))
            .write_rg_storage_buffer(0xBEEF, &rgb(2));
        assert_eq!(
            (
                pass.bindings().len(),
                pass.accesses().len(),
                pass.access_ranges().len()
            ),
            (nb, na, nr)
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn rg_nonzero_set_index_asserts() {
        let env = env();
        let _ = new_pass(&env).read_rg_storage_buffer(H_BAD_SET, &rgb(1));
    }

    #[test]
    #[should_panic(expected = "TryFromIntError")]
    fn rg_binding_table_overflow_unwraps_after_access_registration() {
        let env = env();
        let mut pass = new_pass(&env);
        pass.bindings = vec![PassBinding::Buffer { resource: 0 }; 0x100];
        let _ = pass.read_rg_storage_buffer(H_BUF_A, &rgb(1));
    }

    // ---------------------------------------------------------------- external

    #[test]
    fn bind_external_storage_buffer_builds_tag5_record() {
        let env = env();
        let pass = new_pass(&env).bind_external_storage_buffer(H_BUF_B, &ManagedHandle(77));
        match &pass.external_bindings()[..] {
            [
                ExternalBinding {
                    binding: 4,
                    slot:
                        ExternalSlot::Buffer {
                            vk_buffer,
                            offset,
                            range,
                            handle,
                            flag,
                        },
                },
            ] => {
                assert_eq!(
                    (*vk_buffer, *offset, *range, *flag),
                    (0xBEEF, 0, u64::MAX, false)
                );
                assert_eq!(handle, &ManagedHandle(77));
            }
            other => panic!("{other:?}"),
        }
        let none = new_pass(&env).bind_external_storage_buffer(0xDEAD, &ManagedHandle(5));
        assert!(
            none.external_bindings().is_empty(),
            "unknown name never reaches get_vk_buffer"
        );
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn bind_external_storage_buffer_missing_vk_buffer_unwraps() {
        let env = env();
        let _ = new_pass(&env).bind_external_storage_buffer(H_BUF_B, &ManagedHandle(1));
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn bind_external_storage_buffer_set_index_asserts_before_vk_lookup() {
        let env = env();
        let _ = new_pass(&env).bind_external_storage_buffer(H_BAD_SET, &ManagedHandle(1));
    }

    #[test]
    fn bind_external_raw_overwrites_binding_and_drops_on_miss() {
        let env = env();
        let slot = rhapsody_contract::PassResourceSlot::SampledImage { resource: 5 };
        let pass = new_pass(&env)
            .bind_external_raw(H_BUF_A, ExternalSlot::Other(slot))
            .bind_external_raw(0xDEAD, ExternalSlot::Other(slot));
        assert!(matches!(
            &pass.external_bindings()[..],
            [ExternalBinding {
                binding: 3,
                slot: ExternalSlot::Other(rhapsody_contract::PassResourceSlot::SampledImage {
                    resource: 5
                })
            }]
        ));
    }

    // ---------------------------------------------------------------- ssbo typed access

    #[test]
    fn write_storage_buffer_concurrent_uses_0x37_and_the_callers_group() {
        // O 0x141608450：`mov byte ptr [rsp+0x20], 0x37`、`mov dword ptr [rsp+0x28], r9d`（同步组）。
        let env = env();
        let pass = new_pass(&env).write_storage_buffer_concurrent::<WallColorIdSsbo>(H_BUF_B, 7);
        let a = pass.accesses()[1];
        assert_eq!((a.access, a.group), (0x37, 7));
        assert_eq!(pass.bindings()[1].tag(), 4);
    }

    #[test]
    fn read_and_write_storage_buffer_use_0x1a_and_0x32() {
        let env = env();
        let pass = new_pass(&env)
            .read_storage_buffer::<RtBvhSsbo>(H_BUF_A)
            .write_storage_buffer::<WallColorIdSsbo>(H_BUF_B)
            .write_to_cpu_mapped_storage_buffer::<RtTrianglesSsbo>(0xDEAD);
        let acc: Vec<_> = pass.accesses()[1..]
            .iter()
            .map(|a| (a.access, a.group))
            .collect();
        assert_eq!(acc, [(0x1a, 1), (0x32, 0), (0x32, 0)]);
        let tags: Vec<_> = pass.bindings()[1..].iter().map(|b| b.tag()).collect();
        assert_eq!(tags, [3, 4, 4], "read => tag 3, write => tag 4");
        // 第三个哈希不在注册表：访问/绑定已登记，但没有 AccessRange。
        assert_eq!(pass.access_ranges().len(), 3);
        assert_eq!(
            pass.access_ranges()[1],
            AccessRange {
                resource_index: 3,
                access_index: 1
            }
        );
        assert_eq!(
            pass.access_ranges()[2],
            AccessRange {
                resource_index: 4,
                access_index: 2
            }
        );
    }

    #[test]
    #[should_panic(expected = "Forgot to register it?")]
    fn unregistered_ssbo_panics_inside_get_rg_buffer() {
        #[derive(Clone, Copy)]
        struct Unregistered;
        impl crate::render::ssbo::StorageElement for Unregistered {
            const NAME: &'static str = "Unregistered";
            const BUFFER_LEN: usize = 1;
        }
        impl LibrarySsbo for Unregistered {
            type Storage = crate::render::ssbo::TypedStorageBuffer<Unregistered>;
            fn new_storage_buffer(
                d: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                i: Option<&[u8]>,
            ) -> Self::Storage {
                crate::render::ssbo::TypedStorageBuffer::new_storage_buffer(d, i)
            }
        }
        let env = env();
        let _ = new_pass(&env).read_storage_buffer::<Unregistered>(H_BUF_A);
    }

    #[test]
    fn ray_tracing_reads_four_ssbos_in_order_and_writes_two_uniforms() {
        let env = env();
        let mut pass = new_pass(&env);
        BindGameResources::bind_ray_tracing(&mut pass);
        // color + 4 个 RT/墙色读访问。
        let accesses: Vec<_> = pass
            .accesses()
            .iter()
            .map(|a| (a.access, a.group))
            .collect();
        assert_eq!(accesses, vec![(0x1a, 1); 5]);
        let ranges: Vec<_> = pass
            .access_ranges()
            .iter()
            .map(|r| r.resource_index)
            .collect();
        assert_eq!(ranges, [9, 10, 11, 12, 13]);
        let data = pass.uniform_binder().unwrap().data();
        let tri = u32::from_le_bytes(data[24..28].try_into().unwrap());
        let bvh = u32::from_le_bytes(data[28..32].try_into().unwrap());
        let lib = &env.ssbos;
        assert_eq!(tri, 17);
        assert_ne!(tri, lib.get_buffer::<RtTrianglesSsbo>().unwrap().id as u32);
        assert_eq!(bvh, 9);
        assert_ne!(bvh, lib.get_buffer::<RtBvhSsbo>().unwrap().id as u32);
    }

    // ---------------------------------------------------------------- uniform / textures

    #[test]
    fn uniform_writes_at_reflected_offset() {
        let env = env();
        let pass = new_pass(&env).uniform(H_UNIFORM, 1.5f32);
        assert_eq!(
            &pass.uniform_binder().unwrap().data()[16..20],
            &1.5f32.to_le_bytes()
        );
        // 无 uniform 块：静默。
        let mut r = registry();
        r.uniform_block = None;
        let env2 = env_with(r, &[]);
        let _ = new_pass(&env2).uniform(H_UNIFORM, 1.0f32);
    }

    fn rg_tex(resource: u32, w: u32, h: u32) -> RgTexture {
        RgTexture {
            resource,
            desc: TextureDesc::new([w, h, 1], 0, 0),
        }
    }

    #[test]
    fn bind_texture_read_and_write_groups_views_and_dims_uniform() {
        let env = env();
        let t = rg_tex(55, 640, 480);
        let pass = new_pass(&env).bind_texture(H_TEX, &&t, 0x1a);
        let a = pass.accesses()[1];
        assert_eq!(
            (a.resource, a.access, a.group),
            (55, 0x1a, 1),
            "read => group 1"
        );
        assert_eq!(
            pass.bindings()[1],
            PassBinding::ImageView {
                resource: 55,
                write: false,
                view: TextureViewDesc::default()
            }
        );
        assert_eq!(
            pass.access_ranges()[1],
            AccessRange {
                resource_index: 6,
                access_index: 1
            }
        );
        let d = pass.uniform_binder().unwrap().data();
        assert_eq!(&d[8..12], &640u32.to_le_bytes());
        assert_eq!(&d[12..16], &480u32.to_le_bytes());

        let w = WriteTextureUnchecked(rg_tex(56, 2, 2));
        let pass = new_pass(&env).bind_texture(H_TEX, &w, 0x37);
        let a = pass.accesses()[1];
        assert_eq!(
            (a.resource, a.access, a.group),
            (56, 0x37, 0),
            "write => group 0"
        );
        assert_eq!(
            pass.bindings()[1],
            PassBinding::ImageView {
                resource: 56,
                write: true,
                view: TextureViewDesc::default()
            }
        );
    }

    #[test]
    fn bind_texture_miss_only_writes_dims_uniform_lazily() {
        let env = env();
        // 注册表里没有 0x1234，但 uniform 反射里有它的 `+1` 键。
        let mut r = registry();
        r.uniform.fields.insert(0x1235, 40);
        let env2 = env_with(r, &[]);
        let t = rg_tex(9, 7, 3);
        let pass = new_pass(&env2).bind_texture(0x1234, &&t, 0x1a);
        assert_eq!(
            pass.bindings().len(),
            1,
            "no binding / access registered on a registry miss"
        );
        assert_eq!(pass.accesses().len(), 1);
        let d = pass.uniform_binder().unwrap().data();
        assert_eq!(&d[40..44], &7u32.to_le_bytes());
        assert_eq!(&d[44..48], &3u32.to_le_bytes());

        // 反射里也没有 → 纹理的 dims 闭包根本不会被调用（用会 panic 的纹理证明）。
        struct Bomb;
        impl TextureAccess for Bomb {
            fn as_texture(&self, _: &TextureCtx<'_>) -> TextureBinding {
                panic!("as_texture must not run on a miss")
            }
            fn dims(&self, _: &TextureCtx<'_>) -> Extent2D {
                panic!("dims must not run when the dims uniform is absent")
            }
        }
        impl ComputeTexture for Bomb {
            fn rg_resource(&self, _: &TextureCtx<'_>) -> u32 {
                panic!("rg_resource must not run on a miss")
            }
        }
        let _ = new_pass(&env).bind_texture(0x9999, &Bomb, 0x1a);
    }

    #[test]
    fn with_suffix_dims_is_plus_one() {
        assert_eq!(with_suffix_dims(41), 42);
        assert_eq!(with_suffix_dims(u64::MAX), 0);
    }

    #[test]
    fn bind_texture_preset_resolves_through_the_texture_library() {
        let env = env();
        let preset = TexturePreset(3);
        let pass = new_pass(&env).bind_texture(H_TEX, &preset, 0x1a);
        let expect = env.textures.get_rg_texture(preset).resource as u32;
        assert_eq!(pass.accesses()[1].resource, expect);
        let d = pass.uniform_binder().unwrap().data();
        assert_eq!(&d[8..12], &8u32.to_le_bytes());
        assert_eq!(&d[12..16], &4u32.to_le_bytes());
    }

    #[test]
    fn bind_texture_rg_handle_uses_handle_value() {
        let env = env();
        let h: RgHandle<Texture> = RgHandle(ManagedHandle(31), core::marker::PhantomData);
        let pass = new_pass(&env).bind_texture(H_TEX, &&h, 0x1a);
        assert_eq!(pass.accesses()[1].resource, 31);
        let d = pass.uniform_binder().unwrap().data();
        assert_eq!(
            (&d[8..12], &d[12..16]),
            (&32u32.to_le_bytes()[..], &16u32.to_le_bytes()[..])
        );
    }

    // ---------------------------------------------------------------- B 阶段（i_api）：视图 / 外部纹理 / defines

    fn panic_text(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
        let e = std::panic::catch_unwind(f).err().expect("must panic");
        e.downcast_ref::<String>()
            .cloned()
            .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default()
    }

    fn env_with_disk_texture() -> (Env, u64) {
        let mut e = env();
        let name = NameHash::compute("grass_hm");
        e.textures.add(
            Some("grass_hm".to_owned()),
            Texture {
                id: 900,
                width: 128,
                height: 64,
                format: 0x25,
            },
        );
        (e, name.0)
    }

    #[test]
    fn bind_texture_with_view_stores_the_view_desc_in_the_image_binding() {
        let env = env();
        let t = rg_tex(55, 640, 480);
        let view = TextureViewDesc::new()
            .base_mip_level(2)
            .level_count(Some(1))
            .aspect_mask(4);
        let pass = new_pass(&env).bind_texture_with_view(H_TEX, &&t, 0x37, Some(&view));
        assert_eq!(
            pass.bindings()[1],
            PassBinding::ImageView {
                resource: 55,
                write: true,
                view
            }
        );
        // 视图不影响访问登记：写 → 同步组 0。
        let a = pass.accesses()[1];
        assert_eq!((a.resource, a.access, a.group), (55, 0x37, 0));
        // None → 缺省描述，与无视图版同形。
        let a = new_pass(&env).bind_texture_with_view(H_TEX, &&t, 0x1a, None);
        let b = new_pass(&env).bind_texture(H_TEX, &&t, 0x1a);
        assert_eq!(a.bindings()[1], b.bindings()[1]);
    }

    #[test]
    fn bind_texture_from_disk_is_an_external_image_binding_not_an_rg_access() {
        let (env, name) = env_with_disk_texture();
        let pass = new_pass(&env).bind_texture(H_TEX, &TextureFromDisk(name), 0x1a);
        assert_eq!(
            pass.accesses().len(),
            1,
            "只有 color library 的访问；磁盘纹理不进 RG 访问表"
        );
        assert_eq!(pass.bindings().len(), 1);
        assert_eq!(pass.access_ranges().len(), 1);
        let ext = &pass.external_bindings()[0];
        assert_eq!(ext.binding, 6);
        match &ext.slot {
            ExternalSlot::Image {
                sampler,
                image_view,
                layout,
                handle,
                flag,
            } => {
                assert_eq!((*sampler, *image_view, *layout, *flag), (0, 901, 5, false));
                assert_eq!(*handle, ManagedHandle(900));
            }
            other => panic!("unexpected slot {other:?}"),
        }
        // 维度 uniform：库纹理的 extent。
        let d = pass.uniform_binder().unwrap().data();
        assert_eq!(&d[8..12], &128u32.to_le_bytes());
        assert_eq!(&d[12..16], &64u32.to_le_bytes());
    }

    #[test]
    fn bind_texture_from_disk_registry_miss_only_writes_dims() {
        let (env, name) = env_with_disk_texture();
        let mut r = registry();
        r.uniform.fields.insert(0x1235, 40);
        let env2 = {
            let mut e = env_with(r, &[]);
            e.textures.add(
                Some("grass_hm".to_owned()),
                Texture {
                    id: 900,
                    width: 128,
                    height: 64,
                    format: 0x25,
                },
            );
            e
        };
        let _ = env;
        let pass = new_pass(&env2).bind_texture(0x1234, &TextureFromDisk(name), 0x1a);
        assert!(pass.external_bindings().is_empty());
        let d = pass.uniform_binder().unwrap().data();
        assert_eq!(
            &d[40..44],
            &128u32.to_le_bytes(),
            "extent 惰性取自 get_by_name"
        );
    }

    #[test]
    fn read_only_types_panic_before_the_registry_lookup_when_written() {
        let (env, name) = env_with_disk_texture();
        // TextureFromDisk：连注册表都没查（0x1234 不在表里）就 panic，文案含 `{:?}`。
        let msg = panic_text(std::panic::AssertUnwindSafe(|| {
            let _ = new_pass(&env).bind_texture(0x1234, &TextureFromDisk(name), 0x37);
        }));
        assert!(msg.starts_with("Trying to bind TextureFromDisk("), "{msg}");
        assert!(
            msg.ends_with(" for writing, but it's not writable"),
            "{msg}"
        );
        // `&Handle`（共享引用）同样。
        let h: RgHandle<Texture> = RgHandle(ManagedHandle(31), core::marker::PhantomData);
        let msg = panic_text(std::panic::AssertUnwindSafe(|| {
            let _ = new_pass(&env).bind_texture(0x1234, &&h, 0x37);
        }));
        assert!(msg.starts_with("Trying to bind Handle("), "{msg}");
        // 但 RG 纹理 / 预设可写。
        let _ = new_pass(&env).bind_texture(H_TEX, &TexturePreset(3), 0x37);
    }

    #[test]
    fn external_branch_asserts_read_only_and_rejects_views() {
        let (env, name) = env_with_disk_texture();
        // 原生纹理（标签 1）：写访问不会被入口守卫拦住，而是分支内 `assert!(!access.is_write())`。
        let raw = Texture {
            id: 77,
            width: 32,
            height: 16,
            format: 0x25,
        };
        let msg = panic_text(std::panic::AssertUnwindSafe(|| {
            let _ = new_pass(&env).bind_texture(H_TEX, &&raw, 0x37);
        }));
        assert_eq!(msg, "assertion failed: !access.is_write()");
        // 外部分支带视图 → `panic!("TODO?")`。
        let v = TextureViewDesc::new().base_mip_level(1);
        let msg = panic_text(std::panic::AssertUnwindSafe(|| {
            let _ = new_pass(&env).bind_texture_with_view(H_TEX, &&raw, 0x1a, Some(&v));
        }));
        assert_eq!(msg, "TODO?");
        let msg = panic_text(std::panic::AssertUnwindSafe(|| {
            let _ = new_pass(&env).bind_texture_with_view(
                H_TEX,
                &TextureFromDisk(name),
                0x1a,
                Some(&v),
            );
        }));
        assert_eq!(msg, "TODO?");
        // 原生纹理读：外部绑定，extent 取自纹理本身，view = get_vk_image_view。
        let pass = new_pass(&env).bind_texture(H_TEX, &&raw, 0x1a);
        match &pass.external_bindings()[0].slot {
            ExternalSlot::Image {
                image_view, handle, ..
            } => {
                assert_eq!((*image_view, handle.0), (78, 77));
            }
            other => panic!("unexpected slot {other:?}"),
        }
    }

    /// 带 `PERM_*` 的源（O：`BinLights`、`TgsrMain`…）：defines 必须到达 `ComputeShaderProgram::new`。
    struct PermSource;
    impl AsComputeShaderSource for PermSource {
        fn source_hash(&self) -> u64 {
            0xFEED
        }
        fn as_compute_pipeline(
            &self,
        ) -> crate::render::pass_builder::shader_uri::ComputeShaderSource<'_> {
            crate::render::pass_builder::shader_uri::ComputeShaderSource {
                uri: crate::render::pass_builder::shader_uri::ShaderUri::parse(
                    "/perm.cs.hlsl:cs_main",
                ),
                defs: Default::default(),
            }
        }
        fn pipeline_defines(&self) -> Vec<(String, String)> {
            vec![
                ("PERM_wave32".to_owned(), "1".to_owned()),
                ("PERM_sharpen".to_owned(), "0".to_owned()),
            ]
        }
    }

    #[test]
    fn perm_defines_reach_the_shader_backend_in_order() {
        let env = env();
        let _ = ComputePass::new(
            "perm",
            &PermSource,
            ComputePassInit {
                world: &env,
                backend: &env.backend,
                uniforms: &mut Pool(vec![]),
            },
        );
        assert_eq!(
            *env.backend.last_defines.borrow(),
            vec![
                ("PERM_wave32".to_owned(), "1".to_owned()),
                ("PERM_sharpen".to_owned(), "0".to_owned())
            ]
        );
        // `&str` 源没有 defines。
        let env2 = self::env();
        let _ = new_pass(&env2);
        assert!(env2.backend.last_defines.borrow().is_empty());
    }

    // ---------------------------------------------------------------- dispatch

    #[derive(Default)]
    struct Exec {
        local: [u32; 3],
        barrier: bool,
        log: Vec<String>,
        indirect_ok: bool,
    }
    impl ComputeExec for Exec {
        fn full_barrier_enabled(&self) -> bool {
            self.barrier
        }
        fn issue_full_pipeline_barrier(&mut self) {
            self.log.push("barrier".into());
        }
        fn count_dispatch(&mut self) {
            self.log.push("count".into());
        }
        fn cmd_dispatch(&mut self, x: u32, y: u32, z: u32) {
            self.log.push(format!("dispatch {x} {y} {z}"));
        }
        fn cmd_dispatch_indirect(&mut self, buffer: u32, offset: u64) -> Option<()> {
            self.log.push(format!("indirect {buffer} {offset}"));
            self.indirect_ok.then_some(())
        }
        fn local_size(&self) -> [u32; 3] {
            self.local
        }
        fn push_constants(&mut self, b: &[u8]) {
            self.log.push(format!("push {b:?}"));
        }
    }

    #[test]
    fn dispatch_modes_and_discriminants() {
        let env = env();
        let p = new_pass(&env).dispatch_1d(10);
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads { x: 10, y: 1, z: 1 }
        ));
        assert_eq!(p.dispatch.discriminant(), 1);
        let p = new_pass(&env).dispatch_2d(10, 20);
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads { x: 10, y: 20, z: 1 }
        ));
        let p = new_pass(&env).dispatch_groups(1, 2, 3);
        assert!(matches!(
            p.dispatch,
            DispatchMode::Groups { x: 1, y: 2, z: 3 }
        ));
        assert_eq!(p.dispatch.discriminant(), 0);
        let p = new_pass(&env).dispatch_with(|_| {});
        assert_eq!(p.dispatch.discriminant(), 3);
        assert_eq!(p.name, "test_pass");
    }

    #[test]
    fn indirect_dispatch_registers_0x02_group1_before_recording() {
        let env = env();
        let p = new_pass(&env).dispatch_indirect_raw(&rgb(88), 16);
        let last = *p.builder.accesses.last().unwrap();
        assert_eq!((last.resource, last.access, last.group), (88, 0x02, 1));
        assert_eq!(p.dispatch.discriminant(), 2);
        let p = new_pass(&env).dispatch_indirect_with(&rgb(89), |_| {});
        let last = *p.builder.accesses.last().unwrap();
        assert_eq!((last.resource, last.access, last.group), (89, 0x02, 1));
        assert_eq!(p.dispatch.discriminant(), 4);
    }

    #[test]
    fn execute_threads_divides_by_local_size_rounding_up_with_barrier_first() {
        let env = env();
        let mut e = Exec {
            local: [8, 8, 1],
            barrier: true,
            ..Default::default()
        };
        new_pass(&env).dispatch_2d(100, 17).execute(&mut e);
        assert_eq!(e.log, ["barrier", "count", "dispatch 13 3 1"]);
    }

    #[test]
    fn execute_groups_skips_division_and_pushes_constants_first() {
        let env = env();
        let mut e = Exec {
            local: [0, 0, 0],
            barrier: false,
            ..Default::default()
        };
        new_pass(&env)
            .push_constants(&[1, 2])
            .dispatch_groups(4, 5, 6)
            .execute(&mut e);
        assert_eq!(e.log, ["push [1, 2]", "count", "dispatch 4 5 6"]);
    }

    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn execute_threads_with_zero_local_size_panics_before_the_barrier() {
        let env = env();
        let mut e = Exec {
            local: [8, 0, 1],
            barrier: true,
            ..Default::default()
        };
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            new_pass(&env).dispatch_1d(8).execute(&mut e)
        }));
        assert!(
            e.log.is_empty(),
            "no barrier/dispatch may be issued: {:?}",
            e.log
        );
        std::panic::resume_unwind(r.unwrap_err());
    }

    #[test]
    fn execute_indirect_and_closure_modes() {
        let env = env();
        let mut e = Exec {
            barrier: true,
            indirect_ok: true,
            local: [4, 4, 4],
            ..Default::default()
        };
        new_pass(&env)
            .dispatch_indirect_raw(&rgb(5), 32)
            .execute(&mut e);
        assert_eq!(e.log, ["barrier", "count", "indirect 5 32"]);

        let mut e = Exec {
            local: [4, 4, 4],
            ..Default::default()
        };
        new_pass(&env)
            .dispatch_with(|api| {
                api.dispatch_1d(9);
                api.dispatch_3d(DispatchArgs {
                    x: 8,
                    y: 1,
                    z: 5,
                    indirect_index: None,
                });
            })
            .execute(&mut e);
        assert_eq!(
            e.log,
            ["count", "dispatch 3 1 1", "count", "dispatch 2 1 2"]
        );
    }

    #[test]
    fn indirect_callback_uses_declared_buffer_and_each_native_byte_offset() {
        let env = env();
        let mut exec = Exec {
            barrier: true,
            indirect_ok: true,
            ..Default::default()
        };
        new_pass(&env)
            .dispatch_indirect_with(&rgb(71), |api| {
                api.push_constants(&[1, 2, 3, 4]);
                api.dispatch_indirect(0x6c0);
                api.push_constants(&[9, 8, 7, 6]);
                api.dispatch_indirect(0x6d0);
            })
            .execute(&mut exec);
        assert_eq!(
            exec.log,
            [
                "push [1, 2, 3, 4]",
                "barrier",
                "count",
                "indirect 71 1728",
                "push [9, 8, 7, 6]",
                "barrier",
                "count",
                "indirect 71 1744",
            ]
        );
    }

    #[test]
    #[should_panic(expected = "Resource does not exist")]
    fn execute_indirect_with_missing_buffer_panics() {
        let env = env();
        let mut e = Exec {
            indirect_ok: false,
            ..Default::default()
        };
        new_pass(&env)
            .dispatch_indirect_raw(&rgb(5), 0)
            .execute(&mut e);
    }

    #[test]
    fn world_without_graphics_device_resource_unwraps() {
        struct NoDev(Env);
        impl ResourceLookup for NoDev {
            fn get_resource_ptr(&self, id: TypeId128) -> Option<*const ()> {
                self.0.get_resource_ptr(id)
            }
        }
        impl WorldAccess for NoDev {
            fn graphics_device(&self) -> Option<Arc<dyn ComputePassDevice>> {
                None
            }
        }
        let w = NoDev(env());
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ComputePass::new(
                "p",
                &"/a.cs.hlsl",
                ComputePassInit {
                    world: &w,
                    backend: &w.0.backend,
                    uniforms: &mut Pool(vec![]),
                },
            )
        }));
        let msg = r
            .err()
            .unwrap()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .unwrap_or_default();
        assert!(msg.contains("Option::unwrap"), "{msg}");
        assert_eq!(
            w.0.backend.loads.get(),
            0,
            "device is fetched before any shader work"
        );
    }
}
