import pathlib,json,hashlib
p=pathlib.Path('reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/graph');m=json.loads((p/'candidate-sha256.json').read_text());rows=[]
for rel,sha in m.items():
 actual=hashlib.sha256((p/'candidate'/rel).read_bytes()).hexdigest();rows.append({'path':rel,'expected':sha,'actual':actual,'match':sha==actual})
r={'patch_sha256':hashlib.sha256((p/'graph-stage2.patch').read_bytes()).hexdigest(),'files':rows,'all_match':all(q['match'] for q in rows)};(p.parent/'review/stage2-identity.json').write_text(json.dumps(r,indent=2));print(json.dumps({'patch_sha256':r['patch_sha256'],'all_match':r['all_match'],'files':len(rows)},indent=2))
