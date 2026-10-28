"""One-shot narrow integration of original-evidenced UI geometry/state owners.

Reads the frozen candidates but retains current production fields and APIs.
Run only with the central Cargo/write gate released; never replaces a library.
"""
from pathlib import Path
import shutil
import sys

root = Path('D:/game')
core = root / 'tiny-glade/crates/country-core'
if '--stage' in sys.argv:
    staged = root / 'tiny-glade/reports/business/2026-10-21-business-module-recovery/proposed/tiny-glade/crates/country-core'
    for relative in ['src/systems/ui/resources/widget_params.rs', 'src/systems/ui/resources/ui_state.rs', 'src/systems/history/recording.rs', 'Cargo.toml']:
        destination = staged / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(core / relative, destination)
    core = staged
candidate = root / 'reverse-engineering/tiny-glade/evidence/round12/roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources'

def once(text, old, new):
    assert text.count(old) == 1, (old[:100], text.count(old))
    return text.replace(old, new, 1)

path = core / 'src/systems/ui/resources/widget_params.rs'
text = path.read_text(encoding='utf-8')
original = (candidate / 'widget_params.rs').read_text(encoding='utf-8')
sound = (root / 'reverse-engineering/tiny-glade/evidence/round12/ui-update/sound-type-fragment.rs').read_text(encoding='utf-8')
start = text.index('/// UI 声音播放事件')
end = text.index('/// UI 导航状态', start)
text = text[:start] + sound + '\n' + text[end:]
start = original.index('/// Complete native volume discriminants')
end = original.index('/// 0xb0 B 参数块', start)
text = once(text, '/// 0xb0 B 参数块', original[start:end] + '/// 0xb0 B 参数块')
text = once(text, 'pub struct WidgetParams {', 'pub struct WidgetParams {\n    /// Complete owned geometry for native volume variants.\n    pub geometry: Option<WidgetGeometry>,')
field_start = text.index('    /// Mesh volume variant')
field_end = text.index('\n}', field_start)
text = text[:field_start] + text[field_end:]
text = once(text, '            mesh_volume: None,', '            geometry: None,')
start = original.index('    /// Native tag3 stores')
end = original.index('    /// 0x1409a8e10', start)
methods = original[start:end]
text = once(text, 'impl WidgetParams {', 'impl WidgetParams {\n' + methods)
text = once(text, '        self.mesh_volume = Some(triangles);', '        self.geometry = Some(WidgetGeometry::TriangleMesh(triangles));')
text = once(text, '        self.volume = volume;\n        self', '        self.volume = volume;\n        self.geometry = None;\n        self')
text = once(text, '''    /// @0x78..0x88：`new` 第二参数（16 B，O 拷贝面；语义 U——宿主以
    /// `Option<&str>`/空 16 B 记录传入，字段身份未取证，保持原始 qword）。''', '''    /// Native +0x78..0x88 brush-preview enum, preserved bit for bit:
    /// tag0=None, tag1=kind with cursor fallback, tag2=kind plus Vec3 offset.
    /// ui_update applies the full geometry-specific origin before publication.''')
path.write_text(text, encoding='utf-8')

path = core / 'src/systems/ui/resources/ui_state.rs'
text = path.read_text(encoding='utf-8')
original = (candidate / 'ui_state.rs').read_text(encoding='utf-8')
text = once(text, '    interaction: UiInteraction,\n', '    interaction: UiInteraction,\n    active_params: Option<Arc<WidgetParams>>,\n    hover_intersections: Vec<[u8; 56]>,\n    intersection_payload: [u8; 56],\n')
text = once(text, '            interaction,\n', '            interaction,\n            active_params: None,\n            hover_intersections: Vec::new(),\n            intersection_payload: [0; 56],\n')
text = once(text, '        state.intersection_tag = 2;', '        state.set_intersection_tag(2);')
text = once(text, '''    /// rejects value 2 (`CMP dword ptr [RAX+0x28],2; SETNZ`). The remaining
    /// intersection bytes are host-owned and not modelled here.''', '''    /// rejects value 2 (`CMP dword ptr [RAX+0x28],2; SETNZ`). The complete
    /// intersection_payload above is synchronized with this compatibility view.''')
text = once(text, '        self.intersection_tag = tag;', '        self.intersection_tag = tag;\n        self.intersection_payload[..4].copy_from_slice(&tag.to_le_bytes());')
text = once(text, '        self.intersection_point = point;', '        self.intersection_point = point;\n        for (i, value) in point.to_array().into_iter().enumerate() {\n            self.intersection_payload[36+i*4..40+i*4].copy_from_slice(&value.to_le_bytes());\n        }')
start = original.index('    /// Snapshot in the current registry')
end = original.index('    /// `UiState::add<&str>`', start)
methods = original[start:end]
methods += '''    /// Reuse the original picker cache allocation while retaining the exact
    /// registry iteration order and strong ownership of every candidate.
    pub fn fill_widget_snapshot(&self, output: &mut Vec<(WidgetId, Arc<WidgetParams>)>) {
        output.clear();
        output.extend(self.widgets.read().iter().map(|(id, params)| (*id, Arc::clone(params))));
    }

'''
methods += '''    /// Keep the complete native intersection and the existing typed point API synchronized.
    pub fn replace_intersection(&mut self, payload: [u8; 56]) {
        self.intersection_tag = u32::from_le_bytes(payload[..4].try_into().unwrap());
        self.intersection_point = glam::Vec3::new(
            f32::from_le_bytes(payload[36..40].try_into().unwrap()),
            f32::from_le_bytes(payload[40..44].try_into().unwrap()),
            f32::from_le_bytes(payload[44..48].try_into().unwrap()),
        );
        self.intersection_payload = payload;
    }

'''
text = once(text, '    /// `UiState::add<&str>`', methods + '    /// `UiState::add<&str>`')
start = original.index('    pub fn clearing_canvas_w_layer_and_sound(')
end = original.index('    pub fn clearing_canvas_params(', start)
tstart = text.index('    pub fn clearing_canvas_w_layer_and_sound(')
tend = text.index('    pub fn clearing_canvas_params(', tstart)
text = text[:tstart] + original[start:end] + text[tend:]
start = original.index('    /// B14092e640 returns')
end = original.index('    /// `dragged`', start)
text = once(text, 'impl UiWidgetContext<\'_> {', 'impl UiWidgetContext<\'_> {\n' + original[start:end])
path.write_text(text, encoding='utf-8')

path = core / 'src/systems/history/recording.rs'
text = path.read_text(encoding='utf-8')
text = once(text, '    let id = ui.active_widget();', '''    // Original active Arc at UiState+0x20 outlives ui_update's frame-map clear.
    if let Some(params) = ui.active_params() {
        return !params.flags.no_history_recording();
    }
    let id = ui.active_widget();''')
path.write_text(text, encoding='utf-8')

path = core / 'src/systems/ui/hover_intersection.rs'
text = (root / 'reverse-engineering/tiny-glade/evidence/round12/ui-picker/proposed/tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs').read_text(encoding='utf-8')
text = text.replace('MainCamera::default()', 'MainCamera::new(1.0)')
text = once(text, '    cache.widgets.clear();\n    cache.widgets.extend(ui.registry_snapshot());', '    ui.fill_widget_snapshot(&mut cache.widgets);')
text = text.replace('/../../../reverse-engineering/tiny-glade/evidence/round12/ui-picker/', '/src/systems/ui/fixtures/')
path.write_text(text, encoding='utf-8')
fixtures = core / 'src/systems/ui/fixtures'
fixtures.mkdir(exist_ok=True)
for name in ['native-picker-all-fixtures.json', 'native-arbitration-fixtures.json']:
    (fixtures / name).write_bytes((root / 'reverse-engineering/tiny-glade/evidence/round12/ui-picker' / name).read_bytes())

path = core / 'Cargo.toml'
text = path.read_text(encoding='utf-8')
text = once(text, 'bevy_app.workspace = true', 'bevy_app.workspace = true\nbevy_tasks.workspace = true')
path.write_text(text, encoding='utf-8')
lock_path = root / 'tiny-glade/Cargo.lock'
lock = lock_path.read_text(encoding='utf-8')
start = lock.index('name = "country-core"')
end = lock.index('\n[[package]]', start)
entry = lock[start:end]
entry = once(entry, ' "bevy_state",\n', ' "bevy_state",\n "bevy_tasks",\n')
lock = lock[:start] + entry + lock[end:]
if '--stage' in sys.argv:
    lock_path = root / 'tiny-glade/reports/business/2026-10-21-business-module-recovery/proposed/tiny-glade/Cargo.lock'
lock_path.write_text(lock, encoding='utf-8')
print('Integrated owned geometry, active drag, frame registrations and picker; modules remain unexported until coordinated freeze.')
