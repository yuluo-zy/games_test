// activate  0x1408a8060
// signature: void __cdecl activate(void)
// callees: [{"name": "drop", "address": "140be24b0"}, {"name": "finish<country_core::systems::main_camera::MainCameraPlugin>", "address": "14003bb80"}]


void __cdecl country_core::systems::render_loop::vermeer::VermeerState::activate(void)

{
  undefined8 *in_RCX;
  undefined1 in_DL;
  undefined1 in_R8B;
  
  if ((*(int *)(in_RCX + 9) != 2) && (*(char *)((longlong)in_RCX + 0x3e) != '\a')) {
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  *in_RCX = 0;
  *(undefined1 *)((longlong)in_RCX + 0x3e) = 7;
  in_RCX[8] = 0;
  *(undefined4 *)(in_RCX + 9) = 0;
  *(undefined1 *)((longlong)in_RCX + 0x54) = in_DL;
  *(undefined1 *)((longlong)in_RCX + 0x55) = in_R8B;
  return;
}

