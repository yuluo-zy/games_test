//! B loading graph -> original presentation resolve -> real WSI submission.
//! Explicit validation inputs: a loading-only World, bounded clock and window.
//! This does not run or pretend to complete the gameplay loader. It exercises
//! GameFrameDriver::frame and ProductionRenderer through the recovered outer render.
use std::{cell::RefCell, path::PathBuf, rc::Rc, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use country_core::{
    asset_libraries::{
        configwatch::ConfigWatch, meshwatch::MeshWatch,
        placement_data_library::AssetPlacementDataLibrary, shader_library::AssetShaderLibrary,
    },
    game_config::GameConfig,
    resources::{logo_animation::LogoAnimation, window::WindowSize},
    systems::render_loop::vermeer::VermeerState,
};
use tiny_glade::{
    game_loop::GameFrameDriver, loading_graph, production_renderer::ProductionRenderer,
    run_game::ApplicationShaderLibrary,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    let installed = std::env::var_os("TG_INSTALL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../ljxsj_92385/Tiny Glade")
        })
        .canonicalize()
        .context("original packaged installation")?;
    std::env::set_current_dir(&installed)?;
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Tiny Glade — original loading WSI validation")
            .with_inner_size(winit::dpi::LogicalSize::new(480.0, 270.0))
            .build(&event_loop)?,
    );
    let shaders = Arc::new(AssetShaderLibrary::new());
    // This is the production device, surface, swapchain and per-App owner.
    // Initialization failure is an error; this executable never skips a GPU.
    let mut renderer = ProductionRenderer::new(
        window.clone(),
        installed.join("compiled-shaders"),
        shaders.clone(),
    )?;
    let mut app = bevy_app::App::new();
    {
        let world = app.world_mut();
        world.insert_resource(ApplicationShaderLibrary(shaders.clone()));
        world.insert_resource(GameConfig::default());
        renderer.bridge().install_asset_libraries(world)?;
        world.insert_resource(ConfigWatch::new(false));
        world.insert_resource(MeshWatch::new(false));
        world.insert_resource(AssetPlacementDataLibrary::new());
        world.insert_resource(country_core::render::frame_constants::FrameConstants::default());
        let size = window.inner_size();
        world.insert_resource(WindowSize::new(size.width, size.height));
        world.insert_resource(bevy_time::Time::<()>::default());
        let mut vermeer = VermeerState::default();
        vermeer.activate(true, true);
        world.insert_resource(vermeer);
        world.insert_resource(LogoAnimation::default());
        // This explicit World fixture has no App scheduler; call the same two-entry
        // producer directly before the public helper. GameLoop uses real Startup.
        country_core::startup::init_populate_minimal_startup_ssbo_library::init_populate_minimal_startup_ssbo_library(
        renderer.bridge().hosts().buffers.as_ref(),
        world.resource::<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>(),
    );
        loading_graph::initialize_loading_resources(
            world,
            renderer.device().clone(),
            renderer.bridge().hosts(),
            &installed,
            false,
        )?;
        country_core::resources::logo_animation::startup_animated_logo(
            world,
            renderer.bridge().shader_backend(),
            &shaders,
        )?;
    }
    // The production frame driver owns App::update, asset maintenance, egui,
    // the RenderHost factory and the recovered outer render call. Keep the
    // bounded loading fixture, and flush its MEI output after asset maintenance.
    app.add_systems(bevy_app::Last, loading_graph::flush_mei_frame);
    let mut driver = GameFrameDriver::new(app, egui::Context::default());
    let mut frames = 0;
    let result = Rc::new(RefCell::new(None::<Result<()>>));
    let observed = result.clone();
    event_loop.run(move |event, target| {
        target.set_control_flow(ControlFlow::Poll);
        let step = match event {
            Event::AboutToWait => (|| -> Result<()> {
                driver.app_mut().world_mut().resource_mut::<bevy_time::Time<()>>()
                    .advance_by(Duration::from_secs_f64(1.0 / 60.0));
                driver.frame(Some(egui::RawInput::default()), &mut renderer, |_| {});
                if let Some(error) = renderer.take_error() { return Err(error); }
                frames += 1;
                if frames == 3 {
                    anyhow::ensure!(renderer.any_present_successful(),
                        "production loading graph did not produce a successful WSI present");
                    renderer.shutdown()?;
                    eprintln!("PASS: {frames} production GameFrameDriver loading/presentation frames; real WSI present and shutdown completed");
                    *observed.borrow_mut() = Some(Ok(()));
                    target.exit();
                }
                Ok(())
            })(),
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                Err(anyhow::anyhow!("validation window closed before present verification"))
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                driver.app_mut().world_mut().insert_resource(WindowSize::new(size.width, size.height));
                renderer.resize([size.width, size.height])
            }
            _ => Ok(()),
        };
        if let Err(error) = step {
            *observed.borrow_mut() = Some(Err(error));
            target.exit();
        }
    })?;
    result
        .borrow_mut()
        .take()
        .context("OS event loop exited without validation result")?
}
