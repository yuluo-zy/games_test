from pathlib import Path
import json,hashlib,difflib,subprocess
O=Path(__file__).resolve().parent;R=Path('D:/game')
arch=O/'candidate/tiny-glade/crates/country-core/src/resources/walls/decorator_archivist.rs'
s=arch.read_text(encoding='utf8')
# Preserve source before any document edit; the child module declaration is at
# the end of this file, so it must never be used as a header boundary.
backup=O/'recovery'/'restored-pre-format.rs';backup.parent.mkdir(exist_ok=True)
backup.write_text(s,encoding='utf8')
i=s.index('pub use crate::resources::walls::decorator_archivist::changes::DecoratorArchivist;')
s='''//! B address-scoped decorator archival and restoration owners.
//! The native resource adapter retains both actual collision channels and mesh
//! Arcs. extract_diff_against_storage follows the original live after-state
//! and semantic equality algorithm; the older no-storage API remains a legacy
//! approximation for callers not yet migrated and is not an equivalence gate.
'''+s[i:]
arch.write_text(s,encoding='utf8')
p='tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs'
dest=O/'restore-candidate'/p;s=dest.read_text(encoding='utf8');s=s[s.index('use bevy_ecs::prelude::*;'):]
s='''//! OriginalB restore140817a40/set42, with its actual eight system parameters.
//! mark_mutable runs first; ModeJustChanged is consumed once. With no new
//! event the wall eraser skips storage/collision restoration, but stairs always
//! restore. Other paths clear tracked buckets with their d2s indices, skip
//! deleted authoritative walls, and restore both actual collision owners.
//! A mode change extracts and drops the original diff; it publishes no history.

'''+s
if '/// `restore_archived_decorators` @' in s:
 start=s.index('/// `restore_archived_decorators` @')
 end=s.index('pub fn restore_archived_decorators(',start)
 s=s[:start]+'''/// Original system contract; register this concrete system in set42.
'''+s[end:]
s=s.replace('const LOG_TARGET: &str = "system_decorator::restore_archived_decorators";','')
dest.write_text(s,encoding='utf8')
world=O/'candidate/tiny-glade/crates/country-core/src/systems/collision/world.rs'
r=subprocess.run(['C:/Users/liyu/.rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe',
 '--edition','2024','--config','skip_children=true',str(arch),str(world),str(dest)],stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
(O/'format-v2.log').write_bytes(r.stdout)
assert r.returncode==0,r.stdout.decode(errors='replace')
base=json.loads((O/'restore-base.json').read_text());before=(O/'restore-base'/p).read_bytes();after=dest.read_bytes()
assert hashlib.sha256(before).hexdigest()==base['base_sha256']
diff=''.join(difflib.unified_diff(before.decode().splitlines(True),after.decode().splitlines(True),fromfile='a/'+p,tofile='b/'+p))
(O/'decorator-restore.patch').write_text(diff,encoding='utf8')
(O/'restore-candidate.json').write_text(json.dumps({'path':p,'base_sha256':base['base_sha256'],'proposed_sha256':hashlib.sha256(after).hexdigest(),
 'production_still_at_base':hashlib.sha256((R/p).read_bytes()).hexdigest()==base['base_sha256'],
 'production_written':False,'cargo_run':False,'status':'candidate rustfmt passed; full original independent review and compilation/runtime gates pending',
 'requires':'archival-owner v2 patch; concrete registration restore_archived_decorators without ::<C>, set42'},indent=2))
print('formatted; restore patch ready')
