"""Annotate source imports and function dependencies with PDB/native evidence.

Inline nesting is distinct from physical machine-code calls. Neither restores
the author's use syntax, visibility, runtime dispatch, or missing Rust ABI.
"""
from collections import Counter, defaultdict, OrderedDict
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import subprocess
import struct
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
SKILL = Path('/Users/yuluo/.codex/skills/native-evidence-pack/scripts/collect.py')
spec = importlib.util.spec_from_file_location('native_pack', SKILL)
pack = importlib.util.module_from_spec(spec); spec.loader.exec_module(pack)
BEGIN = '// BEGIN GENERATED DEPENDENCY ANNOTATIONS'
END = '// END GENERATED DEPENDENCY ANNOTATIONS'

def rows(path):
    with gzip.open(path, 'rt') as f:
        for line in f: yield json.loads(line)

def save(path, value):
    tmp = path.with_suffix(path.suffix + '.tmp')
    tmp.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n'); tmp.replace(path)

def source_path(path):
    path = path.replace('\\', '/')
    return path.split('country-slice-private/', 1)[1] if 'country-slice-private/' in path else None

def body(doc): return next((v for k, v in doc.items() if k != 'Kind' and isinstance(v, dict)), {})

class EvidenceWriter:
    def __init__(self, version, paths):
        self.version = version; self.files = OrderedDict()
        for path in paths:
            with gzip.open(ROOT / version / (path + '.dependencies.jsonl.gz'), 'wb', compresslevel=1): pass
    def write(self, path, row):
        if path not in self.files:
            if len(self.files) >= 48: self.files.popitem(last=False)[1].close()
            self.files[path] = gzip.open(ROOT / self.version / (path + '.dependencies.jsonl.gz'), 'ab', compresslevel=1)
        f = self.files.pop(path); self.files[path] = f
        f.write((json.dumps(row, ensure_ascii=False, separators=(',', ':')) + '\n').encode())
    def close(self):
        for f in self.files.values(): f.close()

class Annotation:
    def __init__(self, version):
        self.v = version
        self.out = ROOT / '_dependencies' / version; self.out.mkdir(parents=True, exist_ok=True)
        self.pdb = ROOT / '_pdb' / version
        self.baseline = json.loads((PROJECT / 'tracking/baselines.json').read_text())['baselines'][version]
        self.dependencies = json.loads((PROJECT / 'evidence/multi-version' / f'{version}-dependencies.json').read_text())
        self.policy = json.loads((PROJECT / 'tracking/dependency-exclusions.json').read_text())
        self.sources = json.loads((self.pdb / 'source-files.json').read_text())
        self.paths = {r['path'] for r in self.sources}
        self.packages = defaultdict(list)
        for pkg in self.dependencies['packages']: self.packages[pkg['name'].replace('-', '_')].append(pkg)
        self.funcs = {path: {} for path in self.paths}
        self.selected = defaultdict(list); self.physical = defaultdict(list); self.procedures = {}
        self.types = {}; self.name_cache = {}; self.symbols_by_va = defaultdict(set)
        self.inline_names = Counter(); self.edge_counts = Counter(); self.gaps = []
        self.calls = []; self.native_summary = None
        self.writer = EvidenceWriter(version, self.paths)

    def function(self, path, name, tid):
        key = (name, tid)
        self.funcs[path].setdefault(key, {'name': name, 'type_index': tid, 'dependencies': {}, 'physical_instances': set(), 'inline_instances': set()})
        return key

    def load_sources(self):
        for path in sorted(self.paths):
            for r in rows(ROOT / self.v / (path + '.pdb.jsonl.gz')):
                if r['event'] == 'inlinee': self.function(path, r['name'], r.get('type_index'))
                elif r['event'] == 'symbol' and r.get('class') in ('procedure', 'inline_site'):
                    tid = r.get('resolved_tpi', r.get('type_index')); fn = self.function(path, r['name'], tid)
                    entry = (path, fn, r['record_offset'])
                    location = (r['module'], r['record_offset'])
                    if entry not in self.selected[location]: self.selected[location].append(entry)
                    if r['class'] == 'procedure':
                        self.procedures[location] = r
                        if entry not in self.physical[r['va']]: self.physical[r['va']].append(entry)
                        self.funcs[path][fn]['physical_instances'].add(location)
                    else: self.funcs[path][fn]['inline_instances'].add(location)
        for row in rows(self.pdb / 'types.jsonl.gz'):
            if row['namespace'] == 'IPI': self.types[row['index']] = row.get('decoded', {})
        for r in rows(self.pdb / 'global-symbols.jsonl.gz'):
            d = r.get('decoded', {}); b = body(d)
            if d.get('Kind') == 'S_PUB32' and 'Function' in b.get('Flags', []):
                # YAML may omit zero-valued fields; the CodeView record retains them.
                _, offset, section = struct.unpack_from('<IIH', bytes.fromhex(r['raw_hex']), 4)
                va = self.address(section, offset)
                if va is not None: self.symbols_by_va[va].add(b['Name'])

    def address(self, section, offset):
        secs = self.baseline['pe']['sections']
        if not 1 <= section <= len(secs): return None
        return self.baseline['pe']['image_base'] + secs[section - 1]['rva'] + offset

    def ipi_name(self, idx):
        if idx in self.name_cache: return self.name_cache[idx]
        doc = self.types.get(idx, {}); b = body(doc); name = b.get('Name', f'IPI_{idx:X}')
        scope = b.get('ParentScope', 0); names = []; seen = set()
        while scope and scope not in seen:
            seen.add(scope); s = body(self.types.get(scope, {}))
            if 'String' in s: names.append(s['String'])
            scope = s.get('Id', 0)
        if names: name = ''.join(reversed(names)) + '::' + name
        self.name_cache[idx] = name
        return name

    def classify(self, name, path=None):
        root_match = re.match(r'^([A-Za-z_][A-Za-z0-9_]*)::', name)
        crate = root_match[1] if root_match else None
        result = {'crate': crate, 'source_file': path, 'visibility': 'unknown_from_debug_metadata'}
        packages = self.packages.get(crate, [])
        if crate in ('std', 'core', 'alloc'):
            result['ownership'] = 'rust_standard_library'
            result['packages'] = []; return result
        path = path.replace('\\', '/') if path else ''
        m = re.search(r'/registry/src/[^/]+/(.+)-(\d[^/]*)/', path)
        if m:
            exact = [p for p in packages if p['name'] == m[1] and p['version'] == m[2]]
            result['ownership'] = 'registry_dependency_definition'
            result['packages'] = [{'name': p['name'], 'version': p['version'], 'source': p.get('source')} for p in exact]
            if not exact: result['packages'] = [{'name': m[1], 'version': m[2], 'source': 'PDB_registry_path_only'}]
        elif '/git/checkouts/' in path:
            result['ownership'] = 'git_dependency_definition'
            result['packages'] = [{'name': p['name'], 'version': p['version'], 'source': p.get('source')} for p in packages]
            result['pdb_git_path'] = path
        elif source_path(path):
            private = source_path(path); result['ownership'] = 'private_source_ownership_candidate'
            parts = private.split('/'); pkg = parts[2] if len(parts) > 2 and parts[1] == 'systems' else parts[1] if len(parts) > 1 else ''
            if pkg in self.policy['known_vendored_dependency_names']: result['ownership'] = 'confirmed_vendored_dependency'
            result['packages'] = [{'name': p['name'], 'version': p['version'], 'source': p.get('source', 'local_or_path')} for p in packages]
        elif packages:
            result['ownership'] = 'package_namespace_candidate'
            result['packages'] = [{'name': p['name'], 'version': p['version'], 'source': p.get('source', 'local_or_path')} for p in packages]
        else:
            result['ownership'] = 'unresolved_namespace'; result['packages'] = []
        return result

    def add_edge(self, path, fn, relationship, callee, evidence, src=None, ambiguity=None):
        classification = self.classify(callee, src)
        key = (relationship, callee, src or '', tuple(ambiguity or []))
        deps = self.funcs[path][fn]['dependencies']
        if key not in deps:
            deps[key] = {'relationship': relationship, 'function': callee, **classification, 'instances': 0, 'first_evidence': evidence}
            if ambiguity: deps[key]['target_aliases'] = ambiguity
        deps[key]['instances'] += 1
        self.edge_counts[relationship] += 1
        self.writer.write(path, {'caller': {'name': fn[0], 'type_index': fn[1]}, 'relationship': relationship, 'callee': callee, **classification, 'evidence': evidence, 'target_aliases': ambiguity or []})

    def inline_edges(self):
        relevant = sorted({module for module, _ in self.selected})
        start = time.monotonic()
        for i, mi in enumerate(relevant):
            archive = self.pdb / 'modules' / f'{mi:05d}.jsonl.gz'
            il = {}; checks = {}; ancestry = {}; physical_proc = None
            for r in rows(archive):
                if r['event'] == 'checksum': checks[r['file_id']] = r
                elif r['event'] == 'inlinee': il[r['inlinee']] = r
                elif r['event'] == 'symbol':
                    off = r['record_offset']; own = self.selected.get((mi, off), [])
                    if r.get('class') == 'procedure':
                        physical_proc = r
                        ancestry[off] = own
                        addr = self.address(r['section'], r['section_offset'])
                        if addr is not None: self.symbols_by_va[addr].add(r['name'])
                    elif r.get('class') == 'inline_site':
                        parents = ancestry.get(r['parent'])
                        if parents is None:
                            self.gaps.append({'kind': 'missing_inline_parent', 'module': mi, 'offset': off, 'parent': r['parent']}); parents = []
                        name = self.ipi_name(r['inlinee']); source = checks.get(il.get(r['inlinee'], {}).get('file_id'), {}).get('path')
                        for path, fn, ancestor_offset in parents:
                            relationship = 'pdb_inline_direct' if r['parent'] == ancestor_offset else 'pdb_inline_transitive'
                            self.add_edge(path, fn, relationship, name, {'module': mi, 'record_offset': off, 'parent_record_offset': r['parent'],
                                'ancestor_record_offset': ancestor_offset, 'inlinee': r['inlinee'], 'source_line': il.get(r['inlinee'], {}).get('line'),
                                'archive': archive.relative_to(ROOT).as_posix()}, source)
                        ancestry[off] = parents + own
                    elif r.get('class') == 'scope_end': pass
                    else:
                        # Block scopes may be inline parents in C/C++ records.
                        d = r.get('yaml', '')
                        if 'Kind:            S_BLOCK32' in d:
                            parent = re.search(r'Parent:\s*(\d+)', d)
                            if parent: ancestry[off] = ancestry.get(int(parent[1]), [])
            if i % 250 == 0: print(json.dumps({'version': self.v, 'phase': 'inline_edges', 'modules': i + 1, 'total': len(relevant), 'elapsed_seconds': round(time.monotonic() - start, 1)}), flush=True)

    def native_calls(self):
        binary = Path(self.baseline['binary']['path']); pdb = Path(self.baseline['pdb']['path'])
        assert pack.digest(binary) == self.baseline['binary']['sha256']
        assert pack.digest(pdb) == self.baseline['pdb']['sha256']
        identity = pack.pdb_identity(pdb); pe = pack.PE(binary)
        assert any(r['guid'] == identity['guid'] and r['age'] == identity['age'] for r in pe.rsds)
        assert pe.image_base == self.baseline['pe']['image_base']
        spans = {}; native_ranges = defaultdict(list)
        for r in self.procedures.values():
            va = r['va']; span = (va, r['code_size'])
            try: section = pe.section_for(r['rva'], r['code_size'], executable=True)
            except pack.PackError as exc:
                self.gaps.append({'kind': 'non_file_backed_procedure', 'va': va, 'size': r['code_size'], 'error': str(exc)}); continue
            spans[span] = {'va': va, 'length': r['code_size'], 'file_offset': section['offset'] + r['rva'] - section['rva']}
            native_ranges[va].append(r)
        rangefile = self.out / 'native-ranges.json'; save(rangefile, list(spans.values()))
        out = self.out / 'native-calls.jsonl.gz'
        with gzip.open(out, 'wb', compresslevel=1) as archive, (self.out / 'native-reader.log').open('wb') as log:
            proc = subprocess.Popen([str(ROOT / '_tools/extract_calls'), str(binary), str(rangefile)], stdout=subprocess.PIPE, stderr=log)
            image = binary.read_bytes()
            for line in proc.stdout:
                row = json.loads(line); archive.write(line)
                if row['event'] in ('call', 'external_jump'):
                    rva = row['va'] - pe.image_base; section = pe.section_for(rva, row['size'], executable=True)
                    at = section['offset'] + rva - section['rva']
                    assert image[at:at + row['size']].hex() == row['raw_hex']
                    if row['direct'] and row['raw_hex'].startswith('e8'):
                        displacement = int.from_bytes(bytes.fromhex(row['raw_hex'])[1:], 'little', signed=True)
                        assert row['target_va'] == row['va'] + row['size'] + displacement
                    self.calls.append(row)
                elif row['event'] == 'decode_gap': self.gaps.append(row)
                elif row['event'] == 'complete': self.native_summary = row
            if proc.wait(): raise RuntimeError('Native instruction reader failed')
        assert self.native_summary and self.native_summary['ranges'] == len(spans)
        # Names from retained PDB procedures take priority; public aliases remain explicit.
        mangled = sorted({n for call in self.calls if call.get('direct') for n in self.symbols_by_va.get(call['target_va'], []) if n.startswith(('_ZN', '_R', '?'))})
        demangled = {}
        if mangled:
            result = subprocess.run(['/opt/homebrew/opt/llvm/bin/llvm-cxxfilt', '--no-strip-underscore'], input='\n'.join(mangled) + '\n', text=True, capture_output=True, check=True)
            output = result.stdout.splitlines(); assert len(output) == len(mangled)
            demangled = dict(zip(mangled, output))
        save(self.out / 'demangled-targets.json', demangled)
        for row in self.calls:
            entries = self.physical[row['range_va']]
            names = sorted({re.sub(r'::h[0-9a-f]{16}$', '', demangled.get(n, n)) for n in self.symbols_by_va.get(row.get('target_va'), [])})
            preferred = [n for n in names if '::' in n and not n.startswith(('_ZN', '_R', '?'))]
            callee = preferred[0] if len(preferred) == 1 else 'alias_set_at_' + hex(row['target_va']) if row.get('direct') and names else 'unresolved_direct_target_' + hex(row['target_va']) if row.get('direct') else 'unresolved_indirect_call'
            relationship = 'native_direct_call_in_physical_body' if row['event'] == 'call' and row['direct'] else 'native_indirect_call_unresolved' if row['event'] == 'call' else 'native_external_jump_candidate'
            for path, fn, off in entries:
                self.add_edge(path, fn, relationship, callee, {**row, 'scope': 'physical procedure may contain inlined library code; not automatically a direct source-level call'}, ambiguity=names if len(preferred) != 1 else None)
        save(self.out / 'native-input.json', {'binary': self.baseline['binary'], 'pdb': self.baseline['pdb'], 'debug_identity': identity,
            'extractor_sha256': pack.digest(ROOT / '_tools/extract_calls'), 'llvm_version': subprocess.run(['/opt/homebrew/opt/llvm/bin/llvm-config', '--version'], capture_output=True, text=True, check=True).stdout.strip(),
            'requests': len(spans), 'observed_binary_bytes_verified': True, 'target_executed': False})

    def render(self):
        files = []; edge_summary = Counter(); functions_with_facts = 0; import_roots = 0
        for path in sorted(self.paths):
            functions = self.funcs[path]; libs = defaultdict(list)
            for fn in functions.values():
                for dep in fn['dependencies'].values():
                    if dep['crate'] and dep['ownership'] != 'unresolved_namespace': libs[dep['crate']].append(dep)
            imports = []
            for crate, evidence in sorted(libs.items()):
                identities = {json.dumps(p, sort_keys=True) for dep in evidence for p in dep['packages']}
                imports.append({'crate': crate, 'use_candidate': f'use {crate};', 'original_use_syntax_recovered': False,
                    'packages': [json.loads(p) for p in sorted(identities)], 'ownership_evidence': sorted({d['ownership'] for d in evidence}),
                    'source_files': sorted({d['source_file'] for d in evidence if d['source_file']})})
            import_roots += len(imports)
            rendered = [BEGIN, '// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。',
                '// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。']
            if not imports: rendered.append('// 当前没有可证实的 crate 引用信息。')
            for entry in imports:
                rendered.append('// ' + entry['use_candidate'] + ' // namespace candidate')
                rendered.append('//   package identities: ' + json.dumps(entry['packages'], ensure_ascii=False, separators=(',', ':')))
            rendered += [END, '']
            source = ROOT / self.v / path
            text = source.read_text()
            assert text.startswith('// Generated from PDB metadata')
            text = re.sub(re.escape(BEGIN) + r'.*?' + re.escape(END) + r'\n?', '', text, flags=re.S)
            output = []; active = None; tpi = None; inserted = False; annotated = 0
            def add_function_notes():
                nonlocal annotated
                if active is None: return
                key = (active, tpi); fn = functions.get(key)
                if fn is None: raise RuntimeError(f'Missing function mapping: {self.v} {path} {key}')
                annotated += 1
                output.extend([BEGIN, '// 函数体依赖注解（实现尚未恢复）：', '// {'])
                deps = sorted(fn['dependencies'].values(), key=lambda d: (d['relationship'], d['function'], d['source_file'] or ''))
                if not deps: output.append('//   没有已归属的调用/内联证据；不据此认定函数没有依赖。')
                for dep in deps:
                    output.append('//   [' + dep['relationship'] + '] ' + dep['function'])
                    output.append('//     owner=' + dep['ownership'] + '; count=' + str(dep['instances']))
                    if dep['packages']: output.append('//     packages=' + json.dumps(dep['packages'], ensure_ascii=False, separators=(',', ':')))
                    if dep['source_file']: output.append('//     definition_source=' + dep['source_file'])
                    if dep.get('target_aliases'):
                        output.append('//     target_aliases=' + str(len(dep['target_aliases'])) + ' candidates; complete list: ' + Path(path).name + '.dependencies.json')
                    output.append('//     evidence=' + json.dumps(dep['first_evidence'], ensure_ascii=False, separators=(',', ':')))
                output.extend(['//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。', '// }', END, ''])
            for line in text.splitlines():
                if line.startswith('// ------------------------------------------------------------------') or line.startswith('// === 完整可用 TPI'):
                    add_function_notes(); active = None; tpi = None
                if line.startswith('// 函数符号: '): active = line[len('// 函数符号: '):]
                elif line.startswith('// TPI: '):
                    value = line[len('// TPI: '):]; tpi = int(value, 16) if value.startswith('0x') else None
                output.append(line)
                if not inserted and line.startswith('// 编译单元:'):
                    output.extend(rendered); inserted = True
            add_function_notes()
            assert inserted and annotated == len(functions)
            tmp = source.with_suffix('.rs.tmp'); tmp.write_text('\n'.join(output) + '\n'); tmp.replace(source)
            serial_functions = []
            for fn in functions.values():
                deps = list(fn['dependencies'].values())
                if deps: functions_with_facts += 1
                edge_summary.update(d['relationship'] for d in deps)
                serial_functions.append({**fn, 'dependencies': deps, 'physical_instances': sorted(fn['physical_instances']), 'inline_instances': sorted(fn['inline_instances'])})
            record = {'version': self.v, 'path': path, 'imports': imports, 'functions': serial_functions,
                'binary_sha256': self.baseline['binary']['sha256'], 'pdb_sha256': self.baseline['pdb']['sha256'],
                'author_use_syntax_recovered': False, 'function_bodies_recovered': False, 'direct_source_call_graph_complete': False}
            save(source.with_suffix('.rs.dependencies.json'), record)
            files.append({'path': path, 'functions': len(functions), 'imports': len(imports), 'dependency_records': sum(len(f['dependencies']) for f in functions.values())})
        report = {'version': self.v, 'annotated_files': len(self.paths), 'annotated_functions': sum(len(f) for f in self.funcs.values()),
            'functions_with_dependency_evidence': functions_with_facts, 'use_namespace_candidates': import_roots,
            'aggregate_dependency_records': dict(edge_summary), 'dependency_instance_records': dict(self.edge_counts),
            'native_extraction': self.native_summary, 'gaps': len(self.gaps), 'binary_sha256': self.baseline['binary']['sha256'], 'pdb_sha256': self.baseline['pdb']['sha256'],
            'author_import_syntax_unknown': True, 'runtime_indirect_targets_unknown': True, 'production_project_modified': False}
        save(self.out / 'report.json', report); save(self.out / 'files.json', files); save(self.out / 'gaps.json', self.gaps)
        print(json.dumps(report), flush=True)
        return report

    def run(self):
        self.load_sources(); self.inline_edges(); self.native_calls(); self.writer.close()
        return self.render()

def main():
    reports = [Annotation(v).run() for v in sys.argv[1:] or 'ABC']
    save(ROOT / 'dependency-annotation-report.json', {'versions': reports, 'status': 'complete_available_evidence', 'production_project_modified': False,
        'limitations': ['use statements are namespace candidates, not recovered author syntax', 'inline tree nesting is not a complete source-level call graph',
            'physical body calls can originate from library inline code', 'target aliases are retained, not arbitrarily chosen',
            'external jumps are candidate tail transfers', 'indirect targets and bodies without PDB instances remain unknown']})

if __name__ == '__main__': main()
