// addr=0x14089b2a0 name=assign_key
// signature=void __cdecl assign_key(void)


// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x00014089d203) overlaps instruction at (ram,0x00014089d201)
// 
// WARNING: Type propagation algorithm not settling

void __cdecl country_core::game_key_bindings::KeyBindings::assign_key(void)

{
  code *******_Size;
  code *******pppppppcVar1;
  code *******pppppppcVar2;
  code cVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  code cVar6;
  ushort uVar7;
  uint uVar8;
  int iVar9;
  undefined4 uVar10;
  code *pcVar11;
  undefined8 *puVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  longlong lVar15;
  ushort uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  byte bVar19;
  char cVar20;
  undefined7 uVar26;
  undefined2 *extraout_RAX;
  char *extraout_RAX_00;
  code *******extraout_RAX_01;
  code *******extraout_RAX_02;
  undefined4 *puVar21;
  char *pcVar22;
  int *piVar23;
  code *******extraout_RAX_03;
  void *extraout_RAX_04;
  undefined4 *extraout_RAX_05;
  code *******extraout_RAX_06;
  code *******extraout_RAX_07;
  code *******_Src;
  undefined8 extraout_RAX_08;
  undefined8 uVar24;
  undefined8 extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong lVar25;
  byte bVar27;
  byte bVar28;
  code *******UNRECOVERED_JUMPTABLE;
  byte in_DL;
  byte bVar29;
  code *******_Src_00;
  byte bVar31;
  undefined8 **ppuVar32;
  undefined1 *puVar33;
  undefined1 *puVar34;
  undefined8 *puVar35;
  uint uVar36;
  uint *puVar37;
  longlong lVar38;
  uint *in_R8;
  code *******pppppppcVar39;
  ulonglong uVar40;
  undefined *puVar41;
  longlong lVar42;
  void *_Dst;
  longlong *plVar43;
  longlong unaff_GS_OFFSET;
  bool bVar44;
  uint in_XMM0_Da;
  uint in_XMM0_Db;
  uint in_XMM0_Dc;
  uint in_XMM0_Dd;
  undefined4 in_XMM1_Da;
  undefined4 in_XMM1_Db;
  undefined4 in_XMM1_Dc;
  undefined4 in_XMM1_Dd;
  undefined4 in_XMM2_Da;
  undefined4 in_XMM2_Db;
  undefined4 in_XMM2_Dc;
  undefined4 in_XMM2_Dd;
  undefined4 uVar45;
  undefined8 *apuStack_138 [2];
  undefined1 auStack_128 [8];
  code *******local_120;
  code *******local_118;
  code *******local_110;
  code *******local_108;
  code *******local_100;
  code *******local_f8;
  code *******local_f0;
  undefined8 local_e8;
  code *******local_e0;
  code *******local_d8;
  code *******local_d0;
  code *******local_c8;
  code *******local_c0;
  code *******local_b8;
  code *******local_b0;
  code *******local_a8;
  code *******local_a0;
  code *******local_98;
  code *******local_90;
  code *******local_88;
  code *******local_80;
  code *******local_78;
  code *******local_70;
  code *******local_68;
  code *******local_60;
  code *******local_58;
  code *******local_50;
  undefined2 uVar30;
  
  puVar34 = auStack_128;
  puVar33 = auStack_128;
  local_50 = UNRECOVERED_JUMPTABLE + 0x2a;
  uVar36 = *in_R8;
  puVar37 = (uint *)(ulonglong)uVar36;
  uVar7 = (ushort)in_R8[1];
  uVar40 = (ulonglong)uVar7;
  uVar30 = *(undefined2 *)((longlong)in_R8 + 6);
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x2a);
  if (uVar36 == 0xc3 || uVar8 == 0xc3) {
    if (uVar8 == 0xc3 && uVar36 == 0xc3) {
LAB_14089b313:
      *(uint *)local_50 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b313;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x154) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089be23. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a730 + *(int *)(&DAT_142a9a730 + uVar40 * 4)))();
      return;
    }
  }
  local_58 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x15c);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x15c);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b35d:
      *(uint *)local_58 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b35d;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x2c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089be56. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a744 + *(int *)(&DAT_142a9a744 + uVar40 * 4)))();
      return;
    }
  }
  local_60 = UNRECOVERED_JUMPTABLE + 0x2d;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x2d);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b3a7:
      *(uint *)local_60 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b3a7;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x16c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089be89. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a758 + *(int *)(&DAT_142a9a758 + uVar40 * 4)))();
      return;
    }
  }
  local_68 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x174);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x174);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b3f0:
      *(uint *)local_68 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b3f0;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x2f) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089bebc. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a76c + *(int *)(&DAT_142a9a76c + uVar40 * 4)))();
      return;
    }
  }
  local_70 = UNRECOVERED_JUMPTABLE + 0x30;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x30);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b439:
      *(uint *)local_70 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b439;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x184) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089beef. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a780 + *(int *)(&DAT_142a9a780 + uVar40 * 4)))();
      return;
    }
  }
  local_78 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x18c);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x18c);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b483:
      *(uint *)local_78 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b483;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x32) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089bf22. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a794 + *(int *)(&DAT_142a9a794 + uVar40 * 4)))();
      return;
    }
  }
  _Src = UNRECOVERED_JUMPTABLE + 0x33;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x33);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b4cd:
      *(uint *)_Src = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b4cd;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x19c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089bf55. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a7a8 + *(int *)(&DAT_142a9a7a8 + uVar40 * 4)))();
      return;
    }
  }
  local_98 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x1a4);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x1a4);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b528:
      *(uint *)local_98 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b528;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x35) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089bf88. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a7bc + *(int *)(&DAT_142a9a7bc + uVar40 * 4)))();
      return;
    }
  }
  local_90 = UNRECOVERED_JUMPTABLE + 6;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 6);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b57b:
      *(uint *)local_90 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b57b;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x34) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089bfb8. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a7d0 + *(int *)(&DAT_142a9a7d0 + uVar40 * 4)))();
      return;
    }
  }
  pppppppcVar39 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x3c);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x3c);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b5c6:
      *(uint *)pppppppcVar39 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b5c6;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 8) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089bfe5. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a7e4 + *(int *)(&DAT_142a9a7e4 + uVar40 * 4)))();
      return;
    }
  }
  local_a0 = UNRECOVERED_JUMPTABLE + 3;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 3);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b612:
      *(uint *)local_a0 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b612;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x1c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c012. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a7f8 + *(int *)(&DAT_142a9a7f8 + uVar40 * 4)))();
      return;
    }
  }
  local_a8 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x24);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x24);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b665:
      *(uint *)local_a8 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b665;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 5) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c03f. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a80c + *(int *)(&DAT_142a9a80c + uVar40 * 4)))();
      return;
    }
  }
  uVar8 = *(uint *)UNRECOVERED_JUMPTABLE;
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b6ab:
      *(uint *)UNRECOVERED_JUMPTABLE = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b6ab;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 4) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c06c. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a820 + *(int *)(&DAT_142a9a820 + uVar40 * 4)))();
      return;
    }
  }
  local_b0 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0xc);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0xc);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b6f3:
      *(uint *)local_b0 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b6f3;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 2) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c099. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a834 + *(int *)(&DAT_142a9a834 + uVar40 * 4)))();
      return;
    }
  }
  local_b8 = UNRECOVERED_JUMPTABLE + 9;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 9);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b740:
      *(uint *)local_b8 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b740;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x4c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c0c6. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a848 + *(int *)(&DAT_142a9a848 + uVar40 * 4)))();
      return;
    }
  }
  local_c0 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x54);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x54);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b78d:
      *(uint *)local_c0 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b78d;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0xb) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c0f3. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a85c + *(int *)(&DAT_142a9a85c + uVar40 * 4)))();
      return;
    }
  }
  local_c8 = UNRECOVERED_JUMPTABLE + 0xc;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0xc);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b7da:
      *(uint *)local_c8 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b7da;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 100) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c120. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a870 + *(int *)(&DAT_142a9a870 + uVar40 * 4)))();
      return;
    }
  }
  local_d0 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x6c);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x6c);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b827:
      *(uint *)local_d0 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b827;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0xe) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c14d. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a884 + *(int *)(&DAT_142a9a884 + uVar40 * 4)))();
      return;
    }
  }
  local_d8 = UNRECOVERED_JUMPTABLE + 0xf;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0xf);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b874:
      *(uint *)local_d8 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b874;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x7c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c17a. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a898 + *(int *)(&DAT_142a9a898 + uVar40 * 4)))();
      return;
    }
  }
  local_e0 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x84);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x84);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b8c7:
      *(uint *)local_e0 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b8c7;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x11) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c1ad. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a8ac + *(int *)(&DAT_142a9a8ac + uVar40 * 4)))();
      return;
    }
  }
  local_e8 = UNRECOVERED_JUMPTABLE + 0x12;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x12);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b91a:
      *(uint *)local_e8 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b91a;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x94) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c1e0. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a8c0 + *(int *)(&DAT_142a9a8c0 + uVar40 * 4)))();
      return;
    }
  }
  local_f0 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x9c);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x9c);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b96d:
      *(uint *)local_f0 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b96d;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x14) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c213. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a8d4 + *(int *)(&DAT_142a9a8d4 + uVar40 * 4)))();
      return;
    }
  }
  local_f8 = UNRECOVERED_JUMPTABLE + 0x15;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x15);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089b9c0:
      *(uint *)local_f8 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089b9c0;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0xac) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c246. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a8e8 + *(int *)(&DAT_142a9a8e8 + uVar40 * 4)))();
      return;
    }
  }
  local_100 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0xb4);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0xb4);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089ba13:
      *(uint *)local_100 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089ba13;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x17) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c279. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a8fc + *(int *)(&DAT_142a9a8fc + uVar40 * 4)))();
      return;
    }
  }
  local_108 = UNRECOVERED_JUMPTABLE + 0x18;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x18);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089ba66:
      *(uint *)local_108 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089ba66;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0xc4) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c2ac. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a910 + *(int *)(&DAT_142a9a910 + uVar40 * 4)))();
      return;
    }
  }
  local_110 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0xcc);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0xcc);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bab9:
      *(uint *)local_110 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bab9;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x1a) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c2df. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a924 + *(int *)(&DAT_142a9a924 + uVar40 * 4)))();
      return;
    }
  }
  local_118 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0xe4);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0xe4);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bb0c:
      *(uint *)local_118 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bb0c;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x1d) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c312. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a938 + *(int *)(&DAT_142a9a938 + uVar40 * 4)))();
      return;
    }
  }
  local_120 = UNRECOVERED_JUMPTABLE + 0x1e;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x1e);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bb67:
      *(uint *)local_120 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bb67;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0xf4) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c345. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a94c + *(int *)(&DAT_142a9a94c + uVar40 * 4)))();
      return;
    }
  }
  local_80 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0xfc);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0xfc);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bbbd:
      *(uint *)local_80 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bbbd;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x20) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c378. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a960 + *(int *)(&DAT_142a9a960 + uVar40 * 4)))();
      return;
    }
  }
  _Size = UNRECOVERED_JUMPTABLE + 0x21;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x21);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bc14:
      *(uint *)_Size = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bc14;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x10c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c3ab. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a974 + *(int *)(&DAT_142a9a974 + uVar40 * 4)))
                (UNRECOVERED_JUMPTABLE,&DAT_142a9a974 + *(int *)(&DAT_142a9a974 + uVar40 * 4));
      return;
    }
  }
  local_88 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x114);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x114);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bc6a:
      *(uint *)local_88 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bc6a;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x23) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c3de. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a988 + *(int *)(&DAT_142a9a988 + uVar40 * 4)))
                (UNRECOVERED_JUMPTABLE,&DAT_142a9a988 + *(int *)(&DAT_142a9a988 + uVar40 * 4));
      return;
    }
  }
  pppppppcVar1 = UNRECOVERED_JUMPTABLE + 0x24;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x24);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bcc1:
      *(uint *)pppppppcVar1 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bcc1;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x124) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c411. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a99c + *(int *)(&DAT_142a9a99c + uVar40 * 4)))
                (UNRECOVERED_JUMPTABLE,&DAT_142a9a99c + *(int *)(&DAT_142a9a99c + uVar40 * 4));
      return;
    }
  }
  _Src_00 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 300);
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 300);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bd27:
      *(uint *)_Src_00 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bd27;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x26) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c444. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a9b0 + *(int *)(&DAT_142a9a9b0 + uVar40 * 4)))();
      return;
    }
  }
  pppppppcVar2 = UNRECOVERED_JUMPTABLE + 0x27;
  uVar8 = *(uint *)(UNRECOVERED_JUMPTABLE + 0x27);
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    if (uVar36 == 0xc3 && uVar8 == 0xc3) {
LAB_14089bd80:
      *(uint *)pppppppcVar2 = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bd80;
    if (*(ushort *)((longlong)UNRECOVERED_JUMPTABLE + 0x13c) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c478. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a9c4 + *(int *)(&DAT_142a9a9c4 + uVar40 * 4)))
                (UNRECOVERED_JUMPTABLE,_Src_00,pppppppcVar39,
                 &DAT_142a9a9c4 + *(int *)(&DAT_142a9a9c4 + uVar40 * 4));
      return;
    }
  }
  uVar8 = *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x144);
  uVar16 = CONCAT11((char)((ushort)uVar30 >> 8),uVar8 == 0xc3 || uVar36 == 0xc3);
  puVar35 = (undefined8 *)(ulonglong)uVar16;
  if (uVar8 == 0xc3 || uVar36 == 0xc3) {
    uVar36 = uVar36 ^ 0xc3 | uVar8 ^ 0xc3;
    puVar37 = (uint *)(ulonglong)uVar36;
    if (uVar36 == 0) {
LAB_14089bdd6:
      *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x144) = 0xc3;
    }
  }
  else if (uVar8 == uVar36) {
    if (uVar36 != 0) goto LAB_14089bdd6;
    if (*(ushort *)(UNRECOVERED_JUMPTABLE + 0x29) == uVar7) {
                    // WARNING: Could not recover jumptable at 0x00014089c4ac. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142a9a9d8 + *(int *)(&DAT_142a9a9d8 + uVar40 * 4)))
                (UNRECOVERED_JUMPTABLE,_Src_00,pppppppcVar39,
                 &DAT_142a9a9d8 + *(int *)(&DAT_142a9a9d8 + uVar40 * 4));
      return;
    }
  }
  uVar40 = (ulonglong)*(int *)(&DAT_142a9a9ec + (ulonglong)in_DL * 4);
  bVar44 = 0xfffffffebd565613 < uVar40;
  puVar41 = &DAT_142a9a9ec + uVar40;
  bVar31 = (byte)((ulonglong)pppppppcVar1 >> 8);
  bVar19 = (byte)_Src;
  uVar36 = (uint)_Src;
  uVar26 = (undefined7)((ulonglong)_Src >> 8);
  bVar29 = (byte)_Src_00;
  uVar30 = SUB82(_Src_00,0);
  bVar27 = (byte)UNRECOVERED_JUMPTABLE;
  bVar28 = (byte)((ulonglong)UNRECOVERED_JUMPTABLE >> 8);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(in_DL) {
  case 1:
    local_90 = pppppppcVar39;
    break;
  case 2:
    local_90 = local_a0;
    break;
  case 3:
    local_90 = local_a8;
    break;
  case 4:
    goto code_r0x00014089c5ab;
  case 5:
    local_90 = local_b0;
    break;
  case 6:
    local_90 = local_b8;
    break;
  case 7:
    local_90 = local_c0;
    break;
  case 8:
    local_90 = local_c8;
    break;
  case 9:
    local_90 = local_d0;
    break;
  case 10:
    local_90 = local_d8;
    break;
  case 0xb:
    local_90 = local_e0;
    break;
  case 0xc:
    local_90 = local_e8;
    break;
  case 0xd:
    local_90 = local_f0;
    break;
  case 0xe:
    local_90 = local_f8;
    break;
  case 0xf:
    local_90 = local_100;
    break;
  case 0x10:
    local_90 = local_108;
    break;
  case 0x11:
    local_90 = local_110;
    break;
  case 0x12:
    UNRECOVERED_JUMPTABLE = UNRECOVERED_JUMPTABLE + 0x1b;
    goto code_r0x00014089c5ab;
  case 0x13:
    local_90 = local_118;
    break;
  case 0x14:
    local_90 = local_120;
    break;
  case 0x15:
    local_90 = local_80;
    break;
  case 0x16:
    local_90 = _Size;
    break;
  case 0x17:
    local_90 = local_88;
    break;
  case 0x18:
    local_90 = pppppppcVar1;
    break;
  case 0x19:
    local_90 = _Src_00;
    break;
  case 0x1a:
    local_90 = pppppppcVar2;
    break;
  case 0x1b:
    local_90 = (code *******)((longlong)UNRECOVERED_JUMPTABLE + 0x144);
    break;
  case 0x1c:
    local_90 = local_50;
    break;
  case 0x1d:
    local_90 = local_58;
    break;
  case 0x1e:
    local_90 = local_60;
    break;
  case 0x1f:
    local_90 = local_68;
    break;
  case 0x20:
    local_90 = local_70;
    break;
  case 0x21:
    local_90 = local_78;
    break;
  case 0x22:
    local_90 = _Src;
    break;
  case 0x23:
    local_90 = local_98;
    break;
  case 0x24:
    UNRECOVERED_JUMPTABLE = UNRECOVERED_JUMPTABLE + 0x36;
code_r0x00014089c5ab:
    local_90 = UNRECOVERED_JUMPTABLE;
    break;
  case 0x25:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x26:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x27:
    pcVar11 = (code *)swi(3);
    (*pcVar11)();
    return;
  case 0x28:
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x12d) =
         (code)((char)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x12d) + bVar31);
    apuStack_138[1] = (undefined8 *)&UNK_14089c9a1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX != (undefined2 *)0x0) {
      *(undefined1 *)(extraout_RAX + 1) = 0x42;
      *extraout_RAX = 0x4d4c;
      puVar37[0] = 3;
      puVar37[1] = 0;
      *(undefined2 **)(puVar37 + 2) = extraout_RAX;
      puVar37[4] = 3;
      puVar37[5] = 0;
      return;
    }
    goto code_r0x00014089cb20;
  case 0x29:
    apuStack_138[1] = (undefined8 *)0x14089c92c;
    memcpy(UNRECOVERED_JUMPTABLE,_Src_00,(size_t)_Size);
    *(code ********)puVar37 = _Size;
    *(code ********)(puVar37 + 2) = pppppppcVar2;
    *(code ********)(puVar37 + 4) = _Size;
    puVar37[6] = 0;
    puVar37[7] = 0;
    return;
  case 0x2a:
    *(byte *)_Size = (byte)*(code *)_Size >> 4 | (char)*(code *)_Size << 4;
    LOCK();
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    UNLOCK();
    *(undefined4 *)
     CONCAT71(uVar26,bVar19 + *(char *)(CONCAT62((int6)((ulonglong)_Src_00 >> 0x10),
                                                 CONCAT11((char)((ulonglong)_Src_00 >> 8) + bVar19,
                                                          bVar29)) + 0x66)) = 0x6deb4d4d;
    apuStack_138[1] = (undefined8 *)&UNK_14089ca40;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 != (char *)0x0) {
      builtin_strncpy(extraout_RAX_00,"Back mouse button",0x11);
      puVar37[0] = 0x11;
      puVar37[1] = 0;
      *(char **)(puVar37 + 2) = extraout_RAX_00;
      puVar37[4] = 0x11;
      puVar37[5] = 0;
      return;
    }
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089cb51;
    alloc::raw_vec::handle_error();
  case 0x2b:
    *_Src = (code ******)((longlong)_Src + (longlong)*_Src);
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    local_d8 = (code *******)&local_f8;
    goto code_r0x00014089cae7;
  case 0x2c:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x2d:
    goto code_r0x00014089cd25;
  case 0x2e:
    *(byte *)(UNRECOVERED_JUMPTABLE + 0x2a) = (char)*(code *)(UNRECOVERED_JUMPTABLE + 0x2a) + bVar27
    ;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(byte *)(UNRECOVERED_JUMPTABLE + 0x900d15c) =
         (char)*(code *)(UNRECOVERED_JUMPTABLE + 0x900d15c) + bVar19;
  case 0x32:
    *(undefined **)(puVar37 + 6) = &UNK_142a99171;
    puVar37[8] = 0x12;
    puVar37[9] = 0;
    return;
  case 0x2f:
    *puVar37 = 0xb;
    goto code_r0x00014089cc6c;
  case 0x30:
    return;
  case 0x31:
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) =
         (code)((byte)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) | 6);
    puVar41 = &UNK_142a991c3;
    goto code_r0x00014089cec6;
  case 0x33:
    local_f8 = (code *******)
               anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_c8 = (code *******)&UNK_142a99098;
    goto code_r0x00014089cf9f;
  case 0x34:
    piVar23 = (int *)(CONCAT71(uVar26,bVar19 + bVar28) + -0x7b);
    *piVar23 = *piVar23 + -1;
    *(byte *)_Size = (byte)*(code *)_Size >> 4 | (char)*(code *)_Size << 4;
    puVar21 = (undefined4 *)CONCAT71(uVar26,bVar19 + bVar28 + 'p');
    *puVar21 = 0x55204955;
    puVar37[0] = 5;
    puVar37[1] = 0;
    *(undefined4 **)(puVar37 + 2) = puVar21;
    puVar37[4] = 5;
    puVar37[5] = 0;
    goto code_r0x00014089d0a1;
  case 0x35:
    if ((POPCOUNT((ulonglong)puVar41 & 0xff) & 1U) != 0) {
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x125) =
         (code)((byte)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x125) ^ bVar27);
    local_c8 = (code *******)(ulonglong)(uVar36 + 0x21fc1dc);
    goto code_r0x00014089cf9f;
  case 0x36:
    iRam00000000f9d1cc5a = iRam00000000f9d1cc5a + 1;
    (&UNK_142a9aa4e)[uVar40] = (&UNK_142a9aa4e)[uVar40] & bVar29;
    *(byte *)(UNRECOVERED_JUMPTABLE + 0x3c) = (byte)*(code *)(UNRECOVERED_JUMPTABLE + 0x3c) & bVar29
    ;
    *(uint *)_Src = (uint)UNRECOVERED_JUMPTABLE;
    *(undefined4 *)((longlong)UNRECOVERED_JUMPTABLE + 0x19f) = 0x76657250;
    goto code_r0x00014089cc6c;
  case 0x37:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x38:
    if (puVar41 == (undefined *)0x0) {
      *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) =
           (code)((byte)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) | 6);
      *(undefined **)(puVar37 + 6) = &UNK_142a991ed;
      puVar37[8] = 0x13;
      puVar37[9] = 0;
      return;
    }
    *(uint *)_Src = (uint)UNRECOVERED_JUMPTABLE;
  case 0x40:
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x151) =
         (code)((char)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x151) + bVar27);
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x99) =
         (code)((byte)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x99) & bVar29);
    puVar37[0] = 8;
    puVar37[1] = 0;
    *(code ********)(puVar37 + 2) = _Src;
    puVar37[4] = 8;
    puVar37[5] = 0;
    goto code_r0x00014089d0a1;
  case 0x39:
    *(uint *)_Src = in_XMM0_Da;
    *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x19c) = in_XMM0_Db;
    *(uint *)(UNRECOVERED_JUMPTABLE + 0x34) = in_XMM0_Dc;
    *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0x1a4) = in_XMM0_Dd;
    *(undefined8 *)((longlong)UNRECOVERED_JUMPTABLE + 0x1a6) = 0x73746f687370616e;
    puVar37[0] = 0x16;
    puVar37[1] = 0;
    *(code ********)(puVar37 + 2) = _Src;
    puVar37[4] = 0x16;
    puVar37[5] = 0;
    goto code_r0x00014089d0a1;
  case 0x3a:
    uVar40 = (ulonglong)local_e8 >> 0x20;
    local_e8 = (code *******)CONCAT44((int)uVar40,(int)_Src_00);
    goto code_r0x00014089cd25;
  case 0x3b:
    piVar23 = (int *)(CONCAT71(uVar26,bVar19 + bVar28) + -0x7b);
    *piVar23 = *piVar23 + -1;
    *(byte *)_Size = (byte)*(code *)_Size >> 4 | (char)*(code *)_Size << 4;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x3c:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x3d:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) =
         (code)((byte)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) | 6);
    *(undefined **)(puVar37 + 6) = &UNK_142a99183;
    puVar37[8] = 0x15;
    puVar37[9] = 0;
    return;
  case 0x3e:
    pcVar11 = (code *)((longlong)UNRECOVERED_JUMPTABLE + 0x15f);
    cVar6 = *pcVar11;
    cVar3 = *pcVar11;
    *pcVar11 = (code)((char)cVar3 + bVar27 + bVar44);
    *(char *)_Src =
         ((char)*(code *)_Src - (char)pppppppcVar39) -
         (CARRY1((byte)cVar6,bVar27) || CARRY1((char)cVar3 + bVar27,bVar44));
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) =
         (code)((char)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) + bVar27);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x3f:
    local_100 = (code *******)((ulonglong)&local_f0 & 0xffffffff);
    local_f8 = (code *******)
               anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_c8 = (code *******)&UNK_142a990c0;
    goto code_r0x00014089cf9f;
  case 0x41:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x42:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    apuStack_138[1] = (undefined8 *)&UNK_14089d0d1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_05 == (undefined4 *)0x0) goto code_r0x00014089d27e;
    *(undefined4 *)((longlong)extraout_RAX_05 + 3) = 0x7466654c;
    *extraout_RAX_05 = 0x4c204955;
    puVar21 = extraout_RAX_05;
    goto code_r0x00014089d115;
  case 0x43:
    piVar23 = (int *)(ulonglong)*puVar37;
    *piVar23 = *piVar23 + *puVar37;
    *(char *)((longlong)piVar23 + -0x73) = *(char *)((longlong)piVar23 + -0x73) + bVar27;
    _Src_00 = (code *******)&DAT_00000010;
    puVar37 = puVar37 + 1;
code_r0x00014089cd25:
    apuStack_138[1] = (undefined8 *)&UNK_14089cd2a;
    lang_lib::lookup();
    local_100 = (code *******)&local_f0;
    local_f8 = (code *******)
               anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_c8 = (code *******)&UNK_142a990e8;
    local_f0 = extraout_RAX_03;
    local_e8 = _Src_00;
code_r0x00014089cf9f:
    local_c0 = (code *******)0x2;
    local_b8 = (code *******)&local_100;
    local_b0 = (code *******)0x1;
    local_a8 = (code *******)0x0;
    apuStack_138[1] = (undefined8 *)&UNK_14089cfdb;
    alloc::fmt::format::format_inner();
    *(code ********)(puVar37 + 4) = local_d0;
    *(code ********)puVar37 = local_e0;
    *(code ********)(puVar37 + 2) = local_d8;
    goto code_r0x00014089d0a1;
  case 0x44:
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x15f) =
         (code)((byte)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x15f) | (byte)puVar41);
    cRam000000012989ccfc = cRam000000012989ccfc + (char)pppppppcVar39;
    pcVar22 = (char *)(ulonglong)uRam000000b848000003;
    cVar20 = (char)uRam000000b848000003;
    *pcVar22 = *pcVar22 + cVar20;
    *pcVar22 = *pcVar22 + cVar20;
    pcVar22[-0x77] = pcVar22[-0x77] | 6;
    puVar41 = &UNK_142a99165;
code_r0x00014089cec6:
    *(undefined **)(puVar37 + 6) = puVar41;
    puVar37[8] = 0xc;
    puVar37[9] = 0;
    return;
  case 0x45:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x46:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) =
         (code)((char)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x121) + bVar27);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x47:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    lVar42 = CONCAT71(uVar26,bVar19 + bVar28);
    *(uint *)((longlong)UNRECOVERED_JUMPTABLE + 0xf7) =
         *(int *)((longlong)UNRECOVERED_JUMPTABLE + 0xf7) + (uint)uVar16 +
         (uint)CARRY1(bVar19,bVar28);
    *(int *)(lVar42 + -0x7b) = *(int *)(lVar42 + -0x7b) + -1;
    *(byte *)_Size = (byte)*(code *)_Size >> 4 | (char)*(code *)_Size << 4;
    *(int *)(unaff_GS_OFFSET + lVar42) = *(int *)(unaff_GS_OFFSET + lVar42) + (int)lVar42;
    puVar21 = (undefined4 *)
              (ulonglong)
              (uint)((int)lVar42 +
                    *(int *)((longlong)UNRECOVERED_JUMPTABLE + (longlong)puVar35 * 2 + 0x17f));
    out((char)*puVar37,uVar30);
    *puVar21 = 0x44204955;
    puVar37 = (uint *)((longlong)puVar37 + 1);
code_r0x00014089d115:
    puVar37[0] = 7;
    puVar37[1] = 0;
    *(undefined4 **)(puVar37 + 2) = puVar21;
    puVar37[4] = 7;
    puVar37[5] = 0;
    goto code_r0x00014089d0a1;
  case 0x48:
    _Size = (code *******)((ulonglong)_Src_00 & 0xffffffff);
    goto joined_r0x00014089cc2e;
  case 0x49:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    return;
  case 0x4a:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x4b:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x4c:
    pcVar22 = (char *)(CONCAT71(uVar26,*(undefined1 *)((longlong)_Src * 2)) + -0x73);
    *pcVar22 = *pcVar22 + bVar27;
    _Src_00 = (code *******)&DAT_0000001d;
    apuStack_138[1] = (undefined8 *)&UNK_14089cc28;
    lang_lib::lookup();
    _Src = extraout_RAX_02;
    _Size = _Src_00;
    goto joined_r0x00014089cc2e;
  case 0x4d:
    cVar6 = *(code *)_Size;
    *(byte *)_Size = (char)*(code *)_Size + bVar27;
    if (CARRY1((byte)cVar6,bVar27)) goto code_r0x00014089cb20;
    apuStack_138[1] = (undefined8 *)&UNK_14089cba6;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (code *******)0x0) goto code_r0x00014089d25d;
    *extraout_RAX_01 = (code ******)0x4e20626154204955;
    *(undefined4 *)((longlong)extraout_RAX_01 + 7) = 0x7478654e;
    _Src = extraout_RAX_01;
code_r0x00014089cc6c:
    puVar37[0] = 0xb;
    puVar37[1] = 0;
    *(code ********)(puVar37 + 2) = _Src;
    puVar37[4] = 0xb;
    puVar37[5] = 0;
    goto code_r0x00014089d0a1;
  case 0x4e:
code_r0x00014089cae7:
    local_d0 = (code *******)0x1;
    local_c8 = (code *******)0x0;
    apuStack_138[1] = (undefined8 *)&UNK_14089cb06;
    alloc::fmt::format::format_inner();
    return;
  case 0x4f:
    uVar36 = uVar36 + *(uint *)_Src;
    *(char *)(ulonglong)uVar36 = *(char *)(ulonglong)uVar36 + (char)uVar36;
code_r0x00014089cb20:
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089cb25;
    alloc::raw_vec::handle_error();
  case 0x50:
    *(uint *)_Src = *(uint *)_Src | uVar36;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(code ********)(puVar37 + 2) = _Src;
    puVar37[4] = 9;
    puVar37[5] = 0;
    goto code_r0x00014089d0a1;
  case 0x51:
    *(uint *)_Src = *(uint *)_Src + uVar36;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
  case 0x5f:
code_r0x00014089d27e:
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089d283;
    alloc::raw_vec::handle_error();
  case 0x52:
code_r0x00014089d28a:
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089d299;
    alloc::raw_vec::handle_error();
  case 0x53:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    apuStack_138[1] = (undefined8 *)&UNK_14089d246;
    lang_lib::lookup();
    _Src = extraout_RAX_07;
    _Size = _Src_00;
    goto joined_r0x00014089cc2e;
  case 0x54:
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + -0x7f) =
         (code)((char)*(code *)((longlong)UNRECOVERED_JUMPTABLE + -0x7f) + bVar27);
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + (longlong)_Src) =
         *(code *)((longlong)UNRECOVERED_JUMPTABLE + (longlong)_Src);
    goto code_r0x00014089d476;
  case 0x55:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x56:
    *(byte *)((longlong)puVar35 + -0x72b3fdfd) = *(byte *)((longlong)puVar35 + -0x72b3fdfd) | bVar31
    ;
    goto code_r0x00014089d2db;
  case 0x57:
    goto code_r0x00014089d29f;
  case 0x58:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    goto code_r0x00014089d2ec;
  case 0x59:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x5a:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x136) =
         (code)((char)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x136) + bVar31);
    goto code_r0x00014089d2ec;
  case 0x5b:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x5c:
    _Src = (code *******)
           CONCAT71(uVar26,*(code *)((longlong)pppppppcVar1 + ((ulonglong)_Src & 0xff)));
joined_r0x00014089cc2e:
    if ((longlong)_Src_00 < 0) {
code_r0x00014089d191:
                    // WARNING: Subroutine does not return
      apuStack_138[1] = (undefined8 *)&UNK_14089d1a3;
      alloc::raw_vec::handle_error();
    }
    if (_Src_00 == (code *******)0x0) {
      _Dst = (void *)0x1;
    }
    else {
      apuStack_138[1] = (undefined8 *)&UNK_14089d074;
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_04;
      if (extraout_RAX_04 == (void *)0x0) goto code_r0x00014089d191;
    }
    apuStack_138[1] = (undefined8 *)&UNK_14089d096;
    memcpy(_Dst,_Src,(size_t)_Size);
    *(code ********)puVar37 = _Size;
    *(void **)(puVar37 + 2) = _Dst;
    *(code ********)(puVar37 + 4) = _Size;
    goto code_r0x00014089d0a1;
  case 0x5d:
    goto code_r0x00014089d2cc;
  case 0x5e:
    if (bVar44) goto code_r0x00014089d28a;
    goto code_r0x00014089d21d;
  case 0x60:
    out(*puVar37,uVar30);
    out((char)puVar37[1],uVar30);
    *(uint *)(UNRECOVERED_JUMPTABLE + 0x34) = 0xc7486d72;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x61:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    goto code_r0x00014089d29f;
  case 0x62:
    goto code_r0x00014089d2db;
  case 99:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 100:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x65:
code_r0x00014089d2cc:
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089d2db;
    alloc::raw_vec::handle_error();
  case 0x66:
    ppuVar32 = apuStack_138 + 1;
    cVar20 = '\t';
    puVar12 = puVar35;
    do {
      puVar12 = puVar12 + -1;
      ppuVar32 = ppuVar32 + -1;
      *ppuVar32 = (undefined8 *)*puVar12;
      cVar20 = cVar20 + -1;
    } while ('\0' < cVar20);
    UNRECOVERED_JUMPTABLE =
         (code *******)
         (&DAT_142a9ab2c + *(int *)(&DAT_142a9ab2c + (longlong)UNRECOVERED_JUMPTABLE * 4));
    apuStack_138[1] = puVar35;
  case 0x67:
                    // WARNING: Could not recover jumptable at 0x00014089d314. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)UNRECOVERED_JUMPTABLE)();
    return;
  case 0x68:
    if (!bVar44 && puVar41 != (undefined *)0x0) goto code_r0x00014089d27e;
    goto code_r0x00014089d228;
  case 0x69:
code_r0x00014089d25d:
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089d26d;
    alloc::raw_vec::handle_error();
  case 0x6a:
    *(uint *)_Src = *(uint *)_Src + uVar36;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x6b:
    goto code_r0x00014089d1f7;
  case 0x6c:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    pppppppcVar39 = UNRECOVERED_JUMPTABLE + 0x3c;
  case 0x6d:
code_r0x00014089d476:
    *(uint *)(UNRECOVERED_JUMPTABLE + 0x34) = *(uint *)(pppppppcVar39 + 1);
    *_Src = *pppppppcVar39;
    return;
  case 0x6e:
    pcVar11 = (code *)swi(3);
    (*pcVar11)(bVar27 + (char)((ulonglong)_Src >> 8));
    return;
  case 0x6f:
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x13f) =
         (code)((char)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x13f) + bVar27);
code_r0x00014089d29f:
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089d2af;
    alloc::raw_vec::handle_error();
  case 0x70:
    pcVar11 = (code *)swi(3);
    (*pcVar11)();
    return;
  case 0x71:
    *(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x15f) =
         (code)((byte)*(code *)((longlong)UNRECOVERED_JUMPTABLE + 0x15f) | bVar27);
    *(byte *)_Src_00 = (char)*(code *)_Src_00 + (byte)puVar41;
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    cVar6 = (code)in(uVar30);
    *(code *)_Size = cVar6;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x72:
    *(byte *)_Size = (byte)*(code *)_Size >> 4 | (char)*(code *)_Size << 4;
    *(uint *)_Src = *(uint *)_Src << 1 | (uint)((int)*(uint *)_Src < 0);
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    goto code_r0x00014089d20a;
  case 0x73:
    puVar37[4] = 9;
code_r0x00014089d0a1:
    puVar37[6] = 0;
    puVar37[7] = 0;
    return;
  case 0x74:
code_r0x00014089d1f7:
    apuStack_138[1] = (undefined8 *)&UNK_14089d201;
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Src = extraout_RAX_06;
    if (extraout_RAX_06 != (code *******)0x0) {
code_r0x00014089d20a:
      *_Src = (code ******)0x69666e6f43204955;
      *(undefined2 *)(_Src + 1) = 0x6d72;
code_r0x00014089d21d:
      puVar37[0] = 10;
      puVar37[1] = 0;
      *(code ********)(puVar37 + 2) = _Src;
code_r0x00014089d228:
      puVar37[4] = 10;
      puVar37[5] = 0;
      goto code_r0x00014089d0a1;
    }
code_r0x00014089d2db:
code_r0x00014089d2ec:
                    // WARNING: Subroutine does not return
    apuStack_138[1] = (undefined8 *)&UNK_14089d2f1;
    alloc::raw_vec::handle_error();
  case 0x75:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x76:
    *(byte *)(puVar35 + -7) = *(char *)(puVar35 + -7) + bVar29 + bVar44;
    puVar35[0x1e] = puVar35[-5];
    puVar35[0x1f] = puVar35[-4];
    *(undefined4 *)(puVar35 + 0x1c) = in_XMM2_Da;
    *(undefined4 *)((longlong)puVar35 + 0xe4) = in_XMM2_Db;
    *(undefined4 *)(puVar35 + 0x1d) = in_XMM2_Dc;
    *(undefined4 *)((longlong)puVar35 + 0xec) = in_XMM2_Dd;
    *(undefined4 *)(puVar35 + 0x1a) = in_XMM1_Da;
    *(undefined4 *)((longlong)puVar35 + 0xd4) = in_XMM1_Db;
    *(undefined4 *)(puVar35 + 0x1b) = in_XMM1_Dc;
    *(undefined4 *)((longlong)puVar35 + 0xdc) = in_XMM1_Dd;
    *(uint *)(puVar35 + 0x18) = in_XMM0_Da;
    *(uint *)((longlong)puVar35 + 0xc4) = in_XMM0_Db;
    *(uint *)(puVar35 + 0x19) = in_XMM0_Dc;
    *(uint *)((longlong)puVar35 + 0xcc) = in_XMM0_Dd;
    puVar35[0x27] = _Size;
    apuStack_138[1] = (undefined8 *)&UNK_14089e032;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    puVar35[0x2c] = extraout_RAX_09;
    uVar24 = extraout_RAX_09;
    if (puVar35[0x2e] == 0) {
LAB_14089e28e:
      *(int *)(puVar35 + 0x2f) = (int)CONCAT71((int7)((ulonglong)uVar24 >> 8),1);
      goto LAB_14089e296;
    }
    *(int *)(puVar35 + 0x2f) = (int)CONCAT71((int7)((ulonglong)extraout_RAX_09 >> 8),1);
    goto code_r0x00014089e05f;
  case 0x77:
    if (bVar44) {
      pppppppcVar39 = (code *******)0x3;
    }
    puVar35[0x15] = UNRECOVERED_JUMPTABLE;
    iVar9 = *(int *)(&DAT_142a9abc0 + (longlong)pppppppcVar39 * 4);
    puVar35[0x2b] = _Src_00;
    puVar35[0x16] = pppppppcVar1;
                    // WARNING: Could not recover jumptable at 0x00014089dddf. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_142a9abc0 + iVar9))(&DAT_142a9abc0 + iVar9);
    return;
  case 0x78:
    puVar33 = (undefined1 *)(ulonglong)(uVar36 + 1);
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(undefined **)(puVar33 + -8) = &UNK_14089dea6;
    country_core::utils::geo::mesh_from_pts();
    if (puVar35[0x18] != 0) {
      *(undefined **)(puVar33 + -8) = &UNK_14089decc;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    puVar35[0x26] = puVar35[0x14];
    puVar35[0x24] = puVar35[0x12];
    puVar35[0x25] = puVar35[0x13];
    puVar35[0x22] = puVar35[0x10];
    puVar35[0x23] = puVar35[0x11];
    puVar35[0x20] = puVar35[0xe];
    puVar35[0x21] = puVar35[0xf];
    puVar35[0x1e] = puVar35[0xc];
    puVar35[0x1f] = puVar35[0xd];
    puVar35[0x1c] = puVar35[10];
    puVar35[0x1d] = puVar35[0xb];
    puVar35[0x1a] = puVar35[8];
    puVar35[0x1b] = puVar35[9];
    puVar35[0x18] = puVar35[6];
    puVar35[0x19] = puVar35[7];
    puVar35[0x27] = _Size;
    *(undefined **)(puVar33 + -8) = &UNK_14089df43;
    asset_libraries::mesh_library::AssetMeshLibrary::add();
    puVar35[0x2c] = extraout_RAX_08;
    pcVar11 = *(code **)puVar35[0x2e];
    uVar24 = 0;
    if (pcVar11 != (code *)0x0) {
      uVar24 = puVar35[0x2d];
      *(undefined **)(puVar33 + -8) = &UNK_14089df62;
      uVar24 = (*pcVar11)(uVar24);
    }
    if (*(longlong *)(puVar35[0x2e] + 8) == 0) goto LAB_14089e28e;
    *(int *)(puVar35 + 0x2f) = (int)CONCAT71((int7)((ulonglong)uVar24 >> 8),1);
    puVar34 = puVar33;
code_r0x00014089e05f:
    *(undefined **)(puVar34 + -8) = &UNK_14089e06b;
    std::alloc::__default_lib_allocator::__rust_dealloc();
    puVar33 = puVar34;
LAB_14089e296:
    *(undefined8 *)(puVar33 + -8) = 0x14089e2a9;
    bevy_ecs::system::commands::Commands::spawn_empty();
    lVar42 = puVar35[0x2b];
    if (*(char *)(lVar42 + 0x50) == '\0') {
      uVar45 = 0x3f800000;
      cVar20 = *(char *)(lVar42 + 0x40);
    }
    else {
      uVar45 = *(undefined4 *)(lVar42 + 0x54);
      cVar20 = *(char *)(lVar42 + 0x40);
    }
    if (cVar20 == '\0') {
      *(undefined1 *)(lVar42 + 0x40) = 0;
      *(undefined4 *)(lVar42 + 0x44) = uVar45;
      uVar45 = 0;
    }
    uVar24 = *(undefined8 *)(lVar42 + 0x68);
    *(undefined8 *)(puVar33 + -8) = 0x14089e2f0;
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_10 & 1) == 0) {
                    // WARNING: Subroutine does not return
      *(undefined **)(puVar33 + -8) = &UNK_14089e76f;
      core::option::unwrap_failed();
    }
    plVar43 = (longlong *)puVar35[0x19];
    if (plVar43 == (longlong *)0x0) {
      plVar43 = (longlong *)puVar35[0x1a];
    }
    puVar12 = (undefined8 *)puVar35[0x2b];
    uVar17 = *puVar12;
    uVar18 = puVar12[1];
    uVar4 = *(undefined4 *)(puVar12 + 0xf);
    uVar10 = *(undefined4 *)(puVar12 + 9);
    uVar5 = *(undefined4 *)((longlong)puVar12 + 0x4c);
    uVar13 = puVar12[8];
    uVar14 = puVar35[0x1c];
    lVar42 = plVar43[2];
    if ((ulonglong)(*plVar43 - lVar42) < 0x98) {
      *(undefined8 *)(puVar33 + 0x20) = 1;
      *(undefined8 *)(puVar33 + -8) = 0x14089e6a5;
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar38 = puVar35[0x2b];
    lVar25 = plVar43[1];
    *(code **)(lVar25 + lVar42) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::systems::ui::signifiers::SignifierAnimation>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar12 = (undefined8 *)(lVar25 + 8 + lVar42);
    *puVar12 = uVar17;
    puVar12[1] = uVar18;
    *(undefined8 *)(lVar25 + 0x18 + lVar42) = *(undefined8 *)(lVar38 + 0x7c);
    *(undefined4 *)(lVar25 + 0x20 + lVar42) = *(undefined4 *)(lVar38 + 0x84);
    *(undefined4 *)(lVar25 + 0x24 + lVar42) = uVar45;
    *(undefined4 *)(lVar25 + 0x28 + lVar42) = uVar45;
    *(undefined4 *)(lVar25 + 0x2c + lVar42) = uVar45;
    *(undefined8 *)(lVar25 + 0x38 + lVar42) = puVar35[0x2c];
    *(undefined8 *)(lVar25 + 0x40 + lVar42) = uVar24;
    *(undefined4 *)(lVar25 + 0x48 + lVar42) = uVar10;
    *(undefined4 *)(lVar25 + 0x4c + lVar42) = uVar5;
    *(undefined8 *)(lVar25 + 0x50 + lVar42) = uVar13;
    *(undefined4 *)(lVar25 + 0x58 + lVar42) = uVar4;
    *(undefined8 *)(lVar25 + 0x5c + lVar42) = 0;
    *(undefined1 *)(lVar25 + 0x68 + lVar42) = 0;
    *(undefined8 *)(lVar25 + 0x78 + lVar42) = uVar14;
    *(code **)(lVar25 + 0x88 + lVar42) = bevy_ecs::error::handler::panic;
    lVar25 = lVar42 + 0x98;
    plVar43[2] = lVar25;
    if (*(char *)(lVar38 + 0x89) == '\0') {
      if (*(char *)(lVar38 + 0x8a) == '\0') {
        if ((ulonglong)(*plVar43 - lVar25) < 0x20) {
          *(undefined8 *)(puVar33 + 0x20) = 1;
          *(undefined8 *)(puVar33 + -8) = 0x14089e739;
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        lVar25 = plVar43[1];
        *(code **)(lVar25 + 0x98 + lVar42) =
             core::ops::function::FnOnce::
             call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::UiPass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
        ;
        *(code **)(lVar25 + 0xa0 + lVar42) = bevy_ecs::error::handler::panic;
        *(undefined8 *)(lVar25 + 0xa8 + lVar42) = uVar14;
        *(undefined1 *)(lVar25 + 0xb0 + lVar42) = 0;
        lVar25 = lVar42 + 0xb8;
        plVar43[2] = lVar25;
        lVar38 = 0xd8;
        if ((ulonglong)(*plVar43 - lVar25) < 0x20) {
          *(undefined8 *)(puVar33 + 0x20) = 1;
          *(undefined8 *)(puVar33 + -8) = 0x14089e75e;
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
      }
      else {
        lVar38 = 0xb8;
        if ((ulonglong)(*plVar43 - lVar25) < 0x20) {
          *(undefined8 *)(puVar33 + 0x20) = 1;
          *(undefined8 *)(puVar33 + -8) = 0x14089e4b0;
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
      }
    }
    else {
      if ((ulonglong)(*plVar43 - lVar25) < 0x20) {
        *(undefined8 *)(puVar33 + 0x20) = 1;
        *(undefined8 *)(puVar33 + -8) = 0x14089e6ca;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar25 = plVar43[1];
      *(code **)(lVar25 + 0x98 + lVar42) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::WireframePass>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar25 + 0xa0 + lVar42) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar25 + 0xa8 + lVar42) = uVar14;
      *(undefined1 *)(lVar25 + 0xb0 + lVar42) = 0;
      lVar25 = lVar42 + 0xb8;
      plVar43[2] = lVar25;
      lVar38 = 0xd8;
      if ((ulonglong)(*plVar43 - lVar25) < 0x20) {
        *(undefined8 *)(puVar33 + 0x20) = 1;
        *(undefined8 *)(puVar33 + -8) = 0x14089e470;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
    }
    lVar15 = plVar43[1];
    *(code **)(lVar15 + lVar25) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DontCastShadow>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)(lVar15 + 8 + lVar25) = bevy_ecs::error::handler::panic;
    *(undefined8 *)(lVar15 + 0x10 + lVar25) = uVar14;
    *(undefined1 *)(lVar15 + 0x18 + lVar25) = 0;
    lVar42 = lVar42 + lVar38;
    plVar43[2] = lVar42;
    lVar25 = puVar35[0x2b];
    if ((*(byte *)(lVar25 + 0x58) & 1) != 0) {
      uVar45 = *(undefined4 *)(lVar25 + 0x5c);
      if ((ulonglong)(*plVar43 - lVar42) < 0x28) {
        *(undefined8 *)(puVar33 + 0x20) = 1;
        *(undefined8 *)(puVar33 + -8) = 0x14089e6ef;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar25 = plVar43[1];
      *(code **)(lVar25 + lVar42) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierMinimumLifespan>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar25 + 8 + lVar42) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar25 + 0x10 + lVar42) = uVar14;
      *(undefined4 *)(lVar25 + 0x18 + lVar42) = 0;
      *(undefined4 *)(lVar25 + 0x1c + lVar42) = uVar45;
      *(undefined1 *)(lVar25 + 0x20 + lVar42) = 0;
      lVar42 = lVar42 + 0x28;
      plVar43[2] = lVar42;
      lVar25 = puVar35[0x2b];
    }
    if (*(char *)(lVar25 + 0x88) != '\0') {
      if ((ulonglong)(*plVar43 - lVar42) < 0x20) {
        *(undefined8 *)(puVar33 + 0x20) = 1;
        *(undefined8 *)(puVar33 + -8) = 0x14089e714;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar25 = plVar43[1];
      *(code **)(lVar25 + lVar42) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::systems::ui::signifiers::SignifierParentToVolumeOrigin>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)(lVar25 + 8 + lVar42) = bevy_ecs::error::handler::panic;
      *(undefined8 *)(lVar25 + 0x10 + lVar42) = uVar14;
      *(undefined1 *)(lVar25 + 0x18 + lVar42) = 0;
      plVar43[2] = lVar42 + 0x20;
    }
    if (*(char *)(puVar35 + 0x2f) != '\0') {
      plVar43 = *(longlong **)(puVar35[0x16] + 0x18);
      LOCK();
      lVar42 = *plVar43;
      *plVar43 = *plVar43 + 1;
      UNLOCK();
      if (*plVar43 == 0 || SCARRY8(lVar42,1) != *plVar43 < 0) {
                    // WARNING: Does not return
        pcVar11 = (code *)invalidInstructionException();
        (*pcVar11)();
      }
      *(undefined1 **)(puVar33 + -8) = &LAB_14089e607;
      bevy_ecs::system::commands::EntityCommands::
      insert<country_core::components::transient_mesh::TransientMesh>();
    }
    puVar12 = (undefined8 *)puVar35[0x15];
    puVar12[4] = puVar35[0x1c];
    uVar24 = puVar35[0x18];
    uVar13 = puVar35[0x19];
    uVar14 = puVar35[0x1b];
    puVar12[2] = puVar35[0x1a];
    puVar12[3] = uVar14;
    *puVar12 = uVar24;
    puVar12[1] = uVar13;
    if (*(longlong *)(puVar35[0x2b] + 0x60) != 0) {
      *(undefined1 **)(puVar33 + -8) = &LAB_14089e64c;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  case 0x79:
    *(byte *)_Src = (char)*(code *)_Src + bVar19;
    *(uint *)(UNRECOVERED_JUMPTABLE + 0x22) =
         (*(uint *)(UNRECOVERED_JUMPTABLE + 0x22) + 0xf76b7fe) - (uint)CARRY1(bVar19,bVar28);
    return;
  }
  *(uint *)(local_90 + 1) = in_R8[2];
  *local_90 = *(code *******)in_R8;
  return;
}


