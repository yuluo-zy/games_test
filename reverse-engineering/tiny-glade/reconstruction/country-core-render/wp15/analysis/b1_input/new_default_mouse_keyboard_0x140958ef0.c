// addr=0x140958ef0 name=new_default_mouse_keyboard
// signature=void __cdecl new_default_mouse_keyboard(void)


void __cdecl country_core::input::ActiveInputModeTracker::new_default_mouse_keyboard(void)

{
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 *in_RCX;
  undefined4 in_EDX;
  undefined4 uVar1;
  
  std::time::Instant::now();
  uVar1 = in_EDX;
  std::time::impl$1::add();
  *in_RCX = extraout_RAX;
  *(undefined4 *)(in_RCX + 1) = in_EDX;
  in_RCX[2] = extraout_RAX_00;
  *(undefined4 *)(in_RCX + 3) = uVar1;
  return;
}


