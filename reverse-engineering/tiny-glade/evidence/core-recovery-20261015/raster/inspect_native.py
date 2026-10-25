"""Bounded original B control-flow evidence, independent of the decompiler."""
import hashlib
import json
import struct
from pathlib import Path
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

ROOT = Path(__file__).parent
BIN = Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe')
data = BIN.read_bytes()
digest = hashlib.sha256(data).hexdigest()
assert digest == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', data, 60)[0]
count = struct.unpack_from('<H', data, pe + 6)[0]
sections = pe + 24 + struct.unpack_from('<H', data, pe + 20)[0]
def offset(va):
    for i in range(count):
        vs, rva, rawsize, raw = struct.unpack_from('<IIII', data, sections + i*40 + 8)
        if rva <= va - 0x140000000 < rva + vs:
            if va - 0x140000000 - rva >= rawsize:
                raise ValueError(f'{va:#x} is zero-filled virtual data, not raw-file data')
            return raw + va - 0x140000000 - rva
    raise ValueError(hex(va))

ranges = [(0x140b2bbf0, 0x30), (0x142260ae0, 0x150), (0x142260fa0, 0x400),
          (0x14083f220, 0xd0), (0x14083fa60, 0x80)]
md = Cs(CS_ARCH_X86, CS_MODE_64)
for va, size in ranges:
    lines = [f'{i.address:#x} {i.mnemonic:8} {i.op_str}'
             for i in md.disasm(data[offset(va):offset(va)+size], va)]
    (ROOT/f'{va:#x}.asm').write_text('\n'.join(lines), encoding='utf-8')
metadata = {'input': str(BIN), 'sha256': digest,
            'ranges': [[hex(va), size] for va, size in ranges],
            'distance_threshold': struct.unpack_from('<f', data, offset(0x1429cd060))[0]}
(ROOT/'native-identity.json').write_text(json.dumps(metadata, indent=2), encoding='utf-8')
print(json.dumps(metadata))
