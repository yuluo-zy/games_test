from pathlib import Path
import hashlib,json,difflib
r=Path('D:/game');ev=r/'reverse-engineering/tiny-glade/evidence/round12/wall-gpu';b=ev/'candidates-sheep'
p=b/'crates/systems/critters/src/bird_spots.rs';s=p.read_text(encoding='utf-8').replace('returned.extend(std::mem::take(&mut self.spots).into_vec());','for spot in std::mem::take(&mut self.spots).into_vec() { returned.push(spot); }');p.write_text(s,encoding='utf-8')
files=[];patch=[]
for p in sorted(b.rglob('*.rs')):
 rel=p.relative_to(b);dest=r/'tiny-glade'/rel;before=dest.read_bytes() if dest.exists() else None;after=p.read_bytes();sha=lambda x:hashlib.sha256(x).hexdigest() if x is not None else None
 files.append({'path':str(rel).replace('\\','/'),'base_sha256':sha(before),'candidate_sha256':sha(after),'bytes':len(after),'merge':'narrow hunks; preserve independent CritterControl/critters mod and latest startup assembly'})
 patch.extend(difflib.unified_diff(before.decode('utf-8').splitlines(True) if before else [],after.decode('utf-8').splitlines(True),fromfile='a/tiny-glade/'+str(rel).replace('\\','/'),tofile='b/tiny-glade/'+str(rel).replace('\\','/')))
(b/'manifest.json').write_text(json.dumps({'status':'candidate only; rustfmt parsing complete; no Cargo or production writes; requires independent UI SheepControl and bird/core Wren candidates','native_original_full_birth_policy':'sheep-spawn-native-policy.json','dependencies':['round12/sheep-startup-assets/candidate-checkpoint.json (already applied by root)','round12/birdspot-recycle/candidate-checkpoint.json','independent_verification canonical Critter marker + CritterControl (narrow components hunk)','independent_ui_acceptance SheepControl unique Component/new/snapshot/runtime checkpoint'],'central_adapter':'../SHEEP_APP_ADAPTER.rs','files':files},indent=2),encoding='utf-8')
(b/'candidate.patch').write_text(''.join(patch),encoding='utf-8')
print(len(files),'source files frozen')
