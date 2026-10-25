
/// Install the canonical ICC producer with this App's actual texture owner.
/// Loading and gameplay Apps use their own namespaces on the shared device.
pub fn install_runtime(app: &mut bevy_app::App, textures: std::sync::Arc<crate::render_host::TextureHost>) {
    app.insert_resource(country_core::systems::backend::icc::IccTextureDevice(textures));
    if !app.is_plugin_added::<country_core::systems::backend::icc::IccProfilePlugin>() {
        app.add_plugins(country_core::systems::backend::icc::IccProfilePlugin);
    }
}

/// Native initial/monitor-change dirty query, adapted to the OS main-thread
/// host. Re-query after an App handoff because texture namespaces are distinct.
#[derive(Default)]
pub struct MonitorIccTracker {
    initialized: bool,
    monitor: Option<winit::monitor::MonitorHandle>,
    app_owner: usize,
}
impl MonitorIccTracker {
    pub fn publish_if_changed(&mut self, window: &winit::window::Window, world: &mut bevy_ecs::world::World) {
        let owner = std::sync::Arc::as_ptr(&world.resource::<crate::run_game::ApplicationShaderLibrary>().0) as usize;
        let monitor = window.current_monitor();
        if self.initialized && self.monitor == monitor && self.app_owner == owner { return; }
        self.initialized = true;
        self.monitor = monitor;
        self.app_owner = owner;
        #[cfg(windows)]
        if let Some(monitor) = &self.monitor {
            match get_monitor_icc_profile(monitor) {
                Ok(path) => { world.send_event(country_core::systems::backend::icc::IccProfileChanged(path.into())); }
                // Native query failure warns and publishes no event. Existing
                // active profile survives; parsing a delivered bad path differs.
                Err(error) => log::warn!(target: "tiny_glade::event_loop", "{error}"),
            }
        }
    }
}
