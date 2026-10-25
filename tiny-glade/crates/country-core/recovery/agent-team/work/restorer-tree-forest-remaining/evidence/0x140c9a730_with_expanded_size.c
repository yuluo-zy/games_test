// with_expanded_size 0x140c9a730 size=25
// signature: void __cdecl with_expanded_size(void)


void __cdecl utils::geometry::rectangle::Rectangle2d::with_expanded_size(void)

{
  undefined8 uVar1;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  float in_XMM2_Da;
  float in_XMM3_Da;
  
  in_RCX[2] = CONCAT44(in_XMM3_Da + (float)((ulonglong)in_RDX[2] >> 0x20),
                       in_XMM2_Da + (float)in_RDX[2]);
  uVar1 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = uVar1;
  return;
}

