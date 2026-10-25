from pathlib import Path
import struct,json,hashlib
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).resolve().parent;raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();p=pefile.PE(data=raw);base=p.OPTIONAL_HEADER.ImageBase;image=p.get_memory_mapped_image()
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,image);D,S,STOP=0x20000000,0x30000000,0x50000000;c.mem_map(D,0x100000);c.mem_map(S,0x10000);c.mem_map(STOP,0x1000)
q=lambda a,v:c.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff));d=lambda a,v:c.mem_write(a,struct.pack('<I',v&0xffffffff));u=lambda a:struct.unpack('<Q',c.mem_read(a,8))[0];u32=lambda a:struct.unpack('<I',c.mem_read(a,4))[0]
def hook(cpu,pc,sz,_):
 if pc==STOP:cpu.emu_stop();return
 if pc in [0x140c81c70,0x140c81e10]:
  sp=cpu.reg_read(UC_X86_REG_RSP);target=u(sp);cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,target)
c.hook_add(UC_HOOK_CODE,hook)

def run(name,state=0,tick=999,occupant=(7<<32),generation=7,arch=0,querybits=1,querylen=64,entitylen=1,extra=None):
 c.mem_write(D,bytes(0x100000));c.mem_write(S,bytes(0x10000));manager,arc,closure,query,world,metas,archetypes,columns,columnmap,states,ticks,entries=D+0x100,D+0x200,D+0x300,D+0x400,D+0x800,D+0xc00,D+0x1000,D+0x2000,D+0x3000,D+0x4000,D+0x5000,D+0x6000
 q(manager,arc);q(arc,1);q(arc+8,1);q(arc+0x18,32);q(arc+0x20,entries)
 payloads=[occupant]+(extra or []);q(arc+0x28,len(payloads))
 for i,e in enumerate(payloads):q(entries+24*i,e);c.mem_write(entries+24*i+8,struct.pack('<3fI',1,2,3,0xabcdef01))
 q(closure,query);q(closure+8,world);d(closure+0x14,tick);q(query+0x28,D+0x3500);q(query+0x38,querylen);q(D+0x3500,querybits);q(query+0x118,0)
 q(world+8,metas);q(world+0x10,entitylen);q(world+0x1a8,archetypes);d(metas,generation);d(metas+4,arch);d(metas+0xc,0);d(metas+0x10,0)
 q(archetypes+0x18,columns);q(archetypes+0x38,columnmap);q(columnmap,0xffffffffffffffff);q(columns+0x10,states);q(columns+0x28,ticks)
 c.mem_write(states,bytes([0xa5])*80);d(states+0x30,state);d(ticks,123)
 before=bytes(c.mem_read(states,80));sp=S+0x8000-8;q(sp,STOP);c.reg_write(UC_X86_REG_RSP,sp);c.reg_write(UC_X86_REG_RCX,manager);c.reg_write(UC_X86_REG_RDX,closure);c.reg_write(UC_X86_REG_RFLAGS,0x202)
 c.emu_start(0x1422f86e0,STOP,count=50000);assert c.reg_read(UC_X86_REG_RIP)==STOP
 after=bytes(c.mem_read(states,80));expected=(occupant>>32)!=0 and (occupant&0xffffffff)<entitylen and (occupant>>32)==generation and arch!=0xffffffff and arch<querylen and (querybits>>arch)&1 and state==0
 assert u32(states+0x30)==(1 if expected else state),name
 assert u32(ticks)==(tick if expected else 123),name
 changed=[i for i,(x,y) in enumerate(zip(before,after)) if x!=y];assert changed==([0x30] if expected else []),(name,changed)
 assert u(arc+0x28)==0 and u(arc+0x18)==32 and u(arc+0x20)==entries and c.mem_read(arc+0x10,1)==b'\x00'
 return {'name':name,'input_state':state,'tick':tick,'occupant_bits':hex(occupant),'live_generation':generation,'archetype':arch,'query_bits':querybits,'query_length':querylen,'entity_length':entitylen,'native_state':u32(states+0x30),'native_tick':u32(ticks),'queue_drained':True,'queue_allocation_preserved':True,'unrelated_actor_bytes_preserved':True,'extra_occupants':extra}
cases=[]
for state in [0,1,2,3,4,0xffffffff]:cases.append(run('state-'+str(state),state=state))
for tick in [0,1,0xfffffffe,0xffffffff]:cases.append(run('tick-'+str(tick),tick=tick))
for name,kwargs in [('none',{'occupant':0}),('zero-generation',{'occupant':0xffffffff}),('stale-generation',{'generation':8}),('out-of-bounds',{'occupant':(7<<32)|9}),('no-entity-slots',{'entitylen':0}),('dead-archetype',{'arch':0xffffffff}),('not-in-query',{'querybits':0}),('query-range',{'arch':64}),('duplicate-queue',{'extra':[(7<<32),(8<<32),0]})]:cases.append(run(name,**kwargs))
(OUT/'original-recycle-fixtures.json').write_text(json.dumps({'original_sha256':hashlib.sha256(raw).hexdigest(),'function':{'va':'0x1422f86e0','bytes':378,'sha256':hashlib.sha256(image[0x1422f86e0-base:0x1422f86e0-base+378]).hexdigest()},'scope':'Entire original378-byte recycle monomorph executes unchanged, with original queue draining, live entity generation/query membership and component state/tick writes. Explicit fixture ECS storage supplies original owner records; two profiling dependency calls are isolated boundaries, no game callback is stubbed.','cases':cases},indent=2))
print({'native_recycle_cases':len(cases)})
