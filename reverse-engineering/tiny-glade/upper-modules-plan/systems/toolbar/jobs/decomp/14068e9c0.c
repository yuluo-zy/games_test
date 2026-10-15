// 0x14068e9c0 size
// signature: void __cdecl size(void)

void __cdecl toolbar::mei::layout::LayoutStyle::size(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined8 *in_R8;
  
  uVar1 = in_R8[1];
  in_RDX[5] = *in_R8;
  in_RDX[6] = uVar1;
  in_RCX[8] = in_RDX[8];
  uVar1 = *in_RDX;
  uVar2 = in_RDX[1];
  uVar3 = in_RDX[3];
  in_RCX[2] = in_RDX[2];
  in_RCX[3] = uVar3;
  *in_RCX = uVar1;
  in_RCX[1] = uVar2;
  uVar1 = in_RDX[4];
  uVar2 = in_RDX[5];
  uVar3 = in_RDX[7];
  in_RCX[6] = in_RDX[6];
  in_RCX[7] = uVar3;
  in_RCX[4] = uVar1;
  in_RCX[5] = uVar2;
  return;
}

