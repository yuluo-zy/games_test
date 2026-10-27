//! AppKit window ownership and event scheduling. All rendering stays in the
//! shared app and runs inside winit's RedrawRequested callback on the main thread.
use super::fixture::Result;
use ash::vk;
use objc::{
    Encode, Encoding, Message,
    rc::{StrongPtr, autoreleasepool},
    runtime::{Class, Object, Sel, YES},
};
use rhapsody::native::surface::Surface;
use rhapsody::native::VulkanInstance;
use std::{
    cell::RefCell,
    ffi::CString,
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    keyboard::{KeyCode, PhysicalKey},
    platform::{
        macos::{ActivationPolicy, EventLoopBuilderExtMacOS},
        run_on_demand::EventLoopExtRunOnDemand,
    },
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::{Window as NativeWindow, WindowBuilder},
};

#[link(name = "QuartzCore", kind = "framework")]
unsafe extern "C" {}

pub const PAUSE: u32 = 1;
pub const NEXT: u32 = 2;
pub const STEP: u32 = 4;
pub const BACKEND: &str = "AppKit / CAMetalLayer / Vulkan";

// Cocoa's CGFloat is f64 on both supported 64-bit macOS architectures. These
// two ABI structures are only used to set the existing CAMetalLayer geometry.
#[repr(C)]
#[derive(Clone, Copy)]
struct Size {
    width: f64,
    height: f64,
}
unsafe impl Encode for Size {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CGSize=dd}") }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Rect {
    origin: Point,
    size: Size,
}
unsafe impl Encode for Rect {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("{CGRect={CGPoint=dd}{CGSize=dd}}") }
    }
}

pub fn required_extensions() -> Vec<CString> {
    ["VK_KHR_surface", "VK_EXT_metal_surface"]
        .map(|name| CString::new(name).expect("static extension"))
        .to_vec()
}

pub struct Window {
    // Lab and its Vulkan surface are destroyed before this host. Drop the native
    // window before its event loop; StrongPtr independently retains its layer.
    window: NativeWindow,
    layer: StrongPtr,
    view: *mut Object,
    event_loop: RefCell<EventLoop<()>>,
}
impl Window {
    pub fn new() -> Result<Self> {
        let mut builder = EventLoopBuilder::new();
        builder
            .with_activation_policy(ActivationPolicy::Regular)
            .with_activate_ignoring_other_apps(true);
        let event_loop = builder.build()?;
        let window = WindowBuilder::new()
            .with_title("Rhapsody Window Lab - starting")
            .with_inner_size(LogicalSize::new(1000.0, 620.0))
            .build(&event_loop)?;
        let view = match window.window_handle()?.as_raw() {
            RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast::<Object>(),
            _ => return Err("winit did not return an AppKit view".into()),
        };
        let layer = autoreleasepool(|| -> Result<StrongPtr> {
            let class =
                Class::get("CAMetalLayer").ok_or("QuartzCore CAMetalLayer is unavailable")?;
            let pointer: *mut Object = unsafe { class.send_message(Sel::register("new"), ()) }?;
            if pointer.is_null() {
                return Err("CAMetalLayer allocation failed".into());
            }
            let layer = unsafe { StrongPtr::new(pointer) };
            unsafe {
                let (): () = (&*view).send_message(Sel::register("setLayer:"), (pointer,))?;
                let (): () = (&*view).send_message(Sel::register("setWantsLayer:"), (YES,))?;
            }
            Ok(layer)
        })?;
        let result = Self {
            event_loop: RefCell::new(event_loop),
            layer,
            view,
            window,
        };
        result.update_layer()?;
        Ok(result)
    }
    fn update_layer(&self) -> Result<()> {
        let physical = self.window.inner_size();
        unsafe {
            let bounds: Rect = (&*self.view).send_message(Sel::register("bounds"), ())?;
            let (): () = (&**self.layer).send_message(Sel::register("setBounds:"), (bounds,))?;
            let (): () = (&**self.layer).send_message(
                Sel::register("setContentsScale:"),
                (self.window.scale_factor(),),
            )?;
            let (): () = (&**self.layer).send_message(
                Sel::register("setDrawableSize:"),
                (Size {
                    width: f64::from(physical.width),
                    height: f64::from(physical.height),
                },),
            )?;
        }
        Ok(())
    }
    pub fn create_surface(&self, instance: Arc<VulkanInstance>) -> Result<Arc<Surface>> {
        self.update_layer()?;
        let loader =
            ash::ext::metal_surface::Instance::new(&instance.entry, instance.raw_instance());
        let info = vk::MetalSurfaceCreateInfoEXT::default().layer((*self.layer).cast());
        let handle = unsafe { loader.create_metal_surface(&info, None) }?;
        Ok(Arc::new(unsafe {
            Surface::from_raw(instance, handle, true)
        }))
    }
    pub fn extent(&self) -> (u32, u32) {
        if self.window.is_minimized() == Some(true) {
            (0, 0)
        } else {
            let size = self.window.inner_size();
            (size.width, size.height)
        }
    }
    pub fn title(&self, title: &str) {
        self.window.set_title(title);
    }
    pub fn resize(&self, width: i32, height: i32) -> Result<()> {
        if width <= 0 || height <= 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "window dimensions must be positive",
            )
            .into());
        }
        let _ = self
            .window
            .request_inner_size(LogicalSize::new(f64::from(width), f64::from(height)));
        self.update_layer()?;
        Ok(())
    }
    pub fn minimize(&self) {
        self.window.set_minimized(true);
    }
    pub fn restore(&self) {
        self.window.set_minimized(false);
        self.window.request_redraw();
    }
    pub fn run(
        &self,
        fps: u32,
        mut frame: impl FnMut(&Self, u32, bool) -> Result<bool>,
    ) -> Result<()> {
        let interval = Duration::from_secs_f64(1.0 / f64::from(fps));
        let mut next_frame = Instant::now();
        let mut keys = 0;
        let mut failure = None;
        let mut finished = false;
        let result = self.event_loop.borrow_mut().run_on_demand(|event, target| {
            if finished {
                target.exit();
                return;
            }
            match event {
                Event::WindowEvent { window_id, event } if window_id == self.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            finished = true;
                            target.exit();
                        }
                        WindowEvent::KeyboardInput { event, .. }
                            if event.state == ElementState::Pressed && !event.repeat =>
                        {
                            keys |= match event.physical_key {
                                PhysicalKey::Code(KeyCode::Space) => PAUSE,
                                PhysicalKey::Code(KeyCode::KeyN) => NEXT,
                                PhysicalKey::Code(KeyCode::ArrowRight) => STEP,
                                PhysicalKey::Code(KeyCode::Escape) => {
                                    finished = true;
                                    target.exit();
                                    0
                                }
                                _ => 0,
                            };
                        }
                        WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                            if let Err(error) = self.update_layer() {
                                failure = Some(error);
                                finished = true;
                                target.exit();
                            } else {
                                self.window.request_redraw();
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            let started = Instant::now();
                            match autoreleasepool(|| {
                                self.update_layer()?;
                                frame(self, std::mem::take(&mut keys), true)
                            }) {
                                Ok(true) => {}
                                Ok(false) => {
                                    finished = true;
                                    target.exit();
                                }
                                Err(error) => {
                                    failure = Some(error);
                                    finished = true;
                                    target.exit();
                                }
                            }
                            next_frame = started + interval;
                        }
                        _ => {}
                    }
                }
                Event::AboutToWait => {
                    let now = Instant::now();
                    if now >= next_frame {
                        if self.window.is_minimized() == Some(true) {
                            // A minimized window has no redraw callbacks. Run
                            // control/script state only so timed restore works;
                            // the common frame body forbids GPU work here.
                            match autoreleasepool(|| frame(self, std::mem::take(&mut keys), false))
                            {
                                Ok(true) => {}
                                Ok(false) => {
                                    finished = true;
                                    target.exit();
                                }
                                Err(error) => {
                                    failure = Some(error);
                                    finished = true;
                                    target.exit();
                                }
                            }
                            next_frame = now + Duration::from_millis(10);
                        } else {
                            self.window.request_redraw();
                        }
                    }
                    if next_frame > now {
                        target.set_control_flow(ControlFlow::WaitUntil(next_frame));
                    } else {
                        target.set_control_flow(ControlFlow::Wait);
                    }
                }
                _ => {}
            }
        });
        if let Some(error) = failure {
            return Err(error);
        }
        result?;
        Ok(())
    }
}
