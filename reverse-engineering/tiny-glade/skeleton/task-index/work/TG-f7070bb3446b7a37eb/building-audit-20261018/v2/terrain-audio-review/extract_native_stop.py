from pathlib import Path
import hashlib, json
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
out=Path(__file__).resolve().parent
binary=Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw=binary.read_bytes(); pe=pefile.PE(data=raw,fast_load=True)
start=0x140a59430; end=0x140a59650
code=pe.get_data(start-pe.OPTIONAL_HEADER.ImageBase,end-start)
asm=list(Cs(CS_ARCH_X86,CS_MODE_64).disasm(code,start))
(out/'140a59430-original-B.asm').write_text('\n'.join(f'{i.address:x}: {i.bytes.hex():24} {i.mnemonic} {i.op_str}'for i in asm)+'\n')
(out/'stop-original-identity.json').write_text(json.dumps({'target':'B','binary':str(binary),'binary_sha256':hashlib.sha256(raw).hexdigest(),'start':hex(start),'end_exclusive':hex(end),'range_sha256':hashlib.sha256(code).hexdigest(),'extent':'bounded call target region, may include tail padding; not whole-module parity','instruction_count':len(asm)},indent=2)+'\n')
print({'instructions':len(asm),'calls':[(hex(i.address),i.op_str)for i in asm if i.mnemonic=='call']})
