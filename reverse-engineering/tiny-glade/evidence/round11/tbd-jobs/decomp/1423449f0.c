// 0x1423449f0 undefined FUN_1423449f0(void) body_bytes=476

/* WARNING: Instruction at (ram,0x000142344d72) overlaps instruction at (ram,0x000142344d71)
    */
/* WARNING: Removing unreachable block (ram,0x000142344d74) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1423449f0(char *param_1,undefined8 *param_2,longlong *param_3,longlong *param_4,
                  undefined8 *param_5,undefined8 *param_6,undefined8 *param_7,longlong *param_8,
                  longlong *param_9)

{
  undefined8 *puVar1;
  longlong lVar2;
  float fVar3;
  char cVar4;
  undefined4 uVar5;
  ulonglong uVar6;
  undefined *puVar7;
  longlong lVar8;
  undefined *puVar9;
  longlong lVar10;
  undefined4 *puVar11;
  undefined8 *puVar12;
  undefined8 uVar13;
  longlong lVar14;
  longlong lVar15;
  undefined *puVar16;
  uint uVar17;
  longlong *plVar18;
  float fVar19;
  float fVar20;
  int iVar22;
  undefined1 auVar21 [16];
  float fVar23;
  float fVar24;
  undefined1 auStack_a8 [112];
  
  lVar2 = *param_3;
  fVar24 = *(float *)(lVar2 + 0xe8);
  if (*(char *)(lVar2 + 0xf1) == '\0') {
    *(undefined4 *)param_7[2] = *(undefined4 *)((longlong)param_7 + 0x1c);
    puVar12 = (undefined8 *)*param_7;
    *(undefined4 *)(puVar12 + 2) = 0;
  }
  else {
    puVar12 = param_2;
    plVar18 = param_4;
    func_0x000140949c10(*param_5);
    fVar20 = _DAT_142934694 * _DAT_14304e2fc * fVar24;
    fVar19 = _DAT_1429cd034 * fVar24 + fVar20;
    uVar6 = (ulonglong)*(byte *)(lVar2 + 0xf0);
    switch(uVar6) {
    case 1:
      break;
    case 2:
      break;
    default:
      puVar9 = &UNK_14304e708;
      puVar7 = &UNK_14304e708 + *(int *)(&UNK_14304e708 + uVar6 * 4);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(uVar6) {
      case 3:
        goto code_r0x000142344b47;
      case 4:
        goto code_r0x000142344d63;
      case 5:
        *param_1 = *param_1 + '\b';
        *param_1 = *param_1;
        puVar7 = (undefined *)CONCAT71((int7)((ulonglong)puVar7 >> 8),(char)puVar7 + -8);
        break;
      case 6:
        puVar9 = &UNK_14304e707;
        puVar7 = (undefined *)(ulonglong)((int)puVar7 + 0x5e0b67);
        goto code_r0x000142344d89;
      case 7:
        goto code_r0x000142344d6a;
      }
      goto code_r0x000142344d7a;
    }
code_r0x000142344b47:
    puVar12 = (undefined8 *)*param_7;
    fVar20 = *(float *)(puVar12 + 2);
    cVar4 = func_0x00014093fbe0(*param_2);
    if (cVar4 == '\0') {
code_r0x000142344be2:
      fVar19 = *(float *)(*param_4 + 0x58) / _DAT_1429258d8;
code_r0x000142344bf2:
      uVar5 = *(undefined4 *)((longlong)param_7 + 0x1c);
      puVar11 = (undefined4 *)param_7[2];
      *puVar11 = uVar5;
      fVar19 = fVar19 + *(float *)(puVar12 + 2);
      param_4 = plVar18;
    }
    else {
      FUN_14123fff0(auStack_a8);
      cVar4 = func_0x000141240140(auStack_a8);
      if (cVar4 == '\0') goto code_r0x000142344be2;
      cVar4 = func_0x00014093fa70(*(undefined4 *)*param_6);
      if (cVar4 == '\0') {
        fVar19 = *(float *)(*param_4 + 0x58) / _UNK_142925900;
        goto code_r0x000142344bf2;
      }
      fVar19 = *(float *)(puVar12 + 2);
      uVar5 = *(undefined4 *)((longlong)param_7 + 0x1c);
      puVar11 = (undefined4 *)param_7[2];
      param_4 = plVar18;
    }
    fVar3 = _DAT_142925904;
    fVar23 = 0.0;
    if (0.0 <= fVar19) {
      fVar23 = fVar19;
    }
    fVar19 = _DAT_142925904;
    if (fVar23 <= _DAT_142925904) {
      fVar19 = fVar23;
    }
    *puVar11 = uVar5;
    *(float *)(puVar12 + 2) = fVar19;
    if ((fVar20 != 0.0) || (fVar19 <= 0.0)) {
      if ((fVar20 != _DAT_142925904) || ((NAN(fVar20) || NAN(_DAT_142925904) || (fVar3 <= fVar19))))
      goto LAB_142344cd5;
      *(undefined4 *)param_9[2] = *(undefined4 *)((longlong)param_9 + 0x1c);
      lVar14 = *param_9;
      lVar15 = *(longlong *)(lVar14 + 0x30);
      uVar13 = *(undefined8 *)(lVar14 + 0x40);
      if (lVar15 == *(longlong *)(lVar14 + 0x20)) {
        FUN_140562c30(lVar14 + 0x20,&UNK_142bb5270);
      }
      lVar8 = *(longlong *)(lVar14 + 0x28);
      lVar10 = lVar15 * 0x20;
      *(undefined4 *)(lVar8 + lVar10) = 0x11;
    }
    else {
      *(undefined4 *)param_9[2] = *(undefined4 *)((longlong)param_9 + 0x1c);
      lVar14 = *param_9;
      lVar15 = *(longlong *)(lVar14 + 0x30);
      uVar13 = *(undefined8 *)(lVar14 + 0x40);
      if (lVar15 == *(longlong *)(lVar14 + 0x20)) {
        FUN_140562c30(lVar14 + 0x20,&UNK_142bb5270);
      }
      lVar8 = *(longlong *)(lVar14 + 0x28);
      lVar10 = lVar15 * 0x20;
      *(undefined4 *)(lVar8 + lVar10) = 0x10;
    }
    *(undefined8 *)(lVar8 + 0x18 + lVar10) = uVar13;
    *(longlong *)(lVar14 + 0x30) = lVar15 + 1;
    *(longlong *)(lVar14 + 0x40) = *(longlong *)(lVar14 + 0x40) + 1;
  }
LAB_142344cd5:
  fVar24 = fVar24 * _DAT_14292c76c;
  fVar19 = (float)FUN_140c95ef0(&UNK_14304e668);
  fVar19 = fVar19 * fVar24;
  *(undefined4 *)param_8[2] = *(undefined4 *)((longlong)param_8 + 0x1c);
  puVar7 = *(undefined **)(*param_8 + 0xa0);
  puVar9 = (undefined *)0x0;
  puVar16 = puVar7;
  do {
    if ((undefined *)(*(longlong *)(*param_8 + 0xa8) * 0x28) == puVar9) {
      return;
    }
    puVar1 = (undefined8 *)(puVar16 + 0x10);
    puVar16 = puVar16 + 0x28;
    auVar21._0_4_ = -(uint)((int)*puVar1 == (int)*puVar12);
    iVar22 = -(uint)((int)((ulonglong)*puVar1 >> 0x20) == (int)((ulonglong)*puVar12 >> 0x20));
    auVar21._4_4_ = auVar21._0_4_;
    auVar21._8_4_ = iVar22;
    auVar21._12_4_ = iVar22;
    uVar17 = movmskpd((int)param_4,auVar21);
    param_4 = (longlong *)(ulonglong)uVar17;
    puVar9 = puVar9 + 0x28;
  } while (uVar17 != 3);
  puVar12 = (undefined8 *)(ulonglong)*(byte *)(lVar2 + 0xf0);
  fVar20 = 0.0;
code_r0x000142344d63:
  param_3 = (longlong *)&UNK_14304e718;
code_r0x000142344d6a:
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
  switch((undefined *)
         ((longlong)*(int *)((longlong)param_3 + (longlong)puVar12 * 4) + (longlong)param_3)) {
  case (undefined *)0x142344d73:
    fVar19 = (float)((uint)fVar19 ^ _UNK_1429258f0);
  case (undefined *)0x142344d7a:
code_r0x000142344d7a:
    fVar24 = 0.0;
    fVar20 = fVar19;
    break;
  case (undefined *)0x142344d82:
    fVar19 = (float)((uint)fVar19 ^ _UNK_1429258f0);
  case (undefined *)0x142344d89:
code_r0x000142344d89:
    fVar24 = fVar19;
  }
  *(float *)(puVar7 + -0x20 + (longlong)puVar9) = fVar24;
  *(float *)(puVar7 + -0x1c + (longlong)puVar9) = fVar20;
  return;
}

