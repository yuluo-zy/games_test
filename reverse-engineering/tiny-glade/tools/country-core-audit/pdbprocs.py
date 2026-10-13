import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""独立解析 MSF7 PDB，导出过程表 procs.tsv：rva(绝对VA) \t 大小 \t 模块序号 \t obj \t 全名。
不依赖 Ghidra / llvm-pdbutil，纯 Python（85k 过程约 3 秒）。
用法: python3 -I pdbprocs.py [pdb] [exe] [out.tsv]"""
import mmap, struct, sys
from cca import PDB, EXE, PROCS_TSV, BASE, WORK

pdb_path = sys.argv[1] if len(sys.argv) > 1 else PDB
exe_path = sys.argv[2] if len(sys.argv) > 2 else EXE
out = sys.argv[3] if len(sys.argv) > 3 else PROCS_TSV
WORK.mkdir(parents=True, exist_ok=True)

f = open(pdb_path, 'rb'); m = mmap.mmap(f.fileno(), 0, access=mmap.ACCESS_READ)
assert m[:24] == b'Microsoft C/C++ MSF 7.00'
bs, _, nb, ndir, _, bmap = struct.unpack_from('<6I', m, 32)
dirblocks = struct.unpack_from(f'<{(ndir + bs - 1) // bs}I', m, bmap * bs)
d = b''.join(m[b * bs:(b + 1) * bs] for b in dirblocks)[:ndir]
ns = struct.unpack_from('<I', d, 0)[0]
sizes = struct.unpack_from(f'<{ns}I', d, 4); pos = 4 + 4 * ns; blocks = []
for s in sizes:
    s = 0 if s == 0xffffffff else s
    n = (s + bs - 1) // bs; blocks.append(struct.unpack_from(f'<{n}I', d, pos)); pos += 4 * n


def stream(i):
    return b''.join(m[b * bs:(b + 1) * bs] for b in blocks[i])[:sizes[i]]


e = open(exe_path, 'rb').read()
pe = struct.unpack_from('<I', e, 0x3c)[0]
nsec = struct.unpack_from('<H', e, pe + 6)[0]; opt = struct.unpack_from('<H', e, pe + 20)[0]
secva = [struct.unpack_from('<I', e, pe + 24 + opt + 40 * i + 12)[0] for i in range(nsec)]

dbi = stream(3)
modsz = struct.unpack_from('<i', dbi, 24)[0]
p = 64; mods = []
while p < 64 + modsz:
    symstream, symbytes = struct.unpack_from('<HI', dbi, p + 34)
    q = p + 64; n1 = dbi.index(b'\0', q); n2 = dbi.index(b'\0', n1 + 1)
    mods.append((dbi[n1 + 1:n2].decode('utf8', 'replace'), symstream, symbytes)); p = (n2 + 1 + 3) & ~3

cnt = 0
with open(out, 'w', encoding='utf8') as o:
    for mi, (obj, ss, sb) in enumerate(mods):
        if ss == 0xffff or sb == 0:
            continue
        s = stream(ss); q = 4
        while q < sb:
            ln, kind = struct.unpack_from('<HH', s, q)
            if kind in (0x1110, 0x110F, 0x1147, 0x1146):          # S_GPROC32 / S_LPROC32 (+_ID)
                codesize = struct.unpack_from('<I', s, q + 16)[0]
                off, seg = struct.unpack_from('<IH', s, q + 32)
                nm = s[q + 39:s.index(b'\0', q + 39)].decode('utf8', 'replace')
                if 1 <= seg <= nsec:
                    o.write(f"{BASE + secva[seg - 1] + off:#x}\t{codesize}\t{mi}\t{obj}\t{nm}\n"); cnt += 1
            q += ln + 2
print(cnt, 'procs from', len(mods), 'modules ->', out)
