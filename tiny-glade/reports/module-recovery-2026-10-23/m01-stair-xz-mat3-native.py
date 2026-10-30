"""B utils140c90ed0完整Mat3构造；无外部桥。"""
from pathlib import Path
import struct,json,hashlib,pefile,itertools
from unicorn import *
from unicorn.x86_const import *
root=Path('/Users/yuluo/Desktop/game');raw=(root/'B版本内容/tiny-glade.rne').read_bytes();sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase;im=p.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(im)+4095)&~4095);u.mem_write(base,im);u.mem_map(0x100000,0x200000);u.mem_map(0x400000,0x100000)
values=[0,0x80000000,0x3f800000,0xbf800000,0x3f123456,0xc0265432,0x00000001,0x007fffff,0x00800000,0x7f7fffff,0x7f800000,0xff800000,0x7fc12345];rows=[]
for a,b in itertools.product(values,repeat=2):
 sp=0x1ff008;u.mem_write(sp,struct.pack('<Q',0x100000));u.reg_write(UC_X86_REG_RSP,sp);u.reg_write(UC_X86_REG_RCX,0x401000);u.reg_write(UC_X86_REG_XMM1,a);u.reg_write(UC_X86_REG_XMM2,b);u.emu_start(0x140c90ed0,0x100000,count=1000);assert u.reg_read(UC_X86_REG_RIP)==0x100000;rows.append([a,b,list(struct.unpack('<9I',u.mem_read(0x401000,36)))])
Path('/tmp/m01-stair-xz-mat3-native.json').write_text(json.dumps(dict(original_sha256=sha,scope='full253B140c90ed0; no bridges',rows=rows),indent=2));print('cases',len(rows))
