"""B sphere terrain wrapper with resolved world/terrain query boundaries.

Reuse the existing identity-bound camera instruction runner. Only the two
actual dependency query results are bridged. Original wrapper performs option
forwarding, terrain limit arithmetic, sphere TOI and final nearest/tie choice.
No game file, shared database, allocator or reconstructed binary is modified.
"""
import importlib.util, json, struct
from pathlib import Path
ROOT=next(p for p in Path(__file__).resolve().parents if (p/'ljxsj_92385').is_dir())
ENTRY=ROOT/'games_test/reverse-engineering/tiny-glade/scripts/parallel-camera/native.py'
spec=importlib.util.spec_from_file_location('tg_camera_native',ENTRY)
n=importlib.util.module_from_spec(spec);spec.loader.exec_module(n)
F=lambda b: struct.unpack('<f',struct.pack('<I',b))[0]
B=lambda f: struct.unpack('<I',struct.pack('<f',f))[0]
Q=lambda x:struct.pack('<Q',x)
P=lambda v:struct.pack('<3I',*v)
inputs=[]
def add(origin, direction, radius, position, maximum, mesh_toi=None, flags=(False,False), terrain=True):
 inputs.append({'origin':[B(x) for x in origin], 'direction':[B(x) for x in direction],
 'radius':B(radius),'terrain_position':[B(x) for x in position],'maximum':B(maximum),
 'mesh_toi':None if mesh_toi is None else B(mesh_toi),'flags':list(flags),'terrain':terrain})
for maximum in [0.,.25,1.,2.,4.,10.]:
 for mesh in [None,.5,2.,8.]:add([4.,3.,2.],[0.,-1.,0.],.5,[4.,0.,2.],maximum,mesh)
for direction in [[0.,0.,0.],[1e-30,0.,0.],[1e-20,0.,0.],[1e-10,0.,0.],[2.,0.,0.],[float('nan'),0.,0.]]:
 add([1.,0.,0.],direction,.5,[0.,0.,0.],3.4028234663852886e38)
for radius in [0.,.5,1.,2.,float('nan')]:
 add([1.,0.,0.],[1.,0.,0.],radius,[0.,0.,0.],10.)
add([1.,0.,0.],[1.,0.,0.],.5,[0.,0.,0.],.5,.5)
add([1.,0.,0.],[1.,0.,0.],.5,[0.,0.,0.],.5,.25)
for flags in [(False,True),(True,False),(True,True)]:
 add([4.,3.,2.],[0.,-1.,0.],.5,[4.,0.,2.],4.,flags=flags)

results=[]
for case in inputs:
 sp=n.setup();out=n.data;wrapper=n.data+0x1000;ray=n.data+0x2000;opts=n.data+0x3000
 n.cpu.mem_write(wrapper,Q(n.data+0x4000)+bytes(24)+Q(n.data+0x5000))
 n.cpu.mem_write(ray,P(case['origin'])+P(case['direction']))
 option_bytes=struct.pack('<IIBB',case['maximum'],B(.375),*case['flags'])+bytes(2)
 n.cpu.mem_write(opts,option_bytes)
 n.cpu.reg_write(n.UC_X86_REG_RCX,out);n.cpu.reg_write(n.UC_X86_REG_RDX,wrapper)
 n.cpu.reg_write(n.UC_X86_REG_R8,ray);n.cpu.reg_write(n.UC_X86_REG_XMM3,case['radius'])
 n.cpu.mem_write(sp+0x28,Q(opts));n.cpu.mem_write(sp+0x30,struct.pack('<I',0x20))
 seen={}
 def hook(c,pc):
  if pc==0x1408ccfc0:
   call_sp=c.reg_read(n.UC_X86_REG_RSP);opt_ptr=n.u64(call_sp+0x28)
   seen['forwarded_options']=bytes(c.mem_read(opt_ptr,10)).hex()
   seen['forwarded_radius']=c.reg_read(n.UC_X86_REG_XMM3)&0xffffffff
   seen['forwarded_mask']=struct.unpack('<I',c.mem_read(call_sp+0x30,4))[0]
   blob=bytearray(64);struct.pack_into('<I',blob,0,7)
   if case['mesh_toi'] is None:struct.pack_into('<I',blob,52,2)
   else:
    struct.pack_into('<I',blob,48,case['mesh_toi']);struct.pack_into('<3f',blob,36,0.,1.,0.)
   c.mem_write(c.reg_read(n.UC_X86_REG_RCX),bytes(blob));n.ret();return True
  if pc==0x1409e9a20:
   seen['terrain_limit']=c.reg_read(n.UC_X86_REG_XMM3)&0xffffffff
   # Resolved ray hit supplied by the real dependency boundary. This probe
   # tests the private wrapper, not the validity or discovery of that hit.
   blob=struct.pack('<II',int(case['terrain']),0)+P(case['terrain_position'])
   c.mem_write(c.reg_read(n.UC_X86_REG_RCX),blob);n.ret();return True
  return False
 n.extra_hook=hook;n.call(0x1408cc390);n.extra_hook=None
 raw=bytes(n.cpu.mem_read(out,64));hit=struct.unpack_from('<I',raw,52)[0]==0
 assert seen['forwarded_options']==option_bytes[:10].hex()
 assert seen['forwarded_radius']==case['radius'] and seen['forwarded_mask']==0x20
 case['expected']={'hit':hit,'source_tag':raw[0] if hit else None,
   'toi_bits':struct.unpack_from('<I',raw,48)[0] if hit else None,
   'position_bits':list(struct.unpack_from('<3I',raw,24)) if hit else None}
 case['observed_queries']=seen;results.append(case)
report={'binary_sha256':n.b['binary']['sha256'],'entry':'0x1408cc390','cases':results,
 'case_count':len(results),'target_executed':True,'candidate_executed':False,
 'scope':'Original sphere wrapper only. World/terrain queries are resolved library boundaries. Original instructions perform forwarding, limit math, NaN/floor handling and mesh/terrain tie choice. No full query/scene acceptance.'}
(Path(__file__).parent/'sphere-native-wrapper.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:v for k,v in report.items() if k!='cases'}))
