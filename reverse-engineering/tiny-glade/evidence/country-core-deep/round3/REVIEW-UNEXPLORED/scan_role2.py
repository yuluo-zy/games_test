import re, glob, io, sys

out = io.open('round3/REVIEW-UNEXPLORED/_role_scan.txt', 'w', encoding='utf-8')
n_guess = 0
n_noevid = 0
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
            concl = cells[4] if len(cells) > 4 else ''
            if not re.search(r'\*\*(O|I|U)\*\*', concl):
                continue
            # guess role: any cell contains 猜测 but not 证实 (unconfirmed guess)
            role_cells = [c for c in cells if '猜测' in c]
            for rc in role_cells:
                if '证实' not in rc:
                    n_guess += 1
                    out.write('GUESS-ONLY %s:%d %s | %s\n' % (p, i, cells[0][:70], rc[:120]))
            evid = cells[5] if len(cells) > 5 else ''
            if evid.strip() in ('', '—', '-'):
                n_noevid += 1
                out.write('NO-EVID %s:%d %s\n' % (p, i, cells[0][:80]))
out.write('TOTAL guess-only: %d, no-evidence: %d\n' % (n_guess, n_noevid))
out.close()
print('done, guess-only:', n_guess, 'no-evidence:', n_noevid)
