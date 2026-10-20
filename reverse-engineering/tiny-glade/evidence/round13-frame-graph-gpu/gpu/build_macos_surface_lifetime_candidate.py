from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu/macos-surface-lifetime'
files = []

def change(relative, transform):
    base = (root / relative).read_bytes()
    text = base.decode('utf-8')
    newline = '\r\n' if '\r\n' in text else '\n'
    candidate = transform(text.replace('\r\n', '\n')).replace('\n', newline)
    assert candidate != text
    target = out / 'candidate' / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(candidate.encode('utf-8'))
    files.append({'path': relative, 'base_sha256': hashlib.sha256(base).hexdigest(),
                  'candidate_sha256': hashlib.sha256(candidate.encode('utf-8')).hexdigest()})
    return ''.join(difflib.unified_diff(text.splitlines(keepends=True), candidate.splitlines(keepends=True),
                                      fromfile='a/' + relative, tofile='b/' + relative))

def surface(text):
    marker = '''    /// Wrap an existing Win32 Vulkan surface, allowing explicit fullscreen.'''
    new = '''    /// Wrap a native surface while retaining the actual platform-window
    /// owner through vkDestroySurfaceKHR and any deliberately retained WSI
    /// generation. This uses the same bounded native-owner slot as Win32.
    ///
    /// # Safety
    /// In addition to from_raw's requirements, native_owner must keep all
    /// platform objects needed by handle alive. It must not retain this Surface.
    pub unsafe fn from_raw_retained(
        instance: Arc<VulkanInstance>,
        handle: vk::SurfaceKHR,
        owned: bool,
        native_owner: Arc<dyn std::any::Any + Send + Sync>,
    ) -> Self {
        let mut surface = unsafe { Self::from_raw(instance, handle, owned) };
        surface.native_owner = Some(native_owner);
        surface
    }

'''
    assert text.count(marker) == 1
    return text.replace(marker, new + marker)

def host(text):
    start = text.index('#[cfg(target_os = "macos")]\nmod macos {')
    end = text.index('// Host platform integration.', start)
    mac = text[start:end]
    changes = [
        ('''    pub struct PlatformSurface {
        layer: StrongPtr,
        view: *mut Object,
    }''', '''    pub struct PlatformSurface {
        layer: StrongPtr,
        view: *mut Object,
        window: Arc<winit::window::Window>,
    }'''),
        ('pub fn new(window: &winit::window::Window) -> Result<Self> {',
         'pub fn new(window: &Arc<winit::window::Window>) -> Result<Self> {'),
        ('let surface = Self { layer, view };', '''let surface = Self {
                layer,
                view,
                window: window.clone(),
            };'''),
        ('''                rhapsody::present::Surface::from_raw(instance, handle, true)''', '''                // NSView's hosted layer property retains this CAMetalLayer.
                // Keep its actual content-view/window owner in Surface too:
                // failed WSI teardown can retain Surface after PlatformSurface.
                rhapsody::present::Surface::from_raw_retained(
                    instance,
                    handle,
                    true,
                    self.window.clone(),
                )'''),
    ]
    for old, new in changes:
        assert mac.count(old) == 1, old
        mac = mac.replace(old, new)
    return text[:start] + mac + text[end:]

patch = change('tiny-glade/crates/rhapsody/src/raw/swapchain.rs', surface)
patch += change('tiny-glade/crates/tiny-glade/src/production_renderer.rs', host)
(out / 'candidate.patch').write_text(patch, encoding='utf-8', newline='')
(out / 'manifest.json').write_text(json.dumps({
    'scope': 'macOS native Window/contentView/hosted CAMetalLayer lifetime in existing bounded Surface owner',
    'production_written': False,
    'files': files,
    'validation': 'root owns Cargo; macOS compile/runtime not available on this Windows worker',
}, indent=2), encoding='utf-8')
print('macos-surface-lifetime/candidate.patch generated')
