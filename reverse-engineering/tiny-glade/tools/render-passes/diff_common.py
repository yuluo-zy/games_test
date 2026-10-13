"""Shared by the diff_exec*.py drivers: run spv-diff for all profiles; if the process dies (lavapipe
crash on an extreme input), re-run each profile in its own process so the other profiles still count."""
import os, subprocess
from pathlib import Path


def run_profiles(prefix, profiles, env, timeout=180):
    """prefix: argv up to (excluding) the profile list. Returns (stdout_lines, crashed_profiles, error)."""
    try:
        r = subprocess.run(prefix + [str(p) for p in profiles], capture_output=True, text=True, timeout=timeout, env=env)
        lines = r.stdout.splitlines()
        if 'DONE' in lines or (lines and lines[0].split()[0] in ('SKIP', 'IFACE_MISMATCH')):
            return lines, [], None
    except subprocess.TimeoutExpired:
        pass  # fall through: isolate the profile(s) that hang
    out, crashed = [], []
    for p in profiles:
        try:
            q = subprocess.run(prefix + [str(p)], capture_output=True, text=True, timeout=timeout, env=env)
        except subprocess.TimeoutExpired:
            crashed.append(p); continue
        ls = q.stdout.splitlines()
        if 'DONE' in ls:
            out += [l for l in ls if l.startswith('RES')]
        elif ls and ls[0].split()[0] in ('SKIP', 'IFACE_MISMATCH'):
            return ls, [], None
        else:
            crashed.append(p)
    return out + ['DONE'], crashed, None


# Profiles whose data is "well formed" for typical float/int shader code. A mismatch there is a real DIFF;
# a mismatch only in the remaining (zero / denormal / overflow / ints-read-as-floats) profiles is EDGE:
# usually undefined behaviour (div by zero, NaN->int, float->int overflow) whose result is compiler specific.
CORE_PROFILES = {0, 2, 3, 4, 8, 12} | set(range(16, 32))  # 16..31: coverage-search mixes of small ints + unit floats (gen_mixed in spv-diff); 32+ also contain zeros/huge words -> EDGE


# --- FMA / strict-arithmetic recheck -------------------------------------------------------------------
# A DIFF/EDGE is "explained by FMA" only when this recheck holds (no hard-coded module list):
#   candidate' = dxc -Gis (strict IEEE: no reassociation, `precise` everywhere), SPIR-V float_controls modes stripped,
#   NoContraction injected on every float op (spv_nocontract.py);   oracle' = oracle + NoContraction.
# oracle' vs candidate' bit-identical on every profile  =>  same algorithm; the default-run difference is only
# lavapipe fusing mul+add differently in the two instruction orders.
# Control: oracle vs oracle' differing on the same inputs shows that contraction by itself changes the result.
FMA_RECHECK = True


def strict_variant(cmd, out_path, strip_dgq=False):
    """Recompile `cmd` (a dxc command line ending in -Fo <file> ...) strictly; returns the NoContraction-injected module path or None."""
    import spv_nocontract, spv_strip_dgq, struct
    cmd2 = [x.replace('vulkan1.1', 'vulkan1.2') for x in cmd]
    cmd2[cmd2.index('-Fo') + 1] = str(out_path)
    cmd2.insert(1, '-Gis')
    if subprocess.run(cmd2, capture_output=True, text=True).returncode or not Path(out_path).exists():
        return None
    data = Path(out_path).read_bytes()
    if strip_dgq:
        data = spv_strip_dgq.strip(data)[0]
    w = list(struct.unpack('<%dI' % (len(data) // 4), data))
    out, i = w[:5], 5
    while i < len(w):  # -Gis adds SignedZeroInfNanPreserve & co. (float_controls); the arithmetic order is already baked in
        wc, op = w[i] >> 16, w[i] & 0xffff
        if not ((op == 16 and w[i + 2] in (4459, 4460, 4461, 4462, 4463)) or (op == 17 and w[i + 1] in (4466, 4467, 4468, 4469, 4470))):
            out += w[i:i + wc]
        i += wc
    # fuse variant: same strict instruction order, but no NoContraction -> the driver may contract exactly like it does for the oracle
    fo, i = out[:5], 5
    while i < len(out):
        wc, op = out[i] >> 16, out[i] & 0xffff
        if not (op == 71 and out[i + 2] == 42):
            fo += out[i:i + wc]
        i += wc
    Path(str(out_path) + '.fuse.spv').write_bytes(struct.pack('<%dI' % len(fo), *fo))
    Path(out_path).write_bytes(spv_nocontract.inject(struct.pack('<%dI' % len(out), *out))[0])
    return Path(out_path)


def fma_recheck(a_path, c_strict, run, profiles):
    """run(x, y, profiles) -> list of RES row dicts, or None when the run failed. Returns the evidence dict."""
    import spv_nocontract
    a_nc = Path(str(a_path) + '.nc.spv')
    a_nc.write_bytes(spv_nocontract.inject(Path(a_path).read_bytes())[0])
    strict = run(a_nc, c_strict, profiles)
    control = run(a_path, a_nc, profiles)
    fused = run(a_path, Path(str(c_strict) + '.fuse.spv'), profiles)
    words = lambda rows: None if rows is None else sum(int(r['mismatch']) for r in rows)
    return dict(fma_proven=(words(strict) == 0) if strict is not None else None, fma_strict_diff=words(strict), fma_control_diff=words(control),
                fma_strict_fused_diff=words(fused))  # 0 here = candidate in the original (non-reassociated) order, contracted by the driver, equals the oracle bit for bit


def apply_fma_verdict(verdict, ev):
    """DIFF/EDGE + recheck evidence -> final verdict: proven => EDGE(fma_proven); otherwise a (still) unexplained difference is DIFF."""
    if verdict not in ('DIFF', 'EDGE') or ev is None:
        return verdict
    return 'EDGE' if ev.get('fma_proven') else 'DIFF'


def classify(diff_rows):
    """diff_rows: list of dicts with a 'profile' key. Returns 'DIFF' | 'EDGE' | None."""
    if not diff_rows:
        return None
    return 'DIFF' if any(int(r['profile']) in CORE_PROFILES for r in diff_rows) else 'EDGE'


def with_ms(env):
    """Add SPVDIFF_MS_* (fill shaders for multisampled sampled-image inputs), compiling tools/spv-diff/msfill.hlsl once."""
    out = Path('/tmp/spvdiff_ms')
    src = Path(__file__).resolve().parents[2] / 'tools/spv-diff/msfill.hlsl'
    out.mkdir(exist_ok=True)
    dxc = os.environ.get('DXC', 'dxc')
    for name, entry, prof in (('vs.spv', 'vs_main', 'vs_6_4'), ('fs_f.spv', 'fs_f', 'ps_6_4'), ('fs_u.spv', 'fs_u', 'ps_6_4')):
        f = out / name
        if not f.exists() or f.stat().st_mtime < src.stat().st_mtime:
            subprocess.run([dxc, '-E', entry, '-T', prof, '-spirv', '-fspv-target-env=vulkan1.1', '-HV', '2021', str(src), '-Fo', str(f)], capture_output=True)
    return dict(env, SPVDIFF_MS_VS=str(out / 'vs.spv'), SPVDIFF_MS_FS_F=str(out / 'fs_f.spv'), SPVDIFF_MS_FS_U=str(out / 'fs_u.spv'))
