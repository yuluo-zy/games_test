from pathlib import Path
import json,hashlib,os,subprocess
ROOT=Path('D:/game');BASE=ROOT/'reverse-engineering/tiny-glade/evidence/round12';OUT=BASE/'history/audio-independent-review';OUT.mkdir(exist_ok=True)
sha=lambda b:hashlib.sha256(b).hexdigest()
core=ROOT/'tiny-glade/crates/country-core/src'; original=(BASE/'independent-verification/audio_lifecycle.rs').read_text()
specs=[('path_audio','pub const PATH_LOOP_EVENTS','/// `path_audio_init_system`'),('sheep_audio','pub const SHEEP_EVENTS','/// `sheep_audio_init_system`'),('terrain_stairs_audio','pub const STAIR_DRAW_LOOP_EVENT','/// `terrain_stairs_audio_init_system`'),('decorator_audio','pub const DECORATOR_LOOP_EVENTS','/// `decorator_audio_init_system`')]
mods='pub mod systems {pub mod audio {\n';slices=[]
for name,start,end in specs:
    path=core/f'systems/audio/{name}.rs';source=path.read_text();a=source.index(start);b=source.index(end,a);fragment=source[a:b]
    sliced=OUT/(name+'_constructor_slice.rs');sliced.write_text('use crate::resources::audio_resources::{AudioCore,AudioInstance,EventInstance,Studio};\nuse crate::utils::audio_utils::AudioLoop;\nuse bevy_ecs::prelude::*;\nuse glam::Vec3;\n'+fragment)
    slices.append({'source':str(path),'source_raw_sha256':sha(path.read_bytes()),'normalized_utf8_start':a,'normalized_utf8_end':b,'exact_fragment_sha256':sha(fragment.encode()),'harness_fragment':str(sliced),'scope':'unchanged const/state/constructor source only; unrelated frame/render consumer excluded'})
    mods+=f'#[path="{sliced.as_posix()}"] pub mod {name};\n'
mods+='}}\n'
harness=OUT/'audio_lifecycle_current.rs';harness.write_text(original+'\n'+mods+'\n'+(OUT/'independent_init_tests.rs').read_text())
deps=ROOT/'tiny-glade/target-round12/debug/deps'
rlibs={'bevy_ecs':'libbevy_ecs-8142e9b5516685a0.rlib','log':'liblog-e785196f3f6a6324.rlib','glam':'libglam-6103396c150735c2.rlib','anyhow':'libanyhow-80b790c94b96b226.rlib'}
exe=OUT/'audio_lifecycle_current.exe';cmd=['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe','--edition=2024','--test',str(harness),'-L',f'dependency={deps}','-o',str(exe)]
for name,file in rlibs.items():cmd+=['--extern',f'{name}={deps/file}']
env=os.environ.copy();env['CARGO_MANIFEST_DIR']=str(ROOT/'tiny-glade/crates/country-core')
result={'production_modified':False,'cargo_run':False,'command':cmd,'manifest_dir':env['CARGO_MANIFEST_DIR'],'slices':slices,'included_production_hashes':{str(p):sha(p.read_bytes()) for p in [core/'resources/audio_resources.rs',core/'resources/audio_instance.rs',core/'utils/audio_utils.rs']},'dependency_hashes':{str(deps/f):sha((deps/f).read_bytes()) for f in rlibs.values()}}
build=subprocess.run(cmd,capture_output=True,text=True,env=env);result['compile_exit']=build.returncode;result['compile_stderr']=build.stderr
if build.returncode==0:
    run=subprocess.run([str(exe),'--test-threads=1'],capture_output=True,text=True);result.update(test_exit=run.returncode,stdout=run.stdout,stderr=run.stderr,executable_sha256=sha(exe.read_bytes()));(OUT/'audio-lifecycle-current.log').write_text(run.stdout+run.stderr)
(OUT/'results.json').write_text(json.dumps(result,indent=2))
print(json.dumps({k:result[k] for k in ['compile_exit','test_exit','stdout','stderr','compile_stderr'] if k in result},indent=2))
