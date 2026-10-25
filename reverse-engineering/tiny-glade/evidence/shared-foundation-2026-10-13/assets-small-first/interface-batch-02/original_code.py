"""Bounded inspection of the original B RNE; no guessed inline function ABI."""
import hashlib,json,pathlib,struct
from capstone import Cs,CS_ARCH_X86,CS_MODE_64
OUT=pathlib.Path(__file__).resolve().parent
raw=pathlib.Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()
SHA=hashlib.sha256(raw).hexdigest()
assert SHA=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];op=struct.unpack_from('<H',raw,pe+20)[0]
BASE=struct.unpack_from('<Q',raw,pe+48)[0];SIZE=struct.unpack_from('<I',raw,pe+24+56)[0]
SECTIONS=[struct.unpack_from('<IIII',raw,pe+24+op+40*i+8) for i in range(struct.unpack_from('<H',raw,pe+6)[0])]
def read(va,size):
    for virtual,rva,n,off in SECTIONS:
        if BASE+rva<=va and va+size<=BASE+rva+n:return raw[off+va-BASE-rva:off+va-BASE-rva+size]
    raise ValueError(hex(va))
if __name__=='__main__':
    ranges=[(0x1412a23c2,0x1412a24a0),(0x1412a5528,0x1412a5608),(0x1412a58aa,0x1412a591b),
            (0x142682b3d,0x142682b70),(0x142687c7e,0x142687cac),(0x142687f42,0x142687f5e),
            (0x1420d8eec,0x1420d8f20),(0x1420d9039,0x1420d90bc),(0x1420d922a,0x1420d92bc),
            (0x1420d8c24,0x1420d8c38),(0x141552e40,0x141552e60),(0x14155309b,0x1415530bb),
            (0x141556130,0x14155613c),(0x14067dd80,0x14067dd84),(0x141556080,0x1415560f1)]
    decoder=Cs(CS_ARCH_X86,CS_MODE_64);lines=[]
    for start,end in ranges:
        lines.append(f'RANGE {start:#x}..{end:#x} SHA256={hashlib.sha256(read(start,end-start)).hexdigest()}')
        lines.extend(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in decoder.disasm(read(start,end-start),start))
    (OUT/'bounded-original-asm.txt').write_text('\n'.join(lines)+'\n',encoding='utf8')
    needle=b'as_concrete()';at=raw.find(needle)
    print(raw[at:at+160] if at!=-1 else 'No as_concrete literal')
    print('\n'.join(lines[:100]))
