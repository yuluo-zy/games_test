"""Bounded, read-only instruction evidence. Never executes the original game."""
from pathlib import Path
import hashlib
import json
import struct
import capstone

ROOT = Path('D:/game')
original = ROOT / 'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
ledger = ROOT / 'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv'
data = original.read_bytes()
identity = hashlib.sha256(data).hexdigest()
assert identity == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', data, 0x3c)[0]
assert data[pe:pe + 4] == b'PE\x00\x00'
section_count = struct.unpack_from('<H', data, pe + 6)[0]
optional_size = struct.unpack_from('<H', data, pe + 20)[0]
optional = pe + 24
assert struct.unpack_from('<H', data, optional)[0] == 0x20b
image_base = struct.unpack_from('<Q', data, optional + 24)[0]
sections = [struct.unpack_from('<IIII', data, optional + optional_size + i * 40 + 8)
            for i in range(section_count)]

def body_at(address, size):
    rva = address - image_base
    for virtual_size, virtual_address, raw_size, raw_offset in sections:
        if virtual_address <= rva and rva + size <= virtual_address + raw_size:
            offset = raw_offset + rva - virtual_address
            return data[offset:offset + size]
    raise ValueError('Procedure exceeds a single raw PE section')
decoder = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
procedures = {}
for line in ledger.read_text(encoding='utf-8').splitlines():
    fields = line.split('\t')
    if len(fields) >= 5 and fields[0].startswith('0x'):
        procedures[int(fields[0], 16)] = (int(fields[1]), fields[-1])
out = Path(__file__).parent
records = []
for name, address in [
    ('compile-loading', 0x1401647b0), ('compile-main', 0x140161cd0),
    ('render-world', 0x1404ec110), ('mei', 0x1409c6700),
    ('ui2d', 0x1409c4470), ('logo-overlay', 0x1408a5f50),
    ('refraction', 0x1409bf320), ('pre-taa', 0x1409c3a80),
    ('post-taa', 0x1409c40b0), ('water', 0x1409bf800),
    ('ice', 0x1409c08d0), ('egui', 0x1400ddc80),
    ('menu-bg', 0x1408c9a30), ('session-previews', 0x1408f6c00),
    ('lut-gallery', 0x1408b0560),
]:
    size, symbol = procedures[address]
    body = body_at(address, size)
    assert len(body) == size
    asm = '\n'.join(f'{ins.address:#x}: {ins.mnemonic} {ins.op_str}'
                    for ins in decoder.disasm(body, address)) + '\n'
    (out / f'{name}.asm').write_text(asm, encoding='utf-8')
    records.append(dict(name=name, address=hex(address), size=size, symbol=symbol,
                        body_sha256=hashlib.sha256(body).hexdigest(),
                        assembly_sha256=hashlib.sha256(asm.encode()).hexdigest()))
constant = body_at(0x142a9cc30, 16)
assert struct.unpack('<4I', constant) == (1, 0x3e3851ec, 0x3e3851ec, 0x3e3851ec)
(out / 'manifest.json').write_text(json.dumps(dict(
    original=str(original), original_sha256=identity, ledger=str(ledger),
    evidence='Static bounded decoding only; no original process/GPU execution',
    functions=records, constants=[dict(address='0x142a9cc30', bytes=constant.hex(),
        words=list(struct.unpack('<4I', constant)), sha256=hashlib.sha256(constant).hexdigest(),
        use='LUT gallery color tag1 plus RGB clear values; alpha1 written at 0x1408b0b23')]),
    indent=2) + '\n', encoding='utf-8')
print(f'Bounded decoded {len(records)} procedures; original SHA256 {identity}')
