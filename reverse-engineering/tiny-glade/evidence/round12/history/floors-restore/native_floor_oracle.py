"""Execute complete original1888B floor game function; bridge dependency APIs only.
HashMap/library container, Bevy entity/command reservation and CRT are boundaries.
Public lookup, drain order, gating, math, center/transform/payload remain original.
"""
from pathlib import Path
import json,struct,hashlib,ctypes as C,math
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE,UC_HOOK_MEM_UNMAPPED
from unicorn.x86_const import *
ROOT=Path('D:/game');OUT=Path(__file__).resolve().parent;image=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';raw=image.read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);BASE=pe.OPTIONAL_HEADER.ImageBase;SIZE=pe.OPTIONAL_HEADER.SizeOfImage
cpu=Uc(UC_ARCH_X86,UC_MODE_64);D=0x180000000;S=0x190000000;STOP=0x1a0000000;cpu.mem_map(D,0x40000);cpu.mem_map(S,0x20000);cpu.mem_map(STOP,0x1000);cpu.mem_write(STOP,b'\xc3')
mapped=set()
def demand(c,access,address,size,value,user):
 page=address&~0xfff
 if not BASE<=page<BASE+SIZE:return False
 if page not in mapped:
  c.mem_map(page,0x1000);data=pe.get_data(page-BASE,0x1000);c.mem_write(page,data+bytes(0x1000-len(data)));mapped.add(page)
 return True
cpu.hook_add(UC_HOOK_MEM_UNMAPPED,demand)
def put(a,fmt,*values):cpu.mem_write(a,struct.pack(fmt,*values))
def get(a,fmt):return struct.unpack(fmt,bytes(cpu.mem_read(a,struct.calcsize(fmt))))
def ret(value=None):
 if value is not None:cpu.reg_write(UC_X86_REG_RAX,value)
 sp=cpu.reg_read(UC_X86_REG_RSP);target=get(sp,'<Q')[0];cpu.reg_write(UC_X86_REG_RSP,sp+8);cpu.reg_write(UC_X86_REG_RIP,target)
crt=C.CDLL('ucrtbase');crt.log2f.argtypes=[C.c_float];crt.log2f.restype=C.c_float
trace=[];storage={};live=set();next_entity=1;panicked=False
QUEUE=D+0x5000;QDATA=D+0x6000;ENTITIES=D+0x4000
def bridge(c,pc,size,user):
 global next_entity,panicked
 rcx,rdx,r8,r9=[c.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
 if pc==0x1405b0d70:ret(0) # std BuildHasher::hash_one
 elif pc==0x140b76f90: # hashbrown remove_entry output Option<(WallId,Entity)>
  wall=get(r9,'<Q')[0];entity=storage.pop(wall,None);trace.append(['remove',wall,entity]);put(rcx,'<QQ',wall,entity or 0);ret(rcx)
 elif pc==0x140ceafa0:ret(int(rdx in live)) # Bevy Entities::contains
 elif pc==0x140d14770:trace.append(['despawn',get(rcx+0x20,'<Q')[0]]);ret()
 elif pc==0x140d146a0: # Bevy Commands::spawn_empty, reserve and retain queue
  entity=(1<<32)|next_entity;next_entity+=1;live.add(entity);put(rcx,'<QQQQQ',ENTITIES,QUEUE,0,0,entity);trace.append(['reserve',entity]);ret(rcx)
 elif pc==0x140eec970:storage[rdx]=r8;trace.append(['insert',rdx,r8]);ret(0)
 elif pc==0x142923f50:
  v=struct.unpack('<f',c.reg_read(UC_X86_REG_XMM0).to_bytes(16,'little')[:4])[0];result=crt.log2f(v);bits=struct.unpack('<I',struct.pack('<f',result))[0];c.reg_write(UC_X86_REG_XMM0,bits);ret()
 elif pc==0x1428d9310:panicked=True;trace.append(['panic_unwrap']);c.emu_stop()
 elif pc in [0x1428a7de0,0x1428a9a60]:raise RuntimeError('unexpected Vec grow: fixture reserves full original queue capacity')
cpu.hook_add(UC_HOOK_CODE,bridge)
def reader(where,ids):
 cursor=where+0x80;events=where+0x100;buffer=where+0x200;put(where,'<QQ',cursor,events);put(cursor,'<Q',0)
 put(events,'<9Q',len(ids),buffer,len(ids),0,0,8,0,len(ids),len(ids))
 for i,wall in enumerate(ids):put(buffer+i*16,'<QQ',i,wall)
 return cursor
def shape(tag,radius=2.):
 if tag==0:return struct.pack('<I6f',0,2.,-3.,radius,-.75,12.,13.)
 return struct.pack('<I6f',tag,.6,.8,4.,-5.,6.,7.)
def run(label,tag=0,height=1.,radius=2.,deleted=(),changed=(7,),old=None,exists=True,wall_present=True):
 global storage,trace,live,next_entity,panicked
 cpu.mem_write(D,bytes(0x40000));cpu.mem_write(S,bytes(0x20000));trace=[];storage=dict(old or {});live=set(storage.values()) if exists else set();next_entity=1;panicked=False
 resource=D+0x1000;res=D+0x80;ctrl=D+0x3000;state=ctrl-0x2a0
 put(res,'<Q',resource);put(resource,'<4Q',ctrl,15,0,int(wall_present));cpu.mem_write(ctrl,b'\xff'*32)
 if wall_present:cpu.mem_write(ctrl,b'\x00');put(ctrl-0x2a8,'<Q',7);cpu.mem_write(state+0x258,shape(tag,radius));put(state+0x280,'<f',height)
 delcursor=reader(D+0x10000,deleted);changedcursor=reader(D+0x11000,changed);put(QUEUE,'<QQQ',0x10000,QDATA,0)
 put(D+0x90,'<QQQQ',ENTITIES,QUEUE,0,0)
 sp=S+0x1f008;put(sp,'<Q',STOP);put(sp+0x28,'<Q',D+0x90)
 for reg,val in [(UC_X86_REG_RCX,res),(UC_X86_REG_RDX,D+0x11000),(UC_X86_REG_R8,D+0x10000),(UC_X86_REG_R9,D+0x2000),(UC_X86_REG_RSP,sp)]:cpu.reg_write(reg,val)
 cpu.emu_start(0x1407661e0,STOP,count=100000)
 if not panicked:assert cpu.reg_read(UC_X86_REG_RIP)==STOP,hex(cpu.reg_read(UC_X86_REG_RIP))
 queue_len=get(QUEUE+16,'<Q')[0];payload=bytes(cpu.mem_read(QDATA,queue_len))
 assert queue_len%0xb8==0
 return {'label':label,'input':{'tag':tag,'height_bits':struct.unpack('<I',struct.pack('<f',height))[0],'radius_bits':struct.unpack('<I',struct.pack('<f',radius))[0],'deleted':list(deleted),'changed':list(changed),'old':old,'exists':exists,'wall_present':wall_present},'trace':list(trace),'storage':dict(storage),'deleted_cursor':get(delcursor,'<Q')[0],'changed_cursor':get(changedcursor,'<Q')[0],'panic':panicked,'queue_hex':payload.hex(),'queue_entries':[{'transform_words':list(struct.unpack_from('<12I',payload,i+8)),'descriptor_words':list(struct.unpack_from('<6I',payload,i+0x38)),'floor_words':list(struct.unpack_from('<10I',payload,i+0x58))} for i in range(0,queue_len,0xb8)]}
cases=[]
for label,kw in [('circle',{}),('rectangle',{'tag':1}),('freehand',{'tag':2}),('zeroheight',{'height':0.}),('negativeheight',{'height':-1.}),('nanheight',{'height':math.nan}),('infheight',{'height':math.inf}),('deleted_then_changed',{'deleted':(7,),'old':{7:0x1000000aa}}),('stale_deleted',{'deleted':(7,),'changed':(),'old':{7:0x1000000aa},'exists':False}),('duplicate_changes',{'changed':(7,7)}),('missing_changed',{'wall_present':False}),('unmapped_deleted',{'deleted':(9,),'changed':()})]:cases.append(run(label,**kw))
for radius in [-math.inf,-2.,-0.,0.,.99999994,1.,1.0000001,2.,10.,1000.,math.inf,math.nan]:cases.append(run('radius_boundary',radius=radius))
out={'original_sha256':hashlib.sha256(raw).hexdigest(),'function_sha256':hashlib.sha256(pe.get_data(0x7661e0,1888)).hexdigest(),'scope':'complete original floor1888B executed; dependencies HashMap/Bevy entity/command reserve/CRT bridged; actual game gate/math/order/complete queue payload executed','cases':cases}
(OUT/'native-floor-fixtures.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps({'cases':len(cases),'first':cases[0],'scope':out['scope']},indent=2))
