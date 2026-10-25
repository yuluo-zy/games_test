//! B `crates/tiny-glade/src/icc.rs`（监视器 ICC 配置文件名）。
//!
//! 恢复依据（fresh Ghidra shared 2026-10-17，证据在
//! `recovery/agent-team/work/restorer-tinyglade-C/`）：
//! `tiny_glade::icc::get_monitor_icc_profile` `0x1403ac400`（1,443 B）。
//!
//! ## 逐语句复原（Windows 分支）
//! ```text
//! name = monitor.get_monitor_info().unwrap() → decode_wide → to_string_lossy → String
//! driver = CString::new(name).unwrap()            // 名字含 NUL → panic（O）
//! hdc = CreateICA(driver, null, null, null)
//! size = 0x800; buf = alloc_zeroed(0x800)
//! if GetICMProfileA(hdc, &size, buf) == 0:
//!     err = anyhow!("Failed to obtain the monitor's ICC profile: {}", last_os_error)
//!     DeleteDC(hdc); return Err(err)
//! path = CString::from_vec_with_nul(buf)          // context: "The ICC profile could not be obtained"
//! path = path.into_string()                       // context: "Invalid ICC profile path"
//! DeleteDC(hdc)
//! Ok(path)
//! ```
//! 上下文文案取自 `.rdata`：`0x1429cb930` "The ICC profile could not be
//! obtained"（37 B）与 `0x1429cb968` "Invalid ICC profile path"（25 B）；
//! GetICMProfile 失败文案 `0x1429a7ce0` "Failed to obtain the monitor's ICC
//! profile: "。原版在 **两个 CString 错误路径上不调用 DeleteDC**（DC 泄漏），
//! 本实现保持同一行为。
//!
//! ## 平台缝
//! 原函数只在 Windows winit 后端存在。非 Windows 构建保留同签名并返回明确的
//! 平台错误（平台胶水，不是业务近似）。

/// `tiny_glade::icc::get_monitor_icc_profile` `0x1403ac400`。
#[cfg(windows)]
pub fn get_monitor_icc_profile(
    monitor: &winit::monitor::MonitorHandle,
) -> anyhow::Result<String> {
    use anyhow::Context;
    use std::ffi::CString;
    use winapi::um::wingdi::{CreateICA, DeleteDC, GetICMProfileA};

    // Windows winit 后端的 `get_monitor_info()` 失败 → unwrap panic（O）；
    // 公开 API 的 `name()` 为同一监视器设备名。
    let name = monitor.name().unwrap();
    // 反汇编：名字中查到 NUL → `core::result::unwrap_failed`（原版 unwrap）。
    let driver = CString::new(name).unwrap();
    unsafe {
        let hdc = CreateICA(
            driver.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            std::ptr::null_mut(),
        );
        let mut size: u32 = 0x800;
        let mut buf = vec![0u8; 0x800];
        let ok = GetICMProfileA(hdc, &mut size, buf.as_mut_ptr() as *mut i8);
        if ok == 0 {
            let err = anyhow::anyhow!(
                "Failed to obtain the monitor's ICC profile: {}",
                std::io::Error::last_os_error()
            );
            DeleteDC(hdc);
            return Err(err);
        }
        // 原版错误路径不释放 DC（保持原行为）。
        let cstr =
            CString::from_vec_with_nul(buf).context("The ICC profile could not be obtained")?;
        let path = cstr.into_string().context("Invalid ICC profile path")?;
        DeleteDC(hdc);
        Ok(path)
    }
}

/// 非 Windows 平台：原函数不存在于该 winit 后端。
#[cfg(not(windows))]
pub fn get_monitor_icc_profile(
    _monitor: &winit::monitor::MonitorHandle,
) -> anyhow::Result<String> {
    anyhow::bail!("monitor ICC profiles are only available on Windows (platform glue)")
}
