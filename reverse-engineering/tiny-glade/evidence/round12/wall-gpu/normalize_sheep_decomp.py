from pathlib import Path
p=Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/sheep-startup-query.c');s=p.read_text(encoding='utf-8');s=s.replace('\n\n','\n');p.write_text(s,encoding='utf-8')
