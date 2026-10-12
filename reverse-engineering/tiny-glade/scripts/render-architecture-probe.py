from pathlib import Path
import json,re,subprocess,tomllib
root=Path('D:/game/reverse-engineering/tiny-glade')
target=Path('D:/game/ljxsj_92385/Tiny Glade')
out=root/'evidence/render-architecture';out.mkdir(exist_ok=True)
lock=tomllib.loads((target/'build-info/Cargo.lock').read_text())
selected=[p for p in lock['package'] if p['name'].startswith('bevy') or p['name'] in ['tiny-glade','country-core','rhapsody','ash','shader-compiler','shader-pipeline','shader-variants','vk-sync-fork','gpu-allocator','fsr','xegtao']]
(out/'dependencies.json').write_text(json.dumps(selected,indent=2),encoding='utf8')
p=subprocess.Popen(['C:/Program Files/LLVM/bin/llvm-pdbutil.exe','dump','--publics',str(target/'tiny_glade.pdb')],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,encoding='utf8',errors='replace')
records=[];active=None
for line in p.stdout:
 if 'S_PUB32' in line:active=line.strip()
 elif active and 'addr =' in line:
  m=re.search(r'addr = (\d+):(\d+)',line)
  if m:
   sec,off=map(int,m.groups());records.append({'symbol':active,'section':sec,'offset':off,'va':hex(0x140001000+off) if sec==1 else None})
  active=None
p.wait()
if p.returncode:raise RuntimeError(p.stderr.read())
(out/'public-symbols.json').write_text(json.dumps(records,indent=2),encoding='utf8')
names=[re.search(r'`([^`]+)`',r['symbol']).group(1) for r in records]
q=subprocess.run(['C:/Program Files/LLVM/bin/llvm-cxxfilt.exe'],input='\n'.join(names)+'\n',capture_output=True,text=True,encoding='utf8',check=True)
for r,n in zip(records,q.stdout.splitlines()):r['demangled']=n
(out/'public-symbols.json').write_text(json.dumps(records,indent=2),encoding='utf8')
simple=[r for r in records if any(k in r['demangled'] for k in ['rhapsody::','tiny_glade::render_loop','tiny_glade::game_loop','country_core::render','country_core::gfx','shader_pipeline::']) and not any(k in r['demangled'] for k in ['drop_in_place','alloc::','core::ptr::','bevy_ecs::','_$LT$','_LT_']) and len(r['demangled'])<400]
(out/'symbol-seeds.json').write_text(json.dumps(simple,indent=2),encoding='utf8')
print(json.dumps({'all_records':len(records),'simple_records':len(simple),'output':str(out)}))
