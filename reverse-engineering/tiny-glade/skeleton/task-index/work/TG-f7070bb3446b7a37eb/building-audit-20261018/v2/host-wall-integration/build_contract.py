from pathlib import Path
import json,hashlib,gzip,struct
import pefile
P=Path(__file__).resolve().parent;ROOT=Path('/Users/yuluo/Desktop/game');A=json.load(gzip.open(ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz','rt'))
pe=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True)
def first_arg(s):
 s=s.split('<',1)[1];dep=0
 for i,c in enumerate(s):
  if c in '<(':dep+=1
  elif c in '>)':dep-=1
  elif c==',' and dep==0:return s[:i]
def split_tuple(s):
 b=s[len('tuple$<'):-1];dep=0;pos=0;out=[]
 for i,c in enumerate(b):
  if c in '<(':dep+=1
  elif c in '>)':dep-=1
  elif c==',' and dep==0:out.append(b[pos:i]);pos=i+1
 out.append(b[pos:]);return out
T={}
for fn in ['14050d290','140335b30','1400d5080','1400cbd40','141737fb0','141738630']:
 rec=json.loads((P/(fn+'.pdb.json')).read_text());arg=first_arg(rec['name']);T[fn]={'method':rec['name'].split('<',1)[0],'tuple':split_tuple(arg) if arg.startswith('tuple$<') else [arg]}
(P/'typed-tuples.json').write_text(json.dumps(T,indent=2))
V=[]
for va in [0x14292af98,0x142c5fd60]:
 raw=pe.get_data(va-0x140000000,72);qs=struct.unpack('<9Q',raw)
 V.append({'va':hex(va),'bytes_hex':raw.hex(),'size':qs[1],'alignment':qs[2],'system_type_pointer':hex(qs[4]),'system_type_aliases':A[str(qs[4]-0x140000000)],'proof':'vtable method system_type is instantiated for country_core::systems::GameSystemSet'})
(P/'set-vtable-identity.json').write_text(json.dumps(V,indent=2))
progress=ROOT/'games_test/tiny-glade/reports/core-systems-resources-recovery-2026-10-18/module-progress.json'
(P/'module-progress-review.json').write_text(json.dumps({'input':str(progress),'sha256':hashlib.sha256(progress.read_bytes()).hexdigest(),'rows':len(json.loads(progress.read_text())['rows']),'blocking_findings':[],'boundary':'16 rows label partial regressions, not complete modules. Actual host and full GPU remain explicit gaps. Optional clarify full draw-entry native validation incomplete and typed map uses mock device.'},indent=2))
