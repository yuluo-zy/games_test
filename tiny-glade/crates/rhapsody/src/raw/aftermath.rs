//! 原 `aftermath-rs`（本地 crate，Cargo.lock 记 0.1.3、无 source、依赖 sysreq）：NVIDIA Nsight Aftermath
//! GPU 崩溃转储 SDK 的薄封装。
//!
//! 证据（B patched EXE，机器码为准）：
//! * 导入表 `GFSDK_Aftermath_Lib.x64.dll` 恰好导入 4 个函数：`DisableGpuCrashDumps`（IAT 0x143192d38）、
//!   `EnableGpuCrashDumps`、`GetCrashDumpStatus`、`GetShaderDebugInfoIdentifier`。SDK 本体不是 Rust 依赖，
//!   这里的 [`AftermathApi`] 就是这 4 个入口的抽象；Windows 用 `raw-dylib` 绑定，其余平台只有注入实现。
//! * `Aftermath::new<NvAftermathDelegate>`（0x140176e70）：把 0x38B delegate 装箱（**泄漏**，从不释放），再分配 16B
//!   `{data, vtable}` 作 `pUserData`（同样泄漏），调 `EnableGpuCrashDumps(0x217, 2, 1, cb×4, userData)`，**不看返回值**。
//!   游戏侧只在 `GraphicsDevice` 配置字节（`[cfg+0xbe]==1`）为真时调用（0x140593dbb..0x140593dfc）。
//! * `Drop`（0x140c107b0）= `jmp DisableGpuCrashDumps`（5B 尾调）；0x38B/16B 不在这里释放。
//! * 回调（0x140c106c0 / 0x140c10710 / 0x140c10760）只是把 C 参数转到 delegate vtable `+0x18/+0x20/+0x28`；
//!   vtable（0x1429603f8）槽位 = `dumped`(0x140244790) / `shader_debug_info`(0x140245340) / `description`(0x14003bb80=`ret`)。
//!   第四个 C 回调 `ResolveMarkerCallback` **直接是 0x14003bb80 = 单条 `ret`**（不转发、不写出参）。
//! * `wait_for_status`（0x140c104e0）、`get_shader_debug_info_identifier`（0x140c10620）见各函数。
//!
//! 不能由机器码确定的（I）：0x217/2/1 的头文件名（`GFSDK_Aftermath_Version_API`、WatchedApi=Vulkan、
//! Feature=DeferDebugInfoCallbacks 是按 SDK 头命名的推断，数值是实测）；状态值 2/4 对应的 SDK 枚举名。
use std::{
    ffi::{CStr, c_char, c_void},
    fmt,
    time::Duration,
};

/// `EnableGpuCrashDumps` 第 1 个实参（0x140176e70：`mov ecx,0x217`）。
pub const API_VERSION: u32 = 0x217;
/// 第 2 个实参 2（I：`GpuCrashDumpWatchedApiFlags_Vulkan`）。
pub const WATCHED_APIS: u32 = 2;
/// 第 3 个实参 1（I：`GpuCrashDumpFeatureFlags_DeferDebugInfoCallbacks`）。
pub const FEATURE_FLAGS: u32 = 1;
/// `GFSDK_Aftermath_Result_Success`：原版 `iVar6 != 1` 判失败（0x140c10620）。
pub const RESULT_SUCCESS: i32 = 1;
/// `wait_for_status` 的两个退出值（原版 `cmp al,2 / cmp ecx,4`）；SDK 枚举名未证实。
pub const STATUS_EXIT_A: u8 = 2;
pub const STATUS_EXIT_B: u8 = 4;
/// 轮询间隔（`FUN_140d55980(0, 50_000_000)` = `thread::sleep(50ms)`）。
pub const POLL_INTERVAL: Duration = Duration::from_millis(50);

/// `GFSDK_Aftermath_ShaderDebugInfoIdentifier`（sys.rs:41 derive Default/Debug/Hash/Eq；16B，两个 u64）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ShaderDebugInfoIdentifier(pub [u64; 2]);

/// 回调 `GpuCrashDumpDescriptionCallback` 收到的 `PFN_AddGpuCrashDumpDescription`，原版以
/// `&mut { fn }`（栈上一个函数指针的地址）传给 delegate（0x140c10760：`[rbp-0x10]=rcx; lea rdx,[rbp-0x10]`）。
pub struct GpuCrashDumpDescription {
    add: unsafe extern "C" fn(key: u32, value: *const c_char),
}
impl GpuCrashDumpDescription {
    /// # Safety
    /// `add` 必须是 SDK 提供、仅在回调期间有效的函数指针。
    pub unsafe fn new(add: unsafe extern "C" fn(key: u32, value: *const c_char)) -> Self {
        Self { add }
    }
    pub fn add(&mut self, key: u32, value: &CStr) {
        // SAFETY: 构造时已保证函数指针有效；`value` 是以 NUL 结尾的有效串。
        unsafe { (self.add)(key, value.as_ptr()) }
    }
}

/// 原 `tiny_glade::aftermath::NvAftermathDelegate` 所实现的 trait（方法顺序 = vtable 槽位 +0x18/+0x20/+0x28）。
pub trait NvAftermathDelegate: Send + Sync {
    /// vtable +0x18：GPU 崩溃转储二进制（`pDump,size`）。
    fn dumped(&self, dump: &[u8]);
    /// vtable +0x20：着色器调试信息（`pInfo,size`）。
    fn shader_debug_info(&self, info: &[u8]);
    /// vtable +0x28：崩溃转储描述（原游戏实现是 `ret`，即不添加任何描述）。
    fn description(&self, add: &mut GpuCrashDumpDescription);
}

pub type GpuCrashDumpCb = unsafe extern "C" fn(*const c_void, u32, *mut c_void);
pub type ShaderDebugInfoCb = unsafe extern "C" fn(*const c_void, u32, *mut c_void);
pub type GpuCrashDumpDescriptionCb =
    unsafe extern "C" fn(unsafe extern "C" fn(u32, *const c_char), *mut c_void);
pub type ResolveMarkerCb =
    unsafe extern "C" fn(*const c_void, u32, *mut c_void, *mut *mut c_void, *mut u32);

/// 传给 `EnableGpuCrashDumps` 的四个 C 回调。
#[derive(Clone, Copy)]
pub struct Callbacks {
    pub gpu_crash_dump: GpuCrashDumpCb,
    pub shader_debug_info: ShaderDebugInfoCb,
    pub description: GpuCrashDumpDescriptionCb,
    pub resolve_marker: ResolveMarkerCb,
}

/// 用户数据：原版是 16B `{data, vtable}`（指向泄漏的 delegate 的胖指针），这里同形。
type UserData = &'static dyn NvAftermathDelegate;

unsafe fn delegate<'a>(user: *mut c_void) -> &'a dyn NvAftermathDelegate {
    // SAFETY: 调用方保证 `user` 来自 `Aftermath::new` 泄漏的 `Box<UserData>`。
    unsafe { *user.cast::<UserData>() }
}

unsafe fn bytes<'a>(ptr: *const c_void, size: u32) -> &'a [u8] {
    if ptr.is_null() || size == 0 {
        return &[];
    }
    // SAFETY: SDK 保证回调期间 `ptr` 指向 `size` 字节。
    unsafe { std::slice::from_raw_parts(ptr.cast::<u8>(), size as usize) }
}

/// 0x140c106c0。
///
/// # Safety
/// `user` 必须是 `Aftermath::new` 产生的用户数据；`dump` 指向 `size` 字节。
pub unsafe extern "C" fn gpu_crash_dump_callback(
    dump: *const c_void,
    size: u32,
    user: *mut c_void,
) {
    // SAFETY: 见函数契约。
    unsafe { delegate(user).dumped(bytes(dump, size)) }
}
/// 0x140c10710。
///
/// # Safety
/// 同 [`gpu_crash_dump_callback`]。
pub unsafe extern "C" fn shader_debug_info_callback(
    info: *const c_void,
    size: u32,
    user: *mut c_void,
) {
    // SAFETY: 见函数契约。
    unsafe { delegate(user).shader_debug_info(bytes(info, size)) }
}
/// 0x140c10760。
///
/// # Safety
/// `add` 为 SDK 的 `AddGpuCrashDumpDescription`；`user` 同上。
pub unsafe extern "C" fn gpu_crash_dump_description_callback(
    add: unsafe extern "C" fn(u32, *const c_char),
    user: *mut c_void,
) {
    // SAFETY: 见函数契约。
    unsafe {
        let mut description = GpuCrashDumpDescription::new(add);
        delegate(user).description(&mut description)
    }
}
/// 0x14003bb80：整个函数就是一条 `ret`（ICF 把 437 个空函数折叠到这里），不转发、不写出参。
///
/// # Safety
/// 无前置条件（不解引用任何参数）。
pub unsafe extern "C" fn resolve_marker_callback(
    _marker: *const c_void,
    _size: u32,
    _user: *mut c_void,
    _resolved: *mut *mut c_void,
    _resolved_size: *mut u32,
) {
}

/// 原版导入的四个 SDK 入口。
pub trait AftermathApi {
    /// `GFSDK_Aftermath_EnableGpuCrashDumps(version, watched_apis, flags, cb×4, user_data)`。
    ///
    /// # Safety
    /// 回调须能安全处理 user_data；其指向的数据必须在 SDK 所有回调结束前保持有效。
    unsafe fn enable_gpu_crash_dumps(
        &self,
        version: u32,
        watched_apis: u32,
        flags: u32,
        callbacks: Callbacks,
        user_data: *mut c_void,
    ) -> i32;
    /// `GFSDK_Aftermath_DisableGpuCrashDumps()`。
    fn disable_gpu_crash_dumps(&self) -> i32;
    /// `GFSDK_Aftermath_GetCrashDumpStatus(&mut status)`；返回 `(result, status)`。
    fn crash_dump_status(&self) -> (i32, u32);
    /// `GFSDK_Aftermath_GetShaderDebugInfoIdentifier(version, info, size, &mut id)`。
    fn shader_debug_info_identifier(&self, version: u32, info: &[u8]) -> (i32, [u64; 2]);
}

/// 非 Windows 的占位实现：SDK 不可用，所有调用返回失败码 0，状态恒为 0。
#[derive(Clone, Copy, Debug, Default)]
pub struct UnavailableApi;
impl AftermathApi for UnavailableApi {
    unsafe fn enable_gpu_crash_dumps(
        &self,
        _: u32,
        _: u32,
        _: u32,
        _: Callbacks,
        _: *mut c_void,
    ) -> i32 {
        0
    }
    fn disable_gpu_crash_dumps(&self) -> i32 {
        0
    }
    fn crash_dump_status(&self) -> (i32, u32) {
        (0, 0)
    }
    fn shader_debug_info_identifier(&self, _: u32, _: &[u8]) -> (i32, [u64; 2]) {
        (0, [0; 2])
    }
}

/// 原 `Aftermath`：创建时注册回调，`Drop` 时 `DisableGpuCrashDumps`。原对象是零大小，这里额外保存
/// `Enable` 的返回码（**偏差**：原版丢弃它）。
pub struct Aftermath<A: AftermathApi> {
    api: A,
    enable_result: i32,
}

impl<A: AftermathApi> Aftermath<A> {
    /// 原 `Aftermath::new`（0x140176e70）。delegate 与用户数据按原版**泄漏**。
    pub fn new<D: NvAftermathDelegate + 'static>(delegate: D, api: A) -> Self {
        let leaked: &'static D = Box::leak(Box::new(delegate));
        let user: UserData = leaked;
        let user_data = Box::into_raw(Box::new(user)).cast::<c_void>();
        let callbacks = Callbacks {
            gpu_crash_dump: gpu_crash_dump_callback,
            shader_debug_info: shader_debug_info_callback,
            description: gpu_crash_dump_description_callback,
            resolve_marker: resolve_marker_callback,
        };
        // SAFETY: delegate 与 UserData 都保留到进程结束，回调使用相同的 UserData 类型。
        let enable_result = unsafe {
            api.enable_gpu_crash_dumps(
                API_VERSION,
                WATCHED_APIS,
                FEATURE_FLAGS,
                callbacks,
                user_data,
            )
        };
        Self { api, enable_result }
    }
    pub fn enable_result(&self) -> i32 {
        self.enable_result
    }
    /// `Status::get`：读一次崩溃转储状态（原版把结果码丢弃，只用 `status` 低字节）。
    pub fn status(&self) -> u8 {
        self.api.crash_dump_status().1 as u8
    }
    /// 原 `Status::wait_for_status`（0x140c104e0），用真实 `thread::sleep`。
    pub fn wait_for_status(&self, timeout: Option<Duration>) -> u8 {
        self.wait_for_status_with(timeout, std::thread::sleep)
    }
    /// 可注入 sleep 的版本（便于测试）。
    pub fn wait_for_status_with(
        &self,
        timeout: Option<Duration>,
        mut sleep: impl FnMut(Duration),
    ) -> u8 {
        wait_for_status_with(|| self.status(), timeout, &mut sleep)
    }
    /// 原 `get_shader_debug_info_identifier`（0x140c10620）。
    pub fn shader_debug_info_identifier(
        &self,
        info: &[u8],
    ) -> Result<ShaderDebugInfoIdentifier, AftermathError> {
        let (code, id) = self.api.shader_debug_info_identifier(API_VERSION, info);
        if code != RESULT_SUCCESS {
            return Err(AftermathError {
                call: "GFSDK_Aftermath_GetShaderDebugInfoIdentifier",
                code,
            });
        }
        Ok(ShaderDebugInfoIdentifier(id))
    }
}

impl<A: AftermathApi> Drop for Aftermath<A> {
    /// 0x140c107b0：`jmp GFSDK_Aftermath_DisableGpuCrashDumps`；返回码被丢弃。
    fn drop(&mut self) {
        let _ = self.api.disable_gpu_crash_dumps();
    }
}

/// 原 `wait_for_status` 的循环骨架：先读一次；状态为 2/4 即返回；否则若有超时且 `elapsed >= timeout` 返回
/// 当前状态；睡 50ms，`elapsed += 50ms`（`Duration` 加法溢出 panic "overflow when adding durations"，
/// 串在 0x142b1a5d0），再读。`timeout == None` 对应原 `nanos == 1_000_000_000` 的 niche（无限等待）。
pub fn wait_for_status_with(
    mut status: impl FnMut() -> u8,
    timeout: Option<Duration>,
    sleep: &mut dyn FnMut(Duration),
) -> u8 {
    let mut elapsed = Duration::ZERO;
    let mut current = status();
    while current != STATUS_EXIT_A && current != STATUS_EXIT_B {
        if let Some(limit) = timeout
            && elapsed >= limit
        {
            return current;
        }
        sleep(POLL_INTERVAL);
        elapsed = elapsed
            .checked_add(POLL_INTERVAL)
            .expect("overflow when adding durations");
        current = status();
    }
    current
}

/// `get_shader_debug_info_identifier` 的错误：文案 = `"GFSDK_Aftermath_GetShaderDebugInfoIdentifier: {code:?}\n"`
/// （0x142b1a608 / 0x142b1a636 两段字面量 + 1 个参数）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AftermathError {
    pub call: &'static str,
    pub code: i32,
}
impl fmt::Display for AftermathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {:?}", self.call, self.code)
    }
}
impl std::error::Error for AftermathError {}

/// Windows 绑定：`GFSDK_Aftermath_Lib.x64.dll` 的 4 个导入（与原 EXE 导入表一致）。
/// **未在本机（Linux）编译验证**：仓库没有 Windows 目标；写法遵循 `raw-dylib`（Rust ≥1.71）。
#[cfg(windows)]
pub mod windows_api {
    use super::*;

    #[link(name = "GFSDK_Aftermath_Lib.x64", kind = "raw-dylib")]
    unsafe extern "C" {
        fn GFSDK_Aftermath_EnableGpuCrashDumps(
            api_version: u32,
            watched_apis: u32,
            flags: u32,
            gpu_crash_dump_cb: GpuCrashDumpCb,
            shader_debug_info_cb: ShaderDebugInfoCb,
            description_cb: GpuCrashDumpDescriptionCb,
            resolve_marker_cb: ResolveMarkerCb,
            user_data: *mut c_void,
        ) -> i32;
        fn GFSDK_Aftermath_DisableGpuCrashDumps() -> i32;
        fn GFSDK_Aftermath_GetCrashDumpStatus(status: *mut u32) -> i32;
        fn GFSDK_Aftermath_GetShaderDebugInfoIdentifier(
            api_version: u32,
            info: *const c_void,
            size: u32,
            identifier: *mut [u64; 2],
        ) -> i32;
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct SdkApi;
    impl AftermathApi for SdkApi {
        unsafe fn enable_gpu_crash_dumps(
            &self,
            version: u32,
            watched_apis: u32,
            flags: u32,
            cb: Callbacks,
            user_data: *mut c_void,
        ) -> i32 {
            // SAFETY: 回调与用户数据由 `Aftermath::new` 保证在进程生命周期内有效。
            unsafe {
                GFSDK_Aftermath_EnableGpuCrashDumps(
                    version,
                    watched_apis,
                    flags,
                    cb.gpu_crash_dump,
                    cb.shader_debug_info,
                    cb.description,
                    cb.resolve_marker,
                    user_data,
                )
            }
        }
        fn disable_gpu_crash_dumps(&self) -> i32 {
            // SAFETY: 无参数函数。
            unsafe { GFSDK_Aftermath_DisableGpuCrashDumps() }
        }
        fn crash_dump_status(&self) -> (i32, u32) {
            let mut status = 0u32;
            // SAFETY: 指向栈上有效的 u32。
            let code = unsafe { GFSDK_Aftermath_GetCrashDumpStatus(&mut status) };
            (code, status)
        }
        fn shader_debug_info_identifier(&self, version: u32, info: &[u8]) -> (i32, [u64; 2]) {
            let mut id = [0u64; 2];
            // SAFETY: `info` 是有效切片，`id` 是有效的 16B 输出。
            let code = unsafe {
                GFSDK_Aftermath_GetShaderDebugInfoIdentifier(
                    version,
                    info.as_ptr().cast(),
                    info.len() as u32,
                    &mut id,
                )
            };
            (code, id)
        }
    }
}
