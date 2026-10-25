import sys,re,json
from pathlib import Path
sys.path.insert(0,'D:/game/reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14')
from pdb_reader import Pdb,z
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb');out=set()
for mi,mod,ss,sb in p.modules():
 if ss==65535 or not sb:continue
 d=p.stream(ss)[:sb]
 for m in re.finditer(rb'(?:country_core::resources::session|country_core::startup::startup_session|toolbar::menu::new_session)::[A-Za-z_][A-Za-z_0-9]*',d):out.add(m[0].decode())
print('\n'.join(sorted(out)));Path(__file__).with_name('session-types.txt').write_text('\n'.join(sorted(out)))
