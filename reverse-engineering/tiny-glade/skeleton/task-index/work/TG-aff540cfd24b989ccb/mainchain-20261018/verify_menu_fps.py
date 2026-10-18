from pathlib import Path
import hashlib,json,struct
import pefile
from unicorn import Uc,UC_ARCH_X86,UC_MODE_64
from unicorn.x86_const import UC_X86_REG_XMM1,UC_X86_REG_XMM6,UC_X86_REG_XMM11
binary=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
data=binary.read_bytes();sha=hashlib.sha256(data).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=data,fast_load=True)
start,end=0x1404de7b3,0x1404de7d3
code=pe.get_data(start-0x140000000,end-start)
cases=[]
for bits in [0,0x80000000,0xbf800000,0x41f00000,0x42700000,0x43100000,0x7f800000,0xff800000,0x7fc01234,0x7f801234]:
 u=Uc(UC_ARCH_X86,UC_MODE_64)
 for base in [0x1404de000,0x1429d1000]:
  u.mem_map(base,0x1000);u.mem_write(base,pe.get_data(base-0x140000000,0x1000))
 u.reg_write(UC_X86_REG_XMM6,bits);u.reg_write(UC_X86_REG_XMM11,0)
 u.emu_start(start,end,count=20)
 result=u.reg_read(UC_X86_REG_XMM1)&0xffffffff
 value=struct.unpack('<f',struct.pack('<I',bits))[0]
 expected=struct.unpack('<I',struct.pack('<f',min(value,60.0) if value>0 else 60.0))[0]
 assert result==expected,(bits,result,expected)
 cases.append({'input_bits':bits,'output_bits':result})
report={'artifact_sha256':sha,'entry':hex(start),'exit_before_call':hex(end),'bytes':code.hex(),'boundary':'resolved menu-cap numeric block only; ECS lookups/profiling gate/limiter sleep are not emulated','cases':cases}
(Path(__file__).parent/'evidence/menu-fps-native.json').write_text(json.dumps(report,indent=2))
print('native menu-cap block:',len(cases),'inputs matched')
