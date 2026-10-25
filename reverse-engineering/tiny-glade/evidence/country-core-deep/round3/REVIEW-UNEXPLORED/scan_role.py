import re, glob

# Find rows where 作用 column still has 猜测/待证实 (unconfirmed) or 证据 column empty
for pat in ['chapters/asset_libraries/*.rs.md','chapters/utils/*.rs.md','chapters/startup/*.rs.md',
            'chapters/startup/glade_themes/*.rs.md','chapters/input/*.rs.md','chapters/components/*.rs.md',
            'chapters/geometry/*.rs.md']:
    for p in sorted(glob.glob(pat)):
        text = open(p, encoding='utf-8').read()
        for i, line in enumerate(text.splitlines(), 1):
            line = line.strip()
            if not line.startswith('|'):
                continue
            cells = [c.strip() for c in line.strip('|').split('|')]
            if len(cells) < 6:
                continue
            # 作用 col index 3, 结论 col 4, 证据 col 5 (header: | 函数 | 地址 | 源码行 | 作用 | 逆向结论 | 证据 |)
            role = cells[3] if len(cells) > 3 else ''
            concl = cells[4] if len(cells) > 4 else ''
            evid = cells[5] if len(cells) > 5 else ''
            if not re.search(r'\*\*(O|I|U)\*\*', concl):
                continue
            if '猜测' in role or '待证实' in role:
                print('%s:%d GUESS-ROLE: %s | %s' % (p, i, cells[0][:60], role[:80]))
            if evid.strip() in ('', '—', '-'):
                print('%s:%d NO-EVID: %s' % (p, i, cells[0][:80]))
