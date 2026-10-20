from pathlib import Path
import hashlib, difflib, json

root = Path('D:/game')
out = root/'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/acceptance/present'
source = root/'tiny-glade/crates/tiny-glade'
files = {}
def add(relative, candidate, base=None):
    rel = Path('tiny-glade/crates/tiny-glade')/relative
    if base is None:
        path = root/rel
        base = path.read_text(encoding='utf-8') if path.exists() else ''
    files[str(rel).replace('\\','/')] = (base, candidate)

base = (source/'src/game_loop.rs').read_text(encoding='utf-8')
needle = '    /// B consumes outgoing game requests before the OS event batch.'
assert base.count(needle) == 1
addition = '''    /// Read-only production renderer access for physical-owner/WSI diagnostics.
    pub fn renderer(&self) -> &crate::production_renderer::ProductionRenderer {
        &self.renderer
    }

    /// Opt-in host diagnostic; the normal game keeps its existing ticket policy.
    pub fn enable_world_present_completion_tracking(&mut self) {
        self.renderer.enable_world_present_completion_tracking();
    }

'''
add('src/game_loop.rs', base.replace(needle, addition+needle), base)

base = (source/'src/production_renderer.rs').read_text(encoding='utf-8')
candidate = base.replace('    pending: VecDeque<PresentTicket>,', '''    pending: VecDeque<(PresentTicket, bool)>,
    // Opt-in diagnostic: count rendering AND WSI completion, never submission.
    track_world_present_completions: bool,
    completed_world_presents: u64,''')
candidate = candidate.replace('            pending: VecDeque::new(),', '''            pending: VecDeque::new(),
            track_world_present_completions: false,
            completed_world_presents: 0,''')
needle = '    pub fn any_present_successful(&self) -> bool {'
assert candidate.count(needle) == 1
addition = '''    /// Keep real world-frame tickets until both completion domains are met.
    /// Disabled by default so ordinary game ticket retirement is unchanged.
    pub fn enable_world_present_completion_tracking(&mut self) {
        self.track_world_present_completions = true;
    }

    /// Completed world rendering AND WSI presentation, excluding loading frames.
    pub fn completed_world_presents(&self) -> u64 {
        self.completed_world_presents
    }

'''
candidate = candidate.replace(needle, addition+needle)
needle = '''        while let Some(ticket) = self.pending.front() {
            if !ticket.is_complete()? {
                break;
            }
            self.pending.pop_front();
        }'''
assert candidate.count(needle) == 1
replacement = '''        while let Some((ticket, track_world)) = self.pending.front() {
            if !ticket.is_complete()? || (*track_world && !ticket.is_present_complete()?) {
                break;
            }
            let (_, world_completed) = self.pending.pop_front().expect("checked front ticket");
            if world_completed {
                self.completed_world_presents = self.completed_world_presents.saturating_add(1);
            }
        }'''
candidate = candidate.replace(needle, replacement)
needle = '''        for ticket in &self.pending {
            ticket.wait(TIMEOUT_NS)?;
        }
        self.pending.clear();
        self.swapchain.shutdown(TIMEOUT_NS)?;'''
assert candidate.count(needle) == 1
replacement = '''        for (ticket, _) in &self.pending {
            ticket.wait(TIMEOUT_NS)?;
        }
        let remaining_world = self.pending.iter().filter(|(_, world)| *world).count() as u64;
        self.pending.clear();
        // Shutdown's actual presentation teardown is required before counting
        // outstanding diagnostic tickets; queue/render idle alone is insufficient.
        self.swapchain.shutdown(TIMEOUT_NS)?;
        self.completed_world_presents = self.completed_world_presents.saturating_add(remaining_world);'''
candidate = candidate.replace(needle, replacement)
needle = '            self.renderer.pending.push_back(ticket);'
assert candidate.count(needle) == 1
replacement = '''            let track_world = self.renderer.track_world_present_completions
                && self.world.contains_resource::<country_core::systems::camera_rig::main_camera::MainCamera>()
                && self.world.contains_resource::<country_core::resources::session::ActiveSession>();
            self.renderer.pending.push_back((ticket, track_world));'''
candidate = candidate.replace(needle, replacement)
assert candidate != base
add('src/production_renderer.rs', candidate, base)

base = (source/'Cargo.toml').read_text(encoding='utf-8')
needle = '[dependencies]'
assert base.count(needle) == 1
addition = '''# Actual async original-intro startup and nonempty world WSI acceptance.
[[test]]
name = "original_gameplay_present"
path = "tests/original_gameplay_present.rs"
harness = false

'''
add('Cargo.toml', base.replace(needle, addition+needle), base)

helper = (source/'tests/original_gameplay_chain.rs').read_text(encoding='utf-8')
helper = helper[helper.index('fn assert_nonempty_original_walls('):]
helper = helper.replace('bridge: &CoreRenderBridge,', 'buffers: &tiny_glade::render_host::BufferHost,')
helper = helper.replace('bridge.hosts().buffers.managed_buffer', 'buffers.managed_buffer')
template = (out.parent/'gameplay_present_template.rs').read_text(encoding='utf-8')
candidate = template.replace('// NONEMPTY_HELPER_INSERTION_POINT', helper)
add('tests/original_gameplay_present.rs', candidate)

manifest = []
patch = []
for rel, (base, candidate) in files.items():
    for folder, text in [('base',base), ('candidate',candidate)]:
        target = out/folder/rel
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding='utf-8', newline='\n')
    patch.append(''.join(difflib.unified_diff(base.splitlines(True), candidate.splitlines(True),
        fromfile='a/'+rel if base else '/dev/null', tofile='b/'+rel)))
    manifest.append({'path':rel, 'base_sha256':hashlib.sha256(base.encode()).hexdigest() if base else None,
        'candidate_sha256':hashlib.sha256(candidate.encode()).hexdigest()})
(out/'gameplay-present.patch').write_text(''.join(patch), encoding='utf-8', newline='\n')
(out/'manifest.json').write_text(json.dumps({'scope':'real production GameLoop, original two-App preparation, original-intro content and completed world WSI tickets; no pixel equivalence claim',
    'cargo_executed':False, 'production_modified':False, 'files':manifest},indent=2),encoding='utf-8')
print(out/'gameplay-present.patch')
