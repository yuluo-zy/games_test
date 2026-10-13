#!/usr/bin/env python3
"""Replace the ps_main of a reconstructed raster HLSL with the spirv-cross lift of the oracle PS.

The hand-written PS bodies of the deferred passes were invented (ratio 0.25 vs the bin). The
spirv-cross lift of the oracle module compiles with dxc as-is and lands at ratio ~1.1, i.e. it is
behaviour-exact. This tool keeps the reconstructed file's header (includes, cbuffers, bindings,
VS) and swaps only the pixel stage, renaming the spirv-cross prefixes to the tree's global names.

Usage:
  adopt_ps.py <hlsl> --full <b903.bin> [--reduced <f0ad.bin>] [--marker REGEX]
Everything from the first line matching --marker (default: the reservoir-gather comment or
`uint gather_reservoir_lanterns`) to EOF is replaced. Needs spirv-cross on PATH.
"""
import argparse, re, subprocess, sys, tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import spirv_parse  # noqa: E402

GRAMMAR = HERE.parents[0] / 'skeleton' / '_tools' / 'spirv.core.grammar.json'
if not GRAMMAR.exists():
    GRAMMAR = HERE.parents[1] / 'skeleton' / '_tools' / 'spirv.core.grammar.json'


def lift_ps(bin_path):
    """Return spirv-cross HLSL text of the fragment module in bin_path."""
    g = spirv_parse.Grammar(str(GRAMMAR))
    data = Path(bin_path).read_bytes()
    from recompile_check import parse_target_bin
    for t in parse_target_bin(Path(bin_path), g):
        if t['entry'] != 'ps_main':
            continue
        with tempfile.NamedTemporaryFile(suffix='.spv') as f:
            f.write(t['payload']); f.flush()
            return subprocess.run(['spirv-cross', '--hlsl', '--shader-model', '64', f.name],
                                  capture_output=True, text=True, check=True).stdout
    raise SystemExit('no ps_main module in %s' % bin_path)


def adapt(text):
    """Split the lift into (statics+helpers, body) and rename to the tree's globals."""
    head, _, rest = text.partition('void frag_main()')
    body = 'void frag_main()' + re.split(r'\n\w+ main\(', rest)[0] + '\n'
    statics = [l for l in head.splitlines() if l.startswith('static ') and 'gl_FragCoord' not in l
               and 'out_var_SV_TARGET' not in l]
    helpers = []
    for name in ('spvPackHalf2x16', 'spvUnpackHalf2x16'):
        m = re.search(r'^\w+ %s\(.*?^}\n' % name, head, re.S | re.M)
        if m and name in body:
            helpers.append(m.group(0))

    def rn(s):
        s = re.sub(r'\bframe_constants_view_constants\b', 'view_constants', s)
        s = re.sub(r'\bframe_constants_(\w+)', r'\1', s)
        s = re.sub(r'\b_Globals_(\w+)', r'\1', s)
        s = re.sub(r'\bcolor_library_(\w+)', r'color_library.\1', s)
        s = s.replace('spatial_resolve_offsets_spatial_resolve_offsets_array', 'spatial_resolve_offsets_array')
        s = re.sub(r'\._payload\b', '.payload', s)
        s = s.replace('gl_FragCoord', 'input.position')
        return s

    body = rn(body)
    body = body.replace('void frag_main()', 'float4 ps_main(VsToPs input) : SV_Target0', 1)
    body = body.replace('{\n', '{\n    float4 out_var_SV_TARGET;\n', 1)
    body = body.rstrip()
    assert body.endswith('}')
    body = body[:-1] + '    return out_var_SV_TARGET;\n}\n'
    return [rn(s) for s in statics], helpers, body


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('hlsl')
    ap.add_argument('--full', required=True)
    ap.add_argument('--reduced')
    ap.add_argument('--marker', default=r'^(// ReSTIR reservoir spatial gather|uint gather_reservoir_lanterns)')
    a = ap.parse_args()
    p = Path(a.hlsl)
    lines = p.read_text().splitlines(keepends=True)
    for i, l in enumerate(lines):
        if re.match(a.marker, l):
            cut = i; break
    else:
        raise SystemExit('marker not found')
    out = ''.join(lines[:cut])

    def block(bin_path, tag):
        st, hp, body = adapt(lift_ps(bin_path))
        return ('// %s: pixel stage lifted from %s (spirv-cross, mechanically renamed;\n'
                '// _NNN names are SPIR-V ids). Behaviour-exact; readability pass pending.\n'
                % (tag, Path(bin_path).name)) + '#pragma dxc diagnostic push\n#pragma dxc diagnostic ignored "-Wconversion"\n' + '\n'.join(st) + '\n' + ''.join(hp) + body + '#pragma dxc diagnostic pop\n'

    out += '#if DEFERRED_FULL_LIGHTING\n' + block(a.full, 'b903') + '#else\n'
    if a.reduced:
        out += block(a.reduced, 'f0ad').replace('float4 ps_main(VsToPs input) : SV_Target0', 'void ps_main(VsToPs input)') \
            .replace('    float4 out_var_SV_TARGET;\n', '').replace('    return out_var_SV_TARGET;\n', '')
    out += '#endif\n'
    p.write_text(out)
    print('rewrote', p)


if __name__ == '__main__':
    main()
