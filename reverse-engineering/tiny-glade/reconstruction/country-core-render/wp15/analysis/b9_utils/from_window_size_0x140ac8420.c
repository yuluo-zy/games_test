// addr=0x140ac8420 name=from_window_size
// signature=void __cdecl from_window_size(void)


void __cdecl
country_core::utils::project_cursor_onto_ray::ProportionalScreenSpace::from_window_size(void)

{
  uint uVar1;
  uint uVar2;
  float *in_RCX;
  uint *in_RDX;
  
  uVar1 = *in_RDX;
  uVar2 = in_RDX[1];
  *in_RCX = (float)uVar1;
  in_RCX[1] = (float)uVar2;
  in_RCX[2] = (float)uVar1 / (float)uVar2;
  in_RCX[3] = 1.0;
  return;
}


