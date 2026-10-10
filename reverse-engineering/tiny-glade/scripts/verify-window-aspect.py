"""以原作机器码模拟为独立参照，验证还原的 Rust 数值行为。"""
import hashlib
import json
import random
import struct
import subprocess
from pathlib import Path
from unicorn import Uc, UC_ARCH_X86, UC_MODE_64
from unicorn.x86_const import UC_X86_REG_RCX, UC_X86_REG_XMM0, UC_X86_REG_MXCSR

root = Path('D:/game/reverse-engineering/tiny-glade')
source = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
raw = source.read_bytes()
pe = struct.unpack_from('<I', raw, 0x3c)[0]
section_count, optional_size = struct.unpack_from('<H', raw, pe + 6)[0], struct.unpack_from('<H', raw, pe + 20)[0]
image_base = struct.unpack_from('<Q', raw, pe + 24 + 24)[0]
rva = 0x140949c20 - image_base
section_table = pe + 24 + optional_size
file_offset = None
for i in range(section_count):
    pos = section_table + 40 * i
    virtual_size, virtual_address, raw_size, raw_offset = struct.unpack_from('<IIII', raw, pos + 8)
    if virtual_address <= rva < virtual_address + max(virtual_size, raw_size):
        file_offset = raw_offset + rva - virtual_address
        break
if file_offset is None:
    raise ValueError('函数地址无法映射到发行文件')
code = raw[file_offset:file_offset + 20]
expected = bytes.fromhex('8b01f3480f2ac08b4104f3480f2ac8f30f5ec1c3')
if code != expected:
    raise ValueError('原作指令与已核实的反汇编不一致')
base, data = 0x10000000, 0x20000000
cpu = Uc(UC_ARCH_X86, UC_MODE_64)
cpu.mem_map(base, 0x1000)
cpu.mem_map(data, 0x1000)
cpu.mem_write(base, code)
cpu.reg_write(UC_X86_REG_MXCSR, 0x1f80)
boundary = [0, 1, 2, 3, 255, 256, 1080, 1920, 65535, 65536, 2**24 - 1, 2**24, 2**24 + 1, 2**31 - 1, 2**31, 2**32 - 1]
cases = [(a, b) for a in boundary for b in boundary]
rng = random.Random(20261010)
cases += [(rng.randrange(2**32), rng.randrange(2**32)) for _ in range(1024)]
original = []
for a, b in cases:
    cpu.mem_write(data, struct.pack('<II', a, b))
    cpu.reg_write(UC_X86_REG_RCX, data)
    # 最后一条 RET 不涉及数值结果；在它之前结束，不执行原作启动流程。
    cpu.emu_start(base, base + 19, count=6)
    original.append(cpu.reg_read(UC_X86_REG_XMM0) & 0xffffffff)
binary = root / 'workspace/window-aspect-reconstructed.exe'
subprocess.run(['C:/Users/liyu/.cargo/bin/rustc.exe', '-O', str(root / 'reconstruction/math/window_aspect.rs'), '-o', str(binary)], check=True)
result = subprocess.run([str(binary)], input=''.join(f'{a} {b}\n' for a, b in cases), text=True, capture_output=True, check=True)
recovered = [int(line, 16) for line in result.stdout.splitlines()]
failures = [{'input': cases[i], 'machine_bits': hex(a), 'rust_bits': hex(b)} for i, (a, b) in enumerate(zip(original, recovered)) if a != b]
report = {'source_sha256': hashlib.sha256(raw).hexdigest(), 'original_va': '0x140949c20', 'file_offset': hex(file_offset), 'machine_code_hex': code.hex(),
          'oracle': 'Unicorn 模拟发行文件中的原始指令', 'mxcsr': '0x1f80', 'cases': len(cases), 'failures': failures,
          'ok': len(recovered) == len(original) and not failures, 'full_game_executed': False,
          'limitations': ['仅验证该20字节函数的数值行为', '字段语义名及完整结构布局未还原', '不同MXCSR模式尚未测试']}
(root / 'evidence/native/window-aspect-verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2), encoding='utf-8')
print(json.dumps({'cases': len(cases), 'ok': report['ok'], 'failures': len(failures)}))
if not report['ok']:
    raise SystemExit(1)
