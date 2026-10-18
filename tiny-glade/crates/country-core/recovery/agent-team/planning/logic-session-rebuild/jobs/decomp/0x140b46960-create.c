// 0x140b46960 create
// signature: void __cdecl create(void)


void __cdecl country_core::resources::session::NextSession::create(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined8 *in_R8;
  
  uVar1 = in_RDX[10];
  uVar2 = in_RDX[0xb];
  uVar3 = in_RDX[8];
  uVar4 = in_RDX[9];
  uVar5 = *in_RDX;
  uVar6 = in_RDX[1];
  uVar7 = in_RDX[2];
  uVar8 = in_RDX[3];
  uVar9 = in_RDX[4];
  uVar10 = in_RDX[5];
  uVar11 = in_RDX[6];
  uVar12 = in_RDX[7];
  uVar13 = *in_R8;
  uVar14 = in_R8[1];
  uVar15 = in_R8[3];
  in_RCX[0x11] = in_R8[2];
  in_RCX[0x12] = uVar15;
  in_RCX[0xf] = uVar13;
  in_RCX[0x10] = uVar14;
  in_RCX[0xd] = uVar1;
  in_RCX[0xe] = uVar2;
  in_RCX[0xb] = uVar3;
  in_RCX[0xc] = uVar4;
  in_RCX[9] = uVar11;
  in_RCX[10] = uVar12;
  in_RCX[7] = uVar9;
  in_RCX[8] = uVar10;
  in_RCX[5] = uVar7;
  in_RCX[6] = uVar8;
  in_RCX[3] = uVar5;
  in_RCX[4] = uVar6;
  *in_RCX = 0;
  in_RCX[1] = 8;
  in_RCX[2] = 0;
  *(undefined1 *)(in_RCX + 0x13) = 0;
  return;
}

