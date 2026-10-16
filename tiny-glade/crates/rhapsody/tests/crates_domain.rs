//! crates 域（vk-sync-fork / aftermath-rs / gpu-profiler / dxgi-enumerate-gpus）的聚焦测试。
//! 公开 vk-sync-fork 0.4.0（registry，ash 0.37）的期望值由 `get_image_memory_barrier` 逐变体探测得到后内嵌，
//! 避免为测试引入第二套 ash 依赖。
use ash::vk;
use rhapsody::raw::{
    access_table::{self, *},
    aftermath::{self, *},
    dxgi::{self, *},
    gpu_profiler::*,
};
use std::{
    ffi::{c_char, c_void, CStr},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

/// （公开变体名, 原版访问码, stage, access, layout）。
const PUBLIC_VARIANTS: &[(&str, u8, u32, u32, i32)] = &[
    ("Nothing", 0x00, 0x0, 0x0, 0),
    ("CommandBufferReadNVX", 0x01, 0x20000, 0x20000, 0),
    ("IndirectBuffer", 0x02, 0x2, 0x1, 0),
    ("IndexBuffer", 0x03, 0x4, 0x2, 0),
    ("VertexBuffer", 0x04, 0x4, 0x4, 0),
    ("VertexShaderReadUniformBuffer", 0x05, 0x8, 0x20, 0),
    ("VertexShaderReadSampledImageOrUniformTexelBuffer", 0x06, 0x8, 0x20, 5),
    ("VertexShaderReadOther", 0x07, 0x8, 0x20, 1),
    ("TessellationControlShaderReadUniformBuffer", 0x08, 0x10, 0x8, 0),
    ("TessellationControlShaderReadSampledImageOrUniformTexelBuffer", 0x09, 0x10, 0x20, 5),
    ("TessellationControlShaderReadOther", 0x0a, 0x10, 0x20, 1),
    ("TessellationEvaluationShaderReadUniformBuffer", 0x0b, 0x20, 0x8, 0),
    ("TessellationEvaluationShaderReadSampledImageOrUniformTexelBuffer", 0x0c, 0x20, 0x20, 5),
    ("TessellationEvaluationShaderReadOther", 0x0d, 0x20, 0x20, 1),
    ("GeometryShaderReadUniformBuffer", 0x0e, 0x40, 0x8, 0),
    ("GeometryShaderReadSampledImageOrUniformTexelBuffer", 0x0f, 0x40, 0x20, 5),
    ("GeometryShaderReadOther", 0x10, 0x40, 0x20, 1),
    ("FragmentShaderReadUniformBuffer", 0x11, 0x80, 0x8, 0),
    ("FragmentShaderReadSampledImageOrUniformTexelBuffer", 0x12, 0x80, 0x20, 5),
    ("FragmentShaderReadColorInputAttachment", 0x13, 0x80, 0x10, 5),
    ("FragmentShaderReadDepthStencilInputAttachment", 0x14, 0x80, 0x10, 4),
    ("FragmentShaderReadOther", 0x15, 0x80, 0x20, 1),
    ("ColorAttachmentRead", 0x16, 0x400, 0x80, 2),
    ("DepthStencilAttachmentRead", 0x17, 0x300, 0x200, 4),
    ("ComputeShaderReadUniformBuffer", 0x18, 0x800, 0x8, 0),
    ("ComputeShaderReadSampledImageOrUniformTexelBuffer", 0x19, 0x800, 0x20, 5),
    ("ComputeShaderReadOther", 0x1a, 0x800, 0x20, 1),
    ("AnyShaderReadUniformBuffer", 0x20, 0x10000, 0x8, 0),
    ("AnyShaderReadUniformBufferOrVertexBuffer", 0x21, 0x10000, 0xc, 0),
    ("AnyShaderReadSampledImageOrUniformTexelBuffer", 0x22, 0x10000, 0x20, 5),
    ("AnyShaderReadOther", 0x24, 0x10000, 0x20, 1),
    ("TransferRead", 0x25, 0x1000, 0x800, 6),
    ("HostRead", 0x26, 0x4000, 0x2000, 1),
    ("Present", 0x27, 0x0, 0x0, 1000001002),
    ("CommandBufferWriteNVX", 0x28, 0x20000, 0x40000, 0),
    ("VertexShaderWrite", 0x29, 0x8, 0x40, 1),
    ("TessellationControlShaderWrite", 0x2a, 0x10, 0x40, 1),
    ("TessellationEvaluationShaderWrite", 0x2b, 0x20, 0x40, 1),
    ("GeometryShaderWrite", 0x2c, 0x40, 0x40, 1),
    ("FragmentShaderWrite", 0x2d, 0x80, 0x40, 1),
    ("ColorAttachmentWrite", 0x2e, 0x400, 0x100, 2),
    ("DepthStencilAttachmentWrite", 0x2f, 0x300, 0x400, 3),
    ("DepthAttachmentWriteStencilReadOnly", 0x30, 0x300, 0x600, 1000117001),
    ("StencilAttachmentWriteDepthReadOnly", 0x31, 0x300, 0x600, 1000117000),
    ("ComputeShaderWrite", 0x32, 0x800, 0x40, 1),
    ("AnyShaderWrite", 0x33, 0x10000, 0x40, 1),
    ("TransferWrite", 0x34, 0x1000, 0x1000, 7),
    ("HostWrite", 0x35, 0x4000, 0x4000, 1),
    ("ColorAttachmentReadWrite", 0x36, 0x400, 0x180, 2),
    ("General", 0x37, 0x10000, 0x18000, 1),
    ("RayTracingShaderReadSampledImageOrUniformTexelBuffer", 0x38, 0x200000, 0x20, 5),
    ("RayTracingShaderReadColorInputAttachment", 0x39, 0x200000, 0x10, 5),
    ("RayTracingShaderReadDepthStencilInputAttachment", 0x3a, 0x200000, 0x10, 4),
    ("RayTracingShaderReadAccelerationStructure", 0x3b, 0x200000, 0x200000, 0),
    ("RayTracingShaderReadOther", 0x3c, 0x200000, 0x20, 1),
    ("AccelerationStructureBuildWrite", 0x3d, 0x2000000, 0x400000, 0),
    ("AccelerationStructureBuildRead", 0x3e, 0x2000000, 0x200000, 0),
    ("AccelerationStructureBufferWrite", 0x3f, 0x2000000, 0x1000, 0),];
/// 原版新增（公开枚举里没有）的 6 个读变体码。
const FORK_ONLY: [u8; 6] = [0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x23];
/// 与公开值不同的两项：IndirectBuffer（stage 0x2→0x10000，access 0x1→0x21）、DepthStencilAttachmentWrite（access 0x400→0x600）。
const DIFFERS_FROM_PUBLIC: [u8; 2] = [0x02, 0x2f];

#[test]
fn table_matches_public_vk_sync_fork_except_documented_differences() {
    for &(name, code, stage, access, layout) in PUBLIC_VARIANTS {
        let info = access_info(code).unwrap();
        let same = (info.stage, info.access, info.layout) == (stage, access, layout);
        assert_eq!(same, !DIFFERS_FROM_PUBLIC.contains(&code), "{name} (0x{code:02x}): {info:?}");
    }
    let ind = access_info(0x02).unwrap();
    assert_eq!((ind.stage, ind.access), (0x10000, 0x21));
    assert_eq!(access_info(0x2f).unwrap().access, 0x600);
    // 公开 58 个变体 + 原版 6 个新增 = 64 个码，且互不重叠。
    let mut seen: Vec<u8> = PUBLIC_VARIANTS.iter().map(|v| v.1).chain(FORK_ONLY).collect();
    seen.sort_unstable();
    assert_eq!(seen, (0..64u8).collect::<Vec<_>>());
    // 新增读变体：stage 为 VS..FS 的并集或 ALL，layout 沿用 0/5/4/1 约定。
    assert_eq!(access_info(0x1b).unwrap().stage, 0xf8);
    assert_eq!(access_info(0x1e).unwrap().stage, 0x3f8);
    assert!(FORK_ONLY.iter().all(|c| !is_write(*c)));
}

#[test]
fn memory_barrier_follows_original_stage_defaults_and_write_only_src_access() {
    // 空 → TOP / BOTTOM_OF_PIPE（`cmp r,1; adc r,0` 与 `cmovne eax,0x2000`）。
    let (s, d, b) = get_memory_barrier(&[], &[]);
    assert_eq!((s.as_raw(), d.as_raw()), (1, 0x2000));
    assert_eq!((b.src_access_mask.as_raw(), b.dst_access_mask.as_raw()), (0, 0));
    assert_eq!(b.s_type, vk::StructureType::MEMORY_BARRIER);
    // 写→读：src 取写 access，dst 取读 access。
    let (s, d, b) = get_memory_barrier(&[0x32], &[0x1a]);
    assert_eq!((s.as_raw(), d.as_raw()), (0x800, 0x800));
    assert_eq!((b.src_access_mask.as_raw(), b.dst_access_mask.as_raw()), (0x40, 0x20));
    // 读→读：src 无写 → dst access 清零（RAR/WAR 不需要可见性）。
    let (_, _, b) = get_memory_barrier(&[0x1a], &[0x1a]);
    assert_eq!((b.src_access_mask.as_raw(), b.dst_access_mask.as_raw()), (0, 0));
}

#[test]
fn buffer_barrier_passes_fields_through() {
    let buffer = <vk::Buffer as ash::vk::Handle>::from_raw(0xdead);
    let (s, d, b) = get_buffer_memory_barrier(&BufferBarrier {
        previous: &[0x34],
        next: &[0x03, 0x1a],
        src_queue_family_index: 2,
        dst_queue_family_index: 3,
        buffer,
        offset: 16,
        size: 64,
    });
    assert_eq!((s.as_raw(), d.as_raw()), (0x1000, 0x4 | 0x800));
    assert_eq!((b.src_access_mask.as_raw(), b.dst_access_mask.as_raw()), (0x1000, 0x2 | 0x20));
    assert_eq!((b.src_queue_family_index, b.dst_queue_family_index, b.offset, b.size), (2, 3, 16, 64));
    assert_eq!(b.buffer, buffer);
    assert_eq!(b.s_type, vk::StructureType::BUFFER_MEMORY_BARRIER);
}

fn image_barrier<'a>(prev: &'a [u8], next: &'a [u8]) -> ImageBarrier<'a> {
    ImageBarrier {
        previous: prev,
        next,
        previous_layout: BarrierLayout::Optimal,
        next_layout: BarrierLayout::Optimal,
        discard_contents: false,
        src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
        image: vk::Image::null(),
        range: vk::ImageSubresourceRange::default(),
    }
}

#[test]
fn image_barrier_layouts_and_unconditional_dst_access() {
    // 读→读也带 dst access（image 版无"src 有写"条件，0x140c10c52）。
    let (_, _, b) = get_image_memory_barrier(&image_barrier(&[0x1a], &[0x12]));
    assert_eq!(b.src_access_mask.as_raw(), 0);
    assert_eq!(b.dst_access_mask.as_raw(), 0x20);
    assert_eq!((b.old_layout.as_raw(), b.new_layout.as_raw()), (1, 5));
    // discard_contents：old_layout = UNDEFINED，但 src stage 仍累计。
    let mut d = image_barrier(&[0x2e], &[0x12]);
    d.discard_contents = true;
    let (s, _, b) = get_image_memory_barrier(&d);
    assert_eq!((s.as_raw(), b.old_layout.as_raw()), (0x400, 0));
    // General：Present → PRESENT_SRC_KHR(0x3b9acdea)，其他 → GENERAL；空 next → 0。
    let mut g = image_barrier(&[0x27], &[0x27]);
    g.previous_layout = BarrierLayout::General;
    g.next_layout = BarrierLayout::General;
    let (_, _, b) = get_image_memory_barrier(&g);
    assert_eq!((b.old_layout.as_raw(), b.new_layout.as_raw()), (0x3b9acdea, 0x3b9acdea));
    let mut g = image_barrier(&[0x32], &[]);
    g.previous_layout = BarrierLayout::General;
    let (_, d, b) = get_image_memory_barrier(&g);
    assert_eq!((d.as_raw(), b.old_layout.as_raw(), b.new_layout.as_raw()), (0x2000, 1, 0));
    // 最后一个访问码的布局胜出。
    let (_, _, b) = get_image_memory_barrier(&image_barrier(&[], &[0x34, 0x25]));
    assert_eq!(b.new_layout.as_raw(), 6);
    assert_eq!(b.s_type, vk::StructureType::IMAGE_MEMORY_BARRIER);
    let _ = access_table::ACCESS_CODE_COUNT;
}

// ---- aftermath ------------------------------------------------------------------------------

#[derive(Default)]
struct Recorded {
    dumps: Mutex<Vec<Vec<u8>>>,
    infos: Mutex<Vec<Vec<u8>>>,
    descriptions: AtomicU32,
}
struct Delegate(Arc<Recorded>);
impl NvAftermathDelegate for Delegate {
    fn dumped(&self, dump: &[u8]) {
        self.0.dumps.lock().unwrap().push(dump.to_vec());
    }
    fn shader_debug_info(&self, info: &[u8]) {
        self.0.infos.lock().unwrap().push(info.to_vec());
    }
    fn description(&self, add: &mut GpuCrashDumpDescription) {
        self.0.descriptions.fetch_add(1, Ordering::SeqCst);
        add.add(7, c"hello");
    }
}

#[derive(Default)]
struct MockState {
    enable_args: Mutex<Option<(u32, u32, u32, usize)>>,
    callbacks: Mutex<Option<(Callbacks, usize)>>,
    disabled: AtomicU32,
    statuses: Mutex<Vec<u32>>,
    polls: AtomicU32,
    id_result: Mutex<(i32, [u64; 2])>,
}
#[derive(Clone, Default)]
struct MockApi(Arc<MockState>);
impl AftermathApi for MockApi {
    fn enable_gpu_crash_dumps(&self, v: u32, w: u32, f: u32, cb: Callbacks, user: *mut c_void) -> i32 {
        *self.0.enable_args.lock().unwrap() = Some((v, w, f, user as usize));
        *self.0.callbacks.lock().unwrap() = Some((cb, user as usize));
        aftermath::RESULT_SUCCESS
    }
    fn disable_gpu_crash_dumps(&self) -> i32 {
        self.0.disabled.fetch_add(1, Ordering::SeqCst);
        1
    }
    fn crash_dump_status(&self) -> (i32, u32) {
        let n = self.0.polls.fetch_add(1, Ordering::SeqCst) as usize;
        let list = self.0.statuses.lock().unwrap();
        (1, *list.get(n).or(list.last()).unwrap_or(&0))
    }
    fn shader_debug_info_identifier(&self, version: u32, info: &[u8]) -> (i32, [u64; 2]) {
        assert_eq!(version, 0x217);
        assert!(!info.is_empty());
        *self.0.id_result.lock().unwrap()
    }
}

static DESC_KEYS: Mutex<Vec<(u32, String)>> = Mutex::new(Vec::new());
unsafe extern "C" fn fake_add(key: u32, value: *const c_char) {
    // SAFETY: 测试里传入的是 NUL 结尾的字面量。
    let s = unsafe { CStr::from_ptr(value) }.to_string_lossy().into_owned();
    DESC_KEYS.lock().unwrap().push((key, s));
}

#[test]
fn aftermath_registers_with_original_arguments_forwards_callbacks_and_disables_on_drop() {
    let rec = Arc::new(Recorded::default());
    let api = MockApi::default();
    let state = api.0.clone();
    {
        let am = Aftermath::new(Delegate(rec.clone()), api);
        assert_eq!(am.enable_result(), aftermath::RESULT_SUCCESS);
        let (v, w, f, user) = state.enable_args.lock().unwrap().unwrap();
        assert_eq!((v, w, f), (0x217, 2, 1));
        assert_ne!(user, 0);
        let (cb, user) = state.callbacks.lock().unwrap().unwrap();
        let user = user as *mut c_void;
        // SAFETY: 回调与用户数据来自 `Aftermath::new`，缓冲在调用期间有效。
        unsafe {
            (cb.gpu_crash_dump)([1u8, 2, 3].as_ptr().cast(), 3, user);
            (cb.shader_debug_info)([9u8, 8].as_ptr().cast(), 2, user);
            (cb.gpu_crash_dump)(std::ptr::null(), 0, user);
            (cb.description)(fake_add, user);
            // ResolveMarker 回调是空函数：不写出参。
            let mut out: *mut c_void = 0x55 as *mut c_void;
            let mut out_size = 77u32;
            (cb.resolve_marker)(std::ptr::null(), 0, user, &mut out, &mut out_size);
            assert_eq!((out as usize, out_size), (0x55, 77));
        }
        assert_eq!(*rec.dumps.lock().unwrap(), vec![vec![1, 2, 3], vec![]]);
        assert_eq!(*rec.infos.lock().unwrap(), vec![vec![9, 8]]);
        assert_eq!(rec.descriptions.load(Ordering::SeqCst), 1);
        assert_eq!(*DESC_KEYS.lock().unwrap(), vec![(7, "hello".to_string())]);
        assert_eq!(state.disabled.load(Ordering::SeqCst), 0);
    }
    assert_eq!(state.disabled.load(Ordering::SeqCst), 1, "Drop = DisableGpuCrashDumps");
}

#[test]
fn aftermath_wait_for_status_polls_every_50ms_until_2_or_4_and_honours_timeout() {
    let api = MockApi::default();
    let state = api.0.clone();
    let am = Aftermath::new(Delegate(Arc::default()), api);
    // 立即为 2：不睡眠。
    *state.statuses.lock().unwrap() = vec![2];
    let mut sleeps = Vec::new();
    assert_eq!(am.wait_for_status_with(None, |d| sleeps.push(d)), 2);
    assert!(sleeps.is_empty());
    // 1,1,1,4：睡 3 次 50ms。
    state.polls.store(0, Ordering::SeqCst);
    *state.statuses.lock().unwrap() = vec![1, 1, 1, 4];
    assert_eq!(am.wait_for_status_with(None, |d| sleeps.push(d)), 4);
    assert_eq!(sleeps, vec![Duration::from_millis(50); 3]);
    // 永远 1：timeout 180ms → elapsed 0,50,100,150 各睡一次，200>=180 返回当前状态 1。
    state.polls.store(0, Ordering::SeqCst);
    *state.statuses.lock().unwrap() = vec![1];
    sleeps.clear();
    assert_eq!(am.wait_for_status_with(Some(Duration::from_millis(180)), |d| sleeps.push(d)), 1);
    assert_eq!(sleeps.len(), 4);
    // timeout 0：读一次就返回。
    state.polls.store(0, Ordering::SeqCst);
    sleeps.clear();
    assert_eq!(am.wait_for_status_with(Some(Duration::ZERO), |d| sleeps.push(d)), 1);
    assert!(sleeps.is_empty());
}

#[test]
fn aftermath_identifier_error_text_matches_the_original_format() {
    let api = MockApi::default();
    let state = api.0.clone();
    let am = Aftermath::new(Delegate(Arc::default()), api);
    *state.id_result.lock().unwrap() = (1, [0x1122, 0x3344]);
    assert_eq!(am.shader_debug_info_identifier(&[1]).unwrap(), ShaderDebugInfoIdentifier([0x1122, 0x3344]));
    *state.id_result.lock().unwrap() = (0x8bad_f00d_u32 as i32, [0; 2]);
    let err = am.shader_debug_info_identifier(&[1]).unwrap_err();
    assert_eq!(err.to_string(), format!("GFSDK_Aftermath_GetShaderDebugInfoIdentifier: {:?}\n", 0x8bad_f00d_u32 as i32));
    assert_eq!(ShaderDebugInfoIdentifier::default(), ShaderDebugInfoIdentifier([0, 0]));
}

#[test]
fn aftermath_unavailable_api_reports_failure_without_crashing() {
    let am = Aftermath::new(Delegate(Arc::default()), UnavailableApi);
    assert_eq!(am.enable_result(), 0);
    assert_eq!(am.status(), 0);
    assert!(am.shader_debug_info_identifier(&[1]).is_err());
}

// ---- dxgi -----------------------------------------------------------------------------------

fn desc(name: &str, vendor: u32, low: u32, high: i32) -> DxgiAdapterDesc {
    let mut description = [0u16; 128];
    for (slot, c) in description.iter_mut().zip(name.encode_utf16()) {
        *slot = c;
    }
    DxgiAdapterDesc {
        description,
        vendor_id: vendor,
        device_id: 0xbeef,
        sub_sys_id: 1,
        revision: 2,
        dedicated_video_memory: 3,
        dedicated_system_memory: 4,
        shared_system_memory: 5,
        luid_low_part: low,
        luid_high_part: high,
    }
}

#[test]
fn dxgi_record_layout_and_field_mapping() {
    assert_eq!(std::mem::size_of::<DxgiAdapterDesc>(), 0x130);
    assert_eq!(std::mem::offset_of!(DxgiAdapterDesc, vendor_id), 0x100);
    assert_eq!(std::mem::offset_of!(DxgiAdapterDesc, luid_low_part), 0x128);
    assert_eq!(std::mem::offset_of!(DxgiAdapterDesc, luid_high_part), 0x12c);
    let a = DxgiAdapter::from_desc(&desc("NVIDIA GeForce", 0x10de, 0xdead_beef, -2));
    assert_eq!(a.name, "NVIDIA GeForce");
    assert_eq!(a.vendor_id, 0x10de);
    // HighPart 是有符号数：movsxd 后左移 32，LowPart 零扩展。
    assert_eq!(a.luid, (-2i64 as u64) << 32 | 0xdead_beef);
    // 128 个 WCHAR 全满、无 NUL：整串。
    let mut full = desc("", 1, 0, 0);
    full.description = [b'a' as u16; 128];
    assert_eq!(DxgiAdapter::from_desc(&full).name.len(), 128);
}

#[test]
fn dxgi_enumerate_skips_failed_getdesc_stops_at_first_enum_error_and_maps_factory_failure() {
    let mut src = FixedDxgi {
        factory_error: None,
        adapters: vec![
            Some(desc("A", 0x1002, 1, 0)),
            None,
            Some(desc("B", 0x10de, 2, 0)),
        ],
    };
    let list = dxgi::enumerate(&mut src).unwrap();
    assert_eq!(list.iter().map(|a| a.name.as_str()).collect::<Vec<_>>(), ["A", "B"]);
    assert_eq!(list[1].luid, 2);
    let mut bad = FixedDxgi { factory_error: Some(DxgiError { hresult: 0x887a0004_u32 as i32 }), adapters: vec![] };
    let err = dxgi::enumerate(&mut bad).unwrap_err();
    assert!(err.to_string().starts_with("CreateDXGIFactory"));
    assert!(dxgi::enumerate(&mut FixedDxgi::default()).unwrap().is_empty());
}

// ---- gpu-profiler ---------------------------------------------------------------------------

#[test]
fn gpu_profiler_retrieve_results_pairs_indices_with_ticks_and_sorts_by_tick() {
    let frame = ProfilerSubmittedFrameForTest::build(&["x", "y", "z"], &[2, 0, 1]);
    let (entries, markers) = frame.retrieve_results(&[30, 10, 20, 99]);
    assert_eq!(markers.len(), 3);
    // 配对：(2,30) (0,10) (1,20) → 按 tick 升序。
    let got: Vec<(u32, u64)> = entries.iter().map(|e| (e.marker.0, e.tick.0)).collect();
    assert_eq!(got, vec![(0, 10), (1, 20), (2, 30)]);
    // count > len → panic。
    let frame = ProfilerSubmittedFrameForTest::build(&["x"], &[0, 0]);
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| frame.retrieve_results(&[1]))).is_err());
}

/// `ProfilerSubmittedFrame` 的字段私有（只由 `end_frame` 构造）：这里走真实 Vulkan 的链路测试在下面；
/// 纯逻辑测试用 lavapipe 可用时的真实帧，不可用时跳过。
struct ProfilerSubmittedFrameForTest;
impl ProfilerSubmittedFrameForTest {
    fn build(_names: &[&str], _indices: &[u32]) -> rhapsody::raw::gpu_profiler::ProfilerSubmittedFrame {
        rhapsody::raw::gpu_profiler::ProfilerSubmittedFrame::from_parts(
            _names.iter().fold(FrameMarkers::default(), |mut m, n| {
                m.create_marker(n);
                m
            }),
            _indices.to_vec(),
        )
    }
}

/// 真实 Vulkan（lavapipe）上的整条链：Profiler::new → begin_frame → create/write_marker → end_frame →
/// CopyQueryPoolResults(64_BIT|WAIT) → 读回 → retrieve_results。无 Vulkan 设备时跳过。
#[test]
fn gpu_profiler_chain_runs_on_a_real_device() {
    use rhapsody::raw::{buffer::Buffer, init::create_headless_device, profiler::gpu_profiler_output_buffer_desc};
    let Ok(vk_device) = create_headless_device() else {
        eprintln!("skip: no Vulkan device");
        return;
    };
    let device = &vk_device.device;
    let output = Buffer::new(vk_device.clone(), &gpu_profiler_output_buffer_desc()).unwrap();
    let profiler = Profiler::new(device).unwrap();
    // 索引缓冲初值全是 0xffffffff。
    assert!((0..MAX_MARKERS).all(|i| profiler.index_slot(i) == INDEX_BUFFER_FILL));
    let period = unsafe { vk_device.instance.instance.get_physical_device_properties(vk_device.physical) }
        .limits
        .timestamp_period;
    // SAFETY: 单线程录制并同步等待；对象在等待完成后销毁。
    unsafe {
        let pool = device
            .create_command_pool(
                &vk::CommandPoolCreateInfo::default().queue_family_index(vk_device.queue_family),
                None,
            )
            .unwrap();
        let cmd = device
            .allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(pool)
                    .command_buffer_count(1),
            )
            .unwrap()[0];
        device.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default()).unwrap();
        let mut active = profiler.begin_frame(device, cmd);
        let (a, b, c) = (active.create_marker("a"), active.create_marker("b"), active.create_marker("a"));
        assert_eq!((a, b, c), (MarkerId(0), MarkerId(1), MarkerId(2)));
        active.write_marker(device, cmd, c);
        active.write_marker(device, cmd, a);
        active.write_marker(device, cmd, b);
        assert_eq!(profiler.used_count(), 3);
        assert_eq!(profiler.index_slot(0), 2);
        assert_eq!(profiler.index_slot(2), 1);
        let frame = active.end_frame(device, cmd, output.handle);
        device.end_command_buffer(cmd).unwrap();
        let cmds = [cmd];
        device
            .queue_submit(vk_device.queue, &[vk::SubmitInfo::default().command_buffers(&cmds)], vk::Fence::null())
            .unwrap();
        device.queue_wait_idle(vk_device.queue).unwrap();
        device.destroy_command_pool(pool, None);
        assert_eq!(frame.query_indices(), &[2, 0, 1]);
        let mut bytes = vec![0u8; 0x2000];
        output.read_bytes(0, &mut bytes).unwrap();
        let ticks: Vec<u64> = bytes.chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        let (entries, markers) = frame.retrieve_results(&ticks);
        assert_eq!(entries.len(), 3);
        assert!(entries.windows(2).all(|w| w[0].tick <= w[1].tick), "按 tick 升序");
        assert_eq!(markers.marker_name(MarkerId(2)), "a");
        let span = entries[2].tick.duration_since(entries[0].tick).nanoseconds(period);
        let _ = span; // 数值取决于驱动；只要求换算不 panic。
        profiler.destroy(device);
    }
}

// ---- shader-pipeline 核对（只读；结论对照原机器码，见报告）-------------------------------------------

/// shader `defines` 文件键哈希：`acc ^= SipHash-1-3(key 0,0)((String,String))`（std `DefaultHasher`），
/// 发行文件名里仅有的两个 raster 哈希由单个 depth-only 宏复现（本测试不经过 shader-pipeline，独立核对）。
#[test]
fn shipped_raster_defines_hashes_are_siphash13_xor_of_pair_hashes() {
    use std::hash::{DefaultHasher, Hash, Hasher};
    let fold = |pairs: &[(&str, &str)]| {
        pairs.iter().fold(0u64, |acc, (a, b)| {
            let mut h = DefaultHasher::new();
            (a.to_string(), b.to_string()).hash(&mut h);
            acc ^ h.finish()
        })
    };
    assert_eq!(fold(&[("SHADER_VARIANT_DEPTH_ONLY", "0")]), 0xb903_e43f_fb3d_a915);
    assert_eq!(fold(&[("SHADER_VARIANT_DEPTH_ONLY", "1")]), 0xf0ad_ff76_827e_73ff);
    assert_eq!(fold(&[]), 0, "compute 键：空 defines 哈希为 0");
}
