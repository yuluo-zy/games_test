from pathlib import Path
import hashlib,json,pefile,capstone
root=Path('/Users/yuluo/Desktop/game')
source=root/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
data=source.read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=data,fast_load=True)
start=0x14082c000
size=356
code=data[pe.get_offset_from_rva(start-0x140000000):][:size]
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
cs.detail=True
out=Path(__file__).parent
(out/'init-color-library.asm').write_text('\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in cs.disasm(code,start)))
(out/'identity.json').write_text(json.dumps({'image':str(source),'sha256':hashlib.sha256(data).hexdigest(),'version':'B','address':hex(start),'bytes':size,'code_sha256':hashlib.sha256(code).hexdigest()},indent=2))

start=0x14093f900
code=data[pe.get_offset_from_rva(start-0x140000000):][:245]
(out/'color-default.asm').write_text('\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in cs.disasm(code,start)))
