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
//! buf.truncate(size as usize)                  // O: 0x1403ac625..644, min(size, capacity)
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
        let path = decode_monitor_profile_path(buf, size)?;
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

/// 原 GetICMProfileA 的输出长度包含结尾 NUL；原版先按返回长度截断，不能解码整块零缓冲。
#[cfg(any(windows, test))]
fn decode_monitor_profile_path(mut buffer: Vec<u8>, returned_len: u32) -> anyhow::Result<String> {
    use anyhow::Context;
    // Vec::truncate 与原 min(returned_len, 0x800) 一致；超出容量不能扩充或读越界。
    buffer.truncate(returned_len as usize);
    let path = std::ffi::CString::from_vec_with_nul(buffer)
        .context("The ICC profile could not be obtained")?;
    path.into_string().context("Invalid ICC profile path")
}

#[cfg(test)]
mod profile_length_tests {
    use super::decode_monitor_profile_path;

    #[test]
    fn icc_profile_uses_returned_length_instead_of_zero_filled_capacity() {
        let path = b"C:\\Windows\\System32\\spool\\drivers\\color\\display.icm\0";
        let mut buffer = vec![0; 0x800];
        buffer[..path.len()].copy_from_slice(path);
        assert!(std::ffi::CString::from_vec_with_nul(buffer.clone()).is_err());
        assert_eq!(decode_monitor_profile_path(buffer, path.len() as u32).unwrap().as_bytes(), &path[..path.len() - 1]);
    }

    #[test]
    fn icc_profile_length_keeps_original_missing_nul_and_interior_nul_errors() {
        for (bytes, len) in [(b"abc\0".to_vec(), 3), (b"a\0b\0".to_vec(), 4), (b"abc\0".to_vec(), 0)] {
            let error = decode_monitor_profile_path(bytes, len).unwrap_err();
            assert_eq!(error.to_string(), "The ICC profile could not be obtained");
        }
    }

    #[test]
    fn icc_profile_over_capacity_length_is_clamped_without_reading_out_of_bounds() {
        assert_eq!(decode_monitor_profile_path(b"profile.icm\0".to_vec(), u32::MAX).unwrap(), "profile.icm");
    }

    #[test]
    fn icc_profile_invalid_utf8_keeps_path_error_context() {
        let error = decode_monitor_profile_path(vec![0xff, 0], 2).unwrap_err();
        assert_eq!(error.to_string(), "Invalid ICC profile path");
    }
}
