//! Minimal single-window host. No renderer state is accessed from the window callback.
use std::{
    ffi::c_void,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

type Handle = *mut c_void;
static CLOSED: AtomicBool = AtomicBool::new(false);
static KEYS: AtomicU32 = AtomicU32::new(0);
pub const PAUSE: u32 = 1;
pub const NEXT: u32 = 2;
pub const STEP: u32 = 4;

#[repr(C)]
#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}
#[repr(C)]
#[derive(Default)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
#[repr(C)]
#[derive(Default)]
struct Message {
    hwnd: Handle,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    point: Point,
    private: u32,
}
#[repr(C)]
struct WindowClass {
    style: u32,
    proc: Option<unsafe extern "system" fn(Handle, u32, usize, isize) -> isize>,
    class_extra: i32,
    window_extra: i32,
    instance: Handle,
    icon: Handle,
    cursor: Handle,
    background: Handle,
    menu: *const u16,
    name: *const u16,
}
#[link(name = "user32")]
unsafe extern "system" {
    fn RegisterClassW(class: *const WindowClass) -> u16;
    fn UnregisterClassW(name: *const u16, instance: Handle) -> i32;
    fn CreateWindowExW(
        ex: u32,
        class: *const u16,
        title: *const u16,
        style: u32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        parent: Handle,
        menu: Handle,
        instance: Handle,
        param: Handle,
    ) -> Handle;
    fn DefWindowProcW(hwnd: Handle, msg: u32, w: usize, l: isize) -> isize;
    fn DestroyWindow(hwnd: Handle) -> i32;
    fn ShowWindow(hwnd: Handle, command: i32) -> i32;
    fn PeekMessageW(msg: *mut Message, hwnd: Handle, min: u32, max: u32, remove: u32) -> i32;
    fn TranslateMessage(msg: *const Message) -> i32;
    fn DispatchMessageW(msg: *const Message) -> isize;
    fn GetClientRect(hwnd: Handle, rect: *mut Rect) -> i32;
    fn IsIconic(hwnd: Handle) -> i32;
    fn SetWindowTextW(hwnd: Handle, text: *const u16) -> i32;
    fn SetWindowPos(hwnd: Handle, after: Handle, x: i32, y: i32, w: i32, h: i32, flags: u32)
    -> i32;
    fn LoadCursorW(instance: Handle, name: *const u16) -> Handle;
    fn SetProcessDPIAware() -> i32;
}
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(name: *const u16) -> Handle;
}

unsafe extern "system" fn callback(hwnd: Handle, message: u32, w: usize, l: isize) -> isize {
    match message {
        0x0010 => {
            CLOSED.store(true, Ordering::Relaxed);
            0
        } // WM_CLOSE: defer destruction until Vulkan teardown.
        0x0014 => 1, // WM_ERASEBKGND: Vulkan owns the client area.
        0x0100 => {
            if l & (1 << 30) == 0 {
                let key = match w {
                    0x20 => PAUSE,
                    0x4e => NEXT,
                    0x27 => STEP,
                    0x1b => {
                        CLOSED.store(true, Ordering::Relaxed);
                        0
                    }
                    _ => 0,
                };
                KEYS.fetch_or(key, Ordering::Relaxed);
            }
            0
        }
        _ => unsafe { DefWindowProcW(hwnd, message, w, l) },
    }
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}

pub struct Window {
    pub handle: Handle,
    pub instance: Handle,
    class: Vec<u16>,
}
impl Window {
    pub fn new() -> std::io::Result<Self> {
        CLOSED.store(false, Ordering::Relaxed);
        KEYS.store(0, Ordering::Relaxed);
        let null = std::ptr::null_mut();
        let class = wide("RhapsodyWindowLabV0");
        unsafe {
            SetProcessDPIAware();
            let instance = GetModuleHandleW(std::ptr::null());
            let wc = WindowClass {
                style: 3,
                proc: Some(callback),
                class_extra: 0,
                window_extra: 0,
                instance,
                icon: null,
                cursor: LoadCursorW(null, 32512usize as *const u16),
                background: null,
                menu: std::ptr::null(),
                name: class.as_ptr(),
            };
            if RegisterClassW(&wc) == 0 {
                return Err(std::io::Error::last_os_error());
            }
            let handle = CreateWindowExW(
                0,
                class.as_ptr(),
                wide("Rhapsody Window Lab - starting").as_ptr(),
                0x00cf0000,
                i32::MIN,
                i32::MIN,
                1000,
                620,
                null,
                null,
                instance,
                null,
            );
            if handle.is_null() {
                let error = std::io::Error::last_os_error();
                UnregisterClassW(class.as_ptr(), instance);
                return Err(error);
            }
            ShowWindow(handle, 5);
            Ok(Self {
                handle,
                instance,
                class,
            })
        }
    }
    pub fn poll(&self) -> Option<u32> {
        let mut msg = Message::default();
        unsafe {
            while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, 1) != 0 {
                if msg.message == 0x0012 {
                    CLOSED.store(true, Ordering::Relaxed);
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
        (!CLOSED.load(Ordering::Relaxed)).then(|| KEYS.swap(0, Ordering::Relaxed))
    }
    pub fn extent(&self) -> (u32, u32) {
        let mut rect = Rect::default();
        unsafe {
            if IsIconic(self.handle) != 0 || GetClientRect(self.handle, &mut rect) == 0 {
                return (0, 0);
            }
        }
        (
            (rect.right - rect.left).max(0) as u32,
            (rect.bottom - rect.top).max(0) as u32,
        )
    }
    pub fn title(&self, title: &str) {
        unsafe {
            SetWindowTextW(self.handle, wide(title).as_ptr());
        }
    }
    pub fn resize(&self, width: i32, height: i32) -> std::io::Result<()> {
        if unsafe {
            SetWindowPos(
                self.handle,
                std::ptr::null_mut(),
                0,
                0,
                width,
                height,
                0x0002 | 0x0004 | 0x0010,
            )
        } == 0
        {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
    pub fn minimize(&self) {
        unsafe {
            ShowWindow(self.handle, 6);
        }
    }
    pub fn restore(&self) {
        unsafe {
            ShowWindow(self.handle, 9);
        }
    }
}
impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.handle);
            UnregisterClassW(self.class.as_ptr(), self.instance);
        }
    }
}
