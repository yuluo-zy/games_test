"""执行原Konami全函数：构造真实单桶just_pressed表和Local，不桥接业务判断。"""
from pathlib import Path
import hashlib,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path(__file__).resolve().parents[4];raw=(root/'B版本内容/tiny-glade.rne').read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image()
cpu=Uc(UC_ARCH_X86,UC_MODE_64);cpu.mem_map(base,(len(image)+4095)&~4095);cpu.mem_write(base,image)
data,stack,stop=0x20000000,0x30000000,0x20003000
cpu.mem_map(data,0x4000);cpu.mem_map(stack,0x10000)
ctrl=data+0x1000;resource=data+0x100;local=data+0x200
cpu.mem_write(data,struct.pack('<Q',resource));cpu.mem_write(resource+0x20,struct.pack('<Q',ctrl));cpu.mem_write(resource+0x38,struct.pack('<Q',1))
cpu.mem_write(ctrl,b'\x00'+b'\xff'*15)
rows=[]
for progress in range(14):
 for key in [0,0x14,0x15,0x3a,0x50,0x51,0x52,0x53,0x28]:
  cpu.mem_write(local,struct.pack('<Q',progress));cpu.mem_write(ctrl-12,struct.pack('<III',key,0,0));cpu.mem_write(0x143634b7c,b'\0')
  rsp=stack+0xf008;cpu.mem_write(rsp,struct.pack('<Q',stop));cpu.reg_write(UC_X86_REG_RSP,rsp);cpu.reg_write(UC_X86_REG_RCX,data);cpu.reg_write(UC_X86_REG_RDX,local)
  cpu.emu_start(0x14024b370,stop,count=400);assert cpu.reg_read(UC_X86_REG_RIP)==stop
  rows.append((progress,key,struct.unpack('<Q',cpu.mem_read(local,8))[0],cpu.mem_read(0x143634b7c,1)[0]!=0))
fixture=root/'tiny-glade/tests/fixtures/native/toolbar-konami.ron'
fixture.write_text('[\n'+''.join(f'  ({p}, {k}, {q}, {str(f).lower()}),\n' for p,k,q,f in rows)+']\n')
Path(__file__).with_name('toolbar-konami-native.json').write_text(json.dumps({'binary_sha256':sha,'function':'0x14024b370','cases':len(rows),'scope':'完整原函数，单桶just_pressed输入；全部11个目标为非零KeyCode判别，匹配不进入Unidentified分支；不证明多键哈希桶跨平台顺序。','fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest()},ensure_ascii=False,indent=2)+'\n')
print('native cases',len(rows))
