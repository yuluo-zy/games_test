"""Independent original SSE oracle and exact candidate-function Rust probe.

This verifies the volume arithmetic only, not the FMOD/ECS loop or grid owner.
The Rust function and constants are extracted byte-for-byte from the candidate;
no second Rust formula is supplied as an expected result.
"""
from pathlib import Path
import hashlib
import json
import re
import struct
import subprocess
import pefile
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import (
    UC_X86_REG_XMM0, UC_X86_REG_XMM2, UC_X86_REG_XMM9,
    UC_X86_REG_XMM11, UC_X86_REG_XMM12, UC_X86_REG_XMM13,
    UC_X86_REG_XMM14, UC_X86_REG_XMM15,
)

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
original = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = original.read_bytes()
original_sha = hashlib.sha256(raw).hexdigest()
assert original_sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw)
base = pe.OPTIONAL_HEADER.ImageBase

def native(address, size):
    return pe.get_data(address-base, size)

def bits(value):
    return struct.unpack('<I', struct.pack('<f', value))[0]

constants = [(0x142925904, 0x3f800000),
             (0x142ac4d00, 0x3c800000),
             (0x142ac4d20, 0x42290000)]
for address, expected in constants:
    assert native(address, 4) == struct.pack('<I', expected)
segments = [(0x1409ed2b8, 0x1409ed2c6),
            (0x1409ed355, 0x1409ed372)]
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(0x1409ed000, 0x1000)
cpu.mem_write(0x1409ed000, native(0x1409ed000, 0x1000))
cases = []
for count in range(1, 65):
    for distance_squared in (0.0, 1e-38, 1.0, 42.25, 129.75, 1000000.0, float('inf')):
        for register in (UC_X86_REG_XMM0, UC_X86_REG_XMM2,
                         UC_X86_REG_XMM14):
            cpu.reg_write(register, 0)
        cpu.reg_write(UC_X86_REG_XMM9, 0x3f800000)
        cpu.reg_write(UC_X86_REG_XMM11, 0x3c800000)
        cpu.reg_write(UC_X86_REG_XMM12, 0x42290000)
        cpu.reg_write(UC_X86_REG_XMM13, bits(float(count)))
        cpu.reg_write(UC_X86_REG_XMM15, bits(distance_squared))
        for begin, end in segments:
            cpu.emu_start(begin, end)
        cases.append([bits(float(count)), bits(distance_squared),
                      cpu.reg_read(UC_X86_REG_XMM2) & 0xffffffff])

candidate = HERE.parent/'history/lifecycle-candidate/tiny-glade/crates/country-core/src/systems/audio/water_ambience_audio.rs'
source = candidate.read_text(encoding='utf-8')
function_start = source.index('fn water_ambience_volume(')
brace = source.index('{', function_start)
depth, end = 1, brace+1
while depth:
    depth += (source[end] == '{') - (source[end] == '}')
    end += 1
function = source[function_start:end]
declarations = []
for name in ('WATER_VOLUME_ONE', 'WATER_VOLUME_BIT_WEIGHT', 'WATER_VOLUME_ATTENUATION'):
    declarations.append(re.search(r'pub const '+name+r': f32 = [^;]+;', source).group())
harness = '\n'.join(declarations)+'\n'+function+'\nfn main() {\n'
for index, (count, distance, expected) in enumerate(cases):
    harness += ('assert_eq!(water_ambience_volume(std::hint::black_box(f32::from_bits('
                f'{count})), std::hint::black_box(f32::from_bits({distance}))).to_bits(), '
                f'{expected}, "original SSE case {index}");\n')
harness += f'println!("{len(cases)}/{len(cases)} exact original-SSE volume cases passed");\n}}\n'
(HERE/'probe.rs').write_text(harness, encoding='utf-8')
rustc = Path('C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustc.exe')
command = [str(rustc), '--edition=2024', '-C', 'opt-level=2', str(HERE/'probe.rs'), '-o', str(HERE/'probe.exe')]
compiled = subprocess.run(command, capture_output=True, text=True)
assert compiled.returncode == 0, compiled.stderr
executed = subprocess.run([str(HERE/'probe.exe')], capture_output=True, text=True)
report = {
    'scope': 'Volume arithmetic only; no FMOD/ECS/grid lifecycle acceptance',
    'original_sha256': original_sha,
    'segments': [{'start': hex(a), 'end_exclusive': hex(b),
                  'sha256': hashlib.sha256(native(a, b-a)).hexdigest()} for a,b in segments],
    'constants': [[hex(a), hex(b)] for a,b in constants],
    'candidate': str(candidate),
    'candidate_sha256': hashlib.sha256(candidate.read_bytes()).hexdigest(),
    'exact_function_sha256': hashlib.sha256(function.encode()).hexdigest(),
    'cases': cases, 'rustc_command': command,
    'exit_code': executed.returncode,
    'stdout': executed.stdout, 'stderr': executed.stderr,
}
(HERE/'results.json').write_text(json.dumps(report, indent=2), encoding='utf-8')
print(executed.stdout)
assert executed.returncode == 0, executed.stderr
