#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use objc::{
        Encode, Encoding, Message,
        rc::{StrongPtr, autoreleasepool},
        runtime::{Class, Object, Sel, YES},
    };
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    #[link(name = "QuartzCore", kind = "framework")]
    unsafe extern "C" {}
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
    pub struct PlatformSurface {
        layer: StrongPtr,
        view: *mut Object,
        window: Arc<winit::window::Window>,
    }
    impl PlatformSurface {
        pub fn new(window: &Arc<winit::window::Window>) -> Result<Self> {
            let view = match window.window_handle()?.as_raw() {
                RawWindowHandle::AppKit(handle) => handle.ns_view.as_ptr().cast::<Object>(),
                _ => anyhow::bail!("winit did not return an AppKit view"),
            };
            let layer = autoreleasepool(|| -> Result<StrongPtr> {
                let class =
                    Class::get("CAMetalLayer").context("QuartzCore CAMetalLayer is unavailable")?;
                let pointer: *mut Object = unsafe { class.send_message(Sel::register("new"), ()) }?;
                if pointer.is_null() {
                    anyhow::bail!("CAMetalLayer allocation failed");
                }
                let layer = unsafe { StrongPtr::new(pointer) };
                unsafe {
                    let (): () = (&*view).send_message(Sel::register("setLayer:"), (pointer,))?;
                    let (): () = (&*view).send_message(Sel::register("setWantsLayer:"), (YES,))?;
                }
                Ok(layer)
            })?;
            let surface = Self {
                layer,
                view,
                window: window.clone(),
            };
            surface.update(window)?;
            Ok(surface)
        }
        pub fn update(&self, window: &winit::window::Window) -> Result<()> {
            let size = window.inner_size();
            unsafe {
                let bounds: Rect = (&*self.view).send_message(Sel::register("bounds"), ())?;
                let (): () =
                    (&**self.layer).send_message(Sel::register("setBounds:"), (bounds,))?;
                let (): () = (&**self.layer)
                    .send_message(Sel::register("setContentsScale:"), (window.scale_factor(),))?;
                let (): () = (&**self.layer).send_message(
                    Sel::register("setDrawableSize:"),
                    (Size {
                        width: f64::from(size.width),
                        height: f64::from(size.height),
                    },),
                )?;
            }
            Ok(())
        }
        pub fn create_surface(
            &self,
            instance: Arc<rhapsody::interop::VulkanInstance>,
        ) -> Result<Arc<rhapsody::present::Surface>> {
            let loader =
                ash::ext::metal_surface::Instance::new(&instance.entry, &instance.instance);
            let info = vk::MetalSurfaceCreateInfoEXT::default().layer((*self.layer).cast());
            let handle = unsafe { loader.create_metal_surface(&info, None) }?;
            Ok(Arc::new(unsafe {
                // NSView's hosted layer property retains this CAMetalLayer.
                // Keep its actual content-view/window owner in Surface too:
                // failed WSI teardown can retain Surface after PlatformSurface.
                rhapsody::present::Surface::from_raw_retained(
                    instance,
                    handle,
                    true,
                    self.window.clone(),
                )
            }))
        }
    }
}
