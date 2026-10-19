//! crates 域（vk-sync-fork / aftermath-rs / gpu-profiler / dxgi-enumerate-gpus）的聚焦测试。
//! 访问码表的期望值按公开 vk-sync-fork 0.4.0 的变体内嵌，避免为测试引入第二套 ash 依赖。
use rhapsody::raw::{
    access_table::*,
    aftermath::{self, *},
    dxgi::{self, *},
};
use std::{
    ffi::{CStr, c_char, c_void},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU32, Ordering},
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
    (
        "VertexShaderReadSampledImageOrUniformTexelBuffer",
        0x06,
        0x8,
        0x20,
        5,
    ),
    ("VertexShaderReadOther", 0x07, 0x8, 0x20, 1),
    (
        "TessellationControlShaderReadUniformBuffer",
        0x08,
        0x10,
        0x8,
        0,
    ),
    (
        "TessellationControlShaderReadSampledImageOrUniformTexelBuffer",
        0x09,
        0x10,
        0x20,
        5,
    ),
    ("TessellationControlShaderReadOther", 0x0a, 0x10, 0x20, 1),
    (
        "TessellationEvaluationShaderReadUniformBuffer",
        0x0b,
        0x20,
        0x8,
        0,
    ),
    (
        "TessellationEvaluationShaderReadSampledImageOrUniformTexelBuffer",
        0x0c,
        0x20,
        0x20,
        5,
    ),
    ("TessellationEvaluationShaderReadOther", 0x0d, 0x20, 0x20, 1),
    ("GeometryShaderReadUniformBuffer", 0x0e, 0x40, 0x8, 0),
    (
        "GeometryShaderReadSampledImageOrUniformTexelBuffer",
        0x0f,
        0x40,
        0x20,
        5,
    ),
    ("GeometryShaderReadOther", 0x10, 0x40, 0x20, 1),
    ("FragmentShaderReadUniformBuffer", 0x11, 0x80, 0x8, 0),
    (
        "FragmentShaderReadSampledImageOrUniformTexelBuffer",
        0x12,
        0x80,
        0x20,
        5,
    ),
    (
        "FragmentShaderReadColorInputAttachment",
        0x13,
        0x80,
        0x10,
        5,
    ),
    (
        "FragmentShaderReadDepthStencilInputAttachment",
        0x14,
        0x80,
        0x10,
        4,
    ),
    ("FragmentShaderReadOther", 0x15, 0x80, 0x20, 1),
    ("ColorAttachmentRead", 0x16, 0x400, 0x80, 2),
    ("DepthStencilAttachmentRead", 0x17, 0x300, 0x200, 4),
    ("ComputeShaderReadUniformBuffer", 0x18, 0x800, 0x8, 0),
    (
        "ComputeShaderReadSampledImageOrUniformTexelBuffer",
        0x19,
        0x800,
        0x20,
        5,
    ),
    ("ComputeShaderReadOther", 0x1a, 0x800, 0x20, 1),
    ("AnyShaderReadUniformBuffer", 0x20, 0x10000, 0x8, 0),
    (
        "AnyShaderReadUniformBufferOrVertexBuffer",
        0x21,
        0x10000,
        0xc,
        0,
    ),
    (
        "AnyShaderReadSampledImageOrUniformTexelBuffer",
        0x22,
        0x10000,
        0x20,
        5,
    ),
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
    (
        "DepthAttachmentWriteStencilReadOnly",
        0x30,
        0x300,
        0x600,
        1000117001,
    ),
    (
        "StencilAttachmentWriteDepthReadOnly",
        0x31,
        0x300,
        0x600,
        1000117000,
    ),
    ("ComputeShaderWrite", 0x32, 0x800, 0x40, 1),
    ("AnyShaderWrite", 0x33, 0x10000, 0x40, 1),
    ("TransferWrite", 0x34, 0x1000, 0x1000, 7),
    ("HostWrite", 0x35, 0x4000, 0x4000, 1),
    ("ColorAttachmentReadWrite", 0x36, 0x400, 0x180, 2),
    ("General", 0x37, 0x10000, 0x18000, 1),
    (
        "RayTracingShaderReadSampledImageOrUniformTexelBuffer",
        0x38,
        0x200000,
        0x20,
        5,
    ),
    (
        "RayTracingShaderReadColorInputAttachment",
        0x39,
        0x200000,
        0x10,
        5,
    ),
    (
        "RayTracingShaderReadDepthStencilInputAttachment",
        0x3a,
        0x200000,
        0x10,
        4,
    ),
    (
        "RayTracingShaderReadAccelerationStructure",
        0x3b,
        0x200000,
        0x200000,
        0,
    ),
    ("RayTracingShaderReadOther", 0x3c, 0x200000, 0x20, 1),
    (
        "AccelerationStructureBuildWrite",
        0x3d,
        0x2000000,
        0x400000,
        0,
    ),
    (
        "AccelerationStructureBuildRead",
        0x3e,
        0x2000000,
        0x200000,
        0,
    ),
    (
        "AccelerationStructureBufferWrite",
        0x3f,
        0x2000000,
        0x1000,
        0,
    ),
];
/// 原版新增（公开枚举里没有）的 6 个读变体码。
const FORK_ONLY: [u8; 6] = [0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x23];
/// 与公开值不同的两项：IndirectBuffer（stage 0x2→0x10000，access 0x1→0x21）、DepthStencilAttachmentWrite（access 0x400→0x600）。
const DIFFERS_FROM_PUBLIC: [u8; 2] = [0x02, 0x2f];

#[test]
fn table_matches_public_vk_sync_fork_except_documented_differences() {
    for &(name, code, stage, access, layout) in PUBLIC_VARIANTS {
        let info = access_info(code).unwrap();
        let same = (info.scope.stages.as_raw(), info.scope.access.as_raw(), info.layout.as_raw()) == (stage, access, layout);
        assert_eq!(
            same,
            !DIFFERS_FROM_PUBLIC.contains(&code),
            "{name} (0x{code:02x}): {info:?}"
        );
    }
    let ind = access_info(0x02).unwrap();
    assert_eq!((ind.scope.stages.as_raw(), ind.scope.access.as_raw()), (0x10000, 0x21));
    assert_eq!(access_info(0x2f).unwrap().scope.access.as_raw(), 0x600);
    // 公开 58 个变体 + 原版 6 个新增 = 64 个码，且互不重叠。
    let mut seen: Vec<u8> = PUBLIC_VARIANTS
        .iter()
        .map(|v| v.1)
        .chain(FORK_ONLY)
        .collect();
    seen.sort_unstable();
    assert_eq!(seen, (0..64u8).collect::<Vec<_>>());
    // 新增读变体：stage 为 VS..FS 的并集或 ALL，layout 沿用 0/5/4/1 约定。
    assert_eq!(access_info(0x1b).unwrap().scope.stages.as_raw(), 0xf8);
    assert_eq!(access_info(0x1e).unwrap().scope.stages.as_raw(), 0x3f8);
    assert!(FORK_ONLY.iter().all(|c| !is_write(*c)));
    for (code, spec) in ACCESS_TABLE.iter().enumerate() {
        assert_eq!(spec.scope.write, is_write(code as u8));
    }
    assert_eq!(ACCESS_TABLE.len(), ACCESS_CODE_COUNT);
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
    unsafe fn enable_gpu_crash_dumps(
        &self,
        v: u32,
        w: u32,
        f: u32,
        cb: Callbacks,
        user: *mut c_void,
    ) -> i32 {
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
    let s = unsafe { CStr::from_ptr(value) }
        .to_string_lossy()
        .into_owned();
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
    assert_eq!(
        state.disabled.load(Ordering::SeqCst),
        1,
        "Drop = DisableGpuCrashDumps"
    );
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
    assert_eq!(
        am.wait_for_status_with(Some(Duration::from_millis(180)), |d| sleeps.push(d)),
        1
    );
    assert_eq!(sleeps.len(), 4);
    // timeout 0：读一次就返回。
    state.polls.store(0, Ordering::SeqCst);
    sleeps.clear();
    assert_eq!(
        am.wait_for_status_with(Some(Duration::ZERO), |d| sleeps.push(d)),
        1
    );
    assert!(sleeps.is_empty());
}

#[test]
fn aftermath_identifier_error_text_matches_the_original_format() {
    let api = MockApi::default();
    let state = api.0.clone();
    let am = Aftermath::new(Delegate(Arc::default()), api);
    *state.id_result.lock().unwrap() = (1, [0x1122, 0x3344]);
    assert_eq!(
        am.shader_debug_info_identifier(&[1]).unwrap(),
        ShaderDebugInfoIdentifier([0x1122, 0x3344])
    );
    *state.id_result.lock().unwrap() = (0x8bad_f00d_u32 as i32, [0; 2]);
    let err = am.shader_debug_info_identifier(&[1]).unwrap_err();
    assert_eq!(
        err.to_string(),
        format!(
            "GFSDK_Aftermath_GetShaderDebugInfoIdentifier: {:?}\n",
            0x8bad_f00d_u32 as i32
        )
    );
    assert_eq!(
        ShaderDebugInfoIdentifier::default(),
        ShaderDebugInfoIdentifier([0, 0])
    );
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
    assert_eq!(
        list.iter().map(|a| a.name.as_str()).collect::<Vec<_>>(),
        ["A", "B"]
    );
    assert_eq!(list[1].luid, 2);
    let mut bad = FixedDxgi {
        factory_error: Some(DxgiError {
            hresult: 0x887a0004_u32 as i32,
        }),
        adapters: vec![],
    };
    let err = dxgi::enumerate(&mut bad).unwrap_err();
    assert!(err.to_string().starts_with("CreateDXGIFactory"));
    assert!(
        dxgi::enumerate(&mut FixedDxgi::default())
            .unwrap()
            .is_empty()
    );
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
    assert_eq!(
        fold(&[("SHADER_VARIANT_DEPTH_ONLY", "0")]),
        0xb903_e43f_fb3d_a915
    );
    assert_eq!(
        fold(&[("SHADER_VARIANT_DEPTH_ONLY", "1")]),
        0xf0ad_ff76_827e_73ff
    );
    assert_eq!(fold(&[]), 0, "compute 键：空 defines 哈希为 0");
}
