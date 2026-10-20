from pathlib import Path
import hashlib, json, struct, subprocess
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import *

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/progress-20261020/central-audio-preview'
out.mkdir(exist_ok=True)
binary = root / 'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
pe = pefile.PE(str(binary), fast_load=True)
image = pe.get_memory_mapped_image()
u = Uc(UC_ARCH_X86, UC_MODE_64)
u.mem_map(pe.OPTIONAL_HEADER.ImageBase, (len(image)+4095)&~4095)
u.mem_write(pe.OPTIONAL_HEADER.ImageBase, image)
u.mem_map(0x100000, 0x20000)
expected = bytearray()
for ty in range(256):
    u.mem_write(0x110081, bytes([ty]))
    for subtype in range(256):
        u.mem_write(0x100038, struct.pack('<Q', 0x110000))
        u.reg_write(UC_X86_REG_RSP, 0x100000)
        u.reg_write(UC_X86_REG_RSI, 0x110000)
        u.reg_write(UC_X86_REG_RCX, subtype)
        u.emu_start(0x1408233a9, 0x1408235d0, count=100)
        expected += bytes([u.reg_read(UC_X86_REG_RAX)&255,
                           u.reg_read(UC_X86_REG_R8)&255])
(out/'native-preview-kind.bin').write_bytes(expected)
src = root/'tiny-glade/crates/systems/decorator/src/audio/op_preview_audio.rs'
s = src.read_text(encoding='utf-8')
a = s.index('pub fn preview_kind(')
b = s.index('\n///', a)
harness = '#[derive(Clone, Copy)] struct DecoratorType(u8);\n'+s[a:b]+'''
fn main() {
 let expected = include_bytes!("native-preview-kind.bin");
 let mut failures=0;
 for ty in 0..=255u8 { for sub in 0..=255u8 {
   let (kind, flag)=preview_kind(DecoratorType(ty), sub);
   let n=(ty as usize*256+sub as usize)*2;
   if [kind,flag] != expected[n..n+2] {
     if failures<12 { println!("ty={ty:02x} sub={sub} actual={kind:02x}/{flag} original={:02x}/{}",expected[n],expected[n+1]); }
     failures+=1;
   }
 }}
 println!("65536 original instruction comparisons, failures={failures}");
 assert_eq!(failures,0);
}
'''
(out/'actual-source-preview-kind.rs').write_text(harness,encoding='utf-8')
(out/'manifest.json').write_text(json.dumps({
 'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
 'entry':'1408233a9', 'stop':'1408235d0',
 'inputs':'type byte in [RSI+81], subtype in CL; no function hooks',
 'source_sha256': hashlib.sha256(src.read_bytes()).hexdigest(),
 'cases':65536,
},indent=2),encoding='utf-8')
print('Native table captured:', len(expected), 'bytes')
