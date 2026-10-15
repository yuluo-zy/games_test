import re, glob, os, json

# Scan all chapter tables: find rows that look like function rows (start with |, have addr) but lack **O**/**I**/**U**
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
            if len(cells) < 5:
                continue
            has_addr = bool(re.search(r'0x[0-9a-fA-F]{6,}', line))
            if not has_addr:
                continue
            has_marker = bool(re.search(r'\*\*(O|I|U)\*\*', line))
            if not has_marker:
                # likely a row without conclusion
                first = cells[0]
                # skip header/separator/table-like lines with obvious non-func content
                if first in ('函数', '---') or first.startswith('函数'):
                    continue
                print('%s:%d NO-MARKER: %s' % (p, i, line[:150]))
