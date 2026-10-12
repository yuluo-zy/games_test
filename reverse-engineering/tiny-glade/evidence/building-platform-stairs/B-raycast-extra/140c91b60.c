
void __cdecl utils::IsoTransform::from_translation_rotation(void)

{
  undefined4 uVar1;
  undefined8 uVar2;
  undefined4 *in_RCX;
  undefined4 *in_RDX;
  undefined8 *in_R8;
  undefined4 uVar3;
  
  uVar3 = (undefined4)((ulonglong)*(undefined8 *)(in_RDX + 1) >> 0x20);
  uVar1 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = uVar1;
  in_RCX[2] = uVar3;
  in_RCX[3] = uVar3;
  uVar2 = in_R8[1];
  *(undefined8 *)(in_RCX + 4) = *in_R8;
  *(undefined8 *)(in_RCX + 6) = uVar2;
  return;
}

