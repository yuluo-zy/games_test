// signature: void __cdecl new(void)

void __cdecl country_core::resources::roofs::roof_shape::Roof::new(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  undefined8 *in_R8;
  undefined8 *in_R9;
  undefined4 in_stack_00000028;
  undefined4 in_stack_00000030;
  undefined1 in_stack_00000038;
  undefined8 *in_stack_00000040;
  
  *in_RCX = in_RDX;
  uVar1 = in_R8[1];
  uVar2 = *(undefined8 *)((longlong)in_R8 + 0xc);
  uVar3 = *(undefined8 *)((longlong)in_R8 + 0x14);
  *(undefined8 *)((longlong)in_RCX + 0x24) = *in_R8;
  *(undefined8 *)((longlong)in_RCX + 0x2c) = uVar1;
  in_RCX[6] = uVar2;
  in_RCX[7] = uVar3;
  uVar1 = in_R9[1];
  uVar2 = *(undefined8 *)((longlong)in_R9 + 0xc);
  uVar3 = *(undefined8 *)((longlong)in_R9 + 0x14);
  in_RCX[1] = *in_R9;
  in_RCX[2] = uVar1;
  *(undefined8 *)((longlong)in_RCX + 0x14) = uVar2;
  *(undefined8 *)((longlong)in_RCX + 0x1c) = uVar3;
  *(undefined4 *)((longlong)in_RCX + 0x4c) = in_stack_00000028;
  *(undefined4 *)(in_RCX + 10) = in_stack_00000030;
  *(undefined1 *)((longlong)in_RCX + 0x54) = in_stack_00000038;
  in_RCX[8] = *in_stack_00000040;
  *(undefined4 *)(in_RCX + 9) = *(undefined4 *)(in_stack_00000040 + 1);
  return;
}

