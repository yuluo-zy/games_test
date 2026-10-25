"""直接编译当前生产源的测试；绑定现有utils实际glam指纹，避免多feature rlib串用。"""
from pathlib import Path
import subprocess,json,struct,hashlib,tempfile,platform
root=Path(__file__).resolve().parents[4];project=root/'tiny-glade';reports=Path(__file__).parent;deps=project/'target/debug/deps';fp=project/'target/debug/.fingerprint';source=project/'crates/country-core/src/utils/project_cursor_onto_ray.rs'
u=max(deps.glob('libutils-*.rlib'),key=lambda p:p.stat().st_mtime);d=json.loads((fp/('utils-'+u.stem.split('-')[1])/'lib-utils.json').read_text());fingerprint=next(x[3] for x in d['deps'] if x[1]=='glam');encoded=struct.pack('<Q',fingerprint).hex();g=next(p for p in fp.glob('glam-*/lib-glam') if p.read_text()==encoded);g=deps/('lib'+g.parent.name+'.rlib');ron=max(deps.glob('libron-*.rlib'),key=lambda p:p.stat().st_mtime)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
with tempfile.TemporaryDirectory(prefix='toolbar-raise-projection-') as folder:
 folder=Path(folder);harness=folder/'harness.rs';binary=folder/'projection-tests';harness.write_text('#[path = '+json.dumps(str(source))+']\nmod project_cursor_onto_ray;\n')
 command=['rustc','--edition=2024','--test',str(harness),'-L','dependency='+str(deps),'-o',str(binary)]
 for name,path in [('utils',u),('glam',g),('ron',ron)]:command+=['--extern',name+'='+str(path)]
 build=subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True);(reports/'toolbar-raise-projection-candidate-build.log').write_text(build.stdout)
 report={'source_sha256':sha(source),'cargo_lock_sha256':sha(project/'Cargo.lock'),'manifest_sha256':sha(project/'crates/country-core/Cargo.toml'),'arch':platform.machine(),'dependencies':{str(p):sha(p) for p in [u,g,ron]},'build_exit':build.returncode,'scope':'当前原归属源文件的直接Rust测试，复用现有依赖产物，非生产ECS/GUI集成验收；非NaN输出逐bit。'}
 if build.returncode==0:
  report['candidate_binary_sha256']=sha(binary);run=subprocess.run([str(binary),'--nocapture'],stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True);(reports/'toolbar-raise-projection-candidate-tests.log').write_text(run.stdout);report['tests_exit']=run.returncode;print(run.stdout[-2500:])
 (reports/'toolbar-raise-projection-candidate.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print('build_exit',build.returncode,'tests_exit',report.get('tests_exit'))
