// 0x14068e8b0 gap
// signature: void __cdecl gap(void)

void __cdecl toolbar::mei::layout::LayoutStyle::gap(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined4 in_R8D;
  undefined4 in_XMM3_Da;
  
  *(undefined4 *)(in_RDX + 4) = in_R8D;
  *(undefined4 *)((longlong)in_RDX + 0x24) = in_XMM3_Da;
  in_RCX[8] = in_RDX[8];
  uVar1 = *in_RDX;
  uVar2 = in_RDX[1];
  uVar3 = in_RDX[2];
  uVar4 = in_RDX[3];
  uVar5 = in_RDX[7];
  in_RCX[6] = in_RDX[6];
  in_RCX[7] = uVar5;
  in_RCX[2] = uVar3;
  in_RCX[3] = uVar4;
  *in_RCX = uVar1;
  in_RCX[1] = uVar2;
  *(undefined4 *)(in_RCX + 4) = *(undefined4 *)(in_RDX + 4);
  *(undefined8 *)((longlong)in_RCX + 0x24) = *(undefined8 *)((longlong)in_RDX + 0x24);
  *(undefined4 *)((longlong)in_RCX + 0x2c) = *(undefined4 *)((longlong)in_RDX + 0x2c);
  return;
}

