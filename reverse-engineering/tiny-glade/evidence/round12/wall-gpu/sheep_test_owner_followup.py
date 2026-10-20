from pathlib import Path
import difflib,json,hashlib,re
r=Path('D:/game');rel=Path('crates/country-core/tests/sheep_snapshot_native.rs');p=r/'tiny-glade'/rel;s=p.read_bytes().decode('utf-8');a=s
assert s.count('let control_owner =')==2
s=re.sub(r'(?m)^([ \t]*)let control_owner =',lambda m:m[1]+'let wireframe = world.spawn_empty().id();\n'+m[1]+'let solid = world.spawn_empty().id();\n'+m[0],s)
s=s.replace('Entity::from_raw(1), Entity::from_raw(2)','wireframe, solid')
s=s.replace('SheepControl, apply_sheep_snapshot, sheep_controls','SheepControl, sheep_controls')
s=s.replace('(apply_sheep_snapshot, sheep_controls).chain()','sheep_controls')
needle='    let mut world = World::new();'
init='''
    world.insert_resource(country_core::systems::mode_manager::AppMode::default());
    world.insert_resource(country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::default());
    world.insert_resource(country_core::systems::ui::resources::ui_state::UiState::at_startup());
    world.insert_resource(country_core::resources::gui::DebugGui::default());
    world.insert_resource(bevy_input::ButtonInput::<bevy_input::keyboard::KeyCode>::default());
'''
s=s.replace(needle,needle+init)
b=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu/sheep-test-owner-followup';b.mkdir(exist_ok=True);(b/'sheep_snapshot_native.rs').write_bytes(s.encode());(b/'narrow.patch').write_text(''.join(difflib.unified_diff(a.splitlines(True),s.splitlines(True),fromfile='a/tiny-glade/'+rel.as_posix(),tofile='b/tiny-glade/'+rel.as_posix(),n=3)),encoding='utf-8');(b/'manifest.json').write_text(json.dumps({'base_sha256':hashlib.sha256(a.encode()).hexdigest(),'candidate_sha256':hashlib.sha256(s.encode()).hexdigest()},indent=2));print('real debug owners + complete control system resources prepared')
