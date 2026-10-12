
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x000140bfda59) overlaps instruction at (ram,0x000140bfda57)
    */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong * FUN_140bfc0d0(ulonglong *param_1,ulonglong param_2,longlong param_3)

{
  undefined8 *puVar1;
  code *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  longlong lVar11;
  byte bVar12;
  int iVar13;
  ulonglong *puVar14;
  uint7 uVar19;
  longlong lVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong *puVar20;
  uint uVar21;
  ulonglong uVar22;
  undefined8 *unaff_RBX;
  ulonglong *puVar23;
  ulonglong uVar24;
  ulonglong unaff_R12;
  ulonglong unaff_R13;
  undefined4 unaff_R14D;
  undefined4 uVar25;
  ulonglong unaff_R15;
  undefined1 auVar26 [16];
  int iVar27;
  int iVar28;
  int iVar29;
  int iVar30;
  undefined1 auVar31 [16];
  undefined8 in_stack_fffffffffffffeb8;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  ulonglong uStack_138;
  uint uStack_130;
  undefined4 uStack_12c;
  longlong lStack_128;
  undefined4 uStack_120;
  ulonglong *puStack_118;
  undefined4 auStack_110 [2];
  undefined8 uStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  ulonglong uStack_e8;
  ulonglong uStack_c8;
  ulonglong uStack_c0;
  undefined8 in_stack_ffffffffffffff48;
  ulonglong uStack_b0;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined8 uStack_90;
  ulonglong uStack_88;
  uint uStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  longlong lStack_70;
  ulonglong uStack_68;
  ulonglong uStack_60;
  longlong lStack_58;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  char *pcVar18;
  
  uVar25 = (undefined4)((ulonglong)in_stack_fffffffffffffeb8 >> 0x20);
  puVar23 = &uStack_e8;
  uStack_48 = 0xfffffffffffffffe;
  puVar20 = (ulonglong *)&UNK_142b174cc;
  iVar13 = *(int *)(&UNK_142b174cc + (ulonglong)*(byte *)(param_3 + 0x94) * 4);
  puVar14 = (ulonglong *)(&UNK_142b174cc + iVar13);
  uVar21 = (uint)puVar14;
  uVar19 = (uint7)((ulonglong)puVar14 >> 8);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(*(byte *)(param_3 + 0x94)) {
  case 0:
    uVar25 = 4;
    break;
  case 1:
    uVar25 = 3;
    break;
  case 2:
    uVar25 = 0x3b9c9309;
    break;
  case 3:
    uVar25 = 0;
    uVar21 = 0;
    goto code_r0x000140bfc151;
  case 4:
    goto code_r0x000140bfda50;
  case 5:
    puVar23 = (ulonglong *)(param_2 & 0xffffffff);
    puVar23[-0xb] = unaff_R15;
    puVar23[-10] = unaff_R13;
    puVar23[-9] = unaff_R12;
    *(int *)(puVar23 + -8) = (int)param_3;
    *(undefined4 *)((longlong)puVar23 + -0x3c) = unaff_R14D;
    puVar23[-7] = (ulonglong)unaff_RBX;
    uVar17 = puVar23[0x14];
    FUN_140bfd720(puVar23 + 2,param_2,puVar23 + -0xb,uVar17);
    if (puVar23[2] == 6) {
      FUN_140be24b0(uVar17);
      uVar17 = puVar23[3];
      uVar16 = puVar23[4];
      uVar24 = puVar23[6];
      puVar14 = (ulonglong *)puVar23[0x14];
      puVar14[2] = puVar23[5];
      puVar14[3] = uVar24;
      *puVar14 = uVar17;
      puVar14[1] = uVar16;
      FUN_140c0eb40(puVar23 + -0xb,puVar14,param_2);
      if (*(char *)((longlong)puVar23 + 6) != '\a') {
        param_1[0xc] = puVar23[1];
        uVar17 = *puVar23;
        param_1[10] = puVar23[-1];
        param_1[0xb] = uVar17;
        uVar17 = puVar23[-2];
        param_1[8] = puVar23[-3];
        param_1[9] = uVar17;
        uVar17 = puVar23[-0xb];
        uVar16 = puVar23[-10];
        uVar24 = puVar23[-9];
        uVar22 = puVar23[-8];
        uVar3 = puVar23[-7];
        uVar4 = puVar23[-6];
        uVar5 = puVar23[-4];
        param_1[6] = puVar23[-5];
        param_1[7] = uVar5;
        param_1[4] = uVar3;
        param_1[5] = uVar4;
        param_1[2] = uVar24;
        param_1[3] = uVar22;
        *param_1 = uVar17;
        param_1[1] = uVar16;
        return param_1;
      }
      uVar17 = puVar23[-0xb] - 2;
      goto code_r0x000140bfdd0c;
    }
    uVar17 = puVar23[7];
    puVar20 = (ulonglong *)puVar23[8];
    uVar24 = puVar23[2] - 2;
    uVar16 = 1;
    if (uVar24 < 4) {
      uVar16 = uVar24;
    }
    if (1 < uVar16) {
      puVar23[0x15] = uVar24;
      puVar23[0x1a] = (ulonglong)puVar20;
      puVar23[0x14] = uVar17;
      if (uVar16 == 2) {
        FUN_140befd60(param_2);
        puVar23[-0xb] = (ulonglong)&UNK_142b16f18;
        puVar23[-10] = 1;
        puVar23[-9] = 8;
        puVar23[-8] = 0;
        puVar23[-7] = 0;
        FUN_1428d9390(puVar23 + -0xb,&UNK_142b16f28);
        goto code_r0x000140bfe035;
      }
      *(int *)((longlong)puVar23 + 0xdc) = (int)puVar23[5];
      puVar23[0x12] = puVar23[3];
      puVar23[0x13] = puVar23[4];
      FUN_140befd60(param_2);
      puVar23[0x16] = (longlong)puVar23 + 0xdc;
      puVar23[0x17] = (ulonglong)&UNK_140c8f550;
      puVar23[0x18] = (ulonglong)(puVar23 + 0x12);
      puVar23[0x19] = (ulonglong)&UNK_140109e90;
      puVar23[-0xb] = (ulonglong)&UNK_142b16f70;
      puVar23[-10] = 2;
      puVar23[-7] = 0;
      puVar14 = puVar23 + 0x16;
      goto code_r0x000140bfe019;
    }
    *(undefined1 *)((longlong)param_1 + 0x5e) = 7;
    if (uVar17 == 0 || uVar24 != 1 && uVar24 < 4) {
      return param_1;
    }
    goto code_r0x000140bfdeb5;
  case 6:
    goto code_r0x000140bfda50;
  case 7:
    uVar17 = param_2;
    goto code_r0x000140bfdebb;
  case 8:
    puVar20 = (ulonglong *)0x1;
    if (uVar21 < 4) {
      puVar20 = puVar14;
    }
code_r0x000140bfda50:
                    /* WARNING: Could not recover jumptable at 0x000140bfda5e. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    puVar14 = (ulonglong *)
              (*(code *)(&UNK_142b174dc + *(int *)(&UNK_142b174dc + (longlong)puVar20 * 4)))
                        (&UNK_142b174dc + *(int *)(&UNK_142b174dc + (longlong)puVar20 * 4));
    return puVar14;
  case 9:
    uVar17 = (ulonglong)(uVar21 - 2);
code_r0x000140bfdd0c:
    switch(uVar17) {
    case 0:
      *(undefined1 *)((longlong)param_1 + 0x5e) = 7;
      return param_1;
    default:
      puVar23[0x18] = puVar23[-4];
      puVar23[0x16] = puVar23[-6];
      puVar23[0x17] = puVar23[-5];
      if (1 < *puRam000000014362c3a8) {
        puVar23[0x12] = (ulonglong)(puVar23 + 0x16);
        puVar23[0x13] = (ulonglong)&UNK_14003bad0;
        puVar14 = (ulonglong *)func_0x00014067dd80(&UNK_142b16e10);
        uVar17 = *puVar14;
        uVar16 = puVar14[1];
        uVar24 = puVar14[2];
        puVar23[8] = 2;
        puVar23[9] = (ulonglong)&UNK_142b16d70;
        puVar23[10] = 0x18;
        puVar23[0xc] = (ulonglong)&UNK_142b16e00;
        puVar23[0xd] = 1;
        puVar23[0xe] = (ulonglong)(puVar23 + 0x12);
        puVar23[0xf] = 1;
        puVar23[0x10] = 0;
        puVar23[2] = 0;
        puVar23[3] = (ulonglong)&UNK_142b16d70;
        puVar23[4] = 0x18;
        puVar23[5] = 0;
        puVar23[6] = uVar17;
        puVar23[7] = uVar16;
        *(undefined4 *)(puVar23 + 0xb) = 1;
        *(int *)((longlong)puVar23 + 0x5c) = (int)uVar24;
        func_0x000140d4a330((longlong)puVar23 + 0xdc,puVar23 + 2);
      }
      *(undefined1 *)((longlong)param_1 + 0x5e) = 7;
      uVar17 = puVar23[0x16];
      if (uVar17 == 0) {
        return param_1;
      }
      puVar20 = (ulonglong *)puVar23[0x17];
      break;
    case 2:
      goto code_r0x000140bfe0c9;
    case 3:
      *(int *)((longlong)puVar23 + 0xdc) = (int)puVar23[-8];
      puVar23[0x12] = puVar23[-10];
      puVar23[0x13] = puVar23[-9];
      FUN_140befd60(param_2);
      puVar23[0x16] = (longlong)puVar23 + 0xdc;
      puVar23[0x17] = (ulonglong)&UNK_140c8f550;
      puVar23[0x18] = (ulonglong)(puVar23 + 0x12);
      puVar23[0x19] = (ulonglong)&UNK_140109e90;
      puVar23[2] = (ulonglong)&UNK_142b16ea8;
      puVar23[3] = 2;
      puVar23[6] = 0;
      puVar14 = puVar23 + 0x16;
      goto code_r0x000140bfe0ad;
    }
code_r0x000140bfdeb5:
    param_3 = 1;
code_r0x000140bfdebb:
    func_0x000140613c20(puVar20,uVar17,param_3);
    return param_1;
  case 10:
    puVar14 = (ulonglong *)((ulonglong)uVar19 << 8);
    *(char *)puVar14 = (char)*puVar14;
code_r0x000140bfe0ad:
    puVar23[4] = (ulonglong)puVar14;
    puVar23[5] = 2;
    FUN_1428d9390(puVar23 + 2,&UNK_142b16ec8);
code_r0x000140bfe0c9:
    FUN_140befd60(param_2);
    puVar23[2] = (ulonglong)&UNK_142b16e58;
    puVar23[3] = 1;
    puVar23[4] = 8;
    puVar23[5] = 0;
    puVar23[6] = 0;
    FUN_1428d9390(puVar23 + 2,&UNK_142b16e68);
    pcVar2 = (code *)swi(3);
    puVar14 = (ulonglong *)(*pcVar2)();
    return puVar14;
  case 0xb:
    *(byte *)puVar14 = (byte)*puVar14 + (byte)puVar14;
code_r0x000140bfe019:
    puVar23[-9] = (ulonglong)puVar14;
    puVar23[-8] = 2;
    FUN_1428d9390(puVar23 + -0xb,&UNK_142b16f90);
code_r0x000140bfe035:
                    /* WARNING: Does not return */
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  case 0xc:
    bVar12 = (byte)puVar14 & (byte)*puVar14;
    pcVar18 = (char *)CONCAT71(uVar19,bVar12);
    *pcVar18 = *pcVar18 + bVar12;
    FUN_1428d9430();
    FUN_1428d9596(0,auStack_110,&uStack_88,&stack0xffffffffffffff20,&UNK_142b170c0);
    FUN_1428d9430(&UNK_142b15fb0,0x28,&UNK_142b17078);
    FUN_1428d9760(&UNK_142b15188,0x2b,&stack0xffffffffffffff20,&UNK_142b151b8,&UNK_142b170d8);
    pcVar2 = (code *)swi(3);
    puVar14 = (ulonglong *)(*pcVar2)();
    return puVar14;
  case 0xd:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0xe:
    FUN_140c03fa0(param_1,param_2,&stack0xffffffffffffff20);
    return param_1;
  case 0xf:
    FUN_1418b77a0(&lStack_f8,&stack0xffffffffffffff20,&UNK_142b15090);
    uStack_90 = CONCAT44(uStack_ec,uStack_f0);
    uStack_138 = uStack_e8;
    lStack_128 = CONCAT71(lStack_128._1_7_,0x22);
    uStack_130 = uStack_130 & 0xffffff00;
    FUN_140bfeac0(&stack0xffffffffffffff20,param_2,unaff_RBX + 1,*unaff_RBX,
                  CONCAT44(uVar25,unaff_R14D));
    if (unaff_R13 == 0xb) {
      *param_1 = 0xb;
    }
    else {
      param_1[9] = CONCAT44(uStack_94,uStack_98);
      param_1[7] = 0;
      param_1[8] = CONCAT44(uStack_9c,uStack_a0);
      param_1[5] = CONCAT44((int)((ulonglong)in_stack_ffffffffffffff48 >> 0x20),
                            (int)in_stack_ffffffffffffff48 + uVar21 +
                            (uint)(0xfffffffebd4e8b33 < (ulonglong)(longlong)iVar13));
      param_1[6] = uStack_b0;
      param_1[3] = uStack_c8;
      param_1[4] = uStack_c0;
      param_1[1] = (ulonglong)puVar14;
      param_1[2] = unaff_R12;
      *param_1 = unaff_R13;
    }
    if (lStack_f8 != 0) {
      func_0x000140613c20(uStack_90,lStack_f8 * 0x30,8);
    }
    return param_1;
  }
  uVar21 = 1;
code_r0x000140bfc151:
  uStack_e8 = (ulonglong)uVar21;
  uStack_49 = 1;
  lStack_70 = param_3;
  FUN_1418b8c50();
  lVar11 = lStack_70;
  uVar21 = *(uint *)(lStack_70 + 0x80);
  uVar16 = (ulonglong)uVar21;
  uVar17 = uVar16 * 8;
  if (uVar16 == 0) {
    uStack_78 = 0;
    lVar15 = 4;
  }
  else {
    lVar15 = func_0x000140613c10(uVar17);
    iVar10 = _UNK_142b14f0c;
    iVar9 = _UNK_142b14f08;
    iVar8 = _UNK_142b14f04;
    iVar7 = _UNK_142b14f00;
    iVar6 = _UNK_142aebae4;
    iVar13 = _UNK_142aebae0;
    if (lVar15 == 0) {
      uStack_4a = 1;
      FUN_1428d8fe3(4,uVar17,&UNK_142b15090);
      goto code_r0x000140bfc48f;
    }
    if (uVar21 < 4) {
      uVar24 = 0;
code_r0x000140bfc270:
      do {
        *(int *)(lVar15 + uVar24 * 8) = (int)uVar24;
        *(undefined4 *)(lVar15 + 4 + uVar24 * 8) = 2;
        uVar24 = uVar24 + 1;
      } while (uVar16 != uVar24);
    }
    else {
      uVar24 = (ulonglong)(uVar21 & 0xfffffffc);
      auVar26._8_8_ = 0;
      auVar26._0_8_ = _UNK_142ab9500;
      uVar22 = 0;
      do {
        iVar27 = auVar26._0_4_;
        iVar28 = auVar26._4_4_;
        iVar29 = auVar26._8_4_;
        iVar30 = auVar26._12_4_;
        auVar31._0_8_ = auVar26._0_8_;
        auVar31._8_4_ = iVar28;
        auVar31._12_4_ = iVar6;
        *(undefined8 *)(lVar15 + uVar22) = CONCAT44(iVar13,iVar27);
        ((undefined8 *)(lVar15 + uVar22))[1] = auVar31._8_8_;
        puVar1 = (undefined8 *)(lVar15 + 0x10 + uVar22);
        *puVar1 = CONCAT44(iVar13,iVar27 + iVar13);
        *(int *)(puVar1 + 1) = iVar28 + iVar6;
        *(int *)((longlong)puVar1 + 0xc) = iVar6;
        auVar26._0_4_ = iVar27 + iVar7;
        auVar26._4_4_ = iVar28 + iVar8;
        auVar26._8_4_ = iVar29 + iVar9;
        auVar26._12_4_ = iVar30 + iVar10;
        uVar22 = uVar22 + 0x20;
      } while ((uVar17 & 0xffffffffffffffe0) != uVar22);
      if ((uVar21 & 0xfffffffc) != uVar21) goto code_r0x000140bfc270;
    }
    uStack_78 = *(undefined4 *)(lVar11 + 0x80);
  }
  uStack_e8 = 0;
  _uStack_140 = CONCAT44(uStack_a0,0x26);
  uStack_138 = 0;
  uStack_130 = 0;
  uStack_12c = uStack_f0;
  lStack_128 = lStack_f8;
  uStack_120 = 1;
  auStack_110[0] = 0;
  uStack_108 = 0;
  puStack_118 = puVar23;
  uStack_74 = uVar25;
  lStack_58 = lVar15;
  iVar13 = (**(code **)(param_2 + 0x280))(*(undefined8 *)(param_2 + 0x640),&uStack_140,0,&uStack_90)
  ;
  uStack_4b = iVar13 != 0;
  uStack_68 = uVar16;
  uStack_60 = uVar17;
  if ((bool)uStack_4b) {
    uStack_90 = CONCAT44(uStack_90._4_4_,iVar13);
    FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_90,&UNK_142b15128,&UNK_142b16b08);
code_r0x000140bfc48f:
                    /* WARNING: Does not return */
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  puVar14 = *(ulonglong **)(param_2 + 0x648);
  uStack_88 = (ulonglong)*(uint *)(lVar11 + 0x80);
  if (*(char *)(lVar11 + 0x94) == '\x03') {
    if (*(uint *)(lVar11 + 0x80) == 0) {
      FUN_1428d9518(0,0,&UNK_142d06080);
      goto code_r0x000140bfc48f;
    }
    bVar12 = *(byte *)(lVar11 + 0xc);
  }
  else {
    bVar12 = *(byte *)(lVar11 + 0x90);
  }
  uStack_80 = (uint)bVar12;
  uVar16 = FUN_1418ae9f0(puVar14 + 0x8e,&uStack_90);
  uVar25 = *(undefined4 *)(uVar16 + 0xc);
  uVar17 = puVar14[0x14a];
  *param_1 = *puVar14;
  param_1[1] = uVar16;
  *(undefined4 *)(param_1 + 2) = uVar25;
  *(short *)((longlong)param_1 + 0x14) = (short)uVar17;
  *(undefined1 *)((longlong)param_1 + 0x16) = 4;
  if ((int)uStack_68 != 0) {
    func_0x000140613c20(lStack_58,uStack_60,4);
  }
  if (lStack_100 != 0) {
    func_0x000140613c20(lStack_f8,lStack_100 * 0x24,4);
  }
  return param_1;
}

