import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""反汇编单个 PDB 过程，标注 call 目标(PDB 名)、RIP 相对的 f32/f64/vec4 常量和 lea 字符串。
用法: python3 -I dis.py <0x地址 | 函数名后缀/子串>     名字不唯一时列出候选。"""
import bisect, re, struct, subprocess, sys
from cca import EXE, BASE, load_procs, objdump

procs = load_procs(); starts = sorted(procs)
e = open(EXE, 'rb').read()
pe = struct.unpack_from('<I', e, 0x3c)[0]; nsec = struct.unpack_from('<H', e, pe + 6)[0]; opt = struct.unpack_from('<H', e, pe + 20)[0]
secs = [struct.unpack_from('<IIII', e, pe + 24 + opt + 40 * i + 8) for i in range(nsec)]   # vsize,va,rawsize,rawptr


def rd(va, n):
    r = va - BASE
    for vs, v, rs, rp in secs:
        if v <= r < v + max(vs, rs):
            return e[rp + r - v:rp + r - v + n]


def name(a):
    i = bisect.bisect_right(starts, a) - 1
    if i >= 0 and a < starts[i] + procs[starts[i]][0] + 1:
        s = starts[i]; return procs[s][1] + (f'+{a - s:#x}' if a != s else '')


arg = sys.argv[1]
if arg.startswith('0x'):
    a = int(arg, 16)
else:
    c = [s for s in starts if procs[s][1].endswith(arg) or arg == procs[s][1]] or [s for s in starts if arg in procs[s][1]]
    if len(c) != 1:
        for s in c[:30]:
            print(hex(s), procs[s][0], procs[s][1][:150])
        sys.exit(0 if c else 1)
    a = c[0]
sz, nm = procs[a]; print(f'# {a:#x} size={sz} {nm}')
out = subprocess.run([objdump(), '-d', '--no-show-raw-insn', '-M', 'intel', f'--start-address={a}', f'--stop-address={a + sz}', str(EXE)],
                     capture_output=True, text=True).stdout
for line in out.splitlines():
    m = re.match(r'\s*([0-9a-f]+):\s*(.*)', line)
    if not m:
        continue
    ins = m.group(2); note = ''
    t = re.search(r'(?:call|jmp)\s+0x([0-9a-f]+)', ins)
    if t:
        n = name(int(t.group(1), 16)); note = n[:420] if n else ''
    if not note:
        c = re.search(r'# 0x([0-9a-f]+)', ins)
        if c and re.search(r'\[rip', ins):
            va = int(c.group(1), 16); b = rd(va, 16)
            if b:
                op = ins.split()[0]
                if op.endswith('ss') or 'dword' in ins: note = f'f32={struct.unpack("<f", b[:4])[0]!r} ({b[:4][::-1].hex()})'
                elif op.endswith('sd') or 'qword' in ins: note = f'f64={struct.unpack("<d", b[:8])[0]!r}'
                elif 'xmm' in ins: note = 'v4=' + str([round(x, 6) for x in struct.unpack('<4f', b)])
                elif op == 'lea':
                    n = name(va); note = n[:420] if n else 'str=' + repr(rd(va, 48).split(b'\0')[0][:40])
    print(f'{m.group(1)}: {ins[:90]:90s} {note}')
