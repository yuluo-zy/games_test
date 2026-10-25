
void __cdecl country_core::resources::render::impl$1::from(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 *in_RCX;
  undefined8 *in_RDX;
  
  uVar6 = *(undefined4 *)((longlong)in_RDX + 4);
  uVar7 = *(undefined4 *)(in_RDX + 1);
  uVar8 = *(undefined4 *)((longlong)in_RDX + 0x14);
  uVar9 = *(undefined4 *)(in_RDX + 3);
  uVar1 = *(undefined4 *)(in_RDX + 2);
  uVar3 = *(undefined8 *)((longlong)in_RDX + 0x34);
  uVar5 = in_RDX[4];
  uVar2 = *(undefined4 *)(in_RDX + 6);
  uVar4 = in_RDX[5];
  *in_RCX = (int)*in_RDX;
  in_RCX[1] = uVar6;
  in_RCX[2] = uVar7;
  in_RCX[3] = uVar1;
  in_RCX[4] = uVar8;
  in_RCX[5] = uVar9;
  in_RCX[6] = (int)uVar5;
  in_RCX[7] = (int)((ulonglong)uVar5 >> 0x20);
  in_RCX[8] = (int)uVar4;
  in_RCX[9] = uVar2;
  in_RCX[10] = (int)uVar3;
  in_RCX[0xb] = (int)((ulonglong)uVar3 >> 0x20);
  return;
}

