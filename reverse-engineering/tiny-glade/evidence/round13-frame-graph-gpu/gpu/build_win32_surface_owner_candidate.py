from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu'
relative = 'tiny-glade/crates/tiny-glade/src/production_renderer.rs'
source = root / relative
base = source.read_bytes()
text = base.decode('utf-8')
newline = '\r\n' if '\r\n' in text else '\n'
old = '''        let loader = ash::khr::win32_surface::Instance::new(&instance.entry, &instance.instance);
        let info = vk::Win32SurfaceCreateInfoKHR::default()
            .hinstance(self.hinstance)
            .hwnd(self.hwnd);
        // ProductionRenderer retains the winit Window until after its swapchain
        // and Surface owners drop. Surface::from_raw owns the Vulkan destruction.
        let handle = unsafe { loader.create_win32_surface(&info, None) }
            .context("create Vulkan Win32 surface")?;
        Ok(Arc::new(unsafe {
            rhapsody::present::Surface::from_raw(instance, handle, true)
        }))'''.replace('\n', newline)
new = '''        // Keep the existing Win32-specific Surface constructor: in addition
        // to owning destruction it marks the surface as Win32 for fullscreen.
        // ProductionRenderer retains the winit Window through all WSI owners.
        unsafe { rhapsody::present::Surface::from_win32(instance, self.hinstance, self.hwnd) }
            .context("create Vulkan Win32 surface")'''.replace('\n', newline)
assert text.count(old) == 1
candidate = text.replace(old, new)
target = out / 'win32-owner-candidate' / relative
target.parent.mkdir(parents=True, exist_ok=True)
target.write_bytes(candidate.encode('utf-8'))
(out / 'win32-surface-owner.patch').write_text(''.join(difflib.unified_diff(
    text.splitlines(keepends=True), candidate.splitlines(keepends=True),
    fromfile='a/' + relative, tofile='b/' + relative,
)), encoding='utf-8', newline='')
(out / 'win32-surface-owner-manifest.json').write_text(json.dumps({
    'scope': 'reuse current rhapsody Win32 Surface owner, preserving win32=true fullscreen identity',
    'production_written': False,
    'files': [{'path': relative, 'base_sha256': hashlib.sha256(base).hexdigest(),
               'candidate_sha256': hashlib.sha256(candidate.encode('utf-8')).hexdigest()}],
    'evidence': ['tiny-glade/crates/rhapsody/src/raw/swapchain.rs Surface::from_raw/from_win32_raw/from_win32',
                 'tiny-glade/crates/rhapsody/src/raw/swapchain/fullscreen.rs win32 validation'],
    'validation': 'root owns Cargo and actual GPU acceptance',
}, indent=2), encoding='utf-8')
print('win32-surface-owner.patch generated')
