// addr=0x14089b280 name=assign_mouse_button
// signature=void __cdecl assign_mouse_button(void)


void __cdecl country_core::game_key_bindings::KeyBindings::assign_mouse_button(void)

{
  longlong in_RCX;
  byte in_DL;
  undefined2 in_R8W;
  undefined2 in_R9W;
  
  *(undefined2 *)(in_RCX + 0x1bc + (ulonglong)in_DL * 4) = in_R8W;
  *(undefined2 *)(in_RCX + 0x1be + (ulonglong)in_DL * 4) = in_R9W;
  return;
}


