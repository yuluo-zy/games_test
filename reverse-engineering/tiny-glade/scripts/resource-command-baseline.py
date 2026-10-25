"""Extract JSON command shapes from shipped starting-build fixtures; do not replay."""
from pathlib import Path
import collections, hashlib, json, re
ROOT=Path('D:/game/ljxsj_92385/Tiny Glade')
OUT=Path('D:/game/reverse-engineering/tiny-glade/evidence/resources')
def compact(v,depth=0):
    if depth>=10: return {'type':type(v).__name__}
    if isinstance(v,dict): return {k:compact(x,depth+1) for k,x in v.items()}
    if isinstance(v,list): return {'count':len(v),'first_two':[compact(x,depth+1) for x in v[:2]]}
    return v
tags=collections.Counter(); chainkeys=collections.Counter(); examples={}; fixture_records=[]
for p in sorted((ROOT/'assets/starting-builds').rglob('history.json')):
    v=json.loads(p.read_text()); hist=v.get('History',{}); chain=hist.get('edit_chain',[])
    for i,entry in enumerate(chain):
        chainkeys.update(entry.keys())
        for edit in entry.get('edits',[]):
            if not isinstance(edit,dict): continue
            for tag in edit:
                tags[tag]+=1
                examples.setdefault(tag,{'path':p.relative_to(ROOT).as_posix(),'json_pointer':f'/History/edit_chain/{i}',
                    'command':compact(edit[tag]),'extra_fields':{k:compact(x) for k,x in entry.items() if k!='edits'}})
    fixture_records.append({'path':p.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),
        'keys':list(v),'Version':v.get('Version'),'chain_count':len(chain),'chain_index':hist.get('chain_index'),
        'snapshot_references':hist.get('snapshots')})
ron=ROOT/'assets/nani_meshes.ron'; lines=ron.read_text().splitlines()
ron_records=[{'line':i+1,'text':line.strip()} for i,line in enumerate(lines) if 'subset:' in line]
result={'evidence_id':'TG-RES-COMMANDS-20261010','search_boundary':'All shipped assets/starting-builds/**/history.json; edit tag examples take first occurrence only. RON text scanned for subset lines, not parsed/replayed.',
 'history_count':len(fixture_records),'command_tag_counts':dict(tags),'entry_field_counts':dict(chainkeys),'command_examples':examples,'fixtures':fixture_records,
 'nani_mesh_subsets':{'path':'assets/nani_meshes.ron','sha256':hashlib.sha256(ron.read_bytes()).hexdigest(),'direct_text':ron_records},
 'limitations':'JSON stores historical edits and before/after diff shapes. It does not prove processing order, reconstruction equations, exact runtime behavior, or all interactive commands.'}
(OUT/'command-schemas.json').write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'history_count':len(fixture_records),'command_tag_counts':dict(tags),'entry_field_counts':dict(chainkeys),'subset_records':ron_records},ensure_ascii=False))
