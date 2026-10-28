"""Execute complete original dispatcher. Existing FMOD/tracing functions are
boundary hooks, not replacements for dispatch/reader/Tool-selection instructions."""
from pathlib import Path
import json, struct, hashlib, sys
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path('/home/ubuntu/test/games_test'); OUT=Path(__file__).resolve().parent
raw=(ROOT/'tiny-glade.exe').read_bytes(); pe=pefile.PE(data=raw,fast_load=True)
base=pe.OPTIONAL_HEADER.ImageBase; image=pe.get_memory_mapped_image()
assert hashlib.sha256(image[0x9d6b90:0x9d843f]).hexdigest()=='4b3e4619eedeb72c863bda572677b687207fe33182ce84fb6e7ea5fa758df192'
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x50000000
u.mem_map(D,0x200000);u.mem_map(S,0x20000);u.mem_map(STOP,0x1000)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def rdq(a):return struct.unpack('<Q',u.mem_read(a,8))[0]
def ret():
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,rdq(sp));u.reg_write(UC_X86_REG_RSP,sp+8)
def f3():return struct.unpack('<f',u.reg_read(UC_X86_REG_XMM3).to_bytes(16,'little')[:4])[0]
trace=[]
STATE=D+0x1000

def hook(cpu,pc,sz,_):
 if pc==STOP:cpu.emu_stop();return
 rcx=cpu.reg_read(UC_X86_REG_RCX);rdx=cpu.reg_read(UC_X86_REG_RDX);r8=cpu.reg_read(UC_X86_REG_R8);r9=cpu.reg_read(UC_X86_REG_R9)
 if pc==0x1405b0d70:cpu.reg_write(UC_X86_REG_RAX,0) # hash boundary, fixture bucket hash0
 elif pc==0x140c16a00:cpu.reg_write(UC_X86_REG_RAX,0) # tracing disabled
 elif pc==0x1409d8e50:trace.append(['oneshot',bytes(cpu.mem_read(rdx,r8)).decode()])
 elif pc==0x140a53db0:trace.append(['global',bytes(cpu.mem_read(rdx,r8)).decode(),f3()])
 elif pc==0x140a59430:trace.append(['loop_stop'])
 elif pc==0x140a596b0:trace.append(['loop_start'])
 elif pc==0x140a57da0:trace.append(['position',*struct.unpack('<3f',cpu.mem_read(rdx,12))])
 elif pc==0x140a57670:trace.append(['parameter',bytes(cpu.mem_read(rdx,r8)).decode(),f3()])
 elif pc==0x140a57300:
  if rcx in (STATE,STATE+0x40,STATE+0x70):trace.append(['cached_start',hex(rcx-STATE)])
 elif pc==0x140a54440:
  trace.append(['oneshot',bytes(cpu.mem_read(r8,r9)).decode()]);cpu.mem_write(rcx,bytes(48))
 elif pc==0x14089a280:pass # temporary AudioInstance drop boundary
 elif pc==0x140613c20:pass # no allocated photo names in hooked constructor
 else:return
 ret()
u.hook_add(UC_HOOK_CODE,hook)

def run(sounds=[],modes=[],photo=[],begin=False,finish=False,walls_changed=[],sound_old=[],public_wall=None):
 global trace
 trace=[];u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x20000))
 for reg in [UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RBP,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15]:u.reg_write(reg,0)
 for reg in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):u.reg_write(reg,0)
 def reader(index,records,stride,old=[]):
  a=D+0x3000+index*0x200;count=a+0x30;events=a+0x40;oldbuf=a+0xa0;newbuf=D+0x10000+index*0x10000
  q(a,count);q(a+8,events);q(count,0)
  q(events+8,oldbuf);q(events+0x10,len(old));q(events+0x18,0)
  q(events+0x28,newbuf);q(events+0x30,len(records));q(events+0x38,len(old));q(events+0x40,len(old)+len(records))
  if old:u.mem_write(oldbuf,b''.join(x.ljust(stride,b'\0') for x in old))
  if records:u.mem_write(newbuf,b''.join(x.ljust(stride,b'\0') for x in records))
  return a
 soundrec=lambda x:struct.pack('<I',x[0])+x[1].ljust(20,b'\0')+struct.pack('<Q',0)
 args=[reader(0,[struct.pack('<QQ',0,x) for x in walls_changed],16),
       reader(1,[struct.pack('<I',a[0])+a[1].ljust(12,b'\0')+struct.pack('<I',b)+bytes(12)+bytes(8) for a,b in modes],40),
       reader(2,[],16),reader(3,[bytes(8)] if begin else [],8),reader(4,[bytes(8)] if finish else [],8),
       reader(5,[soundrec(x) for x in sounds],32,[soundrec(x) for x in sound_old]),
       reader(6,[struct.pack('<QB',0,x) for x in photo],16)]
 # Original Res/ResMut views, maintained change tick, and empty PublicWalls map.
 corearg=D;core=D+0x100;inner=D+0x200;statearg=D+0x300;tick=D+0x400;wallarg=D+0x500;wallmap=D+0x600
 q(corearg,core);q(core,inner);q(statearg,STATE);q(statearg+0x10,tick);q(wallarg,wallmap)
 if public_wall is not None:
  wallid,special,shape=public_wall;ctrl=D+0x9000;entry=ctrl-0x2a8
  q(wallmap,ctrl);q(wallmap+8,0);q(wallmap+0x18,1);u.mem_write(ctrl,bytes(16))
  q(entry,wallid);q(entry+0x18,2);u.mem_write(entry+0x38,struct.pack('<f',1.))
  u.mem_write(entry+0x29e,bytes([special]));u.mem_write(entry+0x260,struct.pack('<I',shape))
 # The rotate ring selected by kind34 must expose a valid first instance.
 q(STATE+0xe0,4);q(STATE+0xd8,D+0x2000);q(STATE+0xf8,0)
 sp=S+0x10000-8;q(sp,STOP)
 for index,value in enumerate(args[2:]+[wallarg]):q(sp+0x28+index*8,value)
 for reg,value in [(UC_X86_REG_RCX,corearg),(UC_X86_REG_RDX,statearg),(UC_X86_REG_R8,args[0]),(UC_X86_REG_R9,args[1]),(UC_X86_REG_RSP,sp)]:u.reg_write(reg,value)
 u.reg_write(UC_X86_REG_MXCSR,0x1f80);u.reg_write(UC_X86_REG_RFLAGS,0x202)
 try:u.emu_start(0x1409d6b90,STOP,count=50000)
 except Exception as e:raise RuntimeError(f'{e} pc={u.reg_read(UC_X86_REG_RIP):x} trace={trace}')
 assert u.reg_read(UC_X86_REG_RIP)==STOP
 return {'trace':trace.copy(),'cached_wall':[rdq(STATE+0x30),rdq(STATE+0x38)]}

rows=[]
for kind in range(36):
 payload=struct.pack('<f3fI',-2.5,1.,-3.,8.,0) if kind==34 else bytes(20)
 if kind==1:payload=bytes(4)+struct.pack('<Q',2**64-1)+bytes(8)
 result=run(sounds=[(kind,payload)])
 rows.append({'case':'sound','kind':kind,'payload_hex':payload.hex(),**result})
for tag in range(21):
 for byte in ([0,1,2] if tag==6 else [56,57,58,59,60,61,62,63,0] if tag==11 else [0]):
  rows.append({'case':'mode','tag':tag,'byte':byte,**run(modes=[((tag,bytes([byte])),0)])})
for modes in [[((15,b''),13)],[((13,b''),7)],[((16,b''),7),((17,b''),16)]]:
 rows.append({'case':'mode_stream','modes':[[x[0][0],x[1]] for x in modes],**run(modes=modes)})
rows.append({'case':'photo_height_walls',**run(photo=[0,1],begin=True,finish=True,walls_changed=[1,99])})
rows.append({'case':'sound_last_new',**run(sounds=[(14,b''),(15,b'')])})
rows.append({'case':'sound_last_old',**run(sound_old=[(14,b''),(15,b'')])})
rows.append({'case':'sound_new_beats_old',**run(sounds=[(14,b'')],sound_old=[(15,b'')])})
for special,shape in [(0,0),(0,1),(0,2),(1,2)]:
 rows.append({'case':'wall_mode','special':special,'shape':shape,**run(modes=[((1,struct.pack('<IQ',0,99)),0)],public_wall=(99,special,shape))})
report={'original_body_sha256':'4b3e4619eedeb72c863bda572677b687207fe33182ce84fb6e7ea5fa758df192','original_image_sha256':hashlib.sha256(raw).hexdigest(),'scope':'Complete original6319B dispatcher instructions execute; FMOD instance/global operations and disabled tracing are isolated original call boundaries. No real FMOD/OS/GPU claims. PublicWalls is empty in these first probes.','cases':rows}
(OUT/'signifier-audio-native-execution.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'original_dispatcher_cases':len(rows),'output':str(OUT/'signifier-audio-native-execution.json')}))
