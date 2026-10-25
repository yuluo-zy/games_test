"""Stage canonical mode events on top of the already frozen sound candidates."""
from pathlib import Path
import hashlib, json, difflib, re

report = Path(__file__).resolve().parent
root = report.parents[2]
files = [
    'country-core/src/resources/events.rs',
    'systems/toolbar/src/toolbar_button.rs',
    'systems/color/src/autoclose_color_menu.rs',
    'systems/onboarding/src/hint_focused_edit.rs',
    'systems/onboarding/src/hint_clutter_focus_edit.rs',
    'systems/onboarding/src/exponential_back_off.rs',
    'systems/toolbar/tests/tb_e_toolbar_buttons.rs',
    'systems/toolbar/tests/tod_focus_event_order.rs',
]
manifest, patch = [], []
sound = json.loads((report / 'sound-writer-migration.json').read_text(encoding='utf-8'))
sound_files = {f['source']: f for f in sound['files']}
for suffix in files:
    relative = 'tiny-glade/crates/' + suffix
    live = root / relative
    raw = live.read_bytes()
    original = raw.decode('utf-8').replace('\r\n', '\n')
    target = report / 'proposed' / relative
    base = raw
    if relative in sound_files:
        base = target.read_bytes()
        assert hashlib.sha256(base).hexdigest() == sound_files[relative]['proposed_sha256'], relative
        assert hashlib.sha256(raw).hexdigest() == sound_files[relative]['source_sha256'], relative
    text = base.decode('utf-8').replace('\r\n', '\n')
    if suffix.endswith('resources/events.rs'):
        start = text.index('/// 模式刚切换事件')
        end = text.index('/// 菜单刚打开事件', start)
        text = text[:start] + '''/// Native publisher 140a875e0 writes current AppMode at +0, previous at
/// +0x10, then Bevy EventId at +0x20; finally previous := current.
/// Keep one semantic event identity and both complete AppMode payloads.
/// The rebuilding compiler's private enum layout is not a native ABI contract.
pub use crate::systems::mode_manager::AppModeChanged as ModeJustChanged;

''' + text[end:]
    elif suffix.endswith('toolbar_button.rs'):
        assert text.count('event.tool') == 1
        text = text.replace('event.tool', 'event.current.native_tag()')
        text = text.replace('tool: 7,\n                _gap: [0; 0x1c],', 'current: AppMode::Path,\n                previous: AppMode::default(),')
    elif suffix.endswith('autoclose_color_menu.rs'):
        assert text.count('mode.tool') == 1
        text = text.replace('mode.tool', 'mode.current.native_tag()')
    elif suffix.endswith('hint_focused_edit.rs') or suffix.endswith('hint_clutter_focus_edit.rs'):
        assert text.count('mode.read().next()') == 1
        text = text.replace('Some(first) = mode.read().next()', 'Some(latest) = mode.read().last()')
        text = text.replace('first.tool', 'latest.current.native_tag()').replace('首事件工具', '末事件当前模式 tag')
        if suffix.endswith('hint_focused_edit.rs'):
            text += (report / 'mode-hint-tests-fragment.rs').read_text(encoding='utf-8')
    elif suffix.endswith('exponential_back_off.rs'):
        assert text.count('.is_some_and(|e| (1..=2).contains(&e.tool))') == 1
        text = text.replace('首事件载荷 u32', '末事件当前模式 tag')
        text = text.replace('mode_b\n        .read()\n        .next()', 'mode_b\n        .read()\n        .last()')
        text = text.replace('&e.tool', '&e.current.native_tag()')
        text += (report / 'mode-backoff-tests-fragment.rs').read_text(encoding='utf-8')
    elif suffix.endswith('tb_e_toolbar_buttons.rs'):
        start = text.index('fn mode_changed()')
        end = text.index('\n#[test]', start)
        text = text[:start] + '''fn publish_mode_change(world: &mut World) {
    world.init_resource::<country_core::systems::mode_manager::PreviousAppMode>();
    let mut publish = Schedule::default();
    publish.add_systems(country_core::systems::mode_manager::send_mode_just_changed_event);
    publish.run(world);
}
''' + text[end:]
        text = text.replace('world\n        .resource_mut::<Events<ModeJustChanged>>()\n        .send(mode_changed());', 'publish_mode_change(&mut world);')
        text += (report / 'mode-publisher-tests-fragment.rs').read_text(encoding='utf-8')
    elif suffix.endswith('tod_focus_event_order.rs'):
        text = text.replace('fn mode(world: &mut World, tool: u32) {\n    world.resource_mut::<Events<ModeJustChanged>>().send(ModeJustChanged { tool, _gap: [0; 0x1c] });\n}', '''fn mode(world: &mut World, current: country_core::systems::mode_manager::AppMode) {
    world.init_resource::<country_core::systems::mode_manager::PreviousAppMode>();
    world.insert_resource(current);
    let mut publish = Schedule::default();
    publish.add_systems(country_core::systems::mode_manager::send_mode_just_changed_event);
    publish.run(world);
}''')
        text = text.replace('mode(&mut world, 7);', 'mode(&mut world, country_core::systems::mode_manager::AppMode::Path);')
        text = text.replace('mode(&mut world, 0xd);', 'mode(&mut world, country_core::systems::mode_manager::AppMode::Replay);')
    assert text != original, relative
    encoded = text.replace('\n', '\r\n' if b'\r\n' in raw else '\n').encode('utf-8')
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(encoded)
    manifest.append({'source': relative, 'source_sha256': hashlib.sha256(raw).hexdigest(),
                     'input_candidate_sha256': hashlib.sha256(base).hexdigest(),
                     'includes_sound_migration': relative in sound_files,
                     'proposed_sha256': hashlib.sha256(encoded).hexdigest()})
    patch.extend(difflib.unified_diff(original.splitlines(True), text.splitlines(True), fromfile='a/'+relative, tofile='b/'+relative))
(report / 'mode-event-migration.patch').write_text(''.join(patch), encoding='utf-8', newline='\n')
(report / 'mode-event-migration.json').write_text(json.dumps({'files':manifest, 'production_modified':False, 'compiled':False, 'sound_overlap':'The two onboarding final candidates supersede the sound-only copies; do not rerun sound staging after mode staging.'}, ensure_ascii=False, indent=2)+'\n', encoding='utf-8', newline='\n')
for item in manifest:
    assert hashlib.sha256((root/item['source']).read_bytes()).hexdigest() == item['source_sha256']
print(json.dumps({'files':len(manifest), 'production_modified':False, 'sound_overlaps':sum(x['includes_sound_migration'] for x in manifest)}))
