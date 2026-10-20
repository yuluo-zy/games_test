from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu'
relative = 'tiny-glade/crates/tiny-glade/tests/original_loading_present.rs'
source = root / relative
base = source.read_bytes()
text = base.decode('utf-8')
newline = '\r\n' if '\r\n' in text else '\n'
candidate = text.replace('\r\n', '\n')
replacements = [
    ('use bevy_ecs::world::World;\n', ''),
    ('    egui_backend::EguiBackend,\n    game_loop::GameRenderHostFactory,\n', '    game_loop::GameFrameDriver,\n'),
    ('    render_loop::{FramePacer, RenderMode, render},\n', ''),
    ('    let mut world = World::new();\n', '    let mut app = bevy_app::App::new();\n    {\n    let world = app.world_mut();\n'),
    ('&mut world', 'world'),
    ('''    let mut maintenance = bevy_ecs::schedule::Schedule::default();
    country_core::asset_libraries::runtime::add_asset_library_systems(&mut maintenance);
    let mut egui = EguiBackend::new(egui::Context::default());
    let mut pacer = FramePacer::default();''', '''    }
    // The production frame driver owns App::update, asset maintenance, egui,
    // the RenderHost factory and the recovered outer render call. Keep the
    // bounded loading fixture, and flush its MEI output after asset maintenance.
    app.add_systems(bevy_app::Last, loading_graph::flush_mei_frame);
    let mut driver = GameFrameDriver::new(app, egui::Context::default());'''),
    ('''                world.resource_mut::<bevy_time::Time<()>>()
                    .advance_by(Duration::from_secs_f64(1.0 / 60.0));
                maintenance.run(world);
                loading_graph::flush_mei_frame(world);
                render(&mut renderer.host(world, &mut egui), &mut pacer, RenderMode::LoadingScreen);''', '''                driver.app_mut().world_mut().resource_mut::<bevy_time::Time<()>>()
                    .advance_by(Duration::from_secs_f64(1.0 / 60.0));
                driver.frame(Some(egui::RawInput::default()), &mut renderer, |_| {});'''),
    ('world.insert_resource(WindowSize::new(size.width, size.height));\n                renderer.resize', 'driver.app_mut().world_mut().insert_resource(WindowSize::new(size.width, size.height));\n                renderer.resize'),
    ('PASS: {frames} production loading/presentation frames; real WSI present and shutdown completed', 'PASS: {frames} production GameFrameDriver loading/presentation frames; real WSI present and shutdown completed'),
]
for old, new in replacements:
    assert candidate.count(old) >= 1, repr(old)
    candidate = candidate.replace(old, new)
target = out / 'frame-driver-candidate' / relative
target.parent.mkdir(parents=True, exist_ok=True)
target.write_text(candidate.replace('\n', newline), encoding='utf-8', newline='')
(out / 'frame-driver-manifest.json').write_text(json.dumps({
    'scope': 'bounded loading WSI acceptance through the existing production GameFrameDriver; not full gameplay acceptance',
    'production_written': False,
    'files': [{'path': relative, 'base_sha256': hashlib.sha256(base).hexdigest()}],
    'validation': 'root owns Cargo and actual GPU acceptance',
}, indent=2), encoding='utf-8')
print(target)
