"""Inventory original-renderer CodeView metadata; never invent Rust signatures."""
from pathlib import Path
import collections
import gzip
import hashlib
import json
import re
import subprocess

ROOT = Path('D:/game/reverse-engineering/tiny-glade')
OUT = ROOT / 'evidence/render-skeleton'
PDB = Path('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
RNE = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
LLVM = 'C:/Program Files/LLVM/bin/llvm-pdbutil.exe'
RAW = OUT / 'raw'
SKEL = ROOT / 'reconstruction/render/original-skeleton'
PREFIX = '/home/h3/code/pouncelight/country-slice-private/'

def write_json(path, obj):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')

def selected(path):
    return path.startswith('crates/rhapsody/src/') or path.startswith('crates/country-core/src/render/')

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()

def main():
    modules_text = (RAW / 'modules.txt').read_text(encoding='utf-8-sig')
    modules = [(int(n), name) for n, name in re.findall(r'Mod (\d+) \| `([^`]+)`', modules_text)
               if name.startswith(('rhapsody-', 'country_core-'))]
    print(f'Inspecting {len(modules)} original renderer/core compilands', flush=True)
    files = {}
    procedures = []
    module_reports = []
    native_re = re.compile(r'^\s*(\d+) \| (S_[GL]PROC32(?:_ID)?) \[size = (\d+)\] `([^`]+)`\n\s*parent = (\d+), end = (\d+), addr = (\d+):(\d+), code size = (\d+)\n\s*type = `([^`]+)`[^\n]*', re.M)
    for ordinal, (idx, name) in enumerate(modules):
        cache = RAW / f'module-{idx:05d}.txt.gz'
        if not cache.exists():
            command = [LLVM, 'dump', '--files', '--symbols', '-l', f'--modi={idx}', str(PDB)]
            result = subprocess.run(command, check=True, capture_output=True)
            with gzip.open(cache, 'wb') as f:
                f.write(result.stdout)
        with gzip.open(cache, 'rt', encoding='utf-8') as f:
            text = f.read()
        file_section = text.split('                           Lines', 1)[0]
        owned_files = []
        for checksum, original in re.findall(r'^- \(SHA-256: ([A-Fa-f0-9]+)\) (.+)$', file_section, re.M):
            path = original.replace('\\', '/')
            relative = path[len(PREFIX):] if path.startswith(PREFIX) else path
            if not selected(relative):
                continue
            owned_files.append(relative)
            entry = files.setdefault(relative, {'relative_path': relative, 'original_paths': [], 'source_sha256': [], 'module_indices': [], 'procedure_indices': [], 'native_source_line_ranges': []})
            for key, value in [('original_paths', original), ('source_sha256', checksum.lower()), ('module_indices', idx)]:
                if value not in entry[key]: entry[key].append(value)
        # Line-table overlap proves a source contribution, not a primary function assignment.
        lines_section = text.split('                           Lines', 1)[-1].split('                          Symbols', 1)[0]
        blocks = []
        current = None
        for line in lines_section.splitlines():
            m = re.match(r'(.+) \(SHA-256: ([A-Fa-f0-9]+)\)$', line)
            if m:
                current = m[1].replace('\\', '/')
                if current.startswith(PREFIX): current = current[len(PREFIX):]
            m = re.match(r'\s+([0-9]+):([A-Fa-f0-9]+)-([A-Fa-f0-9]+), line/addr entries = (\d+)', line)
            if m and current:
                blocks.append({'path': current, 'section': int(m[1]), 'start': int(m[2], 16), 'end': int(m[3], 16), 'line_addresses': []})
            elif blocks:
                blocks[-1]['line_addresses'].extend((int(a), int(b, 16)) for a, b in re.findall(r'(\d+) ([A-Fa-f0-9]{8}) [! ]', line))
        module_proc_count = 0
        for m in native_re.finditer(text):
            record, kind, size, fname, parent, end, section, offset, code_size, type_text = m.groups()
            section, offset, code_size = int(section), int(offset), int(code_size)
            contributions = [b for b in blocks if selected(b['path']) and b['section'] == section and b['start'] < offset + code_size and b['end'] > offset]
            if not fname.startswith('rhapsody::') and not contributions and 'country_core::render::' not in fname:
                continue
            entry = {'module_index': idx, 'module_name': name, 'record_offset': int(record), 'record_kind': kind,
                     'name': fname, 'section': section, 'section_offset': offset, 'code_size': code_size,
                     'selection_reason': 'owned_namespace' if fname.startswith(('rhapsody::', 'country_core::render::')) else 'source_contribution' if contributions else 'mentions_renderer_type_in_instantiation',
                     'va': hex(0x140001000 + offset) if section == 1 else None,
                     'pdb_type': type_text, 'rust_signature_recovered': False,
                     'overlapping_owned_source_contributions': contributions,
                     'raw_evidence': str(cache.relative_to(ROOT)).replace('\\', '/')}
            proc_idx = len(procedures)
            procedures.append(entry)
            module_proc_count += 1
            for path in set(b['path'] for b in contributions):
                if path in files:
                    files[path]['procedure_indices'].append(proc_idx)
        for block in blocks:
            if block['path'] in files:
                line_nums = [a for a, b in block['line_addresses']]
                files[block['path']]['native_source_line_ranges'].append({'module_index': idx, 'section': block['section'], 'start': block['start'], 'end': block['end'], 'min_line': min(line_nums) if line_nums else None, 'max_line': max(line_nums) if line_nums else None})
        module_reports.append({'index': idx, 'name': name, 'owned_file_count': len(owned_files), 'selected_native_procedure_count': module_proc_count, 'raw_dump_sha256': sha(cache)})
        if ordinal % 30 == 0: print(f'{ordinal + 1}/{len(modules)} modules; {len(files)} owned files; {len(procedures)} selected native procedures', flush=True)
    public_path = ROOT / 'evidence/render-architecture/public-symbols.json'
    publics = json.loads(public_path.read_text(encoding='utf-8-sig'))
    render_publics = [p for p in publics if 'rhapsody::' in p.get('demangled', '') or 'country_core::render::' in p.get('demangled', '')]
    write_json(OUT / 'files.json', sorted(files.values(), key=lambda f: f['relative_path']))
    write_json(OUT / 'native-procedures.json', procedures)
    write_json(OUT / 'public-symbols.json', render_publics)
    write_json(OUT / 'modules.json', module_reports)
    for path, entry in files.items():
        target = SKEL / path
        target.parent.mkdir(parents=True, exist_ok=True)
        content = ['// Evidence-only source hierarchy recovered from the original Tiny Glade PDB.', '// Author source text and Rust signatures are not recovered.', f'// Original source: {entry["original_paths"][0]}', f'// Original source SHA256: {", ".join(entry["source_sha256"])}', f'// Compiland indices: {", ".join(map(str, entry["module_indices"]))}', '// The following native procedures overlap this file\'s line-table contributions.', '// Overlap includes inline contributions and does not prove primary ownership.']
        for pidx in entry['procedure_indices']:
            p = procedures[pidx]
            content.extend([f'// Procedure [{pidx}]: {p["name"]}', f'// Address: {p["va"]}; size: {p["code_size"]}; module: {p["module_index"]}; record: {p["record_offset"]}', f'// Debug type: {p["pdb_type"]} (not a recovered Rust signature)'])
        target.write_text('\n'.join(content) + '\n', encoding='utf-8')
    counts = collections.Counter(p['pdb_type'] for p in procedures)
    report = {'pdb': str(PDB), 'pdb_sha256': sha(PDB), 'rne': str(RNE), 'rne_sha256': sha(RNE), 'baseline_guid': '{09F1A02A-49F2-F8C8-4C4C-44205044422E}', 'baseline_age': 1,
              'scope': ['all rhapsody and country_core compilands; owned source paths limited to crates/rhapsody/src and crates/country-core/src/render'],
              'module_count': len(modules), 'source_file_count': len(files), 'native_procedure_record_count': len(procedures),
              'native_selection_reason_counts': dict(collections.Counter(p['selection_reason'] for p in procedures)),
              'owned_native_namespace_counts': {'rhapsody': sum(p['name'].startswith('rhapsody::') for p in procedures), 'country_core_render': sum(p['name'].startswith('country_core::render::') for p in procedures)},
              'source_file_scope_counts': {'rhapsody': sum(p.startswith('crates/rhapsody/') for p in files), 'country_core_render': sum(p.startswith('crates/country-core/') for p in files)},
              'retained_render_public_record_count': len(render_publics), 'native_debug_types': dict(counts),
              'complete_original_source_skeleton_established': False,
              'limitations': ['Only compiled contributing source paths are evidenced; removed/unused/generated source files are not enumerated.',
                  'Native procedures are optimized emitted bodies, not an enumeration of logical Rust methods. Generics/aliases/inlined-only functions require separate treatment.',
                  'Line-table overlap establishes contribution and may be caused by inlining; it is not an exact source-function assignment.',
                  'No Rust function signature, field offset, ownership/lifetime, struct/enum layout, generic bounds or implementation body is invented.',
                  'Public subset comes from retained render-architecture/public-symbols.json rather than a fresh all-publics scan; coverage of its originating filter is bounded.',
                  'Inlinee/type stream full recovery is outside this inventory; raw selected-module S_INLINESITE metadata is retained.',
                  'Shader files, game pass callers outside country-core/src/render and private rhapsody-related auxiliary crates are outside this source inventory.',
                  'Renderer inline functions or monomorphizations emitted in compilands other than rhapsody/country_core are not exhaustively covered.',
                  'Comment-only hierarchy is deliberately not added to Cargo and is not executable implementation.']}
    write_json(OUT / 'report.json', report)
    print(json.dumps(report, ensure_ascii=False, indent=2), flush=True)

if __name__ == '__main__':
    main()
