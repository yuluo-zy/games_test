"""原unstuck每羊完整CFG；射线/采样作为受控输入，状态选择执行原指令。"""
from pathlib import Path
import hashlib,itertools,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack=0x20000000,0x30000000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000);rsp=stack+0x7000;state={}
def put(a,fmt,*v):u.mem_write(a,struct.pack('<'+fmt,*v))
def get(a,fmt):return struct.unpack('<'+fmt,u.mem_read(a,struct.calcsize('<'+fmt)))
def bits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def ret(v=None):
 if v is not None:u.reg_write(UC_X86_REG_RAX,v)
 sp=u.reg_read(UC_X86_REG_RSP);ra=get(sp,'Q')[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ra)
def hook(uc,a,size,user):
 if a==0x1408ccc90:
  origin=get(uc.reg_read(UC_X86_REG_R8),'6I');assert origin[3:]==(0x80000000,bits(-1),0x80000000)
  assert uc.reg_read(UC_X86_REG_XMM3)&0xffffffff==bits(1.)
  assert origin[:3]==(bits(1.25),bits(struct.unpack('<f',struct.pack('<I',state['y']))[0]-.5),bits(-2.))
  out=uc.reg_read(UC_X86_REG_RCX);put(out+0x30,'I',0 if state['hit'] else 2);put(out+0x1c,'I',state['hit_y']);ret()
 elif a==0x1409ea800:state['rock_calls']+=1;ret(state['rock'])
 elif a==0x140a14df0:
  assert uc.reg_read(UC_X86_REG_XMM3)&0xffffffff==bits(130.)
  assert get(uc.reg_read(UC_X86_REG_RSP)+0x28,'I')[0]==bits(130.)
  state['water_calls']+=1;uc.reg_write(UC_X86_REG_XMM0,state['water']);ret()
 elif a==0x141246fb0:state['terrain_calls']+=1;uc.reg_write(UC_X86_REG_XMM0,state['h']);ret()
u.hook_add(UC_HOOK_CODE,hook)
inputs=[]
for loc,hit,rock,water,winter,oldflag,y,h,oldh,dt,idle in itertools.product([0,1,2,3,4,5,6,7,255],[0,1],[0,1],[-1.,0.,1.],[0,1],[0,1],[0.,2.],[-1.,2.],[0.],[0.,.016],[0,1]):
 inputs.append([loc,bits(-3.),oldflag,bits(oldh),idle,bits(.7),bits(y),hit,bits(1.5),rock,bits(water),winter,bits(h),bits(dt)])
for loc in [0,1,2,4,6]:
 for col in [3,5,6,8,10,12,13]:
  for hit in [0,1]:
   row=[loc,bits(-3.),1,bits(.2),0,bits(.7),bits(.1),hit,bits(1.5),0,bits(0.),0,bits(0.),bits(.016)];row[col]=0x7fc01234;inputs.append(row)
rows=[]
for row in inputs:
 loc,cache,flag,oldh,idle,timeout,y,hit,hity,rock,water,winter,h,dt=row
 u.mem_write(data,b'\0'*0x10000);u.mem_write(stack,b'\0'*0x10000)
 put(data+0x10,'III',bits(1.25),y,bits(-2.));put(data+0x100,'II',loc,cache);put(data+0x200,'II',flag,oldh);put(data+0x230,'BI',idle,0);put(data+0x234,'I',timeout);put(data+0x328,'I',dt);put(data+0x500,'B',4 if winter else 0)
 for off,p in [(0x40,data+0x400),(0x70,data+0x404),(0x80,data+0x600),(0xa8,data+0x500),(0xb0,data+0x300)]:put(rsp+off,'Q',p)
 put(rsp+0x34,'I',123)
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RBX,data),(UC_X86_REG_RBP,123),(UC_X86_REG_R13,data+0x100),(UC_X86_REG_R14,data+0x200),(UC_X86_REG_XMM6,bits(1.)),(UC_X86_REG_XMM7,bits(130.)),(UC_X86_REG_XMM8,bits(-.5)),(UC_X86_REG_XMM9,int.from_bytes(struct.pack('<2f',-0.,-1.),'little')),(UC_X86_REG_XMM10,0),(UC_X86_REG_XMM11,bits(.5)),(UC_X86_REG_XMM12,int.from_bytes(struct.pack('<4I',*[0x7fffffff]*4),'little')),(UC_X86_REG_XMM13,bits(10.))]:u.reg_write(reg,val)
 state.update(y=y,hit=hit,hit_y=hity,rock=rock,water=water,h=h,rock_calls=0,water_calls=0,terrain_calls=0)
 u.emu_start(0x142324bf1,0x142324995,count=2000)
 rows.append((row,[*get(data+0x100,'II'),*get(data+0x200,'II'),get(data+0x234,'I')[0],state['rock_calls'],state['water_calls'],state['terrain_calls']]))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-coaster-sheep-unstuck.ron';fixture.write_text('[\n'+''.join('('+str(a).replace(' ','')+','+str(b).replace(' ','')+'),\n' for a,b in rows)+']\n')
out=dict(binary_sha256=sha,cases=len(rows),entry='0x142324bf1',exit='0x142324995',fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),columns=['input(loc,cache,ground_flag,ground_h,move_tag,timeout,y,sphere_hit,sphere_y,rock,water,winter,h,dt) float columns as u32 bits','output(loc,cache,ground_flag,ground_h,timeout,rock_calls,water_calls,terrain_calls)'],bridges=['legacy_spherecast returns supplied hit and hit.position.y; original ray origin+direction+radius asserted','rock/water/terrain samples supplied; original lazy call order/count retained','original winter helper140ae8320 executes on supplied theme byte; no save-state approximation'],scope='完整每羊判断、8B状态payload、ground记录清除/保存、Idle计时器；会话事件/Local预热门另以实际ECS测试，不假称此夹具覆盖')
Path(__file__).with_name('toolbar-coaster-sheep-unstuck-native.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n');print(json.dumps(out,ensure_ascii=False))
