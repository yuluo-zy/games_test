#!/usr/bin/env python3
"""Toolchain self-test / proof driver for the shader recompile validator.

Two proofs:

  A. Structural-identity sanity check: the same trivial HLSL compiled twice from
     two different paths must produce the SAME normalized sha256 (and different
     raw sha256). This is the reason raw hashing is useless and the normalized
     hash is the correct path-stable comparison.

  B. Compile-command extraction works: parse one real game `.bin` and show its
     embedded `dxc-cl-option:` command, entry, profile and embedded source paths.

Usage:
    run_self_test.py <out_proof.json>

Requires dxc on PATH (or $DXC). Output is machine-readable proof JSON, meant to
be written under evidence/render-passes/validation/.
"""

import hashlib
import json
import os
import shutil
import struct
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import spirv_parse  # noqa: E402
import recompile_check  # noqa: E402

CANONICAL_FLAGS = recompile_check.CANONICAL_FLAGS


def dxc_version(dxc):
    try:
        out = subprocess.run([dxc, '--version'], capture_output=True, timeout=60)
        text = out.stdout.decode('utf-8', 'replace')
        for line in text.splitlines():
            if 'version' in line.lower() or line.startswith('v'):
                return line.strip()
        return text.splitlines()[0] if text else 'unknown'
    except Exception as e:  # pragma: no cover
        return 'error: %s' % e


def compile_to(dxc, hlsl, out_spv):
    cmd = ([dxc, '-E', 'cs_main', '-T', 'cs_6_4'] + CANONICAL_FLAGS +
           ['-Fo', str(out_spv), str(hlsl)])
    proc = subprocess.run(cmd, capture_output=True, timeout=300)
    return proc.returncode, proc.stderr.decode('utf-8', 'replace')


def main():
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    out_path = Path(sys.argv[1])
    here = Path(__file__).resolve().parent
    dxc = os.environ.get('DXC', shutil.which('dxc') or 'dxc')

    a = here / 'self_test' / 'trivial_a.cs.hlsl'
    b = here / 'self_test' / 'trivial_b.cs.hlsl'

    # ---- proof A: same source, two paths -------------------------------
    tmp = Path(tempfile.mkdtemp(prefix='selftest_'))
    spv_a = tmp / 'a.spv'
    spv_b = tmp / 'b.spv'
    rc_a, err_a = compile_to(dxc, a, spv_a)
    rc_b, err_b = compile_to(dxc, b, spv_b)
    proof_a = {'rc_a': rc_a, 'rc_b': rc_b, 'stderr_a': err_a.strip(),
               'stderr_b': err_b.strip()}
    if rc_a == 0 and rc_b == 0:
        payload_a = spv_a.read_bytes()
        payload_b = spv_b.read_bytes()
        proof_a.update({
            'raw_sha256_a': hashlib.sha256(payload_a).hexdigest(),
            'raw_sha256_b': hashlib.sha256(payload_b).hexdigest(),
            'raw_hashes_equal': hashlib.sha256(payload_a).hexdigest()
                                 == hashlib.sha256(payload_b).hexdigest(),
            'normalized_sha256_a': recompile_check.normalized_sha256(payload_a),
            'normalized_sha256_b': recompile_check.normalized_sha256(payload_b),
            'normalized_hashes_equal':
                recompile_check.normalized_sha256(payload_a)
                == recompile_check.normalized_sha256(payload_b),
            'paths': [str(a), str(b)],
        })
        # fingerprints from both for the structural-identity check
        grammar = spirv_parse.Grammar(recompile_check.DEFAULT_GRAMMAR)
        ma = spirv_parse.parse_module(payload_a, grammar)
        mb = spirv_parse.parse_module(payload_b, grammar)
        fa = spirv_parse.structural_fingerprint(ma)
        fb = spirv_parse.structural_fingerprint(mb)
        proof_a['structural_fingerprint_a'] = fa
        proof_a['structural_fingerprint_b'] = fb
        proof_a['fingerprints_equal'] = fa == fb
        proof_a['bindings_a'] = ma['bindings']
        proof_a['local_size_a'] = spirv_parse.local_size(ma['execution_modes'])

    # ---- proof B: compile-command extraction on one real bin -----------
    bin_path = Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/_blur.cs.hlsl_cs_main_0.bin')
    blob = bin_path.read_bytes()
    modules = []
    for offset, payload in spirv_parse.iter_modules(blob):
        at = payload.find(b'dxc-cl-option')
        raw = payload[at:payload.find(b'\x00', at)].decode('utf-8', 'replace') if at >= 0 else None
        cmd_args = recompile_check.compile_command_from_raw(raw)
        modules.append({
            'offset': offset,
            'raw_command': raw,
            'entry': cmd_args['entry'],
            'profile': cmd_args['profile'],
        })
    proof_b = {
        'bin': str(bin_path),
        'bin_size': len(blob),
        'module_count': len(modules),
        'modules': modules,
        'embedded_source_paths': extract_embedded(bin_path),
    }

    proof = {
        'generated': '2026-10-13',
        'dxc': dxc,
        'dxc_version': dxc_version(dxc),
        'spirv_cross': shutil.which('spirv-cross'),
        'proof_a_structural_identity': proof_a,
        'proof_b_compile_command_extraction': proof_b,
        'conclusion': {
            'a_ok': bool(proof_a.get('normalized_hashes_equal') and proof_a.get('fingerprints_equal')),
            'b_ok': bool(proof_b['modules']) and proof_b['modules'][0]['entry'] == 'cs_main'
                    and proof_b['modules'][0]['profile'] == 'cs_6_4',
        },
    }

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(proof, indent=2), encoding='utf-8')
    print('wrote', out_path)
    print('A normalized hashes equal:', proof_a.get('normalized_hashes_equal'),
          '| raw hashes equal:', proof_a.get('raw_hashes_equal'))
    print('B extracted command:', proof_b['modules'][0]['raw_command'])
    print('conclusion:', proof['conclusion'])


def extract_embedded(bin_path):
    """Fallback: all *.hlsl path strings embedded in a bin."""
    import re
    blob = bin_path.read_bytes()
    ASCII = re.compile(rb'[\x20-\x7e]{4,}')
    HLSL_PATH = re.compile(rb'[A-Za-z0-9_./\\\-]+\.hlsl')
    out, seen = [], set()
    for s in ASCII.findall(blob):
        m = HLSL_PATH.search(s)
        if m:
            p = m.group(0).decode()
            if p not in seen:
                seen.add(p)
                out.append(p)
    return out


if __name__ == '__main__':
    main()
