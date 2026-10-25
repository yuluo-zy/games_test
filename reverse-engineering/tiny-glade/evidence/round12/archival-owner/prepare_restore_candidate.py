from pathlib import Path
import shutil,hashlib,json
R=Path('D:/game');O=Path(__file__).resolve().parent
p='tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs'
for side in ['restore-base','restore-candidate']:
 dest=O/side/p;dest.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(R/p,dest)
(O/'restore-base.json').write_text(json.dumps({'path':p,'base_sha256':hashlib.sha256((R/p).read_bytes()).hexdigest()},indent=2))
