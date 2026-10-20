from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu'
source = root / 'tiny-glade/crates/tiny-glade/src/production_renderer.rs'
base = source.read_bytes()
text = base.decode('utf-8')
newline = '\r\n' if '\r\n' in text else '\n'
old = '#[cfg(not(target_os = "macos"))]\nstruct PlatformSurface;\n#[cfg(not(target_os = "macos"))]\nimpl PlatformSurface {'.replace('\n', newline)
new = '''// Host platform integration. This uses winit's actual Win32 handles and the
// existing ash surface owner; it does not introduce another Vulkan lifetime.
#[cfg(target_os = "windows")]
struct PlatformSurface {
    hwnd: vk::HWND,
    hinstance: vk::HINSTANCE,
}
#[cfg(target_os = "windows")]
impl PlatformSurface {
    fn new(window: &winit::window::Window) -> Result<Self> {
        use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
        let handle = match window.window_handle()?.as_raw() {
            RawWindowHandle::Win32(handle) => handle,
            _ => anyhow::bail!("winit did not return a Win32 window handle"),
        };
        Ok(Self {
            hwnd: handle.hwnd.get(),
            hinstance: handle
                .hinstance
                .context("winit Win32 window has no module instance")?
                .get(),
        })
    }

    fn update(&self, window: &winit::window::Window) -> Result<()> {
        // Win32 owns the client extent. ProductionRenderer::resize updates the
        // swapchain separately; never create a second surface on resize/resume.
        let current = Self::new(window)?;
        anyhow::ensure!(
            self.hwnd == current.hwnd && self.hinstance == current.hinstance,
            "Vulkan surface belongs to a different Win32 window"
        );
        Ok(())
    }

    fn create_surface(
        &self,
        instance: Arc<rhapsody::interop::VulkanInstance>,
    ) -> Result<Arc<rhapsody::present::Surface>> {
        let loader = ash::khr::win32_surface::Instance::new(&instance.entry, &instance.instance);
        let info = vk::Win32SurfaceCreateInfoKHR::default()
            .hinstance(self.hinstance)
            .hwnd(self.hwnd);
        // ProductionRenderer retains the winit Window until after its swapchain
        // and Surface owners drop. Surface::from_raw owns the Vulkan destruction.
        let handle = unsafe { loader.create_win32_surface(&info, None) }
            .context("create Vulkan Win32 surface")?;
        Ok(Arc::new(unsafe {
            rhapsody::present::Surface::from_raw(instance, handle, true)
        }))
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
struct PlatformSurface;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
impl PlatformSurface {'''.replace('\n', newline)
assert text.count(old) == 1
candidate = text.replace(old, new).replace(
    '// The native window and CAMetalLayer outlive every Vulkan surface owner.',
    '// The native window and platform backing outlive every Vulkan surface owner.',
)
relative = 'tiny-glade/crates/tiny-glade/src/production_renderer.rs'
target = out / 'candidate' / relative
target.parent.mkdir(parents=True, exist_ok=True)
target.write_bytes(candidate.encode('utf-8'))
(out / 'production-renderer-win32.patch').write_text(''.join(difflib.unified_diff(
    text.splitlines(keepends=True), candidate.splitlines(keepends=True),
    fromfile='a/' + relative, tofile='b/' + relative,
)), encoding='utf-8', newline='')
(out / 'manifest.json').write_text(json.dumps({
    'scope': 'Windows host surface integration, engineering adapter; no original algorithm recovery claim',
    'production_written': False,
    'files': [{
        'path': relative,
        'base_sha256': hashlib.sha256(base).hexdigest(),
        'candidate_sha256': hashlib.sha256(candidate.encode('utf-8')).hexdigest(),
    }],
    'validation': 'API checked against locally locked ash 0.38.0+1.3.281 / winit fork d03de0a; root owns Cargo and actual GPU acceptance',
}, indent=2), encoding='utf-8')
print(json.dumps({'base_sha256': hashlib.sha256(base).hexdigest(), 'candidate': str(target)}))
