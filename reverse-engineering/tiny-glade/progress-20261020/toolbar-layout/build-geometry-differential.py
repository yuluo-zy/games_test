from pathlib import Path
import struct,json,hashlib,subprocess
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path('D:/game');ev=root/'reverse-engineering/tiny-glade/progress-20261020/toolbar-layout';binary=root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
pe=pefile.PE(str(binary));image=pe.get_memory_mapped_image();ib=pe.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(ib,(len(image)+4095)&~4095);u.mem_write(ib,image);u.mem_map(0x100000,0x30000)
def xmm(n,x):u.reg_write(UC_X86_REG_XMM0+n,int.from_bytes(struct.pack('<4f',*(x+[0.]*(4-len(x)))),'little'))
def out(n):return u.reg_read(UC_X86_REG_XMM0+n).to_bytes(16,'little')
def read(va,n):return list(struct.unpack('<'+'f'*n,u.mem_read(va,n*4)))
constants={6:0x14304e2fc,7:0x14295a0e4,8:0x1429345a8,9:0x1429258d8,12:0x14295a0e8,11:0x142925870,14:0x14304e310,15:0x14304e300}
records=[]
for alignment in range(4):
 for count in [0,1,2,6,12,35,123,4095]:
  for scale in [0.,-0.,0.001,0.1,0.3,0.75,1.,1.3333334,2.5,1000.]:
   for w,h in [(0.,0.),(1.,1.),(1000.,600.),(1920.,1080.),(65535.,4096.)]:
    u.reg_write(UC_X86_REG_RSP,0x120000);u.reg_write(UC_X86_REG_R13,0x100000);u.mem_write(0x1000f0,bytes([alignment]));u.reg_write(UC_X86_REG_ESI,count)
    u.mem_write(0x120058,struct.pack('<Q',0x110000))
    for k,v in constants.items():xmm(k,read(v,2 if k in(14,15) else 1))
    xmm(13,[scale]);xmm(0,[w]);xmm(2,[h])
    u.emu_start(0x142343f2f,0x142343ffc,count=100)
    expected=out(0)[:4]+out(2)[:4]+out(3)[:8]
    records.append(struct.pack('<Iifff',alignment,count,w,h,scale)+expected)
(ev/'geometry-native-cases.bin').write_bytes(b''.join(records))
src=root/'tiny-glade/crates/systems/toolbar/src/toolbar_systems.rs';text=src.read_text(encoding='utf-8')
start=text.index('pub fn activation_rect(');end=text.index('/// `toolbar_auto_hide` 的可见进度更新',start)
functions=text[start:end]
sa=text.index('pub fn auto_hide_sound_kind(');sb=text.index('\n/// O 0x142343a10',sa)
functions+=text[sa:sb]
harness='use glam::Vec2;\n'+functions+'''
fn main() {
 let bytes=include_bytes!("geometry-native-cases.bin");
 for (index,record) in bytes.chunks_exact(36).enumerate() {
  let get=|n:usize|u32::from_le_bytes(record[n*4..n*4+4].try_into().unwrap());
  let (pos,size)=toolbar_activation_geometry(get(0) as u8,Vec2::new(f32::from_bits(get(2)),f32::from_bits(get(3))),f32::from_bits(get(4)),get(1) as i32);
  let actual=[pos.x.to_bits(),pos.y.to_bits(),size.x.to_bits(),size.y.to_bits()];
  let expected=[get(5),get(6),get(7),get(8)];
  assert_eq!(actual,expected,"original B geometry case {index}");
 }
 for (old,new,expected) in [(0.,0.1,Some(16)),(1.,0.9,Some(17)),(0.,f32::NAN,None),(1.,f32::NAN,None),(f32::NAN,0.,None)] {
  assert_eq!(auto_hide_sound_kind(old,new),expected);
 }
 println!("{} original B geometry cases ({} f32 bit comparisons) + 5 sound boundary cases passed",bytes.len()/36,bytes.len()/36*4);
}
'''
# Actual production source extraction only; no rewritten game formula.
(ev/'actual-source-geometry.rs').write_text(harness,encoding='utf-8')
meta={'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'original_range':['0x142343f2f','0x142343ffc'],'geometry_cases':len(records),'expected_sha256':hashlib.sha256(b''.join(records)).hexdigest(),'production_source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'method':'Unicorn original code range; pinned dependency glam 0.29.3; extracted actual production functions, no rewritten formula'}
(ev/'oracle-manifest.json').write_text(json.dumps(meta,indent=2),encoding='utf-8');print(meta)

