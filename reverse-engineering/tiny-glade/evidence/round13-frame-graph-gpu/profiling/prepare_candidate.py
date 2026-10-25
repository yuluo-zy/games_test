from pathlib import Path
import hashlib,json
root=Path('D:/game');out=root/'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/profiling';paths=['tiny-glade/crates/country-core/src/meta_commander.rs','tiny-glade/crates/country-core/Cargo.toml','tiny-glade/crates/tiny-glade/src/game_loop.rs','tiny-glade/crates/tiny-glade/src/production_renderer.rs','tiny-glade/crates/rhapsody/src/rg/profiling.rs','tiny-glade/crates/rhapsody/src/rg/vulkan.rs','tiny-glade/crates/rhapsody/src/rg/executor.rs']
manifest={}
for rel in paths:
 b=(root/rel).read_bytes();manifest[rel]=hashlib.sha256(b).hexdigest()
 for sub in ['base','candidate']:
  p=out/sub/rel;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(b)
(out/'base-sha256.json').write_text(json.dumps(manifest,indent=2))
