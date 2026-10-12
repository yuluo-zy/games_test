"""Check source annotation coverage and independent original-byte call samples."""
from collections import Counter
import gzip
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
COLLECT = Path('/Users/yuluo/.codex/skills/native-evidence-pack/scripts/collect.py')
PYTHON = PROJECT.parents[2] / 'tools/mcp-servers/ghidra-mcp/.venv/bin/python'

def records(path):
    with gzip.open(path, 'rt') as f:
        for line in f: yield json.loads(line)

def main():
    checks = []
    for v in 'ABC':
        out = ROOT / '_dependencies' / v
        baseline = json.loads((PROJECT / 'tracking/baselines.json').read_text())['baselines'][v]
        report = json.loads((out / 'report.json').read_text())
        source_report = json.loads((ROOT / '_pdb' / v / 'report.json').read_text())
        files = json.loads((out / 'files.json').read_text())
        assert report['annotated_files'] == source_report['annotated_files'] == len(files)
        assert report['annotated_functions'] == source_report['logical_symbol_and_type_pairs']
        functions = 0; dependencies = Counter(); imports = 0
        for item in files:
            source = ROOT / v / item['path']; text = source.read_text()
            data = json.loads(source.with_suffix('.rs.dependencies.json').read_text())
            function_count = len(data['functions']); functions += function_count
            imports += len(data['imports'])
            assert text.count('// 函数体依赖注解（实现尚未恢复）：') == function_count
            assert text.count('// 函数符号: ') == function_count
            assert all(not line.strip() or line.startswith('//') for line in text.splitlines())
            assert source.with_suffix('.rs.dependencies.jsonl.gz').is_file()
            for fn in data['functions']:
                for dep in fn['dependencies']:
                    dependencies[dep['relationship']] += 1
                    if dep['relationship'] == 'pdb_inline_direct':
                        ev = dep['first_evidence']; assert ev['parent_record_offset'] == ev['ancestor_record_offset']
                    elif dep['relationship'] == 'pdb_inline_transitive':
                        ev = dep['first_evidence']; assert ev['parent_record_offset'] != ev['ancestor_record_offset']
                    elif dep['relationship'] == 'native_indirect_call_unresolved':
                        assert dep['first_evidence']['direct'] is False
        assert functions == report['annotated_functions']
        assert imports == report['use_namespace_candidates']
        assert dict(dependencies) == report['aggregate_dependency_records']
        original_proc = next(r for r in records(ROOT / v / 'crates/systems/roof/src/create_roof.rs.pdb.jsonl.gz')
                             if r.get('class') == 'procedure' and r['name'] == 'system_roof::create_roof::create_roof')
        manifest = {'schema_version': 1, 'artifact': {'id': f'tiny-glade-{v}', 'version': v,
            'path': baseline['binary']['path'], 'sha256': baseline['binary']['sha256'],
            'debug': {'path': baseline['pdb']['path'], 'sha256': baseline['pdb']['sha256']}},
            'tools': {'symbolizer': '/opt/homebrew/opt/llvm/bin/llvm-symbolizer', 'objdump': '/opt/homebrew/opt/llvm/bin/llvm-objdump'},
            'requests': [{'id': 'roof-create', 'rva': hex(original_proc['rva']), 'length': original_proc['code_size'], 'symbolize_offsets': [0], 'allow_missing_symbols': False}]}
        m = out / 'verification-manifest.json'; m.write_text(json.dumps(manifest, indent=2) + '\n')
        subprocess.run([str(PYTHON), str(COLLECT), str(m), '--output', str(out / 'verification-pack'), '--cache', str(ROOT / '_dependencies/evidence-cache')], check=True, capture_output=True)
        pack = json.loads((out / 'verification-pack/roof-create.json').read_text())
        expected = {int(r['va'], 16): r for r in pack['instructions'] if r['assembly'].startswith('call')}
        actual = {r['va']: r for r in records(out / 'native-calls.jsonl.gz') if r['event'] == 'call' and r['range_va'] == original_proc['va']}
        assert set(actual) == set(expected)
        for va, row in expected.items():
            assert row['bytes'] == actual[va]['raw_hex']
            m = re.search(r'callq\s+(0x[0-9a-f]+)', row['assembly'])
            if m: assert actual[va]['direct'] and int(m[1], 16) == actual[va]['target_va']
            else: assert actual[va]['direct'] is False
        checks.append({'version': v, 'ok': True, 'annotated_files': len(files), 'annotated_functions': functions,
            'use_namespace_candidates': imports, 'dependency_groups': sum(dependencies.values()),
            'independent_original_roof_calls_matched': len(expected), 'no_active_rust_imports_or_fake_function_bodies_added': True,
            'all_generated_function_entries_have_annotations': True, 'native_decode_gaps': report['native_extraction']['decode_gaps']})
        print(json.dumps(checks[-1]), flush=True)
    result = {'ok': True, 'checks': checks, 'scope': 'Dependency annotation coverage, PDB nesting and original-byte call extraction; not reconstructed source imports, runtime dispatch or business validation'}
    (ROOT / 'dependency-annotation-verification.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')

if __name__ == '__main__': main()
