//! wave3 覆盖测试的公共装置：校验层实例、在任意设备上构造三角形 Rig、24+ pass 大图（填充 + 跨块拷贝）。
#![allow(dead_code)]
use crate::draw_support::{self, Rig};
use ash::vk::{self, Handle};
use rhapsody::{
    raw::{
        buffer::{Buffer, BufferDesc},
        command_buffer::VulkanCommandPool,
        device::Device,
        init::{
            GraphicsInstance, GraphicsInstanceLayerAllowlist,
            GraphicsInstanceLayerAllowlistEntry,
        },
        instance::{InstanceBuilder, KHRONOS_VALIDATION_LAYER, ValidationMessenger},
        shader::{
            DynamicRenderingFormats, PipelineCache, PipelineLayout, RasterPipelineDesc,
            RasterPipelineOpts, ShaderModule, VulkanPipeline,
        },
    },
    rg::{
        executor::{PassContext, PassPgoDuration, PassPgoNameId},
        parallel::ParallelRecording,
        vulkan::{
            AccessTable, GpuResource, SessionFrameReceipt, VulkanGraphBackend, VulkanGraphSession,
        },
    },
};
use std::sync::Arc;

pub struct ValidatedEnv {
    pub graphics: GraphicsInstance,
    pub messenger: ValidationMessenger,
    pub layer_enabled: bool,
    pub device: Arc<Device>,
}

/// 带 debug utils 收集器的实例；Khronos 校验层可用时一并启用。无 Vulkan 时返回 None。
pub fn validated_env() -> Option<ValidatedEnv> {
    validated_env_configured(&rhapsody::device::QueueConfig {
        require_present: false,
        ..Default::default()
    })
}

pub fn validated_env_configured(config: &rhapsody::device::QueueConfig) -> Option<ValidatedEnv> {
    use crate::draw_support::gpu_test_support::initialize;
    let mut builder = InstanceBuilder::default();
    let layer_enabled = initialize("validation layer enumeration", builder.enable_validation())?;
    let mut allowlist = GraphicsInstanceLayerAllowlist::default();
    if layer_enabled {
        allowlist
            .entries
            .push(GraphicsInstanceLayerAllowlistEntry::Layer(
                KHRONOS_VALIDATION_LAYER.into(),
            ));
    }
    let graphics = initialize(
        "validated instance",
        GraphicsInstance::new_with_allowlist(&builder, &allowlist),
    )?;
    let messenger = initialize(
        "validation messenger",
        ValidationMessenger::new(&graphics.instance),
    )?;
    let device = initialize(
        "validated logical device",
        Device::new(graphics.instance.clone(), None, &rhapsody::device::DeviceConfig {
            queues: *config,
            ..rhapsody::device::DeviceConfig::headless()
        }),
    )?;
    Some(ValidatedEnv {
        graphics,
        messenger,
        layer_enabled,
        device,
    })
}

/// 与 `Rig::new` 相同的构造，但使用调用方给的设备。
pub fn rig_on(device: Arc<Device>) -> Rig {
    let gd = device.clone();
    let (vs, fs) = draw_support::shaders();
    let vertex = ShaderModule::from_spirv(device.clone(), &vs).unwrap();
    let fragment = ShaderModule::from_spirv(device.clone(), &fs).unwrap();
    let layout = PipelineLayout::new(device.clone(), vec![], vec![]).unwrap();
    let cache = PipelineCache::new(device.clone(), &[]).unwrap();
    let mut desc = RasterPipelineDesc::new(vertex, layout);
    desc.fragment = Some(fragment);
    desc.fragment_entry = Some(std::ffi::CString::new("ps_main").unwrap());
    desc.use_dynamic_rendering(DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM));
    desc.vertex_bindings = vec![vk::VertexInputBindingDescription {
        binding: 0,
        stride: 16,
        input_rate: vk::VertexInputRate::VERTEX,
    }];
    desc.vertex_attributes = vec![vk::VertexInputAttributeDescription {
        location: 0,
        binding: 0,
        format: vk::Format::R32G32B32A32_SFLOAT,
        offset: 0,
    }];
    RasterPipelineOpts(0).apply_to(&mut desc, 1).unwrap();
    let pipeline = VulkanPipeline::create_raster_pipeline(&cache, desc).unwrap();
    let pool = VulkanCommandPool::new(device.clone()).unwrap();
    Rig {
        device,
        gd,
        pipeline,
        pool,
    }
}

// ---------------------------------------------------------------- 大图

pub const BUF_BYTES: u64 = 256;

/// 填充回调实际运行所在的线程（用来观察并行录制是否真的跨线程）。
static THREADS: parking_lot::Mutex<Vec<std::thread::ThreadId>> =
    parking_lot::Mutex::new(Vec::new());
/// 取走并清空：返回去重后的线程数。多个测试并行时会互相污染，使用方应放在同一个测试函数里串行读取。
pub fn take_thread_count() -> usize {
    let mut v = THREADS.lock();
    v.sort_by_key(|t| format!("{t:?}"));
    v.dedup();
    let n = v.len();
    v.clear();
    n
}
pub const TRANSFER_READ: u8 = 0x25;
pub const HOST_READ: u8 = 0x26;
pub const TRANSFER_WRITE: u8 = 0x34;

/// 大图的持久读回缓冲：`fills` 个填充目标 + `copies` 个拷贝目标。
pub struct BigBuffers {
    pub fills: Vec<(u64, Arc<Buffer>)>,
    pub outs: Vec<(u64, Arc<Buffer>)>,
}
impl BigBuffers {
    pub fn new(session: &mut VulkanGraphSession, fills: usize, copies: usize) -> Self {
        let mut make = |usage: vk::BufferUsageFlags| {
            let desc = BufferDesc::new_gpu_to_cpu(BUF_BYTES, usage);
            let buffer = Arc::new(Buffer::new(session.backend.device().clone(), &desc).unwrap());
            (
                session
                    .backend
                    .bind_import(GpuResource::Buffer(buffer.clone()))
                    .unwrap(),
                buffer,
            )
        };
        let fills_v = (0..fills)
            .map(|_| make(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC))
            .collect();
        let outs_v = (0..copies)
            .map(|_| make(vk::BufferUsageFlags::TRANSFER_DST))
            .collect();
        Self {
            fills: fills_v,
            outs: outs_v,
        }
    }
    /// 全部缓冲字节，fills 在前、outs 在后。调用前必须已等到帧完成。
    pub fn read_all(&self) -> Vec<Vec<u8>> {
        self.fills
            .iter()
            .chain(&self.outs)
            .map(|(_, b)| {
                let mut bytes = vec![0u8; BUF_BYTES as usize];
                // SAFETY: 调用者已 wait_submits_done；导出的 HostRead 提供可见性。
                unsafe { b.read_bytes_after_completion(0, &mut bytes).unwrap() };
                bytes
            })
            .collect()
    }
}

/// 第 i 个填充 pass 写入的常量字（带 salt 使不同帧/不同运行的内容互不相同）。
pub fn fill_word(i: usize, salt: u32) -> u32 {
    0x0101_0101u32.wrapping_mul(i as u32 + 1) ^ salt.wrapping_mul(0x9e37_79b9)
}
/// 期望的全部缓冲内容：fills[i] = 常量 i；outs[k] = fills[2k] 的拷贝。
pub fn expected_buffers(fills: usize, copies: usize, salt: u32) -> Vec<Vec<u8>> {
    let word = |i: usize| {
        fill_word(i, salt)
            .to_le_bytes()
            .repeat(BUF_BYTES as usize / 4)
    };
    (0..fills)
        .map(word)
        .chain((0..copies).map(|k| word(2 * k)))
        .collect()
}

pub fn fill_pass_name(i: usize) -> String {
    format!("fill{i}")
}
pub fn copy_pass_name(k: usize) -> String {
    format!("copy{k}")
}
/// 全部 pass 的 PGO 名，按图内声明顺序。
pub fn pass_names(fills: usize, copies: usize) -> Vec<PassPgoNameId> {
    (0..fills)
        .map(|i| PassPgoNameId::from_name(&fill_pass_name(i)))
        .chain((0..copies).map(|k| PassPgoNameId::from_name(&copy_pass_name(k))))
        .collect()
}

pub struct BigRun {
    pub receipt: SessionFrameReceipt,
    pub buffers: Vec<Vec<u8>>,
}

/// 在会话上跑一帧大图（同一帧：24+ 个独立填充 pass，随后若干依赖前面填充结果的拷贝 pass），等完成并读回。
pub fn run_big_graph(
    session: &mut VulkanGraphSession,
    bufs: &BigBuffers,
    salt: u32,
    batch_per_pass: bool,
) -> BigRun {
    let device = session.backend.device().clone();
    let mut graph = session.begin_frame().expect("begin_frame");
    graph.batch_per_pass(batch_per_pass);
    let desc = rhapsody::rg::vulkan::ResourceSpec::Buffer(BufferDesc::new_gpu_to_cpu(
        BUF_BYTES,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::TRANSFER_SRC,
    ))
    .descriptor();
    let out_desc = rhapsody::rg::vulkan::ResourceSpec::Buffer(BufferDesc::new_gpu_to_cpu(
        BUF_BYTES,
        vk::BufferUsageFlags::TRANSFER_DST,
    ))
    .descriptor();
    let fills: Vec<_> = bufs
        .fills
        .iter()
        .enumerate()
        .map(|(i, (token, _))| {
            graph
                .import_unowned(format!("f{i}"), desc, *token, HOST_READ)
                .unwrap()
        })
        .collect();
    let outs: Vec<_> = bufs
        .outs
        .iter()
        .enumerate()
        .map(|(k, (token, _))| {
            graph
                .import_unowned(format!("o{k}"), out_desc, *token, HOST_READ)
                .unwrap()
        })
        .collect();
    for (i, &handle) in fills.iter().enumerate() {
        let device = device.clone();
        let index = handle.raw();
        let word = fill_word(i, salt);
        let mut pass = graph.add_pass(fill_pass_name(i)).with_side_effect();
        pass.add_resource_access(handle, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else {
                return false;
            };
            let Ok(buffer) = managed.buffer_for(index) else {
                return false;
            };

            THREADS.lock().push(std::thread::current().id());
            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                device
                    .raw_device()
                    .cmd_fill_buffer(cb, buffer.raw(), 0, vk::WHOLE_SIZE, word)
            };
            true
        });
    }
    for (k, &out) in outs.iter().enumerate() {
        let device = device.clone();
        let (src, dst) = (fills[2 * k], out);
        let (src_index, dst_index) = (src.raw(), dst.raw());
        let mut pass = graph.add_pass(copy_pass_name(k)).with_side_effect();
        pass.add_resource_access(src, TRANSFER_READ, 0).unwrap();
        pass.add_resource_access(dst, TRANSFER_WRITE, 0).unwrap();
        pass.render_with_context(move |ctx: PassContext<'_>| {
            let Some(managed) = ctx.managed else {
                return false;
            };
            let (Ok(from), Ok(to)) = (managed.buffer_for(src_index), managed.buffer_for(dst_index))
            else {
                return false;
            };

            let cb = vk::CommandBuffer::from_raw(ctx.command.handle());
            unsafe {
                device.raw_device().cmd_copy_buffer(
                    cb,
                    from.raw(),
                    to.raw(),
                    &[vk::BufferCopy {
                        src_offset: 0,
                        dst_offset: 0,
                        size: BUF_BYTES,
                    }],
                )
            };
            true
        });
    }
    for &handle in fills.iter().chain(&outs) {
        graph.export(handle, HOST_READ).unwrap();
    }
    let receipt = session.execute_frame(graph.build()).expect("execute_frame");
    session.wait_submits_done(10_000_000_000).unwrap();
    BigRun {
        receipt,
        buffers: bufs.read_all(),
    }
}

/// 新会话 + 并行配置。
pub fn new_session(device: &Arc<Device>, parallel: ParallelRecording) -> VulkanGraphSession {
    let backend = VulkanGraphBackend::new(device.clone(), AccessTable::original()).unwrap();
    let mut session = VulkanGraphSession::new(backend);
    session.chain.parallel = parallel;
    session
}

/// 给分块器的上一帧统计写入指定耗时（ns），按图内 pass 顺序；`None` 表示该 pass 没有记录。
pub fn seed_pgo(
    session: &mut VulkanGraphSession,
    names: &[PassPgoNameId],
    durations: &[Option<u64>],
) {
    for (name, d) in names.iter().zip(durations) {
        if let Some(d) = d {
            session
                .chain
                .pgo
                .previous
                .record(PassPgoDuration::new(name.clone(), *d));
        }
    }
}
