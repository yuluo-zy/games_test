from pathlib import Path
import hashlib,itertools,json,struct,pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RDX,UC_X86_REG_XMM0,UC_X86_REG_RBP,UC_X86_REG_RAX,UC_X86_REG_RIP
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=raw,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase;image=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(image)+4095)&~4095);u.mem_write(base,image);u.mem_map(0x30000000,0x10000);bp=0x30008000;u.reg_write(UC_X86_REG_RBP,bp)
def bits(f):return struct.unpack('<I',struct.pack('<f',f))[0]
cases={0,0x80000000,bits(-1.),bits(1.),0x7f800000,0xff800000,0x7fc01234,0xffc01234}
for t in [.345,.46,.913,.926,.48,.84,.35,.94,.98,.83,.5]:
 b=bits(t);cases.update([b-1,b,b+1])
rows=[]
for glade,b in itertools.product(range(5),sorted(cases)):
 u.reg_write(UC_X86_REG_RDX,glade);u.reg_write(UC_X86_REG_XMM0,b);u.emu_start(0x140245ca8,0x140245dea,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x140245dea
 ptr=struct.unpack('<Q',u.mem_read(bp+0x320,8))[0];n=u.reg_read(UC_X86_REG_RAX);name=bytes(u.mem_read(ptr,n)).decode();rows.append((glade,b,name))
p=root/'tiny-glade/reports/business/2026-10-23-module-recovery';f=root/'tiny-glade/tests/fixtures/native/toolbar-settings-background.ron';f.write_text('[\n'+''.join(f'({g},{b},"{name}"),\n' for g,b,name in rows)+']\n')
meta={'binary_sha256':hashlib.sha256(raw).hexdigest(),'entry':'0x140245ca8','stop':'0x140245dea','non_contiguous_branch':'0x140246d36..0x140246d85','bridges':[],'cases':len(rows),'fixture':str(f.relative_to(root/'tiny-glade')),'fixture_sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'scope':'5原Glade判别×各阈值前一位/原位/后一位和NaN/Inf/负数。只执行原背景选择指令，不验证图片资源/GUI建树。'}
(p/'toolbar-settings-background-native.json').write_text(json.dumps(meta,ensure_ascii=False,indent=2)+'\n');print(meta)
