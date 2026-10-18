
void __cdecl
bevy_app::plugin::sealed::impl$9::
add_to_app<bevy_app::plugin::sealed::PluginMarker,country_core::startup::MinimalStartupPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::gui::settings::SettingsMenuPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::logo_animation::AnimatedLogo,bevy_app::plugin::sealed::PluginMarker,country_core::resources::loading_screen::LoadingScreen,bevy_app::plugin::sealed::PluginMarker,country_core::systems::render_loop::vermeer::VermeerPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::mei::core::MeiPlugin,bevy_app::plugin::sealed::PluginMarker,toolbar::menu::SplashScreenPlugin>
          (void)

{
  undefined8 local_70;
  
  app::App::add_boxed_plugin();
  if (!SBORROW8(0,local_70)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  app::App::add_boxed_plugin();
  if (!SBORROW8(0,local_70)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  app::App::add_boxed_plugin();
  if (!SBORROW8(0,local_70)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  app::App::add_boxed_plugin();
  if (SBORROW8(0,local_70)) {
    sealed::impl$0::add_to_app<country_core::systems::render_loop::vermeer::VermeerPlugin>();
    app::App::add_boxed_plugin();
    if (!SBORROW8(0,local_70)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    app::App::add_boxed_plugin();
    if (SBORROW8(0,local_70)) {
      return;
    }
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

