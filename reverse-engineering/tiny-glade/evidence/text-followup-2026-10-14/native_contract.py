"""Bounded original B RNE getters/setters, without game startup or GPU calls."""
import ctypes as c
import hashlib
import json
from pathlib import Path
import struct as s
import subprocess

out = Path(__file__).parent
target = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = target.read_bytes()
sha = hashlib.sha256(raw).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = s.unpack_from('<I', raw, 60)[0]
opt = pe + 24
section_count = s.unpack_from('<H', raw, pe + 6)[0]
sections = opt + s.unpack_from('<H', raw, pe + 20)[0]
size = s.unpack_from('<I', raw, opt + 56)[0]
k = c.WinDLL('kernel32', use_last_error=True)
k.VirtualAlloc.argtypes = [c.c_void_p, c.c_size_t, c.c_uint32, c.c_uint32]
k.VirtualAlloc.restype = c.c_void_p
k.VirtualFree.argtypes = [c.c_void_p, c.c_size_t, c.c_uint32]
k.VirtualFree.restype = c.c_int
base = k.VirtualAlloc(None, size, 0x3000, 0x40)
assert base
for i in range(section_count):
    _, rva, length, offset = s.unpack_from('<IIII', raw, sections + i * 40 + 8)
    c.memmove(base + rva, raw[offset:offset + length], length)

# Capture both native return registers. This adapter does not assert a Rust ABI.
code = (bytes.fromhex('53 48 89 cb 48 89 d1 48 83 ec 20 48 b8')
        + s.pack('<Q', base + 0xb8f670)
        + bytes.fromhex('ff d0 48 83 c4 20 48 89 03 48 89 53 08 5b c3'))
thunk = k.VirtualAlloc(None, len(code), 0x3000, 0x40)
assert thunk
c.memmove(thunk, code, len(code))
getter = c.WINFUNCTYPE(None, c.c_void_p, c.c_void_p)(thunk)
align = c.WINFUNCTYPE(None, c.c_void_p, c.c_uint8)(base + 0xb90f40)
width = c.WINFUNCTYPE(None, c.c_void_p, c.c_float)(base + 0xb90ec0)
getters = []
setters = []
for count in [0, 1, 2, 0xffffffff, 0x100000001, 0xffffffffffffffff]:
    obj = c.create_string_buffer(0x4e0)
    s.pack_into('<Q', obj, 0x440, count)
    result = (c.c_uint64 * 2)()
    getter(result, obj)
    assert result[0] == c.addressof(obj) + 0x460
    assert result[1] == count
    getters.append({'count': count, 'texture_offset': result[0] - c.addressof(obj), 'returned_count': result[1]})
for tag in range(6):
    obj = c.create_string_buffer(0x260)
    before = obj.raw
    align(obj, tag)
    assert obj.raw[0x256] == tag
    assert all(a == b for i, (a, b) in enumerate(zip(before, obj.raw)) if i != 0x256)
    setters.append({'setter': 'align', 'tag': tag, 'redraw': obj.raw[0x170]})
for old, new in [(0., 0.), (-0., 0.), (5., 5.), (5., 6.), (float('inf'), float('inf')), (float('nan'), float('nan')), (5., float('nan'))]:
    obj = c.create_string_buffer(0x260)
    s.pack_into('<f', obj, 0x248, old)
    width(obj, new)
    redraw = obj.raw[0x170]
    changed = old != new
    assert redraw == int(changed)
    setters.append({'setter': 'width', 'old_bits': s.unpack('<I', s.pack('<f', old))[0], 'new_bits': s.unpack('<I', s.pack('<f', new))[0], 'redraw': redraw})

ranges = json.loads(Path('D:/game/reverse-engineering/tiny-glade/evidence/tg-text-full-2026-10-13/ranges.json').read_text())
wanted = {'0x140b8e4a0', '0x140b8f670', '0x140b8f3f0', '0x140b90780', '0x140b90ea0', '0x140b90ec0', '0x140b90f40'}
bound_ranges = []
for row in ranges:
    if row['address'] not in wanted:
        continue
    address = int(row['address'], 16)
    data = c.string_at(base + address - 0x140000000, row['size'])
    assert hashlib.sha256(data).hexdigest() == row['bytes_sha256']
    bound_ranges.append(row)
    asm = subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe', '-d', '--start-address=' + row['address'], '--stop-address=' + hex(address + row['size']), str(target)], capture_output=True, text=True, check=True).stdout
    (out / (row['address'] + '.asm.txt')).write_text(asm, encoding='utf-8')
report = {'binary_sha256': sha, 'scope': 'render getter; align byte store; max_width inline owned Buffer equality/redraw', 'original_instruction_execution': True, 'original_constructor_or_gpu_executed': False, 'getters': getters, 'setters': setters, 'passed': len(getters) + len(setters), 'ranges': bound_ranges}
(out / 'native-contract.json').write_text(json.dumps(report, indent=2) + '\n')
k.VirtualFree(thunk, 0, 0x8000)
k.VirtualFree(base, 0, 0x8000)
print(json.dumps({'passed': report['passed'], 'ok': True}))
