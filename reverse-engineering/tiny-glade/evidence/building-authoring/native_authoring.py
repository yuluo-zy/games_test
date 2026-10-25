"""Original B authoring geometry, creation row/ID and unconditional source updates.
HashMap/HashSet storage and hashing bridges have std-only semantics. Every
private shape/curve and game field/ID instruction executes; no game result hook.
"""
import sys,struct,ctypes
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'evidence/building-constructor'))
import native_fixture as nf
from unicorn import UC_HOOK_CODE
from unicorn.x86_const import *
cpu=nf.cpu;data=nf.data;stack=nf.stack;stop=nf.stop
active=False;inserted=[];notifications=[]
crt={0x142923e80:('cosf',1),0x142923f10:('fmodf',2)}
for name,count in crt.values():
 fn=getattr(nf.lib,name);fn.argtypes=[ctypes.c_float]*count;fn.restype=ctypes.c_float

def hook(c,pc,size,user):
 if pc in crt:
  name,count=crt[pc];args=[nf.fl(c.reg_read(r))for r in [UC_X86_REG_XMM0,UC_X86_REG_XMM1][:count]]
  c.reg_write(UC_X86_REG_XMM0,nf.bits(getattr(nf.lib,name)(*args)));nf.ret()
 elif active and pc==0x1405b0d70:nf.ret(0) # std RandomState only.
 elif active and pc==0x141529bb0:
  result,owner,key,source=[c.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]]
  payload=bytes(c.mem_read(source,112));inserted.append((key,payload));control=data+0x18000
  c.mem_write(owner,struct.pack('<4Q',control,15,14,1));c.mem_write(control,bytes([0])+bytes([255])*15)
  c.mem_write(control-0x78,struct.pack('<Q',key)+payload)
  # std Option<old WallState> None; no replaced state needs dropping here.
  c.mem_write(result,struct.pack('<Q',0x8000000000000000));nf.ret(result)
 elif active and pc==0x1402ca2f0:
  notifications.append((c.reg_read(UC_X86_REG_RCX),c.reg_read(UC_X86_REG_RDX)));nf.ret(1)
 elif active and pc==0x140b76b50:
  result,owner=[c.reg_read(r)for r in [UC_X86_REG_RCX,UC_X86_REG_RDX]];control=nf.u64(owner)
  c.mem_write(result,bytes(c.mem_read(control-0x78,120)));c.mem_write(owner+0x18,bytes(8));c.mem_write(control,bytes([255])*16);nf.ret(result)
cpu.hook_add(UC_HOOK_CODE,hook)

def reset():
 inserted.clear();notifications.clear();nf.allocation[0]=nf.heap
 cpu.mem_write(data,bytes(0x40000));cpu.mem_write(stack,bytes(0x200000))
 for r in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:cpu.reg_write(r,0)
 cpu.reg_write(UC_X86_REG_MXCSR,0x1f80);cpu.reg_write(UC_X86_REG_RFLAGS,0x202)
 cpu.mem_write(0x14362c3a8,struct.pack('<Q',data+0x30000)) # logging disabled

def call(entry):
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,sp)
 cpu.emu_start(entry,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop

def curve_at(p):
 n=nf.u64(p+16);points=nf.u64(p+8);ups=nf.u64(p+32)
 assert 2<=n<=65536
 return {'points':bytes(cpu.mem_read(points,n*8)).hex(),'us':bytes(cpu.mem_read(ups,n*4)).hex(),'length':struct.unpack('<I',cpu.mem_read(p+48,4))[0],'n':n,'pointer':p}

def create_geometry(shape):
 """Original creator circle prefix; original whole rectangle as_curve2."""
 tag=struct.unpack_from('<I',shape)[0];frame=stack+0x100080
 if tag==0:
  cpu.mem_write(frame-0x40,shape[4:20]);cpu.reg_write(UC_X86_REG_RBP,frame);cpu.reg_write(UC_X86_REG_RSP,frame-0x80)
  cpu.reg_write(UC_X86_REG_RDX,data+0x10000);cpu.reg_write(UC_X86_REG_XMM6,nf.bits(.5))
  cpu.emu_start(0x14097e9c4,0x14097ea3c,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==0x14097ea3c
  p=frame-0x30
 elif tag==1:
  cpu.mem_write(data+0x1000,shape[4:28]);cpu.reg_write(UC_X86_REG_RCX,data+0x1100);cpu.reg_write(UC_X86_REG_RDX,data+0x1000)
  call(0x140c98cf0);p=data+0x1100
 else:raise ValueError('shape')
 result=curve_at(p);cpu.mem_write(data+0x1200,bytes(cpu.mem_read(p,56)));result['pointer']=data+0x1200
 return result

def mutate(present,wall,source,kind,payload):
 global active
 reset();control=data+0x18000;cpu.mem_write(control,bytes([0])+bytes([255])*15)
 cpu.mem_write(control-0x78,struct.pack('<Q',wall if present else wall^1)+source)
 cpu.mem_write(data,struct.pack('<4Q',control,15,14,int(present or bool(wall&1))))
 cpu.mem_write(data+0x1000,payload);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,wall);cpu.reg_write(UC_X86_REG_R8,data+0x1000)
 active=True
 try:call({'elevation':0x140aaaac0,'enclosure':0x140aa9610,'style':0x140aaa350}[kind])
 finally:active=False
 assert notifications==([(data+0x60,wall)]if present else[])
 return bytes(cpu.mem_read(control-0x70,112)).hex()+(' '+str(wall)if present else' -')

def create(counter,style,shape,placement):
 global active
 shape=bytes(shape);placement=bytes(placement)
 reset();curve=create_geometry(shape);cpu.mem_write(data+0xc0,struct.pack('<Q',counter))
 cpu.mem_write(0x143634d90,bytes([0])) # explicit input to inactive metadata+69 branch
 cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,curve['pointer']);cpu.reg_write(UC_X86_REG_XMM2,struct.unpack_from('<I',placement,0x40)[0]);cpu.reg_write(UC_X86_REG_R9,1)
 sp=stack+0x100008;cpu.mem_write(sp+0x28,bytes([0]))
 active=True
 try:
  call(0x14085a370);wall=cpu.reg_read(UC_X86_REG_RAX);assert wall==(counter+1)&0xffffffffffffffff
  assert len(inserted)==1 and notifications==[(data+0x30,wall)]
  for entry,payload in [(0x140aaaac0,placement[0x3c:0x40]),(0x140aa9610,shape),(0x140aaa350,bytes([style]))]:
   cpu.mem_write(data+0x1000,payload);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,wall);cpu.reg_write(UC_X86_REG_R8,data+0x1000);call(entry)
 finally:active=False
 assert notifications==[(data+0x30,wall)]+[(data+0x60,wall)]*3
 source=bytes(cpu.mem_read(data+0x18000-0x70,112));curve.update(wall=wall,counter=nf.u64(data+0xc0),source=source.hex(),notifications=notifications)
 return curve

def ui_event(shape,placement):
 """Entire original successful event-writing branch, including hint condition.
 This does not claim the earlier actual UI click/border/renderer decision.
 """
 reset();shape=bytes(shape);placement=bytes(placement);sp=stack+0x100080;frame=sp+0x500
 cpu.mem_write(data+0x10000,placement);cpu.mem_write(frame+0x10,shape[4:28]);cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RBP,frame)
 events=data+0x11000;ptr=data+0x12000;cpu.mem_write(events+0x20,struct.pack('<3Q',2,ptr,0));cpu.mem_write(events+0x40,struct.pack('<Q',17))
 if struct.unpack_from('<I',shape)[0]==0:
  cpu.reg_write(UC_X86_REG_R13,data+0x10000);cpu.mem_write(sp+0xf0,struct.pack('<Q',events));cpu.mem_write(sp+0xe0,struct.pack('<Q',data+0x14000));cpu.mem_write(sp+0xa4,struct.pack('<I',0x12345678))
  start,end,height=0x14165c5f2,0x14165c712,0x20
 else:
  cpu.reg_write(UC_X86_REG_RSI,data+0x10000);ref=data+0x13000;cpu.mem_write(ref,struct.pack('<QQQI',events,0,data+0x14000,0)+struct.pack('<I',0x12345678));cpu.reg_write(UC_X86_REG_RAX,ref)
  start,end,height=0x14155ddf7,0x14155e391,0x28
 cpu.emu_start(start,end,count=10000);assert cpu.reg_read(UC_X86_REG_RIP)==end
 assert nf.u64(events+0x30)==1 and nf.u64(events+0x40)==18, (shape.hex(),placement.hex(),nf.u64(events+0x30),nf.u64(events+0x40),hex(cpu.reg_read(UC_X86_REG_RIP)))
 assert struct.unpack('<I',cpu.mem_read(data+0x14000,4))[0]==0x12345678
 tag=bytes(cpu.mem_read(ptr,8));hint=bytes(cpu.mem_read(ptr+8,8))if nf.u64(ptr)else b''
 return (tag+hint+bytes(cpu.mem_read(ptr+16,height-8))).hex()

def remove_source(present,wall,source):
 global active
 reset();control=data+0x18000;cpu.mem_write(control,bytes([0])+bytes([255])*15);cpu.mem_write(control-0x78,struct.pack('<Q',wall if present else wall^1)+source)
 cpu.mem_write(data,struct.pack('<4Q',control,15,14,int(present or bool(wall&1))))
 output=data+0x1000;cpu.reg_write(UC_X86_REG_RCX,output);cpu.reg_write(UC_X86_REG_RDX,data);cpu.reg_write(UC_X86_REG_R8,wall)
 active=True
 try:
  try:call(0x14085aa60)
  except RuntimeError as error:
   if not present and 'panic 0x1428d9390' in str(error):
    assert not notifications and nf.u64(data+0x18)==int(bool(wall&1));return f'error MissingSource {nf.u64(data+0x18)} 0'
   raise
 finally:active=False
 assert notifications==[(data+0x90,wall)] and nf.u64(data+0x18)==0
 return bytes(cpu.mem_read(output,112)).hex()+' 0 1'

def creator_research(tag,counter,shape,height,elevation,style=0,rim=0,roof_color=0,previous=None,roof=None,hint=False,wall_color=0,timber_color=0):
 """Execute the whole original consumer; intentionally research until audited."""
 global active
 reset();cpu.mem_write(data+0x40000,bytes(0x80000));cpu.mem_write(data+0xc0,struct.pack('<Q',counter))
 wrappers=[];owners=[]
 for i in range(16):
  wrapper=data+0x40000+i*0x100;owner=data+0x50000+i*0x100;tick=wrapper+0x60
  cpu.mem_write(wrapper,struct.pack('<QQQI',owner,0,tick,0)+struct.pack('<I',0x12345678));wrappers.append(wrapper);owners.append(owner)
  # Every event writer has independent standard capacity and a real counter.
  cpu.mem_write(owner+0x20,struct.pack('<3Q',32,data+0x80000+i*0x2000,0));cpu.mem_write(owner+0x40,struct.pack('<Q',17))
 # EventReader owns exactly one input event in its first standard buffer.
 reader=data+0x45000;cursor=reader+0x80;events=data+0x46000;event=data+0x47000
 payload=struct.pack('<QQ',int(hint),9)+shape[4:20 if tag==0 else 28]+struct.pack('<ffQ',height,elevation,0)
 cpu.mem_write(event,payload);cpu.mem_write(events,struct.pack('<9Q',1,event,1,0,0,8,0,0,1));cpu.mem_write(reader,struct.pack('<QQ',cursor,events))
 cpu.mem_write(wrappers[0],struct.pack('<Q',data)) # ResMut InnerWalls.
 history=owners[2];cpu.mem_write(history,bytes(0x100));cpu.mem_write(history+0x30,struct.pack('<3Q',128,data+0xa0000,0));cpu.mem_write(history+0x58,struct.pack('<QQii',1,2,-1,-1))
 # Source reads these resources but this branch has no prior public/roof hint.
 for i in [9,10,11,12,13]:cpu.mem_write(owners[i],bytes(0x100))
 cpu.mem_write(owners[10]+0x1c,struct.pack('<III',wall_color,roof_color,timber_color));cpu.mem_write(owners[10]+0x41,bytes([rim,style]))
 bind_context(owners[11],wrappers[12],owners[8],owners[9],previous,roof,data+0xc0000)
 # stack args28..78 = writer/settings/colors/public/query resources.
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop))
 args=[wrappers[i]for i in range(3,14)]
 for offset,pointer in zip(range(0x28,0x80,8),args):cpu.mem_write(sp+offset,struct.pack('<Q',pointer))
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,reader);cpu.reg_write(UC_X86_REG_RDX,wrappers[0]);cpu.reg_write(UC_X86_REG_R8,wrappers[1]);cpu.reg_write(UC_X86_REG_R9,wrappers[2])
 active=True
 try:
  cpu.emu_start(0x14097e560 if tag==0 else 0x140a93790,stop,count=3000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
 finally:active=False
 source=bytes(cpu.mem_read(data+0x18000-0x70,112));curve=curve_at(data+0x18000-0x70);roofptr=nf.u64(owners[1]+0x28)
 assert nf.u64(owners[1]+0x30)==1
 return {'id':nf.u64(data+0xc0),'inserted':len(inserted),'notifications':list(notifications),'source':source.hex(),'curve':curve,'roof':bytes(cpu.mem_read(roofptr,64)).hex(),'history_pending_count':nf.u64(history+0x40),'last':nf.last[-10:]}

def inactive_mode(mode,subtype,tag,preview):
 reset();preview=bytes(preview);app=data+0x1000;resource=data+0x1100;tick=data+0x1200;view=data+0x2000
 cpu.mem_write(data,struct.pack('<Q',app));cpu.mem_write(app,struct.pack('<IB',mode,subtype));cpu.mem_write(view,preview)
 if tag==0:
  cpu.mem_write(resource,struct.pack('<4Q',data+0x3000,0,0,0)+struct.pack('<QQQI',view,0,tick,0)+struct.pack('<I',0x12345678))
  sp=stack+0x100008;cpu.mem_write(sp+0x30,struct.pack('<Q',resource));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.mem_write(sp,struct.pack('<Q',stop))
  cpu.reg_write(UC_X86_REG_RCX,data);cpu.emu_start(0x14165bed0,stop,count=10000)
 else:
  cpu.mem_write(resource,struct.pack('<4Q',data+0x3000,0,0,0)+struct.pack('<QQQI',view,0,tick,0)+struct.pack('<I',0x12345678));cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_R8,resource);call(0x14155d410)
 assert cpu.reg_read(UC_X86_REG_RIP)==stop
 assert struct.unpack('<I',cpu.mem_read(tick,4))[0]==0x12345678
 return '1 '+bytes(cpu.mem_read(view,40)).hex()

def appearance(settings,previous,roof=None):
 """Whole original colour/style selector with actual map/query reads."""
 global active
 reset();wall=9;pub=data+0x10000;control=data+0x15000;out=data+0x1000;config=data+0x2000
 w,r,t,style=settings;cpu.mem_write(config+0x1c,struct.pack('<III',w,r,t));cpu.mem_write(config+0x42,bytes([style]))
 if previous is not None:
  count,length,tag,oldstyle,oldwall,oldtimber=previous
  projection=bytearray(672);struct.pack_into('<Q',projection,0x10,count);struct.pack_into('<f',projection,0x30,length);struct.pack_into('<I',projection,0x258,tag);projection[0x299]=oldstyle
  cpu.mem_write(pub,struct.pack('<4Q',control,15,14,1));cpu.mem_write(control,bytes([0])+bytes([255])*15);cpu.mem_write(control-0x2a8,struct.pack('<Q',wall)+projection)
  cpu.mem_write(data+0x4000+wall*4,struct.pack('<I',oldwall));cpu.mem_write(data+0x5000+wall*4,struct.pack('<I',oldtimber))
 cpu.mem_write(data+0x3000,struct.pack('<3Q',16,data+0x4000,16));cpu.mem_write(data+0x3100,struct.pack('<3Q',16,data+0x5000,16))
 query=data+0x16000;world=data+0x18000;metadata=data+0x1a000
 cpu.mem_write(query,struct.pack('<QQ',world,metadata))
 if roof is not None:
  # Actual HashMap<WallId,Entity> and standard Bevy resolved entity/query carrier.
  ctrl=data+0x1c000;cpu.mem_write(pub+0x30,struct.pack('<4Q',ctrl,15,14,1));cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-16,struct.pack('<QQ',wall,1<<32))
  entities=data+0x1d000;cpu.mem_write(metadata+8,struct.pack('<QQ',entities,1));cpu.mem_write(entities,struct.pack('<5I',1,0,0,0,0))
  mask=data+0x1e000;cpu.mem_write(world+0x28,struct.pack('<Q',mask));cpu.mem_write(world+0x38,struct.pack('<Q',1));cpu.mem_write(mask,struct.pack('<Q',1));cpu.mem_write(world+0x110,struct.pack('<Q',0))
  tables=data+0x1f000;cpu.mem_write(metadata+0x1a8,struct.pack('<Q',tables));cpu.mem_write(tables+0x18,struct.pack('<Q',data+0x20000));cpu.mem_write(tables+0x38,struct.pack('<Q',data+0x21000));cpu.mem_write(data+0x21000,struct.pack('<Q',0xffffffffffffffff));cpu.mem_write(data+0x20010,struct.pack('<Q',data+0x22000));cpu.mem_write(data+0x22000,roof)
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<4Q',data+0x3100,pub,query,config))
 cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,out);cpu.reg_write(UC_X86_REG_RDX,int(previous is not None));cpu.reg_write(UC_X86_REG_R8,wall);cpu.reg_write(UC_X86_REG_R9,data+0x3000)
 active=True
 try:cpu.emu_start(0x140a94880,stop,count=30000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
 finally:active=False
 record=bytes(cpu.mem_read(out,28));opacity=record[16:28]if record[16]else bytes(12)
 return ' '.join(map(str,struct.unpack_from('<III',record)))+' '+str(record[12])+' '+opacity.hex()


def bind_context(pub,query,wall_colors,timber_colors,previous,roof,base):
 """Build only standard map/Entity/query carriers; all game lookups execute."""
 wall=9;control=base;wall_values=base+0x1000;timber_values=base+0x2000
 cpu.mem_write(base,bytes(0x10000))
 if previous is not None:
  count,length,tag,oldstyle,oldwall,oldtimber=previous
  projection=bytearray(672);struct.pack_into('<Q',projection,0x10,count);struct.pack_into('<f',projection,0x30,length);struct.pack_into('<I',projection,0x258,tag);projection[0x299]=oldstyle
  cpu.mem_write(pub,struct.pack('<4Q',control,15,14,1));cpu.mem_write(control,bytes([0])+bytes([255])*15);cpu.mem_write(control-0x2a8,struct.pack('<Q',wall)+projection)
  cpu.mem_write(wall_values+wall*4,struct.pack('<I',oldwall));cpu.mem_write(timber_values+wall*4,struct.pack('<I',oldtimber))
 cpu.mem_write(wall_colors,struct.pack('<3Q',16,wall_values,16));cpu.mem_write(timber_colors,struct.pack('<3Q',16,timber_values,16))
 world=base+0x3000;metadata=base+0x4000;cpu.mem_write(query,struct.pack('<QQ',world,metadata))
 if roof is not None:
  ctrl=base+0x5000;cpu.mem_write(pub+0x30,struct.pack('<4Q',ctrl,15,14,1));cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-16,struct.pack('<QQ',wall,1<<32))
  entities=base+0x6000;cpu.mem_write(metadata+8,struct.pack('<QQ',entities,1));cpu.mem_write(entities,struct.pack('<5I',1,0,0,0,0))
  mask=base+0x7000;cpu.mem_write(world+0x28,struct.pack('<Q',mask));cpu.mem_write(world+0x38,struct.pack('<Q',1));cpu.mem_write(mask,struct.pack('<Q',1));cpu.mem_write(world+0x110,struct.pack('<Q',0))
  tables=base+0x8000;cpu.mem_write(metadata+0x1a8,struct.pack('<Q',tables));cpu.mem_write(tables+0x18,struct.pack('<Q',base+0x9000));cpu.mem_write(tables+0x38,struct.pack('<Q',base+0xa000));cpu.mem_write(base+0xa000,struct.pack('<Q',0xffffffffffffffff));cpu.mem_write(base+0x9010,struct.pack('<Q',base+0xb000));cpu.mem_write(base+0xb000,roof)

def edit_rectangle(present,old_events,new_events,cursor,source,dt):
 """Whole original rectangle edit with actual EventReader::last and audio writes."""
 global active
 reset();old_shape=struct.pack('<I6f',1,1,0,0,0,2,3);curve=create_geometry(old_shape)
 row=bytearray(source);row[:56]=cpu.mem_read(curve['pointer'],56);wall=struct.unpack_from('<Q',(new_events or old_events)[-1],8)[0] if new_events or old_events else 9
 ctrl=data+0x18000;cpu.mem_write(ctrl,bytes([0])+bytes([255])*15);cpu.mem_write(ctrl-0x78,struct.pack('<Q',wall if present else wall^1)+bytes(row));cpu.mem_write(data,struct.pack('<4Q',ctrl,15,14,int(present)))
 reader=data+0x1000;events=data+0x2000;cursor_ptr=data+0x1100;cpu.mem_write(cursor_ptr,struct.pack('<Q',cursor));cpu.mem_write(reader,struct.pack('<QQ',cursor_ptr,events))
 cpu.mem_write(data+0x2100,b''.join(old_events));cpu.mem_write(data+0x3100,b''.join(new_events));total=len(old_events)+len(new_events)
 cpu.mem_write(events,struct.pack('<9Q',len(old_events),data+0x2100,len(old_events),0,len(new_events),data+0x3100,len(new_events),len(old_events),total))
 wrappers=[];owners=[]
 for i in range(4):
  wrapper=data+0x5000+i*0x100;owner=data+0x6000+i*0x100;tick=wrapper+0x60
  cpu.mem_write(wrapper,struct.pack('<QQQI',owner,0,tick,0)+struct.pack('<I',0x12345678));wrappers.append(wrapper);owners.append(owner)
  cpu.mem_write(owner+0x20,struct.pack('<3Q',32,data+0x8000+i*0x2000,0));cpu.mem_write(owner+0x40,struct.pack('<Q',17))
 cpu.mem_write(wrappers[0],struct.pack('<Q',data));cpu.mem_write(owners[3]+0x58,struct.pack('<I',dt))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<Q',wrappers[3]));cpu.reg_write(UC_X86_REG_RSP,sp)
 cpu.reg_write(UC_X86_REG_RCX,reader);cpu.reg_write(UC_X86_REG_RDX,wrappers[0]);cpu.reg_write(UC_X86_REG_R8,wrappers[1]);cpu.reg_write(UC_X86_REG_R9,wrappers[2])
 active=True
 try:cpu.emu_start(0x14097f620,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
 finally:active=False
 updated=cursor<total
 assert notifications==([(data+0x60,wall)]*2 if updated and present else [])
 assert nf.u64(owners[1]+0x30)==int(updated) and nf.u64(owners[2]+0x30)==int(updated)
 result={'source':bytes(cpu.mem_read(ctrl-0x70,112)).hex(),'notifications':list(notifications),'cursor':nf.u64(cursor_ptr),'actual_return':True,'updated':updated}
 if updated:
  result.update(material=bytes(cpu.mem_read(nf.u64(owners[1]+0x28),32)).hex(),motion=bytes(cpu.mem_read(nf.u64(owners[2]+0x28),40)).hex())
  if present:result['curve']=curve_at(ctrl-0x70)
 return result

def invalid_roof_notifications(roof_rows,invalid_ids,marked=False,reset_all=False):
 """Complete process_invalid_roof_notifications with real Query/History/Event ABI."""
 global active
 reset();world=data+0x1000;metadata=data+0x2000;query=data+0x3000;tables=data+0x4000;entities=data+0x5000;columns=data+0x6000;lookup=data+0x7000;roofs=data+0x8000
 cpu.mem_write(query,struct.pack('<QQ',world,metadata));cpu.mem_write(world+0xf8,struct.pack('<QQ',data+0x9000,1));cpu.mem_write(data+0x9000,struct.pack('<I',0));cpu.mem_write(world+0x108,bytes([1]));cpu.mem_write(world+0x110,struct.pack('<QQ',0,1));cpu.mem_write(metadata+0x1a8,struct.pack('<Q',tables))
 cpu.mem_write(tables+8,struct.pack('<QQ',entities,len(roof_rows)));cpu.mem_write(tables+0x18,struct.pack('<Q',columns));cpu.mem_write(tables+0x38,struct.pack('<QQ',lookup,2 if marked else 1));cpu.mem_write(lookup,struct.pack('<QQ',0xffffffffffffffff,1 if marked else 0));cpu.mem_write(columns+0x10,struct.pack('<Q',roofs));cpu.mem_write(entities,b''.join(struct.pack('<Q',(1<<32)+i)for i in range(len(roof_rows))));cpu.mem_write(roofs,b''.join(roof_rows))
 readers=[]
 for i,ids in enumerate([invalid_ids,[0]if reset_all else[]]):
  reader=data+0xa000+i*0x200;events=reader+0x80;cursor=reader+0x60;rows=data+0xb000+i*0x2000
  cpu.mem_write(rows,b''.join(struct.pack('<QQ',j,wall)for j,wall in enumerate(ids)));cpu.mem_write(reader,struct.pack('<QQ',cursor,events));cpu.mem_write(events,struct.pack('<9Q',len(ids),rows,len(ids),0,0,8,0,len(ids),len(ids)));readers.append(reader)
 history=data+0x10000;historywrapper=data+0x10100;writer=data+0x11000;writerwrapper=data+0x11100
 for wrapper,owner in[(historywrapper,history),(writerwrapper,writer)]:cpu.mem_write(wrapper,struct.pack('<QQQI',owner,0,wrapper+0x60,0)+struct.pack('<I',0x12345678))
 cpu.mem_write(history+0x30,struct.pack('<3Q',128,data+0x12000,0));cpu.mem_write(history+0x58,struct.pack('<QQii',1,2,-1,-1));cpu.mem_write(writer+0x20,struct.pack('<3Q',128,data+0x13000,0));cpu.mem_write(writer+0x40,struct.pack('<Q',17))
 sp=stack+0x100008;cpu.mem_write(sp,struct.pack('<Q',stop));cpu.mem_write(sp+0x28,struct.pack('<Q',writerwrapper));cpu.reg_write(UC_X86_REG_RSP,sp);cpu.reg_write(UC_X86_REG_RCX,query);cpu.reg_write(UC_X86_REG_RDX,readers[0]);cpu.reg_write(UC_X86_REG_R8,readers[1]);cpu.reg_write(UC_X86_REG_R9,historywrapper)
 active=True
 try:cpu.emu_start(0x14062eb80,stop,count=1000000);assert cpu.reg_read(UC_X86_REG_RIP)==stop
 finally:active=False
 history_n=nf.u64(history+0x40);history_payloads=[bytes(cpu.mem_read(nf.u64(data+0x12000+i*24+8)+16,56)).hex()for i in range(history_n)]
 n=nf.u64(writer+0x30);return {'actual_return':True,'history_payloads':history_payloads,'events':[bytes(cpu.mem_read(data+0x13000+i*24,24)).hex()for i in range(n)],'history_count':nf.u64(history+0x40),'history_rows':bytes(cpu.mem_read(data+0x12000,nf.u64(history+0x40)*24)).hex()}
