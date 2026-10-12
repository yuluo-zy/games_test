// FUN_1409c6700  0x1409c6700

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409c6700(undefined8 param_1,longlong param_2,undefined4 *param_3,char param_4)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  uint *puVar5;
  code *pcVar6;
  undefined4 *puVar7;
  uint uVar8;
  longlong lVar9;
  ulonglong uVar10;
  undefined8 uVar11;
  longlong *plVar12;
  ulonglong uVar13;
  longlong lVar14;
  undefined *puVar15;
  longlong *plVar16;
  undefined *puVar17;
  undefined *puVar18;
  undefined1 uVar19;
  bool bVar20;
  undefined4 uVar21;
  undefined4 uStack_9b8;
  undefined4 uStack_9b4;
  undefined4 uStack_9b0;
  undefined4 uStack_9ac;
  undefined4 uStack_9a8;
  undefined4 uStack_9a4;
  undefined4 uStack_9a0;
  undefined4 uStack_99c;
  undefined *puStack_5c8;
  undefined8 uStack_5c0;
  undefined8 uStack_5b8;
  undefined4 *puStack_5b0;
  undefined4 uStack_5a8;
  undefined1 auStack_510 [38];
  char cStack_4ea;
  longlong lStack_4e0;
  undefined8 uStack_1d8;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined *puStack_138;
  uint uStack_130;
  undefined4 uStack_12c;
  undefined2 uStack_128;
  ulonglong uStack_e0;
  undefined4 *puStack_d8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  ulonglong uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  longlong *plStack_60;
  longlong *plStack_58;
  char cStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_68 = param_1;
  plStack_58 = (longlong *)param_3;
  cStack_4a = func_0x000140c16a00();
  if (cStack_4a == '\0') {
code_r0x0001409c67a8:
    uStack_c0 = 0;
code_r0x0001409c6978:
    lVar9 = param_2 + 0x38;
    uVar13 = 0x8597ee0727b461b3;
    lVar14 = FUN_1400dd6f0(lVar9,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
    if (((lVar14 != 0) && (uVar13 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar13 != 0)) {
      lVar14 = ~uVar13 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar14) != 0) {
        puVar17 = &UNK_142ac0028;
        uVar13 = FUN_140937aa0(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar14 + 0x10),
                               &UNK_142ac0028,4);
        if ((uVar13 & 1) == 0) {
          FUN_1428d9310(&UNK_142ac0030);
          goto code_r0x0001409c7171;
        }
        uVar13 = 0x864ddca98c2312a6;
        lVar14 = FUN_1400dd6f0(lVar9,0x864ddca98c2312a6,0x27b0e9369e9a484e);
        if (((lVar14 != 0) && (uVar13 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar13 != 0)) {
          lVar14 = ~uVar13 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar14) != 0) {
            puVar5 = *(uint **)(*(longlong *)(param_2 + 0x1e0) + lVar14 + 0x10);
            uStack_a0 = (ulonglong)*puVar5;
            plStack_60 = (longlong *)(ulonglong)puVar5[1];
            uVar13 = 0x958c258b851a69dc;
            lVar14 = FUN_1400dd6f0(lVar9,0x958c258b851a69dc,0x47bebccb61712be8);
            if (((lVar14 != 0) && (uVar13 < *(ulonglong *)(param_2 + 0x218))) &&
               (uVar10 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8), uVar10 != 0))
            {
              lVar14 = ~uVar10 * 0x60;
              if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar14) != 0) {
                uVar10 = func_0x000140b8f670(*(undefined8 *)
                                              (*(longlong *)(param_2 + 0x1e0) + lVar14 + 0x10));
                if (param_4 == '\0') {
                  uStack_70._0_7_ = CONCAT16(7,(undefined6)uStack_70);
                  uVar8 = 0;
                  plVar12 = plStack_58;
                  plVar16 = plStack_58;
                }
                else {
                  uVar11 = func_0x000140bd5790((undefined4 *)((longlong)plStack_58 + 4));
                  func_0x000140bd52a0(&uStack_1d8,uVar11,0x2b);
                  uStack_9a8 = (undefined4)uStack_1c8;
                  uStack_9a4 = uStack_1c8._4_4_;
                  uStack_9a0 = (undefined4)uStack_1c0;
                  uStack_99c = uStack_1c0._4_4_;
                  uStack_9b8 = (undefined4)uStack_1d8;
                  uStack_9b4 = uStack_1d8._4_4_;
                  uStack_9b0 = uStack_1d0;
                  uStack_9ac = uStack_1cc;
                  func_0x000140bd6ce0(&uStack_5b8,&uStack_9b8);
                  puStack_5c8 = &UNK_142ac0078;
                  uStack_5c0 = (undefined4 *)0x8;
                  uStack_8c = FUN_140be94d0(uStack_68,&puStack_5c8);
                  uStack_78 = uStack_1c8;
                  uStack_70 = uStack_1c0;
                  puVar7 = uStack_70;
                  uStack_88 = (undefined4)uStack_1d8;
                  uStack_84 = uStack_1d8._4_4_;
                  uStack_80 = uStack_1d0;
                  uStack_7c = uStack_1cc;
                  uStack_70._6_1_ = (char)((ulonglong)uStack_1c0 >> 0x30);
                  bVar20 = uStack_70._6_1_ != '\a';
                  plVar16 = (longlong *)(undefined4 *)0x0;
                  if (!bVar20) {
                    plVar16 = plStack_58;
                  }
                  uVar8 = (uint)bVar20;
                  plVar12 = (longlong *)&uStack_8c;
                  uStack_70 = puVar7;
                  if (!bVar20) {
                    plVar12 = plStack_58;
                  }
                }
                uVar11 = func_0x000140bd5790((undefined4 *)((longlong)plVar12 + 4));
                puStack_5c8 = (undefined *)(ulonglong)uVar8;
                uStack_5b8 = 0;
                puStack_5b0 = &uStack_8c;
                uStack_1d8 = (undefined *)CONCAT44(uStack_1d8._4_4_,2);
                uVar21 = 0;
                uStack_5c0 = (undefined4 *)plVar16;
                FUN_140867970(&uStack_9b8,&UNK_142ac0080,6,&puStack_5c8,1,&uStack_1d8,uVar11,param_2
                              ,uStack_68);
                uStack_49 = 1;
                uVar8 = func_0x000140be6750();
                uStack_49 = 1;
                plVar12 = (longlong *)FUN_1400d9b20(0);
                if (plVar12 == (longlong *)0x0) {
                  uStack_49 = 1;
                  FUN_1428d80d0(&UNK_142c3aaa0);
                  goto code_r0x0001409c7171;
                }
                uStack_128 = 0;
                uStack_12c = 0x3f800000;
                uStack_1d8 = (undefined *)0x0;
                uStack_1c8 = 0;
                uStack_1c0 = (undefined4 *)0x4;
                uStack_1b8 = 0;
                uStack_148 = (undefined4)*plVar12;
                uStack_144 = *(undefined4 *)((longlong)plVar12 + 4);
                uStack_140 = (undefined4)plVar12[1];
                uStack_13c = *(undefined4 *)((longlong)plVar12 + 0xc);
                *plVar12 = *plVar12 + 1;
                uStack_158 = _UNK_142a96730;
                uStack_150 = _UNK_142a96738;
                uStack_168 = _UNK_142a96720;
                uStack_164 = _UNK_142a96724;
                uStack_160 = _UNK_142a96728;
                uStack_15c = _UNK_142a9672c;
                uStack_198 = 0x100000003;
                uStack_1b0 = 0;
                uStack_1a8 = 1;
                uStack_1a0 = 0;
                uStack_49 = 1;
                puStack_138 = puVar17;
                uStack_130 = uVar8;
                FUN_140905be0(&puStack_5c8,&uStack_1d8,&uStack_9b8,0);
                if (cStack_4ea != '\b') {
                  FUN_14096b940(auStack_510,lStack_4e0 + 0xe8,0xa5a0625b7860559,
                                (float)(longlong)uStack_a0,
                                CONCAT44(uVar21,(float)(longlong)plStack_60));
                }
                func_0x000142923c00(&uStack_1d8,&puStack_5c8,0xf8);
                uStack_e0 = 0x579f39ab91a01865;
                puStack_5c8 = (undefined *)CONCAT44(puStack_5c8._4_4_,2);
                FUN_140908610(&uStack_e0,&uStack_1d8,0x815bf5036966978c,0x1d,&puStack_5c8);
                func_0x000142923c00(&puStack_5c8,&uStack_1d8,0xf8);
                uStack_1d8 = (undefined *)CONCAT44(uStack_1d8._4_4_,2);
                uStack_e0 = uVar10;
                FUN_1408fd6c0(&uStack_e0,&puStack_5c8,0x753991f424563352,0x1d,&uStack_1d8);
                func_0x000142923c00(&uStack_1d8,&puStack_5c8,0xf8);
                uStack_49 = 1;
                FUN_14162c590(&puStack_5c8,&uStack_1d8,0xd7bebe7cbc8b02bf);
                uStack_49 = 1;
                FUN_141629e90(&uStack_1d8,&puStack_5c8,0x912f62afc23333b3);
                uStack_130 = uStack_130 & 0x9fffff7f | 0x40000000;
                func_0x000142923c00(&puStack_5c8,&uStack_1d8,0xf8);
                uStack_49 = 1;
                FUN_140906a90(&puStack_5c8,uVar13 * 6);
                func_0x000142923c00(&puStack_5c8,&uStack_9b8,1000);
                uStack_49 = 0;
                FUN_140868370(&puStack_5c8);
                if (uStack_70._6_1_ != '\a') {
                  uStack_1b8 = CONCAT44(uStack_1b8._4_4_,uStack_70._4_4_);
                  uStack_1c8 = CONCAT44((undefined4)uStack_78,uStack_7c);
                  uStack_1c0 = (undefined4 *)CONCAT44((undefined4)uStack_70,uStack_78._4_4_);
                  uStack_1d8 = (undefined *)CONCAT44(uStack_88,uStack_8c);
                  uStack_1d0 = uStack_84;
                  uStack_1cc = uStack_80;
                  uStack_49 = 0;
                  uVar13 = 0x8597ee0727b461b3;
                  lVar9 = FUN_1400dd6f0(lVar9,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
                  if (((lVar9 != 0) && (uVar13 < *(ulonglong *)(param_2 + 0x218))) &&
                     (uVar13 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar13 * 8),
                     uVar13 != 0)) {
                    lVar9 = ~uVar13 * 0x60;
                    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) != 0) {
                      uStack_49 = 0;
                      puVar17 = &UNK_142ac00a0;
                      uVar13 = FUN_140937aa0(*(undefined8 *)
                                              (*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10),
                                             &UNK_142ac00a0,10);
                      if ((uVar13 & 1) == 0) {
                        uStack_49 = 0;
                        FUN_1428d9310(&UNK_142ac00b0);
                        goto code_r0x0001409c7171;
                      }
                      uStack_e0 = uStack_e0 & 0xffffffff00000000;
                      puStack_d8 = (undefined4 *)plStack_58;
                      uStack_5a8 = (undefined4)uStack_1b8;
                      uStack_5b8 = uStack_1c8;
                      puStack_5b0 = uStack_1c0;
                      puStack_5c8 = uStack_1d8;
                      uStack_5c0 = (undefined4 *)CONCAT44(uStack_1cc,uStack_1d0);
                      uStack_49 = 0;
                      FUN_1409cdec0(uStack_68,param_2,&UNK_142ac00c8,0x11,puVar17,&uStack_e0,
                                    &puStack_5c8);
                      goto code_r0x0001409c7056;
                    }
                  }
                  uStack_49 = 0;
                  FUN_1428d9310(&UNK_142ac0088);
                  goto code_r0x0001409c7171;
                }
code_r0x0001409c7056:
                if (cStack_4a == '\0') {
                  return;
                }
                plVar12 = (longlong *)func_0x000140c18130();
                if (*plVar12 == 1) {
                  plVar12 = plVar12 + 1;
                }
                else if ((*plVar12 != 0) ||
                        (plVar12 = (longlong *)FUN_1428f87d0(plVar12,0), plVar12 == (longlong *)0x0)
                        ) goto code_r0x0001409c70e4;
                if (*plVar12 == 0) {
                  *plVar12 = -1;
                  plStack_58 = plVar12;
                  FUN_140c170f0(plVar12 + 1,uStack_98);
                  *plStack_58 = *plStack_58 + 1;
                  return;
                }
                goto code_r0x0001409c70f0;
              }
            }
            FUN_1428d9310(&UNK_142ac0060);
            goto code_r0x0001409c7171;
          }
        }
        FUN_1428d9310(&UNK_142ac0048);
        goto code_r0x0001409c7171;
      }
    }
  }
  else {
    puVar17 = &UNK_142abf9fb;
    lVar14 = 0x45;
    uVar13 = 0x45;
    lVar9 = FUN_1409be680(&UNK_142abf9fb);
    if (lVar9 == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x45) {
          if ((char)(&UNK_142abf9fb)[uVar13] < -0x40) {
code_r0x0001409c6788:
            FUN_1428d9940(&UNK_142abf9fb,0x45,0,uVar13,&UNK_142abfb10);
            goto code_r0x0001409c67a8;
          }
        }
        else if (uVar13 != 0x45) goto code_r0x0001409c6788;
      }
      puVar17 = &UNK_142abf9fb;
      lVar9 = FUN_1409be680(&UNK_142abf9fb);
      if (lVar9 == 1) {
        uVar10 = uVar13 + 2;
        if (uVar10 != 0) {
          uVar19 = uVar10 == 0x45;
          if (0x44 < uVar10) goto code_r0x0001409c6809;
          uVar19 = (&UNK_142abf9fd)[uVar13] == -0x41;
          if ((char)(&UNK_142abf9fd)[uVar13] < -0x40) {
            do {
              uVar13 = 0x45;
              FUN_1428d9940(&UNK_142abf9fb,0x45);
code_r0x0001409c6809:
            } while (!(bool)uVar19);
          }
        }
        lVar14 = 0x43 - uVar13;
        puVar17 = &UNK_142abf9fb + uVar10;
      }
    }
    puVar15 = &UNK_142abff34;
    puVar18 = &UNK_142abfeef;
    do {
      bVar2 = puVar15[-1];
      uVar8 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar15[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar15[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar15 + -4;
            puVar15 = puVar15 + -4;
            uVar8 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar15 = puVar15 + -3;
            uVar8 = bVar4 & 0xf;
          }
          uVar8 = bVar3 & 0x3f | uVar8 << 6;
        }
        else {
          puVar15 = puVar15 + -2;
          uVar8 = bVar3 & 0x1f;
        }
        uVar8 = (uint)(bVar2 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto code_r0x0001409c68a4;
code_r0x0001409c68b5:
        puVar18 = puVar15 + -0x142abfeee;
        if (puVar18 != (undefined *)0x0) {
          uVar19 = puVar18 == (undefined *)0x45;
          if ((undefined *)0x44 < puVar18) goto code_r0x0001409c68f4;
          uVar19 = puVar15[1] == -0x41;
          if ((char)puVar15[1] < -0x40) {
            do {
              FUN_1428d9940(&UNK_142abfeef,0x45);
code_r0x0001409c68f4:
            } while (!(bool)uVar19);
          }
        }
        lVar9 = 0x45 - (longlong)puVar18;
        puVar18 = &UNK_142abfeef + (longlong)puVar18;
        goto code_r0x0001409c6901;
      }
      puVar15 = puVar15 + -1;
      if (uVar8 == 0x5c) goto code_r0x0001409c68b5;
code_r0x0001409c68a4:
      if (uVar8 == 0x2f) goto code_r0x0001409c68b5;
    } while (puVar15 != &UNK_142abfeef);
    lVar9 = 0x45;
code_r0x0001409c6901:
    plVar12 = (longlong *)func_0x000140c18130();
    if (*plVar12 == 1) {
      plVar12 = plVar12 + 1;
code_r0x0001409c691a:
      if (*plVar12 == 0) {
        *plVar12 = -1;
        plStack_60 = plVar12;
        uStack_b8 = FUN_140c16d40(plVar12 + 1,puVar17,lVar14,puVar18,lVar9,1,0);
        *plStack_60 = *plStack_60 + 1;
        uStack_c0 = 1;
        uStack_98 = uStack_b8;
        goto code_r0x0001409c6978;
      }
    }
    else {
      if ((*plVar12 == 0) &&
         (plVar12 = (longlong *)FUN_1428f87d0(plVar12,0), plVar12 != (longlong *)0x0))
      goto code_r0x0001409c691a;
code_r0x0001409c70e4:
      FUN_1428d80d0(&UNK_142c3aaa0);
    }
code_r0x0001409c70f0:
    FUN_1428d9250(&UNK_142c3ab70);
  }
  FUN_1428d9310(&UNK_142ac0010);
code_r0x0001409c7171:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

