from pathlib import Path
import difflib,json,hashlib
ROOT=Path('D:/game');OUT=Path(__file__).parent/'zero-close'
paths=['tiny-glade/crates/tiny-glade/src/game_loop.rs','tiny-glade/crates/tiny-glade/src/event_loop.rs',
       'tiny-glade/crates/tiny-glade/src/run_game.rs','tiny-glade/crates/tiny-glade/src/lib.rs']
base={p:(ROOT/p).read_bytes() for p in paths}
src={p:b.decode().replace('\r\n','\n') for p,b in base.items()}
def swap(s,a,b):
    assert s.count(a)==1,(s.count(a),a[:120])
    return s.replace(a,b)
p=paths[0]
anchor='fn apply_menu_fps_limit(fps: f32, menu_limits: bool, profiling: bool) -> f32 {'
src[p]=swap(src[p],anchor,'''/// Native 1404dc8d2..dc971: real resize-published display extent gates the
/// whole simulation/render frame. A zero component takes the 1ms wait branch.
pub fn frame_waits_for_window_extent(world: &World) -> bool {
    world.resource::<country_core::render::render_config::FrameRenderConfig>()
        .extent_0.contains(&0)
}

'''+anchor)
anchor='''        // B 1404ddf28..3c: real OS egui input is consumed only while the
'''
src[p]=swap(src[p],anchor,'''        if frame_waits_for_window_extent(self.driver.app.world()) {
            // B's wait skips App::update/MEI/egui/render/frame counters/FPS.
            // The host-only close route shares the real persistent save and
            // shutdown consumers, so zero-size close can finish safely.
            let closed = crate::shutdown_runtime::finish_pending_shutdown_without_frame(
                self.driver.app.world_mut());
            if !closed {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            self.apply_pending_video_settings()?;
            return Ok(!closed);
        }
'''+anchor)
anchor='''        self.shutdown_done = true;
        self.custom_cursor_buffer = None;'''
src[p]=swap(src[p],anchor,'''        // The loader may still own a gameplay World and a GPU warmup bridge.
        // Cancel and join it before releasing the shared graphics device.
        if let Some(mut preparation) = self.preparation.take() {
            preparation.cancel_and_join()?;
        }
        crate::shutdown_runtime::disconnect_save_worker(self.driver.app.world_mut())?;
        self.shutdown_done = true;
        self.custom_cursor_buffer = None;''')
anchor='    fn debug_window_flags_choose_actual_os_egui_input() {'
src[p]=swap(src[p],anchor,'''    fn zero_extent_gate_reads_the_actual_frame_config_owner() {
        use country_core::render::render_config::{FrameRenderConfig, VideoSettings};
        use country_core::resources::window::WindowSize;
        let mut world = World::new();
        world.insert_resource(WindowSize::new(0, 0));
        world.insert_resource(FrameRenderConfig::at_startup(
            WindowSize::new(1280, 720), &VideoSettings::default()));
        assert!(!frame_waits_for_window_extent(&world));
        for extent in [[0, 720], [1280, 0], [0, 0]] {
            world.resource_mut::<FrameRenderConfig>().extent_0 = extent;
            assert!(frame_waits_for_window_extent(&world));
        }
        world.resource_mut::<FrameRenderConfig>().extent_0 = [1280, 720];
        assert!(!frame_waits_for_window_extent(&world));
    }

    #[test]
'''+anchor)
p=paths[1]
src[p]=swap(src[p],'''                    .set(width, height);
                self.app_mut()
''','''                    .set(width, height);
                // Native 1404dad4f/51: the gate reads this actual owner.
                self.app_mut().world_mut()
                    .resource_mut::<country_core::render::render_config::FrameRenderConfig>()
                    .extent_0 = [width, height];
                self.app_mut()
''')
src[p]=swap(src[p],'''                // A close command reaches the app before the loop quit marker is
                // checked. Even a minimized window must run this last update.
                self.window.request_redraw();''','''                // B emits the real command; the main-thread host also services
                // it without a simulated frame when WSI is suspended or zero.
                if self.suspended
                    || crate::game_loop::frame_waits_for_window_extent(self.app().world())
                {
                    if crate::shutdown_runtime::finish_pending_shutdown_without_frame(
                        self.app_mut().world_mut())
                    {
                        self.shut_down()?;
                        target.exit();
                    }
                } else {
                    self.window.request_redraw();
                }''')
p=paths[2]
src[p]=swap(src[p],'''    app.add_systems(bevy_app::Update,
        country_core::systems::save::save_game::save_game_system
            .in_set(GameSystemSet(43))
            .run_if(|active: Option<bevy_ecs::prelude::Res<country_core::resources::session::ActiveSession>>|active.is_some()));''',
'''    crate::shutdown_runtime::install_save_maintenance(&mut app);''')
src[p]=swap(src[p],'''    app.add_systems(
        bevy_app::PostUpdate,
        country_core::resources::shutdown::initiate_game_shutdown,
    );''','''    crate::shutdown_runtime::install_shutdown_maintenance(&mut app);''')
src[p]=swap(src[p],'''    app.add_event::<country_core::resources::events::ShutdownGameCmd>();
    Ok(app)
}''','''    app.add_event::<country_core::resources::events::ShutdownGameCmd>();
    crate::shutdown_runtime::install_shutdown_maintenance(&mut app);
    Ok(app)
}''')
src[p]=swap(src[p],'''    cancel: Arc<std::sync::atomic::AtomicBool>,
}''','''    cancel: Arc<std::sync::atomic::AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}''')
src[p]=swap(src[p],'''        std::thread::Builder::new().name("async load".to_owned()).spawn(move || {''',
'''        let worker = std::thread::Builder::new().name("async load".to_owned()).spawn(move || {''')
src[p]=swap(src[p],'''                Ok(gameplay)
            }))''','''                Ok(())
            }))''')
src[p]=swap(src[p],'''            let _ = sender.send(result);''','''            let result = match result {
                Ok(()) => Ok(gameplay),
                Err(error) => {
                    if let Err(cleanup) = crate::shutdown_runtime::disconnect_save_worker(gameplay.world_mut()) {
                        log::error!("async loader save cleanup: {cleanup:#}");
                    }
                    Err(error)
                }
            };
            let _ = sender.send(result);''')
src[p]=swap(src[p],'''            pending: None,
            cancel,
        })''','''            pending: None,
            cancel,
            worker: Some(worker),
        })''')
anchor='''    /// Original main_loop polls only at the logo's waiting stage and switches
'''
src[p]=swap(src[p],anchor,'''    /// Host ownership barrier: no detached loader may keep updating a World or
    /// warming a graph while the OS owner shuts the graphics device down.
    pub(crate) fn cancel_and_join(&mut self) -> anyhow::Result<()> {
        self.cancel.store(true, std::sync::atomic::Ordering::Release);
        if let Some(worker) = self.worker.take() {
            worker.join().map_err(|_| anyhow::anyhow!("async gameplay worker panicked during shutdown"))?;
        }
        if let Some(mut gameplay) = self.pending.take() {
            crate::shutdown_runtime::disconnect_save_worker(gameplay.world_mut())?;
        }
        // A finished worker can have sent its World before the logo allowed
        // handoff. That real pending owner also needs deterministic save exit.
        while let Ok(result) = self.receiver.try_recv() {
            if let Ok(mut gameplay) = result {
                crate::shutdown_runtime::disconnect_save_worker(gameplay.world_mut())?;
            }
        }
        Ok(())
    }

'''+anchor)
src[p]=swap(src[p],'''        self.cancel
            .store(true, std::sync::atomic::Ordering::Release);''','''        if let Err(error) = self.cancel_and_join() {
            log::error!("gameplay preparation shutdown: {error:#}");
        }''')
src[p]=swap(src[p],'''            cancel: Arc::new(std::sync::atomic::AtomicBool::new(false)) };''',
'''            cancel: Arc::new(std::sync::atomic::AtomicBool::new(false)), worker: None };''')
p=paths[3]
src[p]=swap(src[p],'pub mod run_game;','pub mod run_game;\npub(crate) mod shutdown_runtime;')
new_path='tiny-glade/crates/tiny-glade/src/shutdown_runtime.rs'
src[new_path]=(Path(__file__).parent/'shutdown_runtime.rs').read_text()
patch=[];records=[];OUT.mkdir(parents=True,exist_ok=True)
for p,new in src.items():
    old_bytes=base.get(p,b'');old=old_bytes.decode().replace('\r\n','\n')
    for part,blob in [('base',old_bytes),('candidate',new.encode())]:
        dest=OUT/part/p;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(blob)
    patch.extend(difflib.unified_diff(old.splitlines(True),new.splitlines(True),fromfile='a/'+p if p in base else '/dev/null',tofile='b/'+p))
    records.append({'path':p,'base_sha256':hashlib.sha256(old_bytes).hexdigest() if p in base else None,'candidate_sha256':hashlib.sha256(new.encode()).hexdigest()})
(OUT/'candidate.patch').write_text(''.join(patch),encoding='utf-8')
(OUT/'manifest.json').write_text(json.dumps({'files':records,'scope':'Original zero-extent gate plus explicitly authorized host close adaptation. Narrow hunks; never whole-file replacement.','validation':'No production writes or Cargo by frame owner.'},indent=2)+'\n')
print(json.dumps({'files':len(records),'patch_lines':len(patch)}))
