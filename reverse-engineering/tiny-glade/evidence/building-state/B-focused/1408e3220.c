
void __cdecl country_core::resources::roofs::roof_shape::Roof::pivot_3d(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 *in_RCX;
  longlong in_RDX;
  
  uVar1 = *(undefined4 *)(in_RDX + 0x10 + (ulonglong)*(uint *)(in_RDX + 8) * 8);
  uVar2 = *(undefined4 *)(in_RDX + 0x4c);
  *in_RCX = *(undefined4 *)(in_RDX + 0xc + (ulonglong)*(uint *)(in_RDX + 8) * 8);
  in_RCX[1] = uVar2;
  in_RCX[2] = uVar1;
  return;
}

