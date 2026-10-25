"""Run declaration scenarios through the new crate, then compare their actual
produced commands with the original full queue_submit CPU entry. This is a
boundary differential, not an original whole-graph execution claim.
"""
from pathlib import Path
import copy
import hashlib
import importlib.util
import json
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parents[1]
sys.path.insert(0, 'C:/Users/liyu/AppData/Local/Packages/PythonSoftwareFoundation.Python.3.12_qbz5n2kfra8p0/LocalCache/local-packages/Python312/site-packages')
spec = importlib.util.spec_from_file_location('native_queue', ROOT / 'scripts/native_support/queue_entry.py')
q = importlib.util.module_from_spec(spec)
spec.loader.exec_module(q)

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def original(commands, sequences, timeline, error):
    q.setup(count=len(commands), start=timeline, error=error)
    for index, handle in enumerate(commands):
        address = q.COMMANDS + index * 88
        q.b.write(address, '<QQII', 2, sequences[handle], 0, 0)
        q.b.write(address + 0x48, '<Q', handle)
    q.CPU.emu_start(0x140c04e20, q.b.STOP, count=1000000)
    returned = q.CPU.reg_read(q.UC_X86_REG_RIP) == q.b.STOP
    return {
        'backend': copy.deepcopy(q.STATE['backend']),
        'receipt': list(q.b.read(q.RECEIPT, '<QQ')) if returned else None,
        'next_timeline': q.b.read(q.b.DEVICE + 0x7f8, '<Q')[0],
        'panic': q.STATE['panic'],
        'original_entry_visited': '0x140c04e20' in q.STATE['visited'],
    }

def scenarios():
    example = json.loads((ROOT / 'examples/two-pass.json').read_text(encoding='utf-8'))
    items = [('two_pass', copy.deepcopy(example))]
    one = copy.deepcopy(example); one['passes'] = one['passes'][:1]
    items.append(('one_pass', one))
    separate = copy.deepcopy(example); separate['batch_per_pass'] = True
    items.append(('separate_batches', separate))
    failed = copy.deepcopy(example); failed['queue_error'] = -4; failed['complete'] = False
    items.append(('queue_failure', failed))
    callback = copy.deepcopy(example); callback['fail_callback'] = 1
    items.append(('callback_failure_candidate_only', callback))
    end = copy.deepcopy(example); end['fail_end'] = True
    items.append(('end_failure_candidate_only', end))
    return items

def main():
    command = ['cargo', 'build', '--locked', '--offline', '--release', '--manifest-path', str(ROOT / 'Cargo.toml')]
    subprocess.run(command, check=True)
    items = scenarios()
    output = subprocess.run([str(ROOT / 'target/release/rhapsody.exe')],
        input=''.join(json.dumps(s) + '\n' for _, s in items), text=True, capture_output=True, check=True)
    rust = [json.loads(line) for line in output.stdout.splitlines()]
    assert len(rust) == len(items)
    observations = []
    failures = []
    compared = 0
    for (name, scenario), result in zip(items, rust):
        begins = {}
        for event in result['lifecycle_events']:
            if event.startswith('begin:'):
                _, handle, sequence, tag, index = event.split(':')
                assert (int(tag), int(index)) == (0, 0)
                begins[int(handle)] = int(sequence)
        cursor = scenario['timeline_start']
        native_calls = []
        captures = [a for a in result['api'] if a['op'] == 'queue_submit']
        for call_index, capture in enumerate(captures):
            handles = [h for s in capture['submits'] for h in s['commands']]
            # Only the public API/backend boundary is captured. Original queue
            # builds its arrays from generated command objects at its full entry.
            reference = original(handles, begins, cursor, scenario.get('queue_error', 0))
            expected = {key: capture[key] for key in ['queue', 'fence', 'submits']}
            if reference['backend'] != expected:
                failures.append({'scenario':name,'call':call_index,'field':'Vulkan submit arguments','rust':expected,'original':reference['backend']})
            receipt = result['receipts'][call_index] if result['status'] == 'ok' else None
            if (reference['receipt'][1] if reference['receipt'] else None) != receipt:
                failures.append({'scenario':name,'call':call_index,'field':'receipt','rust':receipt,'original':reference['receipt']})
            cursor += len(handles)
            if reference['next_timeline'] != cursor:
                failures.append({'scenario':name,'field':'timeline reservation','rust':cursor,'original':reference['next_timeline']})
            assert reference['original_entry_visited']
            compared += 1
            native_calls.append(reference)
        if result['next_timeline'] != cursor:
            failures.append({'scenario':name,'field':'candidate next timeline','rust':result['next_timeline'],'expected_from_commands':cursor})
        observations.append({'name':name,'input':scenario,'rust':result,'original_queue_calls':native_calls,
            'scope':'graph candidate to original full queue boundary' if captures else 'candidate-only early-failure chain'})
    original_lock = PROJECT / 'tracking/original-Cargo.lock'
    original_packages = tomllib.loads(original_lock.read_text(encoding='utf-8'))['package']
    packages = tomllib.loads((ROOT / 'Cargo.lock').read_text(encoding='utf-8'))['package']
    dependencies = []
    for p in packages:
        if 'source' not in p: continue
        identity = {k:p.get(k) for k in ['name','version','source','checksum']}
        matched = any(all(o.get(k) == v for k,v in identity.items()) for o in original_packages)
        dependencies.append({**identity,'matches_original':matched})
        if not matched: failures.append({'field':'dependency identity','package':identity})
    inputs = {str(p.relative_to(ROOT)).replace('\\','/'):sha(p)
        for sub in ['src','scripts','tests','examples'] for p in (ROOT / sub).rglob('*')
        if p.is_file() and '__pycache__' not in p.parts}
    inputs.update({name:sha(ROOT / name) for name in ['Cargo.toml','Cargo.lock']})
    report = {'status':'verified_partial_boundary' if not failures else 'mismatch',
        'scenario_count':len(items),'original_queue_calls':compared,'mismatch_count':len(failures),'failures':failures,
        'target_sha256':q.b.EXPECTED_SHA,'original_entry':'0x140c04e20','original_entry_from_beginning':True,
        'submit_arrays_injected':False,'command_source':'current crate graph→compiled→record→counter-produced command objects',
        'dependencies':dependencies,'input_hashes':inputs,'observations':observations,
        'limits':['original full graph compilation/recording are not executed by this queue boundary oracle',
                  'each original queue call starts from a declared CPU fixture, not a live device or whole multi-frame execution',
                  'public allocator/CRT/Vulkan and diagnostic boundaries are captured; private device-error conversion remains supplied',
                  'callback/end failures are candidate-only scenarios',
                  'no original author source, full ABI, Vulkan/GPU or game image equivalence claim']}
    (ROOT / 'evidence/chain-comparison.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({key:report[key] for key in ['status','scenario_count','original_queue_calls','mismatch_count','failures']}))
    if failures: raise SystemExit(1)

if __name__ == '__main__': main()
