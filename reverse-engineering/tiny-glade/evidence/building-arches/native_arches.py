"""Original stone arch profile/walker/CPU owner environment.
Use already verified allocation/CRT and Cargo-only deadlock observer bridges.
The underlying frozen game CPU implementations are never bridged.
"""
import sys,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-structures'))
import native_structures as structures_native
from native_structures import *
from unicorn import UC_HOOK_CODE

def run_to_edge(start,edge,count):
 """Instruction stop at a caller edge, independent of cached until blocks."""
 h=cpu.hook_add(UC_HOOK_CODE,lambda c,a,n,x:c.emu_stop(),begin=edge,end=edge)
 try:cpu.emu_start(start,edge,count=count)
 finally:cpu.hook_del(h)
 assert cpu.reg_read(UC_X86_REG_RIP)==edge,'native caller edge not reached'

def observed_default_brick():
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data)
 cpu.emu_start(0x1408587c0,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'default Brick constructor did not return'
 return bytes(cpu.mem_read(data,96))
if __name__=='__main__':print(observed_default_brick().hex())

def create_brick_writer(pages=64):
 view=structures_native.create_sparse_writer(pages);view['records']=alloc(pages*32*96);cpu.mem_write(view['writer']+0x28,struct.pack('<Q',view['records']));return view

def brick_records(view):
 cap,p,n=struct.unpack('<3Q',cpu.mem_read(view['owned'],24));assert n<=cap
 pages=struct.unpack('<'+'I'*n,cpu.mem_read(p,n*4));out=[]
 for page in pages:
  count=bytes(cpu.mem_read(view['counts']+page,1))[0];assert count<=32
  out.extend(bytes(cpu.mem_read(view['records']+(page*32+i)*96,96))for i in range(count))
 return out

def observed_arch_callback(start,end,us,wall_id,source_length,ordinal=1,emitted_index=0):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));view=create_brick_writer();cpu.mem_write(data+0x100,struct.pack('<3f',*start));cpu.mem_write(data+0x110,struct.pack('<3f',*end));cpu.mem_write(data+0x200,struct.pack('<Q',ordinal));cpu.mem_write(data+0x210,struct.pack('<Q',emitted_index));cpu.mem_write(data+0x220,struct.pack('<Q',wall_id));public=data+0x400;cpu.mem_write(public+0x28c,struct.pack('<f',source_length));growth=data+0x800;cpu.mem_write(data+0x900,struct.pack('<6Q',data+0x200,data+0x210,data+0x220,public,view['writer'],growth))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<f',us[1]));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,data+0x900);cpu.reg_write(UC_X86_REG_RDX,data+0x100);cpu.reg_write(UC_X86_REG_R8,data+0x110);cpu.reg_write(UC_X86_REG_XMM3,bits(us[0]));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 cpu.emu_start(0x141231af0,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'arch callback did not return';assert bytes(cpu.mem_read(growth,1))[0]==0
 return brick_records(view)

def prepare_profiles_current(views,arch,shape,wall_float,flag_294,prepared_count):
 """Whole original1407645f0, with true caller group-override selection block.
 Input segment views are actual original14122ff90 owned records.
 """
 public=data+0x1000;ap=alloc(len(arch)*4);cpu.mem_write(ap,struct.pack('<'+'f'*len(arch),*arch));cpu.mem_write(public+0xe0,struct.pack('<QQ',ap,len(arch)));cpu.mem_write(public+0x258,struct.pack('<I',shape));space=data+0x1600;cpu.mem_write(space,struct.pack('<If',int(shape!=2),1.));override=data+0x1800;cpu.mem_write(override,bytes(12))
 if flag_294 and views:
  assert all(v==views[0]+104*i for i,v in enumerate(views))
  cpu.reg_write(UC_X86_REG_RDI,views[0]);cpu.reg_write(UC_X86_REG_RSI,len(views));cpu.reg_write(UC_X86_REG_R15,0);cpu.reg_write(UC_X86_REG_RBP,stack+0x180000);cpu.reg_write(UC_X86_REG_RSP,stack+0x170000);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  run_to_edge(0x140760628,0x1407606f5,10000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x1407606f5
  cpu.mem_write(override,struct.pack('<III',1,cpu.reg_read(UC_X86_REG_XMM7)&0xffffffff,cpu.reg_read(UC_X86_REG_XMM13)&0xffffffff))
 trim=[];segments=[]
 for v in views:
  sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<QQ',override,public));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,v);cpu.reg_write(UC_X86_REG_XMM1,bits(wall_float));cpu.reg_write(UC_X86_REG_R8,prepared_count);cpu.reg_write(UC_X86_REG_R9,space);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  cpu.emu_start(0x1407645f0,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'stone arch profile did not return'
  trim.append([fl(cpu.reg_read(UC_X86_REG_XMM0)),fl(cpu.reg_read(UC_X86_REG_XMM1))]);fields=[]
  for offset,width,fmt in [(0,4,'f'),(24,12,'3f'),(48,4,'f')]:
   cap,p,n=struct.unpack('<3Q',cpu.mem_read(v+offset,24));assert n<=cap
   fields.append([struct.unpack('<'+fmt,cpu.mem_read(p+width*i,width)) for i in range(n)])
  segments.append(dict(heights=[p[0]for p in fields[0]],positions=[list(p)for p in fields[1]],us=[p[0]for p in fields[2]],tail=bytes(cpu.mem_read(v+72,28)).hex()))
 return trim,segments

def walk_arch_profiles(segments,wall_id,source_length,first_ordinal=1):
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));view=create_brick_writer(256);public=data+0x1000;cpu.mem_write(public+0x28c,struct.pack('<f',source_length));idptr=data+0x200;cpu.mem_write(idptr,struct.pack('<Q',wall_id));ordinalptr=data+0x210;indexptr=data+0x220;growth=data+0x800;env=data+0x900;cpu.mem_write(env,struct.pack('<6Q',ordinalptr,indexptr,idptr,public,view['writer'],growth));options=data+0xa00;cpu.mem_write(options,struct.pack('<Ifff',1,1.,.56,.1))
 for ordinal,s in enumerate(segments,first_ordinal):
  n=len(s['heights']);hp=alloc(n*4);gp=alloc(n*12);up=alloc(n*4);cpu.mem_write(hp,struct.pack('<'+'f'*n,*s['heights']));cpu.mem_write(gp,b''.join(struct.pack('<3f',*p)for p in s['positions']));cpu.mem_write(up,struct.pack('<'+'f'*n,*s['us']));cpu.mem_write(ordinalptr,struct.pack('<Q',ordinal));cpu.mem_write(indexptr,bytes(8))
  sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,options);cpu.reg_write(UC_X86_REG_RDX,hp);cpu.reg_write(UC_X86_REG_R8,n);cpu.reg_write(UC_X86_REG_R9,gp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
  for offset,v in {0x28:n,0x30:up,0x38:n,0x40:env}.items():cpu.mem_write(sp+offset,struct.pack('<Q',v))
  cpu.emu_start(0x141232bb0,stop,count=10000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'arch walker did not return'
 assert bytes(cpu.mem_read(growth,1))[0]==0,'arch writer pool exhausted'
 return brick_records(view)

def accumulate_regular_arch_profiles(segments,trims,arch,third):
 """Original caller full third iteration and closed third+arch seam copies."""
 allocation[0]=heap;cpu.mem_write(data,bytes(0x100000));cpu.mem_write(stack,bytes(0x200000));n=len(segments);sptr=alloc(n*104)
 for i,s in enumerate(segments):
  v=sptr+i*104
  for offset,rows,width in [(0,s['heights'],4),(24,s['positions'],12),(48,s['us'],4)]:
   payload=struct.pack('<'+'f'*len(rows),*rows)if width==4 else b''.join(struct.pack('<3f',*p)for p in rows);p=alloc(len(payload));cpu.mem_write(p,payload);cpu.mem_write(v+offset,struct.pack('<3Q',len(rows),p,len(rows)))
  cpu.mem_write(v+72,bytes.fromhex(s['tail'])+bytes(4))
 tp=alloc(len(trims)*8);cpu.mem_write(tp,b''.join(struct.pack('<2f',*p)for p in trims));ap=alloc(len(arch)*4);cpu.mem_write(ap,struct.pack('<'+'f'*len(arch),*arch));thp=alloc(len(third)*4);cpu.mem_write(thp,struct.pack('<'+'f'*len(third),*third));pubend=data+0x12a0
 cpu.mem_write(pubend-0x1c0,struct.pack('<QQ',ap,len(arch)));cpu.mem_write(pubend-0x1a8,struct.pack('<QQ',thp,len(third)))
 bp=stack+0x180000;cpu.mem_write(bp+0x1a8,struct.pack('<Q',sptr+n*104));cpu.mem_write(bp+0x150,struct.pack('<QQ',tp,n));cpu.mem_write(bp+0x128,struct.pack('<Q',sptr));cpu.mem_write(bp+0x2e8,struct.pack('<Q',pubend));cpu.reg_write(UC_X86_REG_RBP,bp);cpu.reg_write(UC_X86_REG_RSP,stack+0x170000);cpu.reg_write(UC_X86_REG_R14,sptr);cpu.reg_write(UC_X86_REG_R13,sptr);cpu.reg_write(UC_X86_REG_R12,pubend);cpu.reg_write(UC_X86_REG_RDI,0);cpu.reg_write(UC_X86_REG_XMM14,0);cpu.reg_write(UC_X86_REG_XMM6,bits(1.));cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 run_to_edge(0x140761288,0x140761770,10000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x140761770,'arch third owner did not finish'
 run_to_edge(0x140762953,0x1407629b1,1000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x1407629b1,'arch closed copies did not finish'
 return list(struct.unpack('<'+'f'*len(arch),cpu.mem_read(ap,len(arch)*4))),list(struct.unpack('<'+'f'*len(third),cpu.mem_read(thp,len(third)*4)))

def observe_gate_event_send(v,wall_id,ordinal,event_count):
 """Actual caller Events<ConstructGateCmd>::send1407607ea..b34.
 Game120B payload + dependency EventId8; no fabricated buffer/event rule.
 Current real materialized/profile-mutated segment view, legal ample Vec cap.
 """
 bp=stack+0x180000;resource=data+0x2000;ptr=alloc(16*128);cpu.mem_write(resource+0x20,struct.pack('<QQQ',16,ptr,0));cpu.mem_write(resource+0x40,struct.pack('<Q',event_count));cpu.mem_write(bp+0x180,struct.pack('<Q',resource));cpu.mem_write(bp+0x50,struct.pack('<Q',data+0x2800));cpu.mem_write(bp+0x58,struct.pack('<Q',resource+0x20));cpu.mem_write(bp+0x178,struct.pack('<Q',ordinal));cpu.mem_write(bp+0x2b0,struct.pack('<Q',wall_id));cpu.reg_write(UC_X86_REG_RBP,bp);cpu.reg_write(UC_X86_REG_RSP,stack+0x170000);cpu.reg_write(UC_X86_REG_RSI,v);cpu.reg_write(UC_X86_REG_R13,1);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
 run_to_edge(0x1407607ea,0x140760b34,10000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x140760b34,'deferred clone did not reach edge';assert u64(resource+0x30)==1 and u64(resource+0x40)==event_count+1
 raw=bytes(cpu.mem_read(ptr,128));payload=[]
 for offset,width in [(0,4),(24,12),(48,4)]:
  cap,p,n=struct.unpack_from('<3Q',raw,offset);assert n<=cap;payload.append(struct.pack('<Q',n)+bytes(cpu.mem_read(p,n*width)))
 return b''.join(payload)+raw[72:100]+raw[104:128]

def process_platform_original(points,values,shape,special,maximum,roof_word=None):
 """Same full original upstream body, extending only emulator instruction budget.
 Keep accepted upstream fixture files untouched. A count-limit AssertionError
 resumes exact current RIP/registers; no game routine is intercepted or replaced.
 """
 import native_platform_segmentation as platform_native
 original_fixture=platform_native.produce
 original_call=platform_native.call
 def call_with_larger_budget(*args,**kwargs):
  try:return original_call(*args,**kwargs)
  except AssertionError:
   rip=cpu.reg_read(UC_X86_REG_RIP);assert rip!=stop
   cpu.emu_start(rip,stop,count=30000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'platform owner did not actually return within extended budget'
 def invoke_with_larger_budget(*args,**kwargs):
  try:return original_fixture(*args,**kwargs)
  except AssertionError:
   rip=cpu.reg_read(UC_X86_REG_RIP);assert rip!=stop
   cpu.emu_start(rip,stop,count=30000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop,'upstream segmentation did not actually return within extended budget'
   cap,p,n=struct.unpack('<3Q',cpu.mem_read(data+0x1000,24));assert n<=cap
   result=[]
   for i in range(n):
    rawmask=bytes(cpu.mem_read(p+i*136,136));fc,fp,fn,cc,cp,cn=struct.unpack_from('<6Q',rawmask);assert fn<=fc and cn<=cc
    result.append(dict(raw=rawmask.hex(),flags=bytes(cpu.mem_read(fp,fn)).hex(),coords=bytes(cpu.mem_read(cp,cn*8)).hex(),range=rawmask[120:132].hex(),length=rawmask[132:136].hex(),lazy_headers=[rawmask[j:j+24].hex()for j in [48,72,96]]))
   return result
 platform_native.produce=invoke_with_larger_budget
 platform_native.call=call_with_larger_budget
 try:return platform_native.process(points,values,shape,special,maximum,roof_word)
 finally:
  platform_native.produce=original_fixture
  platform_native.call=original_call
