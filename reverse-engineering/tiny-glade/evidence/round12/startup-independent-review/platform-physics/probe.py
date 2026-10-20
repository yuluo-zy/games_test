from pathlib import Path
import pefile,unicorn,struct,hashlib,json
from unicorn.x86_const import *
base=Path(__file__).resolve().parent
src=Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne');raw=src.read_bytes();pe=pefile.PE(data=raw);image_base=pe.OPTIONAL_HEADER.ImageBase
u=unicorn.Uc(unicorn.UC_ARCH_X86,unicorn.UC_MODE_64)
for address in [0x1408ce000,0x142aa4000]:
 u.mem_map(address,0x1000);u.mem_write(address,pe.get_data(address-image_base,0x1000))
u.mem_map(0x10000,0x10000);u.mem_map(0x800000,0x1000)
table=0x142aa44f4
disp=struct.unpack('<i',bytes(u.mem_read(table+7*4,4)))[0]
assert table+disp==0x1408cea79
rows=[]
for wall_a in [0,1,0xffffffff,0x100000000,0xffffffffffffffff]:
 for wall_b in [0,1,0xffffffff,0x100000000,0xffffffffffffffff]:
  for same_hash in [False,True]:
   for other_tag in [7,8]:
    u.mem_write(0x10000,struct.pack('<B7xQQ8x',7,wall_a,0)+bytes(0xc0))
    u.mem_write(0x10100,struct.pack('<B7xQQ8x',other_tag,wall_b,0)+bytes(0xc0))
    u.mem_write(0x10018,struct.pack('<Q',0x10200));u.mem_write(0x10118,struct.pack('<Q',0x10300))
    u.mem_write(0x10280,struct.pack('<Q',0xfeedfacecafebeef));u.mem_write(0x10380,struct.pack('<Q',0xfeedfacecafebeef if same_hash else 0xfeedfacecafebeee))
    u.reg_write(UC_X86_REG_RCX,0x10000);u.reg_write(UC_X86_REG_RDX,0x10100);u.reg_write(UC_X86_REG_RSP,0x1f000);u.mem_write(0x1f000,struct.pack('<Q',0x800000))
    u.emu_start(0x1408cea30,0x800000,count=200)
    got=bool(u.reg_read(UC_X86_REG_RAX)&255);expected=same_hash and other_tag==7 and wall_a==wall_b
    assert got==expected,(wall_a,wall_b,same_hash,other_tag,got)
    rows.append({'a':wall_a,'b':wall_b,'same_hash':same_hash,'tag_b':other_tag,'result':got})
result={'scope':'unchanged original entire Mesh::eq tag7 specialization; no map/Rapier/dirty-owner runtime claim','original_sha256':hashlib.sha256(raw).hexdigest(),'entry':'1408cea30','bytes_sha256':hashlib.sha256(pe.get_data(0x1408cea30-image_base,215)).hexdigest(),'jump7':'1408cea79','cases':rows,'passed':len(rows)}
(base/'mesh-eq-independent.json').write_text(json.dumps(result,indent=2));print(json.dumps({k:v for k,v in result.items() if k!='cases'}))
