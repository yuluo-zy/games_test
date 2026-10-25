from pathlib import Path
import json,hashlib,difflib
root=Path('D:/game');out=root/'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/profiling';base=json.loads((out/'base-sha256.json').read_text());files=[];chunks=[]
for rel,sha in base.items():
 a=(out/'base'/rel).read_bytes();b=(out/'candidate'/rel).read_bytes();h=hashlib.sha256(b).hexdigest()
 if a!=b:
  files.append({'path':rel,'base_sha256':sha,'candidate_sha256':h})
  chunks.extend(difflib.unified_diff(a.decode().splitlines(True),b.decode().splitlines(True),fromfile='a/'+rel,tofile='b/'+rel))
patch=''.join(chunks);(out/'frozen-cpu-draft.patch').write_text(patch)
manifest={'status':'frozen_draft_user_requested_commit_pause','production_modified':False,'cargo_run':False,'syntax_checked':False,'scope':'CPU owned profiling commands and puffin sink lifecycle; GPU/feed/new_frame not yet changed','files':files,'patch_sha256':hashlib.sha256(patch.encode()).hexdigest(),'remaining':['Independent original-byte verification of Start/Finish error/drop branches and duplicate-ID behavior','CPU lifecycle and serde integration validation','Screenshot gather consumer integration by screenshot_chain','GPU timing continuous enable/disable, pending query retirement, named pass feed into puffin','GlobalProfiler.new_frame original main-loop connection','Production integration and root Cargo/GPU acceptance']};(out/'FROZEN_DRAFT.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2));print(json.dumps(manifest,ensure_ascii=False,indent=2))
