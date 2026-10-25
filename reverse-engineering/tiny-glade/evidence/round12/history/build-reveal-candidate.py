"""Original reveal-event correction; evidence-only candidate, no Cargo."""
from pathlib import Path
import difflib,hashlib,json,subprocess
ROOT=Path('D:/game');HERE=Path(__file__).resolve().parent;OUT=HERE/'reveal-candidate'
changes={}
def edit(rel,fn):
    old=(ROOT/rel).read_text(encoding='utf8');new=fn(old);assert old!=new
    changes[rel]=(old,new)
def core(s):
    s=s.replace('TYPE_ID_WRITE_SETTINGS_EVENTS','TYPE_ID_REVEAL_FINISHED_EVENTS')
    s=s.replace('send_write_settings_file','send_startup_reveal_finished')
    s=s.replace('WriteSettingsFileCmd','StartupRevealFinished')
    s=s.replace('missing_settings_event_resource_panics','missing_reveal_finished_event_resource_panics')
    s=s.replace('#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]\npub struct VermeerPassLocals',
        '#[derive(bevy_ecs::prelude::Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]\npub struct VermeerPassLocals')
    s=s.replace('        // 下一帧：Disabled -> 返回 false\n        assert!(!vermeer_pass(&mut h, &mut l, &mut s, params(false)));',
        '        // 下一帧：Disabled -> 返回 false，不重复发布 Finished。\n        assert!(!vermeer_pass(&mut h, &mut l, &mut s, params(false)));\n        assert_eq!(h.events, 1);')
    return s
edit('tiny-glade/crates/country-core/src/systems/render_loop/vermeer.rs',core)
def effects(s):
    s=s.replace('settings_events','reveal_finished_events')
    s=s.replace('with_reveal_finished_events','with_startup_reveal_finished_events')
    s=s.replace('country_core::resources::events::WriteSettingsFileCmd',
        'country_core::systems::save::load_game::StartupRevealFinished')
    return s.replace('send_write_settings_file','send_startup_reveal_finished')
edit('tiny-glade/crates/tiny-glade/src/effects_host.rs',effects)
def post(s):
    start=s.index('fn run_vermeer(');end=s.index('\n/// Snapshot the native screen-space tuple',start)
    part=s[start:end]
    anchor='    world.resource_scope(\n        |world, mut state: bevy_ecs::change_detection::Mut<vermeer::VermeerState>| {'
    assert part.count(anchor)==1
    part=part.replace(anchor,'''    // Original process static143634c9c survives each frame; never recreate
    // prev_active=false before the native temporal-reuse conjunction.
    world.init_resource::<vermeer::VermeerPassLocals>();
    world.resource_scope(
        |world, mut state: bevy_ecs::change_detection::Mut<vermeer::VermeerState>| {
        world.resource_scope(|world, mut locals: bevy_ecs::change_detection::Mut<vermeer::VermeerPassLocals>| {
        world.resource_scope(|world, mut finished: bevy_ecs::change_detection::Mut<
            bevy_ecs::event::Events<country_core::systems::save::load_game::StartupRevealFinished>>| {''')
    part=part.replace('''                false,
            );
            let target = effects.register_texture(target);
            let mut locals = vermeer::VermeerPassLocals::default();''','''                false,
            ).with_startup_reveal_finished_events(&mut finished);
            let target = effects.register_texture(target);''')
    anchor='''            Ok(active)
        },
    )
}'''
    assert part.count(anchor)==1
    part=part.replace(anchor,'''            Ok(active)
        })
        })
        },
    )
}''')
    assert 'let mut locals = vermeer::VermeerPassLocals::default()' not in part
    assert '.with_startup_reveal_finished_events' in part
    return s[:start]+part+s[end:]
edit('tiny-glade/crates/tiny-glade/src/world_postprocess.rs',post)
manifest={'production_files_changed':0,'execution':'rustfmt parsing and git apply --check; no Cargo',
    'files':{}}
patch=[]
for rel,(old,new) in changes.items():
    p=OUT/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(new,encoding='utf8')
    subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe',
        '--edition','2024','--config','skip_children=true','--emit','stdout',str(p)],
        check=True,stdout=subprocess.DEVNULL)
    manifest['files'][rel]={'base_sha256':hashlib.sha256((ROOT/rel).read_bytes()).hexdigest(),
        'candidate_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
    patch.extend(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
p=HERE/'reveal-finished-candidate.patch';p.write_text(''.join(patch),encoding='utf8')
subprocess.run(['git','-c','safe.directory=D:/game','-c','core.protectNTFS=false','apply',
    '--check','--ignore-whitespace',str(p)],cwd=ROOT,check=True)
manifest['patch_sha256']=hashlib.sha256(p.read_bytes()).hexdigest()
(HERE/'reveal-finished-candidate-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
# The central owner is editing run_game.rs. Keep this one semantic replacement
# separate, so no stale copy of shared registration/worker code can overwrite it.
rel='tiny-glade/crates/tiny-glade/src/run_game.rs';old=(ROOT/rel).read_text(encoding='utf8')
anchor='    to.send_event(country_core::resources::events::WriteSettingsFileCmd);'
assert old.count(anchor)==1
new=old.replace(anchor,'''    // main_loop1404da0b5..15a: this exact TypeId is StartupRevealStarted.
    // It precedes Vermeer.into_disable_hold, after the six real owner moves.
    to.send_event(country_core::systems::save::load_game::StartupRevealStarted);''')
(HERE/'reveal-started-handoff-hunk.patch').write_text(''.join(difflib.unified_diff(
    old.splitlines(True),new.splitlines(True),fromfile='a/'+rel,tofile='b/'+rel)),encoding='utf8')
print(json.dumps({'production_files_changed':0,'files':len(changes),'patch_sha256':manifest['patch_sha256']},indent=2))
