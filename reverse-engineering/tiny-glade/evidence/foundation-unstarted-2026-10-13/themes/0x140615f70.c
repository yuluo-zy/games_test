
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong * FUN_140615f70(ulonglong *param_1,ulonglong param_2)

{
  char *pcVar1;
  bool bVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  longlong *plVar8;
  undefined8 uVar9;
  longlong *unaff_RBX;
  ulonglong uVar10;
  byte bVar11;
  ulonglong uVar12;
  undefined4 uVar13;
  ulonglong uVar14;
  longlong *unaff_R13;
  ulonglong uStack_b0;
  undefined1 uStack_a8;
  undefined7 uStack_a7;
  ulonglong uStack_a0;
  longlong *plStack_98;
  ulonglong uStack_90;
  undefined1 uStack_81;
  ulonglong uStack_80;
  longlong *plStack_78;
  ulonglong uStack_70;
  longlong *plStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  longlong *plStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar4 = *(ulonglong *)(param_2 + 0x20);
  uVar3 = *(ulonglong *)(param_2 + 0x28);
  if (uVar3 < uVar4) {
    plVar7 = (longlong *)(param_2 + 0x18);
    do {
      uVar3 = uVar3 + 1;
      bVar11 = *(byte *)(*(longlong *)(param_2 + 0x18) + -1 + uVar3);
      if ((0x20 < (ulonglong)bVar11) || ((0x100002600U >> ((ulonglong)bVar11 & 0x3f) & 1) == 0)) {
        if (bVar11 == 0x5b) {
          pcVar1 = (char *)(param_2 + 0x30);
          *pcVar1 = *pcVar1 + -1;
          if (*pcVar1 != '\0') {
            *(ulonglong *)(param_2 + 0x28) = uVar3;
            uStack_a8 = 1;
            uStack_b0 = param_2;
            FUN_140614f90(&uStack_80,&uStack_b0);
            unaff_R13 = plStack_78;
            uVar4 = 0x8000000000000000;
            if (uStack_80 == 2) {
              uStack_60 = 0x8000000000000000;
              plStack_50 = plStack_78;
            }
            else if (uStack_80 == 0) {
              uVar9 = 0;
LAB_1406165eb:
              uStack_60 = 0x8000000000000000;
              plStack_50 = (longlong *)FUN_1428d9bd0(uVar9,&UNK_142a0a250,&UNK_142a0a260);
            }
            else {
              FUN_140614f90(&uStack_80,&uStack_b0);
              unaff_RBX = plStack_78;
              if (uStack_80 == 2) {
                uStack_60 = 0x8000000000000000;
                plStack_50 = plStack_78;
              }
              else {
                if (uStack_80 == 0) {
                  uVar9 = 1;
                  goto LAB_1406165eb;
                }
                FUN_1406150e0(&uStack_80,&uStack_b0);
                plStack_50 = plStack_78;
                if (uStack_80 == 0x8000000000000001) {
                  uStack_60 = 0x8000000000000000;
                }
                else {
                  if (SBORROW8(0,uStack_80)) {
                    uVar9 = 2;
                    goto LAB_1406165eb;
                  }
                  uStack_60 = uStack_80;
                  uVar4 = uStack_70;
                }
              }
            }
            *(char *)(param_2 + 0x30) = *(char *)(param_2 + 0x30) + '\x01';
            plVar8 = (longlong *)FUN_1400584f0(param_2);
            plVar7 = plVar8;
            if (!SBORROW8(0,uStack_60)) goto LAB_1406167af;
            plVar7 = plStack_50;
            if (plVar8 == (longlong *)0x0) goto LAB_140616846;
            lVar5 = *plVar8;
            plStack_68 = plVar8;
            if (lVar5 != 1) goto LAB_1406167fb;
            FUN_1405e7a00(plVar8 + 1);
            goto LAB_14061682e;
          }
        }
        else {
          if (bVar11 != 0x7b) {
            plVar7 = (longlong *)FUN_1428a4690(param_2,&uStack_81,&UNK_142a0a320);
            goto LAB_140616846;
          }
          pcVar1 = (char *)(param_2 + 0x30);
          *pcVar1 = *pcVar1 + -1;
          if (*pcVar1 != '\0') {
            *(ulonglong *)(param_2 + 0x28) = uVar3;
            uVar14 = 1;
            uVar10 = 0x8000000000000000;
            bVar2 = false;
            plVar8 = (longlong *)0x0;
            if (uVar3 < uVar4) goto LAB_140616050;
            goto LAB_140616421;
          }
        }
        uStack_80 = 0x15;
        goto LAB_140615ffe;
      }
      *(ulonglong *)(param_2 + 0x28) = uVar3;
    } while (uVar3 != uVar4);
  }
  uStack_80 = 5;
LAB_140615ffe:
  uVar4 = FUN_1428a0920(param_2,&uStack_80);
  goto LAB_14061684e;
LAB_1406160b0:
  bVar11 = *(byte *)(*plVar7 + uVar3);
  if ((0x20 < (ulonglong)bVar11) || ((0x100002600U >> ((ulonglong)bVar11 & 0x3f) & 1) == 0))
  goto LAB_1406160e9;
  uVar3 = uVar3 + 1;
  *(ulonglong *)(param_2 + 0x28) = uVar3;
  if (uVar4 == uVar3) goto LAB_1406164c1;
  goto LAB_1406160b0;
LAB_1406164c1:
  uStack_80 = 5;
  uStack_58 = uVar10;
  plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
  goto LAB_14061643c;
LAB_140616050:
  do {
    uVar13 = (undefined4)uVar14;
    while( true ) {
      bVar11 = *(byte *)(*plVar7 + uVar3);
      uVar12 = (ulonglong)bVar11;
      if (0x2c < uVar12) break;
      if ((0x100002600U >> (uVar12 & 0x3f) & 1) == 0) {
        if (uVar12 == 0x2c) {
          if ((uVar14 & 1) != 0) goto LAB_1406165b7;
          uVar3 = uVar3 + 1;
          *(ulonglong *)(param_2 + 0x28) = uVar3;
          if (uVar4 <= uVar3) goto LAB_1406164c1;
          goto LAB_1406160b0;
        }
        break;
      }
      uVar3 = uVar3 + 1;
      *(ulonglong *)(param_2 + 0x28) = uVar3;
      if (uVar4 == uVar3) goto LAB_140616421;
    }
    if (bVar11 == 0x7d) {
      if (((ulonglong)plVar8 & 1) == 0) {
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428d9b60(&UNK_142a0a220,5);
        goto LAB_14061643c;
      }
      if (!bVar2) {
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428d9b60(&UNK_142a0a225,3);
        goto LAB_14061643c;
      }
      uVar4 = uStack_90;
      uVar3 = uVar10;
      plVar6 = plStack_98;
      if (SBORROW8(0,uVar10)) {
        uStack_58 = 0x8000000000000000;
        plVar6 = (longlong *)FUN_1428d9b60(&UNK_142a0a228,3);
        uStack_60 = 0x8000000000000000;
        uVar4 = uStack_90;
        uVar3 = uStack_60;
      }
      goto LAB_140616795;
    }
    if ((uVar14 & 1) == 0) {
      uStack_80 = 8;
      uStack_58 = uVar10;
      plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
      goto LAB_14061643c;
    }
    uVar13 = 0;
LAB_1406160e9:
    if (bVar11 != 0x22) {
      if (bVar11 == 0x7d) {
        uStack_80 = 0x12;
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
      }
      else {
LAB_1406165b7:
        uStack_80 = 0x10;
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
      }
      goto LAB_14061643c;
    }
    uStack_60 = CONCAT44(uStack_60._4_4_,uVar13);
    *(ulonglong *)(param_2 + 0x28) = uVar3 + 1;
    *(undefined8 *)(param_2 + 0x10) = 0;
    uStack_58 = uVar10;
    plStack_50 = plVar8;
    FUN_140b89af0(&uStack_80,plVar7,param_2);
    plVar6 = plStack_78;
    if ((int)uStack_80 == 2) goto LAB_14061643c;
    if (uStack_70 == 3) {
      if (*(char *)((longlong)plStack_78 + 2) != DAT_142b59117 ||
          (short)*plStack_78 != _DAT_142b59115) {
        if (*(char *)((longlong)plStack_78 + 2) != DAT_142b5911a ||
            (short)*plStack_78 != _DAT_142b59118) goto LAB_1406161ec;
        if (!SBORROW8(0,uVar10)) {
          uStack_58 = uVar10;
          plStack_50 = (longlong *)FUN_1428d9c60(&UNK_142a0a228,3);
          goto LAB_14061676c;
        }
        uVar4 = *(ulonglong *)(param_2 + 0x28);
        if (uVar4 < *(ulonglong *)(param_2 + 0x20)) {
          while( true ) {
            uVar4 = uVar4 + 1;
            uVar3 = (ulonglong)*(byte *)(*plVar7 + -1 + uVar4);
            if (0x3a < uVar3) goto LAB_140616876;
            if ((0x100002600U >> (uVar3 & 0x3f) & 1) == 0) break;
            *(ulonglong *)(param_2 + 0x28) = uVar4;
            if (uVar4 == *(ulonglong *)(param_2 + 0x20)) goto LAB_14061667b;
          }
          if (uVar3 == 0x3a) {
            *(ulonglong *)(param_2 + 0x28) = uVar4;
            uStack_58 = 0x8000000000000000;
            FUN_140615e10(&uStack_b0,param_2);
            plStack_98 = (longlong *)CONCAT71(uStack_a7,uStack_a8);
            if (!SBORROW8(0,uStack_b0)) {
              uStack_90 = uStack_a0;
              uVar10 = uStack_b0;
              plVar8 = plStack_50;
              goto LAB_140616208;
            }
          }
          else {
LAB_140616876:
            uStack_80 = 6;
            uStack_58 = 0x8000000000000000;
            plStack_98 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
          }
        }
        else {
LAB_14061667b:
          uStack_80 = 3;
          uStack_58 = 0x8000000000000000;
          plStack_98 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
        }
        uStack_60 = 0x8000000000000000;
        uVar4 = uStack_90;
        uVar3 = uStack_60;
        plVar6 = plStack_98;
        goto LAB_140616795;
      }
      if (bVar2) {
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428d9c60(&UNK_142a0a225,3);
        goto LAB_14061643c;
      }
      if (*(ulonglong *)(param_2 + 0x20) <= *(ulonglong *)(param_2 + 0x28)) {
LAB_14061665e:
        uStack_80 = 3;
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
        goto LAB_14061643c;
      }
      plVar6 = (longlong *)*plVar7;
      uVar4 = *(ulonglong *)(param_2 + 0x28);
      while( true ) {
        uVar3 = uVar4 + 1;
        uVar4 = (ulonglong)*(byte *)((longlong)plVar6 + uVar4);
        if (0x3a < uVar4) goto LAB_140616715;
        if ((0x100002600U >> (uVar4 & 0x3f) & 1) == 0) break;
        *(ulonglong *)(param_2 + 0x28) = uVar3;
        uVar4 = uVar3;
        if (uVar3 == *(ulonglong *)(param_2 + 0x20)) goto LAB_14061665e;
      }
      if (uVar4 != 0x3a) {
LAB_140616715:
        uStack_80 = 6;
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
        goto LAB_14061643c;
      }
      *(ulonglong *)(param_2 + 0x28) = uVar3;
      uStack_58 = uVar10;
      lVar5 = FUN_1406157a0(param_2);
      if (lVar5 != 0) goto LAB_14061643c;
      bVar2 = true;
      plVar8 = plStack_50;
      plStack_68 = plVar6;
    }
    else if ((uStack_70 == 5) &&
            (*(char *)((longlong)plStack_78 + 4) == DAT_142b59114 &&
             (int)*plStack_78 == _DAT_142b59110)) {
      if (plStack_50 != (longlong *)0x0) {
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428d9c60(&UNK_142a0a220,5);
        goto LAB_14061643c;
      }
      if (*(ulonglong *)(param_2 + 0x20) <= *(ulonglong *)(param_2 + 0x28)) {
LAB_140616641:
        uStack_80 = 3;
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
        goto LAB_14061643c;
      }
      plVar6 = (longlong *)*plVar7;
      uVar4 = *(ulonglong *)(param_2 + 0x28);
      while( true ) {
        uVar3 = uVar4 + 1;
        uVar4 = (ulonglong)*(byte *)((longlong)plVar6 + uVar4);
        if (0x3a < uVar4) goto LAB_140616732;
        if ((0x100002600U >> (uVar4 & 0x3f) & 1) == 0) break;
        *(ulonglong *)(param_2 + 0x28) = uVar3;
        uVar4 = uVar3;
        if (uVar3 == *(ulonglong *)(param_2 + 0x20)) goto LAB_140616641;
      }
      if (uVar4 != 0x3a) {
LAB_140616732:
        uStack_80 = 6;
        uStack_58 = uVar10;
        plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
        goto LAB_14061643c;
      }
      *(ulonglong *)(param_2 + 0x28) = uVar3;
      uStack_58 = uVar10;
      lVar5 = FUN_1406157a0(param_2);
      if (lVar5 != 0) goto LAB_14061643c;
      plVar8 = (longlong *)0x1;
      unaff_R13 = plVar6;
    }
    else {
LAB_1406161ec:
      uStack_58 = uVar10;
      plVar6 = (longlong *)FUN_1406149c0(param_2);
      plVar8 = plStack_50;
      if (plVar6 != (longlong *)0x0) goto LAB_14061643c;
    }
LAB_140616208:
    uVar4 = *(ulonglong *)(param_2 + 0x20);
    uVar3 = *(ulonglong *)(param_2 + 0x28);
    uVar14 = uStack_60 & 0xffffffff;
  } while (uVar3 < uVar4);
LAB_140616421:
  uStack_80 = 3;
  uStack_58 = uVar10;
  plVar6 = (longlong *)FUN_1428a0920(param_2,&uStack_80);
LAB_14061643c:
  plStack_50 = plVar6;
  if (SBORROW8(0,uVar10)) {
    uStack_60 = 0x8000000000000000;
    uVar4 = uStack_90;
    uVar3 = uStack_60;
  }
  else {
LAB_14061676c:
    uStack_60 = 0x8000000000000000;
    uVar4 = uStack_90;
    uVar3 = uStack_60;
    plVar6 = plStack_50;
    if (uVar10 != 0) {
      func_0x000140613c20(plStack_98,uVar10,1);
      uVar3 = uStack_60;
      plVar6 = plStack_50;
    }
  }
LAB_140616795:
  plStack_50 = plVar6;
  uStack_60 = uVar3;
  unaff_RBX = plStack_68;
  *(char *)(param_2 + 0x30) = *(char *)(param_2 + 0x30) + '\x01';
  plVar8 = (longlong *)FUN_140058450(param_2);
  plVar7 = plVar8;
  if (SBORROW8(0,uStack_60)) {
    plVar7 = plStack_50;
    if (plVar8 != (longlong *)0x0) {
      lVar5 = *plVar8;
      plStack_68 = plVar8;
      if (lVar5 == 1) {
        FUN_1405e7a00(plVar8 + 1);
      }
      else {
LAB_1406167fb:
        if ((lVar5 == 0) && (plStack_68[2] != 0)) {
          func_0x000140613c20(plStack_68[1],plStack_68[2],1);
        }
      }
LAB_14061682e:
      func_0x000140613c20(plStack_68,0x28,8);
      plVar7 = plStack_50;
    }
  }
  else {
LAB_1406167af:
    if (plVar7 == (longlong *)0x0) {
      param_1[2] = uVar4;
      param_1[3] = (ulonglong)unaff_R13;
      param_1[4] = (ulonglong)unaff_RBX;
      *param_1 = uStack_60;
      param_1[1] = (ulonglong)plStack_50;
      return param_1;
    }
    if (uStack_60 != 0) {
      func_0x000140613c20(plStack_50,uStack_60,1);
    }
  }
LAB_140616846:
  uVar4 = FUN_1428a2200(plVar7,param_2);
LAB_14061684e:
  param_1[1] = uVar4;
  *param_1 = 0x8000000000000000;
  return param_1;
}

