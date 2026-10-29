"""执行 B 灯笼点亮分支及原一次性音频生命周期；SDK/分配边界显式桥接。"""
from pathlib import Path
import hashlib, json, struct
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[3]
raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
D,S,STOP=0x20000000,0x30000000,0x200ff000
u.mem_map(D,0x100000);u.mem_map(S,0x100000)
OK=0x8000000000000004
calls=[];panic=None;mode='';failure=None;playback=2
q=lambda a,v:u.mem_write(a,struct.pack('<Q',v))
uq=lambda a:struct.unpack('<Q',u.mem_read(a,8))[0]
def ret(value=0):
 sp=u.reg_read(UC_X86_REG_RSP);u.reg_write(UC_X86_REG_RIP,uq(sp));u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RAX,value)
def result(out,failed):
 u.mem_write(out,bytes(56));q(out,0 if failed else OK);ret(out)
def hook(uc,a,size,_):
 global panic
 rcx=uc.reg_read(UC_X86_REG_RCX);rdx=uc.reg_read(UC_X86_REG_RDX);r8=uc.reg_read(UC_X86_REG_R8);r9=uc.reg_read(UC_X86_REG_R9)
 if mode=='dispatch' and a==0x140aa3360:
  calls.append({'call':'position_helper','name':bytes(u.mem_read(rdx,r8)).decode(),'position_bits':list(struct.unpack('<III',u.mem_read(r9,12))),'core':rcx});ret()
 elif a==0x140a54440:
  calls.append({'call':'create','name':bytes(u.mem_read(r8,r9)).decode()})
  u.mem_write(rcx,bytes(48));q(rcx,2 if failure=='create'else 1);q(rcx+8,D+0x7000);u.mem_write(rcx+0x28,b'\x02');ret(rcx)
 elif a==0x141436e50:
  words=list(struct.unpack('<12I',u.mem_read(r8,48)))
  calls.append({'call':'set_3d_attributes','position_bits':words[:3],'velocity_bits':words[3:6],'forward_bits':words[6:9],'up_bits':words[9:12]})
  result(rcx,False)
 elif a==0x141437730:
  calls.append({'call':'playback'});out=rcx;result(out,failure=='playback');u.mem_write(out+8,bytes([playback]))
 elif a==0x1414372d0:
  calls.append({'call':'start'});result(rcx,failure=='start')
 elif a==0x1414378f0:
  calls.append({'call':'release'});result(rcx,failure=='release')
 elif a==0x141437440:
  calls.append({'call':'stop','mode':r8});result(rcx,False)
 elif a==0x1428d9760:
  panic={'target':hex(a),'return_address':hex(uq(uc.reg_read(UC_X86_REG_RSP)))};uc.emu_stop()
 elif a==0x140613c20:ret() # 原 AudioInstance 名称 cap=0，通常不会到这里。
 elif a==0x14145e1a0 and mode=='dispatch':uc.emu_stop()
u.hook_add(UC_HOOK_CODE,hook)
def reset():
 global calls,panic
 calls=[];panic=None;u.mem_write(D,bytes(0x10000));u.mem_write(S,bytes(0x100000));sp=S+0xf0008;q(sp,STOP);u.reg_write(UC_X86_REG_RSP,sp)
table=0x142c631dc;delta=struct.unpack('<i',u.mem_read(table+14*4,4))[0];assert table+delta==0x14145f174
name=bytes(u.mem_read(0x142c62a00,25)).decode();assert name=='event:/SFX/Lantern/TurnOn'
positions=[[0,0,0],[0x80000000,0x41dc0000,0xc0500000],[0x449a5000,0xc1180000,0],[0x7fc01234,0x7f800000,0xff800000]]
dispatch=[]
for p in positions:
 reset();mode='dispatch';bp=D+0x1000;event=D+0x2000;q(bp+0x168,D+0x3000);u.mem_write(event+0x18,struct.pack('<III',*p))
 u.reg_write(UC_X86_REG_RBP,bp);u.reg_write(UC_X86_REG_RDI,event);u.emu_start(0x14145f174,STOP,count=10000)
 assert u.reg_read(UC_X86_REG_RIP)==0x14145e1a0 and len(calls)==1 and calls[0]['position_bits']==p
 dispatch.append({'input_position_bits':p,'calls':calls})
lifecycle=[]
for p in positions:
 for playback in range(5):
  for failure in [None,'create','playback','start','release']:
   reset();mode='helper';core=D+0x3000;position=D+0x4000;q(core,D+0x5000);u.mem_write(position,struct.pack('<III',*p))
   for reg,v in [(UC_X86_REG_RCX,core),(UC_X86_REG_RDX,0x142c62a00),(UC_X86_REG_R8,25),(UC_X86_REG_R9,position)]:u.reg_write(reg,v)
   u.emu_start(0x140aa3360,STOP,count=10000)
   assert panic or u.reg_read(UC_X86_REG_RIP)==STOP
   expected_panic=failure in ['create','playback','release'] or (failure=='start' and playback in [2,4])
   assert bool(panic)==expected_panic,(playback,failure,calls,panic)
   lifecycle.append({'input_position_bits':p,'playback':playback,'failure':failure,'calls':calls,'panic':panic})
# 单独执行原 set_position 错误分支，终止于格式化入口，避免伪造 log-once 容器。
reset();mode='position_failure';instance=D+0x6000;position=D+0x4000;q(instance,1);q(instance+8,D+0x7000)
# 在原 SDK 返回比较前注入 Err，终止于原警告格式化路径，不伪造 log-once 容器。
def position_error_hook(uc,a,size,_):
 global panic
 if a==0x140a57e18:
  out=uc.reg_read(UC_X86_REG_RBP)-0x50;q(out,0)
 elif a==0x140a57e2c:panic={'path':'set_position warning formatting','address':hex(a)};uc.emu_stop()
u.hook_add(UC_HOOK_CODE,position_error_hook)
u.reg_write(UC_X86_REG_RCX,instance);u.reg_write(UC_X86_REG_RDX,position);u.emu_start(0x140a57da0,STOP,count=10000)
assert panic and panic['path']=='set_position warning formatting'
report={'binary_sha256':sha,'body':'0x14145dc20','kind14_entry':'0x14145f174','jump_table':{'base':hex(table),'index':14,'signed_offset':delta,'target':hex(table+delta)},'string':{'va':'0x142c62a00','length':25,'value':name},'bridges':['AudioCoreInner::instantiate_audio returns typed variant with cap0','FMOD EventInstance SDK boundaries','panic termination; no Windows unwind execution','deallocation no-op; no allocated name in bridge'],'dispatch':dispatch,'lifecycle':lifecycle,'set_position_error_branch':panic,'boundaries':['set_position warning formatter/log-once containers examined statically, not emulated','SDK implementation/banks/speaker output not executed','creation bridge tests original caller failure gate, not Studio get_event implementation']}
Path(__file__).with_name('m01-lantern-audio-native.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'dispatch':len(dispatch),'lifecycle':len(lifecycle),'fatal':sum(bool(r['panic'])for r in lifecycle),'set_position_error_branch':panic}))
