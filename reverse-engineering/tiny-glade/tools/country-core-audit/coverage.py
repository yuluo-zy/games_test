import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""渲染范围的“原函数地址 → 重建源码引用”覆盖统计。

把 layers.py 的 1252 个渲染范围过程按“去泛型名字”归并成逻辑单元（一个泛型函数的任一实例地址
出现在源码里就算该单元被引用），按组件汇总，并列出未被引用的单元。
注意：这只说明“地址被引用”，不证明实现正确；泛型单态化实例不会逐个标注地址，所以必须按单元统计。
需先运行 calls.py（生成 data/scope_calls.json）。用法: python3 -I coverage.py [--list]
产物: /tmp/uncovered_units.json（未引用单元，按体积降序）"""
import collections, glob, json, re

src = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'layers.py')).read()
ns = {'__file__': os.path.abspath('layers.py')}
exec(compile(src[:src.index('def scc_layers')], 'layers_head', 'exec'), ns)  # 只取 comp/norm/procs/calls，不跑分层
procs, comp, calls, norm = ns['procs'], ns['comp'], ns['calls'], ns['norm']

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'reconstruction', 'country-core', 'src')
text = ''.join(open(p, errors='ignore').read().lower() + '\n' for p in glob.glob(ROOT + '/**/*.rs', recursive=True))
cited = {int(m, 16) for m in re.findall(r'0x(14[0-9a-f]{7})', text)}
cited |= {int(m.replace('_', ''), 16) for m in re.findall(r'0x(14[0-9a-f_]{7,12})', text) if len(m.replace('_', '')) == 9}

units = collections.defaultdict(lambda: {'n': 0, 'size': 0, 'hit': False, 'ex': None})
for a in calls:
    name = procs[a][1]
    c = comp(name)
    if c.startswith(('EXT', 'OUT')):
        continue
    u = units[(c, norm(name))]
    u['n'] += 1
    u['size'] = max(u['size'], procs[a][0])
    u['hit'] |= a in cited
    u['ex'] = u['ex'] or hex(a)

by = collections.defaultdict(lambda: [0, 0, 0, 0])
missing = []
for (c, n), u in units.items():
    b = by[c]
    b[0] += 1; b[2] += u['size']
    if u['hit']:
        b[1] += 1; b[3] += u['size']
    else:
        missing.append((u['size'], u['ex'], c, n, u['n']))

print('源码中引用的原地址数:', len(cited))
print('%-50s %5s %5s %7s %7s' % ('组件', '单元', '已引用', 'KiB', '已引用KiB'))
tot = [0] * 4
for c, b in sorted(by.items()):
    print('%-50s %5d %5d %7.0f %7.0f %4.0f%%' % (c, b[0], b[1], b[2] / 1024, b[3] / 1024, 100 * b[3] / max(1, b[2])))
    tot = [x + y for x, y in zip(tot, b)]
print('合计: 单元 %d, 已引用 %d (%.0f%%); 体积 %.0f / %.0f KiB (%.0f%%)' % (
    tot[0], tot[1], 100 * tot[1] / tot[0], tot[3] / 1024, tot[2] / 1024, 100 * tot[3] / tot[2]))
missing.sort(reverse=True)
json.dump(missing, open('/tmp/uncovered_units.json', 'w'), ensure_ascii=False)
print('未引用单元 %d 个' % len(missing))
if '--list' in sys.argv:
    for size, addr, c, n, k in missing:
        print('%6d %s %-16s x%-3d %s' % (size, addr, c[:16], k, n[:120]))
