"""Unchanged original private xz rotation and box's dimension-to-vertices path.

No emulated game algorithms or byte patches. Stops box before its conversion
of Transform to Mat4 / Vec allocation, exporting the actual24 positions/36
indices which the existing complete box mapping routine consumes.
"""
from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64,UC_HOOK_CODE
from unicorn.x86_const import *
OUT=Path(__file__).parent
raw=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=raw,fast_load=True);base=p.OPTIONAL_HEADER.ImageBase
c=Uc(UC_ARCH_X86,UC_MODE_64);c.mem_map(base,(p.OPTIONAL_HEADER.SizeOfImage+4095)&~4095);c.mem_write(base,p.get_memory_mapped_image())
DATA,STACK=0x20000000,0x30000000;c.mem_map(DATA,0x100000);c.mem_map(STACK,0x100000)
stopped=None
def hook(cpu,pc,size,user):
 global stopped
 if pc in (0x140ada463,DATA+0x100):stopped=pc;cpu.emu_stop()
c.hook_add(UC_HOOK_CODE,hook)
def bits(v):return struct.unpack('<I',struct.pack('<f',v))[0]
def setup():
 global stopped
 stopped=None;c.mem_write(DATA,bytes(0x100000));c.mem_write(STACK,bytes(0x100000))
 for r in (UC_X86_REG_RAX,UC_X86_REG_RBX,UC_X86_REG_RCX,UC_X86_REG_RDX,UC_X86_REG_RSI,UC_X86_REG_RDI,UC_X86_REG_RBP,UC_X86_REG_R8,UC_X86_REG_R9,UC_X86_REG_R10,UC_X86_REG_R11,UC_X86_REG_R12,UC_X86_REG_R13,UC_X86_REG_R14,UC_X86_REG_R15):c.reg_write(r,0)
 for r in range(UC_X86_REG_XMM0,UC_X86_REG_XMM15+1):c.reg_write(r,0)
 c.reg_write(UC_X86_REG_RSP,STACK+0x80000);c.mem_write(STACK+0x80000,struct.pack('<Q',DATA+0x100));c.reg_write(UC_X86_REG_MXCSR,0x1f80);c.reg_write(UC_X86_REG_RFLAGS,0x202)
 c.reg_write(UC_X86_REG_RCX,DATA)
def run_box(dims):
 setup()
 for r,v in zip([UC_X86_REG_XMM1,UC_X86_REG_XMM2,UC_X86_REG_XMM3],dims):c.reg_write(r,bits(v))
 c.emu_start(0x140ada0e0,0,count=10000);assert stopped==0x140ada463
 sp=c.reg_read(UC_X86_REG_RSP)
 return {'input_bits':[bits(v) for v in dims], 'positions_bits':struct.unpack('<72I',c.mem_read(sp+0x170,288)),'indices':struct.unpack('<36Q',c.mem_read(sp+0x50,288))}
def run_xz(x,y):
 setup();c.reg_write(UC_X86_REG_XMM1,bits(x));c.reg_write(UC_X86_REG_XMM2,bits(y))
 c.emu_start(0x140c90fd0,0,count=10000);assert stopped==DATA+0x100
 return {'input_bits':[bits(x),bits(y)],'quaternion_bits':struct.unpack('<4I',c.mem_read(DATA,16))}
report={'source_sha256':hashlib.sha256(raw).hexdigest(),'box_scope':['140ada0e0..140ada463','excluded Transform-to-Mat4 and actual triangle Vec generation'],
 'boxes':[run_box(d) for d in [(2.,4.,6.),(.75,1.,.5),(-2.,4.,-6.),(0.,-0.,0.),(float('nan'),1.,float('inf'))]],
 'xz_scope':'complete140c90fd0/504B using original glam inline normalization+Quat::from_mat3',
 'xz':[run_xz(x,y) for x,y in [(0.,0.),(1.,0.),(0.,1.),(0.,-1.),(-1.,0.),(3.,4.),(-3.,4.),(3.,-4.),(-3.,-4.),(1e30,1e30),(float('nan'),1.),(float('inf'),0.),(1e-40,1e-40)]]}
(OUT/'native-focused-helper-fixtures.json').write_text(json.dumps(report,indent=2),encoding='utf8')
print(json.dumps({'boxes':len(report['boxes']),'xz':len(report['xz']),'first_box':report['boxes'][0],'xz_first3':report['xz'][:3]},indent=2))
