from pathlib import Path
import json,hashlib
R=Path('D:/game/ljxsj_92385/Tiny Glade/assets/starting-builds');O=Path(__file__).resolve().parent
counts={'files':0,'entries':0,'decorator_aux':0,'addresses':0,'nonempty_sides':0,'state_items':0}
shape_keys={};fields={};fixture=None;records=[]
for p in sorted(R.glob('*/*/history.json')):
 data=p.read_bytes();asset=json.loads(data);counts['files']+=1
 chain=asset['History']['edit_chain'];counts['entries']+=len(chain)
 def walk(value,trail):
  global fixture
  if isinstance(value,dict):
   for key,v in value.items():
    if key=='decorators' and isinstance(v,dict) and isinstance(v.get('diffs'),list):
     counts['decorator_aux']+=1
     for i,(address,diff) in enumerate(v['diffs']):
      counts['addresses']+=1
      k=','.join(sorted(diff));shape_keys[k]=shape_keys.get(k,0)+1
      assert set(diff)=={'before','after'},(p,trail,diff.keys())
      for side in ['before','after']:
       state=diff[side];assert isinstance(state,dict)
       counts['nonempty_sides']+=bool(state);counts['state_items']+=len(state)
       for id,info in state.items():
        assert id.isdigit(),(p,id)
        assert isinstance(info,dict),(p,info)
        for field in info:fields[field]=fields.get(field,0)+1
      if fixture is None and asset.get('Version')==27 and diff['before'] and diff['after']:
       fixture={'source':str(p),'source_sha256':hashlib.sha256(data).hexdigest(),'trail':trail+[key,'diffs',i],
        'address':address,'diff':diff,'version':asset.get('Version')}
    walk(v,trail+[key])
  elif isinstance(value,list):
   for i,v in enumerate(value):walk(v,trail+[i])
 walk(chain,['History','edit_chain'])
 records.append({'path':str(p),'sha256':hashlib.sha256(data).hexdigest(),'version':asset.get('Version'),'entries':len(chain)})
(O/'corpus-schema.json').write_text(json.dumps({'counts':counts,'diff_keys':shape_keys,'state_item_fields':fields,'files':records},indent=2))
assert fixture is not None
(O/'original-nonempty-address-state.json').write_text(json.dumps(fixture,indent=2))
print(json.dumps(counts));print(json.dumps(shape_keys));print(json.dumps(fixture)[:2200])
