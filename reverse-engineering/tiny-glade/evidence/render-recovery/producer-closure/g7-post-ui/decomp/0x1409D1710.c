
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409d1710(undefined8 param_1,longlong param_2,longlong param_3)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  uint uVar5;
  uint *puVar6;
  code *pcVar7;
  char cVar8;
  uint uVar9;
  longlong lVar10;
  undefined *puVar11;
  undefined *puVar12;
  undefined8 uVar13;
  longlong *plVar14;
  uint uVar15;
  ulonglong uVar16;
  longlong lVar17;
  ulonglong uVar18;
  undefined *puVar19;
  undefined *puVar20;
  undefined1 uVar21;
  float fVar22;
  float fVar23;
  undefined1 auStack_950 [1000];
  undefined4 auStack_568 [2];
  longlong lStack_560;
  undefined1 auStack_4b0 [38];
  char cStack_48a;
  longlong lStack_480;
  undefined8 auStack_180 [2];
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined *puStack_e0;
  uint uStack_d8;
  undefined4 uStack_d4;
  undefined2 uStack_d0;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 auStack_78 [3];
  longlong *plStack_60;
  longlong *plStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  cVar8 = func_0x000140c16a00();
  plStack_58 = (longlong *)CONCAT71(plStack_58._1_7_,cVar8);
  if (cVar8 == '\0') {
code_r0x0001409d17ad:
    uStack_88 = 0;
code_r0x0001409d1986:
    lVar10 = param_2 + 0x38;
    uVar16 = 0x8597ee0727b461b3;
    lVar17 = FUN_1400dd6f0(lVar10,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
    if (((lVar17 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 != 0)) {
      lVar17 = ~uVar16 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar17) != 0) {
        puVar20 = &UNK_142ac17b0;
        uVar16 = FUN_140937aa0(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar17 + 0x10),
                               &UNK_142ac17b0,0xd);
        if ((uVar16 & 1) == 0) {
          FUN_1428d9310(&UNK_142ac17c0);
          goto code_r0x0001409d1eb6;
        }
        uVar16 = 0x864ddca98c2312a6;
        lVar17 = FUN_1400dd6f0(lVar10,0x864ddca98c2312a6,0x27b0e9369e9a484e);
        if (((lVar17 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 != 0)) {
          lVar17 = ~uVar16 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar17) != 0) {
            puVar6 = *(uint **)(*(longlong *)(param_2 + 0x1e0) + lVar17 + 0x10);
            uVar15 = *puVar6;
            uVar5 = puVar6[1];
            uVar16 = 0x3d7b56e787d47c40;
            lVar10 = FUN_1400dd6f0(lVar10,0x3d7b56e787d47c40,0x67b0b10a8d43bfc7);
            if (((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218))) &&
               (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 != 0))
            {
              lVar10 = ~uVar16 * 0x60;
              if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar10) != 0) {
                if (*(char *)(*(longlong *)(*(longlong *)(param_2 + 0x1e0) + lVar10 + 0x10) + 0xf4)
                    != '\0') {
                  uVar13 = func_0x000140bd5790(param_3 + 4);
                  auStack_568[0] = 0;
                  auStack_180[0] = CONCAT44(auStack_180[0]._4_4_,2);
                  lStack_560 = param_3;
                  FUN_140867970(auStack_950,&UNK_142ac1808,0xd,auStack_568,1,auStack_180,uVar13,
                                param_2,param_1);
                  uStack_49 = 1;
                  uVar9 = func_0x000140be6750();
                  uStack_49 = 1;
                  plVar14 = (longlong *)FUN_1400d9b20(0);
                  if (plVar14 == (longlong *)0x0) {
                    uStack_49 = 1;
                    FUN_1428d80d0(&UNK_142c3aaa0);
                    goto code_r0x0001409d1eb6;
                  }
                  uStack_d0 = 0;
                  uStack_d4 = 0x3f800000;
                  auStack_180[0] = 0;
                  uStack_170 = 0;
                  uStack_168 = 4;
                  uStack_160 = 0;
                  uStack_f0 = (undefined4)*plVar14;
                  uStack_ec = *(undefined4 *)((longlong)plVar14 + 4);
                  uStack_e8 = (undefined4)plVar14[1];
                  uStack_e4 = *(undefined4 *)((longlong)plVar14 + 0xc);
                  *plVar14 = *plVar14 + 1;
                  uStack_100 = _UNK_142a96730;
                  uStack_f8 = _UNK_142a96738;
                  uStack_110 = _UNK_142a96720;
                  uStack_10c = _UNK_142a96724;
                  uStack_108 = _UNK_142a96728;
                  uStack_104 = _UNK_142a9672c;
                  uStack_140 = 0x100000003;
                  uStack_158 = 0;
                  uStack_150 = 1;
                  uStack_148 = 0;
                  uStack_49 = 1;
                  puStack_e0 = puVar20;
                  uStack_d8 = uVar9;
                  FUN_140905be0(auStack_568,auStack_180,auStack_950,0);
                  if (cStack_48a != '\b') {
                    fVar22 = (float)uVar15;
                    fVar23 = (float)uVar5;
                    uVar16 = (longlong)fVar23 & 0xffffffff;
                    if (fVar23 < 0.0) {
                      uVar16 = 0;
                    }
                    if (_UNK_1429a07d0 < fVar23) {
                      uVar16 = 0xffffffff;
                    }
                    uVar18 = (longlong)fVar22 & 0xffffffff;
                    if (fVar22 < 0.0) {
                      uVar18 = 0;
                    }
                    if (_UNK_1429a07d0 < fVar22) {
                      uVar18 = 0xffffffff;
                    }
                    FUN_14096b760(auStack_4b0,lStack_480 + 0xe8,0xa5a0625b7860559,
                                  uVar18 | uVar16 << 0x20);
                  }
                  func_0x000142923c00(auStack_180,auStack_568,0xf8);
                  auStack_78[0] = 0xf9f14d14bcc4e780;
                  auStack_568[0] = 2;
                  FUN_140908610(auStack_78,auStack_180,0x9f0d228ee3dba429,0x1d,auStack_568);
                  func_0x000142923c00(auStack_568,auStack_180,0xf8);
                  uStack_49 = 1;
                  FUN_14162aa90(auStack_180,auStack_568,0x19ce7a8f29cef00d);
                  uStack_d8 = uStack_d8 & 0x9fffff7f | 0x40000000;
                  func_0x000142923c00(auStack_568,auStack_180,0xf8);
                  uStack_49 = 1;
                  FUN_140906a90(auStack_568,6);
                  func_0x000142923c00(auStack_568,auStack_950,1000);
                  uStack_49 = 0;
                  FUN_140868370(auStack_568);
                }
                if ((char)plStack_58 == '\0') {
                  return;
                }
                plVar14 = (longlong *)func_0x000140c18130();
                if (*plVar14 == 1) {
                  plVar14 = plVar14 + 1;
                }
                else if ((*plVar14 != 0) ||
                        (plVar14 = (longlong *)FUN_1428f87d0(plVar14,0), plVar14 == (longlong *)0x0)
                        ) goto code_r0x0001409d1e53;
                if (*plVar14 == 0) {
                  *plVar14 = -1;
                  plStack_58 = plVar14;
                  FUN_140c170f0(plVar14 + 1,plStack_60);
                  *plStack_58 = *plStack_58 + 1;
                  return;
                }
                goto code_r0x0001409d1e5f;
              }
            }
            FUN_1428d9310(&UNK_142ac17f0);
            goto code_r0x0001409d1eb6;
          }
        }
        FUN_1428d9310(&UNK_142ac17d8);
        goto code_r0x0001409d1eb6;
      }
    }
  }
  else {
    puVar20 = &UNK_142ac1445;
    lVar17 = 0x57;
    uVar16 = 0x57;
    lVar10 = FUN_1409cd800(&UNK_142ac1445);
    if (lVar10 == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x57) {
          if ((char)(&UNK_142ac1445)[uVar16] < -0x40) {
code_r0x0001409d178d:
            FUN_1428d9940(&UNK_142ac1445,0x57,0,uVar16,&UNK_142ac1520);
            goto code_r0x0001409d17ad;
          }
        }
        else if (uVar16 != 0x57) goto code_r0x0001409d178d;
      }
      puVar20 = &UNK_142ac1445;
      lVar10 = FUN_1409cd800(&UNK_142ac1445);
      if (lVar10 == 1) {
        uVar18 = uVar16 + 2;
        if (uVar18 != 0) {
          uVar21 = uVar18 == 0x57;
          if (0x56 < uVar18) goto code_r0x0001409d180e;
          uVar21 = (&UNK_142ac1447)[uVar16] == -0x41;
          if ((char)(&UNK_142ac1447)[uVar16] < -0x40) {
            do {
              uVar16 = 0x57;
              FUN_1428d9940(&UNK_142ac1445,0x57);
code_r0x0001409d180e:
            } while (!(bool)uVar21);
          }
        }
        lVar17 = 0x55 - uVar16;
        puVar20 = &UNK_142ac1445 + uVar18;
      }
    }
    puVar11 = &UNK_142ac1795;
    puVar19 = &UNK_142ac1748;
    do {
      bVar2 = puVar11[-1];
      uVar15 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar11[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar11[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar15 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar15 = bVar4 & 0xf;
          }
          uVar15 = bVar3 & 0x3f | uVar15 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar15 = bVar3 & 0x1f;
        }
        uVar15 = (uint)(bVar2 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto code_r0x0001409d18a9;
code_r0x0001409d18bf:
        puVar12 = puVar11 + -0x142ac1748;
        puVar19 = puVar11 + -0x142ac1747;
        if (puVar19 != (undefined *)0x0) {
          uVar21 = puVar19 == (undefined *)0x4d;
          if ((undefined *)0x4c < puVar19) goto code_r0x0001409d1901;
          uVar21 = puVar11[1] == -0x41;
          if ((char)puVar11[1] < -0x40) {
            do {
              puVar12 = (undefined *)FUN_1428d9940(&UNK_142ac1748,0x4d);
code_r0x0001409d1901:
            } while (!(bool)uVar21);
          }
        }
        lVar10 = 0x4c - (longlong)puVar12;
        puVar19 = &UNK_142ac1748 + (longlong)puVar19;
        goto code_r0x0001409d190f;
      }
      puVar11 = puVar11 + -1;
      if (uVar15 == 0x5c) goto code_r0x0001409d18bf;
code_r0x0001409d18a9:
      if (uVar15 == 0x2f) goto code_r0x0001409d18bf;
    } while (puVar11 != &UNK_142ac1748);
    lVar10 = 0x4d;
code_r0x0001409d190f:
    plVar14 = (longlong *)func_0x000140c18130();
    if (*plVar14 == 1) {
      plVar14 = plVar14 + 1;
code_r0x0001409d1928:
      if (*plVar14 == 0) {
        *plVar14 = -1;
        plStack_60 = plVar14;
        uStack_80 = FUN_140c16d40(plVar14 + 1,puVar20,lVar17,puVar19,lVar10,1,0);
        *plStack_60 = *plStack_60 + 1;
        uStack_88 = 1;
        plStack_60 = (longlong *)uStack_80;
        goto code_r0x0001409d1986;
      }
    }
    else {
      if ((*plVar14 == 0) &&
         (plVar14 = (longlong *)FUN_1428f87d0(plVar14,0), plVar14 != (longlong *)0x0))
      goto code_r0x0001409d1928;
code_r0x0001409d1e53:
      FUN_1428d80d0(&UNK_142c3aaa0);
    }
code_r0x0001409d1e5f:
    FUN_1428d9250(&UNK_142c3ab70);
  }
  FUN_1428d9310(&UNK_142ac1798);
code_r0x0001409d1eb6:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

