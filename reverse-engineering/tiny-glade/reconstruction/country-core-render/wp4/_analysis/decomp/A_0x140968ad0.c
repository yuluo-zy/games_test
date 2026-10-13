// ADDRESS: 0x140968ad0
// SIZE: 26
// SIGNATURE: void __cdecl resolution(void)

void __cdecl country_core::render::render_config::FullscreenConfig::resolution(void)

{
  int iVar1;
  uint *in_RCX;
  int *in_RDX;
  
  iVar1 = *in_RDX;
  if (iVar1 == 1) {
    *(undefined8 *)(in_RCX + 1) = *(undefined8 *)(in_RDX + 1);
  }
  *in_RCX = (uint)(iVar1 == 1);
  return;
}

