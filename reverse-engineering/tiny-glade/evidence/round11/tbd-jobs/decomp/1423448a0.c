// 0x1423448a0 undefined FUN_1423448a0(void) body_bytes=70

/* WARNING: Instruction at (ram,0x000142344d69) overlaps instruction at (ram,0x000142344d63)
    */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1423448a0(float param_1,undefined8 *param_2,undefined8 *param_3,ulonglong param_4)

{
  char *pcVar1;
  byte bVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  uint *puVar7;
  undefined8 *puVar8;
  code *pcVar9;
  float fVar10;
  char cVar11;
  undefined4 uVar12;
  undefined *puVar13;
  longlong lVar14;
  longlong *in_RCX;
  longlong lVar15;
  longlong lVar16;
  undefined4 *puVar17;
  undefined8 *puVar18;
  undefined8 uVar19;
  longlong *plVar20;
  longlong lVar21;
  uint uVar22;
  undefined8 *unaff_R13;
  ulonglong uVar23;
  longlong *plVar24;
  float fVar25;
  int iVar27;
  undefined1 auVar26 [16];
  float fVar28;
  float unaff_XMM9_Da;
  float fVar29;
  undefined8 *in_stack_000000b0;
  longlong *in_stack_000000c8;
  int iStack_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  int iStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined1 auStack_178 [280];
  undefined *puStack_60;
  
  puVar7 = (uint *)*param_3;
  uVar22 = *puVar7;
  uVar23 = (ulonglong)uVar22;
  uVar3 = puVar7[1];
  uVar4 = puVar7[2];
  uVar5 = puVar7[3];
  plVar20 = (longlong *)(ulonglong)uVar5;
  bVar2 = *(byte *)(*in_RCX + 0xf0);
  uVar6 = *(uint *)((longlong)param_2 + 0x1c);
  plVar24 = (longlong *)(ulonglong)uVar6;
  puVar7 = (uint *)param_2[2];
  *puVar7 = uVar6;
  puVar18 = (undefined8 *)*param_2;
  puVar13 = &UNK_14304e6f8 + *(int *)(&UNK_14304e6f8 + (ulonglong)bVar2 * 4);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch(bVar2) {
  case 0:
    puStack_60 = &UNK_1423448f4;
    FUN_142376050(puVar18,uVar23,uVar3);
    *puVar7 = uVar6;
    puStack_60 = &UNK_142344906;
    FUN_142376300(puVar18,uVar22,uVar3);
    *puVar7 = uVar6;
    goto FUN_142375b40;
  case 1:
    puStack_60 = &UNK_1423449b1;
    FUN_142376230(puVar18,uVar23,uVar3);
    *puVar7 = uVar6;
    puStack_60 = &UNK_1423449c3;
    FUN_142375f80(puVar18,uVar23,uVar3);
    *puVar7 = uVar6;
    goto FUN_142375eb0;
  case 2:
    puStack_60 = &UNK_142344933;
    FUN_142375c10(puVar18,uVar23,uVar3);
    *puVar7 = uVar6;
    puStack_60 = &UNK_142344945;
    FUN_142375f80(puVar18,uVar23,uVar3);
    *puVar7 = uVar6;
FUN_142375eb0:
    FUN_140fe7cb0(&iStack_1b8,puVar18,uVar4,plVar20);
    if (iStack_1b8 == 4) {
      FUN_142374fd0(auStack_178,CONCAT44(uStack_1ac,uStack_1b0));
      FUN_140fe7b10(&iStack_1b8,puVar18,uVar4,plVar20,auStack_178);
      if (iStack_1b8 == 4) {
        return;
      }
      uStack_188 = uStack_1a8;
      uStack_180 = uStack_1a0;
      iStack_198 = iStack_1b8;
      uStack_194 = uStack_1b4;
      uStack_190 = uStack_1b0;
      uStack_18c = uStack_1ac;
      puVar13 = &UNK_143052e68;
    }
    else {
      uStack_188 = uStack_1a8;
      uStack_180 = uStack_1a0;
      iStack_198 = iStack_1b8;
      uStack_194 = uStack_1b4;
      uStack_190 = uStack_1b0;
      uStack_18c = uStack_1ac;
      puVar13 = &UNK_143052e50;
    }
    FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_198,&UNK_143052c30,puVar13);
    pcVar9 = (code *)swi(3);
    (*pcVar9)();
    return;
  case 3:
    puStack_60 = &UNK_142344972;
    FUN_142376050(puVar18,uVar23,uVar3);
    *puVar7 = uVar6;
    puStack_60 = &UNK_142344984;
    FUN_142375ce0(puVar18,uVar23,uVar3);
    *puVar7 = uVar6;
FUN_142375b40:
    FUN_140fe7cb0(&iStack_1b8,puVar18,uVar4,uVar5);
    if (iStack_1b8 == 4) {
      FUN_142374fd0(auStack_178,CONCAT44(uStack_1ac,uStack_1b0));
      FUN_140fe7b10(&iStack_1b8,puVar18,uVar4,uVar5,auStack_178);
      if (iStack_1b8 == 4) {
        return;
      }
      uStack_188 = uStack_1a8;
      uStack_180 = uStack_1a0;
      iStack_198 = iStack_1b8;
      uStack_194 = uStack_1b4;
      uStack_190 = uStack_1b0;
      uStack_18c = uStack_1ac;
      puVar13 = &UNK_143052e68;
    }
    else {
      uStack_188 = uStack_1a8;
      uStack_180 = uStack_1a0;
      iStack_198 = iStack_1b8;
      uStack_194 = uStack_1b4;
      uStack_190 = uStack_1b0;
      uStack_18c = uStack_1ac;
      puVar13 = &UNK_143052e50;
    }
    FUN_1428d9760(&UNK_143052c50,0x2b,&iStack_198,&UNK_143052c30,puVar13);
    pcVar9 = (code *)swi(3);
    (*pcVar9)();
    return;
  case 4:
                    /* WARNING: Could not emulate address calculation at 0x000142344aea */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_14304e708 + *(int *)(&UNK_14304e708 + (longlong)puVar13 * 4)))();
    return;
  case 5:
    puVar18 = (undefined8 *)
              CONCAT71((int7)((ulonglong)puVar18 >> 8),
                       (char)puVar18 + (char)((ulonglong)param_2 >> 8));
    break;
  case 6:
    break;
  case 7:
    DAT_142925870 =
         DAT_142925870 + (char)puVar13 +
         (0xfffffffebcfb1907U < (ulonglong)(longlong)*(int *)(&UNK_14304e6f8 + (ulonglong)bVar2 * 4)
         );
    break;
  case 8:
    goto code_r0x000142344d57;
  case 9:
    UNK_14304e75b = UNK_14304e75b + -8;
    goto code_r0x000142344d63;
  case 0xb:
    LOCK();
    *(char *)plVar20 = (char)*plVar20 + (char)uVar5;
    UNLOCK();
    pcVar1 = (char *)(((ulonglong)puVar13 & 0xffffffff) + 0xf);
    *pcVar1 = *pcVar1 + (char)(uVar5 >> 8);
    out((short)param_2,uVar5);
  case 10:
    goto code_r0x000142344d63;
  }
  puVar8 = (undefined8 *)*unaff_R13;
  fVar29 = *(float *)(puVar8 + 2);
  puStack_60 = &UNK_142344b5a;
  cVar11 = func_0x00014093fbe0(*puVar18);
  if (cVar11 == '\0') {
code_r0x000142344be2:
    fVar25 = *(float *)(*plVar20 + 0x58) / _DAT_1429258d8;
code_r0x000142344bf2:
    uVar12 = *(undefined4 *)((longlong)unaff_R13 + 0x1c);
    puVar17 = (undefined4 *)unaff_R13[2];
    *puVar17 = uVar12;
    fVar25 = fVar25 + *(float *)(puVar8 + 2);
  }
  else {
    puStack_60 = &UNK_142344b9a;
    FUN_14123fff0(&stack0xffffffffffffffd8);
    puStack_60 = &UNK_142344bae;
    cVar11 = func_0x000141240140(&stack0xffffffffffffffd8);
    if (cVar11 == '\0') goto code_r0x000142344be2;
    puStack_60 = &UNK_142344bc9;
    cVar11 = func_0x00014093fa70(*(undefined4 *)*in_stack_000000b0);
    if (cVar11 == '\0') {
      fVar25 = *(float *)(*plVar20 + 0x58) / _UNK_142925900;
      goto code_r0x000142344bf2;
    }
    fVar25 = *(float *)(puVar8 + 2);
    uVar12 = *(undefined4 *)((longlong)unaff_R13 + 0x1c);
    puVar17 = (undefined4 *)unaff_R13[2];
  }
  fVar10 = _DAT_142925904;
  fVar28 = 0.0;
  if (0.0 <= fVar25) {
    fVar28 = fVar25;
  }
  fVar25 = _DAT_142925904;
  if (fVar28 <= _DAT_142925904) {
    fVar25 = fVar28;
  }
  *puVar17 = uVar12;
  *(float *)(puVar8 + 2) = fVar25;
  if ((fVar29 != 0.0) || (fVar25 <= 0.0)) {
    if ((fVar29 == _DAT_142925904) && ((!NAN(fVar29) && !NAN(_DAT_142925904) && (fVar25 < fVar10))))
    {
      *(undefined4 *)in_stack_000000c8[2] = *(undefined4 *)((longlong)in_stack_000000c8 + 0x1c);
      lVar21 = *in_stack_000000c8;
      lVar16 = *(longlong *)(lVar21 + 0x30);
      uVar19 = *(undefined8 *)(lVar21 + 0x40);
      if (lVar16 == *(longlong *)(lVar21 + 0x20)) {
        puStack_60 = &UNK_142344cb3;
        FUN_140562c30(lVar21 + 0x20,&UNK_142bb5270);
      }
      lVar14 = *(longlong *)(lVar21 + 0x28);
      lVar15 = lVar16 * 0x20;
      *(undefined4 *)(lVar14 + lVar15) = 0x11;
      goto code_r0x000142344cc5;
    }
  }
  else {
    *(undefined4 *)in_stack_000000c8[2] = *(undefined4 *)((longlong)in_stack_000000c8 + 0x1c);
    lVar21 = *in_stack_000000c8;
    lVar16 = *(longlong *)(lVar21 + 0x30);
    uVar19 = *(undefined8 *)(lVar21 + 0x40);
    if (lVar16 == *(longlong *)(lVar21 + 0x20)) {
      puStack_60 = &UNK_142344c64;
      FUN_140562c30(lVar21 + 0x20,&UNK_142bb5270);
    }
    lVar14 = *(longlong *)(lVar21 + 0x28);
    lVar15 = lVar16 * 0x20;
    *(undefined4 *)(lVar14 + lVar15) = 0x10;
code_r0x000142344cc5:
    *(undefined8 *)(lVar14 + 0x18 + lVar15) = uVar19;
    *(longlong *)(lVar21 + 0x30) = lVar16 + 1;
    *(longlong *)(lVar21 + 0x40) = *(longlong *)(lVar21 + 0x40) + 1;
  }
  fVar29 = unaff_XMM9_Da * _DAT_14292c76c;
  puStack_60 = (undefined *)0x142344cf1;
  param_1 = (float)FUN_140c95ef0(&UNK_14304e668);
  param_1 = param_1 * fVar29;
  *(undefined4 *)plVar24[2] = *(undefined4 *)((longlong)plVar24 + 0x1c);
  lVar21 = *(longlong *)(*plVar24 + 0xa0);
  lVar16 = 0;
  do {
    if (*(longlong *)(*plVar24 + 0xa8) * 0x28 == lVar16) {
      return;
    }
    puVar18 = (undefined8 *)(lVar21 + 0x10);
    lVar21 = lVar21 + 0x28;
    auVar26._0_4_ = -(uint)((int)*puVar18 == (int)*puVar8);
    iVar27 = -(uint)((int)((ulonglong)*puVar18 >> 0x20) == (int)((ulonglong)*puVar8 >> 0x20));
    auVar26._4_4_ = auVar26._0_4_;
    auVar26._8_4_ = iVar27;
    auVar26._12_4_ = iVar27;
    uVar22 = movmskpd((int)param_4,auVar26);
    param_4 = (ulonglong)uVar22;
    lVar16 = lVar16 + 0x28;
  } while (uVar22 != 3);
code_r0x000142344d57:
  param_2 = (undefined8 *)(ulonglong)*(byte *)(uVar23 + 0xf0);
code_r0x000142344d63:
                    /* WARNING: Could not recover jumptable at 0x000142344d71. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&UNK_14304e718 + *(int *)(&UNK_14304e718 + (longlong)param_2 * 4)))
            (param_1,&UNK_14304e718 + *(int *)(&UNK_14304e718 + (longlong)param_2 * 4));
  return;
}

