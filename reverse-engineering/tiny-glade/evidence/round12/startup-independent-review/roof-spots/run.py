from pathlib import Path
import os,subprocess,json,hashlib
out=Path(__file__).resolve().parent;dep=Path('D:/game/tiny-glade/target-round12/debug/deps');fp=dep.parent/'.fingerprint';src=Path('D:/game/tiny-glade/crates/systems/critters/src/wren/roof_spots.rs');test=out/'actual_production_roof.rs'
core=max(dep.glob('libcountry_core-*.rlib'),key=lambda p:p.stat().st_mtime);key=core.stem[3:];j=json.loads((fp/key.replace('country_core','country-core')/'lib-country_core.json').read_text());libs={'country_core':core}
for _,n,_,v in j['deps']:
 if n not in ['glam','utils','bevy_ecs','serde_json']:continue
 for d in fp.iterdir():
  p=d/('lib-'+n)
  if p.exists():
   s=p.read_text().strip()
   if len(s)==16 and int.from_bytes(bytes.fromhex(s),'little')==v:libs[n]=dep/('lib'+d.name.replace('bevy_ecs','bevy_ecs')+'.rlib');break
args=['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe','--test','--edition=2024',str(test),'-L','dependency='+str(dep),'-o',str(out/'actual_production_roof.exe')]
for n,p in libs.items():args+=['--extern',n+'='+str(p)]
for p in dep.parent.joinpath('build').glob('*/output'):
 for l in p.read_text(errors='replace').splitlines():
  if l.startswith('cargo:rustc-link-search='):args+=['-L',l.split('=',1)[1]]
env=os.environ.copy();env['CARGO_MANIFEST_DIR']='D:/game/tiny-glade/crates/systems/critters';env['PATH']='D:/game/ljxsj_92385/Tiny Glade;'+env.get('PATH','')
manifest={'production_source':str(src),'production_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'deps':{n:{'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for n,p in libs.items()},'command':args,'note':'Actual production file included without algorithm copy; no Cargo or target mutation'}
(out/'direct-rustc-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
with (out/'direct-rustc.log').open('w') as log:
 r=subprocess.run(args,env=env,stdout=log,stderr=subprocess.STDOUT);print('rustc',r.returncode,flush=True)
 if r.returncode==0:
  t=subprocess.run([str(out/'actual_production_roof.exe'),'--nocapture'],env=env,stdout=log,stderr=subprocess.STDOUT);print('tests',t.returncode,flush=True)
print((out/'direct-rustc.log').read_text(errors='replace')[-10000:])
