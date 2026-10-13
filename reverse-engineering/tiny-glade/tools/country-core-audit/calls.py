import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""对渲染相关范围内的每个 PDB 过程反汇编，抽取直接 call/jmp 目标(解析到所属过程)与间接调用点数。
范围: country_core 的 render:: / asset_libraries:: / systems::render_loop / resources::render:: + tiny_glade::render_loop
输出 data/scope_calls.json。用法: python3 -I calls.py"""
import json, re, subprocess
from concurrent.futures import ThreadPoolExecutor
from cca import EXE, CALLS_JSON, WORK, load_procs, Owner, objdump

procs = load_procs(); own = Owner(procs); OBJ = objdump()
SCOPE = re.compile(r'^country_core::(render::|asset_libraries::|systems::render_loop|resources::render::)|^tiny_glade::render_loop')
S = [a for a, (_, nm) in procs.items() if SCOPE.search(nm)]


def calls(a):
    sz = procs[a][0]
    out = subprocess.run([OBJ, '-d', '--no-show-raw-insn', '-M', 'intel', f'--start-address={a}', f'--stop-address={a + sz}', str(EXE)],
                         capture_output=True, text=True).stdout
    d = set(); ind = 0
    for line in out.splitlines():
        m = re.search(r'\t(call|jmp)\s+(.*)', line)
        if not m:
            continue
        t = re.match(r'0x([0-9a-f]+)', m.group(2))
        if t:
            o = own(int(t.group(1), 16))
            if o is not None and o != a: d.add(o)
        elif m.group(1) == 'call':
            ind += 1
    return a, sorted(d), ind


with ThreadPoolExecutor(8) as ex:
    res = list(ex.map(calls, S))
WORK.mkdir(parents=True, exist_ok=True)
json.dump({hex(a): {'callees': [hex(x) for x in d], 'indirect': i} for a, d, i in res}, open(CALLS_JSON, 'w'))
print(len(S), 'procs; edges', sum(len(d) for _, d, _ in res), '; indirect call sites', sum(i for *_, i in res))
