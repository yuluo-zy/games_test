from pathlib import Path
import hashlib,json,difflib
root=Path('D:/game');e=root/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu';base=e/'candidates-startup';rows=[];patch=[]
for p in sorted(base.glob('crates/**/*.rs')):
 rel=p.relative_to(base);src=root/'tiny-glade'/rel;a=src.read_text(encoding='utf-8');b=p.read_text(encoding='utf-8');rows.append({'path':rel.as_posix(),'base_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'candidate_sha256':hashlib.sha256(p.read_bytes()).hexdigest()});patch.extend(difflib.unified_diff(a.splitlines(True),b.splitlines(True),fromfile='a/tiny-glade/'+rel.as_posix(),tofile='b/tiny-glade/'+rel.as_posix()))
(base/'candidate.patch').write_text(''.join(patch),encoding='utf-8');(base/'manifest.json').write_text(json.dumps({'status':'evidence candidate only; no Cargo/typecheck performed','native_policy_fixture':'wall-startup-native-policy.json','files':rows},indent=2),encoding='utf-8')
print(json.dumps(rows,indent=2))
