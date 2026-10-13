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


# Modules whose only remaining difference was proven to be driver FMA contraction: with NoContraction
# injected into the oracle and `precise` in the candidate the outputs are bit-identical (see
# RECOVERY_TRACKER.md "8 个差异调查结果"). They are reported as EDGE, never SAME.
PROVEN_FMA = {'water/ice_refl_resolve.cs.hlsl'}


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
