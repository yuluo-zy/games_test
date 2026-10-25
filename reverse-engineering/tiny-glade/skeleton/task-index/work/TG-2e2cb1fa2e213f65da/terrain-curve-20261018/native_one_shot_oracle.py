"""Original B tag4/5 dispatch branches with resolved AudioInstance API boundary."""
import json,struct,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
recipe=Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/shared-foundation-2026-10-13/utils-base/native_spline_rdp.py')
s=recipe.read_text().split('def spline(')[0].replace("Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')", "Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')")
exec(compile(s,str(recipe),'exec'),globals());OUT=Path(__file__).resolve().parent
calls=[]
def bridge(uc,address,size,data):
 if address==0x141461cfd:uc.emu_stop()
 elif address==0x140a54440:
  out,name,n=[u.reg_read(r)for r in[UC_X86_REG_RCX,UC_X86_REG_R8,UC_X86_REG_R9]];calls.append({'op':'create','name':bytes(u.mem_read(name,n)).decode()});u.mem_write(out,bytes(48));ret(out)
 elif address==0x140a57da0:
  p=u.reg_read(UC_X86_REG_RDX);calls.append({'op':'position','bits':list(struct.unpack('<3I',u.mem_read(p,12)))});ret()
 elif address==0x140a57300:calls.append({'op':'start'});ret()
 elif address==0x140a57670:
  name,n=[u.reg_read(r)for r in[UC_X86_REG_RDX,UC_X86_REG_R8]];calls.append({'op':'parameter','name':bytes(u.mem_read(name,n)).decode(),'bits':u.reg_read(UC_X86_REG_XMM3)&0xffffffff});ret()
 elif address==0x14089a280:ret()
u.hook_add(UC_HOOK_CODE,bridge)
def case(tag,value,magnitude,speed):
 global calls
 reset();calls=[];frame=STACK+0x8000;core=INPUT+0x1000;studio=INPUT+0x2000;event=INPUT+0x3000;settings=INPUT+0x4000
 for p in[frame-16,core,studio,event,settings]:u.mem_write(p,bytes(512))
 u.mem_write(frame+0x38,struct.pack('<Q',core));u.mem_write(core,struct.pack('<Q',studio));u.mem_write(frame-8,struct.pack('<Q',settings));u.mem_write(settings+0x58,pack([speed]));u.mem_write(event+8,struct.pack('<I',tag)+pack([3,-1,4,value,magnitude]))
 u.reg_write(UC_X86_REG_RBP,frame);u.reg_write(UC_X86_REG_R13,frame);u.reg_write(UC_X86_REG_R15,event)
 for r,v in[(UC_X86_REG_XMM10,5),(UC_X86_REG_XMM11,20),(UC_X86_REG_XMM12,1)]:u.reg_write(r,bits(v))
 u.emu_start(0x141461d90 if tag==5 else 0x141461d4d,STOP,count=10000)
 return {'tag':tag,'value_bits':bits(value),'magnitude_bits':bits(magnitude),'speed_bits':bits(speed),'calls':calls}
cases=[case(tag,value,magnitude,speed)for tag in[4,5]for value,magnitude,speed in[(.4,2.5,1),(26.5,4,2),(-1,.1,0)]]
(OUT/'native-one-shot-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'ranges':['0x141461d4d..0x141461cfd','0x141461d90..0x141461cfd'],'method':'Original branch and numeric parameter calculations execute. AudioCore::instantiate_audio and AudioInstance set_position/start/set_parameter/drop are resolved API boundaries with recorded args; no FMOD/device calls.','cases':cases},indent=2)+'\n');print({'one_shot_cases':len(cases)})
