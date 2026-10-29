"""原WallTriggerVolume::get_collision_geo1408e8d70完整入口至自然ret，保Triangle/inverse_lerp/SIMD；仅桥分配和panic出口。"""
from pathlib import Path
import hashlib,json,struct,copy,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image());u.mem_map(0x200000,0x100000)
bits=lambda f:struct.unpack('<I',struct.pack('<f',f))[0]
read=lambda a,n:bytes(u.mem_read(a,n));q=lambda a:struct.unpack('<Q',read(a,8))[0]
writeq=lambda a,*v:u.mem_write(a,struct.pack('<'+'Q'*len(v),*v))
def words(a,n):return list(struct.unpack('<'+'I'*n,read(a,n*4)))
def pack(v):return struct.pack('<'+'I'*len(v),*v)
def ret(value=None):
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
def quad_input(i,already_bits=False):
 cv=(lambda x:x)if already_bits else bits
 return {k:[[cv(x)for x in v]for v in i[k]]if k in ['uv','points','normals']else[cv(x)for x in i[k]]for k in ['uv','points','us','normals','heights']}
inputs=[]
old=json.loads(p.with_name('m01-circle-trigger-native.json').read_text())
for row in old['rows']:
 i=row['input'];inputs.append(dict(quads=[quad_input(i)],special=i['special'],apply=i['apply'],range=None if i['range']is None else list(map(bits,i['range'])),offset=None if i['offset']is None else bits(i['offset']),minimum=bits(i['minimum']),top=bits(i['top'])))
edge=json.loads(p.with_name('m01-circle-trigger-edge-native.json').read_text())
for row in edge['rows']:
 i=row['input'];inputs.append(dict(quads=[quad_input(i,True)],special=i['special'],apply=True,range=None,offset=i['offset'],minimum=i['minimum'],top=i['top']))
base_input=copy.deepcopy(inputs[0]);base_input.update(apply=True,offset=bits(.315),minimum=bits(.1),top=bits(-.5))
for apply in [False,True]:
 for range_ in [None,[0.,.5],[.5,.5],[-1.,2.],[.8,.2],[float('nan'),1.],[0.,float('nan')]]:
  i=copy.deepcopy(base_input);i['apply']=apply;i['range']=None if range_ is None else list(map(bits,range_));a=copy.deepcopy(i['quads'][0]);b=copy.deepcopy(a);a['us']=[bits(0.),bits(.5)];b['us']=[bits(.5),bits(1.)]
  b['uv']=[[bits(float(j)+17.),bits(float(j)+23.)]for j in range(4)];b['normals']=[[bits(-.5),bits(.25),bits(.75)],[bits(.8),bits(-.2),bits(.4)]];i['quads']=[a,b];inputs.append(i)
inputs.append(dict(base_input,quads=[]))
for h in [0,0x80000000,0x7fc12345,0xffc54321,0x7f800000,0xff800000]:
 for minimum in [0,0x80000000,0x7fc98765]:
  i=copy.deepcopy(base_input);i['quads'][0]['heights']=[h,h];i['minimum']=minimum;inputs.append(i)
rows=[]
for ci,i in enumerate(inputs):
 u.mem_write(0x200000,bytes(0x100000));volume=0x210000;quads=0x211000;params=0x212000;out=0x213000;stack=0x2f0008;stop=0x2ff000
 writeq(volume,len(i['quads']),quads,len(i['quads']));u.mem_write(volume+0x60,bytes([i['special']]))
 for qi,v in enumerate(i['quads']):
  w=[x for a in v['uv']for x in a]+[x for a in v['points']for x in a]+v['us']+[x for a in v['normals']for x in a]+v['heights'];assert len(w)==30;u.mem_write(quads+qi*120,pack(w))
 if i['range']is not None:u.mem_write(params,b'\1');u.mem_write(params+4,pack(i['range']))
 if i['offset']is not None:u.mem_write(params+12,b'\1');u.mem_write(params+16,pack([i['offset']]))
 u.mem_write(params+20,pack([i['minimum'],i['top']]));u.mem_write(params+28,bytes([i['apply']]))
 writeq(stack,stop);u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_RCX,out);u.reg_write(UC_X86_REG_RDX,volume);u.reg_write(UC_X86_REG_R8,params);heap=[0x240000];panic=[];allocations=[]
 def hook(mu,a,n,data):
  if a==0x140613c10:
   size=mu.reg_read(UC_X86_REG_RCX);value=heap[0];heap[0]=(value+size+31)&~15;assert heap[0]<0x2c0000;allocations.append(size);ret(value)
  elif a in [0x1428d9430,0x1428d9390,0x140d8b480,0x1428d8fe3]:panic.append(hex(a));u.emu_stop()
 h=u.hook_add(UC_HOOK_CODE,hook)
 try:u.emu_start(0x1408e8d70,stop,count=20000)
 except Exception as e:raise RuntimeError((ci,hex(u.reg_read(UC_X86_REG_RIP))))from e
 finally:u.hook_del(h)
 result=[]
 if not panic:
  for slot,stride in [(0,9),(24,9),(48,6)]:
   count=q(out+slot+16);ptr=q(out+slot+8);result.append([words(ptr+ti*stride*4,stride)for ti in range(count)])
 rows.append(dict(input=i,panic=panic,allocations=allocations,triangles=result[0]if result else[],normals=result[1]if result else[],uvs=result[2]if result else[]))
output=p.with_name('m01-wall-trigger-collision-oracle.json')
with output.open('x')as f:json.dump(dict(scope=__doc__,rne_sha256=hashlib.sha256(raw).hexdigest(),rows=rows),f,indent=2)
print(json.dumps(dict(cases=len(rows),panics=sum(bool(r['panic'])for r in rows))))
