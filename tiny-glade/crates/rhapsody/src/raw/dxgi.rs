//! 原 `dxgi-enumerate-gpus`（本地 crate，Cargo.lock 记 0.1.0、无 source；依赖 anyhow + windows 0.37）。
//!
//! `enumerate`（0x1418b29c0）读出的事实（机器码为准）：
//! * `CreateDXGIFactory(&IID_IDXGIFactory)`（导入表 `dxgi.dll!CreateDXGIFactory`；IID 字节 @0x142d0324c =
//!   `{7b7166ec-21c7-44ae-b21a-c9ae321ae369}`）。失败 → `Err`（`windows::core::Error::from(HRESULT)`，
//!   上下文串 `"CreateDXGIFactory"` @0x142d0325c，17 字节）。
//! * 循环 `IDXGIFactory::EnumAdapters(index)`（vtable `+0x38` = 槽 7；S_PUB32 0x14213b8e0）。**任何**错误都结束循环并返回
//!   `Ok`（不是只认 `DXGI_ERROR_NOT_FOUND`）。**纠正**旧文档里的 `EnumAdapters1/GetDesc1`：vtable 偏移、
//!   0x130B 拷贝长度（`DXGI_ADAPTER_DESC`，无 `Flags` 尾字段）和 S_PUB32 都证实是旧版接口。
//! * 每个适配器 `IDXGIAdapter::GetDesc`（vtable `+0x40` = 槽 8；0x14213b950）。**GetDesc 失败只跳过该适配器**，继续下一个。
//! * 名字：`Description` 的 128 个 WCHAR 扫到第一个 0（没有 0 则 128），UTF-16 解码（0x14213b9d0 = `Map::fold`
//!   收集 char；解码失败的代理项处理按 `String::from_utf16_lossy`，I）。
//! * 40B 元素 `DxgiAdapter`：`name: String @0x00/0x08/0x10`（写入顺序 ptr,len,cap）、`luid: u64 @0x18`
//!   （`(HighPart as i64 as u64) << 32 | LowPart`，来自 desc+0x128/+0x12c）、`vendor_id: u32 @0x20`（desc+0x100）；
//!   `+0x24` 从不写入（补齐）。**DeviceId / SubSysId / Revision / 内存大小都不进元素**。
//!
//! DXGI 是 Windows API：Windows 下 [`SystemDxgi`] 经 `raw-dylib` 调用（**未在本机编译验证**，仓库无 Windows 目标）；
//! 其他平台通过 [`DxgiSource`] 注入。
use std::fmt;

/// `DXGI_ADAPTER_DESC`（0x130B，原版 `memcpy(..., 0x130)`）。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DxgiAdapterDesc {
    pub description: [u16; 128],
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub luid_low_part: u32,
    pub luid_high_part: i32,
}
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::size_of::<DxgiAdapterDesc>() == 0x130);

/// 原 40B 元素（`Vec<DxgiAdapter>` 的 stride = 0x28）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DxgiAdapter {
    pub name: String,
    pub luid: u64,
    pub vendor_id: u32,
}
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::size_of::<DxgiAdapter>() == 0x28);

impl DxgiAdapter {
    /// 0x1418b2ae1..0x1418b2c71：`GetDesc` 结果 → 元素。
    pub fn from_desc(desc: &DxgiAdapterDesc) -> Self {
        let len = desc
            .description
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(desc.description.len());
        Self {
            name: String::from_utf16_lossy(&desc.description[..len]),
            luid: ((desc.luid_high_part as i64 as u64) << 32) | u64::from(desc.luid_low_part),
            vendor_id: desc.vendor_id,
        }
    }
}

/// `CreateDXGIFactory` 失败（带 `"CreateDXGIFactory"` 上下文）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DxgiError {
    pub hresult: i32,
}
impl fmt::Display for DxgiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CreateDXGIFactory: HRESULT {:#010x}",
            self.hresult as u32
        )
    }
}
impl std::error::Error for DxgiError {}

/// 一次 `EnumAdapters(index)` + `GetDesc` 的结果。
// 镜像原 `GetDesc` 返回的 `DXGI_ADAPTER_DESC` 按值承载，保持 Copy 不装箱。
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Copy, Debug)]
pub enum AdapterProbe {
    /// `EnumAdapters` 返回任何错误：结束枚举。
    End,
    /// `GetDesc` 失败：跳过该适配器。
    DescFailed,
    Desc(DxgiAdapterDesc),
}

/// 系统 DXGI 的抽象（Windows 用 [`SystemDxgi`]，其他平台/测试注入）。
pub trait DxgiSource {
    /// `CreateDXGIFactory`。
    fn open(&mut self) -> Result<(), DxgiError>;
    fn probe(&mut self, index: u32) -> AdapterProbe;
}

/// 原 `enumerate`（0x1418b29c0）。
pub fn enumerate(source: &mut impl DxgiSource) -> Result<Vec<DxgiAdapter>, DxgiError> {
    source.open()?;
    let mut adapters = Vec::new();
    let mut index = 0u32;
    loop {
        match source.probe(index) {
            AdapterProbe::End => return Ok(adapters),
            AdapterProbe::DescFailed => {}
            AdapterProbe::Desc(desc) => adapters.push(DxgiAdapter::from_desc(&desc)),
        }
        index += 1;
    }
}

/// 固定适配器表的注入实现（非 Windows 平台 / 测试 / 调用方自带清单）。
#[derive(Clone, Debug, Default)]
pub struct FixedDxgi {
    pub factory_error: Option<DxgiError>,
    pub adapters: Vec<Option<DxgiAdapterDesc>>,
}
impl DxgiSource for FixedDxgi {
    fn open(&mut self) -> Result<(), DxgiError> {
        self.factory_error.map_or(Ok(()), Err)
    }
    fn probe(&mut self, index: u32) -> AdapterProbe {
        match self.adapters.get(index as usize) {
            None => AdapterProbe::End,
            Some(None) => AdapterProbe::DescFailed,
            Some(Some(desc)) => AdapterProbe::Desc(*desc),
        }
    }
}

#[cfg(windows)]
mod windows_impl {
    use super::*;
    use std::ffi::c_void;

    #[repr(C)]
    struct Guid {
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    }
    /// `IID_IDXGIFactory`，与 0x142d0324c 的 16 字节一致。
    const IID_IDXGIFACTORY: Guid = Guid {
        data1: 0x7b7166ec,
        data2: 0x21c7,
        data3: 0x44ae,
        data4: [0xb2, 0x1a, 0xc9, 0xae, 0x32, 0x1a, 0xe3, 0x69],
    };

    #[link(name = "dxgi", kind = "raw-dylib")]
    unsafe extern "system" {
        fn CreateDXGIFactory(riid: *const Guid, factory: *mut *mut c_void) -> i32;
    }

    type Release = unsafe extern "system" fn(*mut c_void) -> u32;
    type EnumAdapters = unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> i32;
    type GetDesc = unsafe extern "system" fn(*mut c_void, *mut DxgiAdapterDesc) -> i32;

    unsafe fn slot<T: Copy>(com: *mut c_void, index: usize) -> T {
        // SAFETY: COM 对象首字段是 vtable 指针；`index` 是接口约定的槽位。
        unsafe {
            let vtable = *(com as *const *const usize);
            std::mem::transmute_copy::<usize, T>(&*vtable.add(index))
        }
    }

    /// 真实 DXGI（`dxgi.dll`）。
    #[derive(Default)]
    pub struct SystemDxgi {
        factory: Option<*mut c_void>,
    }
    impl DxgiSource for SystemDxgi {
        fn open(&mut self) -> Result<(), DxgiError> {
            let mut factory = std::ptr::null_mut();
            // SAFETY: 输出指针有效；IID 与原版一致。
            let hr = unsafe { CreateDXGIFactory(&IID_IDXGIFACTORY, &mut factory) };
            if hr < 0 || factory.is_null() {
                return Err(DxgiError { hresult: hr });
            }
            self.factory = Some(factory);
            Ok(())
        }
        fn probe(&mut self, index: u32) -> AdapterProbe {
            let Some(factory) = self.factory else {
                return AdapterProbe::End;
            };
            let mut adapter = std::ptr::null_mut();
            // SAFETY: vtable 槽 7 = IDXGIFactory::EnumAdapters（原版 +0x38）。
            let hr = unsafe { slot::<EnumAdapters>(factory, 7)(factory, index, &mut adapter) };
            if hr < 0 || adapter.is_null() {
                return AdapterProbe::End;
            }
            // SAFETY: `DxgiAdapterDesc` 为 repr(C) 且全零合法；槽 8 = IDXGIAdapter::GetDesc（原版 +0x40）。
            unsafe {
                let mut desc: DxgiAdapterDesc = std::mem::zeroed();
                let hr = slot::<GetDesc>(adapter, 8)(adapter, &mut desc);
                slot::<Release>(adapter, 2)(adapter);
                if hr < 0 {
                    AdapterProbe::DescFailed
                } else {
                    AdapterProbe::Desc(desc)
                }
            }
        }
    }
    impl Drop for SystemDxgi {
        fn drop(&mut self) {
            if let Some(factory) = self.factory.take() {
                // SAFETY: 槽 2 = IUnknown::Release。
                unsafe { slot::<Release>(factory, 2)(factory) };
            }
        }
    }
}
#[cfg(windows)]
pub use windows_impl::SystemDxgi;
