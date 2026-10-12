"""Original WallHoleStorage create/add/hash/finalize/reopen instruction chain.
Game hash schema, geometry, wrap rules/cache fingerprints run original code.
Vec/allocator/CRT boundaries supply owned storage; std sort executes original.
"""
import hashlib,json,random,struct,subprocess,platform
from pathlib import Path
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
from verification_provenance import ROOT,candidate_provenance
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
def main():
 b=json.loads((ROOT/'tracking/baselines.json').read_text())['baselines']['B'];raw=Path(b['binary']['path']).read_bytes();assert hashlib.sha256(raw).hexdigest()==b['binary']['sha256'];pe=b['pe'];base=pe['image_base'];cpu=Uc(UC_ARCH_X86,UC_MODE_64);end=max(s['rva']+max(s['raw_size'],s['virtual_size']) for s in pe['sections']);cpu.mem_map(base,(end+4095)&~4095)
 for s in pe['sections']:
  if s['raw_size']:cpu.mem_write(base+s['rva'],raw[s['raw_pointer']:s['raw_pointer']+s['raw_size']])
 data,stack,heap,stop=0x20000000,0x30000000,0x40000000,0x50000000;cpu.mem_map(data,0x10000);cpu.mem_map(stack,0x100000);cpu.mem_map(heap,0x400000);cpu.mem_map(stop,0x1000);allocation=[heap];error=[None]
 def allocate(n):
  p=allocation[0];allocation[0]+=max(0x1000,(n+15)&~15);assert allocation[0]<heap+0x400000;return p
 def ret():
  sp=cpu.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',cpu.mem_read(sp,8))[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,ra)
 def grow(vec,needed):
  cap,p,n=struct.unpack('<QQQ',cpu.mem_read(vec,24));new_cap=max(4,cap*2,needed);out=allocate(new_cap*20)
  if n:cpu.mem_write(out,bytes(cpu.mem_read(p,n*20)))
  cpu.mem_write(vec,struct.pack('<QQQ',new_cap,out,n))
 def bridge(uc,address,size,user):
  if address==0x140613c10:cpu.reg_write(UC_X86_REG_RAX,allocate(cpu.reg_read(UC_X86_REG_RCX)));ret()
  elif address in [0x140613c20,0x14289aa40]:ret()
  elif address==0x140563600:
   p=cpu.reg_read(UC_X86_REG_RCX);n=struct.unpack('<Q',cpu.mem_read(p+16,8))[0];grow(p,n+1);ret()
  elif address==0x1428a9a60:grow(cpu.reg_read(UC_X86_REG_RCX),cpu.reg_read(UC_X86_REG_R8));ret()
  elif address in [0x142923c00,0x142923c20]:
   dst,src,n=(cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);cpu.mem_write(dst,bytes(cpu.mem_read(src,n)));cpu.reg_write(UC_X86_REG_RAX,dst);ret()
  elif address==0x142923bf0:
   dst,value,n=(cpu.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]);cpu.mem_write(dst,bytes([value&255])*n);cpu.reg_write(UC_X86_REG_RAX,dst);ret()
  elif address in [0x1428d9430,0x1428d9390,0x1428d9310]:
   error[0]='NonOrderableBounds' if address==0x1428d9310 else 'AlreadyFinalized' if address==0x1428d9390 else 'InvalidDimensions';uc.emu_stop()
 cpu.hook_add(UC_HOOK_CODE,bridge)
 def call(entry,rcx=0,rdx=0,r8=0,xmm1=0):
  error[0]=None;sp=stack+0x80008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,rcx);cpu.reg_write(UC_X86_REG_RDX,rdx);cpu.reg_write(UC_X86_REG_R8,r8);cpu.reg_write(UC_X86_REG_XMM1,xmm1);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.emu_start(entry,stop,count=2000000);assert error[0] or cpu.reg_read(UC_X86_REG_RIP)==stop;return error[0]
 def new():
  allocation[0]=heap;cpu.mem_write(stack,bytes(0x100000));cpu.mem_write(data,bytes(0x10000));call(0x140f86de0,rcx=data);cpu.mem_write(data+8,struct.pack('<QQQ',64,data+0x2000,0));cpu.mem_write(data+0x20,struct.pack('<QQQ',128,data+0x8000,0))
 def add(storage,hole):cpu.mem_write(data+0x1000,hole);assert not call(0x140f87860,rcx=storage+8,rdx=data+0x1000,r8=hole[16])
 def finalize(storage,length,closed):
  raw_fp=bytes(cpu.mem_read(storage+0x40,16));previous=bytes(cpu.mem_read(storage+0x50,16));e=call(0x140f87300,rcx=storage,r8=closed,xmm1=length)
  if e:return 'error '+e
  assert cpu.reg_read(UC_X86_REG_RAX)&255==int(raw_fp!=previous)
  assert struct.unpack('<Q',cpu.mem_read(storage,8))[0]==1;p,n=struct.unpack('<QQ',cpu.mem_read(storage+0x10,16));actual_length=struct.unpack('<I',cpu.mem_read(storage+0x20,4))[0];actual_closed=cpu.mem_read(storage+0x24,1)[0];fp_sum,fp_xor=struct.unpack('<QQ',cpu.mem_read(storage+0x40,16));assert bytes(cpu.mem_read(storage+0x50,16))==previous
  records=[bytes(cpu.mem_read(p+i*20,18)).hex() for i in range(n)]
  prev_sum,prev_xor=struct.unpack('<QQ',previous)
  return f'{int(raw_fp!=previous)} {actual_length:08x} {actual_closed} {fp_sum:016x} {fp_xor:016x} {prev_sum:016x} {prev_xor:016x} '+','.join(records)
 rng=random.Random(20261013);lines=[];want=[];groups=[]
 for i in range(256):
  edges=[0,0x80000000,0x7f800000,0xff800000,0x7fc12345,0xffc01234];vals=[rng.choice(edges) if i<128 else bits(rng.uniform(-30,30)) for _ in range(4)];hole=struct.pack('<IIIIB',*vals,i%7);cpu.mem_write(data+0x1000,hole);call(0x14145b910,rcx=data+0x1000);want.append(f'{cpu.reg_read(UC_X86_REG_RAX):016x}');lines.append('hash '+hole.hex());groups.append('hash_schema')
 def holes(n,case):
  rows=[]
  for j in range(n):
   values=[rng.uniform(-3,13),rng.uniform(-1,8),rng.uniform(.01,7),rng.uniform(.01,6)]
   if case%9==0:values[0]=j%3*2.0;values[2]=2.0
   rows.append(struct.pack('<ffffB',*values,j%5))
  return rows
 for i in range(256):
  n=[0,1,2,3,7,10,14,20,24][i%9];closed=i%2;length=bits([1.,4.,10.,20.][i%4]);hs=holes(n,i);new()
  for h in hs:add(data,h)
  want.append(finalize(data,length,closed).rstrip());lines.append(f'final {length:08x} {closed} '+' '.join(h.hex() for h in hs));groups.append('finalize_whole')
 for i in range(128):
  n=[0,1,2,5,11][i%5];hs=holes(n,i);added=holes(i%4,i+1);length=bits(10.);closed=i%2;new()
  for h in hs:add(data,h)
  first=finalize(data,length,closed).rstrip();assert not first.startswith('error');assert not call(0x140f86f50,rcx=data+0x400,rdx=data)
  for h in added:add(data+0x400,h)
  second=finalize(data+0x400,bits(15.),1-closed).rstrip();want.append(first+' | '+second);lines.append(f'cycle {length:08x} {closed} {bits(15.):08x} {1-closed} {n} '+' '.join(h.hex() for h in hs+added));groups.append('clone_reopen_add_finalize')
 for i in range(128):
  n=[0,1,2,5,11][i%5];hs=holes(n,i);next_holes=list(reversed(hs)) if i%3==0 else hs if i%3==1 else holes(n,i+1);new()
  for h in hs:add(data,h)
  first=finalize(data,bits(10.),i%2).rstrip();assert not first.startswith('error')
  previous_fp=bytes(cpu.mem_read(data+0x40,16));p,count=struct.unpack('<QQ',cpu.mem_read(data+0x10,16));previous_geometry=b''.join(bytes(cpu.mem_read(p+j*20,18)) for j in range(count))
  cpu.mem_write(data+0x6040,struct.pack('<QQQ',128,data+0xc000,0));sp=stack+0x70000;cpu.mem_write(sp,bytes(0x100));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RBX,data+0x60);cpu.reg_write(UC_X86_REG_RDI,data+0x6060);cpu.reg_write(UC_X86_REG_XMM7,0);cpu.emu_start(0x140f86aa7,0x140f86c8f,count=10000);assert cpu.reg_read(UC_X86_REG_RIP)==0x140f86c8f
  assert struct.unpack('<Q',cpu.mem_read(data,8))[0]==0 and bytes(cpu.mem_read(data+0x40,16))==bytes(16) and bytes(cpu.mem_read(data+0x50,16))==previous_fp
  assert struct.unpack('<Q',cpu.mem_read(data+0x18,8))[0]==0
  prev_count=struct.unpack('<Q',cpu.mem_read(data+0x6050,8))[0];assert prev_count==count and b''.join(bytes(cpu.mem_read(data+0xc000+j*20,18)) for j in range(count))==previous_geometry
  for h in next_holes:add(data,h)
  second=finalize(data,bits(15.),1-i%2).rstrip();want.append(first+' | '+second);lines.append(f'frame {bits(10.):08x} {i%2} {bits(15.):08x} {1-i%2} {n} '+' '.join(h.hex() for h in hs+next_holes));groups.append('frame_clear_recollect_finalize')
 got=subprocess.run([str(ROOT/'reconstruction/target/release/wall-holes-probe')],input='\n'.join(lines)+'\n',capture_output=True,text=True,check=True).stdout.splitlines();assert len(got)==len(want);fail=[{'case':i,'group':groups[i],'input':lines[i],'expected':e,'actual':a.rstrip()} for i,(a,e) in enumerate(zip(got,want)) if a.rstrip()!=e]
 report={'ok':not fail,'case_count':768,'hash_cases':256,'whole_finalize_cases':256,'whole_reopen_cycle_cases':128,'frame_transition_recollection_cases':128,'failure_count':len(fail),'failures':fail[:10],'candidate_binary_sha256':hashlib.sha256((ROOT/'reconstruction/target/release/wall-holes-probe').read_bytes()).hexdigest(),'candidate_build_mode':'cargo release --locked','candidate_runtime':platform.system()+'/'+platform.machine(),'baseline':'B','binary_sha256':b['binary']['sha256'],'scope':'Whole new/add/hash/finalize/clone-and-reopen/add/finalize lifecycle plus original resolved frame-row transition/recollection/finalize, both fingerprints, cached length/closed flag, Aabb conversion/wrap duplicates and ordered type/origin geometry. Record padding excluded, not fabricated.','dependency_boundary':['Standard Vec/allocator capacity/reallocation, CRT memory and Windows stack probe','Original std stable sort executes; Rust uses std stable sort/DefaultHasher, their implementations not reconstructed.','Outer frame-row starts after resolved HashMap lookup; all actual game state transition executes.'],'limitations':['Original global WallHoles/PrevWallHoles map-retain/lookup and actual ECS producer/query integration tracked separately.'],**candidate_provenance()};(ROOT/'evidence/building-holes/storage-B-verification.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps({'ok':not fail,'cases':768,'failures':len(fail)}));assert not fail
if __name__=='__main__':main()
