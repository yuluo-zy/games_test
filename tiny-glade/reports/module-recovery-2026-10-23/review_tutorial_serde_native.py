"""原TutorialCompletionState完整serializer；仅桥接锁定RON的结构/字段/end输出边界。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes();digest=hashlib.sha256(raw).hexdigest();assert digest=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im)
data,stack,stop=0x20000000,0x30000000,0x2000f000;u.mem_map(data,0x10000);u.mem_map(stack,0x10000);fields=[]
def qw(addr,v):u.mem_write(addr,struct.pack('<Q',v))
def readname():return bytes(u.mem_read(u.reg_read(UC_X86_REG_R8),u.reg_read(UC_X86_REG_R9))).decode()
def hook(uc,addr,size,ud):
 if addr in [0x1413132c0,0x141311e70,0x141312a40]:
  rsp=u.reg_read(UC_X86_REG_RSP);out=u.reg_read(UC_X86_REG_RCX)
  if addr==0x1413132c0:
   assert readname()=='TutorialCompletionState';assert struct.unpack('<Q',u.mem_read(rsp+0x28,8))[0]==19
  if addr==0x141311e70:
   valueptr=struct.unpack('<Q',u.mem_read(rsp+0x28,8))[0];fields.append((readname(),bool(u.mem_read(valueptr,1)[0])))
  u.mem_write(out,bytes([0x32])+bytes(71));ret=struct.unpack('<Q',u.mem_read(rsp,8))[0];u.reg_write(UC_X86_REG_RSP,rsp+8);u.reg_write(UC_X86_REG_RIP,ret)
u.hook_add(UC_HOOK_CODE,hook)
def run(mask):
 fields.clear();u.mem_write(data,bytes(0xe000));u.mem_write(data+0x1000,bytes((mask>>i)&1 for i in range(19)));u.mem_write(stack,bytes(0x10000));rsp=stack+0xf008;qw(rsp,stop)
 for reg,val in [(UC_X86_REG_RSP,rsp),(UC_X86_REG_RCX,data),(UC_X86_REG_RDX,data+0x1000),(UC_X86_REG_R8,data+0x2000)]:u.reg_write(reg,val)
 u.emu_start(0x140a65fe0,stop,count=10000);assert u.reg_read(UC_X86_REG_RIP)==stop and len(fields)==19
 return list(fields)
if __name__=='__main__':
 masks=[0,(1<<19)-1,0x55555,0x2aaaa]+[1<<i for i in range(19)];seed=0x12345678
 for _ in range(256):seed=(seed*1664525+1013904223)&0xffffffff;masks.append(seed&((1<<19)-1))
 rows=[(mask,run(mask)) for mask in masks]
 fixture=root/'tiny-glade/tests/fixtures/native/tutorial-completion-serde.ron';fixture.write_text('[\n'+''.join('('+str(mask)+',['+','.join('('+json.dumps(name)+','+str(v).lower()+')' for name,v in values)+']),\n' for mask,values in rows)+']\n')
 report={'binary_sha256':digest,'function':'0x140a65fe0','normal_return':'0x140a664b9','whole_function':True,'bridges':[{'address':hex(a),'boundary':'locked RON SerializeStruct/serialize_field<bool>/end sink'} for a in [0x1413132c0,0x141311e70,0x141312a40]],'cases':len(rows),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'scope':'原私有19字段serializer完整成功路径执行；0/全1/交错/19one-hot/256确定性伪随机mask。比较原字段名称、顺序和布尔偏移投影；不宣称RON内部格式化差分或错误分配路径恢复。'}
 (root/'tiny-glade/reports/business/2026-10-23-module-recovery/tutorial-completion-serde-native.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n');print(len(rows),'cases')
