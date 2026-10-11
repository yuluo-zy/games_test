
void __cdecl country_core::resources::render::Affine3Packed::from_mat3_translation(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined4 *in_R8;
  
  uVar4 = in_RDX[1];
  uVar1 = *in_R8;
  uVar2 = *(undefined4 *)(in_RDX + 4);
  uVar3 = *(undefined8 *)(in_R8 + 1);
  *in_RCX = *in_RDX;
  in_RCX[1] = uVar4;
  uVar4 = in_RDX[3];
  in_RCX[2] = in_RDX[2];
  in_RCX[3] = uVar4;
  *(undefined4 *)(in_RCX + 4) = uVar2;
  *(undefined4 *)((longlong)in_RCX + 0x24) = uVar1;
  *(int *)(in_RCX + 5) = (int)uVar3;
  *(int *)((longlong)in_RCX + 0x2c) = (int)((ulonglong)uVar3 >> 0x20);
  return;
}

