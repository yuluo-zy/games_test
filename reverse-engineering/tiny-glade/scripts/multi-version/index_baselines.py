"""Stream identity/PDB metadata into bounded version records; never run the game."""
from pathlib import Path
from collections import defaultdict, Counter
import hashlib, json, re, struct, subprocess, tomllib, uuid

ROOT = Path(__file__).resolve().parents[2]
GAME = ROOT.parents[2]
LLVM = Path('/opt/homebrew/Cellar/llvm/22.1.2/bin')
OUT = ROOT / 'evidence/multi-version'
BASELINES = {
    'A': GAME / 'Tiny Glade',
    'B': GAME / 'ljxsj_92385/Tiny Glade',
    'C': GAME / 'tiny.glade/XJ07036/Tiny Glade v1.16.0',
}

def digest(path):
    with path.open('rb') as file:
        return hashlib.file_digest(file, 'sha256').hexdigest()

def save(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')

def pe_metadata(path):
    # Read only PE headers, section records and CodeView debug directory.
    with path.open('rb') as f:
        dos = f.read(64)
        assert dos[:2] == b'MZ'
        offset = struct.unpack_from('<I', dos, 60)[0]
        f.seek(offset)
        coff = f.read(24)
        assert coff[:4] == b'PE\0\0'
        count = struct.unpack_from('<H', coff, 6)[0]
        optional_size = struct.unpack_from('<H', coff, 20)[0]
        optional = f.read(optional_size)
        assert struct.unpack_from('<H', optional)[0] == 0x20b
        sections = []
        for _ in range(count):
            raw = f.read(40)
            vs, rva, size, pointer = struct.unpack_from('<IIII', raw, 8)
            sections.append({'name': raw[:8].rstrip(b'\0').decode(), 'rva': rva,
                             'virtual_size': vs, 'raw_size': size, 'raw_pointer': pointer})
        def file_offset(rva):
            for section in sections:
                if section['rva'] <= rva < section['rva'] + section['raw_size']:
                    return section['raw_pointer'] + rva - section['rva']
            raise ValueError('RVA is not file backed')
        debug_rva, debug_size = struct.unpack_from('<II', optional, 112 + 6 * 8)
        f.seek(file_offset(debug_rva))
        directory = f.read(debug_size)
        codeview = []
        for i in range(0, len(directory), 28):
            _, _, _, _, ty, size, _, pointer = struct.unpack_from('<IIHHIIII', directory, i)
            if ty != 2:
                continue
            f.seek(pointer)
            record = f.read(size)
            if record[:4] == b'RSDS':
                codeview.append({'guid': str(uuid.UUID(bytes_le=record[4:20])).upper(),
                    'age': struct.unpack_from('<I', record, 20)[0],
                    'pdb_path': record[24:].rstrip(b'\0').decode(errors='replace')})
    return {'image_base': struct.unpack_from('<Q', optional, 24)[0],
            'coff_timestamp': struct.unpack_from('<I', coff, 8)[0],
            'sections': sections, 'codeview': codeview}

def pdb_lines(path, flag):
    process = subprocess.Popen([str(LLVM / 'llvm-pdbutil'), 'dump', flag, str(path)],
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    yield from process.stdout
    if process.wait():
        raise RuntimeError(process.stderr.read())

def sources(path):
    rows = defaultdict(set)
    dependency_paths = set()
    rustc_commits = set()
    for line in pdb_lines(path, '--files'):
        match = re.match(r'- \(SHA-256: ([A-Fa-f0-9]+)\) (.+)', line)
        if not match:
            continue
        checksum, name = match.groups()
        if 'country-slice-private/' in name and name.endswith('.rs'):
            rows[name.split('country-slice-private/', 1)[1]].add(checksum.lower())
        if '/.cargo/' in name:
            dependency_paths.add(name)
        commit = re.search(r'/rustc/([a-f0-9]{40})/', name)
        if commit:
            rustc_commits.add(commit[1])
    return ([{'path': name, 'sha256': sorted(checks)} for name, checks in sorted(rows.items())],
            sorted(dependency_paths), sorted(rustc_commits))

def public_functions(path, pe):
    active = None
    result = []
    for line in pdb_lines(path, '--publics'):
        if 'S_PUB32' in line:
            match = re.search(r'`([^`]+)`', line)
            active = match[1] if match else None
        elif active and 'addr =' in line:
            name, active = active, None
            # Game methods only; exclude dependency generic wrappers containing game types.
            if not name.startswith(('_ZN12country_core', '_ZN11system_roof',
                    '_ZN23system_wall_constructor', '_ZN16system_decorator', '_ZN10tiny_glade')):
                continue
            if 'flags = function' not in line:
                continue
            match = re.search(r'addr = (\d+):(\d+)', line)
            if not match or not 1 <= int(match[1]) <= len(pe['sections']):
                continue
            rva = pe['sections'][int(match[1])-1]['rva'] + int(match[2])
            result.append({'name': name, 'canonical_candidate': re.sub(r'17h[a-f0-9]{16}E$', 'E', name),
                           'rva': hex(rva), 'va': hex(pe['image_base'] + rva)})
    return result

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    identities, version_sources, version_symbols = {}, {}, {}
    for version, directory in BASELINES.items():
        target = directory / ('tiny-glade.rne' if version == 'B' else 'tiny-glade.exe')
        pdb = directory / 'tiny_glade.pdb'
        pe = pe_metadata(target)
        summary = ''.join(pdb_lines(pdb, '--summary'))
        (OUT / f'{version}-pdb-summary.txt').write_text(summary)
        guid = re.search(r'GUID:\s*\{([^}]+)\}', summary)[1].upper()
        age = int(re.search(r'Age:\s*(\d+)', summary)[1])
        assert any(c['guid'] == guid and c['age'] == age for c in pe['codeview'])
        own, dependency_paths, rustc = sources(pdb)
        symbols = public_functions(pdb, pe)
        save(OUT / f'{version}-sources.json', {'source': str(pdb), 'files': own,
            'dependency_source_paths': dependency_paths, 'rustc_commits': rustc})
        save(OUT / f'{version}-functions.json', {'baseline_id': version, 'functions': symbols,
            'scope': 'Selected direct game-method public symbols; excludes generic wrappers; not a complete function inventory'})
        lock = directory / 'build-info/Cargo.lock'
        packages = tomllib.loads(lock.read_text())['package'] if lock.exists() else []
        save(OUT / f'{version}-dependencies.json', {'baseline_id': version,
            'lockfile': str(lock) if lock.exists() else None,
            'lock_sha256': digest(lock) if lock.exists() else None,
            'packages': packages, 'unavailable': not lock.exists(),
            'feature_sets_recovered': False, 'patches_excluded': False})
        if lock.exists():
            (OUT / f'{version}-original-Cargo.lock').write_bytes(lock.read_bytes())
        identities[version] = {'baseline_id': version, 'root': str(directory),
            'binary': {'path': str(target), 'bytes': target.stat().st_size, 'sha256': digest(target)},
            'pdb': {'path': str(pdb), 'bytes': pdb.stat().st_size, 'sha256': digest(pdb), 'guid': guid, 'age': age},
            'pdb_matches_binary': True, 'pe': pe, 'game_source_paths': len(own),
            'dependency_package_count': len(packages) if lock.exists() else None,
            'toolchain_info': json.loads((directory/'build-info/toolchain-info.json').read_text())
                if (directory/'build-info/toolchain-info.json').exists() else None,
            'analysis_database': {'status': 'not_bound', 'path': None},
            'target_executed': False}
        version_sources[version], version_symbols[version] = own, symbols
        print(json.dumps({'baseline': version, 'pdb_match': True, 'source_paths': len(own),
                          'selected_symbols': len(symbols), 'packages':len(packages)}), flush=True)
    save(ROOT / 'tracking/baselines.json', {'schema_version': 1, 'target_execution': False,
        'scope': 'Local artifact identity, not verified official origin', 'baselines': identities})
    source_map = defaultdict(dict)
    for version, rows in version_sources.items():
        for row in rows:
            source_map[row['path']][version] = row['sha256']
    symbol_map = defaultdict(lambda: defaultdict(list))
    for version, rows in version_symbols.items():
        for row in rows:
            symbol_map[row['canonical_candidate']][version].append(row)
    save(ROOT / 'tracking/version-map.json', {'schema_version': 1,
        'mapping_status': 'candidate_only', 'behavior_equivalence_verified': False,
        'source_files': [{'path': p, 'versions': v} for p,v in sorted(source_map.items())],
        'functions': [{'canonical_candidate': p, 'versions': dict(v)} for p,v in sorted(symbol_map.items())]})
    modules = defaultdict(lambda: defaultdict(int))
    for version, rows in version_sources.items():
        for row in rows:
            parts = row['path'].split('/')
            if len(parts) > 2 and parts[0] == 'crates':
                module = '/'.join(parts[:3]) if parts[1] == 'systems' else '/'.join(parts[:2])
            else:
                module = parts[0]
            modules[module][version] += 1
    save(ROOT / 'tracking/module-coverage.json', {'schema_version': 1,
        'scope': 'PDB source-path inventory; complete functionality denominator is not established',
        'overall_completion_percent': None,
        'modules': [{'path': p, 'source_counts': dict(v), 'source_status': 'not_assessed',
            'system_acceptance': 'not_verified'} for p,v in sorted(modules.items())],
        'existing_verified_local_progress': 'tracking/project.json'})

if __name__ == '__main__':
    main()
