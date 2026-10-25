from pathlib import Path
import struct,json,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).parent
pe=pefile.PE('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');BASE=pe.OPTIONAL_HEADER.ImageBase

def run(rows,events=(),hide=False,cursor=None,multiplier=1.,water=2.,garden=3., operation="brush",mode=7,path=0.8,phase=0,timer=0.,debug_enabled=True):
 u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(BASE,(pe.OPTIONAL_HEADER.SizeOfImage+4095)&~4095)
 for s in pe.sections:u.mem_write(BASE+s.VirtualAddress,s.get_data())
 M=0x10000000;u.mem_map(M,0x1000000);STOP=M;heap=M+0x1000
 def alloc(n=256):
  nonlocal heap
  a=heap;heap+=(n+255)&~255;return a
 def w(a,v):u.mem_write(a,struct.pack('<Q',v&0xffffffffffffffff))
 def q(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
 def d(a,v):u.mem_write(a,struct.pack('<I',v))
 def f(a,v):u.mem_write(a,struct.pack('<f',v))
 query,state,world,archs,ids=[alloc(2048) for _ in range(5)]
 w(query,state);w(query+8,world);d(query+0x14,42);w(state+0xf8,ids);w(state+0x100,len(rows));u.mem_write(state+0x108,b'\1');w(state+0x110,0);w(state+0x118,1);w(world+0x1a8,archs)
 if operation!='brush':w(state+0x110,1)
 if operation=='human':w(state+0x118,0)
 transforms=[]
 for i,(kind,translation) in enumerate(rows):
  d(ids+i*4,i);arch=archs+i*0x48;entities,columns,cmap,brush,ticks,trans=[alloc(256) for _ in range(6)]
  w(arch+8,entities);w(arch+0x10,1);w(arch+0x18,columns);w(arch+0x38,cmap);w(arch+0x40,2)
  w(entities,0x100000000+i);w(cmap,~0);w(columns+0x10,brush);u.mem_write(brush,struct.pack('<3f',1.,float(kind),3.));w(columns+0x30+0x28,ticks)
  if translation is not None:
   w(cmap+8,~1);w(columns+0x30+0x10,trans);u.mem_write(trans,struct.pack('<4f3f3f',0,0,0,1,*translation,2,2,2));transforms.append(trans)
  else:transforms.append(None)
  if operation=='ui2d':u.mem_write(trans,struct.pack('<4f',1.,2.,30.,40.))
 reader,readcursor,collection,buf=[alloc(4096) for _ in range(4)]
 w(reader,readcursor);w(reader+8,collection);w(collection+0x28,buf);w(collection+0x30,len(events));w(collection+0x40,len(events))
 for i,(kind,flag,position) in enumerate(events):
  a=buf+i*32;w(a,i);u.mem_write(a+8,bytes([flag]));u.mem_write(a+12,struct.pack('<3f',*position));u.mem_write(a+24,bytes([kind]))
 cursorarg,cursordata,commands,queue,queuebuf,debugarg,debug,multarg,mult,brusharg,brush=[alloc(8192) for _ in range(11)]
 w(cursorarg,cursordata)
 if cursor is not None:u.mem_write(cursordata,b'\1');u.mem_write(cursordata+4,struct.pack('<3f',*cursor))
 w(commands+8,queue);w(queue,8192);w(queue+8,queuebuf);w(debugarg,debug);u.mem_write(debug+0xfa,bytes([hide]));w(multarg,mult);f(mult,multiplier);w(brusharg,brush);f(brush+4,water);f(brush+8,garden)
 actionsarg,actions=alloc(),alloc(1024);w(actionsarg,actions);d(actions+0x110,phase);f(actions+0x114,timer);u.mem_write(debug+0xf8,bytes([debug_enabled]));
 modearg,modeval=alloc(),alloc();w(modearg,modeval);d(modeval,mode);f(brush,path)
 rsp=M+0xf00008;w(rsp,STOP)
 for i,a in enumerate([debugarg,multarg,brusharg]):w(rsp+0x28+i*8,a)
 for r,v in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,query),(UC_X86_REG_RDX,reader),(UC_X86_REG_R8,cursorarg),(UC_X86_REG_R9,commands)]:u.reg_write(r,v)
 look_calls=[]
 def hook(uc,a,size,ctx):
  if a==0x140c16a00:
   sp=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RAX,0);uc.reg_write(UC_X86_REG_RIP,q(sp));uc.reg_write(UC_X86_REG_RSP,sp+8)
  elif a==0x140c96d40 and operation=='look':
   dest=uc.reg_read(UC_X86_REG_RCX);u.mem_write(dest,struct.pack('<3f',15.,25.,35.));sp=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RAX,dest);uc.reg_write(UC_X86_REG_RIP,q(sp));uc.reg_write(UC_X86_REG_RSP,sp+8)
  elif a==0x140c9ce00 and operation=='look':
   rcx,rdx,r8=[uc.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]]
   look_calls.append({'translation_bits':list(struct.unpack('<3I',u.mem_read(rcx+16,12))),'target_bits':list(struct.unpack('<3I',u.mem_read(rdx,12))),'up_bits':list(struct.unpack('<3I',u.mem_read(r8,12)))})
   sp=uc.reg_read(UC_X86_REG_RSP);uc.reg_write(UC_X86_REG_RIP,q(sp));uc.reg_write(UC_X86_REG_RSP,sp+8)
  elif a==STOP:uc.emu_stop()
 u.hook_add(UC_HOOK_CODE,hook)
 if operation=='path':
  for r,v in [(UC_X86_REG_RCX,modearg),(UC_X86_REG_RDX,query),(UC_X86_REG_R8,brusharg)]:u.reg_write(r,v)
 elif operation=='look':u.reg_write(UC_X86_REG_RDX,modearg)
 elif operation=='human':u.reg_write(UC_X86_REG_RDX,actionsarg)
 elif operation=='ui2d':u.mem_write(modeval,struct.pack('<2f',12.,34.));u.reg_write(UC_X86_REG_RDX,modearg)
 entry={'brush':0x140865050,'path':0x140b1c010,'look':0x140873de0,'human':0x1408b2b30,'ui2d':0x14092a100}[operation]
 try:u.emu_start(entry,STOP,count=100000)
 except Exception as e:raise RuntimeError(str(e)+' RIP='+hex(u.reg_read(UC_X86_REG_RIP)))
 cmds=[];off=0
 while off<q(queue+16):
  a=queuebuf+off;fn=q(a)
  if fn==0x1412dce70:cmds.append({'remove':q(a+16)});off+=24
  elif fn==0x1412db8c0:cmds.append({'insert':q(a+0x48),'transform_bits':list(struct.unpack('<10I',u.mem_read(a+8,40)))});off+=104
  else:raise ValueError(hex(fn))
 return {'transforms':[None if a is None else list(struct.unpack('<10I',u.mem_read(a,40))) for a in transforms],'commands':cmds,'consumed':q(readcursor),'look_calls':look_calls}


if __name__=='__main__':
 cases=[]
 for phase in range(5):
  for timer in [-0.1,0.,0.1,float('nan')]:
   for debug in [False,True]:
    x=dict(rows=[(2.,None),(4.,(1.,2.,3.)),(float('nan'),None)],operation='human',phase=phase,timer=timer,debug_enabled=debug,cursor=(10.,20.,30.));r=run(**x);x['timer_bits']=struct.unpack('<I',struct.pack('<f',x.pop('timer')))[0];x['rows'][2]=(0x7fc00000,None);cases.append({'input':x,'native':r})
 (OUT/'native-human-scale-fixtures.json').write_text(json.dumps(cases,indent=2),encoding='utf8');print('human',len(cases))
 a=[{'count':n,'native':run([(0,(1.,2.,3.))]*n,operation='ui2d')} for n in range(4)]
 (OUT/'native-ui2d-follow-fixtures.json').write_text(json.dumps(a,indent=2),encoding='utf8');print('ui2d',len(a))
