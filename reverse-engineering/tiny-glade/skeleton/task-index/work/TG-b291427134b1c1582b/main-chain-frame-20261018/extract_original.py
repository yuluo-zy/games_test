from pathlib import Path
import hashlib,pefile,capstone,json
root=Path('/Users/yuluo/Desktop/game')
source=root/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
data=source.read_bytes()
assert hashlib.sha256(data).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=data,fast_load=True)
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
out=Path(__file__).parent
identity=[]
for start,size in [(0x1409dca90,631),(0x140533b10,631),(0x1409ee1e0,3623)]:
 off=pe.get_offset_from_rva(start-0x140000000)
 code=data[off:off+size]
 (out/f'{start:#x}.asm').write_text('\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in cs.disasm(code,start)))
 identity.append({'address':hex(start),'size':size,'sha256':hashlib.sha256(code).hexdigest()})
(out/'native-identity.json').write_text(json.dumps({'image':str(source),'sha256':hashlib.sha256(data).hexdigest(),'version':'B','functions':identity},indent=2))
