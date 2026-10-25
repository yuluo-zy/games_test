from pathlib import Path
p=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/wall-gpu/probe_corner_owner_research.py');s=p.read_text(encoding='utf-8');s=s.replace('options=buf(bytes(options));output=vec_header();holes=vec_header()','options=buf(bytes(options));output=vec_header();holes=buf(struct.pack(\'<3Q\',0,4,0)+bytes(0x68-24))');p.write_text(s,encoding='utf-8')
