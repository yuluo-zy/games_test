from pathlib import Path
import hashlib,json
r=Path(__file__).resolve().parent;repo=Path('D:/game')
source=r/'proposed/tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs'
target=repo/'tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
files=[source,r/'native-picker-all-fixtures.json',r/'native-arbitration-fixtures.json',r/'native-manifest.json',r/'native-cfg.json',r/'is_inside_bbx.patch']
files+=[r.parent/'roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources'/n for n in ['widget_params.rs','ui_state.rs']]
out=dict(candidate_only=True,source=str(source),target=str(target),target_existed=target.exists(),base_sha256=sha(target)if target.exists()else None,candidate_sha256=sha(source),files=[dict(path=str(p),sha256=sha(p))for p in files],private_helper_base=dict(path=str(repo/'tiny-glade/crates/utils/src/geometry/aabb.rs'),sha256=sha(repo/'tiny-glade/crates/utils/src/geometry/aabb.rs')),integration=['country-core/src/systems/ui/mod.rs: add pub mod hover_intersection','country-core/Cargo.toml: bevy_tasks.workspace = true','apply narrow is_inside_bbx.patch after base SHA verification','mount complete original set6 nine-system tuple supplied by ui_update owner'],validation='Native39+21 executed. Direct actual candidate rustc compiled;39-case test FAILED case21 screen min edge. Original44B proves current private helper includes boundary incorrectly; candidate-only correction provided. Latest in-source tests uncompiled. No Cargo run by this agent.')
(r/'candidate-manifest.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(out['candidate_sha256'])
