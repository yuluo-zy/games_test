"""原render_hints Graphic完整正常路径；桥仅libm floorf与最终MEI图片输出边界。"""
from pathlib import Path
import hashlib,json,struct,math,itertools,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest();assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack,stop=0x20000000,0x30000000,0x2000f000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000)
def qw(addr,v):u.mem_write(addr,struct.pack('<Q',v))
def ret():
 rsp=u.reg_read(UC_X86_REG_RSP);address=struct.unpack('<Q',u.mem_read(rsp,8))[0];u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,address)
output=None
def hook(uc,addr,size,ud):
 global output
 if addr==0x142923ee0:
  bits=uc.reg_read(UC_X86_REG_XMM0)&0xffffffff;x=struct.unpack('<f',struct.pack('<I',bits))[0]
  if math.isfinite(x):bits=struct.unpack('<I',struct.pack('<f',float(math.floor(x))))[0]
  uc.reg_write(UC_X86_REG_XMM0,bits);ret()
 elif addr==0x14228a4e0:
  rsp=uc.reg_read(UC_X86_REG_RSP);p,n=struct.unpack('<QQ',uc.mem_read(rsp+0x28,16));height=struct.unpack('<I',uc.mem_read(rsp+0x38,4))[0]
  output=(int(bytes(uc.mem_read(p,n)).decode()[1:]),height);ret()
u.hook_add(UC_HOOK_CODE,hook)
def run(length,period,time,height,static=False):
 global output
 output=None;u.mem_write(data,bytes(0xe000));u.mem_write(stack,bytes(0x10000));u.mem_write(data,struct.pack('<I',0xc5));u.mem_write(data+40,struct.pack('<I',height))
 for i in range(length):
  text=f'f{i}'.encode();u.mem_write(data+0x4000+i*16,text);qw(data+0x3000+i*16,data+0x4000+i*16);qw(data+0x3008+i*16,len(text))
 if static:
  qw(data+8,1<<63);qw(data+16,1<<63);qw(data+24,data+0x4000);qw(data+32,2)
 else:
  qw(data+8,length);qw(data+16,data+0x3000);qw(data+24,length);u.mem_write(data+32,struct.pack('<I',period))
 u.mem_write(data+0x1008,struct.pack('<I',time))
 rsp=stack+0xf008;qw(rsp,stop)
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x1000),(UC_X86_REG_R8,data+0x2000),(UC_X86_REG_R9,0)]:u.reg_write(reg,val)
 u.emu_start(0x141c52700,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop and output is not None
 return output
if __name__=='__main__':
 rows=[];periods=[0,0x3f333333,0xbf333333,0x7f800000,0x7fc00000,1];times=[0,0x80000000,0x3f333332,0x3f333333,0x3f333334,0x3fb33333,0x40066666,0xbf800000,0x7f800000,0xff800000,0x7fc00000,0x7f7fffff]
 for length,period,time,height in itertools.product(range(1,10),periods,times,[0x42800000,0x42c80000]):
  index,returned_height=run(length,period,time,height);rows.append((False,length,period,time,height,index,returned_height))
 for time,height in itertools.product(times,[0x42800000,0x42c80000]):
  index,returned_height=run(1,0,time,height,True);rows.append((True,1,0,time,height,index,returned_height))
 fixture=root/'tiny-glade/tests/fixtures/native/hint-graphic-frames.ron';fixture.write_text('[\n'+''.join('('+','.join(str(v).lower() for v in row)+'),\n' for row in rows)+']\n')
 report={'binary_sha256':digest,'entry':'0x141c52700','normal_return':'0x141c5309c','branch':'Graphic only','bridges':[{'address':'0x142923ee0','boundary':'libm floorf; no business bypass'},{'address':'0x14228a4e0','boundary':'final MEI image sink; capture selected image and height'}],'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'静态图及帧数1..9、period0/±0.7/Inf/NaN/subnormal、time边界/负/±Inf/NaN/MAX；比较原selected frame和heightbits，未声明全部MEI/OS/GPU差分。空动画保留原rem-by-zero panic另测。'}
 (root/'tiny-glade/reports/business/2026-10-23-module-recovery/hint-graphic-frames-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(len(rows),'cases')
