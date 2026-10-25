
void __cdecl country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle(void)

{
  undefined8 uVar1;
  undefined8 *in_RCX;
  byte *in_RDX;
  
  if ((*in_RDX & 1) != 0) {
    uVar1 = *(undefined8 *)(in_RDX + 0xc);
    *in_RCX = *(undefined8 *)(in_RDX + 4);
    in_RCX[1] = uVar1;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}

