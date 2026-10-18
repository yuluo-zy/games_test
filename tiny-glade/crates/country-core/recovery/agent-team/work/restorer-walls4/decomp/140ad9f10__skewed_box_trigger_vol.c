// void __cdecl skewed_box_trigger_vol(void)

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
skewed_box_trigger_vol(void)

{
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  int iVar13;
  undefined8 *in_RCX;
  int iVar14;
  undefined8 *in_RDX;
  int iVar15;
  int iVar16;
  int iVar17;
  undefined8 *in_R8;
  int in_R9D;
  int iVar18;
  int iVar19;
  undefined4 uStack_68;
  undefined4 uStack_50;
  undefined4 uStack_38;
  
  uVar1 = *(undefined4 *)((longlong)in_R8 + 0x14);
  uStack_68 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_R8 + 0xc) >> 0x20);
  uVar5 = in_R8[3];
  uVar2 = *(undefined4 *)((longlong)in_R8 + 0x2c);
  uVar11 = in_R8[4];
  uStack_50 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_R8 + 0x24) >> 0x20);
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x14);
  uStack_38 = (undefined4)((ulonglong)*(undefined8 *)((longlong)in_RDX + 0xc) >> 0x20);
  uVar6 = in_RDX[3];
  uVar7 = *(undefined8 *)((longlong)in_RDX + 0x24);
  uVar4 = *(undefined4 *)((longlong)in_RDX + 0x2c);
  uVar8 = *in_R8;
  uVar10 = in_R8[1];
  uVar9 = *in_RDX;
  uVar12 = in_RDX[1];
  iVar18 = in_R9D + 1;
  iVar14 = in_R9D + 2;
  iVar17 = in_R9D + 3;
  iVar13 = in_R9D + 4;
  iVar15 = in_R9D + 5;
  iVar16 = in_R9D + 6;
  iVar19 = in_R9D + 7;
  *(undefined4 *)(in_RCX + 10) = *(undefined4 *)(in_RDX + 4);
  *(undefined8 *)((longlong)in_RCX + 0x54) = uVar7;
  *(undefined4 *)((longlong)in_RCX + 0x5c) = uVar4;
  in_RCX[8] = CONCAT44(uVar3,uStack_38);
  in_RCX[9] = uVar6;
  in_RCX[6] = uVar9;
  in_RCX[7] = uVar12;
  in_RCX[4] = uVar11;
  in_RCX[5] = CONCAT44(uVar2,uStack_50);
  in_RCX[2] = CONCAT44(uVar1,uStack_68);
  in_RCX[3] = uVar5;
  *in_RCX = uVar8;
  in_RCX[1] = uVar10;
  *(int *)(in_RCX + 0xc) = in_R9D;
  *(int *)((longlong)in_RCX + 100) = iVar18;
  *(int *)(in_RCX + 0xd) = iVar14;
  *(int *)((longlong)in_RCX + 0x6c) = iVar14;
  *(int *)(in_RCX + 0xe) = iVar17;
  *(int *)((longlong)in_RCX + 0x74) = in_R9D;
  *(int *)(in_RCX + 0xf) = iVar13;
  *(int *)((longlong)in_RCX + 0x7c) = iVar15;
  *(int *)(in_RCX + 0x10) = iVar16;
  *(int *)((longlong)in_RCX + 0x84) = iVar16;
  *(int *)(in_RCX + 0x11) = iVar19;
  *(int *)((longlong)in_RCX + 0x8c) = iVar13;
  *(int *)(in_RCX + 0x12) = iVar17;
  *(int *)((longlong)in_RCX + 0x94) = iVar14;
  *(int *)(in_RCX + 0x13) = iVar16;
  *(int *)((longlong)in_RCX + 0x9c) = iVar16;
  *(int *)(in_RCX + 0x14) = iVar19;
  *(int *)((longlong)in_RCX + 0xa4) = iVar17;
  *(int *)(in_RCX + 0x15) = iVar18;
  *(int *)((longlong)in_RCX + 0xac) = in_R9D;
  *(int *)(in_RCX + 0x16) = iVar13;
  *(int *)((longlong)in_RCX + 0xb4) = iVar13;
  *(int *)(in_RCX + 0x17) = iVar15;
  *(int *)((longlong)in_RCX + 0xbc) = iVar18;
  *(int *)(in_RCX + 0x18) = iVar14;
  *(int *)((longlong)in_RCX + 0xc4) = iVar18;
  *(int *)(in_RCX + 0x19) = iVar15;
  *(int *)((longlong)in_RCX + 0xcc) = iVar15;
  *(int *)(in_RCX + 0x1a) = iVar16;
  *(int *)((longlong)in_RCX + 0xd4) = iVar14;
  *(int *)(in_RCX + 0x1b) = in_R9D;
  *(int *)((longlong)in_RCX + 0xdc) = iVar17;
  *(int *)(in_RCX + 0x1c) = iVar19;
  *(int *)((longlong)in_RCX + 0xe4) = iVar19;
  *(int *)(in_RCX + 0x1d) = iVar13;
  *(int *)((longlong)in_RCX + 0xec) = in_R9D;
  return;
}

