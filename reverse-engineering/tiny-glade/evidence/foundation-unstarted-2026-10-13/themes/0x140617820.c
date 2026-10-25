
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140617820(longlong *param_1,longlong param_2)

{
  char *pcVar1;
  bool bVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong uVar6;
  longlong *plVar7;
  longlong *plVar8;
  byte bVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong *plVar12;
  longlong *plVar13;
  longlong lStack_b0;
  longlong *plStack_a8;
  longlong *plStack_a0;
  longlong lStack_98;
  longlong *plStack_90;
  longlong lStack_88;
  longlong *plStack_80;
  longlong *plStack_78;
  undefined1 uStack_69;
  longlong lStack_68;
  longlong *plStack_60;
  longlong *plStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar6 = *(ulonglong *)(param_2 + 0x20);
  if (*(ulonglong *)(param_2 + 0x28) < uVar6) {
    plVar8 = (longlong *)(param_2 + 0x18);
    uVar10 = *(ulonglong *)(param_2 + 0x28);
    do {
      uVar4 = uVar10 + 1;
      bVar9 = *(byte *)(*(longlong *)(param_2 + 0x18) + -1 + uVar4);
      if ((0x20 < (ulonglong)bVar9) || ((0x100002600U >> ((ulonglong)bVar9 & 0x3f) & 1) == 0)) {
        if (bVar9 == 0x5b) {
          pcVar1 = (char *)(param_2 + 0x30);
          *pcVar1 = *pcVar1 + -1;
          if (*pcVar1 != '\0') {
            *(ulonglong *)(param_2 + 0x28) = uVar4;
            if (uVar6 <= uVar4) goto LAB_140617b35;
            lVar5 = uVar10 + 2;
            goto LAB_140617b10;
          }
        }
        else {
          if (bVar9 != 0x7b) {
            plVar13 = (longlong *)FUN_1428a4690(param_2,&uStack_69,&UNK_142a0a360);
            goto LAB_140617e75;
          }
          pcVar1 = (char *)(param_2 + 0x30);
          *pcVar1 = *pcVar1 + -1;
          if (*pcVar1 != '\0') {
            *(ulonglong *)(param_2 + 0x28) = uVar4;
            lVar11 = -0x8000000000000000;
            lStack_88 = -0x8000000000000000;
            bVar2 = true;
            if (uVar4 < uVar6) goto LAB_1406178fe;
            goto LAB_140617b93;
          }
        }
        lStack_68 = 0x15;
        goto LAB_1406178ae;
      }
      *(ulonglong *)(param_2 + 0x28) = uVar4;
      uVar10 = uVar4;
    } while (uVar4 != uVar6);
  }
  lStack_68 = 5;
LAB_1406178ae:
  lVar5 = FUN_1428a0920(param_2,&lStack_68);
  goto LAB_140617e80;
  while( true ) {
    *(longlong *)(param_2 + 0x28) = lVar5;
    lVar11 = (1 - uVar6) + lVar5;
    lVar5 = lVar5 + 1;
    if (lVar11 == 1) break;
LAB_140617b10:
    bVar9 = *(byte *)(*(longlong *)(param_2 + 0x18) + -1 + lVar5);
    if ((0x20 < (ulonglong)bVar9) || ((0x100002600U >> ((ulonglong)bVar9 & 0x3f) & 1) == 0)) {
      if (bVar9 == 0x5d) {
        plVar7 = (longlong *)FUN_1428d9bd0(0,&UNK_142a0a2b0,&UNK_142a0a260);
        goto LAB_140617b49;
      }
      FUN_1406158f0(&lStack_68,param_2);
      plVar13 = plStack_58;
      lVar5 = lStack_68;
      plVar7 = plStack_60;
      if (SBORROW8(0,lStack_68)) {
        plVar13 = plVar8;
        lVar5 = -0x8000000000000000;
      }
      goto LAB_140617c7a;
    }
  }
LAB_140617b35:
  lStack_68 = 2;
  plVar7 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
LAB_140617b49:
  plVar13 = plVar8;
  lVar5 = -0x8000000000000000;
LAB_140617c7a:
  *(char *)(param_2 + 0x30) = *(char *)(param_2 + 0x30) + '\x01';
  lStack_68 = lVar5;
  plStack_60 = plVar7;
  plStack_58 = plVar13;
  plVar8 = (longlong *)FUN_1400584f0(param_2);
  if (!SBORROW8(0,lVar5)) {
    if (plVar8 == (longlong *)0x0) goto LAB_140617ca9;
    if (plVar13 != (longlong *)0x0) {
      plVar12 = plVar7 + 1;
      do {
        if (plVar12[-1] != 0) {
          func_0x000140613c20(*plVar12,plVar12[-1],1);
        }
        plVar12 = plVar12 + 5;
        plVar13 = (longlong *)((longlong)plVar13 + -1);
      } while (plVar13 != (longlong *)0x0);
    }
    goto LAB_140617da1;
  }
  plVar13 = plVar7;
  if (plVar8 != (longlong *)0x0) {
    lVar5 = *plVar8;
    plStack_50 = plVar8;
    if (lVar5 != 1) goto LAB_140617d6a;
    FUN_1405e7a00(plVar8 + 1);
    goto LAB_140617e61;
  }
  goto LAB_140617e75;
LAB_140617960:
  bVar9 = *(byte *)(*plVar8 + uVar4);
  if ((0x20 < (ulonglong)bVar9) || ((0x100002600U >> ((ulonglong)bVar9 & 0x3f) & 1) == 0))
  goto LAB_140617999;
  uVar4 = uVar4 + 1;
  *(ulonglong *)(param_2 + 0x28) = uVar4;
  if (uVar6 == uVar4) goto LAB_140617cdf;
  goto LAB_140617960;
LAB_140617cdf:
  lStack_68 = 5;
  plStack_50 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
  goto LAB_140617bab;
LAB_1406178fe:
  do {
    while( true ) {
      bVar9 = *(byte *)(*plVar8 + uVar4);
      uVar10 = (ulonglong)bVar9;
      if (0x2c < uVar10) break;
      if ((0x100002600U >> (uVar10 & 0x3f) & 1) == 0) {
        if (uVar10 == 0x2c) {
          if (bVar2) goto LAB_140617e38;
          uVar4 = uVar4 + 1;
          *(ulonglong *)(param_2 + 0x28) = uVar4;
          if (uVar6 <= uVar4) goto LAB_140617cdf;
          goto LAB_140617960;
        }
        break;
      }
      uVar4 = uVar4 + 1;
      *(ulonglong *)(param_2 + 0x28) = uVar4;
      if (uVar6 == uVar4) goto LAB_140617b93;
    }
    if (bVar9 == 0x7d) {
      plVar13 = plStack_78;
      lVar5 = lStack_88;
      plVar7 = plStack_80;
      if (!SBORROW8(0,lStack_88)) goto LAB_140617c13;
      plStack_50 = (longlong *)FUN_1428d9b60(&UNK_142a0a280,6);
      goto LAB_140617bab;
    }
    if (!bVar2) {
      lStack_68 = 8;
      plStack_50 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
      goto LAB_140617bab;
    }
    bVar2 = false;
LAB_140617999:
    if (bVar9 != 0x22) {
      if (bVar9 == 0x7d) {
        lStack_68 = 0x12;
        plStack_50 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
      }
      else {
LAB_140617e38:
        lStack_68 = 0x10;
        plStack_50 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
      }
      goto LAB_140617bab;
    }
    *(ulonglong *)(param_2 + 0x28) = uVar4 + 1;
    *(undefined8 *)(param_2 + 0x10) = 0;
    FUN_140b89af0(&lStack_68,plVar8,param_2);
    plStack_50 = plStack_60;
    if ((int)lStack_68 == 2) goto LAB_140617bab;
    if ((plStack_58 == (longlong *)0x6) &&
       (*(short *)((longlong)plStack_60 + 4) == _DAT_142b5911f && (int)*plStack_60 == _DAT_142b5911b
       )) {
      if (!SBORROW8(0,lStack_88)) {
        plStack_50 = (longlong *)FUN_1428d9c60(&UNK_142a0a280,6);
        goto LAB_140617bab;
      }
      uVar6 = *(ulonglong *)(param_2 + 0x28);
      if (*(ulonglong *)(param_2 + 0x20) <= uVar6) {
LAB_140617ea8:
        lStack_68 = 3;
        plStack_50 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
        goto LAB_140617bab;
      }
      while( true ) {
        uVar6 = uVar6 + 1;
        uVar10 = (ulonglong)*(byte *)(*plVar8 + -1 + uVar6);
        if (0x3a < uVar10) goto LAB_140617f09;
        if ((0x100002600U >> (uVar10 & 0x3f) & 1) == 0) break;
        *(ulonglong *)(param_2 + 0x28) = uVar6;
        if (uVar6 == *(ulonglong *)(param_2 + 0x20)) goto LAB_140617ea8;
      }
      if (uVar10 != 0x3a) {
LAB_140617f09:
        lStack_68 = 6;
        plStack_50 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
        goto LAB_140617bab;
      }
      *(ulonglong *)(param_2 + 0x28) = uVar6;
      FUN_1406158f0(&lStack_b0,param_2);
      plStack_50 = plStack_a8;
      if (SBORROW8(0,lStack_b0)) goto LAB_140617bab;
      lStack_98 = lStack_b0;
      plStack_90 = plStack_a0;
      if (!SBORROW8(0,lStack_88)) {
        FUN_14014a860(&lStack_88);
      }
      lStack_88 = lStack_98;
      plStack_80 = plStack_50;
      plStack_78 = plStack_90;
    }
    else {
      plStack_50 = (longlong *)FUN_1406149c0(param_2);
      if (plStack_50 != (longlong *)0x0) goto LAB_140617bab;
    }
    uVar6 = *(ulonglong *)(param_2 + 0x20);
    uVar4 = *(ulonglong *)(param_2 + 0x28);
  } while (uVar4 < uVar6);
LAB_140617b93:
  lStack_68 = 3;
  plStack_50 = (longlong *)FUN_1428a0920(param_2,&lStack_68);
LAB_140617bab:
  plVar12 = plStack_80;
  lVar3 = lStack_88;
  plVar13 = plVar8;
  lVar5 = lVar11;
  plVar7 = plStack_50;
  if (!SBORROW8(0,lStack_88)) {
    if (plStack_78 != (longlong *)0x0) {
      plVar8 = plStack_80 + 1;
      plVar13 = plStack_78;
      do {
        if (plVar8[-1] != 0) {
          func_0x000140613c20(*plVar8,plVar8[-1],1);
        }
        plVar8 = plVar8 + 5;
        plVar13 = (longlong *)((longlong)plVar13 + -1);
      } while (plVar13 != (longlong *)0x0);
    }
    plVar13 = plVar12;
    plVar7 = plStack_50;
    if (lVar3 != 0) {
      func_0x000140613c20(plVar12,lVar3 * 0x28,8);
      plVar7 = plStack_50;
    }
  }
LAB_140617c13:
  *(char *)(param_2 + 0x30) = *(char *)(param_2 + 0x30) + '\x01';
  lStack_68 = lVar5;
  plStack_60 = plVar7;
  plStack_58 = plVar13;
  plVar8 = (longlong *)FUN_140058450(param_2);
  if (SBORROW8(0,lVar5)) {
    plVar13 = plVar7;
    if (plVar8 != (longlong *)0x0) {
      lVar5 = *plVar8;
      plStack_50 = plVar8;
      if (lVar5 == 1) {
        FUN_1405e7a00(plVar8 + 1);
      }
      else {
LAB_140617d6a:
        if ((lVar5 == 0) && (plStack_50[2] != 0)) {
          func_0x000140613c20(plStack_50[1],plStack_50[2],1);
        }
      }
LAB_140617e61:
      func_0x000140613c20(plStack_50,0x28,8);
      plVar13 = plVar7;
    }
  }
  else {
    if (plVar8 == (longlong *)0x0) {
LAB_140617ca9:
      *param_1 = lVar5;
      param_1[1] = (longlong)plVar7;
      param_1[2] = (longlong)plVar13;
      return param_1;
    }
    if (plVar13 != (longlong *)0x0) {
      plVar12 = plVar7 + 1;
      do {
        if (plVar12[-1] != 0) {
          func_0x000140613c20(*plVar12,plVar12[-1],1);
        }
        plVar12 = plVar12 + 5;
        plVar13 = (longlong *)((longlong)plVar13 + -1);
      } while (plVar13 != (longlong *)0x0);
    }
LAB_140617da1:
    plVar13 = plVar8;
    if (lVar5 != 0) {
      func_0x000140613c20(plVar7,lVar5 * 0x28,8);
    }
  }
LAB_140617e75:
  lVar5 = FUN_1428a2200(plVar13,param_2);
LAB_140617e80:
  param_1[1] = lVar5;
  *param_1 = -0x8000000000000000;
  return param_1;
}

