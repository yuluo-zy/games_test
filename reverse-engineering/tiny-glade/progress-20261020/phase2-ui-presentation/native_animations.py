from pathlib import Path
import pefile,struct,itertools,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
H=Path(__file__).resolve().parent;R=H.parents[3];binary=(R/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes();p=pefile.PE(data=binary,fast_load=True);img=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(img)+4095)&~4095);u.mem_write(base,img);u.mem_map(0x100000,0x100000)
def q(a,v):u.mem_write(a,struct.pack('<Q',v))
def f(a,v):u.mem_write(a,struct.pack('<f',v))
def x(reg,v):u.reg_write(reg,struct.unpack('<I',struct.pack('<f',v))[0])
focus=bytearray();appear=bytearray();vals=[-1.,-0.,0.,.1,.25,.5,.75,1.,1.25,float('nan'),float('inf')];scales=[-0.,0.,.5,1.,2.,float('nan')];customs=[None,(0,2.),(1,.8),(1,0.),(1,-1.),(1,float('nan')),(1,float('inf'))]
for slider,reset,custom,select,scale in itertools.product((False,True),(False,True),customs,vals,scales):
 f(0x120004,select);f(0x121010,scale);q(0x180030,int(custom is not None));q(0x180038,int(reset));q(0x18003c,int(slider));q(0x180048,0x123000);u.mem_write(0x123000,struct.pack('<If',*(custom or (0,0.))))
 for reg,v in [(UC_X86_REG_RSP,0x180000),(UC_X86_REG_RAX,0x120000),(UC_X86_REG_RCX,0),(UC_X86_REG_RSI,0x121000),(UC_X86_REG_R13,0x122000)]:u.reg_write(reg,v)
 for reg,v in [(UC_X86_REG_XMM6,1.2),(UC_X86_REG_XMM7,1.5),(UC_X86_REG_XMM8,1.)]:x(reg,v)
 u.emu_start(0x14233bf5e,0x14233bbba,count=500)
 focus+=struct.pack('<4B4f',slider,reset,custom is not None,custom[0] if custom else 0,custom[1] if custom else 0.,select,scale,struct.unpack('<f',u.mem_read(0x121010,4))[0])
for value,scale in itertools.product(vals,scales):
 f(0x120000,value);f(0x121010,scale)
 for reg,v in [(UC_X86_REG_RSP,0x180000),(UC_X86_REG_R13,0x120000),(UC_X86_REG_RBX,0x121000),(UC_X86_REG_RBP,0x122000),(UC_X86_REG_RDI,0)]:u.reg_write(reg,v)
 x(UC_X86_REG_XMM6,1.);x(UC_X86_REG_XMM7,0.)
 u.emu_start(0x14233c0d8,0x14233c11e,count=500)
 appear+=struct.pack('<2f',value,scale)+u.mem_read(0x121010,4)
(H/'native-focus.bin').write_bytes(focus);(H/'native-appear.bin').write_bytes(appear);(H/'native-animation-manifest.json').write_text(json.dumps({'focus_cases':len(focus)//20,'focus_stride':20,'appear_cases':len(appear)//12,'appear_stride':12,'hooks':0,'focus_range':['14233bf5e','14233bbba'],'appear_range':['14233c0d8','14233c11e'],'easing':'actual original called with nativeCubicOut tag6','binary_sha256':hashlib.sha256(binary).hexdigest()},indent=2));print(len(focus)//20,'focus;',len(appear)//12,'appear; no hooks')
