
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a69340(longlong *param_1,longlong *param_2)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined4 uVar6;
  uint uVar7;
  longlong lVar8;
  longlong lVar9;
  code *pcVar10;
  bool bVar11;
  bool bVar12;
  bool bVar13;
  float fVar14;
  char cVar15;
  longlong lVar16;
  undefined *puVar17;
  undefined *puVar18;
  longlong lVar19;
  longlong *plVar20;
  uint uVar21;
  ulonglong uVar22;
  longlong lVar23;
  ulonglong uVar24;
  undefined4 *puVar25;
  ulonglong uVar26;
  longlong lVar27;
  float *pfVar28;
  ulonglong uVar29;
  longlong lVar30;
  undefined *puVar31;
  undefined *puVar32;
  uint *puVar33;
  undefined1 uVar34;
  float fVar35;
  undefined4 uVar36;
  float fVar37;
  float fVar38;
  undefined4 *puStack_108;
  undefined4 *puStack_100;
  undefined8 uStack_f0;
  longlong lStack_e0;
  longlong lStack_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  longlong lStack_b8;
  longlong lStack_a8;
  undefined4 *puStack_98;
  
  cVar15 = func_0x000140c16a00();
  if (cVar15 == '\0') {
LAB_140a693e1:
    bVar11 = false;
    goto LAB_140a695b6;
  }
  puVar31 = &UNK_142ad4b20;
  lVar27 = 0x47;
  uVar26 = 0x47;
  lVar16 = FUN_140a67380(&UNK_142ad4b20);
  if (lVar16 == 1) {
    if (uVar26 != 0) {
      if (uVar26 < 0x47) {
        if ((char)(&UNK_142ad4b20)[uVar26] < -0x40) {
LAB_140a693c1:
          FUN_1428d9940(&UNK_142ad4b20,0x47,0,uVar26,&UNK_142ad4c78);
          goto LAB_140a693e1;
        }
      }
      else if (uVar26 != 0x47) goto LAB_140a693c1;
    }
    puVar31 = &UNK_142ad4b20;
    lVar16 = FUN_140a67380(&UNK_142ad4b20);
    if (lVar16 == 1) {
      uVar29 = uVar26 + 2;
      if (uVar29 != 0) {
        uVar34 = uVar29 == 0x47;
        if (0x46 < uVar29) goto LAB_140a69442;
        uVar34 = (&UNK_142ad4b22)[uVar26] == -0x41;
        if ((char)(&UNK_142ad4b22)[uVar26] < -0x40) {
          do {
            uVar26 = 0x47;
            FUN_1428d9940(&UNK_142ad4b20);
LAB_140a69442:
          } while (!(bool)uVar34);
        }
      }
      lVar27 = 0x45 - uVar26;
      puVar31 = &UNK_142ad4b20 + uVar29;
    }
  }
  puVar17 = &UNK_142ad51de;
  puVar32 = &UNK_142ad51a4;
  do {
    bVar3 = puVar17[-1];
    uVar21 = (uint)(char)bVar3;
    if ((char)bVar3 < '\0') {
      bVar4 = puVar17[-2];
      if ((char)bVar4 < -0x40) {
        bVar5 = puVar17[-3];
        if ((char)bVar5 < -0x40) {
          pbVar1 = puVar17 + -4;
          puVar17 = puVar17 + -4;
          uVar21 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          puVar17 = puVar17 + -3;
          uVar21 = bVar5 & 0xf;
        }
        uVar21 = bVar4 & 0x3f | uVar21 << 6;
      }
      else {
        puVar17 = puVar17 + -2;
        uVar21 = bVar4 & 0x1f;
      }
      uVar21 = (uint)(bVar3 & 0x3f) | uVar21 << 6;
      if (uVar21 != 0x5c) goto LAB_140a694e9;
LAB_140a694ff:
      puVar18 = puVar17 + -0x142ad51a4;
      puVar32 = puVar17 + -0x142ad51a3;
      if (puVar32 != (undefined *)0x0) {
        uVar34 = puVar32 == (undefined *)0x3a;
        if ((undefined *)0x39 < puVar32) goto LAB_140a69541;
        uVar34 = puVar17[1] == -0x41;
        if ((char)puVar17[1] < -0x40) {
          do {
            puVar18 = (undefined *)FUN_1428d9940(&UNK_142ad51a4,0x3a);
LAB_140a69541:
          } while (!(bool)uVar34);
        }
      }
      lVar16 = 0x39 - (longlong)puVar18;
      puVar32 = &UNK_142ad51a4 + (longlong)puVar32;
      goto LAB_140a6954f;
    }
    puVar17 = puVar17 + -1;
    if (uVar21 == 0x5c) goto LAB_140a694ff;
LAB_140a694e9:
    if (uVar21 == 0x2f) goto LAB_140a694ff;
  } while (puVar17 != &UNK_142ad51a4);
  lVar16 = 0x3a;
LAB_140a6954f:
  plVar20 = (longlong *)func_0x000140c18130();
  if (*plVar20 == 1) {
    plVar20 = plVar20 + 1;
  }
  else if ((*plVar20 != 0) ||
          (plVar20 = (longlong *)FUN_1428f87d0(plVar20,0), plVar20 == (longlong *)0x0))
  goto LAB_140a69ad2;
  if (*plVar20 == 0) {
    *plVar20 = -1;
    uStack_f0 = FUN_140c16d40(plVar20 + 1,puVar31,lVar27,puVar32,lVar16,1,0);
    *plVar20 = *plVar20 + 1;
    bVar11 = true;
LAB_140a695b6:
    fVar14 = _DAT_142925904;
    uVar21 = _DAT_1429258f0;
    lVar16 = *param_1;
    lVar27 = param_1[1];
    uVar6 = *(undefined4 *)((longlong)param_1 + 0x14);
    puVar33 = *(uint **)(lVar16 + 0xf8);
    puVar2 = puVar33 + *(longlong *)(lVar16 + 0x100);
    cVar15 = *(char *)(lVar16 + 0x108);
    lVar8 = *param_2;
    lStack_e0 = 8;
    lStack_c8 = 0;
    bVar12 = false;
    lStack_a8 = 0;
    uVar29 = 0;
    uVar26 = 0;
    lStack_b8 = 0;
    bVar13 = false;
    do {
      if (cVar15 == '\0') {
        if (uVar29 == uVar26) {
          do {
            if (puVar33 == puVar2) goto LAB_140a69a3f;
            uVar7 = *puVar33;
            puVar33 = puVar33 + 1;
            uVar26 = *(ulonglong *)(*(longlong *)(lVar27 + 0x100) + 0x58 + (ulonglong)uVar7 * 0xa0);
          } while (uVar26 == 0);
          lVar30 = *(longlong *)(lVar27 + 0x100) + (ulonglong)uVar7 * 0xa0;
          uVar22 = (ulonglong)*(uint *)(lVar30 + 0x94);
          lVar23 = *(longlong *)(lVar27 + 0x1a8);
          lVar19 = *(longlong *)(lVar23 + 0x18 + uVar22 * 0x48);
          lVar9 = *(longlong *)(lVar23 + 0x38 + uVar22 * 0x48);
          uVar29 = *(ulonglong *)(lVar16 + 0x118);
          uVar24 = *(ulonglong *)(lVar30 + 0x88);
          if ((uVar29 < uVar24) && (*(longlong *)(*(longlong *)(lVar30 + 0x80) + uVar29 * 8) != 0))
          {
            bVar12 = true;
            if ((*(ulonglong *)(lVar23 + 0x40 + uVar22 * 0x48) <= uVar29) ||
               (uVar29 = *(ulonglong *)(lVar9 + uVar29 * 8), uVar29 == 0)) {
              lStack_a8 = 0;
              uVar29 = *(ulonglong *)(lVar16 + 0x120);
              goto joined_r0x000140a6983b;
            }
            lStack_a8 = *(longlong *)(lVar19 + 0x10 + ~uVar29 * 0x30);
            uVar29 = *(ulonglong *)(lVar16 + 0x120);
            if (uVar29 < uVar24) goto LAB_140a698c2;
LAB_140a698fa:
            bVar13 = false;
          }
          else {
            bVar12 = false;
            uVar29 = *(ulonglong *)(lVar16 + 0x120);
joined_r0x000140a6983b:
            if (uVar24 <= uVar29) goto LAB_140a698fa;
LAB_140a698c2:
            if (*(longlong *)(*(longlong *)(lVar30 + 0x80) + uVar29 * 8) == 0) goto LAB_140a698fa;
            lVar23 = ~*(ulonglong *)(lVar9 + uVar29 * 8) * 0x30;
            lStack_b8 = *(longlong *)(lVar19 + 0x10 + lVar23);
            lStack_d0 = *(longlong *)(lVar19 + 0x28 + lVar23);
            bVar13 = true;
          }
          lVar23 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar16 + 0x110) * 8) * 0x30;
          lStack_c8 = *(longlong *)(lVar19 + 0x10 + lVar23);
          lStack_d8 = *(longlong *)(lVar19 + 0x28 + lVar23);
          lStack_e0 = *(longlong *)(lVar30 + 0x50);
          uVar29 = 0;
        }
        uVar24 = (ulonglong)*(uint *)(lStack_e0 + 8 + uVar29 * 0x10);
        lVar19 = lStack_c8 + uVar24 * 0x1c;
        puStack_98 = (undefined4 *)(lStack_d8 + uVar24 * 4);
        lVar23 = 0;
        if (bVar12) {
          lVar23 = lStack_a8;
        }
        if (bVar13) {
          pfVar28 = (float *)(lStack_b8 + uVar24 * 8);
          puVar25 = (undefined4 *)(lStack_d0 + uVar24 * 4);
          puStack_100 = puVar25;
        }
        else {
          pfVar28 = (float *)0x0;
          puVar25 = puStack_100;
        }
      }
      else {
        if (uVar29 == uVar26) {
          do {
            if (puVar33 == puVar2) goto LAB_140a69a3f;
            uVar7 = *puVar33;
            puVar33 = puVar33 + 1;
            uVar26 = *(ulonglong *)(*(longlong *)(lVar27 + 0x1a8) + 0x10 + (ulonglong)uVar7 * 0x48);
          } while (uVar26 == 0);
          lVar23 = *(longlong *)(lVar27 + 0x1a8) + (ulonglong)uVar7 * 0x48;
          lVar19 = *(longlong *)(lVar23 + 0x18);
          lVar9 = *(longlong *)(lVar23 + 0x38);
          if ((*(ulonglong *)(lVar16 + 0x118) < *(ulonglong *)(lVar23 + 0x40)) &&
             (uVar29 = *(ulonglong *)(lVar9 + *(ulonglong *)(lVar16 + 0x118) * 8), uVar29 != 0)) {
            lStack_a8 = *(longlong *)(lVar19 + 0x10 + ~uVar29 * 0x30);
            bVar12 = true;
          }
          else {
            bVar12 = false;
          }
          lVar30 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar16 + 0x110) * 8) * 0x30;
          lStack_c8 = *(longlong *)(lVar19 + 0x10 + lVar30);
          lStack_d8 = *(longlong *)(lVar19 + 0x28 + lVar30);
          if ((*(ulonglong *)(lVar16 + 0x120) < *(ulonglong *)(lVar23 + 0x40)) &&
             (uVar29 = *(ulonglong *)(lVar9 + *(ulonglong *)(lVar16 + 0x120) * 8), uVar29 != 0)) {
            lVar23 = ~uVar29 * 0x30;
            lStack_b8 = *(longlong *)(lVar19 + 0x10 + lVar23);
            lStack_d0 = *(longlong *)(lVar19 + 0x28 + lVar23);
            bVar13 = true;
            uVar29 = 0;
          }
          else {
            uVar29 = 0;
            bVar13 = false;
          }
        }
        uVar24 = uVar29 & 0xffffffff;
        lVar19 = lStack_c8 + uVar24 * 0x1c;
        puStack_98 = (undefined4 *)(lStack_d8 + uVar24 * 4);
        lVar23 = 0;
        if (bVar12) {
          lVar23 = lStack_a8;
        }
        if (bVar13) {
          pfVar28 = (float *)(lStack_b8 + uVar24 * 8);
          puStack_108 = (undefined4 *)(lStack_d0 + uVar24 * 4);
          puVar25 = puStack_108;
        }
        else {
          pfVar28 = (float *)0x0;
          puVar25 = puStack_108;
        }
      }
      fVar35 = *(float *)(lVar19 + 0x10);
      fVar37 = fVar14;
      if ((fVar35 != 0.0) || (NAN(fVar35))) {
        fVar38 = *(float *)(lVar8 + 0x28);
        if (lVar23 != 0) {
          fVar38 = (float)((uint)fVar38 ^ uVar21);
        }
        *puStack_98 = uVar6;
        fVar38 = (fVar14 / fVar35) * fVar38 + *(float *)(lVar19 + 0x14);
        fVar35 = 0.0;
        if (0.0 <= fVar38) {
          fVar35 = fVar38;
        }
        if (fVar35 <= fVar14) {
          fVar37 = fVar35;
        }
      }
      else {
        if (lVar23 != 0) {
          fVar37 = 0.0;
        }
        *puStack_98 = uVar6;
      }
      *(float *)(lVar19 + 0x14) = fVar37;
      uVar36 = FUN_140c95ef0(lVar19);
      uVar29 = uVar29 + 1;
      *puStack_98 = uVar6;
      *(undefined4 *)(lVar19 + 0x18) = uVar36;
      if (pfVar28 != (float *)0x0) {
        fVar35 = *(float *)(lVar8 + 0x28);
        *puVar25 = uVar6;
        *pfVar28 = fVar35 + *pfVar28;
      }
    } while( true );
  }
  goto LAB_140a69ade;
LAB_140a69a3f:
  if (!bVar11) {
    return;
  }
  plVar20 = (longlong *)func_0x000140c18130();
  if (*plVar20 == 1) {
    plVar20 = plVar20 + 1;
  }
  else if ((*plVar20 != 0) ||
          (plVar20 = (longlong *)FUN_1428f87d0(plVar20,0), plVar20 == (longlong *)0x0)) {
LAB_140a69ad2:
    FUN_1428d80d0(&UNK_142c3aaa0);
    goto LAB_140a69ade;
  }
  if (*plVar20 == 0) {
    *plVar20 = -1;
    FUN_140c170f0(plVar20 + 1,uStack_f0);
    *plVar20 = *plVar20 + 1;
    return;
  }
LAB_140a69ade:
  FUN_1428d9250(&UNK_142c3ab70);
  pcVar10 = (code *)swi(3);
  (*pcVar10)();
  return;
}

