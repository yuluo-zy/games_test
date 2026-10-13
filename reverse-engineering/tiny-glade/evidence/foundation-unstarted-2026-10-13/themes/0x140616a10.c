
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong * FUN_140616a10(longlong *param_1,longlong *param_2)

{
  longlong lVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong *plVar5;
  longlong *plVar6;
  byte bVar7;
  ulonglong uVar8;
  longlong *plVar9;
  longlong *unaff_R13;
  longlong *unaff_R14;
  longlong *plVar10;
  longlong *plVar11;
  longlong *unaff_R15;
  longlong *plVar12;
  bool bVar13;
  longlong *plStack_e0;
  longlong *plStack_d8;
  longlong *plStack_d0;
  longlong *plStack_c8;
  longlong *plStack_c0;
  longlong *plStack_b8;
  undefined1 uStack_a9;
  longlong *plStack_a8;
  longlong *plStack_a0;
  longlong *plStack_98;
  longlong *plStack_90;
  longlong *plStack_88;
  longlong *plStack_70;
  longlong *plStack_68;
  longlong *plStack_60;
  longlong *plStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar4 = param_2[4];
  if ((ulonglong)param_2[5] < uVar4) {
    plVar11 = param_2 + 3;
    plVar9 = (longlong *)0x100002600;
    uVar8 = param_2[5];
    do {
      uVar2 = uVar8 + 1;
      bVar7 = *(byte *)(param_2[3] + -1 + uVar2);
      if ((0x20 < (ulonglong)bVar7) || ((0x100002600U >> ((ulonglong)bVar7 & 0x3f) & 1) == 0)) {
        if (bVar7 == 0x5b) {
          plVar11 = param_2 + 6;
          *(char *)plVar11 = (char)*plVar11 + -1;
          if ((char)*plVar11 != '\0') {
            param_2[5] = uVar2;
            plVar11 = unaff_R14;
            plStack_c8 = param_2;
            if (uVar4 <= uVar2) goto LAB_140616e25;
            lVar3 = uVar8 + 2;
            goto LAB_140616e00;
          }
        }
        else {
          if (bVar7 != 0x7b) {
            plVar5 = (longlong *)FUN_1428a4690(param_2,&uStack_a9,&UNK_142a0a340);
            goto LAB_1406174a1;
          }
          plVar9 = param_2 + 6;
          *(char *)plVar9 = (char)*plVar9 + -1;
          if ((char)*plVar9 != '\0') {
            param_2[5] = uVar2;
            plStack_c8 = (longlong *)0x8000000000000000;
            bVar13 = true;
            plStack_58 = (longlong *)0x8000000000000000;
            plVar10 = unaff_R13;
            if (uVar2 < uVar4) goto LAB_140616aee;
            goto LAB_140616e74;
          }
        }
        plStack_a8 = (longlong *)0x15;
        goto LAB_140616a9e;
      }
      param_2[5] = uVar2;
      uVar8 = uVar2;
    } while (uVar2 != uVar4);
  }
  plStack_a8 = (longlong *)0x5;
LAB_140616a9e:
  lVar3 = FUN_1428a0920(param_2,&plStack_a8);
  goto LAB_1406174a9;
LAB_140616e00:
  bVar7 = *(byte *)(param_2[3] + -1 + lVar3);
  if (((ulonglong)bVar7 < 0x21) && ((0x100002600U >> ((ulonglong)bVar7 & 0x3f) & 1) != 0))
  goto code_r0x000140616e11;
  if (bVar7 == 0x5d) {
    plStack_60 = (longlong *)FUN_1428d9bd0(0,&UNK_142a0a3b0,&UNK_142a0a260);
    goto LAB_140616f89;
  }
  plStack_c0 = (longlong *)((ulonglong)plStack_c0 & 0xffffffffffffff00);
  FUN_140615b70(&plStack_a8,param_2);
  plVar11 = plStack_98;
  unaff_R13 = plStack_a0;
  plVar6 = plStack_a8;
  plStack_60 = plStack_a0;
  if (SBORROW8(0,(longlong)plStack_a8)) {
    plVar11 = unaff_R14;
    goto LAB_140616f89;
  }
  plStack_e0 = plStack_a8;
  plStack_d8 = plStack_a0;
  plStack_d0 = plStack_98;
  FUN_1406150e0(&plStack_a8,&plStack_c8);
  plVar9 = plStack_a0;
  unaff_R15 = plStack_a8;
  if (plStack_a8 != (longlong *)0x8000000000000001) {
    plVar10 = plStack_98;
    if (!SBORROW8(0,(longlong)plStack_a8)) goto LAB_140616f93;
    plVar9 = (longlong *)FUN_1428d9bd0(1,&UNK_142a0a3b0,&UNK_142a0a260);
  }
  if (plVar11 != (longlong *)0x0) {
    unaff_R15 = unaff_R13 + 4;
    plVar10 = plVar11;
    do {
      if (unaff_R15[-4] != 0) {
        func_0x000140613c20(unaff_R15[-3],unaff_R15[-4],1);
      }
      if (unaff_R15[-1] != 0) {
        func_0x000140613c20(*unaff_R15,unaff_R15[-1],1);
      }
      unaff_R15 = unaff_R15 + 6;
      plVar10 = (longlong *)((longlong)plVar10 + -1);
      plVar11 = (longlong *)0x0;
    } while (plVar10 != (longlong *)0x0);
  }
  plStack_60 = plVar9;
  if (plVar6 != (longlong *)0x0) {
    func_0x000140613c20(unaff_R13,(longlong)plVar6 * 0x30,8);
  }
  goto LAB_140616f89;
code_r0x000140616e11:
  param_2[5] = lVar3;
  lVar1 = (1 - uVar4) + lVar3;
  lVar3 = lVar3 + 1;
  if (lVar1 == 1) goto LAB_140616e25;
  goto LAB_140616e00;
LAB_140616e25:
  plStack_a8 = (longlong *)0x2;
  plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
LAB_140616f89:
  plVar6 = (longlong *)0x8000000000000000;
  plVar10 = unaff_R13;
LAB_140616f93:
  *(char *)(param_2 + 6) = (char)param_2[6] + '\x01';
  plStack_a0 = plStack_60;
  plStack_a8 = plVar6;
  plStack_98 = plVar11;
  plStack_90 = unaff_R15;
  plStack_88 = plVar9;
  plVar12 = (longlong *)FUN_1400584f0(param_2);
  plVar5 = plStack_60;
  if (!SBORROW8(0,(longlong)plVar6)) {
    if (plVar12 == (longlong *)0x0) goto LAB_140617385;
    plStack_58 = plVar12;
    if (plVar11 != (longlong *)0x0) {
      plVar10 = plStack_60 + 4;
      do {
        if (plVar10[-4] != 0) {
          func_0x000140613c20(plVar10[-3],plVar10[-4],1);
        }
        if (plVar10[-1] != 0) {
          func_0x000140613c20(*plVar10,plVar10[-1],1);
        }
        plVar10 = plVar10 + 6;
        plVar11 = (longlong *)((longlong)plVar11 + -1);
      } while (plVar11 != (longlong *)0x0);
    }
    goto LAB_1406173bf;
  }
  if (plVar12 != (longlong *)0x0) {
    lVar3 = *plVar12;
    plStack_58 = plVar12;
    if (lVar3 != 1) goto LAB_14061740b;
    FUN_1405e7a00(plVar12 + 1);
    goto LAB_14061748a;
  }
  goto LAB_1406174a1;
LAB_140616b50:
  bVar7 = *(byte *)(*plVar11 + uVar2);
  if ((0x20 < (ulonglong)bVar7) || ((0x100002600U >> ((ulonglong)bVar7 & 0x3f) & 1) == 0))
  goto LAB_140616b89;
  uVar2 = uVar2 + 1;
  param_2[5] = uVar2;
  if (uVar4 == uVar2) goto LAB_14061705f;
  goto LAB_140616b50;
LAB_14061705f:
  plStack_a8 = (longlong *)0x5;
  plStack_68 = plStack_58;
  uStack_49 = 1;
  plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
  goto LAB_140616e98;
LAB_140616aee:
  do {
    while( true ) {
      bVar7 = *(byte *)(*plVar11 + uVar2);
      uVar8 = (ulonglong)bVar7;
      if (0x2c < uVar8) break;
      if ((0x100002600U >> (uVar8 & 0x3f) & 1) == 0) {
        if (uVar8 == 0x2c) {
          if (bVar13) goto LAB_14061719d;
          uVar2 = uVar2 + 1;
          param_2[5] = uVar2;
          if (uVar4 <= uVar2) goto LAB_14061705f;
          goto LAB_140616b50;
        }
        break;
      }
      uVar2 = uVar2 + 1;
      param_2[5] = uVar2;
      if (uVar4 == uVar2) goto LAB_140616e74;
    }
    if (bVar7 == 0x7d) {
      bVar13 = SBORROW8(0,(longlong)plStack_c8);
      if (bVar13) {
        plStack_68 = plStack_58;
        uStack_49 = 1;
        plStack_60 = (longlong *)FUN_1428d9b60(&UNK_142a0a3c0,5);
      }
      else {
        plStack_98 = plStack_b8;
        plStack_a8 = plStack_c8;
        plStack_a0 = plStack_c0;
        plVar6 = plStack_c8;
        plVar9 = plStack_70;
        plVar11 = plStack_b8;
        unaff_R15 = plStack_58;
        plVar5 = plStack_a0;
        if (!SBORROW8(0,(longlong)plStack_58)) goto LAB_140617351;
        plStack_60 = (longlong *)FUN_1428d9b60(&UNK_142a0a3c5,3);
        plStack_68 = (longlong *)0x8000000000000000;
        uStack_49 = 0;
        FUN_1405d1240(&plStack_a8);
      }
      goto LAB_140616e9a;
    }
    if (!bVar13) {
      plStack_a8 = (longlong *)0x8;
      plStack_68 = plStack_58;
      uStack_49 = 1;
      plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
      goto LAB_140616e98;
    }
    bVar13 = false;
LAB_140616b89:
    if (bVar7 != 0x22) {
      if (bVar7 == 0x7d) {
        plStack_a8 = (longlong *)0x12;
        plStack_68 = plStack_58;
        uStack_49 = 1;
        plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
      }
      else {
LAB_14061719d:
        plStack_a8 = (longlong *)0x10;
        plStack_68 = plStack_58;
        uStack_49 = 1;
        plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
      }
      goto LAB_140616e98;
    }
    param_2[5] = uVar2 + 1;
    param_2[2] = 0;
    plStack_68 = plStack_58;
    uStack_49 = 1;
    FUN_140b89af0(&plStack_a8,plVar11,param_2);
    plStack_60 = plStack_a0;
    if ((int)plStack_a8 == 2) goto LAB_140616e98;
    plVar9 = plVar10;
    if (plStack_98 == (longlong *)0x3) {
      if (*(char *)((longlong)plStack_a0 + 2) == DAT_142b59457 &&
          (short)*plStack_a0 == _DAT_142b59455) {
        if (!SBORROW8(0,(longlong)plStack_58)) {
          plStack_68 = plStack_58;
          uStack_49 = 1;
          plStack_60 = (longlong *)FUN_1428d9c60(&UNK_142a0a3c5,3);
          bVar13 = true;
          goto LAB_14061728e;
        }
        uVar4 = param_2[5];
        if (uVar4 < (ulonglong)param_2[4]) {
          while( true ) {
            uVar4 = uVar4 + 1;
            uVar8 = (ulonglong)*(byte *)(*plVar11 + -1 + uVar4);
            if (0x3a < uVar8) goto LAB_140617520;
            if ((0x100002600U >> (uVar8 & 0x3f) & 1) == 0) break;
            param_2[5] = uVar4;
            if (uVar4 == param_2[4]) goto LAB_1406171ef;
          }
          if (uVar8 == 0x3a) {
            param_2[5] = uVar4;
            plStack_68 = (longlong *)0x8000000000000000;
            uStack_49 = 1;
            FUN_140615e10(&plStack_e0,param_2);
            plStack_58 = plStack_e0;
            plVar9 = plStack_d0;
            plVar6 = plStack_d8;
            if (!SBORROW8(0,(longlong)plStack_e0)) goto LAB_140616ca0;
            plStack_60 = plStack_d8;
          }
          else {
LAB_140617520:
            plStack_a8 = (longlong *)0x6;
            plStack_68 = (longlong *)0x8000000000000000;
            uStack_49 = 1;
            plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
          }
        }
        else {
LAB_1406171ef:
          plStack_a8 = (longlong *)0x3;
          plStack_68 = (longlong *)0x8000000000000000;
          uStack_49 = 1;
          plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
        }
        bVar13 = true;
        unaff_R15 = (longlong *)0x8000000000000000;
        plVar9 = plStack_70;
        goto LAB_1406172a9;
      }
LAB_140616c7d:
      plStack_68 = plStack_58;
      uStack_49 = 1;
      plStack_60 = (longlong *)FUN_1406149c0(param_2);
      plVar6 = plStack_70;
      if (plStack_60 != (longlong *)0x0) goto LAB_140616e98;
    }
    else {
      if ((plStack_98 != (longlong *)0x5) ||
         (*(char *)((longlong)plStack_a0 + 4) != DAT_142b59454 || (int)*plStack_a0 != _DAT_142b59450
         )) goto LAB_140616c7d;
      if (!SBORROW8(0,(longlong)plStack_c8)) {
        plStack_68 = plStack_58;
        uStack_49 = 1;
        plStack_60 = (longlong *)FUN_1428d9c60(&UNK_142a0a3c0,5);
        goto LAB_140616e98;
      }
      uVar4 = param_2[5];
      if ((ulonglong)param_2[4] <= uVar4) {
LAB_1406171c6:
        plStack_a8 = (longlong *)0x3;
        plStack_68 = plStack_58;
        uStack_49 = 1;
        plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
        goto LAB_140616e98;
      }
      while( true ) {
        uVar4 = uVar4 + 1;
        uVar8 = (ulonglong)*(byte *)(*plVar11 + -1 + uVar4);
        if (0x3a < uVar8) goto LAB_1406174f7;
        if ((0x100002600U >> (uVar8 & 0x3f) & 1) == 0) break;
        param_2[5] = uVar4;
        if (uVar4 == param_2[4]) goto LAB_1406171c6;
      }
      if (uVar8 != 0x3a) {
LAB_1406174f7:
        plStack_a8 = (longlong *)0x6;
        plStack_68 = plStack_58;
        uStack_49 = 1;
        plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
        goto LAB_140616e98;
      }
      param_2[5] = uVar4;
      plStack_68 = plStack_58;
      uStack_49 = 1;
      FUN_140615b70(&plStack_e0,param_2);
      plVar5 = plStack_d0;
      plVar6 = plStack_e0;
      plStack_60 = plStack_d8;
      if (SBORROW8(0,(longlong)plStack_e0)) goto LAB_140616e98;
      if (!SBORROW8(0,(longlong)plStack_c8)) {
        FUN_1405d1240(&plStack_c8);
      }
      plStack_c8 = plVar6;
      plStack_c0 = plStack_60;
      plStack_b8 = plVar5;
      plVar6 = plStack_70;
    }
LAB_140616ca0:
    plStack_70 = plVar6;
    uVar4 = param_2[4];
    uVar2 = param_2[5];
    plVar10 = plVar9;
  } while (uVar2 < uVar4);
LAB_140616e74:
  plStack_a8 = (longlong *)0x3;
  plStack_68 = plStack_58;
  uStack_49 = 1;
  plStack_60 = (longlong *)FUN_1428a0920(param_2,&plStack_a8);
LAB_140616e98:
  bVar13 = true;
LAB_140616e9a:
  if (SBORROW8(0,(longlong)plStack_58)) {
    unaff_R15 = (longlong *)0x8000000000000000;
    plVar9 = plStack_70;
  }
  else {
LAB_14061728e:
    unaff_R15 = plStack_58;
    plVar9 = plStack_70;
    if (plStack_58 == (longlong *)0x0) {
      unaff_R15 = (longlong *)0x0;
    }
    else {
      func_0x000140613c20(plStack_70,plStack_58,1);
    }
  }
LAB_1406172a9:
  plVar5 = plStack_c0;
  plVar6 = plStack_c8;
  plVar11 = plVar6;
  if ((bool)(!SBORROW8(0,(longlong)plStack_c8) & bVar13)) {
    plStack_58 = param_1;
    if (plStack_b8 != (longlong *)0x0) {
      plVar9 = plStack_c0 + 4;
      plVar12 = plStack_b8;
      do {
        if (plVar9[-4] != 0) {
          func_0x000140613c20(plVar9[-3],plVar9[-4],1);
        }
        if (plVar9[-1] != 0) {
          func_0x000140613c20(*plVar9,plVar9[-1],1);
        }
        plVar9 = plVar9 + 6;
        plVar12 = (longlong *)((longlong)plVar12 + -1);
      } while (plVar12 != (longlong *)0x0);
    }
    param_1 = plStack_58;
    plVar9 = plStack_70;
    if (plVar6 != (longlong *)0x0) {
      plVar11 = (longlong *)((longlong)plVar6 * 0x10);
      func_0x000140613c20(plVar5,(longlong)plVar6 * 0x30,8);
    }
  }
  plVar6 = (longlong *)0x8000000000000000;
  plVar5 = plStack_60;
LAB_140617351:
  plStack_60 = plVar5;
  *(char *)(param_2 + 6) = (char)param_2[6] + '\x01';
  plStack_a0 = plStack_60;
  plStack_a8 = plVar6;
  plStack_98 = plVar11;
  plStack_90 = unaff_R15;
  plStack_88 = plVar9;
  plVar12 = (longlong *)FUN_140058450(param_2);
  plVar5 = plStack_60;
  if (SBORROW8(0,(longlong)plVar6)) {
    if (plVar12 != (longlong *)0x0) {
      lVar3 = *plVar12;
      plStack_58 = plVar12;
      if (lVar3 == 1) {
        FUN_1405e7a00(plVar12 + 1);
      }
      else {
LAB_14061740b:
        plVar5 = plStack_60;
        if ((lVar3 == 0) && (plStack_58[2] != 0)) {
          func_0x000140613c20(plStack_58[1],plStack_58[2],1);
        }
      }
LAB_14061748a:
      func_0x000140613c20(plStack_58,0x28,8);
    }
  }
  else {
    if (plVar12 == (longlong *)0x0) {
LAB_140617385:
      param_1[2] = (longlong)plVar11;
      param_1[3] = (longlong)unaff_R15;
      param_1[4] = (longlong)plVar9;
      param_1[5] = (longlong)plVar10;
      *param_1 = (longlong)plVar6;
      param_1[1] = (longlong)plStack_60;
      return param_1;
    }
    plStack_58 = plVar12;
    if (plVar11 != (longlong *)0x0) {
      plVar10 = plStack_60 + 4;
      do {
        if (plVar10[-4] != 0) {
          func_0x000140613c20(plVar10[-3],plVar10[-4],1);
        }
        if (plVar10[-1] != 0) {
          func_0x000140613c20(*plVar10,plVar10[-1],1);
        }
        plVar10 = plVar10 + 6;
        plVar11 = (longlong *)((longlong)plVar11 + -1);
      } while (plVar11 != (longlong *)0x0);
    }
LAB_1406173bf:
    if (plVar6 != (longlong *)0x0) {
      func_0x000140613c20(plStack_60,(longlong)plVar6 * 0x30,8);
    }
    plVar5 = plStack_58;
    if (unaff_R15 != (longlong *)0x0) {
      func_0x000140613c20(plVar9,unaff_R15,1);
      plVar5 = plStack_58;
    }
  }
LAB_1406174a1:
  lVar3 = FUN_1428a2200(plVar5,param_2);
LAB_1406174a9:
  param_1[1] = lVar3;
  *param_1 = -0x8000000000000000;
  return param_1;
}

