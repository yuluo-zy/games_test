// address 0x14098f970
// name new_straight
// signature void __cdecl new_straight(void)

void __cdecl
country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_straight(void)

{
  int iVar1;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined4 in_XMM2_Da;
  undefined4 in_XMM3_Da;
  undefined4 in_stack_00000028;
  undefined4 in_stack_00000030;
  char in_stack_00000038;
  byte in_stack_00000040;
  
  iVar1 = (uint)in_stack_00000040 * 2 + 2;
  if (in_stack_00000038 == '\0') {
    iVar1 = (uint)in_stack_00000040 * 2 + 1;
  }
  *(undefined4 *)(in_RCX + 1) = *(undefined4 *)(in_RDX + 1);
  *in_RCX = *in_RDX;
  *(undefined4 *)((longlong)in_RCX + 0xc) = 0;
  *(undefined4 *)(in_RCX + 2) = in_XMM2_Da;
  *(undefined4 *)((longlong)in_RCX + 0x14) = in_XMM3_Da;
  *(undefined4 *)(in_RCX + 3) = in_stack_00000028;
  *(undefined4 *)((longlong)in_RCX + 0x1c) = in_stack_00000030;
  *(int *)(in_RCX + 4) = iVar1;
  *(undefined8 *)((longlong)in_RCX + 0x24) = 0x3dc28f5c00000001;
  *(undefined8 *)((longlong)in_RCX + 0x2c) = 0x100000000;
  *(undefined8 *)((longlong)in_RCX + 0x34) = 0;
  *(undefined4 *)((longlong)in_RCX + 0x3c) = 0;
  return;
}

