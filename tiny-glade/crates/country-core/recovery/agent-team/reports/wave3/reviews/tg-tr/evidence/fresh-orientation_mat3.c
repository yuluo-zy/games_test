
void __cdecl utils::geometry::rectangle::Rectangle2d::orientation_mat3(void)

{
  undefined4 uVar1;
  uint uVar2;
  undefined4 *in_RCX;
  undefined4 *in_RDX;
  
  uVar1 = *in_RDX;
  uVar2 = in_RDX[1];
  *in_RCX = uVar1;
  in_RCX[1] = 0;
  in_RCX[2] = uVar2;
  in_RCX[6] = uVar2 ^ 0x80000000;
  in_RCX[7] = 0;
  in_RCX[8] = uVar1;
  *(undefined8 *)(in_RCX + 3) = 0x3f80000000000000;
  in_RCX[5] = 0;
  return;
}

