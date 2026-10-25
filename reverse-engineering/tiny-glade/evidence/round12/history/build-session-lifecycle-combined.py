"""Rebase the frozen three candidates into one reviewable current-source patch.

No production writes. Do not run this while the source integration window is open.
"""
from pathlib import Path
import difflib, hashlib, json, re, subprocess

ROOT=Path('D:/game')
HERE=Path(__file__).resolve().parent
OUT=HERE/'session-lifecycle-combined'
patches=['startup-unload-owner-candidate.patch','init-photomode-session-reset.patch',
    'clear-canvas-candidate.patch']
rels=set()
for name in patches:
    rels.update(re.findall(r'^\+\+\+ b/(.+)$',(HERE/name).read_text(encoding='utf8'),re.M))
rels.add('tiny-glade/crates/tiny-glade/src/gameplay_startup.rs')
before={}
for rel in sorted(rels):
    source=ROOT/rel
    before[rel]=source.read_text(encoding='utf8') if source.exists() else ''
    target=OUT/rel
    target.parent.mkdir(parents=True,exist_ok=True)
    if source.exists():target.write_bytes(source.read_bytes())
    elif target.exists():target.unlink()
for name in patches:
    subprocess.run(['git','-c','safe.directory=D:/game','-c','core.protectNTFS=false',
        'apply','--ignore-whitespace','--directory='+OUT.relative_to(ROOT).as_posix(),
        str(HERE/name)],cwd=ROOT,check=True)
rel='tiny-glade/crates/tiny-glade/src/gameplay_startup.rs'
p=OUT/rel;s=p.read_text(encoding='utf8')
anchor='    crate::mei_host::install_mei_frame_systems(app);'
assert s.count(anchor)==1
s=s.replace(anchor,'''    // Original run_game_thread1405a0237..31a: clear_canvas is Update set33,
    // guarded solely by on_event<ClearCanvasCmd>. OnExit's false command must
    // execute during Reload before the next session, without an in_state(Game).
    country_core::systems::clear_canvas::register_clear_canvas_resources(
        app.world_mut(), hosts.textures.clone());
    app.add_systems(bevy_app::Update,
        country_core::systems::clear_canvas::clear_canvas
            .in_set(crate::run_game::GameSystemSet(33))
            .run_if(bevy_ecs::schedule::common_conditions::on_event::<
                country_core::systems::mode_manager::ClearCanvasCmd>));
'''+anchor)
p.write_text(s,encoding='utf8')
manifest={'production_files_changed':0,'execution':'rustfmt parse and git apply --check only; no Cargo',
    'depends_on_original_evidence':['startup-session-lifecycle-evidence.json',
        'clear-canvas-original.json','clear-canvas-typed-contract.json',
        '../central-independent-water/results.json'],
    'known_open':['Actual StartupRevealStarted/Finished writers not yet restored.',
        'Forest placement payload/default-prefab producer remains incomplete.',
        'Whole-App first and second session execution awaits central acceptance.'],
    'files':{}}
patch=[]
for rel in sorted(rels):
    target=OUT/rel;updated=target.read_text(encoding='utf8')
    subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe',
        '--edition','2024','--config','skip_children=true','--emit','stdout',str(target)],
        check=True,stdout=subprocess.DEVNULL)
    source=ROOT/rel
    manifest['files'][rel]={'base_sha256':hashlib.sha256(source.read_bytes()).hexdigest() if source.exists() else None,
        'candidate_sha256':hashlib.sha256(target.read_bytes()).hexdigest()}
    patch.extend(difflib.unified_diff(before[rel].splitlines(True),updated.splitlines(True),
        fromfile='a/'+rel if source.exists() else '/dev/null',tofile='b/'+rel))
destination=HERE/'session-lifecycle-combined.patch'
destination.write_text(''.join(patch),encoding='utf8')
subprocess.run(['git','-c','safe.directory=D:/game','-c','core.protectNTFS=false',
    'apply','--check','--ignore-whitespace',str(destination)],cwd=ROOT,check=True)
manifest['patch_sha256']=hashlib.sha256(destination.read_bytes()).hexdigest()
(HERE/'session-lifecycle-combined-manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8')
print(json.dumps({'production_files_changed':0,'candidate_files':len(rels),
    'patch_sha256':manifest['patch_sha256'],'validation':manifest['execution']},indent=2))
