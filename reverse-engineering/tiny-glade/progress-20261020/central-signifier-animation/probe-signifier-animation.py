from pathlib import Path
import hashlib,json,struct,itertools
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path('D:/game');out=root/'reverse-engineering/tiny-glade/progress-20261020/central-signifier-animation'
out.mkdir(exist_ok=True)
binary=root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
pe=pefile.PE(str(binary),fast_load=True);im=pe.get_memory_mapped_image()
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(pe.OPTIONAL_HEADER.ImageBase,(len(im)+4095)&~4095);u.mem_write(pe.OPTIONAL_HEADER.ImageBase,im);u.mem_map(0x100000,0x50000)
durations=[0,0x80000000,0x3f800000,0x3e99999a,0xc0000000,0x7fc12345,0x7f800000,1]
progress=[0,0x80000000,0x3eaaaaab,0x3f800000,0x40000000,0x7fc01234]
deltas=[0,0x80000000,0x3c83126f,0xbf800000,0x7fc05678,0x7f800000]
records=bytearray()
for tag,duration,t,delta,marked in itertools.product([0,1,6],durations,progress,deltas,[0,1]):
    u.mem_write(0x110000,struct.pack('<IIBB2xfIII',tag,0,0,1,0.3,duration,t,0))
    u.mem_write(0x1000f0,struct.pack('<Q',0x120000))
    u.mem_write(0x120028,struct.pack('<I',delta))
    u.reg_write(UC_X86_REG_RBP,0x100100);u.mem_write(0x100140,struct.pack('<Q',0x130000));u.mem_write(0x1000f0,struct.pack('<Q',0x120000))
    u.reg_write(UC_X86_REG_RSI,0x110000);u.reg_write(UC_X86_REG_RAX,marked)
    u.reg_write(UC_X86_REG_RSP,0x140000);u.reg_write(UC_X86_REG_XMM6,0);u.reg_write(UC_X86_REG_XMM7,0x3f800000);u.reg_write(UC_X86_REG_XMM8,0x80000000)
    u.emu_start(0x140a69998,0x140a69a17,count=1000)
    result=bytes(u.mem_read(0x110014,8))
    records+=struct.pack('<IIIII',tag,duration,t,delta,marked)+result
(out/'native-animation.bin').write_bytes(records)
(out/'manifest.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'start':'140a69998','stop':'140a69a17','cases':len(records)//28,'hooks':[],'easing_tags':[0,1,6],'remaining_easing_tags':'reuse canonical utils implementation; not covered by this native execution batch'},indent=2))
print('Original animation cases',len(records)//28,'with actual native easing, no hooks')
