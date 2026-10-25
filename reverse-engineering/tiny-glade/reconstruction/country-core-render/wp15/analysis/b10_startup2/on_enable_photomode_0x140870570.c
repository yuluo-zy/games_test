// addr=0x140870570 name=on_enable_photomode
// signature=void __cdecl on_enable_photomode(void)


void __cdecl country_core::startup::init_photomode_state::PhotomodeState::on_enable_photomode(void)

{
  undefined4 *in_RCX;
  undefined4 uVar1;
  
  if (_DEV_OVERRIDE_IS_DEMO_MODE == '\0') {
    uVar1 = 0x40400000;
  }
  else {
    uVar1 = 0x3f800000;
  }
  *in_RCX = uVar1;
  in_RCX[1] = 0xffffffff;
  *(undefined1 *)(in_RCX + 2) = 0;
  *(undefined1 *)((longlong)in_RCX + 10) = 0;
  *(undefined1 *)(in_RCX + 3) = 0;
  return;
}


