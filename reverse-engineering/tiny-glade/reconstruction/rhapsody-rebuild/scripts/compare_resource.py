"""运行原作资源入口，保存候选实现必须满足的字节/返回值边界。

这里核对的是从完整机器码入口运行得到的合同；没有把 Python 投影模型
当成已恢复的 Rust 模块，也不执行诊断函数、资源设备创建或真实 GPU。
"""
from pathlib import Path
import hashlib
import importlib.util
import json
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parents[1]
# 复用现有运行环境；不安装、升级或重新配置逆向工具。
sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
spec = importlib.util.spec_from_file_location('resource_native_fixture', ROOT / 'scripts/native_support/pe_fixture.py')
b = importlib.util.module_from_spec(spec)
spec.loader.exec_module(b)

FUNCTIONS = {
    'validate_read_access': (0x14003bb80, 1),
    'ResourceHandle::raw': (0x140833250, 3),
    'RuntimeResource::borrow': (0x140bd6c80, 96),
    'descriptor_from_texture': (0x140bd6ce0, 18),
    'descriptor_from_buffer': (0x140bd6d00, 14),
    'ResourceOrigin::ty': (0x140bd6d10, 151),
}
OBJECT = b.MEMORY + 0x1000
ARC_ALLOCATION = b.MEMORY + 0x2000
INPUT = b.MEMORY + 0x3000
OUTPUT = b.MEMORY + 0x4000


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def reset():
    b.CPU.mem_write(b.MEMORY, bytes(0x20000))
    b.CPU.mem_write(b.STACK, bytes(0x20000))
    b.STATE.clear()
    b.STATE.update(cut=b.STOP, visited=set(), panic='ok', cut_reached=False)
    for register in (b.UC_X86_REG_RAX, b.UC_X86_REG_RBX, b.UC_X86_REG_RCX,
                     b.UC_X86_REG_RDX, b.UC_X86_REG_RSI, b.UC_X86_REG_RDI,
                     b.UC_X86_REG_RBP, b.UC_X86_REG_R8, b.UC_X86_REG_R9,
                     b.UC_X86_REG_R10, b.UC_X86_REG_R11, b.UC_X86_REG_R12,
                     b.UC_X86_REG_R13, b.UC_X86_REG_R14, b.UC_X86_REG_R15):
        b.CPU.reg_write(register, 0)
    pointer = b.STACK + 0x10000
    b.write(pointer, '<Q', b.STOP)
    b.CPU.reg_write(b.UC_X86_REG_RSP, pointer)


def run(name, rcx, rdx=0):
    entry, _ = FUNCTIONS[name]
    b.CPU.reg_write(b.UC_X86_REG_RCX, rcx)
    b.CPU.reg_write(b.UC_X86_REG_RDX, rdx)
    b.CPU.emu_start(entry, b.STOP, count=1000)
    returned = b.CPU.reg_read(b.UC_X86_REG_RIP) == b.STOP
    if not returned and b.STATE['panic'] == 'ok':
        raise RuntimeError(f"未到达返回或已知诊断边界：{name}")
    return {
        'returned': returned,
        'panic_boundary': None if returned else ('unsupported_descriptor'
            if b.CPU.reg_read(b.UC_X86_REG_RIP) == 0x1428d9390 else b.STATE['panic']),
        'panic_address': None if returned else hex(b.CPU.reg_read(b.UC_X86_REG_RIP)),
        'rax': b.CPU.reg_read(b.UC_X86_REG_RAX),
        'rdx': b.CPU.reg_read(b.UC_X86_REG_RDX),
        'entry_visited': entry in b.STATE['visited'],
        'visited': [hex(x) for x in sorted(b.STATE['visited'])],
    }


def main():
    cases = []
    mismatches = []

    def save(name, inputs, actual, expected):
        # 投影模型用于清晰描述字节合同，不声称它是作者原始类型定义。
        failures = {key: {'actual': actual[key], 'expected': value}
                    for key, value in expected.items() if actual[key] != value}
        if failures:
            mismatches.append({'function': name, 'input': inputs, 'fields': failures})
        assert actual['entry_visited']
        cases.append({'function': name, 'input': inputs, 'original': actual,
                      'inferred_byte_contract': expected, 'matches_contract': not failures})

    for value in (0, 1, 65535, 0x80000000, 0xffffffff, 0x123456789abcdef0):
        reset()
        save('ResourceHandle::raw', {'rcx': value}, run('ResourceHandle::raw', value),
             {'returned': True, 'rax': value & 0xffffffff})

    for tag in range(6):
        reset()
        b.write(OBJECT, '<QQ', tag, ARC_ALLOCATION)
        actual = run('RuntimeResource::borrow', OBJECT)
        if tag < 4:
            expected = {'returned': True, 'rax': tag // 2, 'rdx': OBJECT + 8}
        elif tag == 4:
            expected = {'returned': True, 'rax': 2, 'rdx': ARC_ALLOCATION + 16}
        else:
            expected = {'returned': False, 'panic_boundary': 'kind'}
        save('RuntimeResource::borrow', {'runtime_variant_tag': tag}, actual, expected)

    # Create 分支由 descriptor 的 niche 字节判别。覆盖整个字节域，避免
    # 根据字段名误以为只接受 0/1 或者给标签 8 擅自补造资源类型。
    for descriptor_tag in range(256):
        reset()
        b.write(OBJECT, '<Q', 0)
        b.write(OBJECT + 0x36, '<B', descriptor_tag)
        actual = run('ResourceOrigin::ty', OBJECT)
        expected = ({'returned': False, 'panic_boundary': 'unsupported_descriptor'}
                    if descriptor_tag == 8 else
                    {'returned': True, 'rax': int(descriptor_tag == 7)})
        save('ResourceOrigin::ty', {'origin_tag': 0, 'descriptor_byte_at_54': descriptor_tag}, actual, expected)
    for origin_tag in (1, 2):
        for kind in (0, 1, 2, 7, 255):
            reset()
            b.write(OBJECT, '<QB', origin_tag, kind)
            actual = run('ResourceOrigin::ty', OBJECT)
            save('ResourceOrigin::ty', {'origin_tag': origin_tag, 'kind_byte_at_8': kind}, actual,
                 {'returned': True, 'rax': kind})
    reset()
    b.write(OBJECT, '<Q', 3)
    save('ResourceOrigin::ty', {'origin_tag': 3}, run('ResourceOrigin::ty', OBJECT),
         {'returned': False, 'panic_boundary': 'kind'})

    for name, byte_count in [('descriptor_from_texture', 32), ('descriptor_from_buffer', 16)]:
        for seed in (0, 41, 255):
            reset()
            source = bytes((seed + i * 17) & 255 for i in range(32))
            before = bytes([0xa5] * 32)
            b.CPU.mem_write(INPUT, source)
            b.CPU.mem_write(OUTPUT, before)
            actual = run(name, OUTPUT, INPUT)
            actual['output_hex'] = bytes(b.CPU.mem_read(OUTPUT, 32)).hex()
            expected_bytes = bytearray(before)
            expected_bytes[:byte_count] = source[:byte_count]
            if byte_count == 16:
                expected_bytes[30] = 7
            save(name, {'source_hex': source.hex(), 'output_before_hex': before.hex()}, actual,
                 {'returned': True, 'rax': OUTPUT, 'output_hex': expected_bytes.hex()})
    reset()
    save('validate_read_access', {}, run('validate_read_access', OBJECT), {'returned': True})

    # 编译真正的候选 Rust 消费者，再使用同一组原码输入对照结果。
    subprocess.run(['cargo', 'build', '--locked', '--offline', '--release',
                    '--manifest-path', str(ROOT / 'Cargo.toml'), '--example', 'resource_probe'], check=True)
    process = subprocess.run([str(ROOT / 'target/release/examples/resource_probe.exe')],
                             input=''.join(json.dumps(case) + '\n' for case in cases),
                             capture_output=True, text=True, check=True)
    outputs = [json.loads(line) for line in process.stdout.splitlines()]
    assert len(outputs) == len(cases)
    for case, rust in zip(cases, outputs):
        native = case['original']
        expected = {'returned': native['returned']}
        if not native['returned']:
            expected['panic_boundary'] = native['panic_boundary']
        elif case['function'] == 'ResourceHandle::raw':
            expected['raw'] = native['rax']
        elif case['function'] == 'ResourceOrigin::ty':
            expected['kind'] = native['rax']
        elif case['function'] == 'RuntimeResource::borrow':
            expected['kind'] = native['rax']
            tag = case['input']['runtime_variant_tag']
            pointer = OBJECT + 8 if tag < 4 else ARC_ALLOCATION + 16
            expected['points_to_payload'] = native['rdx'] == pointer
        elif case['function'].startswith('descriptor_from_'):
            expected['output_hex'] = native['output_hex']
        failures = {key: {'rust': rust[key], 'original': value}
                    for key, value in expected.items() if rust[key] != value}
        if failures:
            mismatches.append({'comparison': 'Rust→原码', 'function': case['function'],
                               'input': case['input'], 'fields': failures})
        case['rust'] = rust
        case['rust_matches_original'] = not failures

    # 保存限定函数区间反汇编和原跳表，避免反编译器读到相邻数据。
    objdump = shutil.which('llvm-objdump') or 'C:/Program Files/LLVM/bin/llvm-objdump.exe'
    disassembly = {}
    machine_code = {}
    for name, (entry, length) in FUNCTIONS.items():
        text = subprocess.run([objdump, '-d', f'--start-address={hex(entry)}',
                               f'--stop-address={hex(entry + length)}', str(b.TARGET)],
                              capture_output=True, text=True, check=True).stdout
        disassembly[name] = text
        machine_code[name] = {'entry': hex(entry), 'pdb_length': length,
                              'bytes_hex': bytes(b.CPU.mem_read(entry, length)).hex()}
    tables = {}
    for name, table, count in [('borrow', 0x142b102c0, 6), ('origin_ty', 0x142b102d8, 4)]:
        relative = list(b.read(table, '<' + 'i' * count))
        tables[name] = {'address': hex(table), 'relative_offsets': relative,
                        'targets': [hex(table + value) for value in relative]}
    ledger = PROJECT / 'evidence/render-recovery/parallel/registry-pool/type-ledger.json'
    report = {
        'status': 'verified_partial_resource_boundary' if not mismatches else 'mismatch',
        'binary_sha256': b.EXPECTED_SHA,
        'entry_count': len(FUNCTIONS), 'native_case_count': len(cases),
        'mismatch_count': len(mismatches), 'mismatches': mismatches,
        'comparison_kind': '候选Rust资源消费者与原入口机器码的字节/返回值边界比较',
        'rust_implementation_compared': True, 'rust_case_count': len(outputs),
        'functions': machine_code, 'jump_tables': tables, 'bounded_disassembly': disassembly,
        'cases': cases,
        'inputs': {'scripts/compare_resource.py': sha(Path(__file__)),
                   'scripts/native_support/pe_fixture.py': sha(ROOT / 'scripts/native_support/pe_fixture.py'),
                   str(ledger): sha(ledger),
                   **{name: sha(ROOT / name) for name in ['Cargo.toml', 'Cargo.lock',
                       'src/rg/resource.rs', 'src/rg/resource_registry.rs', 'src/rg/graph.rs',
                       'examples/resource_probe.rs']}},
        'observations': [
            'ResourceHandle::raw 原入口只返回 ECX 的低32位；不解引用内存。',
            'RuntimeResource::borrow 对标签0/1返回Texture借用，对2/3返回Buffer借用，对4借用Arc分配的数据区+16；5诊断失败。',
            'ResourceOrigin::ty 的Create标签0读取self+54：7返回1，8诊断失败，其余返回0。Import标签1/2原样返回self+8字节；3诊断失败。',
            'Texture转换复制32字节；Buffer转换复制16字节且写output+30=7，其他输出字节未写入。',
            '两个validate_read_access实例合并到单字节ret入口，本实例不验证任何访问。',
        ],
        'layout_constraints': {
            'ResourceOrigin': {'size_from_prior_ledger': 72, 'tag_offset': 0, 'tag_width': 8,
                               'create_descriptor_offset': 24, 'descriptor_niche_offset': 30,
                               'import_kind_offset': 8},
            'RuntimeResource': {'size_from_prior_ledger': 64, 'tag_offset': 0, 'tag_width': 8,
                                'owned_or_unowned_payload_offset': 8, 'arc_pointer_offset': 8},
            'descriptor_union': {'minimum_observed_output_span': 32, 'buffer_niche': {'offset': 30, 'value': 7}},
        },
        'limits': [
            'PDB给出源码路径/函数名/边界；原签名表为void()占位，这里没有从PDB取回完整Rust类型字段。',
            'ResourceOrigin72字节、RuntimeResource64字节及字段语义来自已保存的汇编账本，不是新的PDB类型恢复声明。',
            'Rust ResourceOriginLayout适配的标签/字节结果、Descriptor转换字节、ResourceHandle返回及RuntimeResource实际引用路径已对照；高层ResourceOrigin整个图声明和设备生命周期不在此范围。',
            'RuntimeResource在Rust侧使用安全枚举和Arc，其物理布局不声称等于原64字节布局；只比较资源类型及借用原载荷语义。',
            'Rust OriginTypeError返回值与原诊断停止表示相同失败类别，不声称恢复原panic消息/错误ABI。',
            '借用返回的类型标签0/1/2及Arc数据偏移已观察；原始Rust类型名、生命周期/所有权仍需上层代码接线。',
            'ty标签8失败的具体资源语义未知；未执行格式化和panic运行时，只在真实诊断调用入口停止。',
            '有效枚举域外的u64标签未探测，不能保证原函数处理非法对象。',
            'TextureDesc/BufferDesc字段业务名称、真实managed校验/设备资源创建/Vulkan/GPU未覆盖。',
        ],
    }
    (ROOT / 'evidence/resource-comparison.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({k: report[k] for k in ['status', 'entry_count', 'native_case_count', 'mismatch_count', 'rust_implementation_compared']}, ensure_ascii=False))
    if mismatches:
        raise SystemExit(1)


if __name__ == '__main__':
    main()
