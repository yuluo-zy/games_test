from pathlib import Path
import json,hashlib,difflib,subprocess
ROOT=Path('D:/game');HERE=Path(__file__).parent;OUT=HERE/'zero-close-v2'
files=['tiny-glade/crates/tiny-glade/src/game_loop.rs','tiny-glade/crates/tiny-glade/src/event_loop.rs',
       'tiny-glade/crates/tiny-glade/src/run_game.rs','tiny-glade/crates/tiny-glade/src/lib.rs']
new='tiny-glade/crates/tiny-glade/src/shutdown_runtime.rs'
assert not (OUT/'manifest.json').exists(),'Frozen output already exists; do not overwrite'
if not (OUT/'initial-applied.json').exists():
    for p in files:
        blob=(ROOT/p).read_bytes()
        for directory in ['base','initial']:
            target=OUT/directory/p;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(blob)
    # Apply the first close revision only to this evidence-owned staging tree.
    subprocess.run(['git','apply','--ignore-space-change','--directory='+str((OUT/'initial').relative_to(ROOT)).replace('\\','/'),
                    str(HERE/'zero-close/candidate.patch')],cwd=ROOT,check=True)
    (OUT/'initial-applied.json').write_text('{}\n')
base={p:(OUT/'base'/p).read_bytes() for p in files}
src={p:(OUT/'initial'/p).read_text(encoding='utf-8').replace('\r\n','\n') for p in files+[new]}
def swap(s,a,b):
    assert s.count(a)==1,(s.count(a),a[:110])
    return s.replace(a,b)
p=files[0]
src[p]=swap(src[p],'    pub(crate) shutdown_done: bool,','    pub(crate) shutdown_done: bool,\n    pub(crate) shutdown_progress: crate::shutdown_runtime::ShutdownProgress,')
src[p]=swap(src[p],'            shutdown_done: false,','            shutdown_done: false,\n            shutdown_progress: Default::default(),')
start=src[p].index('    pub(crate) fn shut_down(&mut self) -> anyhow::Result<()> {')
end=src[p].index('\n    /// B 0x1404e2470:',start)
src[p]=src[p][:start]+(HERE/'shutdown_function.txt').read_text(encoding='utf-8')+src[p][end:]
p=files[1]
src[p]=swap(src[p],'''                *event_failure.borrow_mut() = Some(error);
                if let Err(shutdown) = self.shut_down() {
                    log::error!("GPU shutdown after loop failure: {shutdown:#}");
                }
                target.exit();''','''                // Preserve the first failure across cleanup retries, retaining
                // World/Window/renderer while a real wait remains incomplete.
                if event_failure.borrow().is_none() {
                    *event_failure.borrow_mut() = Some(error);
                } else {
                    log::error!("shutdown retry: {error:#}");
                }
                if !self.shutdown_progress.started {
                    if let Err(shutdown) = self.shut_down() {
                        log::error!("shutdown after loop failure: {shutdown:#}");
                    }
                }
                if self.shutdown_done {
                    target.exit();
                } else {
                    target.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                        std::time::Instant::now() + std::time::Duration::from_millis(1)));
                    self.window.request_redraw();
                }''')
src[p]=swap(src[p],'''        use winit::event::{Event, WindowEvent};
        match event {''','''        use winit::event::{Event, WindowEvent};
        if self.shutdown_progress.started {
            let result = self.shut_down();
            if self.shutdown_done {
                target.exit();
            } else {
                target.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                    std::time::Instant::now() + std::time::Duration::from_millis(1)));
                self.window.request_redraw();
            }
            return result;
        }
        match event {''')
p=files[2]
start=src[p].index('    /// Host ownership barrier: no detached loader')
end=src[p].index('    /// Original main_loop polls only',start)
src[p]=src[p][:start]+(HERE/'loader_shutdown_methods.txt').read_text(encoding='utf-8')+src[p][end:]
src[p]=swap(src[p],'''        if let Err(error) = self.cancel_and_join() {
            log::error!("gameplay preparation shutdown: {error:#}");
        }''','''        if let Err(error) = self.stop_worker() {
            log::error!("gameplay preparation shutdown: {error:#}");
        }
        if let Err(error) = self.disconnect_save_workers() {
            log::error!("pending gameplay save shutdown: {error:#}");
        }''')
src[new]=(HERE/'shutdown_runtime.rs').read_text(encoding='utf-8')
patch=[];records=[]
for p,new_source in src.items():
    old=base.get(p,b'').decode().replace('\r\n','\n')
    dest=OUT/'candidate'/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(new_source.encode())
    patch.extend(difflib.unified_diff(old.splitlines(True),new_source.splitlines(True),fromfile='a/'+p if p in base else '/dev/null',tofile='b/'+p))
    records.append({'path':p,'base_sha256':hashlib.sha256(base[p]).hexdigest() if p in base else None,'candidate_sha256':hashlib.sha256(new_source.encode()).hexdigest()})
(OUT/'candidate.patch').write_bytes(''.join(patch).encode('utf-8'))
(OUT/'manifest.json').write_text(json.dumps({'files':records,'scope':'Original zero-size branch plus authorized host close/cleanup adaptation. Persistent systems, explicit phases, retained owners on retry. Narrow hunks only.','depends_on':'screenshot/host-migration.patch screenshot API, omit its GameLoop hunk','validation':'No production writes or Cargo by frame owner.'},indent=2)+'\n')
print(json.dumps({'files':len(records),'patch_lines':len(patch)}))
