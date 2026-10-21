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


/// 原 Moved 的 monitor/rate 分支；泛型仅用于无需构造 OS 句柄的精确状态测试。
#[cfg(any(windows, test))]
struct MovedMonitorState<M> {
    monitor: Option<M>,
    dirty: bool,
}

#[cfg(any(windows, test))]
impl<M: Eq> MovedMonitorState<M> {
    fn on_moved(
        &mut self,
        current: M,
        refresh_rate: impl Fn(&M) -> Option<u32>,
        mut send: impl FnMut(crate::event_loop::OsWindowEvent) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.monitor.as_ref() == Some(&current) {
            return Ok(());
        }
        send(crate::event_loop::OsWindowEvent::MonitorChanged)?;
        if self.monitor.as_ref().is_some_and(|old| refresh_rate(old) != refresh_rate(&current)) {
            // 原版仅在刷新率改变时更新缓存；同刷新率跨屏仍保旧 monitor，不能以新身份节流。
            self.monitor = Some(current);
            send(crate::event_loop::OsWindowEvent::SurfaceStale)?;
        }
        self.dirty = true;
        Ok(())
    }

    fn take_dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }
}

/// 原 OS 查询的主线程适配；Weak 身份不延长旧 App 的纹理/设备寿命。
pub struct MonitorIccTracker {
    #[cfg(windows)]
    moved: MovedMonitorState<winit::monitor::MonitorHandle>,
    #[cfg(windows)]
    owner: Option<std::sync::Weak<
        dyn country_core::render::load_texture::boundary::TextureDevice + Send + Sync,
    >>,
}

impl MonitorIccTracker {
    pub fn new(window: &winit::window::Window) -> Self {
        #[cfg(windows)]
        {
            // 原 run_event_loop 初始化真实监视器，并令第一次 OS 批次查询 dirty。
            Self {
                moved: MovedMonitorState { monitor: window.current_monitor(), dirty: true },
                owner: None,
            }
        }
        #[cfg(not(windows))]
        {
            let _ = window;
            Self {}
        }
    }

    #[cfg(windows)]
    pub fn on_moved(
        &mut self,
        window: &winit::window::Window,
        send: impl FnMut(crate::event_loop::OsWindowEvent) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        // Windows winit 正常返回实际 MonitorHandle；平台返回 None 时不制造假句柄。
        if let Some(current) = window.current_monitor() {
            self.moved.on_moved(current, |monitor| monitor.refresh_rate_millihertz(), send)?;
        }
        Ok(())
    }

    pub fn publish_pending(&mut self, world: &mut bevy_ecs::world::World) -> anyhow::Result<()> {
        #[cfg(windows)]
        {
            use anyhow::Context;
            let owner = world.get_resource::<country_core::systems::backend::icc::IccTextureDevice>()
                .context("ICC monitor query requires this App's texture namespace")?;
            // Weak 保持控制块身份，旧对象地址不能被另一个 Arc 重用；普通帧不增减引用计数。
            let owner_changed = !self.owner.as_ref().is_some_and(|previous| {
                std::ptr::addr_eq(previous.as_ptr(), std::sync::Arc::as_ptr(&owner.0))
            });
            if !self.moved.take_dirty() && !owner_changed {
                return Ok(());
            }
            if owner_changed {
                self.owner = Some(std::sync::Arc::downgrade(&owner.0));
            }
            // 原版先消费 dirty 再查询，失败不每帧重试；查询原缓存 monitor，不替换原同率分支。
            if let Some(monitor) = &self.moved.monitor {
                publish_monitor_profile_result(world, get_monitor_icc_profile(monitor))?;
            }
        }
        #[cfg(not(windows))]
        {
            // Win32 自动来源不适用于 Mac；sRGB CAMetalLayer 由系统完成显示转换。
            // 不伪造 None/ICC 事件；显式核心 ICC 事件仍执行完整原算法。
            let _ = world;
        }
        Ok(())
    }
}

#[cfg(any(windows, test))]
fn publish_monitor_profile_result(
    world: &mut bevy_ecs::world::World,
    result: anyhow::Result<String>,
) -> anyhow::Result<()> {
    match result {
        Ok(path) => {
            use anyhow::Context;
            world.send_event(country_core::systems::backend::icc::IccProfileChanged(path.into()))
                .context("ICC monitor event owner is not installed in the current App")?;
        }
        // 原查询失败只警告，不发事件，不清原 active profile；送达坏路径才由核心清旧值。
        Err(error) => log::warn!(target: "tiny_glade::event_loop", "{error:#}"),
    }
    Ok(())
}

#[cfg(test)]
mod monitor_publish_tests {
    use super::publish_monitor_profile_result;
    use bevy_ecs::{event::Events, world::World};
    use country_core::systems::backend::icc::{ActiveIccProfile, IccProfileChanged};

    #[test]
    fn icc_query_failure_preserves_existing_app_resource_and_does_not_emit_event() {
        let mut world = World::new();
        world.insert_resource(ActiveIccProfile::default());
        world.init_resource::<Events<IccProfileChanged>>();
        let original = world.resource::<ActiveIccProfile>() as *const ActiveIccProfile;
        publish_monitor_profile_result(&mut world, Err(anyhow::anyhow!("real OS query failed"))).unwrap();
        assert_eq!(world.resource::<ActiveIccProfile>() as *const ActiveIccProfile, original);
        assert!(world.resource::<Events<IccProfileChanged>>().is_empty());
    }

    #[test]
    fn icc_successful_query_publishes_exact_path_only_to_current_world() {
        let mut world = World::new();
        let mut other = World::new();
        world.init_resource::<Events<IccProfileChanged>>();
        other.init_resource::<Events<IccProfileChanged>>();
        publish_monitor_profile_result(&mut world, Ok("C:\\display.icm".into())).unwrap();
        let events: Vec<_> = world.resource_mut::<Events<IccProfileChanged>>().drain().collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].0, std::path::PathBuf::from("C:\\display.icm"));
        assert!(other.resource::<Events<IccProfileChanged>>().is_empty());
    }

    #[test]
    fn icc_successful_query_requires_real_event_owner() {
        let mut world = World::new();
        assert!(publish_monitor_profile_result(&mut world, Ok("profile.icm".into())).is_err());
    }
}


#[cfg(test)]
mod monitor_dirty_tests {
    use super::MovedMonitorState;
    use crate::event_loop::OsWindowEvent;
    use std::cell::Cell;

    fn tag(event: OsWindowEvent) -> u8 {
        match event {
            OsWindowEvent::MonitorChanged => 5,
            OsWindowEvent::SurfaceStale => 6,
            _ => panic!("unexpected monitor notification"),
        }
    }

    #[test]
    fn same_monitor_does_not_read_rate_emit_or_dirty() {
        let mut state = MovedMonitorState { monitor: Some(1), dirty: false };
        state.on_moved(1, |_| panic!("same monitor must not read rate"), |_| {
            panic!("same monitor must not notify")
        }).unwrap();
        assert!(!state.take_dirty());
    }

    #[test]
    fn equal_rate_cross_monitor_keeps_original_cache_and_repeats_notifications() {
        for rate in [None, Some(60_000)] {
            let mut state = MovedMonitorState { monitor: Some(1), dirty: false };
            let mut events = Vec::new();
            for _ in 0..2 {
                state.on_moved(2, |_| rate, |event| { events.push(tag(event)); Ok(()) }).unwrap();
                assert_eq!(state.monitor, Some(1));
                assert!(state.take_dirty());
                assert!(!state.take_dirty());
            }
            assert_eq!(events, [5, 5]);
        }
    }

    #[test]
    fn different_optional_rates_replace_cache_then_send_stale_once() {
        for (old, new) in [(None, Some(60_000)), (Some(60_000), None), (Some(60_000), Some(144_000))] {
            let mut state = MovedMonitorState { monitor: Some(1), dirty: false };
            let mut events = Vec::new();
            state.on_moved(2, |monitor| if *monitor == 1 { old } else { new }, |event| {
                events.push(tag(event)); Ok(())
            }).unwrap();
            assert_eq!(events, [5, 6]);
            assert_eq!(state.monitor, Some(2));
            assert!(state.take_dirty());
            state.on_moved(2, |_| panic!("cached monitor must skip rate"), |_| panic!("duplicate stale")).unwrap();
        }
    }

    #[test]
    fn absent_original_cache_is_not_filled_by_a_move() {
        let mut state = MovedMonitorState { monitor: None, dirty: true };
        assert!(state.take_dirty());
        let mut events = Vec::new();
        state.on_moved(2, |_| panic!("no cached rate to compare"), |event| { events.push(tag(event)); Ok(()) }).unwrap();
        assert_eq!(events, [5]);
        assert_eq!(state.monitor, None);
        assert!(state.take_dirty());
    }

    #[test]
    fn first_send_failure_keeps_cache_and_dirty_before_any_rate_read() {
        let mut state = MovedMonitorState { monitor: Some(1), dirty: false };
        let error = state.on_moved(2, |_| panic!("failed first send must not read rate"), |event| {
            assert_eq!(tag(event), 5); anyhow::bail!("first send failed")
        }).unwrap_err();
        assert_eq!(error.to_string(), "first send failed");
        assert_eq!(state.monitor, Some(1));
        assert!(!state.take_dirty());
    }

    #[test]
    fn second_send_failure_keeps_already_replaced_cache_without_marking_dirty() {
        let mut state = MovedMonitorState { monitor: Some(1), dirty: false };
        let calls = Cell::new(0);
        let error = state.on_moved(2, |monitor| Some(*monitor * 60_000), |event| {
            calls.set(calls.get() + 1);
            match tag(event) { 5 => Ok(()), 6 => anyhow::bail!("second send failed"), _ => unreachable!() }
        }).unwrap_err();
        assert_eq!(error.to_string(), "second send failed");
        assert_eq!(calls.get(), 2);
        assert_eq!(state.monitor, Some(2));
        assert!(!state.take_dirty());
    }
}
