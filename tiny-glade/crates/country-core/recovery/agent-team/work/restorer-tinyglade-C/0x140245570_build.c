// build @ 0x140245570
// sig: void __cdecl build(void)
// size: 189


void __cdecl tiny_glade::gui::settings::impl$0::build(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong uVar1;
  
  bevy_app::app::App::insert_resource<tiny_glade::gui::settings::LastAppliedVideoSettings>();
  bevy_app::app::App::insert_resource<country_core::resources::window::AvailableMonitors>();
  uVar1 = 0x242fe79a6b1f09d3;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_00 != 0) && (uVar1 < *(ulonglong *)(extraout_RAX + 0x218))) &&
      (uVar1 = *(ulonglong *)(*(longlong *)(extraout_RAX + 0x210) + uVar1 * 8), uVar1 != 0)) &&
     (*(longlong *)(*(longlong *)(extraout_RAX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)) {
    return;
  }
  bevy_ecs::event::registry::EventRegistry::
  register_event<tiny_glade::app_config::ApplyVideoSettingsCmd>();
  return;
}

