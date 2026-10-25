from pathlib import Path
import json,gzip,hashlib
ROOT=Path('/Users/yuluo/Desktop/game');OUT=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f7070bb3446b7a37eb/building-audit-20261018/v2/history-lifecycle-review/insta-producers'
selected={}
for rel in ['crates/systems/decorator/src/ui_systems/ui_focus_stairs.rs','crates/systems/decorator/src/ui_systems/ui_place_decorator.rs','crates/systems/color/src/color_menu.rs']:
 p=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/B'/rel
 meta=p.with_name(p.name+'.enriched.json.gz');d=json.loads(gzip.decompress(meta.read_bytes()))
 names=['InstaCreate' ] if 'color_menu'not in rel else ['FinishInputRecordingCmd']
 selected[rel]={'source':str(meta),'sha256':hashlib.sha256(meta.read_bytes()).hexdigest(),'binding':d['binding'],'functions':[{k:x.get(k)for k in ['name','compiled_instances']}for x in d['functions']if x.get('compiled_instances')],'interfaces':[{k:x.get(k)for k in['id','function_type_spelling','candidate_binding','binding_status','evidence']}for x in d['file_interface_candidates']if any(n in x.get('function_type_spelling','')for n in names)]}
(OUT/'insta-consumer-producer-interfaces.json').write_text(json.dumps(selected,indent=2)+'\n')
# The original assembly lines are byte-checked against the bound image again.
import re,pefile
pe=pefile.PE(str(ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True);checked={}
for p in OUT.glob('*.asm'):
 count=0
 for l in p.read_text().splitlines():
  m=re.match(r'^([0-9a-f]+): ([0-9a-f]+)',l)
  if m:
   va=int(m[1],16);raw=bytes.fromhex(m[2]);assert pe.get_data(va-0x140000000,len(raw))==raw;(count:=count+1)
 checked[p.name]=count
(OUT/'original-byte-verification.json').write_text(json.dumps({'original_sha256':hashlib.sha256((ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()).hexdigest(),'instructions':checked},indent=2)+'\n')
