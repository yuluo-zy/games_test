// deactivate  0x1408a8140
// signature: void __cdecl deactivate(void)
// callees: [{"name": "drop", "address": "140be24b0"}, {"name": "finish<country_core::systems::main_camera::MainCameraPlugin>", "address": "14003bb80"}]


void __cdecl country_core::systems::render_loop::vermeer::VermeerState::deactivate(void)

{
  longlong in_RCX;
  
  if ((*(int *)(in_RCX + 0x48) != 2) && (*(char *)(in_RCX + 0x3e) != '\a')) {
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  *(undefined4 *)(in_RCX + 0x48) = 2;
  return;
}

