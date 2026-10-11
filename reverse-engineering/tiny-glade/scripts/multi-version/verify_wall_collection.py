"""Whole raw Curve2 -> original resample -> terrain -> Curve3 publication oracle.
Allocator/Vec growth/CRT are dependency bridges; game samplers, curve rules,
mapping closure and normalized parameters all execute original B instructions.
"""
import json, math, random, struct, hashlib, subprocess
from pathlib import Path
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
from verification_provenance import ROOT, candidate_provenance

def fb(f): return struct.unpack('<I',struct.pack('<f',f))[0]
def f32(f): return struct.unpack('<f',struct.pack('<f',f))[0]
def val(i): return struct.unpack('<f',struct.pack('<I',i))[0]
def hex_floats(data): return ''.join(f'{v:08x}' for v in struct.unpack('<'+'I'*(len(data)//4),data))
def main():
 b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B'];raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];pe=b['pe'];base=pe['image_base']
 cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in pe['sections']);cpu.mem_map(base,(end+4095)&~4095)
 for s in pe['sections']:
  if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
 data,stack,heap,stop=0x20000000,0x30000000,0x40000000,0x50000000
 cpu.mem_map(data,0x200000);cpu.mem_map(stack,0x10000);cpu.mem_map(heap,0x400000);cpu.mem_map(stop,0x1000)
 grid=b''.join(struct.pack('<H',(i*73+(i//512)*101)&65535) for i in range(512*512));cpu.mem_write(data+0x10000,grid)
 cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x100))
 alloc=[heap]
 def ret():
  sp=cpu.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',cpu.mem_read(sp,8))[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
 def bridge(uc,address,size,user):
  if address==0x140613c10:
   count=cpu.reg_read(UC_X86_REG_RCX);p=alloc[0];alloc[0]+=max(0x1000,(count+15)&~15);assert alloc[0]<heap+0x400000;cpu.reg_write(UC_X86_REG_RAX,p);ret()
  elif address==0x140613c20:ret()
  elif address==0x140b9f180:
   vec=cpu.reg_read(UC_X86_REG_RCX);cap=struct.unpack('<Q',cpu.mem_read(vec,8))[0];cpu.mem_write(vec,struct.pack('<Q',max(4,cap*2)));ret()
  elif address==0x142923fa0:
   x=val(cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff);rounded=math.floor(x+.5) if x>=0 else math.ceil(x-.5);cpu.reg_write(UC_X86_REG_XMM0,fb(float(rounded)));ret()
  elif address in [0x142923c00,0x142923c20]:
   dst,src,n=(cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);cpu.mem_write(dst,bytes(cpu.mem_read(src,n)));cpu.reg_write(UC_X86_REG_RAX,dst);ret()
 cpu.hook_add(UC_HOOK_CODE,bridge)
 def reset(fix,lo,hi):
  alloc[0]=heap;cpu.mem_write(stack,bytes(0x10000));cpu.mem_write(data+0x100,bytes([fix]));cpu.mem_write(data+0x1000,bytes(0x1000));cpu.mem_write(data+0x1008,struct.pack('<Q',data+0x10000));cpu.mem_write(data+0x1050,struct.pack('<II',lo,hi));cpu.reg_write(UC_X86_REG_RSP,stack+0x8008);cpu.mem_write(stack+0x8008,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
  for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):cpu.reg_write(r,0)
 rng=random.Random(20261012);lines=[];expected=[];rows=[]
 edges=[-math.inf,-1000.,-65.,-64.999,-1.,-0.,0.,1.,64.999,65.,1000.,math.inf,math.nan]
 for i in range(1024):
  fix=i%2;lo,hi=fb(-3.),fb(18.);x=fb(edges[i%len(edges)] if i<169 else rng.uniform(-90,90));z=fb(edges[(i//len(edges))%len(edges)] if i<169 else rng.uniform(-90,90));reset(fix,lo,hi);cpu.reg_write(UC_X86_REG_RCX,data+0x1000);cpu.reg_write(UC_X86_REG_XMM1,x);cpu.reg_write(UC_X86_REG_XMM2,z);cpu.emu_start(0x1409e9640,stop,count=10000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
  lines.append(f'terrain {fix} {lo:08x} {hi:08x} {x:08x} {z:08x}');expected.append(f'{cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff:08x}');rows.append('terrain')
 for i in range(256):
  fix=i%2;tag=i%4;lo,hi=fb(-3.),fb(18.);floor=fb([0.,-4.,.75,12.][i%4]);n=2+i%6;points=[(f32(rng.uniform(-7,7)),f32(rng.uniform(-7,7))) for _ in range(n)]
  if i%4==0:points.append(points[0])
  reset(fix,lo,hi);curve=data+0x2000;source=data+0x3000;out=data+0x4000;point_ptr=data+0x5000;cpu.mem_write(point_ptr,b''.join(struct.pack('<ff',*p) for p in points));cpu.mem_write(curve,struct.pack('<QQQ',len(points),point_ptr,len(points))+bytes(24)+struct.pack('<f',1.));cpu.mem_write(source+0x58,struct.pack('<I',floor));sp=cpu.reg_read(UC_X86_REG_RSP);cpu.mem_write(sp+0x28,struct.pack('<Q',data+0x1000));cpu.mem_write(sp+0x30,struct.pack('<Q',source));cpu.reg_write(UC_X86_REG_RCX,out);cpu.reg_write(UC_X86_REG_RDX,curve);cpu.reg_write(UC_X86_REG_R8,tag==1);cpu.reg_write(UC_X86_REG_XMM3,fb(.5));cpu.emu_start(0x14146bb70,stop,count=2000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
  ptr,count=struct.unpack('<QQ',cpu.mem_read(out+8,16));u_ptr=struct.unpack('<Q',cpu.mem_read(out+32,8))[0];length=struct.unpack('<I',cpu.mem_read(out+48,4))[0];outline=bytes(cpu.mem_read(ptr,count*12));u=bytes(cpu.mem_read(u_ptr,count*4))
  terrain_y=[]
  for p in struct.iter_unpack('<III',outline):
   cpu.mem_write(stack+0x8008,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,stack+0x8008);cpu.reg_write(UC_X86_REG_RCX,data+0x1000);cpu.reg_write(UC_X86_REG_XMM1,p[0]);cpu.reg_write(UC_X86_REG_XMM2,p[2]);cpu.emu_start(0x1409e9640,stop,count=10000);terrain_y.append(cpu.reg_read(UC_X86_REG_XMM0)&0xffffffff)
  lines.append(f'collect {fix} {lo:08x} {hi:08x} {tag} {floor:08x} '+' '.join(f'{fb(v):08x}' for p in points for v in p))
  expected.append(f'{count} RAW {length:08x} {hex_floats(outline)} {hex_floats(u)} '+''.join(f'{y:08x}' for y in terrain_y));rows.append('collect')
 result=subprocess.run([str(ROOT/'reconstruction/target/release/wall-collection-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(result)==len(expected);fail=[]
 for i,(got,want) in enumerate(zip(result,expected)):
  if rows[i]=='collect':parts=got.split();parts[1]='RAW';got=' '.join(parts)
  if got!=want:fail.append({'case':i,'scope':rows[i],'expected':want,'actual':got})
 report={'ok':not fail,'case_count':len(rows),'terrain_cases':1024,'whole_collection_cases':256,'failure_count':len(fail),'failures':fail[:10],'baseline':'B','binary_sha256':b['binary']['sha256'],'scope':'Original game terrain sampler, whole source segment resampling/mapping/Curve3 construction, all positions/U/mapped length and post-mapping terrain samples. Raw Curve2 construction covered separately by existing curve reports.','dependency_bridges':['standard allocator/free/Vec growth (capacity only)','CRT memcpy/roundf; no terrain or curve business substituted'],**candidate_provenance()}
 (ROOT/'evidence/building-collection/B-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':len(rows),'failures':len(fail)}));assert not fail
if __name__=='__main__':main()
