"""执行原浮点Tweaker全体指令；仅roundf依赖桥及panic边界外置。"""
from pathlib import Path
import ctypes,hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
ROOT=Path(__file__).resolve().parents[4]
raw=(ROOT/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(len(image)+4095)&~4095);cpu.mem_write(base,image)
data,stack,stop=0x20000000,0x30000000,0x20002000
cpu.mem_map(data,0x3000);cpu.mem_map(stack,0x10000)
libm=ctypes.CDLL('/usr/lib/libSystem.B.dylib');libm.roundf.argtypes=[ctypes.c_float];libm.roundf.restype=ctypes.c_float
round_address=0x142923fa0;panic_address=0x140d8b480
cpu.mem_write(round_address,b'\xc3');panicked=False
bits=lambda x:struct.unpack('<I',struct.pack('<f',x))[0]
def hook(uc,address,size,_):
    global panicked
    if address==round_address:
        x=struct.unpack('<f',uc.reg_read(UC_X86_REG_XMM0).to_bytes(16,'little')[:4])[0]
        uc.reg_write(UC_X86_REG_XMM0,int.from_bytes(struct.pack('<f',libm.roundf(x)),'little'))
    elif address==panic_address:
        panicked=True;uc.emu_stop()
cpu.hook_add(UC_HOOK_CODE,hook,begin=round_address,end=round_address)
cpu.hook_add(UC_HOOK_CODE,hook,begin=panic_address,end=panic_address)
rows=[]
for increase,entry in [(False,0x14067da10),(True,0x14067da90)]:
 for value in [bits(x) for x in [-0.15,-0.,0.,0.05,0.45,0.95,1.05,float('inf')]]+[0x7fc00042]:
  for step in [0.1,0.25,-0.1,0.]:
   for minimum,maximum in [(-1.,1.),(0.,1.),(1.,-1.)]:
    panicked=False;before=[value,bits(step),bits(minimum),bits(maximum)]
    cpu.mem_write(data,struct.pack('<Q3I',data+0x100,*before[1:]))
    cpu.mem_write(data+0x100,struct.pack('<I',value));rsp=stack+0xf008;cpu.mem_write(rsp,struct.pack('<Q',stop))
    cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RSP,rsp);cpu.reg_write(UC_X86_REG_MXCSR,0x1f80)
    cpu.emu_start(entry,stop,count=200)
    assert panicked or cpu.reg_read(UC_X86_REG_RIP)==stop
    result=struct.unpack('<I',cpu.mem_read(data+0x100,4))[0]
    rows.append((increase,tuple(before),result,panicked))
out=ROOT/'tiny-glade/tests/fixtures/native/toolbar-tweaker-float.ron'
out.write_text('[\n'+''.join('  (increase: '+str(a).lower()+', before: '+str(b)+', after: '+str(c)+', panicked: '+str(d).lower()+'),\n' for a,b,c,d in rows)+']\n')
Path(__file__).with_name('toolbar-tweaker-float-native.json').write_text(json.dumps({'binary_sha256':sha,'functions':['0x14067da10','0x14067da90'],
 'scope':'原加减/除法/乘法/限幅/写入与panic门控均执行；roundf桥接libSystem；panic格式化/展开不执行。有限结果比较f32 bits；NaN比较类别（跨ISA无效运算符号/载荷不属此证明）。',
 'cases':len(rows),'fixture_sha256':hashlib.sha256(out.read_bytes()).hexdigest()},ensure_ascii=False,indent=2)+'\n')
print('native cases',len(rows))
