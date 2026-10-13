#!/usr/bin/env python3
"""Tiny Glade compiled-shaders -> shader-source skeleton extraction.

Extends scripts/render-passes/parse_shader_catalog.py (reuses its filename
parsing + family classification + SPIR-V word walking) to extract, for each of
the 404 distribution .bin shaders:

  * the EMBEDDED source file paths (dxc -Zi -Qembed_debug stores them as
    OpString / OpSource records; main file first, includes after),
  * entry point(s), stage, variant hash (from filename),
  * LocalSize (from OpExecutionMode),
  * the OpName / OpMemberName table (resource + representative local/function
    names),
  * per-bin pass/family linkage.

Emits, under a skeleton evidence directory:

  raw-per-bin.jsonl            per-bin extraction (Observation)
  per-shader-mapping.json      source path -> .bin(s)+pass+entry+variant
  include-graph.json           file -> included files -> used-by bins
  source-tree.md               human source tree with counts
  summary.md                   stats / open unknowns

This is analysis-only evidence generation. The 404 .bin are read-only; nothing
under the distribution is ever written.

Usage:
    extract_shader_skeleton.py <shader_dir> <spirv_grammar.json> <index_json> <out_dir>
"""

import collections
import hashlib
import json
import re
import struct
import sys
from pathlib import Path

SHADER_DIR = Path(sys.argv[1])
GRAMMAR_PATH = Path(sys.argv[2])
INDEX_PATH = Path(sys.argv[3])
OUT_DIR = Path(sys.argv[4])

sys.path.insert(0, str(Path(__file__).parent))
from parse_shader_catalog import (  # noqa: E402  (reuse existing parser)
    FN_RE,
    HEX16,
    SPIRV_MAGIC,
    parse_filename,
    classify_family,
)

# ---------------------------------------------------------------- grammar
_grammar = json.loads(GRAMMAR_PATH.read_text())
_OPS = {i['opcode']: i['opname'] for i in _grammar['instructions']}
_ENUMS = {}
for k in _grammar['operand_kinds']:
    _ENUMS[k['kind']] = {
        (int(v['value'], 0) if isinstance(v['value'], str) else v['value']): v['enumerant']
        for v in k.get('enumerants', [])
    }

def _enum(kind, value):
    return _ENUMS.get(kind, {}).get(value, str(value))


# ---------------------------------------------------------------- SPIR-V walk
OP_SOURCE = 3
OP_NAME = 5
OP_MEMBER_NAME = 6
OP_STRING = 7
OP_LINE = 8
OP_SOURCE_EXT = 10
OP_EXT_INST_IMPORT = 11
OP_EXT_INST = 12
OP_ENTRY_POINT = 15
OP_EXEC_MODE = 16
OP_CAPABILITY = 17
OP_VARIABLE = 59
OP_TYPE_POINTER = 32
OP_TYPE_IMAGE = 25
OP_TYPE_SAMPLER = 26
OP_TYPE_SAMPLED_IMAGE = 27
OP_TYPE_STRUCT = 30
OP_TYPE_ACCEL = 5349
OP_MODULE_PROCESSED = 330

DEC_BLOCK = 'Block'
DEC_BUFFER_BLOCK = 'BufferBlock'
DEC_DESCRIPTOR_SET = 'DescriptorSet'
DEC_BINDING = 'Binding'

_SOURCE_PATH_RE = re.compile(r'^shaders/.*\.hlsl$')


def _words_to_string(ws):
    raw = struct.pack('<' + 'I' * len(ws), *ws)
    return raw.split(b'\x00')[0].decode('utf-8', errors='replace')


def parse_module_extended(payload):
    """Full module walk: names + entries + modes + bindings + EMBEDDED SOURCES."""
    n = len(payload) // 4
    w = struct.unpack('<' + 'I' * n, payload)
    if w[0] != 0x07230203:
        return None
    names = {}          # id -> OpName string
    member_names = {}   # (struct_id, member_idx) -> name
    deco = collections.defaultdict(list)
    variables = {}
    pointed = {}
    image_info = {}
    sampler_types = set()
    sampledimage_types = set()
    struct_types = set()
    accel_types = set()
    entries = []
    modes = []
    caps = []
    string_ids = {}     # OpString id -> text
    source_file_ids = []  # OpSource file ids in emission order
    module_processed = []  # dxc command lines
    first_lines = []

    i = 5
    while i < n:
        wc = w[i] >> 16
        op = w[i] & 0xffff
        if wc == 0 or i + wc > n:
            break
        a = w[i + 1:i + wc]
        if op == OP_NAME:
            if len(a) >= 2:
                names[a[0]] = _words_to_string(a[1:])
        elif op == OP_MEMBER_NAME:
            if len(a) >= 3:
                member_names[(a[0], a[1])] = _words_to_string(a[2:])
        elif op == 71:  # OpDecorate
            if len(a) >= 2:
                deco[a[0]].append({'kind': _enum('Decoration', a[1]), 'args': list(a[2:])})
        elif op == OP_STRING:
            if len(a) >= 2:
                string_ids[a[0]] = _words_to_string(a[1:])
        elif op == OP_SOURCE:
            # operands: Source Language, Version[, File][, Source]
            if len(a) >= 1:
                fid = a[2] if len(a) >= 3 else None
                if fid is not None:
                    source_file_ids.append(fid)
        elif op == OP_LINE:
            if len(a) >= 2:
                first_lines.append((a[0], a[1]))
        elif op == OP_MODULE_PROCESSED:
            if a:
                module_processed.append(_words_to_string(a))
        elif op == OP_VARIABLE:
            if len(a) >= 3:
                variables[a[1]] = {'type': a[0], 'storage': _enum('StorageClass', a[2])}
        elif op == OP_ENTRY_POINT:
            if len(a) >= 3:
                entries.append({'stage': _enum('ExecutionModel', a[0]), 'entry': _words_to_string(a[2:])})
        elif op == OP_EXEC_MODE:
            if len(a) >= 2:
                modes.append({'mode': _enum('ExecutionMode', a[1]), 'args': list(a[2:])})
        elif op == OP_CAPABILITY:
            if a:
                caps.append(_enum('Capability', a[0]))
        elif op == OP_TYPE_POINTER:
            if len(a) >= 3:
                pointed[a[0]] = a[2]
        elif op == OP_TYPE_IMAGE:
            if len(a) >= 8:
                image_info[a[0]] = {
                    'dim': _enum('Dim', a[2]), 'depth': a[3], 'arrayed': a[4],
                    'ms': a[5], 'sampled': a[6], 'format': _enum('ImageFormat', a[7]),
                }
        elif op == OP_TYPE_SAMPLER:
            if a:
                sampler_types.add(a[0])
        elif op == OP_TYPE_SAMPLED_IMAGE:
            if a:
                sampledimage_types.add(a[0])
        elif op == OP_TYPE_STRUCT:
            if a:
                struct_types.add(a[0])
        elif op == OP_TYPE_ACCEL:
            if a:
                accel_types.add(a[0])
        i += wc

    # ---- binding kind inference (same rules as parse_shader_catalog)
    def infer_kind(var_type_id, storage):
        if storage == 'PushConstant':
            return 'PushConstant'
        t = pointed.get(var_type_id)
        if t in image_info:
            img = image_info[t]
            if img['sampled'] == 2:
                return 'StorageImage(%s)' % img['dim']
            if img['sampled'] == 1:
                return 'SampledImage(%s)' % img['dim']
            return 'Image(sampled=runtime)'
        if t in sampledimage_types:
            return 'SampledImage'
        if t in sampler_types:
            return 'Sampler'
        if t in accel_types:
            return 'AccelerationStructure'
        if storage == 'UniformConstant':
            if t in struct_types:
                return 'UniformConstant(struct)'
            return 'UniformConstant(other)'
        if storage == 'Uniform':
            if t in struct_types:
                d = [x['kind'] for x in deco.get(t, [])]
                if DEC_BLOCK in d:
                    return 'UniformBuffer(UBO)'
                if DEC_BUFFER_BLOCK in d:
                    return 'StorageBuffer(SSBO,BufferBlock)'
                return 'Uniform-struct(no Block decor)'
            return 'Uniform(non-struct)'
        if storage == 'StorageBuffer':
            return 'StorageBuffer(SSBO)'
        return storage

    bindings = []
    for vid, info in variables.items():
        dset = None
        bind = None
        for d in deco.get(vid, []):
            if d['kind'] == DEC_DESCRIPTOR_SET:
                dset = d['args'][0] if d['args'] else None
            elif d['kind'] == DEC_BINDING:
                bind = d['args'][0] if d['args'] else None
        is_push = info['storage'] == 'PushConstant'
        if dset is not None or bind is not None or is_push:
            bindings.append({
                'name': names.get(vid),
                'storage': info['storage'],
                'set': dset,
                'binding': bind,
                'kind': infer_kind(info['type'], info['storage']),
            })
    bindings.sort(key=lambda b: (b['set'] if b['set'] is not None else 0,
                                 b['binding'] if b['binding'] is not None else 0))

    # sources: file paths in OpSource emission order (main file is usually
    # first; leading-include files may precede it for files that open with
    # `#include "inc/common.hlsl"`).
    ordered_paths = []
    for fid in source_file_ids:
        s = string_ids.get(fid)
        if s and _SOURCE_PATH_RE.match(s) and s not in ordered_paths:
            ordered_paths.append(s)

    local_size = None
    for md in modes:
        if md['mode'] == 'LocalSize':
            local_size = list(md['args'][:3]) if len(md['args']) >= 3 else list(md['args'])

    return {
        'entries': entries,
        'execution_modes': modes,
        'local_size': local_size,
        'capabilities': caps,
        'bindings': bindings,
        'member_names': [{'struct_id': k[0], 'member': k[1], 'name': v}
                         for k, v in sorted(member_names.items())],
        'debug_names': [v for v in names.values()],
        'source_paths': ordered_paths,
        'first_line': (string_ids.get(first_lines[0][0]) if first_lines else None),
        'module_processed': module_processed,
    }


def split_modules(payload):
    """Split a .bin into SPIR-V modules (raster bins hold 2: VS + FS)."""
    out = []
    cursor = 0
    while True:
        at = payload.find(SPIRV_MAGIC, cursor)
        if at < 0:
            break
        n = struct.unpack_from('<I', payload, at - 4)[0]
        mod = payload[at:at + 4 * n]
        if len(mod) == 4 * n:
            out.append({'offset': at, 'payload': mod,
                        'sha256': hashlib.sha256(mod).hexdigest()})
        cursor = at + 4 * n
    return out


# ---------------------------------------------------------------- path helpers
def resolve_path(p):
    """Normalise `../` in an embedded source path (raw evidence stays raw)."""
    parts = p.split('/')
    out = []
    for part in parts:
        if part == '..':
            if out and out[-1] != '..':
                out.pop()
            else:
                out.append(part)
        else:
            out.append(part)
    return '/'.join(out)


def bin_pass_transform(p, stage):
    """Inverse of the .bin filename encoding:
    `shaders/<dir>/<name>.<stage>.hlsl` -> `<dir>_<name>` pass name."""
    rel = p[len('shaders/'):]
    if not rel.endswith('.' + stage + '.hlsl'):
        return None
    return rel[:-(len(stage) + 6)].replace('/', '_')


# ---------------------------------------------------------------- main
def main():
    index = json.loads(INDEX_PATH.read_text()) if INDEX_PATH.exists() else None

    # build INDEX.json lookup: render_world pass by artifact and by uri fragment
    artifact_to_pass = collections.defaultdict(list)   # artifact name -> [pass ids]
    uri_to_pass = collections.defaultdict(list)
    if index:
        for pw in index.get('render_world_passes', []):
            for art in pw.get('artifacts', []):
                artifact_to_pass[art].append(pw['id'])
            for u in pw.get('uris', []):
                # uri like "/nani/cull_and_bucket*.cs.hlsl:cs_main" or "/dof/fused.cs.hlsl"
                base = u.split(':', 1)[0].strip('/')
                uri_to_pass[base].append(pw['id'])

    def index_pass_ids(pass_name, main_source):
        ids = []
        if pass_name in artifact_to_pass:
            ids += artifact_to_pass[pass_name]
        if main_source:
            rel = main_source[len('shaders/'):]                 # dir/name.cs.hlsl
            name = rel.rsplit('/', 1)[-1]                        # name.cs.hlsl
            stem = rel.split('.')[0]                             # dir/name (before first '.')
            name_stem = name.split('.')[0]                       # name
            for ubase, plist in uri_to_pass.items():
                u = ubase.split('(', 1)[0].strip()               # drop parenthetical variant notes
                u = u.strip('/')
                if u.endswith('.cs.hlsl') or u.endswith('.raster.hlsl'):
                    u_stem = u.split('.')[0]                     # dir/name
                    hits = (u == rel or u_stem == stem)
                elif '*' in u:
                    prefix = u.replace('*', '')
                    hits = stem.startswith(prefix) or name_stem.startswith(prefix)
                else:
                    hits = (u == stem or u == name_stem)
                if hits:
                    ids += plist
        return sorted(set(ids))

    files = sorted(SHADER_DIR.glob('*.bin'))
    if not files:
        sys.exit('no .bin files found in %s' % SHADER_DIR)

    per_bin = []
    path_as_main = collections.defaultdict(list)      # path -> [bin recs]
    path_as_include = collections.defaultdict(list)   # path -> [bin names]
    raw_path_seen = collections.defaultdict(list)     # raw path -> bin names

    for path in files:
        meta = parse_filename(path.name)
        if meta is None:
            per_bin.append({'bin': path.name, 'error': 'unparsable filename'})
            continue
        payload = path.read_bytes()
        rec = {
            'bin': path.name,
            'file_sha256': hashlib.sha256(payload).hexdigest(),
            'size': len(payload),
            'pass': meta['pass'],
            'stage': meta['stage'],
            'entry_hint': meta['entry_hint'],
            'variant': meta['variant'],
            'family': classify_family(meta['pass'], meta['stage']),
            'modules': [],
        }
        mods = split_modules(payload)
        sources_all = []
        for m in mods:
            pm = parse_module_extended(m['payload'])
            if pm is None:
                continue
            rec['modules'].append({
                'offset': m['offset'],
                'sha256': m['sha256'],
                'entries': pm['entries'],
                'execution_modes': pm['execution_modes'],
                'local_size': pm['local_size'],
                'capabilities': pm['capabilities'],
                'bindings': pm['bindings'],
                'member_names': pm['member_names'],
                'debug_names': pm['debug_names'],
                'source_paths': pm['source_paths'],
                'first_line': pm['first_line'],
                'module_processed': pm['module_processed'],
            })
            for sp in pm['source_paths']:
                if sp not in sources_all:
                    sources_all.append(sp)
                    raw_path_seen[sp].append(path.name)

        rec['sources'] = sources_all
        # main = embedded path whose inverse-transform matches the bin pass name.
        # parse_filename's pass keeps the leading `_` separator; the source-path
        # inverse transform does not, so strip it for the comparison.
        pass_key = meta['pass'].lstrip('_')
        main_sources = [s for s in sources_all
                        if bin_pass_transform(s, meta['stage']) == pass_key]
        rec['main_source'] = main_sources[0] if len(main_sources) == 1 else main_sources
        rec['source_count'] = len(sources_all)
        rec['includes'] = [s for s in sources_all if s not in (main_sources
                                                              if not isinstance(main_sources, str) else [main_sources])]
        rec['sources_before_main'] = (
            sources_all[:sources_all.index(rec['main_source'])]
            if isinstance(rec['main_source'], str) else [])
        rec['index_pass_ids'] = index_pass_ids(meta['pass'],
                                               rec['main_source'] if isinstance(rec['main_source'], str) else None)
        per_bin.append(rec)

        ms = rec['main_source']
        if isinstance(ms, str):
            path_as_main[ms].append({
                'bin': path.name, 'pass': meta['pass'], 'stage': meta['stage'],
                'variant': meta['variant'], 'family': rec['family'],
                'file_sha256': rec['file_sha256'],
                'index_pass_ids': rec['index_pass_ids'],
            })
            for inc in rec['includes']:
                path_as_include[inc].append(path.name)

    # ---- raw path uniqueness (must reproduce the 419 known from prior agent)
    unique_raw = sorted({p for p in raw_path_seen})
    unique_resolved = {}
    for raw in unique_raw:
        resolved = resolve_path(raw)
        unique_resolved.setdefault(resolved, []).append(raw)

    # ---- include graph.
    # Observation: per-bin ordered source lists give, for every main file, the
    # full set of files it pulls in (include closure) in first-touch order.
    # The shared-layer nesting is inferred from cross-bin relative ordering:
    #   after(F)  = {G : G follows F in a strict majority of co-occurrences}
    #   direct(F) = minimal elements of after(F) (not descendants of another
    #               follower). For main files the observed include set replaces
    #               after(F). This is the MINIMAL DAG consistent with all
    #               ordering observations (Inference; sibling/descendant is
    #               fundamentally ambiguous from preorder alone, so real direct
    #               edges may be under-reported when a file both includes and
    #               inherits the same include).
    cooccur = collections.defaultdict(lambda: collections.Counter())
    for rec in per_bin:
        ms = rec.get('main_source')
        if not isinstance(ms, str):
            continue
        incs = [s for s in rec['sources'] if s != ms]
        for i in range(len(incs)):
            for j in range(i + 1, len(incs)):
                cooccur[incs[i]][incs[j]] += 1

    def after(f):
        """Files that follow f in a strict majority of co-occurrences."""
        out = set()
        for g, c in cooccur.get(f, {}).items():
            c_other = cooccur.get(g, {}).get(f, 0)
            if c > c_other:
                out.add(g)
        return out

    observed = {rec['main_source']: set(rec['includes']) for rec in per_bin
                if isinstance(rec.get('main_source'), str)}
    all_files = set(observed) | {v for vals in observed.values() for v in vals}

    after_map = {f: after(f) for f in all_files}

    direct = {}
    for f in all_files:
        if f in observed:
            cand = set(observed[f])
        else:
            cand = set(after_map.get(f, ()))
        reduced = set()
        for g in cand:
            # drop g if it follows some other candidate h (g is h's descendant)
            if not any(h in cand and h != g and g in after_map.get(h, ())
                       for h in cand):
                reduced.add(g)
        direct[f] = reduced

    include_only = sorted(f for f in all_files if f not in observed)
    shared_usage = {}
    for inc in include_only:
        bins = path_as_include.get(inc, [])
        mains = sorted({r['main_source'] for r in per_bin if isinstance(r.get('main_source'), str)
                        and inc in r['includes']})
        shared_usage[inc] = {'include_bin_count': len(set(bins)),
                             'used_by_mains': mains}

    # ---- OUTPUT -------------------------------------------------------------
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    (OUT_DIR / 'raw-per-bin').mkdir(parents=True, exist_ok=True)

    # 1) raw-per-bin.jsonl
    with open(OUT_DIR / 'raw-per-bin.jsonl', 'w', encoding='utf-8') as f:
        for rec in per_bin:
            f.write(json.dumps(rec, ensure_ascii=False) + '\n')

    # 2) per-shader-mapping.json
    mapping = []
    for resolved in sorted(unique_resolved):
        raw_variants = unique_resolved[resolved]
        main_bins = path_as_main.get(resolved, [])
        # bins that embed any raw variant as include
        incl_bins = []
        for raw in raw_variants:
            incl_bins += path_as_include.get(raw, [])
        incl_bins = sorted(set(incl_bins))
        kinds = []
        has_cs_main = any(r.endswith('.cs.hlsl') for r in raw_variants)
        has_raster_main = any(r.endswith('.raster.hlsl') for r in raw_variants)
        if main_bins:
            kind = 'compute_main' if has_cs_main else ('raster_main' if has_raster_main else 'main')
        else:
            kind = 'include_only'
        mapping.append({
            'path': resolved,
            'raw_path_variants': raw_variants,
            'kind': kind,
            'main_in_bins': main_bins,
            'include_in_bins': incl_bins,
            'main_count': len(main_bins),
            'include_count': len(incl_bins),
        })

    # 3) include-graph.json
    include_only = sorted(f for f in all_files if f not in observed)
    shared_usage = {}
    for inc in include_only:
        bins = path_as_include.get(inc, [])
        mains = sorted({r['main_source'] for r in per_bin if isinstance(r.get('main_source'), str)
                        and inc in r['includes']})
        shared_usage[inc] = {'include_bin_count': len(set(bins)), 'used_by_mains': mains}

    include_graph = {
        'method': 'Per-bin embedded source path sets (OpString/OpSource). '
                  'include_closures (main->its include set) are Observation. '
                  'direct_include_edges are the minimal DAG consistent with all '
                  'cross-bin ordering observations (Inference): a file G is a '
                  'direct include of F when G is in F\'s closure and G is not a '
                  'majority-order descendant of another file in that closure. '
                  'Sibling-vs-descendant is ambiguous from preorder, so some '
                  'real direct edges may be under-reported.',
        'include_closures': {k: sorted(v) for k, v in sorted(observed.items())},
        'direct_include_edges': {k: sorted(v) for k, v in sorted(direct.items())},
        'include_only_files': include_only,
        'shared_inc_usage': shared_usage,
    }

    (OUT_DIR / 'per-shader-mapping.json').write_text(
        json.dumps({
            'generated': '2026-10-13',
            'source_dir': str(SHADER_DIR),
            'bins': len(per_bin),
            'unique_raw_paths': len(unique_raw),
            'expected_raw_paths': 419,
            'unique_resolved_paths': len(unique_resolved),
            'path_resolution_note': 'raw embedded strings are kept verbatim; '
                                    '`../` components are collapsed to a logical file '
                                    'for the skeleton tree. See raw_path_variants.',
            'paths': mapping,
        }, indent=1, ensure_ascii=False), encoding='utf-8')

    (OUT_DIR / 'include-graph.json').write_text(
        json.dumps(include_graph, indent=1, ensure_ascii=False), encoding='utf-8')

    # 4) source-tree.md
    tree = build_tree_text(unique_resolved)
    (OUT_DIR / 'source-tree.md').write_text(tree, encoding='utf-8')

    # 5) summary
    empty = [r['bin'] for r in per_bin if not r.get('sources')]
    no_main = [r['bin'] for r in per_bin
               if r.get('sources') and not isinstance(r.get('main_source'), str)]
    sum_lines = [
        '# shader-source skeleton extraction summary',
        '',
        'Date: 2026-10-13. Input: %d .bin under compiled-shaders/ (read-only oracle).' % len(per_bin),
        '',
        '## Counts (Observation)',
        '- bins: %d (%d compute 1-module, %d raster 2-module)' % (
            len(per_bin),
            sum(1 for r in per_bin if len(r.get('modules', [])) == 1),
            sum(1 for r in per_bin if len(r.get('modules', [])) == 2)),
        '- unique embedded source paths (raw strings): %d (prior agent: 419)' % len(unique_raw),
        '- unique resolved (../-collapsed) paths: %d' % len(unique_resolved),
        '- files compiled as main in >=1 bin: %d; include-only files: %d' % (
            sum(1 for p in mapping if p['kind'] != 'include_only'),
            sum(1 for p in mapping if p['kind'] == 'include_only')),
        '- include-only files that are `.inc.hlsl`: %d' % sum(
            1 for p in mapping if p['kind'] == 'include_only' and p['path'].endswith('.inc.hlsl')),
        '- include-only files that are plain `.hlsl` (non-inc): %d' % sum(
            1 for p in mapping if p['kind'] == 'include_only' and not p['path'].endswith('.inc.hlsl')),
        '',
        '## Empty-path shaders (need Investigation)',
        '- bins with empty embedded path set: %d %s' % (
            len(empty), json.dumps(empty[:20], ensure_ascii=False)),
        '- bins with sources but no unique main match: %d %s' % (
            len(no_main), json.dumps(no_main[:20], ensure_ascii=False)),
        '',
        '## Include graph (Inference)',
        '- observed main->include closure edges: %d' % sum(len(v) for v in observed.values()),
        '- direct-include hypothesis edges (majority-consensus minimal DAG): %d' % sum(
            len(v) for v in direct.values()),
        '- top shared inc files by bin count: %s' % ', '.join(
            '%s=%d' % (k, v['include_bin_count'])
            for k, v in sorted(shared_usage.items(), key=lambda kv: -kv[1]['include_bin_count'])[:12]),
        '',
        '## Open unknowns',
        '- exact #include nesting (transitive reduction is the minimal consistent graph).',
        '- full source text is not embedded (OpSource holds newline-padding only).',
        '- per-file line tables vs embedded path order edge cases.',
    ]
    (OUT_DIR / 'summary.md').write_text('\n'.join(sum_lines) + '\n', encoding='utf-8')

    print('wrote', OUT_DIR)
    print('bins:', len(per_bin), 'raw paths:', len(unique_raw),
          'resolved:', len(unique_resolved),
          'empty:', len(empty), 'no-main:', len(no_main))


def build_tree_text(unique_resolved):
    """Directory tree of resolved source paths with per-dir file counts.

    Paths carry the leading `shaders/` root; the root is rendered as the
    top-level node (no duplicated `shaders/shaders/` level).
    """
    root = {}
    for p in unique_resolved:
        parts = p.split('/')
        assert parts[0] == 'shaders', p
        parts = parts[1:]                      # drop the leading shaders/ root
        node = root
        for part in parts[:-1]:
            node = node.setdefault(part, {})
        node.setdefault('__files__', []).append(parts[-1])

    def count_files(node):
        c = len(node.get('__files__', []))
        for d in node:
            if not d.startswith('__'):
                c += count_files(node[d])
        return c

    total = count_files(root)
    lines = []
    lines.append('# Tiny Glade shader source tree (resolved embedded paths)')
    lines.append('')
    lines.append('Total files: %d' % total)
    lines.append('')

    def emit(node, depth=0):
        dirs = sorted(k for k in node if not k.startswith('__'))
        files = sorted(node.get('__files__', []))
        for d in dirs:
            sub = node[d]
            lines.append('%s%s/  (%d files)' % ('    ' * depth, d, count_files(sub)))
            emit(sub, depth + 1)
        for f in files:
            lines.append('%s%s' % ('    ' * (depth + 1), f))

    lines.append('shaders/')
    emit(root, depth=0)
    return '\n'.join(lines) + '\n'


if __name__ == '__main__':
    main()
