"""Check extraction coverage against retained independent LLVM PDB audits."""
from collections import Counter
import gzip
import hashlib
import json
from pathlib import Path
import re
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
HEADER = Path('/opt/homebrew/opt/llvm/include/llvm/DebugInfo/CodeView/CodeViewSymbols.def')
KINDS = {n: int(v, 16) for n, v in re.findall(r'\b(?:SYMBOL_RECORD(?:_ALIAS)?|CV_SYMBOL)\(\s*(S_\w+)\s*,\s*(0x[\da-fA-F]+)', HEADER.read_text())}

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for b in iter(lambda: f.read(1024 * 1024), b''): h.update(b)
    return h.hexdigest()

def validate_raw(row):
    raw = bytes.fromhex(row['raw_hex'])
    length, kind = struct.unpack_from('<HH', raw)
    assert length + 2 == len(raw)
    assert kind == row['kind']

def main():
    checks = []
    for v in 'ABC':
        r = ROOT / '_pdb' / v
        report = json.loads((r / 'report.json').read_text())
        audit = json.loads((PROJECT / 'evidence/reconstruction-strategy-2026-10-11' / f'{v}-pdb-capability.json').read_text())
        assert report['pdb_sha256'] == audit['pdb']['sha256']
        assert sha(r / 'original.pdb') == report['pdb_sha256']
        expected = audit['record_counts']['all_modules']
        observed = {int(k): n for k, n in report['module_symbol_counts_by_kind'].items()}
        assert observed == {KINDS[k]: n for k, n in expected['symbols'].items()}
        assert report['native_completion']['modules'] == expected['module_count']
        assert len(list((r / 'modules').glob('*.jsonl.gz'))) == expected['module_count']
        assert report['events']['module_end'] == expected['module_count']
        counts = {'TPI': Counter(), 'IPI': Counter()}
        with gzip.open(r / 'types.jsonl.gz', 'rt') as f:
            for line in f:
                t = json.loads(line); validate_raw(t)
                counts[t['namespace']][t['decoded']['Kind']] += 1
        assert dict(counts['TPI']) == audit['TPI_type_counts']
        assert dict(counts['IPI']) == audit['IPI_counts']
        sources = json.loads((r / 'source-files.json').read_text())
        assert len(sources) == report['annotated_files']
        assert all((ROOT / v / s['path']).read_text().startswith('// Generated from PDB metadata') for s in sources)
        assert all((ROOT / v / (s['path'] + '.pdb.jsonl.gz')).is_file() for s in sources)
        assert sum(s['logical_symbol_and_type_pairs'] for s in sources) == report['logical_symbol_and_type_pairs']
        # Real PDB regression cases: duplicate-start file blocks previously hid
        # a private-source contribution behind an inline/library contribution.
        cause_path = r / 'unfilled-source-path-causes.json'
        overlap_cases = 0
        if cause_path.exists():
            causes = json.loads(cause_path.read_text())
            names = {name for s in sources for name in re.findall(r'^// 函数符号: (.+)$', (ROOT / v / s['path']).read_text(), re.M)}
            expected_names = {p['name'] for p in causes['records'] if p['reason'] == 'existing_skeleton_path_missed_by_mapping'}
            assert expected_names <= names
            overlap_cases = len(expected_names)
        # Independent target-module comparison with the project's retained original evidence.
        target_module = {'A': 3196, 'B': 3467, 'C': 3778}[v]
        sample_procs = []; sample_inlines = []
        with gzip.open(r / 'modules' / f'{target_module:05d}.jsonl.gz', 'rt') as f:
            for line in f:
                row = json.loads(line)
                if row['event'] == 'symbol':
                    validate_raw(row)
                    if row.get('class') == 'procedure': sample_procs.append(row)
                    elif row.get('class') == 'inline_site': sample_inlines.append(row)
        original = subprocess.run(['/opt/homebrew/opt/llvm/bin/llvm-pdbutil', 'dump', f'--modi={target_module}', '--symbols', '--files', '--il', '-l', str(r / 'original.pdb')], check=True, capture_output=True, text=True).stdout
        for row in sample_procs:
            offset = row['record_offset']
            m = re.search(rf'^\s*{offset} \| S_[GL]PROC32[^\n]*`([^`]+)`\s*\n\s*parent = (\d+), end = (\d+), addr = (\d+):(\d+), code size = (\d+)', original, re.M)
            assert m and m[1] == row['name']
            assert tuple(map(int, m.groups()[1:])) == (row['parent'], row['end'], row['section'], row['section_offset'], row['code_size'])
        for row in sample_inlines:
            m = re.search(rf'^\s*{row["record_offset"]} \| S_INLINESITE[^\n]*\n\s*inlinee = 0x([0-9A-Fa-f]+)[^\n]*parent = (\d+), end = (\d+)', original, re.M)
            assert m and (int(m[1], 16), int(m[2]), int(m[3])) == (row['inlinee'], row['parent'], row['end'])
        checks.append({'version': v, 'ok': True, 'source_files': len(sources), 'modules': expected['module_count'], 'module_symbol_records': sum(observed.values()),
            'tpi_records': sum(counts['TPI'].values()), 'ipi_records': sum(counts['IPI'].values()),
            'independent_symbol_kind_counts_match': True, 'independent_type_kind_counts_match': True,
            'all_type_raw_records_checked': True, 'roof_module_raw_symbol_records_checked': True,
            'independent_roof_procedures': len(sample_procs), 'independent_roof_inline_sites': len(sample_inlines)})
        checks[-1]['equal_start_source_mapping_regression_cases'] = overlap_cases
    result = {'ok': True, 'checks': checks, 'scope': 'PDB metadata integrity, coverage and source placement; not ABI reconstruction or game behavior acceptance'}
    (ROOT / 'pdb-verification.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(result, ensure_ascii=False))

if __name__ == '__main__': main()
