import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""核对文档/代码里引用的 0x14xxxxxxx 地址是否落在真实 PDB 函数上，名字是否对得上。
输出各工作包的 起始地址/内部地址/无函数 统计，明细写入 data/addr_issues.json。
用法: python3 -I addrcheck.py"""
import collections, json, os, re
from cca import CC_SRC, CC_WP, WORK, ADDR_RE, load_procs, Owner

procs = load_procs(); own = Owner(procs)
ident = re.compile(r'`?([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_$<>]*)*)`?')
short = lambda nm: re.sub(r'<.*>', '', nm).split('::')[-1]
stat = collections.Counter(); per = collections.defaultdict(collections.Counter); bad = []; seen = set()
for root in (CC_WP, CC_SRC):
    for dp, _, fs in os.walk(root):
        if '/target' in dp:
            continue
        for f in fs:
            if not re.search(r'\.(rs|md|c|txt)$', f):
                continue
            p = os.path.join(dp, f)
            wp = (re.search(r'(wp\d+|country-core/src)', p) or [None, '?'])[1]
            for ln, line in enumerate(open(p, errors='replace'), 1):
                for mt in ADDR_RE.finditer(line):
                    a = int(mt.group(1), 16)
                    if (p, ln, a) in seen:
                        continue
                    seen.add((p, ln, a))
                    o = own(a); kind = 'none' if o is None else ('start' if o == a else 'inside')
                    stat[kind] += 1; per[wp][kind] += 1
                    if kind == 'start':
                        ctx = line if not f.endswith('.c') else f
                        ids = {short(x) for x in ident.findall(ctx) if len(x) > 2}
                        fn = short(procs[a][1])
                        if any(fn in x for x in ids): per[wp]['name_match'] += 1
                        else:
                            per[wp]['name_other'] += 1
                            bad.append((wp, p.split('reconstruction/')[-1], ln, hex(a), procs[a][1][:110], line.strip()[:140]))
                    else:
                        bad.append((wp, p.split('reconstruction/')[-1], ln, hex(a), kind + ':' + (procs[o][1][:100] if o else '-'), line.strip()[:140]))
print(dict(stat))
for k in sorted(per): print(k, dict(per[k]))
WORK.mkdir(parents=True, exist_ok=True)
json.dump(bad, open(WORK / 'addr_issues.json', 'w'), ensure_ascii=False, indent=0)
