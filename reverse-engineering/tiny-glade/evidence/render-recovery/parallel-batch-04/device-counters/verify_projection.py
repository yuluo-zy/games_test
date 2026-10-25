from pathlib import Path
import gzip
import hashlib
import json
import random
import struct
import subprocess
import sys

sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64, UC_HOOK_CODE
from unicorn.x86_const import UC_X86_REG_RSI, UC_X86_REG_RBX, UC_X86_REG_RBP, UC_X86_REG_RDI

root = Path('D:/game/reverse-engineering/tiny-glade')
output = root / 'evidence/render-recovery/parallel-batch-04/device-counters'
candidate = root / 'reconstruction/render/work-packages/device-counters-r6'
binary = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = binary.read_bytes()
digest = hashlib.sha256(raw).hexdigest()
assert digest == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
header = struct.unpack_from('<I', raw, 60)[0]
optional = header + 24
base = struct.unpack_from('<Q', raw, optional + 24)[0]
image_size = struct.unpack_from('<I', raw, optional + 56)[0]
machine = Uc(UC_ARCH_X86, UC_MODE_64)
machine.mem_map(base, (image_size + 4095) & ~4095)
for section_index in range(struct.unpack_from('<H', raw, header + 6)[0]):
    section_start = optional + struct.unpack_from('<H', raw, header + 20)[0] + 40 * section_index
    virtual_size, address, raw_size, offset = struct.unpack_from('<IIII', raw, section_start + 8)
    machine.mem_write(base + address, raw[offset:offset + raw_size])
seed_vector = list(struct.unpack('<QQ', machine.mem_read(0x142ae0d00, 16)))
assert seed_vector == [1, 1]
data = 0x20000000
machine.mem_map(data, 0x10000)
device = data
command_buffer = data + 0x2000
counters_address = data + 0x3000
stop_state = {}

def stop_hook(cpu, address, instruction_size, user):
    if address in [0x140bed31d, 0x140bed3a8]:
        stop_state['address'] = address
        cpu.emu_stop()

machine.hook_add(UC_HOOK_CODE, stop_hook)
random_source = random.Random(20261012)
seeds = [0, 1, 2, 0xffffffff, 0x8000000000000000, 0xffffffffffffffff]
cases = []
for count in [1, 2, 3, 8]:
    for tag in [0, 1, 2, 3, 0xffffffff]:
        for index in [0, 1, 2, 7, 8, 0xfffffffe, 0xffffffff]:
            for seed in seeds:
                cases.append((tag, index, [seed] * count))
for case_index in range(2000):
    count = random_source.randrange(1, 9)
    tag = random_source.getrandbits(32)
    index = random_source.choice([0, 1, 3, 7, 8, 0xffffffff, random_source.getrandbits(32)])
    cases.append((tag, index, [random_source.getrandbits(64) for _ in range(count)]))
original_lines = []
input_lines = []
for tag, index, values in cases:
    stop_state.clear()
    machine.mem_write(device + 0x668, struct.pack('<QQ', counters_address, len(values)))
    machine.mem_write(counters_address, struct.pack('<' + 'Q' * len(values), *values))
    machine.mem_write(command_buffer, bytes([0xa5]) * 88)
    machine.reg_write(UC_X86_REG_RSI, device)
    machine.reg_write(UC_X86_REG_RBX, command_buffer)
    machine.reg_write(UC_X86_REG_RBP, tag)
    machine.reg_write(UC_X86_REG_RDI, index)
    machine.emu_start(0x140bed2d8, 0x140bed3b4, count=50)
    assert 'address' in stop_state
    after = list(struct.unpack('<' + 'Q' * len(values), machine.mem_read(counters_address, 8 * len(values))))
    if stop_state['address'] == 0x140bed31d:
        state_tag, sequence, stored_tag, stored_index = struct.unpack('<QQII', machine.mem_read(command_buffer, 24))
        assert state_tag == 1 and stored_tag == tag and stored_index == index
        assert machine.mem_read(command_buffer + 0x56, 1) == b'\x00'
        result = f'ok {sequence:x}'
    else:
        assert bytes(machine.mem_read(command_buffer, 88)) == bytes([0xa5]) * 88
        result = 'bounds'
    original_lines.append(result + ''.join(f' {value:x}' for value in after))
    input_lines.append(' '.join(f'{value:x}' for value in [tag, index, *values]))
subprocess.run(['cargo', 'build', '--release', '--locked', '--offline', '--manifest-path', str(candidate / 'Cargo.toml')], check=True)
runner = subprocess.run([str(candidate / 'target/release/device-counter-oracle.exe')], input='\n'.join(input_lines) + '\n', text=True, capture_output=True, check=True)
candidate_lines = runner.stdout.splitlines()
assert len(candidate_lines) == len(original_lines)
failures = [{'input': input_line, 'native': native, 'rust': rust} for input_line, native, rust in zip(input_lines, original_lines, candidate_lines) if native != rust]
with gzip.open(output / 'native-projection-oracle.jsonl.gz', 'wt', encoding='utf8') as stream:
    for input_line, native in zip(input_lines, original_lines):
        stream.write(json.dumps({'input': input_line, 'native': native}) + '\n')
for name, start, end in [
    ('constructor-family-selection', 0x140bf2c18, 0x140bf2e48),
    ('constructor-counter-production', 0x140bf600e, 0x140bf61a8),
    ('constructor-counter-fields', 0x140bf6605, 0x140bf664b),
    ('seed-initializer-use', 0x1418b7510, 0x1418b7630),
    ('counter-slice-transfer', 0x1418c9520, 0x1418c95ca),
    ('auxiliary-queue-production', 0x1418b48f0, 0x1418b4b68),
    ('cb-sequence-producer', 0x140bed2d8, 0x140bed340),
]:
    disassembly = subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe', '-d', f'--start-address={start:#x}', f'--stop-address={end:#x}', str(binary)], text=True, capture_output=True, check=True)
    (output / (name + '.asm.txt')).write_text(disassembly.stdout, encoding='utf8')
report = {
    'binary_sha256': digest,
    'cases': len(cases),
    'mismatches': len(failures),
    'failures': failures[:10],
    'native_start': '0x140bed2d8',
    'native_stops': ['0x140bed31d', '0x140bed3a8'],
    'candidate_sha256': hashlib.sha256((candidate / 'src/lib.rs').read_bytes()).hexdigest(),
    'initializer_simd_constant': {'address': '0x142ae0d00', 'u64_values': seed_vector},
    'boundaries': [
        'Original CPU-only counter-selection, atomic xadd, and CB field-store instruction slice after successful Vulkan begin.',
        'No entry-state checks, Vulkan callbacks, bucket insert, panic unwind, allocation failure, or concurrent atomic scheduling executed.',
        'Large tag/index/seed cases include raw invalid representations and wrap edges; they do not establish safe caller reachability.',
        'Generic initializer and slice transfer disassembly retained as constructor callsite evidence; standard collection implementations are reused, not reconstructed.',
    ],
}
(output / 'native-validation.json').write_text(json.dumps(report, indent=2), encoding='utf8')
print(json.dumps(report, indent=2))
assert not failures
