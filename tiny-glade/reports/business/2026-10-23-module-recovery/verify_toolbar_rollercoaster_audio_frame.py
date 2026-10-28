"""原ride逐车帧尾音频指令；桥接设备调用作顺序/参数记录，不模拟FMOD。"""
from pathlib import Path
import hashlib,json,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack=0x20000000,0x30000000;u.mem_map(data,0x20000);u.mem_map(stack,0x20000);rbp,rsp=stack+0x8000,stack+0x7008
state={}
def put(a,fmt,*v):u.mem_write(a,struct.pack('<'+fmt,*v))
def get(a,fmt):return struct.unpack('<'+fmt,u.mem_read(a,struct.calcsize('<'+fmt)))
def ret(v=None):
 if v is not None:u.reg_write(UC_X86_REG_RAX,v)
 sp=u.reg_read(UC_X86_REG_RSP);ra=get(sp,'Q')[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ra)
def hook(uc,a,size,user):
 if a==0x1403ba53f:uc.emu_stop();return
 if a==0x140c946a0:put(uc.reg_read(UC_X86_REG_RCX),'3f',*state['position']);ret();return
 ptr=uc.reg_read(UC_X86_REG_RCX)
 if a in [0x140a57da0,0x140a575a0,0x140a57670,0x140a57300,0x140a57470,0x140a57550]:
  sound=state['ids'][ptr];name='';vals=[]
  kind={0x140a57da0:4,0x140a575a0:1,0x140a57670:2,0x140a57300:3,0x140a57470:5,0x140a57550:6}[a]
  if kind==4:vals=list(get(uc.reg_read(UC_X86_REG_RDX),'3I'))
  if kind==1:vals=[uc.reg_read(UC_X86_REG_XMM1)&0xffffffff]
  if kind==2:
   name=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RDX),uc.reg_read(UC_X86_REG_R8))).decode();vals=[uc.reg_read(UC_X86_REG_XMM3)&0xffffffff]
  state['calls'].append((kind,sound,name,vals))
  ret(int(sound>=30 and (state['playing']>>(sound-30))&1) if kind==6 else None);return
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for y,speed,cross,bottom,timer,playing in itertools.product([-.31,-.3,0.0],[0.,2.,40.],[False,True],[False,True],[0.,.1],[0,1,63]):
 u.mem_write(data,b'\0'*0x20000);u.mem_write(stack,b'\0'*0x20000)
 cart,st=data+0x1000,data+0x2000;state.update(position=(1.25,y,-3.0),calls=[],ids={},playing=playing)
 for idx,off in enumerate([0x30,0x68,0xa0,0xd8]):
  state['ids'][cart+off]=idx;put(cart+off,'Q',0);put(cart+off+0x30,'f',.7)
 for off,count,start,ptr in [(0x110,10,10,data+0x3000),(0x128,10,20,data+0x4000),(0x140,6,30,data+0x5000)]:
  put(cart+off,'QQQ',count,ptr,count)
  for idx in range(count):state['ids'][ptr+idx*56]=start+idx;put(ptr+idx*56,'Q',0);put(ptr+idx*56+0x30,'f',.7)
 put(cart+0x158,'Qf',2,timer);put(cart+0x194,'f',-1.25);put(cart+0x1a0,'f',.08)
 put(st+0x40,'2f',0,speed)
 for off,v in [(0x370,cart),(0x600,cart+0x1b0),(0x780,st),(0x760,st+0x18),(0x738,1),(0x718,data+0x6000)]:put(rbp+off,'Q',v)
 put(data+0x6000,'B',int(cross));put(rbp+0x610,'B',int(bottom));put(rbp+0x724,'f',.016)
 u.reg_write(UC_X86_REG_RBP,rbp);u.reg_write(UC_X86_REG_RSP,rsp);u.reg_write(UC_X86_REG_XMM7,0x3f800000)
 u.emu_start(0x1403b9edb,0x1403ba53f,count=20000)
 rows.append(((list(get(data+0x1000+0x194,'I')),list(struct.unpack('<3I',struct.pack('<3f',*state['position']))),struct.unpack('<I',struct.pack('<f',speed))[0],cross,bottom,struct.unpack('<I',struct.pack('<f',timer))[0],playing),(get(cart+0x1a0,'I')[0],get(cart+0x160,'I')[0],get(cart+0x158,'Q')[0],state['calls'])))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-audio-frame.ron'
def ron(v):
 if isinstance(v,bool):return str(v).lower()
 if isinstance(v,str):return json.dumps(v)
 if isinstance(v,tuple):return '('+','.join(map(ron,v))+')'
 if isinstance(v,list):return '['+','.join(map(ron,v))+']'
 return str(v)
fixture.write_text(ron(rows)+'\n');out=dict(binary_sha256=sha,cases=len(rows),entry='0x1403b9edb',exit='0x1403ba53f',fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),columns=['input(Ax_bits1,position_bits3,speed_bits,cross,bottom,wheel_timer_bits,lantern_playing_mask)','output(peak_bits,wheel_timer_bits,wheel_index,calls(kind,id,name,value_bits_vec))'],bridges=['Spline.position_at_t resolves supplied position','AudioInstance device wrappers record volume/parameter/start/stop/position/is_playing; controlled lantern playback mask'],scope='单车全部4实例/3池可用，原位置更新/速度参数/过零选择池/水面分支/触底轮声/计时索引/Ax音量完整指令。无FMOD设备执行，不覆盖None/Err，单帧输入是明确边界')
Path(__file__).with_name('toolbar-rollercoaster-audio-frame-native.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n');print(json.dumps(out,ensure_ascii=False))
