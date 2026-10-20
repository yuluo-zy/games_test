from pathlib import Path
import hashlib,json,difflib
r=Path('D:/game');e=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu';b=e/'candidates-roof-startup';p=b/'crates/country-core/src/startup/startup_roof.rs';rel=p.relative_to(b);a=r/'tiny-glade'/rel
(b/'candidate.patch').write_text(''.join(difflib.unified_diff(a.read_text(encoding='utf-8').splitlines(True),p.read_text(encoding='utf-8').splitlines(True),fromfile='a/tiny-glade/'+rel.as_posix(),tofile='b/tiny-glade/'+rel.as_posix())),encoding='utf-8')
(b/'manifest.json').write_text(json.dumps({'status':'candidate evidence only; rustfmt parsed, not Cargo/typechecked/tested','native_policy':'roof-startup-native-policy.json','files':[{'path':rel.as_posix(),'base_sha256':hashlib.sha256(a.read_bytes()).hexdigest(),'candidate_sha256':hashlib.sha256(p.read_bytes()).hexdigest()}]},indent=2),encoding='utf-8')
print(hashlib.sha256(p.read_bytes()).hexdigest())

