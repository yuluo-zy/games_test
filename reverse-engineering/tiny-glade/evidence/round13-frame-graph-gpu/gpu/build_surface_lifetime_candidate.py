from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu'
files = []

def change(relative, transform):
    base = (root / relative).read_bytes()
    text = base.decode('utf-8')
    newline = '\r\n' if '\r\n' in text else '\n'
    candidate = transform(text.replace('\r\n', '\n')).replace('\n', newline)
    assert candidate != text
    target = out / 'surface-lifetime-candidate' / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(candidate.encode('utf-8'))
    files.append({'path': relative, 'base_sha256': hashlib.sha256(base).hexdigest(),
                  'candidate_sha256': hashlib.sha256(candidate.encode('utf-8')).hexdigest()})
    return ''.join(difflib.unified_diff(text.splitlines(keepends=True), candidate.splitlines(keepends=True),
                                      fromfile='a/' + relative, tofile='b/' + relative))

def swapchain(text):
    old = '''    lost: std::sync::atomic::AtomicBool,
}'''
    new = '''    lost: std::sync::atomic::AtomicBool,
    // Dropped after Surface::drop destroys VkSurfaceKHR. Swapchain teardown
    // may retain this Surface on an unproven WSI timeout, so keep its actual
    // native window alive with it rather than relying on a host's field order.
    native_owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
}'''
    assert text.count(old) == 1
    text = text.replace(old, new)
    old = '''            lost: std::sync::atomic::AtomicBool::new(false),
        }'''
    new = '''            lost: std::sync::atomic::AtomicBool::new(false),
            native_owner: None,
        }'''
    assert text.count(old) == 1
    text = text.replace(old, new)
    old = '''    ) -> Result<Arc<Self>> {
        let loader = ash::khr::win32_surface::Instance::new(&instance.entry, &instance.instance);
        let info = vk::Win32SurfaceCreateInfoKHR::default()
            .hinstance(hinstance)
            .hwnd(hwnd);
        let handle = unsafe { loader.create_win32_surface(&info, None) }?;
        Ok(Arc::new(unsafe {
            Self::from_win32_raw(instance, handle, true)
        }))
    }
}
impl Drop for Surface'''
    new = '''    ) -> Result<Arc<Self>> {
        unsafe { Self::create_win32(instance, hinstance, hwnd, None) }
    }

    /// Create a Win32 surface retaining the actual HWND owner until Vulkan
    /// surface destruction, including deliberately retained WSI generations.
    /// This is one bounded owner slot; cloning/replacing a swapchain does not
    /// append additional native-window leases.
    ///
    /// # Safety
    /// In addition to from_win32's requirements, native_owner must retain the
    /// window that owns hwnd. The owner must not strongly refer to this Surface.
    pub unsafe fn from_win32_retained(
        instance: Arc<VulkanInstance>,
        hinstance: vk::HINSTANCE,
        hwnd: vk::HWND,
        native_owner: Arc<dyn std::any::Any + Send + Sync>,
    ) -> Result<Arc<Self>> {
        unsafe { Self::create_win32(instance, hinstance, hwnd, Some(native_owner)) }
    }

    unsafe fn create_win32(
        instance: Arc<VulkanInstance>,
        hinstance: vk::HINSTANCE,
        hwnd: vk::HWND,
        native_owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
    ) -> Result<Arc<Self>> {
        let loader = ash::khr::win32_surface::Instance::new(&instance.entry, &instance.instance);
        let info = vk::Win32SurfaceCreateInfoKHR::default()
            .hinstance(hinstance)
            .hwnd(hwnd);
        let handle = unsafe { loader.create_win32_surface(&info, None) }?;
        let mut surface = unsafe { Self::from_win32_raw(instance, handle, true) };
        surface.native_owner = native_owner;
        Ok(Arc::new(surface))
    }
}
impl Drop for Surface'''
    assert text.count(old) == 1
    return text.replace(old, new)

def host(text):
    start = text.index('// Host platform integration.')
    end = text.index('#[cfg(not(any(target_os = "macos", target_os = "windows")))]', start)
    old = text[start:end]
    new = '''// Host platform integration. This uses winit's actual Win32 handles and the
// existing Surface owner, including its native-window lifetime on failed WSI teardown.
#[cfg(target_os = "windows")]
struct PlatformSurface {
    hwnd: vk::HWND,
    hinstance: vk::HINSTANCE,
    window: Arc<winit::window::Window>,
}
#[cfg(target_os = "windows")]
impl PlatformSurface {
    fn native_handles(window: &winit::window::Window) -> Result<(vk::HWND, vk::HINSTANCE)> {
        use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let handle = match window.window_handle()?.as_raw() {
            RawWindowHandle::Win32(handle) => handle,
            _ => anyhow::bail!("winit did not return a Win32 window handle"),
        };
        Ok((
            handle.hwnd.get(),
            handle
                .hinstance
                .context("winit Win32 window has no module instance")?
                .get(),
        ))
    }

    fn new(window: &Arc<winit::window::Window>) -> Result<Self> {
        let (hwnd, hinstance) = Self::native_handles(window)?;
        Ok(Self {
            hwnd,
            hinstance,
            window: window.clone(),
        })
    }

    fn update(&self, window: &winit::window::Window) -> Result<()> {
        // Win32 owns the client extent. ProductionRenderer::resize updates the
        // swapchain separately; never create a second surface on resize/resume.
        let (hwnd, hinstance) = Self::native_handles(window)?;
        anyhow::ensure!(
            self.hwnd == hwnd && self.hinstance == hinstance,
            "Vulkan surface belongs to a different Win32 window"
        );
        Ok(())
    }

    fn create_surface(
        &self,
        instance: Arc<rhapsody::interop::VulkanInstance>,
    ) -> Result<Arc<rhapsody::present::Surface>> {
        // Keep both the native Win32 identity and its actual window owner.
        // Surface destruction releases this lease after vkDestroySurfaceKHR;
        // failed swapchain teardown retains the same lease with the Surface.
        unsafe {
            rhapsody::present::Surface::from_win32_retained(
                instance,
                self.hinstance,
                self.hwnd,
                self.window.clone(),
            )
        }
        .context("create Vulkan Win32 surface")
    }
}

'''
    return text[:start] + new + text[end:]

patch = change('tiny-glade/crates/rhapsody/src/raw/swapchain.rs', swapchain)
patch += change('tiny-glade/crates/tiny-glade/src/production_renderer.rs', host)
(out / 'surface-native-lifetime.patch').write_text(patch, encoding='utf-8', newline='')
(out / 'surface-native-lifetime-manifest.json').write_text(json.dumps({
    'scope': 'retain the actual native Window in existing Surface lifetime; no device or WSI fallback policy change',
    'production_written': False,
    'files': files,
    'validation': 'root owns Cargo, Surface Send/Sync admission and actual GPU acceptance',
}, indent=2), encoding='utf-8')
print('surface-native-lifetime.patch generated')
