// addr=0x14089dc00 name=position
// signature=void __cdecl position(void)


void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::position(void)

{
  void *in_RCX;
  void *in_RDX;
  undefined8 *in_R8;
  
  *(undefined4 *)((longlong)in_RDX + 0x84) = *(undefined4 *)(in_R8 + 1);
  *(undefined8 *)((longlong)in_RDX + 0x7c) = *in_R8;
  memcpy(in_RCX,in_RDX,0x90);
  return;
}


