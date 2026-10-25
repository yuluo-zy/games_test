"""Identity-bound SPIR-V CFG proof and one-binding SSGI candidate preparation."""
from pathlib import Path
import difflib
import hashlib
import json
import re
import struct
import subprocess

ROOT = Path('/Users/yuluo/Desktop/game')
OUT = Path(__file__).parent
PACK = ROOT / 'ljxsj_92385/Tiny Glade/compiled-shaders/_ssgi_temporal_filter.cs.hlsl_cs_main_0.bin'
PACK_SHA = 'd31c87e724cf473fea5cc9cf65e66699177e57997eb8b8aaf9651482313920b1'
RETAINED = ROOT / 'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-06f4abbe570c5f0ee3/render-prelude-lut-20261018/scene-adapter/_ssgi_temporal_filter.cs.hlsl_cs_main_0.bin.spv'
SPV_SHA = '243e9f543962788aa5130959ef59e6e2c8aefd990bb9da3ff8f0af184b0aa7f3'
SOURCE = ROOT / 'games_test/tiny-glade/crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs'
RELATIVE = Path('crates/country-core/src/systems/render_loop/compute_passes/ssgi.rs')


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    packed, spv = PACK.read_bytes(), RETAINED.read_bytes()
    assert sha(packed) == PACK_SHA and sha(spv) == SPV_SHA
    assert len(spv) == 13032 and packed[23:23 + len(spv)] == spv
    words = struct.unpack(f'<{len(spv) // 4}I', spv)
    assert words[0] == 0x07230203
    cursor, raw = 5, []
    while cursor < len(words):
        count, opcode = words[cursor] >> 16, words[cursor] & 0xffff
        assert count and cursor + count <= len(words)
        raw.append({'word': cursor, 'opcode': opcode, 'operands': list(words[cursor + 1:cursor + count])})
        cursor += count
    assert cursor == len(words)
    original = OUT / 'ssgi-temporal-original.spv'
    original.write_bytes(spv)
    assembly = OUT / 'ssgi-temporal-original.raw.spvasm'
    subprocess.run(['/opt/homebrew/bin/spirv-dis', '--raw-id', str(original), '-o', str(assembly)], check=True)
    text = assembly.read_text()
    names = {name: identifier for identifier, name in re.findall(r'OpName\s+(%\d+)\s+"([^"]+)"', text)}
    globals_id, history_id, frame_id = names['$Globals'], names['history_tex'], names['frame_constants']
    assert re.search(rf'OpDecorate {re.escape(history_id)} Binding 15\b', text)
    constants = {}
    for identifier, literal in re.findall(r'(%\d+) = OpConstant %\d+ (-?\d+)\s*$', text, re.M):
        constants[identifier] = int(literal)
    for identifier, value in re.findall(r'(%\d+) = OpConstant(True|False) %\d+', text):
        constants[identifier] = value == 'True'
    blocks, block, in_function = {}, None, False
    for line in text.splitlines():
        match = re.match(r'\s*(?:(%\d+)\s*=\s*)?(Op\w+)\s*(.*?)\s*$', line)
        if not match:
            continue
        result, op, arguments = match.groups()
        arguments = arguments.split()
        if op == 'OpFunction':
            assert not in_function
            in_function = True
        elif op == 'OpFunctionEnd':
            in_function = False
        elif in_function and op == 'OpLabel':
            block = result
            blocks[block] = []
        elif in_function and block is not None:
            blocks[block].append({'result': result, 'op': op, 'args': arguments})
    assert len(re.findall(r'= OpFunction ', text)) == 1
    entry = next(iter(blocks))
    successors = {}
    for label, instructions in blocks.items():
        term = instructions[-1]
        if term['op'] == 'OpBranch':
            successors[label] = term['args'][:1]
        elif term['op'] == 'OpBranchConditional':
            successors[label] = term['args'][1:3]
        else:
            assert term['op'] == 'OpReturn'
            successors[label] = []
    predecessors = {label: set() for label in blocks}
    for parent, children in successors.items():
        for child in children:
            predecessors[child].add(parent)
    dominators = {label: ({entry} if label == entry else set(blocks)) for label in blocks}
    changed = True
    while changed:
        changed = False
        for label in blocks:
            if label == entry:
                continue
            value = {label} | set.intersection(*(dominators[p] for p in predecessors[label]))
            if value != dominators[label]:
                dominators[label], changed = value, True
    history_loads = [(label, ins) for label, body in blocks.items() for ins in body if history_id in ins['args']]
    assert len(history_loads) == 1
    history_block, history_load = history_loads[0]
    assert history_load['op'] == 'OpLoad'
    history_image = history_load['result']
    sampled_images = [(label, ins) for label, body in blocks.items() for ins in body if history_image in ins['args']]
    assert len(sampled_images) == 1 and sampled_images[0][1]['op'] == 'OpSampledImage'
    sampled_id = sampled_images[0][1]['result']
    samples = [(label, ins) for label, body in blocks.items() for ins in body if sampled_id in ins['args']]
    assert len(samples) == 1 and samples[0][1]['op'] == 'OpImageSampleExplicitLod'

    def specialize(history_valid, view_history_valid=1):
        values, pointers = dict(constants), {}
        label, previous, trace = entry, None, []
        while True:
            assert label not in trace  # Original main is an acyclic CFG.
            trace.append(label)
            for ins in blocks[label]:
                result, op, args = ins['result'], ins['op'], ins['args']
                if op == 'OpAccessChain':
                    if args[1] == globals_id and constants.get(args[2]) == 0:
                        pointers[result] = history_valid
                    elif args[1] == frame_id and constants.get(args[2]) == 8:
                        pointers[result] = view_history_valid
                elif op == 'OpLoad' and args[1] in pointers:
                    values[result] = pointers[args[1]]
                elif op in ('OpINotEqual', 'OpIEqual') and args[1] in values and args[2] in values:
                    equal = values[args[1]] == values[args[2]]
                    values[result] = not equal if op == 'OpINotEqual' else equal
                elif op == 'OpLogicalNot' and args[1] in values:
                    values[result] = not values[args[1]]
                elif op == 'OpPhi':
                    selected = [value for value, parent in zip(args[1::2], args[2::2]) if parent == previous]
                    assert len(selected) == 1
                    if selected[0] in values:
                        values[result] = values[selected[0]]
                elif op == 'OpBranch':
                    previous, label = label, args[0]
                elif op == 'OpBranchConditional':
                    assert args[0] in values, f'Unknown control condition {args[0]}'
                    previous, label = label, args[1] if values[args[0]] else args[2]
                elif op == 'OpReturn':
                    return trace
    zero = specialize(0)
    nonzero = specialize(1)
    assert zero == ['%82', '%88', '%97', '%95']
    assert history_block not in zero and history_block in nonzero
    zero_reads_frame_constants = any(frame_id in ins['args'] for label in zero for ins in blocks[label])
    assert not zero_reads_frame_constants
    proof = {'original_pack': str(PACK), 'pack_sha256': PACK_SHA, 'spv_offset': 23,
             'spv_bytes': len(spv), 'spv_words': len(words), 'spv_sha256': SPV_SHA,
             'retained_equals_original_stage': True, 'full_instruction_walk': len(raw),
             'history_variable': history_id, 'history_binding': 15,
             'unique_history_load': {'block': history_block, **history_load},
             'unique_history_sample': {'block': samples[0][0], **samples[0][1]},
             'history_block_dominators': sorted(dominators[history_block]),
             'all_edges': successors, 'zero_history_path': zero, 'nonzero_history_witness_path': nonzero,
             'zero_path_independent_of_frame_constants': not zero_reads_frame_constants,
             'scope': 'Exact native SPIR-V control proof for history_valid==0; nonzero remains active. No GPU/Cargo.'}
    (OUT / 'cfg-proof.json').write_text(json.dumps(proof, indent=2) + '\n')
    (OUT / 'original-words.json').write_text(json.dumps(raw, indent=2) + '\n')

    before = SOURCE.read_text()
    needle = '    let p3 = dispatch_extent(p3, &temporal_out);'
    assert before.count(needle) == 1
    after = before.replace(needle, '''    // Original packaged SSGI temporal SPV: history_valid==0 takes the
    // input-only branch; its sole history_tex sample is unreachable.
    // Keep this exact read binding and synchronization, skipping only the
    // previous-content requirement while the temporal pair is invalid.
    let p3 = if !pair.fresh {
        p3.inactive_texture_contents(binding::HISTORY_TEX)
    } else {
        p3
    };
''' + needle)
    marker = '    #[test]\n    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value")]\n'
    assert after.count(marker) == 1
    after = after.replace(marker, '''    #[test]
    fn only_invalid_ssgi_history_skips_previous_contents() {
        for valid in [false, true] {
            let (e, out) = run(valid);
            let pass = &out.passes[2];
            assert_eq!(e.uniform_u32(pass, binding::HISTORY_VALID), u32::from(valid));
            assert_eq!(pass.inactive_texture_bindings,
                if valid { Vec::<u8>::new() } else { vec![1] });
            // This is the original five-access sequence, including both
            // writes and the still-declared history read, not a removed slot.
            assert_eq!(accesses(pass).iter().map(|a| (a.1, a.2)).collect::<Vec<_>>(),
                [(0x19, 1), (0x19, 1), (0x19, 1), (0x32, 0), (0x32, 0)]);
            assert!(out.passes[0].inactive_texture_bindings.is_empty());
            assert!(out.passes[1].inactive_texture_bindings.is_empty());
        }
    }

''' + marker)
    candidate = OUT / RELATIVE
    candidate.parent.mkdir(parents=True, exist_ok=True)
    candidate.write_text(after)
    (OUT / 'ssgi-single-history.patch').write_text(''.join(difflib.unified_diff(
        before.splitlines(keepends=True), after.splitlines(keepends=True),
        fromfile='a/' + str(RELATIVE), tofile='b/' + str(RELATIVE))))
    manifest = {'production_source': str(SOURCE), 'before_sha256': sha(before.encode()),
                'candidate': str(candidate), 'candidate_sha256': sha(after.encode()),
                'patch': 'ssgi-single-history.patch',
                'test_filter': 'only_invalid_ssgi_history_skips_previous_contents',
                'status': 'candidate only, CPU test not run; root owns production/Cargo/GPU'}
    (OUT / 'candidate-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps({'proof': proof, 'candidate': manifest}, indent=2))


if __name__ == '__main__':
    main()
