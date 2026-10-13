// addr=0x14089dc60 name=scale
// signature=void __cdecl scale(void)


void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::scale(void)

{
  void *in_RCX;
  void *in_RDX;
  undefined4 in_XMM2_Da;
  
  *(undefined4 *)((longlong)in_RDX + 0x50) = 1;
  *(undefined4 *)((longlong)in_RDX + 0x54) = in_XMM2_Da;
  memcpy(in_RCX,in_RDX,0x90);
  return;
}


