//! DXGI adapter enumeration (names, LUIDs and vendor IDs), used to match
//! Vulkan physical devices with Windows adapters.
//!
//! * `CreateDXGIFactory(&IID_IDXGIFactory)`; failure is an error.
//! * `IDXGIFactory::EnumAdapters(index)` until it returns any error.
//! * `IDXGIAdapter::GetDesc` per adapter; a failed description skips only
//!   that adapter.
//! * The name is `Description` up to its first NUL, decoded lossily.
//!
//! Windows uses [`SystemDxgi`] via `raw-dylib`; other platforms and tests
//! inject a [`DxgiSource`].

/// Win32 `DXGI_ADAPTER_DESC` (0x130 bytes on 64-bit targets).
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

/// One enumerated adapter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DxgiAdapter {
    pub name: String,
    /// `(HighPart as i64 as u64) << 32 | LowPart`.
    pub luid: u64,
    pub vendor_id: u32,
}

impl DxgiAdapter {
    /// Convert a `GetDesc` result.
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("CreateDXGIFactory failed: HRESULT {:#010x}", *.hresult as u32)]
pub struct DxgiError {
    pub hresult: i32,
}

/// 一次 `EnumAdapters(index)` + `GetDesc` 的结果。
// Carries `DXGI_ADAPTER_DESC` by value so the probe stays Copy without boxing.
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

/// Enumerate all adapters whose description can be read.
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
        // Overflowing a `u32` adapter index is not a realistic state. If it ever
        // happened it would be reported as E_BOUNDS, and `DxgiError`'s text
        // (which names `CreateDXGIFactory`) would then mislabel it.
        index = index.checked_add(1).ok_or(DxgiError { hresult: 0x8000000Bu32 as i32 })?;
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
    /// `IID_IDXGIFactory` `{7b7166ec-21c7-44ae-b21a-c9ae321ae369}`.
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
            // SAFETY: valid output pointer and the IDXGIFactory IID.
            let hr = unsafe { CreateDXGIFactory(&IID_IDXGIFACTORY, &mut factory) };
            if hr < 0 || factory.is_null() {
                return Err(DxgiError { hresult: hr });
            }
            if let Some(previous) = self.factory.replace(factory) {
                // SAFETY: the previous owned COM reference is superseded.
                unsafe { slot::<Release>(previous, 2)(previous); }
            }
            Ok(())
        }
        fn probe(&mut self, index: u32) -> AdapterProbe {
            let Some(factory) = self.factory else {
                return AdapterProbe::End;
            };
            let mut adapter = std::ptr::null_mut();
            // SAFETY: vtable slot 7 is IDXGIFactory::EnumAdapters.
            let hr = unsafe { slot::<EnumAdapters>(factory, 7)(factory, index, &mut adapter) };
            if hr < 0 || adapter.is_null() {
                return AdapterProbe::End;
            }
            // SAFETY: `DxgiAdapterDesc` is repr(C) and valid when zeroed; slot 8
            // is IDXGIAdapter::GetDesc.
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

#[cfg(test)]
mod error_display_tests {
    use super::*;

    #[test]
    fn display_text_is_stable() {
        assert_eq!(
            DxgiError { hresult: 0x887a0004_u32 as i32 }.to_string(),
            "CreateDXGIFactory failed: HRESULT 0x887a0004"
        );
    }
}
