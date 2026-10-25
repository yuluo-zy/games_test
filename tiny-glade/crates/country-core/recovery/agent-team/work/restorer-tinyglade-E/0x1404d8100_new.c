
void __cdecl tiny_glade::game_loop::GameLoop::new(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 *in_RCX;
  void *in_RDX;
  undefined8 in_R8;
  undefined8 in_R9;
  undefined8 in_stack_00000028;
  undefined8 in_stack_00000030;
  undefined8 in_stack_00000038;
  undefined8 in_stack_00000040;
  undefined8 in_stack_00000048;
  void *in_stack_00000050;
  void *in_stack_00000058;
  undefined8 *in_stack_00000060;
  undefined8 in_stack_00000068;
  void *in_stack_00000070;
  
  memcpy(in_RCX + 0x13c,in_stack_00000070,0xa8);
  memcpy(in_RCX + 6,in_RDX,0x658);
  *in_RCX = in_R8;
  in_RCX[1] = in_R9;
  in_RCX[0x151] = 0x8000000000000000;
  in_RCX[2] = in_stack_00000028;
  in_RCX[3] = in_stack_00000030;
  in_RCX[4] = in_stack_00000038;
  in_RCX[5] = in_stack_00000040;
  in_RCX[0x21d] = in_stack_00000048;
  memcpy(in_RCX + 0xd1,in_stack_00000050,0xa8);
  memcpy(in_RCX + 0xe6,in_stack_00000058,0x2b0);
  uVar1 = in_stack_00000060[1];
  uVar2 = in_stack_00000060[2];
  uVar3 = in_stack_00000060[3];
  in_RCX[0x219] = *in_stack_00000060;
  in_RCX[0x21a] = uVar1;
  in_RCX[0x21b] = uVar2;
  in_RCX[0x21c] = uVar3;
  in_RCX[0x21e] = in_stack_00000068;
  *(undefined4 *)(in_RCX + 0x217) = 1000000000;
  *(undefined4 *)(in_RCX + 0x218) = 0;
  in_RCX[0x21f] = 0;
  in_RCX[0x220] = 0;
  *(undefined4 *)(in_RCX + 0x221) = 0;
  *(undefined2 *)((longlong)in_RCX + 0x1114) = 1;
  return;
}

