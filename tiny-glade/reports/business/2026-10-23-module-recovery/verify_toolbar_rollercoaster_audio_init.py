"""完整执行原CoasterState构造与其private音频构造/三池collectors；桥接分配与音频后端。"""
from pathlib import Path
import hashlib,json,struct,math
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image)
data,stack,heap,stop=0x20000000,0x30000000,0x40000000,0x2000ff00
u.mem_map(data,0x10000);u.mem_map(stack,0x10000);u.mem_map(heap,0x1000000)
f32=lambda n:struct.unpack('<f',struct.pack('<f',n))[0]
bits=lambda n:struct.unpack('<I',struct.pack('<f',n))[0]
def xmmf(reg):return struct.unpack('<f',struct.pack('<I',u.reg_read(reg)&0xffffffff))[0]
state={}
def ret(value=None):
 if value is not None:u.reg_write(UC_X86_REG_RAX,value)
 sp=u.reg_read(UC_X86_REG_RSP);ra=struct.unpack('<Q',u.mem_read(sp,8))[0];u.reg_write(UC_X86_REG_RSP,sp+8);u.reg_write(UC_X86_REG_RIP,ra)
def alloc(size,align):
 state['top']=(state['top']+align-1)&~(align-1);ptr=state['top'];state['top']+=max(size,1);assert state['top']<heap+0x1000000;return ptr

def hook(uc,address,size,user):
 if address==0x142923e60:
  uc.reg_write(UC_X86_REG_XMM0,bits(float(math.ceil(xmmf(UC_X86_REG_XMM0)))));ret();return
 if address==0x142923f70:
  uc.reg_write(UC_X86_REG_XMM0,bits(math.pow(xmmf(UC_X86_REG_XMM0),xmmf(UC_X86_REG_XMM1))));ret();return
 if address==0x140c946a0:
  t=xmmf(UC_X86_REG_XMM2);uc.mem_write(uc.reg_read(UC_X86_REG_RCX),struct.pack('<3f',f32(t*3),0,0));ret();return
 if address in [0x140613c00,0x140613c10,0x140613c40]:
  ret(alloc(uc.reg_read(UC_X86_REG_RCX),uc.reg_read(UC_X86_REG_RDX)));return
 if address==0x140613c20:ret();return
 if address==0x140613c30:
  old,oldsize,align,newsize=[uc.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8,UC_X86_REG_R9]];ptr=alloc(newsize,align);uc.mem_write(ptr,bytes(uc.mem_read(old,min(oldsize,newsize))));ret(ptr);return
 if address==0x142923c00:
  dst,src,n=[uc.reg_read(r) for r in [UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_R8]];uc.mem_write(dst,bytes(uc.mem_read(src,n)));ret(dst);return
 if address==0x140a55f40:
  out=uc.reg_read(UC_X86_REG_RCX);name=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_R8),uc.reg_read(UC_X86_REG_R9))).decode();state['calls'].append(['create',name]);uc.mem_write(out,b'\0'*48);ret();return
 if address==0x140a575a0:
  state['calls'].append(['volume',xmmf(UC_X86_REG_XMM1)]);ret();return
 if address==0x140a57670:
  name=bytes(uc.mem_read(uc.reg_read(UC_X86_REG_RDX),uc.reg_read(UC_X86_REG_R8))).decode();state['calls'].append(['parameter',name,xmmf(UC_X86_REG_XMM3)]);ret();return
 if address==0x140a57300:state['calls'].append(['start']);ret();return
u.hook_add(UC_HOOK_CODE,hook)
rows=[]
for cart_count,selected in [(1,0),(3,1),(12,0)]:
 state.update(top=heap,calls=[]);u.mem_write(data,b'\0'*0x10000);u.mem_write(stack,b'\0'*0x10000)
 u.mem_write(data+0x11c,struct.pack('<f',2.))
 u.mem_write(data+0x200,struct.pack('<QQQ',cart_count,data+0x1000,cart_count))
 for i in range(cart_count):
  cart=data+0x1000+0x1b0*i;u.mem_write(cart+0x10,struct.pack('<3f',i*3.,0,0))
  for off in [0x30,0x68,0xa0,0xd8]:u.mem_write(cart+off,struct.pack('<Q',2))
  for off in [0x110,0x128,0x140]:u.mem_write(cart+off,struct.pack('<Q',0x8000000000000000))
 rsp=stack+0xf008;u.mem_write(rsp,struct.pack('<Q',stop));u.mem_write(rsp+0x28,struct.pack('<Q',data+0x800))
 for reg,value in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x100),(UC_X86_REG_R8,data+0x200),(UC_X86_REG_R9,selected)]:u.reg_write(reg,value)
 try:u.emu_start(0x1403b6b40,stop,count=1000000)
 except Exception as e:print('RIP',hex(u.reg_read(UC_X86_REG_RIP)),repr(e));raise
 assert u.reg_read(UC_X86_REG_RIP)==stop
 carts=[]
 for i in range(cart_count):
  cart=data+0x1000+0x1b0*i
  entries=[struct.unpack('<Q',u.mem_read(cart+off,8))[0]!=2 for off in [0x30,0x68,0xa0,0xd8]]
  pools=[]
  for off in [0x110,0x128,0x140]:
   cap,ptr,n=struct.unpack('<QQQ',u.mem_read(cart+off,24));pools.append(None if cap==0x8000000000000000 else n)
  carts.append(dict(entries=entries,pools=pools))
 rows.append(dict(cart_count=cart_count,selected=selected,carts=carts,calls=state['calls']))
report=dict(binary_sha256=sha,entry='0x1403b6b40',cases=len(rows),rows=rows,bridges=['Spline3D.position_at_t resolved linear P(t)=(3t,0,0)','ceilf,powf use host math rounded f32; no original CRT equivalence claimed','alloc/zeroed/realloc/dealloc owned bump heap preserving copies','AudioCore.instantiate returns actual Disabled variant; AudioInstance volume/parameter/start backend interface calls recorded, not FMOD'],scope='完整原构造器与privateCoasterAudioInstance::new、三个Vec collectors执行；覆盖选择车、邻车、低于音量阈值的远车。非完整ride、FMOD或OS输入验收。')
Path(__file__).with_name('toolbar-rollercoaster-audio-init-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
print(json.dumps([dict(cart_count=r['cart_count'],selected=r['selected'],carts=r['carts'],calls=len(r['calls'])) for r in rows],ensure_ascii=False))

fixture_rows=[]
for row in rows:
 calls=[]
 for call in row['calls']:
  if call[0]=='create':calls.append(f'(0,{json.dumps(call[1])},0)')
  elif call[0]=='volume':calls.append(f'(1,"",{bits(call[1])})')
  elif call[0]=='parameter':calls.append(f'(2,{json.dumps(call[1])},{bits(call[2])})')
  else:calls.append('(3,"",0)')
 carts=[]
 for cart in row['carts']:
  entries=','.join(str(x).lower() for x in cart['entries'])
  pools=','.join('None' if x is None else f'Some({x})' for x in cart['pools'])
  carts.append(f'(({entries}),({pools}))')
 fixture_rows.append(f'({row["cart_count"]},{row["selected"]},[{",".join(carts)}],[{",".join(calls)}])')
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-rollercoaster-audio-init.ron'
fixture.write_text('[\n'+',\n'.join(fixture_rows)+'\n]\n')
report['fixture_sha256']=hashlib.sha256(fixture.read_bytes()).hexdigest()
Path(__file__).with_name('toolbar-rollercoaster-audio-init-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
