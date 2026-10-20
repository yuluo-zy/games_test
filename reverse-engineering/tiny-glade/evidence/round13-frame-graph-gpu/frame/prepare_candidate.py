from pathlib import Path
import difflib, hashlib, json, struct

ROOT = Path('D:/game')
OUT = Path(__file__).parent
FILES = [
    'tiny-glade/crates/country-core/src/systems/time_update.rs',
    'tiny-glade/crates/country-core/src/systems/backend/input/input_record_replay.rs',
    'tiny-glade/crates/country-core/tests/input_pipeline.rs',
    'tiny-glade/crates/tiny-glade/src/game_loop.rs',
]

def swap(s, old, new):
    assert s.count(old) == 1, (old[:80], s.count(old))
    return s.replace(old, new)

base = {p: (ROOT/p).read_bytes() for p in FILES}
src = {p: b.decode('utf-8').replace('\r\n', '\n') for p,b in base.items()}
p = FILES[0]
src[p] = swap(src[p], '        game_clock::{ClockGate, DeltaSource},', '        game_clock::{ClockGate, DeltaSource, GameClockError},')
src[p] = swap(src[p], '    mut replay: ResMut<InputRecordReplayState>,', '    replay: Res<InputRecordReplayState>,')
start = src[p].index('    let source = if let Some(frame) = replay.current_frame() {')
end = src[p].index('        DeltaSource::Replay {', start)
src[p] = src[p][:start] + '''    // B 140922c61..c83 checks the replay index before reading +0xc8.
    // Replay mode with no current frame is an invariant failure, not live time.
    let source = if replay.mode == crate::input::InputRecordReplayMode::Replay {
        let frame = replay.current_frame().expect("Out of bounds access");
''' + src[p][end:]
start = src[p].index('    // GameClock 保留原算法的写入顺序；')
src[p] = src[p][:start] + '''    // B writes delta at 140922ca0/140922d7f, then converts and adds it.
    // Invalid input/overflow panics before any later clock writes. Keep those
    // effects on the actual resource; rolling back or zeroing it changes B.
    if let Err(error) = game.update(gate, source) {
        match error {
            GameClockError::InvalidDelta if game.delta < 0.0 => {
                panic!("cannot convert float seconds to Duration: value is negative");
            }
            GameClockError::InvalidDelta => {
                panic!("cannot convert float seconds to Duration: value is either too big or NaN");
            }
            GameClockError::DurationOverflow => panic!("overflow when adding durations"),
        }
    }
}
'''

# Delete only the recovery state made unreachable by the original panic path.
p = FILES[1]
src[p] = swap(src[p], '    input_suppressed_this_frame: bool,\n', '')
src[p] = swap(src[p], '            input_suppressed_this_frame: false,\n', '')
start = src[p].index('    /// 时钟失败时终止回放，')
end = src[p].index('    /// 先检查回退目标，', start)
src[p] = src[p][:start] + src[p][end:]
src[p] = swap(src[p], '''    world
        .resource_mut::<InputRecordReplayState>()
        .input_suppressed_this_frame = false;
''', '')
src[p] = swap(src[p], '''    if replay.input_suppressed_this_frame {
        *actions = ActionSetStates::default();
        values.current = Default::default();
        return;
    }
''', '')

p = FILES[2]
start = src[p].index('fn clock_overflow_stops_replay_without_partial_clock_or_live_action_leak() {')
end = src[p].index('\n#[test]\nfn right_click_', start)
src[p] = src[p][:start] + '''fn clock_overflow_panics_after_delta_write_without_advancing_other_fields() {
    let (mut world, mut schedule) = fixture();
    two_frame_recording(&mut world);
    world.resource_mut::<GameTime>().elapsed = std::time::Duration::MAX;
    let mut expected = *world.resource::<GameTime>();
    expected.delta = 0.1;
    world.send_event(InputReplayCommand::Replay { rewind: 0 });
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        schedule.run(&mut world);
    }));
    assert!(result.is_err());
    assert_eq!(*world.resource::<GameTime>(), expected);
    let replay = world.resource::<InputRecordReplayState>();
    assert_eq!(replay.mode, InputRecordReplayMode::Replay);
    assert_eq!(replay.stream.frame_count(), 2);
    assert!(replay.last_error.is_none());
}

#[test]
fn malformed_in_memory_replay_time_panics_after_publishing_the_bad_delta() {
    for bad in [-1.0, f32::MAX, f32::NAN] {
        let (mut world, mut schedule) = fixture();
        two_frame_recording(&mut world);
        world.resource_mut::<InputRecordReplayState>()
            .stream.last_frame_mut().unwrap().time_delta_seconds = bad;
        world.send_event(InputReplayCommand::Replay { rewind: 0 });
        schedule.run(&mut world);
        let before = *world.resource::<GameTime>();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            schedule.run(&mut world);
        }));
        assert!(result.is_err());
        let after = *world.resource::<GameTime>();
        assert_eq!(after.delta.to_bits(), bad.to_bits());
        assert_eq!(GameTime { delta: before.delta, ..after }, before);
        assert_eq!(world.resource::<InputRecordReplayState>().mode, InputRecordReplayMode::Replay);
    }
}
''' + src[p][end:]
start = src[p].index('fn stop_and_invalid_frame_release_capture_and_allow_future_live_requests() {')
src[p] = swap(src[p], 'fn stop_and_invalid_frame_release_capture_and_allow_future_live_requests() {', 'fn stop_releases_capture_and_allows_future_live_requests() {')
src[p] = swap(src[p], '    for invalid_frame in [false, true] {\n', '    {\n')
start = src[p].index('        if invalid_frame {', start)
end = src[p].index('        world.send_event(CursorCaptureCmd(false));', start)
src[p] = src[p][:start] + '        world.send_event(InputReplayCommand::Stop);\n' + src[p][end:]

p = FILES[3]
src[p] = swap(src[p], '        let input = self.egui_io.take_egui_input(&self.window);', '''        // B 1404ddf28..3c: real OS egui input is consumed only while the
        // debug or experimental window is active. Other frames still begin
        // and end egui with RawInput::default (1404ddf49 -> 140c2c770).
        let input = if frame_egui_input_enabled(self.driver.app.world()) {
            self.egui_io.take_egui_input(&self.window)
        } else {
            egui::RawInput::default()
        };''')
anchor = 'fn apply_menu_fps_limit(fps: f32, menu_limits: bool, profiling: bool) -> f32 {'
src[p] = swap(src[p], anchor, '''/// B 1404ddf28..3c: absent DebugGui has the default-input behavior.
pub fn frame_egui_input_enabled(world: &World) -> bool {
    world.get_resource::<country_core::resources::gui::DebugGui>()
        .is_some_and(|debug| debug.enabled || debug.experimental_window)
}

''' + anchor)
anchor = '    fn camera_resource_presence_selects_the_native_frame_mode() {'
src[p] = swap(src[p], anchor, '''    fn debug_window_flags_choose_actual_os_egui_input() {
        use country_core::resources::gui::DebugGui;
        let mut world = World::new();
        assert!(!frame_egui_input_enabled(&world));
        world.insert_resource(DebugGui::default());
        assert!(!frame_egui_input_enabled(&world));
        world.resource_mut::<DebugGui>().enabled = true;
        assert!(frame_egui_input_enabled(&world));
        world.resource_mut::<DebugGui>().enabled = false;
        world.resource_mut::<DebugGui>().experimental_window = true;
        assert!(frame_egui_input_enabled(&world));
        world.resource_mut::<DebugGui>().experimental_window = false;
        assert!(!frame_egui_input_enabled(&world));
    }

    #[test]
''' + anchor)

patch = []
manifest = []
for p in FILES:
    old = base[p].decode('utf-8').replace('\r\n', '\n')
    new = src[p]
    dest = OUT/'candidate'/p
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_bytes(new.encode())
    saved = OUT/'base'/p
    saved.parent.mkdir(parents=True, exist_ok=True)
    saved.write_bytes(base[p])
    patch.extend(difflib.unified_diff(old.splitlines(True), new.splitlines(True), fromfile='a/'+p,tofile='b/'+p))
    manifest.append({'path':p,'base_sha256':hashlib.sha256(base[p]).hexdigest(),'candidate_sha256':hashlib.sha256(new.encode()).hexdigest()})
(OUT/'candidate.patch').write_text(''.join(patch), encoding='utf-8')

# Bind source evidence to original PE bytes; only standard library parsing.
rne = ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
blob = rne.read_bytes()
pe = struct.unpack_from('<I',blob,0x3c)[0]
nsec = struct.unpack_from('<H',blob,pe+6)[0]
optlen = struct.unpack_from('<H',blob,pe+20)[0]
imagebase = struct.unpack_from('<Q',blob,pe+24+24)[0]
sections = pe+24+optlen
def native(va, length):
    rva=va-imagebase
    for i in range(nsec):
        off=sections+i*40
        vs,addr,rawsize,raw=struct.unpack_from('<IIII',blob,off+8)
        if addr <= rva < addr+max(vs,rawsize):
            at=raw+rva-addr
            return blob[at:at+length]
    raise ValueError(hex(va))
evidence={
    'path':str(rne),'sha256':hashlib.sha256(blob).hexdigest(),
    'time_update':{'address':'0x140922c30','size':0x376,'sha256':hashlib.sha256(native(0x140922c30,0x376)).hexdigest()},
    'panic_strings':{hex(a):native(a,n).decode() for a,n in [(0x142aad5e0,0x3b),(0x142aad61b,0x48),(0x142aad928,0x1e),(0x142ab3224,0x14)]},
    'files':manifest,'validation':'Candidate only. No Cargo run or production writes by frame owner.'}
(OUT/'manifest.json').write_text(json.dumps(evidence,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'files':len(manifest),'original_sha256':evidence['sha256'],'panic_strings':evidence['panic_strings']},ensure_ascii=False))
