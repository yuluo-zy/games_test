"""Build evidence-qualified file/function queues without changing game sources."""
from collections import Counter, defaultdict
import csv
import gzip
import hashlib
import importlib.util
import itertools
import json
from pathlib import Path
import re
import sqlite3

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
OUT = ROOT / 'task-index'
CATEGORIES = {'tools': '01-工具与低复杂度候选', 'business': '02-常规业务与待补证', 'core': '03-核心枢纽与高影响'}
RANK = {'tools': 1, 'business': 2, 'core': 3}
BASES = json.loads((PROJECT / 'tracking/baselines.json').read_text())['baselines']
POLICY = json.loads((PROJECT / 'tracking/dependency-exclusions.json').read_text())
AUDITOR_PATH = Path('/Users/yuluo/.codex/skills/native-module-handoff/scripts/audit_handoff.py')
spec = importlib.util.spec_from_file_location('handoff_auditor', AUDITOR_PATH)
auditor = importlib.util.module_from_spec(spec); spec.loader.exec_module(auditor)

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as f:
        for b in iter(lambda: f.read(1024 * 1024), b''): h.update(b)
    return h.hexdigest()

def identity(*values): return hashlib.sha256('\0'.join(values).encode()).hexdigest()[:18]
def canonical(name): return re.sub(r'::h[0-9a-f]{16}(?: \(\.llvm\.\d+\))?$', '', name)
def caller_identity(name):
    # For graph counts only. Preserve exact instantiated names in nodes and in
    # cross-version matching, never turn this normalization into ABI equality.
    result = []; depth = 0
    for char in canonical(name):
        if char == '<': depth += 1
        elif char == '>' and depth: depth -= 1
        elif not depth: result.append(char)
    simplified = ''.join(result)
    return simplified if simplified and not simplified.startswith('::') else canonical(name)
def read_gz(path):
    with gzip.open(path, 'rt') as f: return json.load(f)
def jsonlines(path):
    with gzip.open(path, 'rt') as f:
        for line in f: yield json.loads(line)
def save(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_suffix(path.suffix + '.tmp'); tmp.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n'); tmp.replace(path)
def origin(path):
    path = path.replace('\\', '/')
    return path.split('country-slice-private/', 1)[1] if 'country-slice-private/' in path else None
def excluded(path, meta):
    if path.endswith('_manifest_stub.rs'): return 'manifest_placeholder_not_original_source'
    if path.startswith('_unmapped/'): return 'unmapped_type_or_symbol_metadata_bucket_not_source_implementation'
    if '/_metadata/' in path: return 'metadata_placeholder_not_original_source'
    if meta.get('dependency_class', '').startswith('dependency_reuse'): return 'confirmed_dependency_reuse'
    sources = meta.get('original_paths', meta.get('original_pdb_paths', []))
    if sources and all('/.cargo/' in s.replace('\\', '/') or '/rustc/' in s.replace('\\', '/') for s in sources):
        return 'external_or_standard_library_source'
    if path.startswith(('rhapsody2/', '_references/')): return 'dependency_reference_metadata'
    if path.startswith('crates/frozen/'):
        return 'generic_container_definition_not_game_business; retain original frozen API and game call strategies'
    parts = path.split('/'); crate = parts[2] if path.startswith('crates/systems/') else parts[1] if path.startswith('crates/') else ''
    if crate in POLICY['known_vendored_dependency_names']: return 'confirmed_vendored_dependency'
    return None

def family(path):
    if re.search(r'wall|roof|decorator|timberframe|gate|fence|stairs|opening', path): return 'building'
    if 'history' in path or re.search(r'save|snapshot|load_glade', path): return 'history_and_persistence'
    if '/utils/' in path: return 'utilities'
    if re.search(r'terrain|water|tree|forest|critters|clutter', path): return 'world_systems'
    return 'other_game_or_owner_review'

def deferred(path):
    return bool(re.search(r'/render/|/render_loop/|/rhapsody/|/shader|/shaders|/gpu|/compute_passes/|^_generated/', path))

class Index:
    def __init__(self):
        self.files = {}; self.functions = {}; self.deps = {}; self.edges = {}
        self.by_name = defaultdict(set); self.by_path_name = defaultdict(set); self.by_rva = defaultdict(set)
        self.body_cache = {}; self.blobs = {}; self.input_hashes = {}; self.enriched_references = {}
        self.file_groups = {}; self.function_groups = {}; self.tasks = []
        self.retained_index = json.loads((PROJECT / 'tracking/function-index.json').read_text())
        self.existing_sources = defaultdict(set)
        for p in (PROJECT / 'reconstruction').rglob('*.rs'):
            if any(s in p.parts for s in ('tests', 'bin', 'examples', 'target')): continue
            for name in re.findall(r'\b(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_][\w]*)', p.read_text()):
                self.existing_sources[name].add(p.relative_to(PROJECT).as_posix())

    def function(self, version, path, name):
        name = canonical(name); fid = identity(version, path, name)
        if fid not in self.functions:
            file = self.files[version, path]
            root = re.match(r'^([A-Za-z_][\w]*)::', name)
            namespace = root[1] if root else None
            library_definition = namespace in ('std', 'core', 'alloc') or namespace in POLICY['external_package_definition_roots']
            self.functions[fid] = {'id': fid, 'version': version, 'path': path, 'name': name,
                'logical_group_id': identity(path, name), 'source_sha256': file['source_sha256'],
                'ownership': 'dependency_definition_or_wrapper' if library_definition or file['excluded_reason'] else 'private_game_scope_candidate',
                'category': None, 'native_instances': [], 'body_hashes': [], 'body_sizes': [], 'physical_reference_callers': set(),
                'native_public_alias_ambiguity': False,
                'inline_direct_in': set(), 'native_in': set(), 'out': set(), 'library_dependencies': set(), 'dependency_relations': Counter(),
                'ambiguous_alias_groups': 0, 'indirect_call_groups': 0, 'candidate_interfaces': 0, 'reasons': []}
            self.by_name[version, name].add(fid); self.by_path_name[version, path, name].add(fid)
        return self.functions[fid]

    def load(self):
        for version in 'ABC':
            binary = Path(BASES[version]['binary']['path'])
            if sha(binary) != BASES[version]['binary']['sha256']: raise RuntimeError('Binary identity changed')
            self.blobs[version] = binary.read_bytes()
            for source in sorted((ROOT / version).rglob('*.rs')):
                path = source.relative_to(ROOT / version).as_posix(); meta_path = source.with_suffix('.rs.pdb.json')
                enriched_path = source.with_suffix('.rs.enriched.json.gz'); enriched = read_gz(enriched_path) if enriched_path.exists() else None
                meta = json.loads(meta_path.read_text()) if meta_path.exists() else enriched.get('source', {}) if enriched else {}
                if not meta.get('original_paths'):
                    original = re.search(r'^// Original source: (.+)$', source.read_text(), re.M)
                    if original: meta['original_paths'] = [original[1]]
                record = {'version': version, 'path': path, 'source': str(source), 'source_sha256': meta.get('source_sha256', meta.get('checksums', [])),
                    'excluded_reason': excluded(path, meta), 'category': None, 'functions': [], 'family': family(path), 'deferred': deferred(path),
                    'metadata_path': (meta_path if meta_path.exists() else enriched_path if enriched else source).relative_to(PROJECT).as_posix(),
                    'source_fingerprint': sha(source), 'original_paths': meta.get('original_paths', []), 'metadata_status': 'available' if meta else 'missing'}
                self.files[version, path] = record
                dep_path = source.with_suffix('.rs.dependencies.json')
                if dep_path.exists():
                    data = json.loads(dep_path.read_text())
                    self.input_hashes[dep_path.relative_to(PROJECT).as_posix()] = sha(dep_path)
                    for item in data['functions']:
                        fn = self.function(version, path, item['name']); self.deps[fn['id']] = item['dependencies']
                        fn['dependency_relations'].update(d['relationship'] for d in item['dependencies'])
                        for dep in item['dependencies']:
                            if dep.get('crate') and dep['ownership'] not in ('private_source_ownership_candidate', 'unresolved_namespace'):
                                fn['library_dependencies'].add(dep['crate'])
                            if dep.get('target_aliases'): fn['ambiguous_alias_groups'] += 1
                            if dep['relationship'] == 'native_indirect_call_unresolved': fn['indirect_call_groups'] += 1
                raw = source.with_suffix('.rs.pdb.jsonl.gz')
                if raw.exists():
                    for row in jsonlines(raw):
                        if row.get('class') == 'procedure':
                            fn = self.function(version, path, row['name']); rva = row.get('rva')
                            if rva is None:
                                sections = BASES[version]['pe']['sections']; sec = row['section']
                                if not 1 <= sec <= len(sections): continue
                                rva = sections[sec - 1]['rva'] + row['section_offset']
                            item = {'module': row['module'], 'record_offset': row['record_offset'], 'rva': rva, 'code_size': row['code_size']}
                            if item not in fn['native_instances']: fn['native_instances'].append(item)
                            if len(row.get('public_aliases', [])) > 1: fn['native_public_alias_ambiguity'] = True
                            self.by_rva[version, rva].add(fn['id'])
                        elif row.get('event') == 'inlinee' and row.get('name'):
                            self.function(version, path, row['name'])
                        elif row.get('class') == 'inline_site' and row.get('name'):
                            self.function(version, path, row['name'])
                if enriched:
                    self.input_hashes[enriched_path.relative_to(PROJECT).as_posix()] = sha(enriched_path)
                    for item in enriched['functions']:
                        fn = self.function(version, path, item['name'])
                        fn['candidate_interfaces'] = max(fn['candidate_interfaces'], len(item.get('candidate_interface_ids', [])))
                        self.enriched_references[fn['id']] = item.get('reference_callers', [])
                        fn['physical_reference_callers'].update(r['caller_entry_rva'] for r in item.get('reference_callers', []) if r.get('caller_entry_rva') is not None)
                record['functions'] = sorted(f['id'] for f in self.functions.values() if f['version'] == version and f['path'] == path)
            print(json.dumps({'phase': 'loaded', 'version': version, 'files': sum(v == version for v, _ in self.files), 'functions': sum(f['version'] == version for f in self.functions.values())}), flush=True)

    def edge(self, caller, callee, relationship, evidence):
        if caller == callee: return
        if self.functions[callee]['ownership'] != 'private_game_scope_candidate': return
        if self.functions[caller]['ownership'] != 'private_game_scope_candidate': return
        key = (caller, callee, relationship)
        self.edges.setdefault(key, {'caller': caller, 'callee': callee, 'relationship': relationship, 'first_evidence': evidence})
        self.functions[callee]['inline_direct_in' if relationship == 'pdb_inline_direct' else 'native_in'].add(caller)
        self.functions[caller]['out'].add(callee)

    def graph(self):
        for fid, deps in self.deps.items():
            fn = self.functions[fid]; v = fn['version']
            for dep in deps:
                relation = dep['relationship']; target = None
                if relation == 'pdb_inline_direct':
                    p = origin(dep.get('source_file') or '')
                    candidates = self.by_path_name[v, p, canonical(dep['function'])]
                    if len(candidates) == 1: target = next(iter(candidates))
                elif relation == 'native_direct_call_in_physical_body' and not dep.get('target_aliases'):
                    candidates = self.by_name[v, canonical(dep['function'])]
                    if len(candidates) == 1: target = next(iter(candidates))
                if target: self.edge(fid, target, relation, dep['first_evidence'])
        # Retained original transfer references add only uniquely assigned endpoints.
        # Same-address public aliases can still differ semantically: this count is
        # explicitly a physical-reference relation, not a Rust source call proof.
        for target, refs in self.enriched_references.items():
            v = self.functions[target]['version']
            for ref in refs:
                src = self.by_rva[v, ref.get('caller_entry_rva')]
                dst = self.by_rva[v, ref.get('target_rva')]
                if len(src) == len(dst) == 1 and target in dst and ref.get('kind') == 'direct_call' and not self.functions[target]['native_public_alias_ambiguity']:
                    self.edge(next(iter(src)), target, 'native_reference_unique_indexed_endpoints', ref)

    def body_hash(self, v, rva, size):
        key = (v, rva, size)
        if key not in self.body_cache:
            for sec in BASES[v]['pe']['sections']:
                if sec['rva'] <= rva and rva + size <= sec['rva'] + sec['raw_size']:
                    offset = sec['raw_pointer'] + rva - sec['rva']; data = self.blobs[v][offset:offset + size]
                    if len(data) != size: break
                    self.body_cache[key] = hashlib.sha256(data).hexdigest(); return self.body_cache[key]
            self.body_cache[key] = None
        return self.body_cache[key]

    def classify(self):
        for fn in self.functions.values():
            sizes = [x['code_size'] for x in fn['native_instances'] if x['code_size'] > 0]
            hashes = [self.body_hash(fn['version'], x['rva'], x['code_size']) for x in fn['native_instances'] if x['code_size'] > 0]
            fn['body_hashes'] = sorted(set(h for h in hashes if h)); fn['body_sizes'] = sorted(set(sizes))
            callers = fn['inline_direct_in'] | fn['native_in']
            logical_callers = defaultdict(set)
            for c in callers:
                key = caller_identity(self.functions[c]['name'])
                if key != caller_identity(fn['name']): logical_callers[key].add(self.functions[c]['path'])
            caller_files = {next(iter(paths)) for paths in logical_callers.values() if len(paths) == 1 and next(iter(paths)) != fn['path']}
            max_size = max(sizes, default=0); out = len({caller_identity(self.functions[c]['name']) for c in fn['out']})
            fn['metrics'] = {'unique_indexed_callers': len(logical_callers), 'direct_inline_callers': len({caller_identity(self.functions[c]['name']) for c in fn['inline_direct_in']}), 'physical_native_callers': len({caller_identity(self.functions[c]['name']) for c in fn['native_in']}),
                'cross_file_callers': len(caller_files), 'unique_private_callees': out, 'unique_observed_physical_reference_callers': len(fn['physical_reference_callers']),
                'native_max_bytes': max_size, 'native_min_bytes': min(sizes, default=0), 'indirect_call_groups': fn['indirect_call_groups'],
                'ambiguous_alias_groups': fn['ambiguous_alias_groups'], 'library_namespace_count': len(fn['library_dependencies'])}
            fn['metrics']['target_public_alias_ambiguity'] = fn['native_public_alias_ambiguity']
            fn['metrics']['instantiated_or_overlap_caller_nodes'] = len(callers)
            fn['metrics']['ambiguous_caller_source_groups'] = sum(len(paths) > 1 for paths in logical_callers.values())
            if fn['ownership'] != 'private_game_scope_candidate':
                fn['reasons'] = ['dependency or reference source; reuse its API, do not reconstruct implementation']; continue
            name = fn['name']; path = fn['path']
            orchestrator = bool(re.search(r'create|construct|publish|commit|replay|history|update|edit|finish|delete|startup|load|save', name))
            trait_candidate = bool(re.search(r'::(?:fmt|clone|deserialize|serialize|visit_\w+|drop|default)$', name))
            tool_hint = bool(re.search(r'/utils/|/math|curve|intersection|random|interpol|sample|normalize|::(?:length|max_y|min_y|flat_roof_y|angle|distance|clamp|lerp|from_index|to_index|is_\w+|as_\w+|get_\w+)$', path + ' ' + name))
            high_fan_in = len(logical_callers) >= 8 or (len(logical_callers) >= 5 and len(caller_files) >= 3)
            complex_owner = orchestrator and (out >= 8 or max_size >= 1536)
            if high_fan_in or complex_owner or (out >= 12 and max_size >= 1024):
                fn['category'] = 'core'
                if high_fan_in: fn['reasons'].append('many distinct indexed callers; prioritize contract and change impact')
                if complex_owner or out >= 12: fn['reasons'].append('large/orchestrating body with several consumers or state transitions')
            elif tool_hint and sizes and max_size <= 256 and out <= 2 and not fn['indirect_call_groups'] and not fn['ambiguous_alias_groups'] and not trait_candidate:
                fn['category'] = 'tools'; fn['reasons'].append('small evidenced utility/accessor body with limited private dependencies')
            else:
                fn['category'] = 'business'; fn['reasons'].append('ordinary business or insufficient evidence for a small utility')
            if trait_candidate: fn['reasons'].append('custom/derived trait ownership must be checked; do not rewrite Serde/container glue')
            if not sizes: fn['reasons'].append('inline-only or absent standalone range; difficulty remains uncertain')
            fn['difficulty'] = 'low_static_complexity_candidate' if fn['category'] == 'tools' else 'low_complexity_high_impact' if high_fan_in and sizes and max_size <= 256 else 'high_orchestration_candidate' if complex_owner else 'needs_contract_analysis'
            fn['implementation_readiness'] = 'candidate_after_abi_field_and_dependency_checks'
            fn['priority_score'] = 4 * len(logical_callers) + 8 * len(caller_files) + 2 * out + min(max_size // 256, 30) + (20 if complex_owner else 0)
            leaf = name.split('::')[-1].split('<', 1)[0]
            candidates = self.existing_sources.get(leaf, set())
            fn['existing_rust_name_candidates'] = sorted(candidates) if 0 < len(candidates) <= 4 else []
            fn['existing_rust_mapping_status'] = 'name-only candidates; native/ABI correspondence must be checked'
            fn['retained_B_native_evidence'] = []
            if fn['version'] == 'B':
                addresses = {BASES['B']['pe']['image_base'] + n['rva'] for n in fn['native_instances']}
                for known in self.retained_index['functions']:
                    if int(known['va'], 16) in addresses or name in known['names']:
                        fn['retained_B_native_evidence'].append(known)
        for (v, path), file in self.files.items():
            relevant = [self.functions[i] for i in file['functions'] if self.functions[i]['category']]
            cats = [f['category'] for f in relevant]
            file['category'] = max(cats, key=RANK.get) if cats else 'business' if not file['excluded_reason'] else None
            file['function_category_counts'] = dict(Counter(cats))
            callers = set().union(*(f['inline_direct_in'] | f['native_in'] for f in relevant)) if relevant else set()
            logical_callers = defaultdict(set)
            for i in callers: logical_callers[caller_identity(self.functions[i]['name'])].add(self.functions[i]['path'])
            file['incoming_callers'] = len(logical_callers)
            file['cross_file_callers'] = len({next(iter(paths)) for paths in logical_callers.values() if len(paths) == 1 and next(iter(paths)) != path})
            file['priority_score'] = max((f.get('priority_score', 0) for f in relevant), default=0)
            file['claim_mode'] = 'excluded_reuse' if file['excluded_reason'] else 'deferred' if file['deferred'] else 'analysis_required' if not relevant else 'candidate_reconstruction'
            self.file_groups.setdefault(path, {})[v] = file
        for fn in self.functions.values(): self.function_groups.setdefault(fn['logical_group_id'], {})[fn['version']] = fn

    def comparisons(self, variants, body=False):
        pairs = []
        for a, b in itertools.combinations(sorted(variants), 2):
            x = variants[a]; y = variants[b]; sx = set(x['source_sha256']); sy = set(y['source_sha256'])
            row = {'versions': [a, b], 'same_file_source_checksum_set': bool(sx) and sx == sy,
                'shared_source_checksums': sorted(sx & sy), 'source_scope': 'entire source-file checksum, not isolated function text', 'behavior_equivalence': 'unverified'}
            if body:
                hx = set(x['body_hashes']); hy = set(y['body_hashes']); common = hx & hy
                row.update({'same_native_byte_sets': bool(hx) and hx == hy, 'some_native_instances_identical': bool(common), 'shared_body_sha256': sorted(common),
                    'native_status': 'same_observed_body_byte_sets' if hx and hx == hy else 'some_instances_same_bytes' if common else 'bytes_differ_or_codegen_changed' if hx and hy else 'native_ranges_missing',
                    'abi_equivalence': 'unknown; placeholder TPI indices are not ABI evidence'})
            pairs.append(row)
        return pairs

    def tasks_and_handoffs(self):
        for path, variants in sorted(self.file_groups.items()):
            eligible = {v: f for v, f in variants.items() if not f['excluded_reason']}
            task_id = 'TG-' + identity(path)
            for file in variants.values(): file['task_id'] = task_id if eligible else None
            if not eligible: continue
            target = next(v for v in 'CBA' if v in eligible)
            category = max((f['category'] for f in eligible.values()), key=RANK.get)
            fnids = [i for f in eligible.values() for i in f['functions'] if self.functions[i]['category']]
            callees = {self.functions[c]['path'] for i in fnids for c in self.functions[i]['out']} - {path}
            callers = {self.functions[c]['path'] for i in fnids for c in self.functions[i]['inline_direct_in'] | self.functions[i]['native_in']} - {path}
            task = {'id': task_id, 'source_path': path, 'category': category, 'target_version': target, 'versions': sorted(eligible),
                'priority_score': max(f['priority_score'] for f in eligible.values()), 'family': eligible[target]['family'],
                'claim_mode': eligible[target]['claim_mode'], 'cross_version_pairs': self.comparisons(eligible), 'function_ids': fnids,
                'related_callee_tasks': ['TG-' + identity(p) for p in sorted(callees) if p in self.file_groups and any(not f['excluded_reason'] for f in self.file_groups[p].values())],
                'related_caller_tasks': ['TG-' + identity(p) for p in sorted(callers) if p in self.file_groups and any(not f['excluded_reason'] for f in self.file_groups[p].values())],
                'source_inputs': {v: {'path': f['source'], 'sha256': f['source_fingerprint'], 'binary_sha256': BASES[v]['binary']['sha256'], 'pdb_sha256': BASES[v]['pdb']['sha256']} for v, f in eligible.items()},
                'assignment_unit': 'one source-file family across A/B/C; one owner to prevent overlapping file edits',
                'deliverable': 'independent candidate Rust + ABI/field/dependency contracts + producer/consumer and removal/history boundaries + evidence handoff',
                'implementation_status': 'not_assessed; existing Rust must be checked before duplicate work',
                'semantic_acceptance': False}
            task['existing_reconstruction_candidates'] = sorted({p for i in fnids for p in self.functions[i].get('existing_rust_name_candidates', [])})
            task['known_B_evidence_function_ids'] = [i for i in fnids if self.functions[i].get('retained_B_native_evidence')]
            task['data_structure_metadata'] = {v: (ROOT / v / (path + '.structures.json.gz')).relative_to(PROJECT).as_posix()
                for v in eligible if (ROOT / v / (path + '.structures.json.gz')).exists()}
            facts = []; deps = []
            for v, file in eligible.items():
                facts.append({'id': v + '-source', 'version': v, 'kind': 'source_owner', 'evidence': file['metadata_path'], 'claim': 'source attribution candidate and metadata, not recovered author text'})
                d = ROOT / v / (path + '.dependencies.json')
                if d.exists():
                    facts.append({'id': v + '-deps', 'version': v, 'kind': 'dependency', 'evidence': d.relative_to(PROJECT).as_posix()})
                    deps.append(d.relative_to(PROJECT).as_posix())
            ledger = {'target_version': target, 'module': path, 'versions': {v: {'artifact_sha256': BASES[v]['binary']['sha256'], 'pdb_sha256': BASES[v]['pdb']['sha256']} for v in eligible},
                'facts': facts, 'dependencies': deps, 'rust_files': [str((ROOT / v / path).relative_to(PROJECT)) for v in eligible],
                'links': [{'name': 'metadata_and_source_navigation', 'status': 'evidence_ready', 'fact_ids': [f['id'] for f in facts], 'gaps': ['metadata ownership and signatures still need native contracts']},
                    {'name': 'authoritative_input_and_abi', 'status': 'unknown', 'fact_ids': [], 'gaps': ['recover actual inputs, units, fields and ABI; do not use void() placeholder']},
                    {'name': 'candidate_rust_and_consumers', 'status': 'unknown', 'fact_ids': [], 'gaps': ['inspect existing reconstruction; deliver actual callers/outputs and library API boundaries']},
                    {'name': 'edit_removal_history_and_acceptance', 'status': 'unknown', 'fact_ids': [], 'gaps': ['stateful owners require update/removal/history; pure helpers require original consumer checks', 'business validation is a separate authorized phase']} ]}
            handoff = OUT / 'handoffs' / (task_id + '.json'); save(handoff, ledger)
            audit = auditor.audit(handoff, PROJECT); save(OUT / 'handoffs' / (task_id + '.audit.json'), audit)
            if not audit['ok']: raise RuntimeError(audit['errors'])
            task['handoff'] = handoff.relative_to(ROOT).as_posix(); task['handoff_audit'] = (OUT / 'handoffs' / (task_id + '.audit.json')).relative_to(ROOT).as_posix()
            save(OUT / 'tasks' / (task_id + '.json'), task); self.tasks.append(task)

    def outputs(self):
        OUT.mkdir(exist_ok=True); (OUT / 'claims').mkdir(exist_ok=True)
        db = OUT / 'index.sqlite3.tmp'
        if db.exists(): db.unlink()
        con = sqlite3.connect(db)
        con.executescript('CREATE TABLE files(id TEXT PRIMARY KEY,version TEXT,path TEXT,category TEXT,excluded_reason TEXT,task_id TEXT,priority INTEGER,payload TEXT); CREATE TABLE functions(id TEXT PRIMARY KEY,version TEXT,path TEXT,name TEXT,category TEXT,group_id TEXT,priority INTEGER,payload TEXT); CREATE TABLE edges(caller TEXT,callee TEXT,relationship TEXT,payload TEXT); CREATE TABLE tasks(id TEXT PRIMARY KEY,category TEXT,target_version TEXT,claim_mode TEXT,priority INTEGER,payload TEXT); CREATE INDEX function_name ON functions(name); CREATE INDEX incoming_edges ON edges(callee); CREATE INDEX outgoing_edges ON edges(caller); CREATE INDEX task_queue ON tasks(category,claim_mode,priority);')
        encode = lambda obj: json.dumps(obj, ensure_ascii=False, separators=(',', ':'))
        function_groups = []
        for gid, variants in self.function_groups.items():
            comparisons = self.comparisons(variants, body=True)
            group = {'id': gid, 'path': next(iter(variants.values()))['path'], 'name': next(iter(variants.values()))['name'], 'versions': {v: f['id'] for v, f in variants.items()},
                'pairs': comparisons, 'grouping_status': 'same path + exact canonical name candidate; not an ABI or behavior match',
                'reuse_rule': 'one logical analysis target; use C contracts when C exists; identical source/bytes may share implementation only after interface and dependency checks'}
            function_groups.append(group)
            for f in variants.values(): f['cross_version_pairs'] = comparisons
        for f in self.functions.values():
            f['distinct_callers'] = sorted(f['inline_direct_in'] | f['native_in']); f['private_callees'] = sorted(f['out'])
            for field in ('inline_direct_in', 'native_in', 'out'): del f[field]
            f['physical_reference_callers'] = sorted(f['physical_reference_callers']); f['library_dependencies'] = sorted(f['library_dependencies'])
            f['dependency_relations'] = dict(f['dependency_relations'])
            con.execute('INSERT INTO functions VALUES(?,?,?,?,?,?,?,?)', (f['id'], f['version'], f['path'], f['name'], f['category'], f['logical_group_id'], f.get('priority_score', 0), encode(f)))
        for (v, path), file in self.files.items(): con.execute('INSERT INTO files VALUES(?,?,?,?,?,?,?,?)', (identity(v, path), v, path, file['category'], file['excluded_reason'], file['task_id'], file['priority_score'], encode(file)))
        for edge in self.edges.values(): con.execute('INSERT INTO edges VALUES(?,?,?,?)', (edge['caller'], edge['callee'], edge['relationship'], encode(edge)))
        for task in self.tasks: con.execute('INSERT INTO tasks VALUES(?,?,?,?,?,?)', (task['id'], task['category'], task['target_version'], task['claim_mode'], task['priority_score'], encode(task)))
        con.commit(); con.close(); db.replace(OUT / 'index.sqlite3')
        for name, records in [('functions', self.functions.values()), ('function-groups', function_groups), ('files', self.files.values())]:
            with gzip.open(OUT / (name + '.jsonl.gz'), 'wt', compresslevel=1) as f:
                for record in records: f.write(encode(record) + '\n')
        for category, dirname in CATEGORIES.items():
            with (OUT / (dirname + '-函数.csv')).open('w', newline='') as f:
                writer = csv.writer(f); writer.writerow(['function_id', 'logical_group_id', 'version', 'path', 'name', 'distinct_callers', 'cross_file_callers', 'max_native_bytes', 'file_task_id'])
                for fn in sorted((x for x in self.functions.values() if x['category'] == category), key=lambda x: (-x.get('priority_score', 0), x['name'])):
                    metrics = fn['metrics']; writer.writerow([fn['id'], fn['logical_group_id'], fn['version'], fn['path'], fn['name'], metrics['unique_indexed_callers'], metrics['cross_file_callers'], metrics['native_max_bytes'], self.files[fn['version'], fn['path']]['task_id']])
        save(OUT / 'tasks.json', self.tasks)
        active_ids = {t['id'] for t in self.tasks}
        for obsolete in (OUT / 'tasks').glob('TG-*.json'):
            if obsolete.stem not in active_ids and not (OUT / 'claims' / (obsolete.stem + '.json')).exists():
                destination = OUT / '_retired/tasks' / obsolete.name; destination.parent.mkdir(parents=True, exist_ok=True); obsolete.replace(destination)
                for suffix in ('.json', '.audit.json'):
                    old = OUT / 'handoffs' / (obsolete.stem + suffix)
                    if old.exists():
                        destination = OUT / '_retired/handoffs' / old.name; destination.parent.mkdir(parents=True, exist_ok=True); old.replace(destination)
        same_items = []
        for group in function_groups:
            variants = self.function_groups[group['id']]
            eligible = [f for f in variants.values() if f['category']]
            if not eligible: continue
            src = ['='.join(p['versions']) for p in group['pairs'] if p['same_file_source_checksum_set']]
            code = ['='.join(p['versions']) for p in group['pairs'] if p['same_native_byte_sets']]
            if src or code: same_items.append((group, eligible[0]['category'], src, code))
        same_items.sort(key=lambda r: (not bool(r[3]), r[1] != 'tools', r[0]['path'], r[0]['name']))
        with (OUT / '跨版本相同项.csv').open('w', newline='') as f:
            writer = csv.writer(f); writer.writerow(['logical_function_group_id', 'source_path', 'function_name', 'same_file_source_versions', 'same_native_byte_sets_versions', 'behavior_equivalence'])
            for group, _, src, code in same_items: writer.writerow([group['id'], group['path'], group['name'], ';'.join(src), ';'.join(code), 'unverified'])
        navigation = ['# A/B/C 同源与同码候选', '', '同名、同路径候选中的源文件 SHA-256 与独立过程字节指纹分别比较。行为和 ABI 等价均未验收；不按同址/同字节合并不同名字。完整清单见 [CSV](跨版本相同项.csv)，完整范围与 hash 可用 show-function 查询。', '', '| 函数组 | 文件 / 符号 | 同源文件版本 | 相同过程字节版本 |', '|---|---|---|---|']
        for group, _, src, code in same_items[:80]:
            navigation.append('| ' + group['id'] + ' | ' + group['path'] + '<br>' + group['name'].replace('|', '\\|') + ' | ' + (', '.join(src) or '未证') + ' | ' + (', '.join(code) or '未证') + ' |')
        (OUT / '跨版本相同项.md').write_text('\n'.join(navigation) + '\n')
        source_pair_counts = Counter(); body_pair_counts = Counter()
        for group in function_groups:
            for pair in group['pairs']:
                if pair['same_file_source_checksum_set']: source_pair_counts['='.join(pair['versions'])] += 1
                if pair['same_native_byte_sets']: body_pair_counts['='.join(pair['versions'])] += 1
        summary = {'file_variants': len(self.files), 'function_variants': len(self.functions), 'logical_function_groups': len(function_groups),
            'business_file_categories_by_version': {v: dict(Counter(f['category'] for (ver, _), f in self.files.items() if ver == v and f['category'])) for v in 'ABC'},
            'excluded_files_by_version': {v: sum(bool(f['excluded_reason']) for (ver, _), f in self.files.items() if ver == v) for v in 'ABC'},
            'function_categories_by_version': {v: dict(Counter(f['category'] for f in self.functions.values() if f['version'] == v and f['category'])) for v in 'ABC'},
            'task_categories': dict(Counter(t['category'] for t in self.tasks)), 'tasks': len(self.tasks), 'source_identical_function_pairs': dict(source_pair_counts),
            'private_function_groups_with_same_source_or_bytes': len(same_items), 'private_function_groups_with_identical_native_instances': sum(bool(r[3]) for r in same_items),
            'native_byte_identical_function_pairs': dict(body_pair_counts), 'graph_edges': len(self.edges), 'behavior_equivalence_verified': False,
            'difficulty_policy': 'Static planning heuristic, not proven implementation readiness; unknown ABI/fields are never zero-filled',
            'ownership_policy': 'Confirmed libraries/reference code excluded from reconstruction queues; private/local files remain owner candidates',
            'input_sha256': self.input_hashes, 'production_project_modified': False}
        save(OUT / 'summary.json', summary)
        for category, dirname in CATEGORIES.items():
            selected = sorted((t for t in self.tasks if t['category'] == category), key=lambda t: (t['claim_mode'] == 'deferred', -t['priority_score'], t['source_path']))
            with (OUT / (dirname + '.csv')).open('w', newline='') as f:
                writer = csv.writer(f); writer.writerow(['task_id', 'source_path', 'target', 'versions', 'claim_mode', 'priority_score', 'same_source_pairs'])
                for t in selected: writer.writerow([t['id'], t['source_path'], t['target_version'], '/'.join(t['versions']), t['claim_mode'], t['priority_score'], ','.join('='.join(p['versions']) for p in t['cross_version_pairs'] if p['same_file_source_checksum_set'])])
            text = [f'# {dirname}', '', f'跨版本文件组任务：{len(selected)}。完整索引见同名 CSV，函数分类及依赖可用领取工具查询。', '', '| 任务 | 文件 | 主目标 | 状态 | 影响评分 | 同源版本 |', '|---|---|---|---|---:|---|']
            for t in selected[:40]:
                same = ', '.join('='.join(p['versions']) for p in t['cross_version_pairs'] if p['same_file_source_checksum_set']) or '待对比'
                text.append(f"| [{t['id']}](tasks/{t['id']}.json) | {t['source_path']} | {t['target_version']} | {t['claim_mode']} | {t['priority_score']} | {same} |")
            (OUT / (dirname + '.md')).write_text('\n'.join(text) + '\n')
        # Navigation for every version, including excluded reference files.
        for version in 'ABC':
            with (OUT / f'{version}-文件分类.csv').open('w', newline='') as f:
                writer = csv.writer(f); writer.writerow(['path', 'category', 'excluded_reason', 'claim_mode', 'task_id', 'distinct_callers', 'cross_file_callers', 'function_categories'])
                for (v, path), file in sorted(self.files.items()):
                    if v == version: writer.writerow([path, file['category'], file['excluded_reason'], file['claim_mode'], file['task_id'], file['incoming_callers'], file['cross_file_callers'], json.dumps(file['function_category_counts'])])
        print(json.dumps({k: v for k, v in summary.items() if k != 'input_sha256'}, ensure_ascii=False), flush=True)

    def run(self):
        OUT.mkdir(exist_ok=True); self.load(); self.graph(); self.classify(); self.tasks_and_handoffs(); self.outputs()
        from build_abc_common_index import build
        build()

if __name__ == '__main__': Index().run()
