#[cfg(test)]
mod startup_handoff_integration {
    use super::*;
    use country_core::{
        input::ActiveInputModeTracker,
        render::{adaptive_render_state::AdaptiveRenderState, render_config::{FrameRenderConfig, MonitorMatch, VideoMode}},
        resources::{gui::{EguiContextResource, PendingEguiContextResource}, logo_animation::LogoAnimation,
            window::{AvailableMonitor, AvailableMonitors}},
        systems::{render_loop::vermeer::{VermeerState, VermeerPhase}, save::load_game::{StartupRevealStarted, StartupRevealFinished}},
    };

    fn events(app: &mut App) {
        app.add_event::<StartupRevealStarted>().add_event::<StartupRevealFinished>();
    }

    fn six_owners(app: &mut App) -> (usize, usize, FrameRenderConfig) {
        let mut config = GameConfig::default();
        config.video.fps_limit = 117.5;
        config.lang_override = Some("original-loading-owner".to_owned());
        let config_allocation = config.lang_override.as_ref().unwrap().as_ptr() as usize;
        let window = WindowSize::new(1431, 913);
        let frame = FrameRenderConfig {
            extent_0: [1431,913], extent_8: [1171,733], extent_10: [1001,607],
            world_resolution_scale: 0.75, tgsr_resolution_scale: 0.5, flag_20: true,
        };
        let monitors = AvailableMonitors { monitors: vec![AvailableMonitor {
            monitor: MonitorMatch { name: Some("loading-monitor".into()), physical_position: [-1431,91], physical_size: [1431,913] },
            video_modes: vec![VideoMode { resolution: [1431,913], refresh_rate_millihertz: 144_001, bit_depth: 30 }],
        }] };
        let monitor_allocation = monitors.monitors.as_ptr() as usize;
        app.insert_resource(config);
        app.insert_resource(ActiveInputModeTracker::new_default_controller(std::time::Duration::from_millis(827)));
        app.insert_resource(window);
        app.insert_resource(monitors);
        app.insert_resource(frame);
        app.insert_resource(AdaptiveRenderState::at_startup(true));
        (config_allocation, monitor_allocation, frame)
    }

    #[test]
    fn ordinary_startup_waits_for_logo_then_moves_real_owners_and_starts_reveal() {
        let mut loading = App::new();
        let (config_ptr, monitors_ptr, expected_frame) = six_owners(&mut loading);
        events(&mut loading);
        let mut logo = LogoAnimation::default();
        logo.progress[3] = 0.25;
        loading.insert_resource(logo);
        let context = egui::Context::default();
        let marker = egui::Id::new("loading-egui-owner");
        context.data_mut(|data| data.insert_temp(marker, 912_u32));
        loading.insert_resource(EguiContextResource(context.clone()));
        loading.insert_resource(VermeerState { frame_count: 71, progress: 0.37,
            timer: 8.25, target_extent: [1431,913], phase: VermeerPhase::Ready,
            hold: true, freeze_camera: false, ..Default::default() });
        let loading_shaders = Arc::new(country_core::asset_libraries::shader_library::AssetShaderLibrary::new());
        loading.insert_resource(ApplicationShaderLibrary(loading_shaders.clone()));
        let mut gameplay = App::new();
        events(&mut gameplay);
        let gameplay_shaders = Arc::new(country_core::asset_libraries::shader_library::AssetShaderLibrary::new());
        gameplay.insert_resource(ApplicationShaderLibrary(gameplay_shaders.clone()));
        gameplay.insert_resource(WindowSize::new(1,2));
        gameplay.insert_resource(GameConfig::default());
        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send(Ok(std::mem::take(gameplay.main_mut()))).unwrap();
        let mut preparation = GameplayPreparation { receiver, pending: None,
            cancel: Arc::new(std::sync::atomic::AtomicBool::new(false)) };
        assert!(!preparation.poll(&mut loading).unwrap());
        assert_eq!(loading.world().resource::<LogoAnimation>().progress[3],1.0);
        assert_eq!(loading.world().resource::<GameConfig>().video.fps_limit,117.5);
        assert!(loading.world().resource::<Events<StartupRevealStarted>>().is_empty());
        loading.world_mut().resource_mut::<LogoAnimation>().progress[4]=1.0;
        assert!(preparation.poll(&mut loading).unwrap());
        let world=loading.world();
        assert_eq!(world.resource::<GameConfig>().lang_override.as_ref().unwrap().as_ptr() as usize,config_ptr);
        assert_eq!(world.resource::<GameConfig>().video.fps_limit,117.5);
        assert_eq!(*world.resource::<ActiveInputModeTracker>(),ActiveInputModeTracker::new_default_controller(std::time::Duration::from_millis(827)));
        assert_eq!(*world.resource::<WindowSize>(),WindowSize::new(1431,913));
        let monitors=world.resource::<AvailableMonitors>();
        assert_eq!(monitors.monitors.as_ptr() as usize,monitors_ptr);
        assert_eq!(monitors.monitors[0].monitor.name.as_deref(),Some("loading-monitor"));
        assert_eq!(monitors.monitors[0].video_modes[0].refresh_rate_millihertz,144_001);
        assert_eq!(*world.resource::<FrameRenderConfig>(),expected_frame);
        assert_eq!(*world.resource::<AdaptiveRenderState>(),AdaptiveRenderState::at_startup(true));
        assert_eq!(world.resource::<Events<StartupRevealStarted>>().len(),1);
        assert!(world.resource::<Events<StartupRevealFinished>>().is_empty());
        let reveal=world.resource::<VermeerState>();
        assert_eq!(reveal.frame_count,71); assert_eq!(reveal.progress,0.37);
        assert_eq!(reveal.timer,8.25); assert_eq!(reveal.target_extent,[1431,913]);
        assert_eq!(reveal.phase,VermeerPhase::Ready); assert!(!reveal.hold); assert!(!reveal.freeze_camera);
        assert_eq!(world.resource::<EguiContextResource>().0.data(|data|data.get_temp::<u32>(marker)),Some(912));
        world.resource::<EguiContextResource>().0.data_mut(|data|data.insert_temp(marker,913_u32));
        assert_eq!(context.data(|data|data.get_temp::<u32>(marker)),Some(913));
        assert!(Arc::ptr_eq(&world.resource::<ApplicationShaderLibrary>().0,&gameplay_shaders));
        assert!(!Arc::ptr_eq(&world.resource::<ApplicationShaderLibrary>().0,&loading_shaders));
    }

    #[test]
    fn skipped_splash_moves_pending_context_and_finishes_without_started_or_loading_wait() {
        let mut app=App::new(); events(&mut app);
        let context=egui::Context::default();let marker=egui::Id::new("skip-context");
        context.data_mut(|data|data.insert_temp(marker,53_u32));
        app.insert_resource(PendingEguiContextResource(context.clone()));
        // No LoadingGameComplete, preparation manager, logo, or six-owner bundle
        // exists: the original skip path requires none of those objects.
        finish_skipped_splash(app.world_mut()).unwrap();
        assert!(!app.world().contains_resource::<PendingEguiContextResource>());
        assert_eq!(app.world().resource::<EguiContextResource>().0.data(|data|data.get_temp::<u32>(marker)),Some(53));
        app.world().resource::<EguiContextResource>().0.data_mut(|data|data.insert_temp(marker,54_u32));
        assert_eq!(context.data(|data|data.get_temp::<u32>(marker)),Some(54));
        assert_eq!(app.world().resource::<Events<StartupRevealFinished>>().len(),1);
        assert!(app.world().resource::<Events<StartupRevealStarted>>().is_empty());
    }

    #[test]
    fn started_is_published_after_all_six_moves_before_vermeer_and_egui_move() {
        let mut loading=App::new();six_owners(&mut loading);
        let context=egui::Context::default();
        loading.insert_resource(EguiContextResource(context));
        // Deliberately omit Vermeer: the real handoff exposes the exact native
        // mutation boundary rather than replacing failure with a ready marker.
        let mut gameplay=App::new();events(&mut gameplay);
        assert!(transfer_loading_resources(loading.world_mut(),gameplay.world_mut()).is_err());
        let from=loading.world();let to=gameplay.world();
        assert!(!from.contains_resource::<GameConfig>() && to.contains_resource::<GameConfig>());
        assert!(!from.contains_resource::<ActiveInputModeTracker>() && to.contains_resource::<ActiveInputModeTracker>());
        assert!(!from.contains_resource::<WindowSize>() && to.contains_resource::<WindowSize>());
        assert!(!from.contains_resource::<AvailableMonitors>() && to.contains_resource::<AvailableMonitors>());
        assert!(!from.contains_resource::<FrameRenderConfig>() && to.contains_resource::<FrameRenderConfig>());
        assert!(!from.contains_resource::<AdaptiveRenderState>() && to.contains_resource::<AdaptiveRenderState>());
        assert_eq!(to.resource::<Events<StartupRevealStarted>>().len(),1);
        assert!(from.contains_resource::<EguiContextResource>());
        assert!(!to.contains_resource::<EguiContextResource>());
    }
}
