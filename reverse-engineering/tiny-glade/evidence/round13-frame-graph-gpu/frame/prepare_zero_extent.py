from pathlib import Path
import json, hashlib, difflib

ROOT = Path('D:/game')
OUT = Path(__file__).parent/'zero-extent'
FILES = ['tiny-glade/crates/tiny-glade/src/game_loop.rs',
         'tiny-glade/crates/tiny-glade/src/event_loop.rs']
base = {p:(ROOT/p).read_bytes() for p in FILES}
sources = {p:b.decode().replace('\r\n','\n') for p,b in base.items()}

def swap(s, old, new):
    assert s.count(old) == 1, (s.count(old),old[:100])
    return s.replace(old,new)

p=FILES[0]
anchor='fn apply_menu_fps_limit(fps: f32, menu_limits: bool, profiling: bool) -> f32 {'
sources[p]=swap(sources[p],anchor,'''/// B 1404dc8d2..dc971 checks the current display extent before App::update.
/// Resize publishes this exact owner at 1404dad4f/51. A zero component sleeps
/// for one millisecond and takes the common loop tail without simulating.
pub fn frame_waits_for_window_extent(world: &World) -> bool {
    world.resource::<country_core::render::render_config::FrameRenderConfig>()
        .extent_0.contains(&0)
}

'''+anchor)
anchor='''        // B 1404ddf28..3c: real OS egui input is consumed only while the
'''
sources[p]=swap(sources[p],anchor,'''        if frame_waits_for_window_extent(self.driver.app.world()) {
            // Native 1404dc965: Duration { secs: 0, nanos: 1_000_000 }.
            // Do not consume egui/input, update the App/MEI, advance either
            // frame counter, render, or run the FPS limiter on this branch.
            std::thread::sleep(std::time::Duration::from_millis(1));
            // Native rejoins at de84d: OS/show/settings tail remains alive.
            self.apply_pending_video_settings()?;
            return Ok(true);
        }
'''+anchor)
anchor='    fn debug_window_flags_choose_actual_os_egui_input() {'
sources[p]=swap(sources[p],anchor,'''    fn zero_extent_gate_reads_the_actual_frame_config_owner() {
        use country_core::render::render_config::{FrameRenderConfig, VideoSettings};
        use country_core::resources::window::WindowSize;
        let mut world = World::new();
        // WindowSize deliberately differs: B reads FrameRenderConfig.extent_0.
        world.insert_resource(WindowSize::new(0, 0));
        world.insert_resource(FrameRenderConfig::at_startup(
            WindowSize::new(1280, 720), &VideoSettings::default()));
        assert!(!frame_waits_for_window_extent(&world));
        for extent in [[0, 720], [1280, 0], [0, 0]] {
            world.resource_mut::<FrameRenderConfig>().extent_0 = extent;
            assert!(frame_waits_for_window_extent(&world));
        }
        world.resource_mut::<FrameRenderConfig>().extent_0 = [1280, 720];
        assert!(!frame_waits_for_window_extent(&world));
    }

    #[test]
'''+anchor)
p=FILES[1]
anchor='''                    .set(width, height);
                self.app_mut()
'''
sources[p]=swap(sources[p],anchor,'''                    .set(width, height);
                // B 1404dad4f/51, immediately after WindowSize::set: publish
                // the same physical size to the pre-update frame gate owner.
                self.app_mut().world_mut()
                    .resource_mut::<country_core::render::render_config::FrameRenderConfig>()
                    .extent_0 = [width, height];
                self.app_mut()
''')
sources[p]=swap(sources[p],'''                // checked. Even a minimized window must run this last update.
''','''                // checked. A zero-size frame follows B's pre-update wait;
                // the event queue and OS loop remain live until restoration.
''')
OUT.mkdir(parents=True,exist_ok=True)
patch=[]; records=[]
for p in FILES:
    old=base[p].decode().replace('\r\n','\n');new=sources[p]
    for folder,blob in [('base',base[p]),('candidate',new.encode())]:
        dest=OUT/folder/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(blob)
    patch.extend(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+p,tofile='b/'+p))
    records.append({'path':p,'base_sha256':hashlib.sha256(base[p]).hexdigest(),'candidate_sha256':hashlib.sha256(new.encode()).hexdigest()})
(OUT/'candidate.patch').write_text(''.join(patch),encoding='utf-8')
(OUT/'manifest.json').write_text(json.dumps({'files':records,'validation':'Candidate only; no production writes or Cargo by frame owner.'},indent=2)+'\n')
print(json.dumps(records))
