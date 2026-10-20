from pathlib import Path
import json,hashlib,shutil
R=Path('D:/game');O=Path(__file__).resolve().parent;A=R/'reverse-engineering/tiny-glade/evidence/round12/archival-owner/frozen-v2'
files={
 'tiny-glade/crates/country-core/src/resources/history/decorator_edit.rs':R/'tiny-glade/crates/country-core/src/resources/history/decorator_edit.rs',
 'tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs':A/'candidate/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs',
 'tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs':A/'candidate/tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs'}
out={}
for name,p in files.items():
 raw=p.read_bytes();out[name]={'base_sha256':hashlib.sha256(raw).hexdigest(),'base_source':str(p)}
 for side in ['base','candidate']:
  d=O/side/name;d.parent.mkdir(parents=True,exist_ok=True);d.write_bytes(raw)
(O/'base-manifest.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
