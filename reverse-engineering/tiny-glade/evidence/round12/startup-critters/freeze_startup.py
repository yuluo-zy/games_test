from pathlib import Path
import hashlib,json,difflib,shutil
R=Path('D:/game');O=Path(__file__).resolve().parent;C=O/'candidate';F=O/'frozen-startup-v1';F.mkdir(exist_ok=True)
bases=json.loads((O/'candidate-base-hashes.json').read_text());files=[];patch='*** Begin Patch\n'
for rel,base in bases.items():
 p=C/rel;production=R/rel
 current=hashlib.sha256(production.read_bytes()).hexdigest() if production.exists() else None
 assert current==base,('base changed: merge hunk instead',rel,base,current)
 raw=p.read_bytes();proposed=raw.decode();old=production.read_text(encoding='utf8') if production.exists() else ''
 if base is None: patch+='*** Add File: '+rel+'\n'+''.join('+'+x+'\n' for x in proposed.splitlines())
 else:
  diff=list(difflib.unified_diff(old.splitlines(),proposed.splitlines(),n=3,lineterm=''))[2:]
  patch+='*** Update File: '+rel+'\n'+'\n'.join('@@' if x.startswith('@@ ') else x for x in diff)+'\n'
 q=F/'candidate'/rel;q.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,q)
 assert q.read_bytes()==raw
 files.append({'path':rel,'base_sha256':base,'proposed_sha256':hashlib.sha256(raw).hexdigest()})
patch+='*** End Patch\n';(F/'startup.patch').write_text(patch,encoding='utf8',newline='\n')
q=F/'gameplay_startup.inc.rs';shutil.copyfile(O/'integration/gameplay_startup.inc.rs',q)
manifest={'scope':'complete startup critters assets/births only; Plugin/true loop/control/upload recovery follows separately',
'status':'candidate only; rustfmt parsed9files; original4 gate combinations executed with explicit host boundaries; Rust tests prepared, not yet compiled/run',
'files':files,'patch_sha256':hashlib.sha256((F/'startup.patch').read_bytes()).hexdigest(),
'host_integration_sha256':hashlib.sha256(q.read_bytes()).hexdigest(),
'host_integration':'narrow insert in current gameplay_startup.rs, GameplaySessionStartup::Critters; root owns shared file integration',
'original_evidence':['native-manifest.json','startup_critters.original.asm','original-policy-fixtures.json','contract-result.json']}
(F/'manifest.json').write_text(json.dumps(manifest,indent=2),encoding='utf8');print(json.dumps(manifest,indent=2))
