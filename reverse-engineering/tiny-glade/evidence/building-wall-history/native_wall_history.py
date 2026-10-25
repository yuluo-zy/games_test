"""B native Wall compound constructor and replay authorities, std carriers only."""
import sys,struct
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-authoring'))
import native_authoring as author
from unicorn.x86_const import *
cpu=author.cpu;nf=author.nf;data=author.data;stack=author.stack;stop=author.stop

def snapshot(p):
 raw=bytes(cpu.mem_read(p,112));curve=author.curve_at(p);tag=struct.unpack_from('<I',raw,56)[0];active=20 if tag==0 else 28 if tag==1 else 4
 return {'curve':{k:v for k,v in curve.items()if k!='pointer'},'enclosure':raw[56:56+active].hex(),'scalar_bits':raw[84:100].hex(),'metadata':raw[100:106].hex()}

def compound(owner):
 tag,ptr,count=struct.unpack('<3Q',cpu.mem_read(owner,24));assert tag==7
 groups=[]
 for i in range(count):
  array,n=struct.unpack('<2Q',cpu.mem_read(ptr+i*16,16));group=[]
  for j in range(n):
   arc=nf.u64(array+j*8);raw=bytes(cpu.mem_read(arc,264));op=struct.unpack_from('<Q',raw,16)[0]
   if op in[0x8000000000000000,0x8000000000000001]:
    group.append({'kind':'create'if op==0x8000000000000000 else'delete','source':snapshot(arc+24),'wall':nf.u64(arc+136),'color':struct.unpack_from('<I',raw,152 if op==0x8000000000000000 else 148)[0]})
   else:
    # Niche normal variant begins directly with previous source Curve cap.
    group.append({'kind':'state_change','previous':snapshot(arc+16),'next':snapshot(arc+128),'wall':nf.u64(arc+240),'range':raw[248:260].hex()})
  groups.append(group)
 return groups

def new_wall_owner(shape,tail,wall,color):
 """Full ev_new_wall, including seed crashy_slice and History::add -> RET."""
 author.reset();curve=author.create_geometry(shape);source=data+0x1000
 cpu.mem_write(source,bytes(cpu.mem_read(curve['pointer'],56))+tail)
 history=data+0x2000;cpu.mem_write(history+0x30,struct.pack('<3Q',128,data+0x3000,0));cpu.mem_write(history+0x58,struct.pack('<QQii',1,2,-1,-1))
 cpu.reg_write(UC_X86_REG_RCX,history);cpu.reg_write(UC_X86_REG_RDX,wall);cpu.reg_write(UC_X86_REG_R8,source);cpu.reg_write(UC_X86_REG_R9,color)
 author.active=True
 try:author.call(0x140a83300)
 finally:author.active=False
 assert nf.u64(history+0x40)==1
 return {'actual_return':True,'groups':compound(data+0x3000),'owner_pointer':data+0x3000,'source_pointer':source,'history_pointer':history}

def deleted_wall_owner(shape,tail,wall,color):
 """Actual wall_deleted snapshot clone+next andbuild->RET, no game result bridge."""
 author.reset();curve=author.create_geometry(shape);source=data+0x1000;builder=data+0x2000
 cpu.mem_write(source,bytes(cpu.mem_read(curve['pointer'],56))+tail);cpu.mem_write(builder,struct.pack('<3Q',0,8,0))
 cpu.reg_write(UC_X86_REG_RCX,builder);cpu.reg_write(UC_X86_REG_RDX,wall);cpu.reg_write(UC_X86_REG_R8,source);cpu.reg_write(UC_X86_REG_R9,color)
 author.active=True
 try:
  author.call(0x140a876a0);assert nf.u64(builder+16)==1
  cpu.reg_write(UC_X86_REG_RCX,builder);author.call(0x140a82d50)
 finally:author.active=False
 ptr=cpu.reg_read(UC_X86_REG_RAX);n=cpu.reg_read(UC_X86_REG_RDX);assert n==1
 cpu.mem_write(data+0x3000,struct.pack('<3Q',7,ptr,n))
 return {'actual_return':True,'groups':compound(data+0x3000),'owner_pointer':data+0x3000,'source_pointer':source}

def owner_arcs(owner):
 _,ptr,count=struct.unpack('<3Q',cpu.mem_read(owner,24));groups=[]
 for i in range(count):
  array,n=struct.unpack('<2Q',cpu.mem_read(ptr+i*16,16));groups.append([nf.u64(array+j*8)for j in range(n)])
 return groups

def instant_process(arc,undo,source_pointer,wall,present):
 """Complete instant process restores/mutates/removes actual authority -> RET."""
 ctrl=data+0x18000;row=bytes(cpu.mem_read(source_pointer,112));cpu.mem_write(data,struct.pack('<4Q',ctrl,15,14,int(present)));cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-0x78,struct.pack('<Q',wall)+row);cpu.mem_write(data+0xc0,struct.pack('<Q',100))
 state=data+0x10000;cpu.mem_write(state,struct.pack('<IfQfBB2x',0,.2,arc,0.,int(undo),0))
 writers=[];owners=[]
 for i in range(4):
  writer=data+0x11000+i*0x100;owner=data+0x12000+i*0x100
  cpu.mem_write(writer,struct.pack('<QQQI',owner,0,writer+0x60,0)+struct.pack('<I',0x12345678));cpu.mem_write(owner+0x20,struct.pack('<3Q',32,data+0x20000+i*0x2000,0));cpu.mem_write(owner+0x40,struct.pack('<Q',17));writers.append(writer);owners.append(owner)
 terrain=data+0x14000;terrain_res=data+0x14100;cpu.mem_write(terrain_res,struct.pack('<Q',terrain));cpu.mem_write(terrain+8,struct.pack('<Q',data+0x40000));cpu.mem_write(data+0x40000,bytes(512*512*2));cpu.mem_write(terrain+0x50,struct.pack('<2f',.75,.75));cpu.mem_write(0x14362c308,struct.pack('<Q',data+0x14200))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop))
 for offset,ptr in[(0x28,writers[1]),(0x30,writers[2]),(0x38,terrain_res),(0x40,writers[3])]:cpu.mem_write(sp+offset,struct.pack('<Q',ptr))
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,state);cpu.reg_write(UC_X86_REG_XMM1,nf.bits(.016));cpu.reg_write(UC_X86_REG_R8,data);cpu.reg_write(UC_X86_REG_R9,writers[0]);author.notifications.clear();author.inserted.clear();author.active=True
 try:cpu.emu_start(0x140abfb30,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
 finally:author.active=False
 result={'actual_return':True,'finished':cpu.mem_read(state+21,1)[0],'source_count':nf.u64(data+24),'counter':nf.u64(data+0xc0),'notifications':list(author.notifications),'events':[]}
 if result['source_count']:result['source']=snapshot(ctrl-0x70)
 for i,owner in enumerate(owners):
  n=nf.u64(owner+0x30);result['events'].append({'slot':i,'count':n,'bytes':bytes(cpu.mem_read(nf.u64(owner+0x28),n*[48,24,24,32][i])).hex()})
 return result

def expand_owner(owner,undo,instant):
 """Attempt complete replay_edit entry with real recorded compound ownership."""
 entry=data+0x8000;aux=data+0x8100;replay=data+0x9000;resource=data+0x9100
 cpu.mem_write(aux,struct.pack('<QQ',1,1));cpu.mem_write(aux+0x10,struct.pack('<4Q',data+0x8200,15,15,0));cpu.mem_write(data+0x8200,bytes([255])*16);cpu.mem_write(aux+0x40,struct.pack('<QQ',1,1))
 cpu.mem_write(entry,struct.pack('<5Q',1,owner,1,aux,aux+0x40));cpu.mem_write(replay,struct.pack('<4Q',0,8,0,0));cpu.mem_write(resource,struct.pack('<QQQI',replay,0,resource+0x60,0)+struct.pack('<I',0x12345678))
 cpu.reg_write(UC_X86_REG_RCX,entry);cpu.reg_write(UC_X86_REG_RDX,int(undo));cpu.reg_write(UC_X86_REG_R8,int(instant));cpu.reg_write(UC_X86_REG_R9,resource)
 author.active=True
 try:author.call(0x140af6170)
 finally:author.active=False
 return {'actual_return':True,'replay_header':bytes(cpu.mem_read(replay,64)).hex()}
