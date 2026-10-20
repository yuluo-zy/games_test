from pathlib import Path
import pefile,struct,json,hashlib
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
root=Path('D:/game');ev=root/'reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-layout';p=root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe';data=p.read_bytes();assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=data,fast_load=True);image=pe.get_memory_mapped_image();ib=pe.OPTIONAL_HEADER.ImageBase;u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(ib,(len(image)+4095)&~4095);u.mem_write(ib,image);u.mem_map(0x100000,0x30000)
records=[]
for align in range(4):
 for w,h in [(0,0),(1,1),(320,480),(480,320),(1280,720),(1920,1080),(2560,1440),(3840,2160),(65535,4096),(0x7fffffff,0xffffffff),(0xffffffff,0x7fffffff)]:
  for icon in [0.,-0.,.001,.1,.3,.5,.75,1.,1.3333334,2.,4.,1000.,-1.,float('inf'),float('-inf'),float('nan')]:
   u.mem_write(0x1000e8,struct.pack('<f',icon));u.mem_write(0x1000f0,bytes([align]));u.mem_write(0x101000,struct.pack('<II',w,h))
   for reg,val in [(UC_X86_REG_R10,0x100000),(UC_X86_REG_R11,0x101000),(UC_X86_REG_R9,0x102000),(UC_X86_REG_R8,0x103000),(UC_X86_REG_RCX,0x104000),(UC_X86_REG_EAX,42)]:u.reg_write(reg,val)
   u.emu_start(0x140ff0052,0x140ff00be,count=80)
   expected=bytes(u.mem_read(0x102000,4));assert bytes(u.mem_read(0x10305c,4))==struct.pack('<I',42)
   records.append(struct.pack('<IIIf',align,w,h,icon)+expected)
blob=b''.join(records);(ev/'size-mult-native-cases.bin').write_bytes(blob)
src=root/'tiny-glade/crates/systems/toolbar/src/toolbar_update.rs';text=src.read_text(encoding='utf-8');a=text.index('pub fn toolbar_size_multiplier(');b=text.index('/// B original inline system.',a)
h=text[a:b]+'''fn main(){
 let bytes=include_bytes!("size-mult-native-cases.bin");
 for (i,r) in bytes.chunks_exact(20).enumerate(){let q=|o|u32::from_le_bytes(r[o..o+4].try_into().unwrap());
 let actual=toolbar_size_multiplier(q(0) as u8,[q(4),q(8)],f32::from_bits(q(12)));
 assert_eq!(actual.to_bits(),q(16),"original multiplier case {i}");}
 println!("{} original multiplier cases passed by exact f32 bits",bytes.len()/20);
}
''';(ev/'actual-source-size-mult.rs').write_text(h,encoding='utf-8')
meta={'binary_sha256':hashlib.sha256(data).hexdigest(),'original_wrapper':'0x140feff70 / 673B','arithmetic_range':['0x140ff0052','0x140ff00be'],'cases':len(records),'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'fixtures_sha256':hashlib.sha256(blob).hexdigest(),'native_change_tick':42};(ev/'size-mult-manifest.json').write_text(json.dumps(meta,indent=2));print(meta)
