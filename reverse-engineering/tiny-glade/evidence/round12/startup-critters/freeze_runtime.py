from pathlib import Path
import hashlib,json,difflib,subprocess,shutil
R=Path('D:/game');O=Path(__file__).resolve().parent;C=O/'runtime-candidate';F=O/'frozen-runtime-v2'
assert not F.exists(), 'immutable delivery already exists'
manifest=json.loads((C/'manifest.json').read_text())
rel='tiny-glade/crates/systems/critters/src/lib.rs';p=C/rel;s=p.read_text(encoding='utf8');s+='\n#[cfg(test)]\nmod upload_tests;\n';p.write_text(s,encoding='utf8')
rel='tiny-glade/crates/systems/critters/src/upload_tests.rs';manifest.append({'path':rel,'base_sha256':None})
patch='*** Begin Patch\n'
for e in manifest:
 p=C/e['path'];base=R/e['path'];current=hashlib.sha256(base.read_bytes()).hexdigest() if base.exists() else None
 assert current==e['base_sha256'],('production changed, merge hunk',e['path'],current,e['base_sha256'])
 subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe','--edition','2024','--config','skip_children=true',str(p)],check=True)
 raw=p.read_bytes();e['proposed_sha256']=hashlib.sha256(raw).hexdigest();old=base.read_text(encoding='utf8').replace('\r','') if base.exists() else ''
 if e['base_sha256'] is None:patch+='*** Add File: '+e['path']+'\n'+''.join('+'+x+'\n' for x in raw.decode().splitlines())
 else:
  diff=list(difflib.unified_diff(old.splitlines(),raw.decode().splitlines(),n=3,lineterm=''))[2:]
  patch+='*** Update File: '+e['path']+'\n'+'\n'.join('@@' if x.startswith('@@ ') else x for x in diff)+'\n'
 q=F/'candidate'/e['path'];q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q);assert q.read_bytes()==raw
patch+='*** End Patch\n';(F/'runtime.patch').write_text(patch,encoding='utf8',newline='\n')
(F/'gameplay_runtime.inc.rs').write_text('''// Narrow insertion into the real gameplay runtime installer. Existing GameSystemSet gating applies.
app.init_resource::<country_core::resources::critters::CritterControl>();
app.add_systems(bevy_app::Update,
    system_critters::control_critters::control_critters.in_set(crate::run_game::GameSystemSet(36)));
app.add_systems(bevy_app::Update,
    (system_critters::butterfly::butterflies_send_to_gpu,
     system_critters::firefly::fireflies_send_to_gpu).in_set(crate::run_game::GameSystemSet(44)));
''',encoding='utf8')
report={'scope':'canonical control identity/default/full inline system and original Stage44 mapped upload pair; NOT whole CritterPlugin behavior group',
'status':'rustfmt parsed; original324 arithmetic executions PASS; Rust ECS mapped-buffer and population-boundary tests prepared but uncompiled/unrun; root owns Cargo and independent review',
'files':manifest,'patch_sha256':hashlib.sha256((F/'runtime.patch').read_bytes()).hexdigest(),
'observed':['control default0 intensity,false firefly,true birds; one canonical TypeId shared with actual draw callbacks', 'enabled=(TimeOfDay.day_night_factor>=.05&&factor<.8); intensity+=delta*(enabled?.2:-.3), MAXSS/MINSS clamp; no birds write', 'original full Query collect -> empty no lookup/no write -> nonzero strict population24/96 -> library full type validation -> optional downcast -> cpu_write', 'mapped SSBO write uses real existing owner, missing entries skip, incorrect population panics before entry lookup, short mapped slice retains original panic'],
'design_update':'ResMut<AssetSsboLibrary> explicitly owns original shared interior mapped-memory writes, serializing safe library borrow without alternate SSBO/vector resource',
'unknown':['full Stage36 private species leaves and exact resources not all restored; old configure MUST NOT be used as full runtime installer', 'candidate meaningful ECS tests need central compile/run; native upload collectors use original Bevy, only static PDB/bytes checked in this batch'],
'evidence':['original-control-fixtures.json','probe_control.py','upload-result.artifacts/001.stdout.txt','141d87140.original.asm','142316760.original.asm','14230edb0.original.asm','14230f1e0.original.asm','14067d440.original.asm']}
(F/'manifest.json').write_text(json.dumps(report,indent=2),encoding='utf8');print(json.dumps({'files':len(manifest),'patch_sha256':report['patch_sha256']},indent=2))
