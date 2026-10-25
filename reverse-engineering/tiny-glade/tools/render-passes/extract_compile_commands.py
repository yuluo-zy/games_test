#!/usr/bin/env python3
"""Extract per-bin dxc compile commands from Tiny Glade's compiled-shaders .bin files.

The game ships 404 precompiled SPIR-V shader `.bin` files; every SPIR-V module
embeds its exact dxc invocation as the string `dxc-cl-option: <args>` plus, via
`-Zi -Qembed_debug`, the full set of source/include paths it was built from.

For each `.bin` this script:
  1. locates every embedded `dxc-cl-option:` command string (one per SPIR-V module),
  2. parses it into structured args (input temp path, `-E` entry, `-T` profile,
     canonical flags, `-Fo` output temp path),
  3. collects every embedded source/include path (`*.hlsl`) as ASCII strings,
  4. reads the SPIR-V `OpEntryPoint` execution model to confirm the stage mapping,
and emits `evidence/render-passes/validation/compile-commands.json`.

This is analysis-only evidence generation over the read-only game `.bin` oracle
(the 404 bins are never modified). It does not run the game.

Usage:
    extract_compile_commands.py <shader_dir> <out_compile_commands.json>

Output schema (compile_commands.json):
    generated, source_dir, shader_count, module_count, missing_dxc_cl_option,
    canonical_flags, compile_commands: [ per bin {
        file, file_sha256, size, module_count, main_source_path,
        embedded_source_paths: [all *.hlsl paths, first-seen order],
        modules: [ { index, offset, entry, profile, stage, execution_model,
                     raw_command, args: {input_temp, output_temp, entry, profile, flags} } ]
    } ]
"""

import hashlib
import json
import re
import struct
import sys
from pathlib import Path

SPIRV_MAGIC = b'\x03\x02\x23\x07'
ASCII = re.compile(rb'[\x20-\x7e]{4,}')
HLSL_PATH = re.compile(rb'[A-Za-z0-9_./\\\-]+\.hlsl')
HEX16 = re.compile(r'^[0-9a-f]{16}$')

EXEC_MODELS = {
    0: 'Vertex', 1: 'TessellationControl', 2: 'TessellationEvaluation',
    3: 'Geometry', 4: 'Fragment', 5: 'GLCompute', 6: 'Kernel',
    7: 'TaskNV', 8: 'MeshNV', 5264: 'RayGeneration', 5265: 'Intersection',
    5266: 'AnyHit', 5267: 'ClosestHit', 5268: 'Miss', 5295: 'Callable',
    5313: 'TaskEXT', 5314: 'MeshEXT',
}

STAGE_FROM_PROFILE = {
    'cs': 'compute', 'vs': 'vertex', 'ps': 'fragment', 'hs': 'hull',
    'ds': 'domain', 'gs': 'geometry', 'ms': 'mesh', 'as': 'amplification',
    'lib': 'library', 'rgen': 'raygen', 'rchit': 'closest_hit', 'rmiss': 'miss',
}


def spirv_modules(b):
    """Yield (offset, payload) for each SPIR-V module found in the byte blob."""
    cursor = 0
    while True:
        at = b.find(SPIRV_MAGIC, cursor)
        if at < 0:
            break
        n = struct.unpack_from('<I', b, at - 4)[0]
        payload = b[at:at + 4 * n]
        if len(payload) != 4 * n or n == 0:
            break
        yield at, payload
        cursor = at + 4 * n


def exec_model(payload):
    """Return the (OpEntryPoint execution model, entry name) of a module."""
    n = len(payload) // 4
    w = struct.unpack('<' + 'I' * n, payload)
    i = 5  # skip header
    while i < n:
        wc = w[i] >> 16
        op = w[i] & 0xffff
        if wc == 0 or i + wc > n:
            break
        a = w[i + 1:i + wc]
        if op == 15:  # OpEntryPoint: ExecutionModel, <id>, Name, interface...
            if len(a) >= 3:
                name = struct.pack('<' + 'I' * (len(a) - 2), *a[2:]).split(b'\x00')[0]
                return EXEC_MODELS.get(a[0], a[0]), name.decode('utf-8', 'replace')
        i += wc
    return None, None


def embedded_hlsl_paths(b):
    """All *.hlsl source/include paths embedded as ASCII strings, first-seen order."""
    seen = []
    seen_set = set()
    for s in ASCII.findall(b):
        m = HLSL_PATH.search(s)
        if not m:
            continue
        p = m.group(0).decode('utf-8', 'replace').strip()
        if p not in seen_set:
            seen_set.add(p)
            seen.append(p)
    return seen


def parse_command(raw):
    """Parse a `dxc-cl-option: ...` string into structured args."""
    body = raw.split('dxc-cl-option:', 1)[1].strip() if 'dxc-cl-option:' in raw else raw.strip()
    toks = body.split()
    args = {'input_temp': None, 'output_temp': None, 'entry': None,
            'profile': None, 'flags': []}
    i = 0
    while i < len(toks):
        t = toks[i]
        if t == '-E':
            args['entry'] = toks[i + 1] if i + 1 < len(toks) else None
            i += 2
        elif t == '-T':
            args['profile'] = toks[i + 1] if i + 1 < len(toks) else None
            i += 2
        elif t == '-Fo':
            args['output_temp'] = toks[i + 1] if i + 1 < len(toks) else None
            i += 2
        elif t == '-HV':
            args['flags'].append(t + ' ' + (toks[i + 1] if i + 1 < len(toks) else ''))
            i += 2
        elif t.startswith('-'):
            args['flags'].append(t)
            i += 1
        else:
            if args['input_temp'] is None:
                args['input_temp'] = t  # first bare token is the input source path
            i += 1
    return args


def filename_pass_stem(name):
    """Return (pass_stem, stage) from a bin filename.

    _<pass>.<stage>.hlsl_<entry>[_<variant>].bin  ->  (<pass>, cs|raster)
    The pass stem is the on-disk path with '/' replaced by '_' (unambiguous only
    in this direction; the reverse split needs the embedded paths as candidates).
    """
    m = re.match(r'^_(.+)\.(cs|raster)\.hlsl_(.+?)\.bin$', name)
    if not m:
        return None, None
    return m.group(1), m.group(2)


def roundtrip(p, stage):
    """On-disk path (include-root relative, stage ext stripped) as the filename pass stem."""
    ext = '.%s.hlsl' % stage  # '.cs.hlsl' or '.raster.hlsl'
    rel = p[len('shaders/'):] if p.startswith('shaders/') else p
    if rel.endswith(ext):
        rel = rel[:-len(ext)]
    return rel.replace('/', '_')


def main_source_match(embedded, stage, pass_stem):
    """Pick the embedded main source for a bin.

    The filename pass stem is the on-disk path (relative to the include root,
    stage extension stripped) with '/' replaced by '_' and the leading '_'
    stripped. This roundtrip is lossless in the embedded -> stem direction, so we
    pick the embedded non-include path whose roundtrip equals the pass stem.
    """
    ext = '.%s.hlsl' % stage
    candidates = [p for p in embedded if p.endswith(ext)]
    for p in candidates:
        if roundtrip(p, stage) == pass_stem:
            return p, 'roundtrip_exact'
    if candidates:
        return candidates[0], 'first_noninclude'
    return (embedded[0] if embedded else None), 'fallback_first'


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    shader_dir = Path(sys.argv[1])
    out_path = Path(sys.argv[2])

    bins = sorted(shader_dir.glob('*.bin'))
    if not bins:
        sys.exit('no .bin files found in %s' % shader_dir)

    canonical_flags = []
    compile_commands = []
    missing = []
    total_modules = 0
    main_source_crosscheck = []

    for p in bins:
        b = p.read_bytes()
        raw_commands = []
        start = 0
        while True:
            at = b.find(b'dxc-cl-option', start)
            if at < 0:
                break
            end = b.find(b'\x00', at)
            if end < 0:
                end = at + 300
            raw_commands.append(b[at:end].decode('utf-8', 'replace'))
            start = at + 1

        if not raw_commands:
            missing.append(p.name)

        embedded = embedded_hlsl_paths(b)
        pass_stem, stage = filename_pass_stem(p.name)
        main_source, ms_method = main_source_match(embedded, stage, pass_stem)
        main_source_crosscheck.append({
            'file': p.name,
            'main_source_path': main_source,
            'main_source_method': ms_method,
        })

        modules = []
        for idx, (offset, payload) in enumerate(spirv_modules(b)):
            raw = raw_commands[idx] if idx < len(raw_commands) else None
            args = parse_command(raw) if raw else None
            emodel, ename = exec_model(payload)
            profile = args['profile'] if args else None
            stage = STAGE_FROM_PROFILE.get(profile.split('_')[0], 'unknown') if profile else 'unknown'
            modules.append({
                'index': idx,
                'offset': offset,
                'entry': (args or {}).get('entry') or ename,
                'profile': profile,
                'stage': stage,
                'execution_model': emodel,
                'raw_command': raw,
                'args': args,
            })
            if args and args['flags'] and not canonical_flags:
                canonical_flags = args['flags']
            total_modules += 1

        compile_commands.append({
            'file': p.name,
            'file_sha256': hashlib.sha256(b).hexdigest(),
            'size': len(b),
            'module_count': len(modules),
            'main_source_path': main_source,
            'embedded_source_paths': embedded,
            'modules': modules,
        })

    out = {
        'generated': '2026-10-13',
        'source_dir': str(shader_dir),
        'shader_count': len(bins),
        'module_count': total_modules,
        'missing_dxc_cl_option': missing,
        'canonical_flags': canonical_flags,
        'compile_commands': compile_commands,
        'main_source_crosscheck': {
            'total': len(main_source_crosscheck),
            'roundtrip_exact': sum(1 for c in main_source_crosscheck if c['main_source_method'] == 'roundtrip_exact'),
            'first_noninclude': sum(1 for c in main_source_crosscheck if c['main_source_method'] == 'first_noninclude'),
            'fallback_first': sum(1 for c in main_source_crosscheck if c['main_source_method'] == 'fallback_first'),
            'checks': main_source_crosscheck,
        },
        'limitations': [
            'dxc version used by the original build is not embedded (SPIR-V generator word is 0x0); '
            'recompilation must rely on structural fingerprint + normalized hash, not byte parity.',
            'The `-I` include root of the original build is inferred as the directory containing `shaders/` '
            'because embedded include paths are relative to it (e.g. `inc/common.hlsl`).',
            '`input_temp`/`output_temp` are ephemeral `/tmp/.tmpXXXX` paths from the original build; '
            'the real source path is `main_source_path`, recovered from the embedded debug info.',
        ],
    }

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(out, indent=2), encoding='utf-8')
    print('wrote', out_path)
    print('bins:', len(bins), 'modules:', total_modules,
          'missing dxc-cl-option:', len(missing))
    print('main-source roundtrip_exact:',
          out['main_source_crosscheck']['roundtrip_exact'], '/', len(main_source_crosscheck))
    print('canonical flags:', canonical_flags)


if __name__ == '__main__':
    main()
