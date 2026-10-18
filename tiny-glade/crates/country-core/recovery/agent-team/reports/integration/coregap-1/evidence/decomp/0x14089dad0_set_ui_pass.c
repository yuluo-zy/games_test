// 0x14089dad0 set_ui_pass :: void __cdecl set_ui_pass(void)

void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::set_ui_pass(void)

{
  void *in_RCX;
  void *in_RDX;
  byte in_R8B;
  
  *(byte *)((longlong)in_RDX + 0x8a) = in_R8B ^ 1;
  memcpy(in_RCX,in_RDX,0x90);
  return;
}


