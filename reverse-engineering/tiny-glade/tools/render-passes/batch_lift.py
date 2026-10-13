#!/usr/bin/env python3
"""Batch-replace under-fidelity reconstructed shaders with the spirv-cross lift of their oracle bins.

For each target main source, every oracle bin that names it as main_source is lifted
(spirv-cross --hlsl, per SPIR-V module) and the file is rewritten as variant/stage-guarded blocks:

    #if defined(TGV_<binhash>) && defined(__SHADER_STAGE_PIXEL) ... #endif

The sweep/recompile tooling defines TGV_<binhash> (or TGV_0 for hash-less bins) per bin, so the
variants need no hand-kept manifest. A rewritten file is kept only if every module compiles,
the binding hard gate passes and the instruction ratio lands in [lo, hi]; otherwise it is restored
with `git checkout`. Needs spirv-cross on PATH and $DXC / LD_LIBRARY_PATH like sweep_all.py.

Usage:
  batch_lift.py --from-sweep [--below 0.8] [--above 1.25] [--only SUBSTR] [--jobs N] [--dry-run]
  batch_lift.py FILE.hlsl [FILE.hlsl ...]          (paths relative to shaders/)
Writes evidence/render-passes/integration/batch-lift-report.json
"""
import argparse, json, os, re, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SRC = ROOT / 'reconstruction/shader-source/shaders'
ORACLE = ROOT / 'evidence/render-passes/shaders/oracle-bin'
REPORT = ROOT / 'evidence/render-passes/integration/batch-lift-report.json'
sys.path.insert(0, str(HERE))
import spirv_parse  # noqa: E402
import sweep_all  # noqa: E402
from recompile_check import parse_target_bin  # noqa: E402

GRAMMAR = ROOT / 'skeleton' / '_tools' / 'spirv.core.grammar.json'
if not GRAMMAR.exists():
    GRAMMAR = ROOT / 'evidence/render-passes/../../skeleton/_tools/spirv.core.grammar.json'
STAGE_MACRO = {'cs': '__SHADER_STAGE_COMPUTE', 'vs': '__SHADER_STAGE_VERTEX', 'ps': '__SHADER_STAGE_PIXEL'}
SUBMUL_HELPER = '''// dxc factors (a*c) - (b*c) into (a-b)*c, which rounds differently from the original; precise temporaries keep both products.
template<typename T> T spv_fmsub(T a, T b, float c) { precise T p = a * c; precise T q = b * c; precise T r = p - q; return r; }
'''

FREXP_HELPER = '''// dxc 1.9/1.10 lowers the native frexp() to invalid SPIR-V (OpConvertSToF on a non-int); same
// workaround as inc/curve.hlsl.
float3 incb_frexp(float3 x, out int3 exponent)
{
    // denormals are normalised first (x * 2^24) so the exponent/mantissa match GLSL Frexp exactly
    uint3 bits0 = asuint(x);
    bool3 denorm = ((bits0 & 0x7F800000u) == 0u) & ((bits0 & 0x7FFFFFFFu) != 0u);
    float3 xs = select(denorm, x * 16777216.0f, x);
    uint3 bits = asuint(xs);
    int3 biased = int3(int((bits.x >> 23u) & 0xFFu), int((bits.y >> 23u) & 0xFFu), int((bits.z >> 23u) & 0xFFu));
    int3 e = biased - int3(127, 127, 127) - select(denorm, int3(24, 24, 24), int3(0, 0, 0));
    float3 m = asfloat((bits & 0x807FFFFFu) | 0x3F800000u);
    bool3 big = (m >= 1.0f) | (m <= -1.0f);
    m = lerp(m, m * 0.5f, float3(big));
    e += int3(big);
    bool3 special = (biased == 0) | (biased == 0xFF);
    exponent = select(special, int3(0, 0, 0), e);
    return select(special, xs, m);
}
#define frexp incb_frexp
'''
PRAGMA = ('#pragma dxc diagnostic push\n#pragma dxc diagnostic ignored "-Wconversion"\n'
          '#pragma dxc diagnostic ignored "-Wunused-value"\n#pragma dxc diagnostic ignored "-Wambig-lit-shift"\n')


def variant_id(bin_name):
    return sweep_all.tgv_id(bin_name)


def snake(camel):
    return re.sub(r'(?<!^)(?=[A-Z])', '_', camel).lower()


def fix_cbuffer_names(text):
    """spirv-cross names cbuffers after their struct type; the SPIR-V binding name is the variable."""
    def rep(m):
        t = m.group(1)
        if t == 'type_Globals':
            name = '$Globals'
        elif t.startswith('type_ConstantBuffer_'):
            name = snake(t[len('type_ConstantBuffer_'):])
        elif t.startswith('type_'):
            name = t[len('type_'):]
        else:
            return m.group(0)
        return 'cbuffer %s' % name
    return re.sub(r'cbuffer (type_\w+)', rep, text)


def fix_push_constants(text):
    m = re.search(r'cbuffer push_constants\s*\{(.*?)\};\n', text, re.S)
    if not m:
        return text
    members = re.findall(r'(\w+(?:<\w+>)?) push_constants_(\w+)\s*(?:\[\d+\])?\s*: packoffset\([^)]*\);', m.group(1))
    decl = 'struct PushConstants\n{\n%s};\n[[vk::push_constant]]\nPushConstants push_constants;\n' % ''.join('    %s %s;\n' % x for x in members)
    text = text[:m.start()] + decl + text[m.end():]
    return re.sub(r'\bpush_constants_(\w+)', r'push_constants.\1', text)


def fix_bool_casts(text):
    """float(a == b) -> (a == b ? 1.0f : 0.0f): dxc 1.9 emits an invalid ConvertSToF for bool operands."""
    out, i = [], 0
    while True:
        m = re.compile(r'\bfloat\(').search(text, i)
        if not m:
            out.append(text[i:]); break
        j, depth, top = m.end(), 1, ''
        while depth:
            c = text[j]
            depth += (c == '(') - (c == ')')
            if depth == 1 or (depth == 0 and False):
                top += c
            j += 1
        inner = text[m.end():j - 1]
        flat = re.sub(r'\([^()]*\)', '', top)
        flat = re.sub(r'\b\w+<[\w, ]+>', 'T', flat)  # template args (Load<uint>) are not comparisons
        if re.search(r'(?<![<>!=])(==|!=|<=|>=|<|>)(?![<>=])|&&|\|\|', re.sub(r'<<|>>', '', flat)) and '?' not in flat:
            out.append(text[i:m.start()] + '((%s) ? 1.0f : 0.0f)' % inner)
        else:
            out.append(text[i:m.end()]); j = m.end()
        i = j
    return ''.join(out)


def fix_bool_vector_casts(text):
    """floatN(boolN(...)) -> select(boolN(...), 1, 0): same dxc 1.9 ConvertSToF bug for vectors."""
    pat = re.compile(r'\bfloat([234])\((bool\1\()')
    while True:
        m = pat.search(text)
        if not m:
            return text
        n, j, depth = m.group(1), m.end(), 1
        while depth:
            depth += (text[j] == '(') - (text[j] == ')'); j += 1
        inner_end = j  # index after the matching ')' of boolN(
        # the outer floatN( closes right after
        assert text[inner_end] == ')'
        cond = text[m.start(2):inner_end]
        text = text[:m.start()] + 'select(%s, float%s(%s), float%s(%s))' % (
            cond, n, ', '.join(['1.0f'] * int(n)), n, ', '.join(['0.0f'] * int(n))) + text[inner_end + 1:]


def fix_io_locations(text):
    """spirv-cross keeps the original location in the semantic (`: TEXCOORD2`) but dxc renumbers semantic
    fields sequentially (0,2,3 -> 0,1,2). Pin them with [[vk::location(N)]] so stage interfaces stay put."""
    return re.sub(r'^(\s+)((?:nointerpolation |noperspective )?\w+(?:\d)? \w+ : TEXCOORD(\d+);)',
                  lambda m: '%s[[vk::location(%s)]] %s' % (m.group(1), m.group(3), m.group(2)), text, flags=re.M)


_SREM = re.compile(r'(\b\w+) - ((?:\w+|\([^()]*\))(?: \* (?:\w+|\([^()]*\)))*) \* \(\1 / \2\)')


def fix_srem(text):
    """spirv-cross expands OpSRem to `a - b * (a / b)`; with b == 0 that hits an integer divide-by-zero
    (UB) where the original executes a plain remainder. Emit `%` (dxc -> OpSRem) when b is not a literal."""
    return _SREM.sub(lambda m: m.group(0) if re.fullmatch(r'\d+', m.group(2)) else '(%s %% %s)' % (m.group(1), m.group(2)), text)


def fix_lift_bugs(text):
    text = fix_io_locations(fix_bool_vector_casts(fix_bool_casts(text)))
    text = re.sub(r'(\w+)\.mantissa = frexp\((.+?), \1\.exponent\);',
                  r'{ int3 _fe; \1.mantissa = frexp(\2, _fe); \1.exponent = _fe; }', text)
    text = re.sub(r'\b_alias_', '__alias_', text)  # spirv-cross collapses the double underscore
    text = fix_srem(text)
    text = re.sub(r'\(\(([\w.]+) \* ([\d.]+f)\) - \(([\w.]+) \* \2\)\)', r'spv_fmsub(\1, \3, \2)', text)
    # dxc reassociates (c*a)*(c*a) -> (c*c)*(a*a), which flips a sphere/AABB boundary test in
    # bin_lights_to_cells (n_14dca8ba); `precise` keeps the original evaluation order.
    text = re.sub(r'\bfloat (_\d+) = ([\d.]+f) \* asfloat\(532487669', r'precise float \1 = \2 * asfloat(532487669', text)
    for v in set(re.findall(r'precise float (_\d+) = [\d.]+f \* asfloat\(532487669', text)):
        text = re.sub(r'\bfloat (_\d+) = %s \* %s;' % (v, v), r'precise float \1 = %s * %s;' % (v, v), text)
    # nested-loop array copy emitted with both indices: collapse to the inner loop.
    pat = re.compile(r'\[unroll\]\s*for \(int (_\d+ident) = 0; \1 < (\d+); \1\+\+\)\s*\{\s*'
                     r'\[unroll\]\s*for \(int (_\d+ident) = 0; \3 < \2; \3\+\+\)\s*\{\s*'
                     r'(\w+)\[\1\]\[\3\] = (.*?);\s*\}\s*\}', re.S)
    def collapse(m):
        i, n, j, arr, expr = m.groups()
        expr = expr.replace('%s * 4 + ' % i, '')
        return '[unroll]\n    for (int %s = 0; %s < %s; %s++)\n    {\n        %s[%s] = %s;\n    }' % (j, j, n, j, arr, j, expr)
    return pat.sub(collapse, text)


def _logical(payload):
    """PhysicalStorageBuffer64 -> Logical so spirv-cross' HLSL backend accepts the module."""
    import struct
    w = list(struct.unpack('<%dI' % (len(payload) // 4), payload))
    i, phys = 5, False
    while i < len(w):
        wc, op = w[i] >> 16, w[i] & 0xffff
        if op == 14 and w[i + 1] == 5348:
            w[i + 1], phys = 0, True
        i += wc
    return struct.pack('<%dI' % len(w), *w), phys


def fix_physical_loads(text):
    """spirv-cross prints a PhysicalStorageBuffer deref as `(ptr).value` / `asfloat(ptr).value`, where
    ptr is a uint2 address. Rewrite to vk::RawBufferLoad<T>; T comes from the declaration of the target."""
    def load(t, ptr):
        n = 1 if t in ('uint', 'int', 'float') else int(t[-1])
        return 'vk::RawBufferLoad<%s>(((uint64_t)%s.y << 32) | %s.x, %d)' % (t, ptr, ptr, 4 if n == 3 else 4 * min(n, 4))

    deref = r'(?:asfloat\((\w+)\)|\((\w+)\))\.value'
    # declaration: `T v = (ptr).value;`
    text = re.sub(r'(\w+) (\w+) = %s;' % deref,
                  lambda m: '%s %s = %s;' % (m.group(1), m.group(2), load(m.group(1), m.group(3) or m.group(4))), text)

    # assignment to a variable declared earlier: look the type up
    def assign(m):
        d = re.search(r'\b(u?int[234]?|float[234]?) %s\b' % re.escape(m.group(1)), text[:m.start()])
        return m.group(0) if not d else '%s = %s;' % (m.group(1), load(d.group(1), m.group(2) or m.group(3)))
    return re.sub(r'(\w+) = %s;' % deref, assign, text)


def lift_module(payload):
    payload, phys = _logical(payload)
    with tempfile.NamedTemporaryFile(suffix='.spv') as f:
        f.write(payload); f.flush()
        p = subprocess.run(['spirv-cross', '--hlsl', '--shader-model', '64', f.name],
                           capture_output=True, text=True)
    if p.returncode:
        raise RuntimeError(p.stderr.strip()[:300])
    return fix_physical_loads(p.stdout) if phys else p.stdout


def hand_original(rel):
    """The hand-written baseline of a source file (git HEAD), so repeated patching never nests wrappers."""
    try:
        p = subprocess.run(['git', 'show', 'HEAD:' + str((SRC / rel).relative_to(ROOT.parents[1]))], capture_output=True, text=True, cwd=ROOT)
        if p.returncode == 0 and p.stdout:
            return p.stdout
    except Exception:
        pass
    return open(SRC / rel, errors='replace').read()


def lift_file(rel, pairs=None):
    """Return new file text, or raise. pairs = {(bin_name, entry)} limits lifting to those modules
    and switches to patch mode (hand-written code is kept, guarded off for exactly those modules)."""
    g = spirv_parse.Grammar(str(GRAMMAR))
    bins = []
    for line in open(ROOT / 'evidence/render-passes/skeleton/raw-per-bin.jsonl'):
        b = json.loads(line)
        if (b.get('main_source') or '').split('shaders/', 1)[-1] == rel:
            bins.append(b['bin'])
    blocks, guards = [], []
    for bn in sorted(bins):
        vid = variant_id(bn)
        for t in parse_target_bin(ORACLE / bn, g):
            if pairs is not None and (bn, t['entry']) not in pairs:
                continue
            stage = {'cs_6': 'cs', 'vs_6': 'vs', 'ps_6': 'ps'}[t['profile'][:4]]
            text = fix_lift_bugs(fix_push_constants(fix_cbuffer_names(lift_module(t['payload']))))
            text = re.sub(r'\n(\w+) main\(', '\n\\1 %s(' % t['entry'], text)
            guards.append('(defined(TGV_%s) && (__SHADER_TARGET_STAGE == %s))' % (vid, STAGE_MACRO[stage]))
            blocks.append('#if defined(TGV_%s) && (__SHADER_TARGET_STAGE == %s)\n// lifted from %s (spirv-cross; behaviour-exact, names are SPIR-V ids)\n%s%s%s#pragma dxc diagnostic pop\n#endif\n'
                          % (vid, STAGE_MACRO[stage], bn, PRAGMA, (FREXP_HELPER if 'frexp(' in text else '') + (SUBMUL_HELPER if 'spv_fmsub(' in text else ''), text))
    if pairs is not None:
        orig = hand_original(rel)
        return ('// PATCHED: modules %s are lifted from the oracle bins by tools/render-passes/batch_lift.py;\n'
                '// the hand-written code below is excluded for exactly those (variant, stage) pairs.\n'
                '#if !(%s)\n%s\n#endif\n\n%s') % (', '.join(sorted('%s:%s' % (b[-24:], e) for b, e in pairs)), ' || '.join(guards), orig, '\n'.join(blocks))
    first = open(SRC / rel, errors='replace').read().splitlines()[0]
    head = '%s\n// LIFTED: body generated by tools/render-passes/batch_lift.py from the oracle bins\n// (the hand-written version scored below the fidelity gate). Variants selected via TGV_<binhash>.\n\n' % first
    return head + '\n'.join(blocks)


def lift_and_check(rel, pairs=None):
    path = SRC / rel
    old = path.read_text()
    try:
        text = lift_file(rel, pairs)
        if old.startswith('// KNOWN EXCEPTION') and '// PATCHED:' in old:
            text = old[:old.index('// PATCHED:')] + text  # keep the hand-written exception note
        path.write_text(text)
    except Exception as e:
        path.write_text(old)
        return {'file': rel, 'status': 'LIFT_FAIL', 'detail': str(e)[:300]}
    return {'file': rel, 'status': 'WRITTEN', '_old': old}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('files', nargs='*')
    ap.add_argument('--from-sweep', action='store_true')
    ap.add_argument('--below', type=float, default=0.8)
    ap.add_argument('--above', type=float, default=1.25)
    ap.add_argument('--only')
    ap.add_argument('--jobs', type=int, default=os.cpu_count() or 4)
    ap.add_argument('--accept-lo', type=float, default=0.8)
    ap.add_argument('--accept-hi', type=float, default=2.0)
    ap.add_argument('--max-dist', type=float, default=0.3, help='patch mode: also lift modules whose opcode-histogram distance is >= this')
    ap.add_argument('--refresh', action='store_true', help='regenerate every lifted/patched file (after changing the lifter)')
    ap.add_argument('--from-diff', help='diff_exec.py JSON: lift (bin, entry) pairs whose verdict is DIFF (implies --patch)')
    ap.add_argument('--bins', help='patch mode: comma-separated bin-name substrings to restrict the lifted modules')
    ap.add_argument('--patch', action='store_true', help='lift only failing (bin,entry) modules, keep hand-written code')
    ap.add_argument('--dry-run', action='store_true')
    a = ap.parse_args()
    targets = list(a.files)
    if a.from_sweep:
        sw = json.load(open(ROOT / 'evidence/render-passes/integration/fidelity-sweep.json'))
        for r in sw['results']:
            if 'f0ad' in r['bin']:
                continue
            for m in r.get('modules', []):
                if m.get('ratio') is not None and (m['ratio'] < a.below or m['ratio'] > a.above):
                    targets.append(r['hlsl'])
    pairmap, olddist = {}, {}
    if a.refresh:
        g = spirv_parse.Grammar(str(GRAMMAR))
        stage_of = {'__SHADER_STAGE_VERTEX': 'vs', '__SHADER_STAGE_PIXEL': 'ps', '__SHADER_STAGE_COMPUTE': 'cs'}
        a.patch = True
        for f in SRC.rglob('*.hlsl'):
            rel = str(f.relative_to(SRC))
            head = f.read_text(errors='replace')[:1200]
            if 'LIFTED: body generated' in head:
                targets.append(rel)
            elif '// PATCHED:' in head:
                full = f.read_text(errors='replace')
                pairs = set()
                for m in re.finditer(r'#if defined\(TGV_\w+\) && \(__SHADER_TARGET_STAGE == (__SHADER_STAGE_\w+)\)\n// lifted from (\S+)', full):
                    st, bn = stage_of[m.group(1)], m.group(2)
                    for t in parse_target_bin(ORACLE / bn, g):
                        if t['profile'][:2] == st:
                            pairs.add((bn, t['entry']))
                pairmap[rel] = pairs
        targets = sorted(set(targets) | set(pairmap))
    elif a.from_diff:
        a.patch = True
        for r in json.load(open(a.from_diff)):
            if r['verdict'] == 'DIFF':
                pairmap.setdefault(r['file'], set()).add((r['bin'], r['entry']))
                olddist[(r['file'], r['bin'], r['entry'])] = 99
        targets = [t for t in pairmap if not a.only or a.only in t]
    elif a.patch:
        sw = json.load(open(ROOT / 'evidence/render-passes/integration/fidelity-sweep.json'))
        for r in sw['results']:
            for m in r.get('modules', []):
                if m.get('ratio') is not None and (not m['hard_pass'] or m['ratio'] < a.accept_lo or (m.get('hist_dist', 0) >= a.max_dist and m.get('target_ops', 0) >= 60)):
                    pairmap.setdefault(r['hlsl'], set()).add((r['bin'], m['entry']))
                    olddist[(r['hlsl'], r['bin'], m['entry'])] = m.get('hist_dist', 99)
        if a.bins:
            keys = a.bins.split(',')
            pairmap = {f: {pb for pb in ps if any(k in pb[0] for k in keys)} for f, ps in pairmap.items()}
            pairmap = {f: ps for f, ps in pairmap.items() if ps}
        targets = [t for t in pairmap if not a.only or a.only in t]
    targets = sorted(set(t for t in targets if not a.only or a.only in t))
    print(len(targets), 'targets')
    if a.dry_run:
        print('\n'.join(targets)); return
    # lift sequentially (files are independent but git checkout/ write races are avoided); check in parallel
    results = []
    for rel in targets:
        results.append(lift_and_check(rel, pairmap.get(rel) if (a.patch and rel in pairmap) else None))
    with ThreadPoolExecutor(a.jobs) as ex:
        # re-run the sweep per file for the acceptance check
        def chk(r):
            if r['status'] != 'WRITTEN':
                return r
            out = Path(tempfile.mkdtemp(prefix='bl_')) / 'o.json'
            subprocess.run([sys.executable, str(HERE / 'sweep_all.py'), '--only', r['file'], '--out', str(out)],
                           capture_output=True, text=True, cwd=ROOT)
            sw = json.load(open(out))
            bad = []
            for x in sw['results']:
                if 'error' in x:
                    bad.append('%s: error' % x['bin'][-22:]); continue
                for m in x['modules']:
                    if m.get('ratio') is None:
                        bad.append('%s %s: no-fingerprint(compile fail?)' % (x['bin'][-22:], m['entry']))
                    elif a.patch and (r['file'], x['bin'], m['entry']) in olddist and m['hard_pass'] and a.accept_lo <= m['ratio'] \
                            and (m['hist_dist'] < a.max_dist or m['hist_dist'] < olddist[(r['file'], x['bin'], m['entry'])]):
                        pass  # lifted module improved on the hand-written one
                    elif not m['hard_pass'] or not (a.accept_lo <= m['ratio'] and (m['ratio'] <= a.accept_hi or m.get('target_ops', 99) < 30)):
                        bad.append('%s %s ratio=%s hard=%s' % (x['bin'][-22:], m['entry'], m['ratio'], m['hard_pass']))
            r['status'] = 'REJECTED' if bad else 'ACCEPTED'
            r['problems'] = bad[:6]
            if bad:
                (SRC / r['file']).write_text(r['_old'])
            return r
        results = list(ex.map(chk, results))
    json.dump([{k: v for k, v in r.items() if k != '_old'} for r in results], open(REPORT, 'w'), indent=1, ensure_ascii=False)
    from collections import Counter
    print(Counter(r['status'] for r in results))
    for r in results:
        if r['status'] != 'ACCEPTED':
            print(r['status'], r['file'], '|', (r.get('problems') or [r.get('detail')])[0])


if __name__ == '__main__':
    main()
