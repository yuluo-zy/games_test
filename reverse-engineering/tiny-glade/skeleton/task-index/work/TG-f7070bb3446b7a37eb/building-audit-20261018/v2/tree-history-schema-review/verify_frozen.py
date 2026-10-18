from pathlib import Path
import json,hashlib,re
import pefile
BASE=Path('/Users/yuluo/Desktop/game')
WORK=BASE/'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work'
OUT=WORK/'TG-f7070bb3446b7a37eb/building-audit-20261018/v2/tree-history-schema-review'
schema=WORK/'TG-ab197f381ed7615ce6/core-wall-tree-adapter-20261018'
hide=WORK/'TG-fba4fef341225b6a29/core-wall-tree-adapter-20261018'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
result={'native_executed':False,'cargo_executed':False,'production_changed':False,'manifests':[],'packs':[],'keys':[],'bases':[]}
for p in [schema/'frozen-history-schema.json',hide/'frozen-inputs-v2.json',hide/'frozen-inputs-v3.json']:
 d=json.loads(p.read_text());rows=[]
 for item in d['inputs']:
  q=Path(item['path']);actual=sha(q);rows.append({'path':str(q),'sha256':actual,'matches':actual==item['sha256'],'size_matches':item.get('size',q.stat().st_size)==q.stat().st_size})
 assert all(r['matches'] and r['size_matches'] for r in rows)
 result['manifests'].append({'path':str(p),'sha256':sha(p),'count':len(rows),'all_match':True})
pe=pefile.PE(str(BASE/'ljxsj_92385/Tiny Glade/tiny-glade.rne'),fast_load=True)
result['binary_sha256']=sha(BASE/'ljxsj_92385/Tiny Glade/tiny-glade.rne')
assert result['binary_sha256']=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
for p in sorted((schema/'history-review/native-pack').glob('tree-*.json')):
 d=json.loads(p.read_text());items=d['instructions'];n=0
 for i in items:
  raw=bytes.fromhex(i['bytes']);rva=int(i['rva'],0)
  assert pe.get_data(rva,len(raw))==raw,(p,i)
  n+=1
 result['packs'].append({'path':str(p),'instructions_verified':n,'byte_count':sum(len(bytes.fromhex(i['bytes'])) for i in items)})
for k in json.loads((schema/'history-review/serializer-key-bindings.json').read_text()):
 va=int(k['instruction_va'],0);raw=bytes.fromhex(k['instruction_bytes'])
 assert pe.get_data(va-0x140000000,len(raw))==raw
 target=va+len(raw)+int.from_bytes(raw[-4:],'little',signed=True)
 assert target==int(k['target_va'],0)
 prefix=k['text_prefix'].encode();assert pe.get_data(target-0x140000000,len(prefix))==prefix
 result['keys'].append({'instruction_va':k['instruction_va'],'target_va':k['target_va'],'matches':True})
for group in ['TG-ab197f381ed7615ce6','TG-14a1d4b6fb871ce8de','TG-347c9d16cc440d611d','TG-3add2880f5e9bb64c3','TG-6da6686f3f5f072d5c','TG-ad24c4f393c2282e68']:
 p=WORK/group/'core-wall-tree-adapter-20261018/source-base.json'
 if p.parent.name!='core-wall-tree-adapter-20261018':continue
 d=json.loads(p.read_text());src=Path(d['path'])
 result['bases'].append({'source':str(src),'base_sha256':d['sha256'],'current_sha256':sha(src),'matches':sha(src)==d['sha256']})
# Failure branch text and symbol-independent message binding in UIdelete.
result['ui_delete_unwrap_message']=pe.get_data(0x142ef5e98-0x140000000,43).decode()
(OUT/'verification.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k in ['manifests','packs','binary_sha256','ui_delete_unwrap_message','bases']},indent=2))
