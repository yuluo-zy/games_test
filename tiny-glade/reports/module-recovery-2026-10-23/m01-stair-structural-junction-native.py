"""原1407d32d1..1407d3755完整junction阶段；图/邻接/计数是真实显式输入，图存储只记录add_edge_by_span。"""
from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
p=Path(__file__).resolve();root=p.parents[3];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);u.mem_write(base,pe.get_memory_mapped_image());u.mem_map(0x200000,0x100000)
read=lambda a,n:bytes(u.mem_read(a,n));q=lambda a:struct.unpack('<Q',read(a,8))[0];words=lambda a,n:list(struct.unpack('<'+'I'*n,read(a,n*4)))
writeq=lambda a,*v:u.mem_write(a,struct.pack('<'+'Q'*len(v),*v))
def ret(value=None):
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,q(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
rows=[]
for center,neighbors in [(1,[2,3,4]),(3,[1,5,8]),(10,[2,3,4]),(3,[8,1,5])]:
 for mode in ['complete','first_missing','first_zero','all_missing']:
  u.mem_write(0x200000,bytes(0x100000));rbp=0x2e0000;stack=0x2d0008;graph=0x210000;records=0x211000;structural=0x212000;ctrl=0x213000;heap=[0x240000]
  nodes=[center]+neighbors;mapping={center:neighbors,**{node:[center]for node in neighbors}}
  writeq(graph,len(nodes),records,len(nodes))
  vectors={}
  for i,node in enumerate(nodes):
   u.mem_write(records+i*40+32,struct.pack('<I',node));at=0x214000+i*0x100;ptr=at+32;vectors[node]=at
   writeq(at,len(mapping[node]),ptr,len(mapping[node]))
   u.mem_write(ptr,b''.join(struct.pack('<II',other,0)for other in mapping[node]))
  counts=[]
  for i,other in enumerate(neighbors):
   if mode=='all_missing' or (mode=='first_missing' and i==0):continue
   counts.append((min(center,other),max(center,other),0 if mode=='first_zero'and i==0 else i+2))
  writeq(structural+0x180,ctrl,15,15-len(counts),len(counts),0,0)
  u.mem_write(ctrl,bytes([0]*len(counts)+[255]*(16-len(counts))))
  for i,v in enumerate(counts):u.mem_write(ctrl-(i+1)*12,struct.pack('<III',*v))
  writeq(rbp+0x218,0x230000);writeq(rbp+0x250,0x231000);u.mem_write(rbp+0x2b8,struct.pack('<I',1))
  u.reg_write(UC_X86_REG_RSP,stack);u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSI,structural);u.reg_write(UC_X86_REG_R14,0)
  edges=[];panic=[];bridges=set()
  def hook(mu,a,n,data):
   rcx=mu.reg_read(UC_X86_REG_RCX);rdx=mu.reg_read(UC_X86_REG_RDX);r8=mu.reg_read(UC_X86_REG_R8);r9=mu.reg_read(UC_X86_REG_R9)
   if a==0x1408fe8b0:bridges.add(hex(a));ret(graph)
   elif a==0x14125de50:bridges.add(hex(a));ret(vectors.get(words(rdx,1)[0],0))
   elif a==0x1405b1be0:bridges.add(hex(a));ret(0)
   elif a==0x1428a7f20:
    bridges.add(hex(a));capacity=max(4,rdx+r8);ptr=heap[0];heap[0]+=capacity*4+32
    if rdx:u.mem_write(ptr,read(q(rcx+8),rdx*4))
    writeq(rcx,capacity,ptr);ret()
   elif a==0x140869ff0:bridges.add(hex(a));edges.append(dict(a=words(rdx,3),b=words(r8,3),transient=r9&255));ret()
   elif a==0x1428d9310:panic.append(hex(a));u.emu_stop()
  h=u.hook_add(UC_HOOK_CODE,hook)
  try:u.emu_start(0x1407d32d1,0x1407d3755,count=20000)
  except Exception as e:raise RuntimeError((center,neighbors,mode,hex(u.reg_read(UC_X86_REG_RIP))))from e
  finally:u.hook_del(h)
  rows.append(dict(center=center,neighbors=neighbors,counts=counts,mode=mode,edges=edges,panic=panic,bridges=sorted(bridges)))
out=p.with_suffix('.json')
with out.open('x')as f:json.dump(dict(scope=__doc__,rne_sha256=hashlib.sha256(raw).hexdigest(),rows=rows),f,indent=2)
print(json.dumps(dict(cases=len(rows),panics=sum(bool(r['panic'])for r in rows))))
