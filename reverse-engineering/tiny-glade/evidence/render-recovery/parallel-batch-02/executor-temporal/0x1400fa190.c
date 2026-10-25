
undefined8
FUN_1400fa190(undefined8 param_1,longlong *param_2,longlong param_3,longlong *param_4,
             undefined8 param_5)

{
  byte *pbVar1;
  byte *pbVar2;
  undefined4 *puVar3;
  byte bVar4;
  byte bVar5;
  code *pcVar6;
  undefined8 uVar7;
  char cVar8;
  undefined *puVar9;
  longlong *plVar10;
  uint uVar11;
  undefined *puVar12;
  undefined4 *puVar13;
  longlong lVar14;
  longlong lVar15;
  longlong lVar16;
  bool bVar17;
  undefined1 auStack_2d0 [72];
  longlong lStack_288;
  longlong lStack_280;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined6 uStack_268;
  char cStack_262;
  undefined1 uStack_261;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  longlong lStack_1e8;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  longlong lStack_1d8;
  char cStack_1d0;
  undefined3 uStack_1cf;
  undefined1 uStack_1cc;
  undefined3 uStack_1cb;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  char cStack_d8;
  undefined4 uStack_d7;
  undefined3 uStack_d3;
  longlong *plStack_b8;
  longlong lStack_b0;
  undefined8 uStack_a8;
  longlong lStack_a0;
  undefined8 uStack_98;
  longlong *plStack_90;
  longlong lStack_88;
  longlong lStack_80;
  longlong *plStack_78;
  longlong *plStack_70;
  undefined4 *puStack_68;
  longlong lStack_60;
  longlong *plStack_58;
  undefined1 uStack_50;
  undefined1 uStack_4f;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_4c = 1;
  uStack_4d = 1;
  plStack_70 = param_4;
  lStack_60 = param_3;
  cVar8 = func_0x000140c16a00();
  if (cVar8 == '\0') {
    lStack_a0 = 0;
  }
  else {
    puVar9 = &UNK_14294c49a;
    puVar12 = &UNK_14294c478;
    do {
      pbVar1 = puVar9 + -1;
      uVar11 = (uint)(char)*pbVar1;
      if ((int)uVar11 < 0) {
        bVar4 = puVar9[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar9[-3];
          if ((char)bVar5 < -0x40) {
            pbVar2 = puVar9 + -4;
            puVar9 = puVar9 + -4;
            uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
          }
          else {
            puVar9 = puVar9 + -3;
            uVar11 = bVar5 & 0xf;
          }
          uVar11 = bVar4 & 0x3f | uVar11 << 6;
        }
        else {
          puVar9 = puVar9 + -2;
          uVar11 = bVar4 & 0x1f;
        }
        uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto code_r0x0001400fa279;
code_r0x0001400fa295:
        puVar12 = puVar9 + -0x14294c477;
        if (puVar12 != (undefined *)0x0) {
          if (puVar12 < (undefined *)0x22) {
            if (-0x41 < (char)puVar9[1]) goto code_r0x0001400fa2ec;
          }
          else if (puVar12 == (undefined *)0x22) goto code_r0x0001400fa2ec;
          uStack_4c = 1;
          uStack_4d = 1;
          FUN_1428d9940(&UNK_14294c478,0x22,puVar12,0x22,&UNK_14294c120);
          goto code_r0x0001400faed7;
        }
code_r0x0001400fa2ec:
        lVar14 = 0x21 - (longlong)(puVar9 + -0x14294c478);
        puVar12 = puVar9 + 1;
        goto code_r0x0001400fa2f7;
      }
      puVar9 = puVar9 + -1;
      if (uVar11 == 0x5c) goto code_r0x0001400fa295;
code_r0x0001400fa279:
      if (uVar11 == 0x2f) goto code_r0x0001400fa295;
    } while (puVar9 != &UNK_14294c478);
    lVar14 = 0x22;
code_r0x0001400fa2f7:
    plVar10 = (longlong *)func_0x000140c18130();
    if (*plVar10 == 0) {
      uStack_4c = 1;
      uStack_4d = 1;
      plVar10 = (longlong *)FUN_1428a1640(plVar10,0);
      if (plVar10 == (longlong *)0x0) goto code_r0x0001400fae77;
    }
    else {
      if ((int)*plVar10 != 1) {
code_r0x0001400fae77:
        uStack_4c = 1;
        uStack_4d = 1;
        FUN_1428d80d0(&UNK_142932a10);
        goto code_r0x0001400faed7;
      }
      plVar10 = plVar10 + 1;
    }
    if (*plVar10 != 0) {
      uStack_4c = 1;
      uStack_4d = 1;
      FUN_1428d9250(&UNK_142932ab8);
      goto code_r0x0001400faed7;
    }
    *plVar10 = -1;
    plStack_58 = plVar10;
    uStack_98 = FUN_140c16d40(plVar10 + 1,&UNK_14294c49a,0x14,puVar12,lVar14,1,0);
    *plStack_58 = *plStack_58 + 1;
    lStack_a0 = 1;
  }
  lStack_88 = param_2[0x2f];
  uStack_4e = 1;
  uStack_4f = 1;
  uStack_a8 = param_1;
  func_0x000140be95e0(&uStack_1f8);
  lVar14 = *(longlong *)(lStack_60 + 0xa0);
  if ((lVar14 != 0) && ((*(byte *)(param_2 + 0x31) & 1) != 0)) {
    lVar16 = 0;
    do {
      FUN_140be9600(&uStack_1f8,lVar16);
      lVar16 = lVar16 + 1;
    } while (lVar16 != lVar14);
  }
  FUN_140be9600(&uStack_1f8,lVar14);
  lVar14 = lStack_1e8;
  if (uStack_1f0 + lStack_1e8 * 6 != (undefined4 *)0x18 && lStack_1e8 != 0) {
    *(undefined1 *)(uStack_1f0 + lStack_1e8 * 6 + -2) = 1;
  }
  puStack_68 = uStack_1f0;
  lStack_80 = uStack_1f8;
  lVar16 = plStack_70[5];
  if ((char)lVar16 != '\x02') {
    uStack_49 = 1;
    uStack_4a = 1;
    uStack_4b = 1;
    cVar8 = func_0x000140c16a00();
    plStack_58 = (longlong *)CONCAT71(plStack_58._1_7_,cVar8);
    if (cVar8 == '\0') {
      uStack_1f8 = 0;
    }
    else {
      puVar9 = &UNK_14294c49a;
      puVar12 = &UNK_14294c478;
      do {
        pbVar1 = puVar9 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar9[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar9[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar9 + -4;
              puVar9 = puVar9 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar9 = puVar9 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar9 = puVar9 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x0001400fa549;
code_r0x0001400fa565:
          puVar12 = puVar9 + -0x14294c477;
          if (puVar12 != (undefined *)0x0) {
            if (puVar12 < (undefined *)0x22) {
              if (-0x41 < (char)puVar9[1]) goto code_r0x0001400fa5c3;
            }
            else if (puVar12 == (undefined *)0x22) goto code_r0x0001400fa5c3;
            uStack_49 = 1;
            uStack_4a = 1;
            uStack_4b = 1;
            FUN_1428d9940(&UNK_14294c478,0x22,puVar12,0x22,&UNK_14294c120);
            goto code_r0x0001400faed7;
          }
code_r0x0001400fa5c3:
          lVar15 = 0x21 - (longlong)(puVar9 + -0x14294c478);
          puVar12 = puVar9 + 1;
          goto code_r0x0001400fa5ce;
        }
        puVar9 = puVar9 + -1;
        if (uVar11 == 0x5c) goto code_r0x0001400fa565;
code_r0x0001400fa549:
        if (uVar11 == 0x2f) goto code_r0x0001400fa565;
      } while (puVar9 != &UNK_14294c478);
      lVar15 = 0x22;
code_r0x0001400fa5ce:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        uStack_49 = 1;
        uStack_4a = 1;
        uStack_4b = 1;
        plVar10 = (longlong *)FUN_1428a1640(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x0001400fae93;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x0001400fae93:
          uStack_49 = 1;
          uStack_4a = 1;
          uStack_4b = 1;
          FUN_1428d80d0(&UNK_142932a10);
          goto code_r0x0001400faed7;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        uStack_49 = 1;
        uStack_4a = 1;
        uStack_4b = 1;
        FUN_1428d9250(&UNK_142932ab8);
        goto code_r0x0001400faed7;
      }
      *plVar10 = -1;
      plStack_78 = plVar10;
      uStack_1f0 = (undefined4 *)FUN_140c16d40(plVar10 + 1,&UNK_14294c4ae,0x14,puVar12,lVar15,1,0);
      *plStack_78 = *plStack_78 + 1;
      uStack_1f8 = 1;
    }
    FUN_140bebdf0(param_2,lStack_88);
    if ((char)plStack_58 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        uStack_49 = 1;
        uStack_4a = 1;
        uStack_4b = 1;
        plVar10 = (longlong *)FUN_1428a1640(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x0001400faeb6;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x0001400faeb6:
          uStack_49 = 1;
          uStack_4a = 1;
          uStack_4b = 1;
          FUN_1428d80d0(&UNK_142932a10);
          goto code_r0x0001400faed7;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        uStack_49 = 1;
        uStack_4a = 1;
        uStack_4b = 1;
        FUN_1428d9250(&UNK_142932ab8);
        goto code_r0x0001400faed7;
      }
      *plVar10 = -1;
      plStack_58 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_1f0);
      *plStack_58 = *plStack_58 + 1;
    }
    lStack_1d8 = plStack_70[4];
    uStack_1f8 = *plStack_70;
    uStack_1f0 = (undefined4 *)plStack_70[1];
    lStack_1e8 = plStack_70[2];
    uStack_1e0 = (undefined4)plStack_70[3];
    uStack_1dc = *(undefined4 *)((longlong)plStack_70 + 0x1c);
    uStack_1cf = (undefined3)*(undefined4 *)((longlong)plStack_70 + 0x29);
    uStack_1cc = (undefined1)*(undefined4 *)((longlong)plStack_70 + 0x2c);
    uStack_1cb = (undefined3)((uint)*(undefined4 *)((longlong)plStack_70 + 0x2c) >> 8);
    uStack_49 = 1;
    uStack_4a = 0;
    uStack_4b = 1;
    cStack_1d0 = (char)lVar16;
    FUN_140c0f040(&lStack_288,&uStack_1f8);
    if (cStack_262 != '\b') {
      lStack_1e8 = CONCAT17(uStack_261,CONCAT16(cStack_262,uStack_268));
      uStack_1f8 = CONCAT44(uStack_274,uStack_278);
      uStack_1f0 = (undefined4 *)CONCAT44(uStack_26c,uStack_270);
      lVar16 = param_2[7];
      if (lVar16 == param_2[5]) {
        FUN_140562db0(param_2 + 5,&UNK_14294c518);
      }
      lVar15 = param_2[6];
      *(longlong *)(lVar15 + 0x10 + lVar16 * 0x18) = lStack_1e8;
      puVar3 = (undefined4 *)(lVar15 + lVar16 * 0x18);
      *puVar3 = (undefined4)uStack_1f8;
      puVar3[1] = uStack_1f8._4_4_;
      puVar3[2] = (undefined4)uStack_1f0;
      puVar3[3] = uStack_1f0._4_4_;
      param_2[7] = lVar16 + 1;
      lVar16 = *param_2;
      *param_2 = 1;
      param_2[1] = lStack_288;
      *(undefined4 *)(param_2 + 2) = 0x400;
      if (lVar16 != 0) {
        uStack_49 = 1;
        uStack_4a = 0;
        uStack_4b = 1;
        FUN_1428d9430(&UNK_14294c530,0x20,&UNK_14294c550);
        goto code_r0x0001400faed7;
      }
      lVar16 = param_2[3];
      param_2[3] = 1;
      param_2[4] = lStack_280;
      if (lVar16 != 0) {
        uStack_49 = 1;
        uStack_4a = 0;
        uStack_4b = 1;
        FUN_1428d9430(&UNK_14294c530,0x20,&UNK_14294c568);
        goto code_r0x0001400faed7;
      }
    }
  }
  lStack_b0 = lVar14 + -1;
  puVar3 = puStack_68 + lVar14 * 6;
  plStack_78 = param_2 + 0x1d;
  plStack_b8 = param_2 + 8;
  puVar13 = puStack_68;
  lVar14 = 0;
  while ((puVar13 != puVar3 && (*(char *)(puVar13 + 4) != '\x02'))) {
    uStack_e8 = *puVar13;
    uStack_e4 = puVar13[1];
    uStack_e0 = puVar13[2];
    uStack_dc = puVar13[3];
    uStack_d7 = *(undefined4 *)((longlong)puVar13 + 0x11);
    uStack_d3 = (undefined3)((uint)puVar13[5] >> 8);
    cStack_d8 = *(char *)(puVar13 + 4);
    FUN_140bebc90(param_2);
    cVar8 = func_0x000140c16a00();
    plStack_58 = (longlong *)CONCAT71(plStack_58._1_7_,cVar8);
    if (cVar8 == '\0') {
      uStack_1f8 = 0;
    }
    else {
      puVar9 = &UNK_14294c49a;
      do {
        pbVar1 = puVar9 + -1;
        uVar11 = (uint)(char)*pbVar1;
        if ((int)uVar11 < 0) {
          bVar4 = puVar9[-2];
          if ((char)bVar4 < -0x40) {
            bVar5 = puVar9[-3];
            if ((char)bVar5 < -0x40) {
              pbVar2 = puVar9 + -4;
              puVar9 = puVar9 + -4;
              uVar11 = bVar5 & 0x3f | (*pbVar2 & 7) << 6;
            }
            else {
              puVar9 = puVar9 + -3;
              uVar11 = bVar5 & 0xf;
            }
            uVar11 = bVar4 & 0x3f | uVar11 << 6;
          }
          else {
            puVar9 = puVar9 + -2;
            uVar11 = bVar4 & 0x1f;
          }
          uVar11 = (uint)(*pbVar1 & 0x3f) | uVar11 << 6;
          if (uVar11 != 0x5c) goto code_r0x0001400fa929;
code_r0x0001400fa960:
          puVar12 = puVar9 + -0x14294c477;
          if (puVar12 != (undefined *)0x0) {
            if (puVar12 < (undefined *)0x22) {
              if (-0x41 < (char)puVar9[1]) goto code_r0x0001400fa984;
            }
            else if (puVar12 == (undefined *)0x22) goto code_r0x0001400fa984;
            FUN_1428d9940(&UNK_14294c478,0x22,puVar12,0x22,&UNK_14294c120);
            goto code_r0x0001400faed7;
          }
code_r0x0001400fa984:
          lVar16 = 0x21 - (longlong)(puVar9 + -0x14294c478);
          puVar9 = puVar9 + 1;
          goto code_r0x0001400fa990;
        }
        puVar9 = puVar9 + -1;
        if (uVar11 == 0x5c) goto code_r0x0001400fa960;
code_r0x0001400fa929:
        if (uVar11 == 0x2f) goto code_r0x0001400fa960;
      } while (puVar9 != &UNK_14294c478);
      lVar16 = 0x22;
      puVar9 = &UNK_14294c478;
code_r0x0001400fa990:
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1428a1640(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x0001400fae11;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x0001400fae11:
          FUN_1428d80d0(&UNK_142932a10);
          goto code_r0x0001400faed7;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142932ab8);
        goto code_r0x0001400faed7;
      }
      *plVar10 = -1;
      plStack_90 = plVar10;
      uStack_1f0 = (undefined4 *)FUN_140c16d40(plVar10 + 1,&UNK_14294c580,0xc,puVar9,lVar16,1,0);
      *plStack_90 = *plStack_90 + 1;
      uStack_1f8 = 1;
    }
    plVar10 = plStack_78;
    if (SBORROW8(0,param_2[0x1d])) {
      plVar10 = (longlong *)0x0;
    }
    FUN_1402c1d40(lStack_60,&uStack_e8,lStack_88,param_5,plVar10,plStack_b8,param_2[0x2e] + 0x88,
                  param_2[0x2e] + 0x48);
    if ((char)plStack_58 != '\0') {
      plVar10 = (longlong *)func_0x000140c18130();
      if (*plVar10 == 0) {
        plVar10 = (longlong *)FUN_1428a1640(plVar10,0);
        if (plVar10 == (longlong *)0x0) goto code_r0x0001400fae22;
      }
      else {
        if ((int)*plVar10 != 1) {
code_r0x0001400fae22:
          FUN_1428d80d0(&UNK_142932a10);
          goto code_r0x0001400faed7;
        }
        plVar10 = plVar10 + 1;
      }
      if (*plVar10 != 0) {
        FUN_1428d9250(&UNK_142932ab8);
        goto code_r0x0001400faed7;
      }
      *plVar10 = -1;
      plStack_58 = plVar10;
      FUN_140c170f0(plVar10 + 1,uStack_1f0);
      *plStack_58 = *plStack_58 + 1;
    }
    puVar13 = puVar13 + 6;
    lVar16 = lVar14 + 1;
    bVar17 = lVar14 != lStack_b0;
    lVar14 = lVar16;
    if (bVar17) {
      FUN_140bebdf0(param_2,lStack_88);
    }
  }
  if (lStack_80 != 0) {
    func_0x000140613c20(puStack_68,lStack_80 * 0x18,8);
  }
  lVar14 = lStack_60;
  func_0x000142923c00(&lStack_288,lStack_60,0x90);
  func_0x000142923c00(&uStack_1f8,lVar14 + 0xa8,0x108);
  uStack_50 = 1;
  FUN_140be9040(auStack_2d0,&uStack_1f8);
  uVar7 = uStack_a8;
  uStack_50 = 0;
  FUN_140bea180(uStack_a8,&lStack_288,auStack_2d0);
  if (lStack_a0 == 0) goto code_r0x0001400fac40;
  plVar10 = (longlong *)func_0x000140c18130();
  if (*plVar10 == 0) {
    uStack_4c = 0;
    uStack_4d = 0;
    plVar10 = (longlong *)FUN_1428a1640(plVar10,0);
    if (plVar10 == (longlong *)0x0) goto code_r0x0001400fae5b;
  }
  else {
    if ((int)*plVar10 != 1) {
code_r0x0001400fae5b:
      uStack_4c = 0;
      uStack_4d = 0;
      FUN_1428d80d0(&UNK_142932a10);
      goto code_r0x0001400faed7;
    }
    plVar10 = plVar10 + 1;
  }
  if (*plVar10 == 0) {
    *plVar10 = -1;
    plStack_58 = plVar10;
    FUN_140c170f0(plVar10 + 1,uStack_98);
    *plStack_58 = *plStack_58 + 1;
code_r0x0001400fac40:
    FUN_1405da820(lStack_60 + 0x90);
    lVar14 = lStack_60;
    if (*(longlong *)(lStack_60 + 0x1b0) != 0) {
      func_0x000140613c20(*(undefined8 *)(lStack_60 + 0x1b8),*(longlong *)(lStack_60 + 0x1b0) * 0x18
                          ,8);
    }
    if (*(longlong *)(lVar14 + 0x1c8) != 0) {
      func_0x000140613c20(*(undefined8 *)(lVar14 + 0x1d0),*(longlong *)(lVar14 + 0x1c8) * 0x38,8);
    }
    if (*(longlong *)(lVar14 + 0x1e0) != 0) {
      func_0x000140613c20(*(undefined8 *)(lVar14 + 0x1e8),*(longlong *)(lVar14 + 0x1e0) * 0x48,8);
    }
    if (*(longlong *)(lVar14 + 0x200) != 0) {
      func_0x000140613c20(*(undefined8 *)(lVar14 + 0x208),*(longlong *)(lVar14 + 0x200) << 2,4);
    }
    return uVar7;
  }
  uStack_4c = 0;
  uStack_4d = 0;
  FUN_1428d9250(&UNK_142932ab8);
code_r0x0001400faed7:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

