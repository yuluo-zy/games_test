from pathlib import Path
import hashlib,json,difflib,subprocess
ROOT=Path(__file__).resolve().parent
repo=ROOT.parents[4]
changes={}
def stage(rel,text):
    p=repo/rel
    base=p.read_text(encoding='utf-8') if p.exists() else ''
    candidate=ROOT/'proposed'/rel;candidate.parent.mkdir(parents=True,exist_ok=True)
    candidate.write_text(text,encoding='utf-8')
    changes[rel]={'base':base,'base_sha256':hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None,'proposed':text,'proposed_sha256':hashlib.sha256(candidate.read_bytes()).hexdigest()}
rel='tiny-glade/crates/country-core/src/systems/debug/sheep_controls.rs'
stage(rel,(ROOT/'sheep_controls.rs').read_text(encoding='utf-8')+'\n'+(ROOT/'sheep_controls.tests.rs').read_text(encoding='utf-8'))
rel='tiny-glade/crates/country-core/Cargo.toml'
text=(repo/rel).read_text(encoding='utf-8')
if 'rfd.workspace' not in text:text=text.replace('ron.workspace = true','ron.workspace = true\nrfd.workspace = true')
stage(rel,text)
for domain in ['runtime','picking']:
    stage('tiny-glade/crates/country-core/tests/fixtures/original-sheep-controls-'+domain+'-b.json',(ROOT/('native-'+domain+'-fixtures.json')).read_text(encoding='utf-8'))
formatter='C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe'
syntax=subprocess.run([formatter,'--edition','2024','--emit','stdout',str(ROOT/'proposed/tiny-glade/crates/country-core/src/systems/debug/sheep_controls.rs')],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,text=True)
assert syntax.returncode==0,syntax.stderr
manifest=[{'path':path,**{k:v for k,v in info.items() if k not in ['base','proposed']}} for path,info in changes.items()]
patch=''.join(''.join(difflib.unified_diff(info['base'].splitlines(True),info['proposed'].splitlines(True),fromfile='a/'+path if info['base'] else '/dev/null',tofile='b/'+path)) for path,info in changes.items())
(ROOT/'candidate.patch').write_text(patch,encoding='utf-8')
(ROOT/'candidate-manifest.json').write_text(json.dumps({'candidate_only':True,'files':manifest,'dependencies':'wall-gpu19 startup/DTO/component migration; capture goal_tag==1 owner narrow hunk','validation':{'syntax':'rustfmt stdout parsed, no mutation','rust_compilation':'not run','native_full_runtime':32,'native_full_runtime_picking':48,'gui_callbacks':'static original decomp and independent30 bounded ABI fixtures; no actual Egui native input'}},indent=2)+'\n',encoding='utf-8')
print(json.dumps({'candidate_files':len(manifest),'syntax':True,'production_modified':False,'cargo_run':False}))
