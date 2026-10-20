from pathlib import Path
import os,subprocess,json,hashlib,sys
sys.stdout.reconfigure(encoding='utf8')
out=Path(__file__).resolve().parent;dep=Path('D:/game/tiny-glade/target-round12/debug/deps');fp=dep.parent/'.fingerprint';core=max(dep.glob('libcountry_core-*.rlib'),key=lambda p:p.stat().st_mtime)
key=core.stem[3:];j=json.loads((fp/key.replace('country_core','country-core')/'lib-country_core.json').read_text());libs={'country_core':core}
for _,n,_,v in j['deps']:
 if n not in ['glam','utils','bevy_ecs','hashbrown','parking_lot','serde_json']:continue
 for d in fp.iterdir():
  p=d/('lib-'+n)
  if p.exists():
   s=p.read_text().strip()
   if len(s)==16 and int.from_bytes(bytes.fromhex(s),'little')==v:libs[n]=dep/('lib'+d.name+'.rlib');break
libs['bevy_tasks']=max(dep.glob('libbevy_tasks-*.rlib'),key=lambda p:p.stat().st_mtime)
args=['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe','--test','--edition=2024',str(out/'current_candidate.rs'),'-L','dependency='+str(dep),'-o',str(out/'current_candidate.exe')]
for n,p in libs.items():args+=['--extern',n+'='+str(p)]
for p in dep.parent.joinpath('build').glob('*/output'):
 for l in p.read_text(errors='replace').splitlines():
  if l.startswith('cargo:rustc-link-search='):args+=['-L',l.split('=',1)[1]]
env=os.environ.copy();env['CARGO_MANIFEST_DIR']='D:/game/tiny-glade/crates/country-core';env['PATH']='D:/game/ljxsj_92385/Tiny Glade;'+env.get('PATH','')
manifest={'candidate_source':str(out/'proposed/tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs'),'deps':{n:{'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}for n,p in libs.items()},'command':args,'note':'Actual candidate files directly included; no shadow algorithm, no Cargo, no target mutation'}
(out/'direct-candidate-manifest.json').write_text(json.dumps(manifest,indent=2))
with(out/'direct-candidate.log').open('w')as log:
 r=subprocess.run(args,env=env,stdout=log,stderr=subprocess.STDOUT);print('rustc',r.returncode,flush=True)
 if not r.returncode:
  t=subprocess.run([str(out/'current_candidate.exe'),'original_tests::','--nocapture'],env=env,stdout=log,stderr=subprocess.STDOUT);print('tests',t.returncode,flush=True)
print((out/'direct-candidate.log').read_text(errors='replace')[-6000:])
