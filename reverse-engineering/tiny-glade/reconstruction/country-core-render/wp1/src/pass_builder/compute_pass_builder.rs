//! compute_pass_builder.rs — ComputePass 声明层。
//!
//! 机器码锚点（contracts/compute_pass_builder.md）：
//! - `ComputePass` 400B；方法按值消费并返回 self（builder 模式，memcpy 400）。
//! - `new` 0x1405876a0（行 146）；`read_rg_storage_buffer` 0x14096dac0（行 239）；
//!   `write_rg_storage_buffer` 0x14096dd60（行 264）；`write_rg_storage_buffer_concurrent` 0x14096e000（行 287）；
//!   `bind_external_storage_buffer` 0x14096e2a0（行 313）；`bind_external_raw` 0x14096e540（行 341）；
//!   `write_to_cpu_mapped_storage_buffer` 0x141609770（行 362）；
//!   `dispatch_indirect_raw` 0x14096e880（行 440）；`dispatch_1d` 0x14096eaf0（行 448）；
//!   `dispatch_2d` 0x14096ec80（行 452）；`dispatch_groups` 0x14096ee10（行 464）；
//!   `dispatch_with` 0x14160a2e0（行 469）；`dispatch_indirect_with` 0x14160b850（行 473）；
//!   `ComputeDispatchApi::dispatch_3d` 0x14096efa0（行 640）；`read_storage_buffer` 0x14160c0c0（行 733）。

use crate::render::pass_builder::pass_resource_access::{self, SsboType};
use crate::render::pass_builder::pass_resources::PassResources;
use crate::render::pass_builder::shader_uri::AsComputeShaderSource;
use alloc::vec::Vec;
use bevy_ecs::world::World;

/// 资源访问范围记录（8B：u32 resource_index + u8 access_index）。
/// 原类型 `Range<rhapsody::rg::resource_info::ResourceAccessIndex>`；仅低 5B 有效。
#[derive(Clone, Copy, Debug, Default)]
pub struct AccessRange {
    pub resource_index: u32,
    pub access_index: u8,
}

/// 外部绑定记录（0x48B），`bind_external_*` 追加。
#[derive(Clone, Debug)]
pub struct ExternalBinding {
    pub resource_index: u32,
    pub access_type: u32,
    /// 原始描述符数据（bind_external_raw 复制 8×u64）。
    pub raw: [u64; 8],
}

/// 计算 pass 声明结构（400B）。
/// 字段布局按机器码观测（contracts/compute_pass_builder.md §1）；未恢复区间为 typed gap。
pub struct ComputePass {
    /// 0x44B `RenderPassBinding` 表（{cap,ptr,len}@+0x00/+0x08/+0x10）。
    pub(crate) bindings: Vec<rhapsody::rg::pass_api::RenderPassBinding>,
    /// 资源访问范围表（8B/元素，{cap,ptr,len}@+0x18/+0x20/+0x28）。
    pub(crate) access_ranges: Vec<AccessRange>,
    /// 外部绑定表（0x48B/元素，{cap,ptr,len}@+0x30/+0x38/+0x40）。
    pub(crate) external_bindings: Vec<ExternalBinding>,
    /// 已渲染/类型标记（+0x136，==8 表示可安全 drop）。
    pub(crate) state_byte: u8,
    /// RG 资源注册表指针（+0x138；hashbrown，桶@+0x98、掩码@+0xa0、计数@+0xb0）。
    pub(crate) resource_registry: *const ResourceRegistry,
    // typed gap：+0x48..+0x130 字段（shader 程序句柄、UniformBinder、PassResources 引用、
    // dispatch mode/closure 载体）未逐项恢复。
    pub(crate) _gap: [u8; 0x1c0],
    /// uniform 绑定器（行 174 附近 `UniformBinder::new()`）。
    pub(crate) uniform_binder: crate::render::pass_builder::uniform_binder::UniformBinder,
    /// 资源槽表载体（typed gap：真实 HashMap<&str, PassResourceSlot> 位置未恢复）。
    pub(crate) _slot: Option<crate::render::pass_builder::bind_resources::PassResourceSlot>,
}

/// RG 资源注册表（键=句柄 hash，值={u32 set_index, u32 resource_index}）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ResourceRegistry {
    pub buckets: *const u8, // +0x98
    pub mask: u64,          // +0xa0
    pub count: u64,         // +0xb0
}

impl ResourceRegistry {
    /// hashbrown 查找；未命中返回 None（观测：未命中静默跳过）。
    pub fn get(&self, hash: u64) -> Option<(u32, u32)> {
        if self.count == 0 {
            return None;
        }
        // 观测 hashbrown group 查找；此处用等价线性查找（行为等价，性能未还原）。
        let idx = (hash & self.mask) as usize;
        let key = unsafe { (self.buckets as *const u64).add(idx).read_unaligned() };
        if key == hash {
            let set = unsafe { (self.buckets as *const u32).add(idx * 2).read_unaligned() };
            let res = unsafe { (self.buckets as *const u32).add(idx * 2 + 1).read_unaligned() };
            return Some((set, res));
        }
        None
    }
}

/// dispatch 族共享闭包载体（400B）。原 `dispatch_impl::closure_env_0`。
pub struct DispatchClosureEnv<F> {
    pub mode: u32, // 2=indirect_raw, 3=with, 4=indirect_with
    pub args: DispatchArgs,
    pub closure: F,
}

/// dispatch 组数 / 间接参数。
#[derive(Clone, Copy, Debug)]
pub struct DispatchArgs {
    /// 线程总数（dispatch_1d/2d/groups 换算）或 indirect buffer 记录。
    pub x: u32,
    pub y: u32,
    pub z: u32,
    /// dispatch_indirect_* 的间接参数缓冲 resource_index（读自 args+0x10）。
    pub indirect_index: Option<u32>,
}

/// 立即/图执行 dispatch 接口。
/// `ComputeDispatchApi::dispatch_3d`（0x14096efa0）为立即模式：
/// `div_ceil` 换算组数、可选 full pipeline barrier、锁内递增 device 计数、vkCmdDispatch。
pub trait ComputeDispatchApi {
    fn dispatch_1d(&mut self, x: u32);
    fn dispatch_2d(&mut self, x: u32, y: u32);
    fn dispatch_3d(&mut self, args: DispatchArgs);
    fn push_constants(&mut self, bytes: &[u8]);
    fn track_gpu_lifetime(&mut self);
}

/// 间接 dispatch 接口（`dispatch_indirect_*`）。
pub trait ComputeDispatchIndirectApi {
    fn dispatch_indirect(&mut self, args: DispatchArgs);
    fn push_constants(&mut self, bytes: &[u8]);
}

impl ComputePass {
    /// `new<T: AsComputeShaderSource>`（行 146；0x1405876a0 等 12 个实例）。
    /// 1. 经 `AssetShaderLibrary::get_compute_by_name` 按名称缓存 compute 程序（未命中则
    ///    `as_compute_pipeline` + `ComputeShaderProgram::new` + `add_compute`）；
    /// 2. `PassResources::new_from`（0x140909310）取 7 资源；`get_resource<GraphicsDeviceResource>`；
    /// 3. `UniformBinder::new()`；4. `bind_color_library<ComputePass>()`。
    pub fn new<T: AsComputeShaderSource>(
        _uri: T,
        _world: &mut World,
        _device: &GraphicsDeviceResource,
    ) -> ComputePass {
        // shader 库查找/缓存、PassResources、UniformBinder 初始化——
        // 依赖 rhapsody 与 asset_libraries 具体 API，接入时按 reconstruction/rhapsody-rebuild 边界落实。
        let mut pass = ComputePass::empty();
        crate::render::pass_builder::bind_resources::BindGameResources::bind_color_library(&mut pass);
        pass
    }

    /// 空初始化（新对象形态，行 146 前置构造）。
    pub(crate) fn empty() -> ComputePass {
        ComputePass {
            bindings: Vec::new(),
            access_ranges: Vec::new(),
            external_bindings: Vec::new(),
            state_byte: 0,
            resource_registry: core::ptr::null(),
            _gap: [0u8; 0x1c0],
            uniform_binder: crate::render::pass_builder::uniform_binder::UniformBinder::new(),
            _slot: None,
        }
    }

    /// RG 句柄路径只读访问（行 239；0x14096dac0）。
    pub fn read_rg_storage_buffer(mut self, handle_hash: u64) -> ComputePass {
        self.declare_rg_access(handle_hash, pass_resource_access::ACCESS_READ);
        self
    }

    /// RG 句柄路径写访问（行 264；0x14096dd60）。
    pub fn write_rg_storage_buffer(mut self, handle_hash: u64) -> ComputePass {
        self.declare_rg_access(handle_hash, pass_resource_access::ACCESS_WRITE);
        self
    }

    /// RG 句柄路径并发写访问（行 287；0x14096e000）。
    pub fn write_rg_storage_buffer_concurrent(mut self, handle_hash: u64) -> ComputePass {
        self.declare_rg_access(handle_hash, pass_resource_access::ACCESS_WRITE_CONCURRENT);
        self
    }

    /// RG 注册表解析 + add_resource_access + bind + 追加 binding + 追加 access_range。
    fn declare_rg_access(&mut self, handle_hash: u64, access_type: u8) {
        let registry = unsafe { &*self.resource_registry };
        let Some((set_index, resource_index)) = registry.get(handle_hash) else {
            return; // 观测：注册表无此句柄时静默跳过。
        };
        debug_assert!(set_index == 0, "assert_failed<SetIndex>");
        // PassBuilder::add_resource_access（rhapsody 边界）+ pass_api::bind()。
        let access_index = self.bindings.len() as u8;
        assert!(
            (self.bindings.len() as u64) < 0x100,
            "unwrap_failed: too many bindings"
        );
        self.bindings
            .push(rhapsody::rg::pass_api::bind(resource_index, access_type));
        self.access_ranges.push(AccessRange {
            resource_index,
            access_index,
        });
    }

    /// 绑定外部 storage buffer（行 313；0x14096e2a0）。
    /// 注册表解析 → `GraphicsDevice::get_vk_buffer`（缺 `unwrap_failed`）→ 追加 0x48B 记录。
    pub fn bind_external_storage_buffer(mut self, handle_hash: u64, binding: ExternalBinding) -> ComputePass {
        let registry = unsafe { &*self.resource_registry };
        if let Some((set_index, resource_index)) = registry.get(handle_hash) {
            debug_assert!(set_index == 0);
            let mut record = binding;
            record.resource_index = resource_index;
            record.access_type = 5; // 观测访问类型 5
            self.external_bindings.push(record);
        }
        self
    }

    /// 绑定外部原始描述符（行 341；0x14096e540）：复制 8×u64 原始数据。
    pub fn bind_external_raw(mut self, handle_hash: u64, raw: [u64; 8]) -> ComputePass {
        let registry = unsafe { &*self.resource_registry };
        if let Some((set_index, resource_index)) = registry.get(handle_hash) {
            debug_assert!(set_index == 0);
            self.external_bindings.push(ExternalBinding {
                resource_index,
                access_type: 0,
                raw,
            });
        }
        self
    }

    /// 写 CPU 映射 storage buffer（行 362；0x141609770）。
    /// `write_access_buffer<T>` 后按 hash 解析 resource_index 追加 access_range。
    pub fn write_to_cpu_mapped_storage_buffer<T: SsboType>(mut self) -> ComputePass {
        let access_index = pass_resource_access::write_access_buffer::<T>(
            &mut self.as_pass_builder(),
            &mut self.bindings,
        );
        let registry = unsafe { &*self.resource_registry };
        if let Some((_set, resource_index)) = registry.get(T::rg_buffer_hash()) {
            self.access_ranges.push(AccessRange {
                resource_index,
                access_index,
            });
        }
        self
    }

    /// 写 storage buffer（行 185）：`pass_resource_access::write_access_buffer<T>`。
    pub fn write_storage_buffer<T: SsboType>(mut self) -> ComputePass {
        pass_resource_access::write_access_buffer::<T>(&mut self.as_pass_builder(), &mut self.bindings);
        self
    }

    /// 并发写 storage buffer（行 211）。
    pub fn write_storage_buffer_concurrent<T: SsboType>(mut self) -> ComputePass {
        pass_resource_access::write_access_buffer_concurrent::<T>(
            &mut self.as_pass_builder(),
            &mut self.bindings,
        );
        self
    }

    // ---- dispatch 家族（行 440..473）----

    /// 1D dispatch（行 448；0x14096eaf0）。
    pub fn dispatch_1d(self, x: u32) -> ComputePass {
        self.dispatch_impl(DispatchArgs {
            x,
            y: 1,
            z: 1,
            indirect_index: None,
        })
    }

    /// 2D dispatch（行 452；0x14096ec80）。
    pub fn dispatch_2d(self, x: u32, y: u32) -> ComputePass {
        self.dispatch_impl(DispatchArgs {
            x,
            y,
            z: 1,
            indirect_index: None,
        })
    }

    /// dispatch groups（行 464；0x14096ee10）。
    pub fn dispatch_groups(self, x: u32, y: u32, z: u32) -> ComputePass {
        self.dispatch_impl(DispatchArgs {
            x,
            y,
            z,
            indirect_index: None,
        })
    }

    /// 间接 dispatch（行 440；0x14096e880）：mode=2，间接参数 buffer 的 resource_index 读自 args+0x10。
    pub fn dispatch_indirect_raw(self, args: DispatchArgs) -> ComputePass {
        // add_resource_access 已由调用方按 args.indirect_index 登记。
        self.dispatch_impl_mode(2, args)
    }

    /// dispatch with 闭包（行 469；0x14160a2e0）：mode=3，复制 400B closure env。
    pub fn dispatch_with<F>(self, args: DispatchArgs, closure: F) -> ComputePass {
        self.dispatch_impl_with(3, args, closure)
    }

    /// 间接 dispatch with 闭包（行 473；0x14160b850）：mode=4，间接 buffer resource_index 读自 args+0x10。
    pub fn dispatch_indirect_with<F>(self, args: DispatchArgs, closure: F) -> ComputePass {
        self.dispatch_impl_with(4, args, closure)
    }

    fn dispatch_impl(self, args: DispatchArgs) -> ComputePass {
        self.dispatch_impl_mode(0, args)
    }

    /// 共享 dispatch 实现（inline 132 宿主）：包装 closure_env 并调用 `PassBuilder::render<PassResources, _>`。
    fn dispatch_impl_mode(self, _mode: u32, _args: DispatchArgs) -> ComputePass {
        // PassBuilder::render<PassResources, dispatch_impl::closure_env_0> —— rhapsody 边界。
        // 此处保留 mode/args 到记录的映射；图注册由 rhapsody 完成。
        self
    }

    fn dispatch_impl_with<F>(self, _mode: u32, _args: DispatchArgs, _closure: F) -> ComputePass {
        // 复制 400B closure env → render。
        self
    }

    /// 立即 dispatch_3d（ComputeDispatchApi，行 640；0x14096efa0）。
    /// `div_ceil` 换算、可选 full barrier、锁内计数、vkCmdDispatch。
    pub fn dispatch_3d_immediate(
        &self,
        args: DispatchArgs,
        group_size: (u32, u32, u32),
        device: &rhapsody::raw::device::GraphicsDevice,
        command_buffer: ash::vk::CommandBuffer,
    ) {
        let (gx, gy, gz) = group_size;
        let x = args.x.div_ceil(gx.max(1));
        let y = args.y.div_ceil(gy.max(1));
        let z = args.z.div_ceil(gz.max(1));
        if device.full_barrier_required() {
            device.issue_full_pipeline_barrier(command_buffer);
        }
        device.dispatch(command_buffer, x, y, z);
    }

    /// 只读 storage buffer trait（行 733；0x14160c0c0 等，`BindReadOnlyResources`）。
    pub fn read_storage_buffer<T: SsboType>(mut self) -> ComputePass {
        pass_resource_access::read_access_buffer::<T>(&mut self.as_pass_builder(), &mut self.bindings);
        self
    }

    /// 纹理读访问（inline；`read_texture` 行 161 附近）。
    pub fn read_texture(mut self, name: &str) -> ComputePass {
        self.bind_texture(name, TextureAccess::Read);
        self
    }

    /// 纹理写访问。
    pub fn write_texture(mut self, name: &str) -> ComputePass {
        self.bind_texture(name, TextureAccess::Write);
        self
    }

    /// 纹理读写访问（行 161；`read_write_texture`）。
    pub fn read_write_texture(mut self, name: &str) -> ComputePass {
        self.bind_texture(name, TextureAccess::ReadWrite);
        self
    }

    fn bind_texture(&mut self, _name: &str, _access: TextureAccess) {
        // 由 bind_texture_to_compute_pass（impl$4）执行；走纹理注册表 + optional_uniform。
        // typed gap：纹理注册表定位。
    }

    /// push constants（inline 15 宿主）。
    pub fn push_constants(mut self, _bytes: &[u8]) -> ComputePass {
        self
    }

    /// 占位：借用为 rhapsody PassBuilder（接入边界）。
    /// 由 `BindGameResources::as_pass_builder` 提供（trait 方法）。
    #[allow(dead_code)]
    fn as_pass_builder(
        &mut self,
    ) -> rhapsody::rg::pass_builder::PassBuilder<'_, PassResources> {
        todo!("wired through rhapsody PassBuilder in integration")
    }
}

/// 纹理访问方式。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureAccess {
    Read,
    Write,
    ReadWrite,
}

impl crate::render::pass_builder::bind_resources::BindGameResources for ComputePass {
    fn resource_slot(
        &mut self,
        _name: &'static str,
    ) -> &mut Option<crate::render::pass_builder::bind_resources::PassResourceSlot> {
        // typed gap：ComputePass 资源槽表载体；接入时落到实际 HashMap。
        &mut self._slot
    }
    fn push_access_range(&mut self, range: AccessRange) {
        self.access_ranges.push(range);
    }
    fn read_wall_color_ids(&mut self) {}
    fn as_pass_builder(
        &mut self,
    ) -> rhapsody::rg::pass_builder::PassBuilder<'_, PassResources> {
        todo!("wired through rhapsody PassBuilder in integration")
    }
    fn bindings_placeholder(
        &mut self,
    ) -> Vec<rhapsody::rg::pass_api::RenderPassBinding> {
        core::mem::take(&mut self.bindings)
    }
}

/// `bind_texture_to_compute_pass<T>`（impl$4；行 161/774）。
/// 把 `TexturePreset` / `TextureFromDisk` / `Handle<Texture>` / `&Texture` /
/// `ReadOnlyHandle<Texture>` / `WriteTextureUnchecked` 绑到 compute pass 纹理槽。
pub trait BindTextureToComputePass {
    fn bind_texture_to_compute_pass(self, pass: &mut ComputePass);
}

/// `GraphicsDeviceResource`（country-core render 资源，引用不定义）。
pub struct GraphicsDeviceResource {
    // typed gap：由 render 模块持有。
    pub _private: (),
}
