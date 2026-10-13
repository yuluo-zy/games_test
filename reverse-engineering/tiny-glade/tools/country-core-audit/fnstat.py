import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""country-core Rust 源码的函数体分类：stub(todo!/unimplemented!)、trivial(<=2行)、partial、real。
用法: python3 -I fnstat.py [src目录]"""
import collections, os, re, sys
from cca import CC_SRC

root = sys.argv[1] if len(sys.argv) > 1 else str(CC_SRC)
fnre = re.compile(r'\bfn\s+(\w+)[^;{]*\{')
stats = collections.Counter(); per = collections.defaultdict(collections.Counter)
for dp, _, fs in os.walk(root):
    for f in fs:
        if not f.endswith('.rs'):
            continue
        p = os.path.join(dp, f); s = open(p, encoding='utf8').read()
        key = '/'.join(os.path.relpath(p, root).split('/')[:2])
        for m in fnre.finditer(s):
            i = m.end(); d = 1; j = i
            while j < len(s) and d:
                d += (s[j] == '{') - (s[j] == '}'); j += 1
            code = re.sub(r'/\*.*?\*/', '', re.sub(r'//.*', '', s[i:j - 1]), flags=re.S).strip()
            n = len([l for l in code.splitlines() if l.strip()])
            hasstub = bool(re.search(r'todo!|unimplemented!', code))
            k = 'stub' if hasstub and n <= 3 else 'trivial' if n <= 2 else 'partial' if hasstub else 'real'
            stats[k] += 1; per[key][k] += 1
print(dict(stats))
for k, v in sorted(per.items(), key=lambda x: -sum(x[1].values())):
    print(f'{k:55s} {dict(v)}')
