from pathlib import Path
import pefile,struct,json
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
r=Path('D:/game');ev=r/'reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-layout';pe=pefile.PE(str(r/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'),fast_load=True);im=pe.get_memory_mapped_image();u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(pe.OPTIONAL_HEADER.ImageBase,(len(im)+4095)&~4095);u.mem_write(pe.OPTIONAL_HEADER.ImageBase,im);u.mem_map(0x100000,0x20000);out=[]
for k in range(13):
 u.mem_write(0x100210,struct.pack('<Q',0x110000));u.mem_write(0x110000,bytes([2,k])+bytes(62));u.reg_write(UC_X86_REG_RBP,0x100000);u.reg_write(UC_X86_REG_RSP,0x10fff8);u.emu_start(0x141e4e817,0x141e4e85b,count=100);out.append(u.reg_read(UC_X86_REG_XMM0)&0xffffffff)
(ev/'native-slider-initial-values.json').write_text(json.dumps(out));s=(r/'tiny-glade/crates/systems/toolbar/src/resouces_and_components/toolbar_layout.rs').read_text(encoding='utf-8');a=s.index('pub fn initial_slider_value_01(');b=s.index('/// Original Style',a);math=(r/'tiny-glade/crates/systems/toolbar/src/photomode/slider_math.rs').read_text(encoding='utf-8');math='\n'.join(x for x in math.splitlines() if not x.startswith('//!'));math=math[:math.index('#[cfg(test)]')] if '#[cfg(test)]' in math else math
h='mod photomode { pub mod slider_math { '+math+'}}\n'+s[a:b]+'fn main(){ let expected:[u32;13]='+str(out)+'; for (k,b) in expected.into_iter().enumerate(){ assert_eq!(initial_slider_value_01(k as u8).to_bits(),b); } println!("13/13 actual-source slider initial values match original f32 bits"); }';(ev/'actual-source-slider-initial.rs').write_text(h,encoding='utf-8');print(out)
