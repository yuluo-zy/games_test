import struct,capstone,re,subprocess,json,hashlib
from pathlib import Path
ROOT=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
roots={'A':Path('/Users/yuluo/Desktop/game/Tiny Glade'),'B':Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade'),'C':Path('/Users/yuluo/Desktop/game/tiny.glade/XJ07036/Tiny Glade v1.16.0')}
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64);cs.detail=True
report={}
for v,root in roots.items():
 exe=root/('tiny-glade.rne' if v=='B' else 'tiny-glade.exe');b=exe.read_bytes();pe=struct.unpack_from('<I',b,60)[0];ns=struct.unpack_from('<H',b,pe+6)[0];op=struct.unpack_from('<H',b,pe+20)[0];s=pe+24+op;ss=[struct.unpack_from('<8sIIII',b,s+i*40) for i in range(ns)]
 def read(a,n):
  r=a-0x140000000
  for _,vs,rv,sz,off in ss:
   if rv<=r<rv+sz:return b[off+r-rv:off+r-rv+n]
  return b''
 p=subprocess.Popen(['/opt/homebrew/Cellar/llvm/22.1.2/bin/llvm-pdbutil','dump','--publics',str(root/'tiny_glade.pdb')],stdout=subprocess.PIPE,text=True)
 name=None;pub=[]
 for l in p.stdout:
  if 'S_PUB32' in l:
   m=re.search(r'`([^`]+)`',l);name=m[1] if m else None
  elif name and 'addr =' in l:
   m=re.search(r'addr = (\d+):(\d+)',l)
   if m and 'flags = function' in l and m[1]=='0001':
    if (('NaniBundle' in name and 'component_ids' in name) or (any(t in name for t in ['RoofSpire','RoofSupportBeams','RoofTiles','NaniInstanceDesc','components..transform..Transform']) and 'register_required_components' in name)):pub.append((name,0x140001000+int(m[2])))
   name=None
 p.wait();bundles=[];required=[]
 def scan(a,n=10000):
  out=[]
  for i in cs.disasm(read(a,n),a):
   out.append(i)
   if i.mnemonic in ['ret','int3']:break
  return out
 def names(a):
  out=[]
  for i in scan(a,65536):
   if i.mnemonic=='lea' and 'rip' in i.op_str:
    target=i.address+i.size+i.operands[1].mem.disp;raw=read(target,180)
    if raw and any(t in raw[:75] for t in [b'components::drawable::NaniInstanceDesc',b'components::transform::Transform',b'InstanceDataMarker']):out.append({'string_va':hex(target),'text':raw.decode('utf-8',errors='replace')})
  return out
 def deep_names(a):
  z=names(a)
  for i in scan(a,8192):
   if i.mnemonic=='call' and i.op_str.startswith('0x'):z.extend(names(int(i.op_str,16)))
  return z
 for name,a in pub:
  calls=[int(i.op_str,16) for i in scan(a) if i.mnemonic=='call' and i.op_str.startswith('0x')]
  if 'component_ids' in name:
   fields=[]
   own_names=names(a)
   if own_names:fields.append({'register_va':hex(a),'names':own_names,'inlined':True})
   for c in calls:
    ns=deep_names(c)
    if ns:fields.append({'register_va':hex(c),'names':ns})
   if any(z['text'].startswith('country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StoneFloorInstanceData>') for f in fields for z in f['names']):bundles.append({'va':hex(a),'name':name,'fields':fields})
  else:required.append({'va':hex(a),'name':name,'calls':[hex(x) for x in calls]})
 sources=json.load(open(ROOT/f'evidence/multi-version/{v}-sources.json'))['files']
 report[v]={'binary':str(exe),'sha256':hashlib.sha256(b).hexdigest(),'stone_floor_bundles':bundles,'required_registration_candidates':required,'private_sources':[f for f in sources if f['path'] in ['crates/country-core/src/components/drawable.rs','crates/country-core/src/resources/roofs/roof_tiles.rs','crates/utils/src/components/transform.rs','crates/systems/roof/src/utils/components.rs','crates/country-core/src/render/nani/systems.rs']]}
 print(v,len(bundles),len(required),flush=True)
report['scope']='Bounded PDB/capstone registration candidate comparison; B typed StoneFloor component IDs resolved; A/C generic candidates not mapped by string extraction. No inference of absence from that bounded search.'
report['A']['registration_mapping_status']='No register_required API symbols selected in older Bevy; full typed bundle mapping pending; original create tuple names StoneFloor NaniBundle.'
report['B']['registration_mapping_status']='StoneFloor component IDs 142588be0 resolved to descriptor/Transform/typedmarker. Five direct derive required functions only self-register push/pop; marker likewise no required edge.'
report['C']['registration_mapping_status']='Required Descriptor/Transform/RoofTiles collapse to RET140003310; generic bundle has 3 component IDs but typed variant mapping pending; create tuple names same StoneFloor NaniBundle.'
report['version_behavior_changes']={'A':'StoneFloor upload48, Roof height28/maxY40; no B style tail16','B':'StoneFloor upload64, direct roof appearance tail and no wall resource lookup in this update','C':'PublicWall validity/style lookup and extra margins/floor appearance branches; not equivalent to B update'}
report['observed_stone_floor_subset_kind']={'A':{'value':9,'store':'0x141988c4b'},'B':{'value':9,'store':'0x1421ab2d3'},'C':{'value':7,'store':'0x142826f61'}}
json.dump(report,open(ROOT/'evidence/building-nani/three-version-registration.json','w'),indent=2)
