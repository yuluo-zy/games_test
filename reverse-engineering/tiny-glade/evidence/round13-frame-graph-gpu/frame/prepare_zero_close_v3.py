from pathlib import Path
import json,hashlib,difflib,subprocess
ROOT=Path('D:/game');HERE=Path(__file__).parent;OUT=HERE/'zero-close-v3'
files=['tiny-glade/crates/tiny-glade/src/game_loop.rs','tiny-glade/crates/tiny-glade/src/event_loop.rs',
       'tiny-glade/crates/tiny-glade/src/run_game.rs','tiny-glade/crates/tiny-glade/src/lib.rs']
new='tiny-glade/crates/tiny-glade/src/shutdown_runtime.rs'
assert not (OUT/'manifest.json').exists(),'Frozen output already exists; do not overwrite'
if not (OUT/'initial-applied.json').exists():
    for p in files:
        blob=(ROOT/p).read_bytes()
        for part in ['base','initial']:
            target=OUT/part/p;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(blob)
    subprocess.run(['git','apply','--ignore-space-change','--directory='+str((OUT/'initial').relative_to(ROOT)).replace('\\','/'),
                    str(HERE/'zero-close-v2/candidate.patch')],cwd=ROOT,check=True)
    (OUT/'initial-applied.json').write_text('{}\n')
base={p:(OUT/'base'/p).read_bytes() for p in files}
src={p:(OUT/'initial'/p).read_text(encoding='utf-8') for p in files+[new]}
def swap(s,a,b):
    assert s.count(a)==1,(s.count(a),a[:120])
    return s.replace(a,b)
p=files[2]
src[p]=swap(src[p],'''pub struct GameplayPreparation {
    receiver: std::sync::mpsc::Receiver<anyhow::Result<bevy_app::SubApp>>,''','''// A failed/cancelled producer still owns its actual World. Never erase that
// ownership into an anyhow error (SubApp is Send, but need not be Sync).
struct GameplayPreparationResult {
    app: bevy_app::SubApp,
    result: anyhow::Result<()>,
}

#[derive(Debug)]
struct GameplayPreparationCancelled;
impl std::fmt::Display for GameplayPreparationCancelled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("gameplay preparation cancelled")
    }
}
impl std::error::Error for GameplayPreparationCancelled {}

pub struct GameplayPreparation {
    receiver: std::sync::mpsc::Receiver<GameplayPreparationResult>,''')
src[p]=swap(src[p],'''if cancelled.load(Ordering::Acquire) { anyhow::bail!("gameplay preparation cancelled"); }''',
'''if cancelled.load(Ordering::Acquire) { return Err(GameplayPreparationCancelled.into()); }''')
src[p]=swap(src[p],'''            let result = match result {
                Ok(()) => Ok(gameplay),
                Err(error) => {
                    if let Err(cleanup) = crate::shutdown_runtime::disconnect_save_worker(gameplay.world_mut()) {
                        log::error!("async loader save cleanup: {cleanup:#}");
                    }
                    Err(error)
                }
            };
            let _ = sender.send(result);''','''            // Both success and error retain the original World until the OS
            // owner has stopped save/capture/GPU consumers. No worker-side drop.
            let _ = sender.send(GameplayPreparationResult { app: gameplay, result });''')
src[p]=swap(src[p],'''        while let Ok(result) = self.receiver.try_recv() {
            if let Ok(gameplay) = result {
                assert!(self.pending.is_none(), "async loader sent its World twice");
                self.pending = Some(gameplay);
            }
        }''','''        while let Ok(packet) = self.receiver.try_recv() {
            assert!(self.pending.is_none(), "async loader sent its World twice");
            self.pending = Some(packet.app);
            if let Err(error) = packet.result {
                if !error.is::<GameplayPreparationCancelled>() && first_error.is_none() {
                    first_error = Some(error);
                }
            }
        }''')
src[p]=swap(src[p],'''                Ok(Ok(app)) => self.pending = Some(app),
                Ok(Err(error)) => return Err(error),''','''                Ok(packet) => {
                    // Store the failed World before propagating its error.
                    self.pending = Some(packet.app);
                    packet.result?;
                }''')
src[p]=swap(src[p],'''        sender.send(Ok(std::mem::take(gameplay.main_mut()))).unwrap();''','''        sender.send(GameplayPreparationResult {
            app: std::mem::take(gameplay.main_mut()), result: Ok(()) }).unwrap();''')
# SendError::unwrap requires Debug; SubApp is intentionally opaque.
src[p]=swap(src[p],'''            app: std::mem::take(gameplay.main_mut()), result: Ok(()) }).unwrap();''','''            app: std::mem::take(gameplay.main_mut()), result: Ok(()) }).ok().expect("preparation receiver");''')
patch=[];records=[]
for p,text in src.items():
    text=text.replace('\r\n','\n');old=base.get(p,b'').decode().replace('\r\n','\n')
    dest=OUT/'candidate'/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(text.encode())
    patch.extend(difflib.unified_diff(old.splitlines(True),text.splitlines(True),fromfile='a/'+p if p in base else '/dev/null',tofile='b/'+p))
    records.append({'path':p,'base_sha256':hashlib.sha256(base[p]).hexdigest() if p in base else None,'candidate_sha256':hashlib.sha256(text.encode()).hexdigest()})
(OUT/'candidate.patch').write_bytes(''.join(patch).encode())
(OUT/'manifest.json').write_text(json.dumps({'files':records,'scope':'v3 keeps failed/cancelled loader Worlds in the same owning result packet until OS-side save/capture/GPU cleanup. Narrow hunks only.','depends_on':'screenshot/host-migration.patch screenshot API, omit its GameLoop hunk','validation':'No production writes or Cargo by frame owner.'},indent=2)+'\n')
print(json.dumps({'files':len(records),'patch_lines':len(patch)}))
