// addr=0x14089dcc0 name=minimum_lifespan
// signature=void __cdecl minimum_lifespan(void)


void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::minimum_lifespan(void)

{
  void *in_RCX;
  void *in_RDX;
  undefined4 in_XMM2_Da;
  
  *(undefined4 *)((longlong)in_RDX + 0x58) = 1;
  *(undefined4 *)((longlong)in_RDX + 0x5c) = in_XMM2_Da;
  memcpy(in_RCX,in_RDX,0x90);
  return;
}


