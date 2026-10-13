// addr=0x14089dcf0 name=animation_easing
// signature=void __cdecl animation_easing(void)


void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::animation_easing(void)

{
  void *in_RCX;
  void *in_RDX;
  undefined4 in_R8D;
  undefined4 in_XMM3_Da;
  
  *(undefined4 *)((longlong)in_RDX + 0x48) = in_R8D;
  *(undefined4 *)((longlong)in_RDX + 0x4c) = in_XMM3_Da;
  memcpy(in_RCX,in_RDX,0x90);
  return;
}


