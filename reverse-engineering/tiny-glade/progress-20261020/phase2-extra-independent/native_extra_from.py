from pathlib import Path
import pefile,struct,hashlib,json,random
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import *
H=Path(__file__).resolve().parent;R=H.parents[3]
binary=(R/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe').read_bytes();sha=hashlib.sha256(binary).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
p=pefile.PE(data=binary,fast_load=True);img=p.get_memory_mapped_image();base=p.OPTIONAL_HEADER.ImageBase
u=Uc(UC_ARCH_X86,UC_MODE_64);u.mem_map(base,(len(img)+4095)&~4095);u.mem_write(base,img);u.mem_map(0x100000,0x100000)
rng=random.Random(0x839520);cases=[];blob=bytearray()
# Each actual leaf writes into the ExtraInfo field's native placement +0x30.
for kind,fn in [(0,0x140839560),(1,0x140839560),(4,0x140839520),(5,0x140839530),(6,0x140839540),(7,0x140839550)]:
 for i in range(96):
  before=bytes(rng.randrange(256) for _ in range(72));payload=bytearray(rng.randrange(256) for _ in range(20));payload[17]=kind
  if i<8:payload[:4]=struct.pack('<I',[0,0x80000000,0x7fc01234,0xffc01234,0x7f800000,0xff800000,0x3f800000,0xffffffff][i])
  u.mem_write(0x110000,before);u.mem_write(0x120000,bytes(payload));u.mem_write(0x180000,struct.pack('<Q',0x1f0000))
  u.reg_write(UC_X86_REG_RSP,0x180000);u.reg_write(UC_X86_REG_RCX,0x110030)
  u.reg_write(UC_X86_REG_RDX,0x120000 if kind<2 else struct.unpack('<I',payload[:4])[0]);u.reg_write(UC_X86_REG_R8,payload[4]);u.reg_write(UC_X86_REG_XMM1,struct.unpack('<I',payload[:4])[0])
  u.emu_start(fn,0x1f0000,count=10);after=bytes(u.mem_read(0x110000,72));assert u.reg_read(UC_X86_REG_RAX)==0x110030
  blob+=bytes([kind])+before+payload+after
  cases.append({'kind':kind,'from':hex(fn),'before':before.hex(),'input':payload.hex(),'after':after.hex()})
(H/'native-from-fixtures.bin').write_bytes(blob);(H/'native-from-fixtures.json').write_text(json.dumps(cases,indent=2));(H/'manifest.json').write_text(json.dumps({'binary_sha256':sha,'cases':len(cases),'record_stride':165,'hooks':0,'destination_base':'DecoratorInfo+0x30','leaf_discriminant':'ExtraInfo+0x11','full_record_output_sha256':hashlib.sha256(blob).hexdigest()},indent=2));print(len(cases),'native conversions; no hooks')
