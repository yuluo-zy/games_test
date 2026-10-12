"""Extract all records with LLVM and annotate the isolated source skeleton.

No missing Rust ABI is invented. Raw PDB, type/symbol records and C13 bytes
remain available, including records that cannot be assigned to a source file.
"""
from __future__ import annotations
import bisect
from collections import Counter, defaultdict, OrderedDict
import gzip
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import time
import yaml

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
EXE = ROOT / '_tools/export_pdb'
LLVM = '/opt/homebrew/opt/llvm/bin/llvm-pdbutil'
LOADER = yaml.CSafeLoader
MARKER = '// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.'

def digest(p):
    h = hashlib.sha256()
    with p.open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''): h.update(block)
    return h.hexdigest()

def atomic_json(path, value):
    tmp = path.with_suffix(path.suffix + '.tmp')
    tmp.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')
    tmp.replace(path)

def decode(row):
    if 'yaml' not in row: return {}
    try: return yaml.load(row['yaml'], Loader=LOADER) or {}
    except yaml.YAMLError as exc:
        row['yaml_parse_error'] = str(exc)
        return {}

def norm(path):
    path = path.replace('\\', '/')
    return path.split('country-slice-private/', 1)[-1] if 'country-slice-private/' in path else None

def body(doc):
    return next((v for k, v in doc.items() if k != 'Kind' and isinstance(v, dict)), {})

class Writer:
    def __init__(self, version, paths):
        self.version = version
        self.handles = OrderedDict()
        self.counts = Counter()
        for rel in paths:
            p = ROOT / version / (rel + '.pdb.jsonl.gz')
            with gzip.open(p, 'wb', compresslevel=1): pass
    def put(self, rel, row):
        if rel not in self.handles:
            if len(self.handles) >= 48: self.handles.popitem(last=False)[1].close()
            self.handles[rel] = gzip.open(ROOT / self.version / (rel + '.pdb.jsonl.gz'), 'ab', compresslevel=1)
        f = self.handles.pop(rel); self.handles[rel] = f
        f.write((json.dumps(row, ensure_ascii=False, separators=(',', ':')) + '\n').encode())
        self.counts[rel] += 1
    def close(self):
        for f in self.handles.values(): f.close()
        self.handles.clear()

class Version:
    def __init__(self, v, baseline, manifest, *, source_mapper=None, source_entries=None, report_dir=None):
        self.v = v; self.baseline = baseline
        self.source_mapper = source_mapper or norm
        self.dir = report_dir or ROOT / '_pdb' / v
        self.dir.mkdir(parents=True, exist_ok=True)
        self.sources = source_entries if source_entries is not None else {r['original_path']: r for r in manifest['files'] if r['version'] == v and r['kind'] == 'empty_source_placeholder'}
        self.writer = Writer(v, self.sources)
        self.types = {'TPI': {}, 'IPI': {}}
        self.aliases = defaultdict(list)
        self.functions = defaultdict(dict)
        self.ipi_context_refs = defaultdict(set)
        self.files = {p: {'modules': [], 'original_paths': set(), 'checksums': set(), 'line_blocks': 0, 'inline_sites': 0, 'procedure_instances': 0} for p in self.sources}
        self.events = Counter(); self.symbols = Counter(); self.errors = []
        self.modules = []; self.complete = None
        self.unassigned_procedures = 0
        self.mapping_counts = Counter()

    def type_doc(self, ns, idx): return self.types[ns].get(idx, {}).get('decoded', {})

    def ipi_function(self, idx):
        doc = self.type_doc('IPI', idx); b = body(doc)
        name = b.get('Name', f'IPI_{idx:X}')
        if doc.get('Kind') == 'LF_FUNC_ID':
            scope_idx = b.get('ParentScope', 0); scopes = []; seen = set()
            while scope_idx and scope_idx not in seen:
                seen.add(scope_idx); s = body(self.type_doc('IPI', scope_idx))
                if 'String' in s: scopes.append(s['String'])
                scope_idx = s.get('Id', 0)
            scope = ''.join(reversed(scopes))
            if scope: name = scope + '::' + name
        elif doc.get('Kind') == 'LF_MFUNC_ID':
            parent = body(self.type_doc('TPI', b.get('ClassType', 0))).get('Name', '?')
            name = parent + '::' + name
        return name, b.get('FunctionType')

    def type_graph(self, start):
        result = {}; pending = list(start)
        type_keys = {'ReturnType', 'ArgumentList', 'ReferentType', 'ModifiedType', 'ClassType', 'ThisType', 'FieldList', 'DerivationList', 'VTableShape', 'ElementType', 'IndexType', 'UnderlyingType', 'Type', 'TypeIndex', 'MemberType', 'MethodList', 'ContinuationIndex', 'ArgIndices'}
        def refs(obj, key=''):
            if isinstance(obj, dict):
                for k, value in obj.items(): yield from refs(value, k)
            elif isinstance(obj, list):
                for value in obj: yield from refs(value, key)
            elif isinstance(obj, int) and key in type_keys and obj >= 0x1000: yield obj
        while pending:
            idx = pending.pop()
            if idx in result or idx not in self.types['TPI']: continue
            result[idx] = self.types['TPI'][idx]
            pending.extend(refs(result[idx].get('decoded', {})))
        return result

    def add_function(self, path, name, tid, kind, ref, line=None, ipi=None):
        key = (name, tid)
        functions = self.functions[path]
        if key not in functions:
            functions[key] = {'name': name, 'tpi': tid, 'kinds': set(), 'instances': 0, 'lines': set(), 'first_ref': ref, 'ipi': set(), 'addresses': set()}
        item = functions[key]; item['kinds'].add(kind); item['instances'] += 1
        if line is not None: item['lines'].add(line)
        if ipi is not None:
            item['ipi'].add(ipi)
            self.ipi_context_refs[path].add(ipi)
        if ref.get('rva') is not None: item['addresses'].add(ref['rva'])

    def ipi_graph(self, starts):
        result = {}; pending = list(starts)
        def refs(value, key=''):
            if isinstance(value, dict):
                for k, item in value.items(): yield from refs(item, k)
            elif isinstance(value, list):
                for item in value: yield from refs(item, key)
            elif isinstance(value, int) and key in {'Id', 'ParentScope', 'ArgIndices', 'StringIndices', 'SourceFile'} and value >= 0x1000:
                yield value
        while pending:
            index = pending.pop()
            if index in result or index not in self.types['IPI']: continue
            result[index] = self.types['IPI'][index]
            pending.extend(refs(result[index].get('decoded', {})))
        return result

    def address(self, section, offset):
        sections = self.baseline['pe']['sections']
        if not 1 <= section <= len(sections): return {'rva': None, 'va': None}
        rva = sections[section - 1]['rva'] + offset
        return {'rva': rva, 'va': self.baseline['pe']['image_base'] + rva}

    def process_module(self, rows, archive):
        m = rows[0]; mi = m['index']; module_ref = archive.relative_to(ROOT).as_posix()
        available = {self.source_mapper(p) for p in m['files']} & self.sources.keys()
        for p in available:
            self.files[p]['modules'].append(mi)
            self.writer.put(p, {'event': 'module_reference', 'module': mi, 'archive': module_ref, 'name': m['name'], 'object': m['object'], 'stream': m['stream']})
        if not available: return
        checks = {r['file_id']: r for r in rows if r['event'] == 'checksum'}
        def file_path(fid): return self.source_mapper(checks.get(fid, {}).get('path', ''))
        for check in checks.values():
            path = self.source_mapper(check.get('path', ''))
            if path in self.sources:
                self.files[path]['original_paths'].add(check['path'])
                self.files[path]['checksums'].add(check['checksum'])
                self.writer.put(path, check)
        lineblocks = defaultdict(list); il = {}
        for r in rows:
            if r['event'] == 'lines':
                path = file_path(r['file_id'])
                lineblocks[r['section']].append((r['section_offset'], r['section_offset'] + r['code_size'], path, r))
                if path in self.sources:
                    self.files[path]['line_blocks'] += 1
                    self.writer.put(path, {**r, **self.address(r['section'], r['section_offset'])})
            elif r['event'] == 'inlinee':
                il[r['inlinee']] = r
                path = file_path(r['file_id'])
                if path in self.sources:
                    name, tid = self.ipi_function(r['inlinee'])
                    self.add_function(path, name, tid, 'inlinee_declaration', {'module': mi, 'inlinee': r['inlinee']}, r['line'], r['inlinee'])
                    self.writer.put(path, {**r, 'name': name, 'type_index': tid, 'source_path': path})
        for blocks in lineblocks.values(): blocks.sort(key=lambda b: b[0])
        starts = {sec: [b[0] for b in blocks] for sec, blocks in lineblocks.items()}
        max_ends = {}
        for sec, blocks in lineblocks.items():
            running = 0; ends = []
            for block in blocks:
                running = max(running, block[1]); ends.append(running)
            max_ends[sec] = ends
        proc = None; proc_paths = set(); proc_line = {}; scopes = {}; current_scope = None
        for r in rows:
            if r['event'] != 'symbol': continue
            off = r['record_offset']; cls = r.get('class')
            if cls == 'procedure':
                proc = r; current_scope = off; proc_paths = set(); proc_line = {}
                sec = r['section']; start = r['section_offset']; end = start + r['code_size']
                blocks = lineblocks[sec]
                # A physical function can have several file blocks with the same
                # start (macros, inline sources, ICF aliases). Include every
                # overlapping block, not just the last equal-start entry.
                at = bisect.bisect_right(starts.get(sec, []), start)
                while at > 0 and max_ends[sec][at - 1] > start: at -= 1
                while at < len(blocks) and blocks[at][0] < end:
                    a, b, path, block = blocks[at]; at += 1
                    if b <= start or path not in self.sources: continue
                    proc_paths.add(path)
                    relevant_lines = [x['line'] for x in block['lines'] if start <= a + x['offset'] < end]
                    prior = proc_line.get(path)
                    found = min(relevant_lines) if relevant_lines else None
                    proc_line[path] = min(prior, found) if prior is not None and found is not None else prior if prior is not None else found
                addr = self.address(sec, start); enriched = {**r, **addr}
                tid = r['type_index']; name = r['name']
                if r['type_namespace'] == 'IPI':
                    resolved_name, tid = self.ipi_function(tid)
                    enriched['ipi_name'] = resolved_name; enriched['resolved_tpi'] = tid
                enriched['public_aliases'] = self.aliases.get((sec, start), [])
                enriched['source_mapping'] = 'line_table_overlap'
                if not proc_paths:
                    self.unassigned_procedures += 1
                for path in proc_paths:
                    self.mapping_counts['procedure_line_overlap'] += 1
                    self.files[path]['procedure_instances'] += 1
                    self.add_function(path, name, tid, 'compiled_procedure', {'module': mi, 'record_offset': off, **addr}, proc_line[path])
                    self.writer.put(path, {**enriched, 'source_path': path, 'source_line': proc_line[path]})
                scopes[off] = proc_paths
            elif cls == 'inline_site':
                meta = il.get(r['inlinee'], {}); path = file_path(meta.get('file_id'))
                parent_paths = scopes.get(r['parent'], proc_paths)
                own_paths = {path} if path in self.sources else set()
                scopes[off] = own_paths or parent_paths
                name, tid = self.ipi_function(r['inlinee'])
                if path in self.sources:
                    self.mapping_counts['inlinee_file_checksum'] += 1
                    self.files[path]['inline_sites'] += 1
                    ref = {'module': mi, 'record_offset': off, 'inlinee': r['inlinee']}
                    self.add_function(path, name, tid, 'inline_instance', ref, meta.get('line'), r['inlinee'])
                    self.writer.put(path, {**r, 'name': name, 'type_index': tid, 'source_line': meta.get('line'), 'source_path': path,
                        'physical_procedure': {'record_offset': proc['record_offset'], 'name': proc['name'], **self.address(proc['section'], proc['section_offset'])} if proc else None,
                        'scope_reference': {'module': mi, 'parent_record_offset': r['parent'], 'archive': module_ref},
                        'annotation_offset_basis': 'relative to physical procedure; raw annotation operations preserved'})
            elif cls != 'scope_end':
                # Contextual records are not claimed to be definitions in these files.
                decoded = decode(r) if 'yaml' in r else {}
                if decoded.get('Kind') == 'S_BUILDINFO':
                    build_id = body(decoded).get('BuildId')
                    if build_id is not None:
                        for path in available: self.ipi_context_refs[path].add(build_id)
                if proc and off <= proc['end']:
                    for path in proc_paths:
                        self.writer.put(path, {**r, 'association': 'physical_procedure_scope', 'procedure_record_offset': proc['record_offset']})
                elif 'yaml' in r:
                    d = decoded
                    if d.get('Kind') in ('S_COMPILE3', 'S_COMPILE2', 'S_BUILDINFO', 'S_OBJNAME', 'S_ENVBLOCK'):
                        for path in available: self.writer.put(path, {**r, 'association': 'compiland_context'})

    def extract(self):
        pdb = Path(self.baseline['pdb']['path']); observed = digest(pdb)
        if observed != self.baseline['pdb']['sha256']: raise RuntimeError(f'{self.v}: PDB identity hash mismatch')
        snap = self.dir / 'original.pdb'
        if not snap.exists(): subprocess.run(['/bin/cp', '-c', str(pdb), str(snap)], check=True)
        if digest(snap) != observed: raise RuntimeError('PDB snapshot hash mismatch')
        self.initial_source_hash = observed
        # This dump retains container metadata not represented by per-module records.
        metadata = self.dir / 'container.txt'
        with metadata.open('wb') as f:
            subprocess.run([LLVM, 'dump', '--summary', '--streams', '--stream-blocks', '--named-streams', '--modules', '--section-contribs', '--section-map', '--section-headers', '--fpo', str(snap)], stdout=f, check=True)
        with (self.dir / 'injected-sources.txt').open('wb') as f:
            inject = subprocess.run([LLVM, 'pretty', '--native', '--injected-sources', '--injected-source-content', str(snap)], stdout=f, stderr=subprocess.PIPE)
        self.injected_source_status = {'exit_code': inject.returncode, 'stderr': inject.stderr.decode(errors='replace')}
        atomic_json(self.dir / 'input.json', {'version': self.v, 'pdb': self.baseline['pdb'], 'binary': self.baseline['binary'], 'sections': self.baseline['pe']['sections'], 'source_snapshot_sha256': observed, 'injected_sources_query': self.injected_source_status})
        raw_types = gzip.open(self.dir / 'types.jsonl.gz', 'wb', compresslevel=1)
        raw_globals = gzip.open(self.dir / 'global-symbols.jsonl.gz', 'wb', compresslevel=1)
        progress_path = self.dir / 'progress.json'
        started = time.monotonic(); rows = None; modfile = None; modpath = None
        with (self.dir / 'reader.log').open('wb') as log:
            p = subprocess.Popen([str(EXE), str(snap)], stdout=subprocess.PIPE, stderr=log)
            try:
                for line in p.stdout:
                    try: r = json.loads(line)
                    except json.JSONDecodeError:
                        (self.dir / 'failed-record.bin').write_bytes(line)
                        raise
                    event = r['event']; self.events[event] += 1
                    if event == 'type':
                        r['decoded'] = decode(r); self.types[r['namespace']][r['index']] = r
                        raw_types.write((json.dumps(r, separators=(',', ':'), ensure_ascii=False) + '\n').encode())
                    elif event == 'identity':
                        expected = self.baseline['pdb']['guid'].replace('-', '').upper()
                        actual = r['guid'].replace('-', '').strip('{}').upper()
                        if expected != actual or r['age'] != self.baseline['pdb']['age']: raise RuntimeError('PDB GUID/age mismatch')
                        atomic_json(self.dir / 'identity.json', r)
                    elif event == 'symbol' and r['module'] == -1:
                        d = decode(r); r['decoded'] = d; raw_globals.write((json.dumps(r, separators=(',', ':'), ensure_ascii=False) + '\n').encode())
                        b = body(d)
                        if d.get('Kind') == 'S_PUB32': self.aliases[(b.get('Segment'), b.get('Offset'))].append({'name': b.get('Name'), 'record_offset': r['record_offset'], 'flags': b.get('Flags')})
                    elif event == 'module':
                        self.modules.append(r); rows = [r]
                        modpath = self.dir / 'modules' / f"{r['index']:05d}.jsonl.gz"
                        modpath.parent.mkdir(exist_ok=True); modfile = gzip.open(modpath, 'wb', compresslevel=1); modfile.write(line)
                    elif event == 'module_end':
                        modfile.write(line); modfile.close(); self.process_module(rows, modpath); rows = None; modfile = None
                        if r['module'] % 100 == 0:
                            status = {'version': self.v, 'modules': len(self.modules), 'symbols': self.events['symbol'], 'elapsed_seconds': round(time.monotonic() - started, 1)}
                            atomic_json(progress_path, status); print(json.dumps(status), flush=True)
                    elif event == 'complete': self.complete = r
                    elif rows is not None:
                        modfile.write(line); rows.append(r)
                    if event == 'symbol' and r['module'] != -1: self.symbols[r['kind']] += 1
                    if 'decode_error' in r: self.errors.append({'event': event, 'module': r.get('module'), 'kind': r.get('kind'), 'index': r.get('index'), 'record_offset': r.get('record_offset'), 'error': r['decode_error']})
                if p.wait(): raise RuntimeError(f'Native reader failed with exit {p.returncode}')
                if self.complete is None: raise RuntimeError('Missing extraction completion record')
            except BaseException:
                p.terminate(); p.wait(); raise
            finally:
                if modfile: modfile.close()
                raw_types.close(); raw_globals.close(); self.writer.close()
        if digest(pdb) != observed: raise RuntimeError('PDB input changed during extraction')
        atomic_json(self.dir / 'modules.json', self.modules)
        atomic_json(self.dir / 'decode-gaps.json', self.errors)
        self.write_sources()

    def write_sources(self):
        summaries = []; declarations = Counter(); no_functions = []
        for path, original in sorted(self.sources.items()):
            f = self.files[path]; functions = self.functions[path]; refs = set()
            lines = [MARKER, f'// Version: {self.v}', f'// Original source: {path}', f"// PDB SHA-256: {self.initial_source_hash}",
                '// Source SHA-256: ' + ', '.join(original['source_sha256']),
                '// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。',
                '// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。',
                '// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。',
                    '// 完整逐条元数据: ' + path.split('/')[-1] + '.pdb.jsonl.gz',
                f'// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/{self.v}/',
                '// 编译单元: ' + ', '.join(map(str, f['modules'])), '']
            if original.get('dependency_class'):
                lines += ['// 归属: ' + original['dependency_class'],
                    '// Repository: ' + original.get('repository', 'unknown'),
                    '// Revision: ' + original.get('revision', 'unknown'),
                    '// Source kind: ' + original.get('source_kind', 'recorded_source'), '']
            for (name, tid), fn in sorted(functions.items(), key=lambda item: (min(item[1]['lines']) if item[1]['lines'] else 10**10, item[0][0])):
                doc = self.type_doc('TPI', tid); b = body(doc)
                placeholder = doc.get('Kind') == 'LF_PROCEDURE' and b.get('ReturnType') == 3 and b.get('ParameterCount') == 0
                declarations['placeholder_signatures' if placeholder else 'nonplaceholder_type_records' if doc else 'missing_type_records'] += 1
                if tid is not None: refs.add(tid)
                lines += ['// ------------------------------------------------------------------', '// 函数符号: ' + name,
                    '// 记录形式: ' + ', '.join(sorted(fn['kinds'])) + f"; instances={fn['instances']}",
                    '// 源码行: ' + (', '.join(map(str, sorted(fn['lines']))) or 'unknown'),
                    '// TPI: ' + (f'0x{tid:X}' if tid is not None else 'unknown'),
                    '// 签名状态: ' + ('void() 调试占位；真实参数、返回类型和 Rust ABI 缺失' if placeholder else 'PDB 中有类型记录，原文及类型关系见下方' if doc else 'PDB 中未找到类型记录'),
                    '// IPI IDs: ' + (', '.join(f'0x{x:X}' for x in sorted(fn['ipi'])) or 'none'),
                    '// 首条位置: ' + json.dumps(fn['first_ref'], ensure_ascii=False, separators=(',', ':'))]
                if fn['addresses']: lines.append('// 编译实例 RVAs: ' + ', '.join(hex(x) for x in sorted(fn['addresses'])))
                # The complete name includes available generic instantiations, impl/closure scopes.
                # It is deliberately a comment: PDB placeholder ABI cannot become a Rust fn().
                if placeholder: lines.append('// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复')
                elif doc:
                    lines += ['// PDB function type:'] + ['// ' + s for s in yaml.dump(doc, allow_unicode=True, sort_keys=False).rstrip().splitlines()]
                lines.append('')
            if not functions:
                no_functions.append(path)
                lines += ['// PDB 中没有可直接归属到此文件的过程/内联函数记录。', '// 文件仍保留 checksum、编译单元引用及存在的源码行记录。', '']
            ipi_graph = self.ipi_graph(self.ipi_context_refs[path])
            for record in ipi_graph.values():
                function_type = body(record.get('decoded', {})).get('FunctionType')
                if function_type is not None: refs.add(function_type)
            graph = self.type_graph(refs)
            if graph:
                lines += ['// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===']
                for idx, row in sorted(graph.items()):
                    lines.append(f'// TPI 0x{idx:X}:')
                    lines += ['// ' + s for s in row.get('yaml', 'unavailable').rstrip().splitlines()]
            if ipi_graph:
                lines += ['// === 可用 IPI 函数身份、作用域字符串与构建信息引用图 ===']
                for index, row in sorted(ipi_graph.items()):
                    lines.append(f'// IPI 0x{index:X}:')
                    lines += ['// ' + s for s in row.get('yaml', 'unavailable').rstrip().splitlines()]
                    self.writer.put(path, {**row, 'event': 'referenced_ipi_type'})
            for index, row in sorted(graph.items()):
                self.writer.put(path, {**row, 'event': 'referenced_tpi_type'})
            summary = {'version': self.v, 'path': path, 'source_sha256': original['source_sha256'], 'pdb_sha256': self.initial_source_hash,
                'metadata_records': self.writer.counts[path], 'logical_symbol_and_type_pairs': len(functions), **f,
                'type_indices': sorted(graph), 'metadata_file': path + '.pdb.jsonl.gz'}
            summary['ipi_type_indices'] = sorted(ipi_graph)
            summary['original_paths'] = sorted(summary['original_paths']); summary['checksums'] = sorted(summary['checksums'])
            for record in summary['checksums']:
                if record not in original['source_sha256']: raise RuntimeError(f'Checksum conflict: {path}')
            source = ROOT / self.v / path
            if source.stat().st_size and not source.read_text().startswith(MARKER): raise RuntimeError(f'Refuse overwrite manually edited source {source}')
            # Keep separately generated interfaces/closures/caller annotations on refresh.
            previous = source.read_text()
            enhancement_begin = '// === PDB ENRICHMENT V1 BEGIN ==='
            enhancement_end = '// === PDB ENRICHMENT V1 END ==='
            if enhancement_begin in previous and enhancement_end in previous:
                retained = previous.split(enhancement_begin, 1)[1].split(enhancement_end, 1)[0]
                lines.extend(['', enhancement_begin + retained + enhancement_end])
            assembly_begin = '// === ORIGINAL ASSEMBLY V1 BEGIN ==='
            assembly_end = '// === ORIGINAL ASSEMBLY V1 END ==='
            if assembly_begin in previous and assembly_end in previous:
                retained = previous.split(assembly_begin, 1)[1].split(assembly_end, 1)[0]
                lines.extend(['', assembly_begin + retained + assembly_end])
            structures_begin = '// === PDB STRUCTURES V1 BEGIN ==='
            structures_end = '// === PDB STRUCTURES V1 END ==='
            if structures_begin in previous and structures_end in previous:
                retained = previous.split(structures_begin, 1)[1].split(structures_end, 1)[0]
                lines.extend(['', structures_begin + retained + structures_end])
            project_begin = '// === WHOLE PROJECT EVIDENCE V1 BEGIN ==='
            project_end = '// === WHOLE PROJECT EVIDENCE V1 END ==='
            if project_begin in previous and project_end in previous:
                retained = previous.split(project_begin, 1)[1].split(project_end, 1)[0]
                lines.extend(['', project_begin + retained + project_end])
            render_begin = '// === RENDER PASSES V1 BEGIN ==='
            render_end = '// === RENDER PASSES V1 END ==='
            if render_begin in previous and render_end in previous:
                retained = previous.split(render_begin, 1)[1].split(render_end, 1)[0]
                lines.extend(['', render_begin + retained + render_end])
            tmp = source.with_suffix('.rs.tmp'); tmp.write_text('\n'.join(lines) + '\n'); tmp.replace(source)
            atomic_json(source.with_suffix('.rs.pdb.json'), summary)
            summaries.append(summary)
        self.writer.close()
        report = {'version': self.v, 'status': 'complete_metadata_extraction', 'pdb_sha256': self.initial_source_hash,
            'native_completion': self.complete, 'events': dict(self.events), 'module_symbol_counts_by_kind': dict(self.symbols),
            'annotated_files': len(self.sources), 'logical_symbol_and_type_pairs': sum(len(f) for f in self.functions.values()),
            'signature_status': dict(declarations), 'mapping_counts': dict(self.mapping_counts),
            'files_without_assigned_function_records': no_functions, 'unassigned_procedures_in_modules_with_private_sources': self.unassigned_procedures,
            'decode_gaps': len(self.errors), 'raw_unrecognized_records_preserved': True,
            'all_module_records_archived': True, 'raw_pdb_sha256_verified': True,
            'limitations': ['PDB source paths do not contain author source text', 'Rust placeholder void() is not a recovered source signature',
                'No missing parameter/local/type/enum layout information is inferred', 'Line-table overlap identifies code attribution, not necessarily definition ownership',
                'Inline annotations preserved as operations; no unsupported interpretation of PC/source intervals',
                'Whole-PDB information without source ownership remains in global/module archives and the exact original snapshot',
                'Confirmed dependencies are archived as metadata, never implemented in the game project']}
        atomic_json(self.dir / 'source-files.json', summaries)
        atomic_json(self.dir / 'report.json', report)
        print(json.dumps({k: v for k, v in report.items() if k not in ('limitations', 'files_without_assigned_function_records', 'module_symbol_counts_by_kind')}, ensure_ascii=False), flush=True)
        return report

def main():
    baselines = json.loads((PROJECT / 'tracking/baselines.json').read_text())['baselines']
    manifest = json.loads((ROOT / 'file-manifest.json').read_text())
    reports = []
    for v in sys.argv[1:] or ['A', 'B', 'C']:
        version = Version(v, baselines[v], manifest); version.extract()
        reports.append(json.loads((version.dir / 'report.json').read_text()))
    atomic_json(ROOT / 'pdb-restoration-report.json', {'scope': 'Isolated metadata reconstruction only', 'versions': reports, 'production_project_modified': False})

if __name__ == '__main__': main()
