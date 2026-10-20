from pathlib import Path
p=Path('reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv')
for l in p.open(encoding='utf-8'):
 if l.startswith(('0x1422f86e0\t','0x140f86330\t','0x140f85ee0\t')):print(l.strip())
