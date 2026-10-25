
void FUN_140929a90(longlong *param_1,undefined8 *param_2)

{
  byte *pbVar1;
  uint *puVar2;
  float fVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  char cVar7;
  longlong lVar8;
  code *pcVar9;
  bool bVar10;
  longlong lVar11;
  undefined *puVar12;
  undefined *puVar13;
  undefined8 uVar14;
  longlong *plVar15;
  uint uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong lVar19;
  uint *puVar20;
  undefined *puVar21;
  ulonglong uVar22;
  undefined *puVar23;
  undefined1 uVar24;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  longlong lStack_90;
  longlong lStack_88;
  longlong lStack_80;
  longlong lStack_78;
  longlong *plStack_70;
  char cStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  cStack_61 = func_0x000140c16a00();
  if (cStack_61 == '\0') {
LAB_140929b28:
    uStack_d0 = 0;
    goto LAB_140929cfa;
  }
  puVar21 = &UNK_142aae665;
  lVar19 = 0x50;
  uVar18 = 0x50;
  lVar11 = FUN_140923920(&UNK_142aae665);
  if (lVar11 == 1) {
    if (uVar18 != 0) {
      if (uVar18 < 0x50) {
        if ((char)(&UNK_142aae665)[uVar18] < -0x40) {
LAB_140929b08:
          FUN_1428d9940(&UNK_142aae665,0x50,0,uVar18,&UNK_142aae738);
          goto LAB_140929b28;
        }
      }
      else if (uVar18 != 0x50) goto LAB_140929b08;
    }
    puVar21 = &UNK_142aae665;
    lVar11 = FUN_140923920(&UNK_142aae665);
    if (lVar11 == 1) {
      uVar22 = uVar18 + 2;
      if (uVar22 != 0) {
        uVar24 = uVar22 == 0x50;
        if (0x4f < uVar22) goto LAB_140929b89;
        uVar24 = (&UNK_142aae667)[uVar18] == -0x41;
        if ((char)(&UNK_142aae667)[uVar18] < -0x40) {
          do {
            uVar18 = 0x50;
            FUN_1428d9940(&UNK_142aae665,0x50);
LAB_140929b89:
          } while (!(bool)uVar24);
        }
      }
      lVar19 = 0x4e - uVar18;
      puVar21 = &UNK_142aae665 + uVar22;
    }
  }
  puVar12 = &UNK_142aaf586;
  puVar23 = &UNK_142aaf53c;
  do {
    bVar4 = puVar12[-1];
    uVar16 = (uint)(char)bVar4;
    if ((char)bVar4 < '\0') {
      bVar5 = puVar12[-2];
      if ((char)bVar5 < -0x40) {
        bVar6 = puVar12[-3];
        if ((char)bVar6 < -0x40) {
          pbVar1 = puVar12 + -4;
          puVar12 = puVar12 + -4;
          uVar16 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          puVar12 = puVar12 + -3;
          uVar16 = bVar6 & 0xf;
        }
        uVar16 = bVar5 & 0x3f | uVar16 << 6;
      }
      else {
        puVar12 = puVar12 + -2;
        uVar16 = bVar5 & 0x1f;
      }
      uVar16 = (uint)(bVar4 & 0x3f) | uVar16 << 6;
      if (uVar16 != 0x5c) goto LAB_140929c29;
LAB_140929c3f:
      puVar13 = puVar12 + -0x142aaf53c;
      puVar23 = puVar12 + -0x142aaf53b;
      if (puVar23 != (undefined *)0x0) {
        uVar24 = puVar23 == (undefined *)0x4a;
        if ((undefined *)0x49 < puVar23) goto LAB_140929c81;
        uVar24 = puVar12[1] == -0x41;
        if ((char)puVar12[1] < -0x40) {
          do {
            puVar13 = (undefined *)FUN_1428d9940(&UNK_142aaf53c,0x4a);
LAB_140929c81:
          } while (!(bool)uVar24);
        }
      }
      lVar11 = 0x49 - (longlong)puVar13;
      puVar23 = &UNK_142aaf53c + (longlong)puVar23;
      goto LAB_140929c8f;
    }
    puVar12 = puVar12 + -1;
    if (uVar16 == 0x5c) goto LAB_140929c3f;
LAB_140929c29:
    if (uVar16 == 0x2f) goto LAB_140929c3f;
  } while (puVar12 != &UNK_142aaf53c);
  lVar11 = 0x4a;
LAB_140929c8f:
  plVar15 = (longlong *)func_0x000140c18130();
  if (*plVar15 == 1) {
    plVar15 = plVar15 + 1;
  }
  else if ((*plVar15 != 0) ||
          (plVar15 = (longlong *)FUN_1428f87d0(plVar15,0), plVar15 == (longlong *)0x0))
  goto LAB_140929ff9;
  if (*plVar15 == 0) {
    *plVar15 = -1;
    plStack_70 = plVar15;
    uStack_c8 = FUN_140c16d40(plVar15 + 1,puVar21,lVar19,puVar23,lVar11,1,0);
    *plStack_70 = *plStack_70 + 1;
    uStack_d0 = 1;
    uStack_98 = uStack_c8;
LAB_140929cfa:
    lStack_80 = *param_1;
    lVar11 = param_1[1];
    puVar20 = *(uint **)(lStack_80 + 0xf8);
    puVar2 = puVar20 + *(longlong *)(lStack_80 + 0x100);
    cVar7 = *(char *)(lStack_80 + 0x108);
    plStack_70 = (longlong *)*param_2;
    uStack_b0 = param_2[1];
    uStack_a8 = param_2[3];
    uStack_a0 = param_2[2];
    lStack_88 = 8;
    uVar18 = 0;
    lStack_90 = 8;
    uVar22 = 0;
    lStack_78 = 0;
    bVar10 = false;
    while (cVar7 == '\0') {
      if (uVar22 == uVar18) {
        do {
          if (puVar20 == puVar2) goto LAB_140929f73;
          uVar16 = *puVar20;
          puVar20 = puVar20 + 1;
          uVar18 = *(ulonglong *)(*(longlong *)(lVar11 + 0x100) + 0x58 + (ulonglong)uVar16 * 0xa0);
        } while (uVar18 == 0);
        lVar19 = *(longlong *)(lVar11 + 0x100) + (ulonglong)uVar16 * 0xa0;
        uVar22 = *(ulonglong *)(lStack_80 + 0x118);
        if ((uVar22 < *(ulonglong *)(lVar19 + 0x88)) &&
           (*(longlong *)(*(longlong *)(lVar19 + 0x80) + uVar22 * 8) != 0)) {
          uVar17 = (ulonglong)*(uint *)(lVar19 + 0x94);
          lVar8 = *(longlong *)(lVar11 + 0x1a8);
          bVar10 = true;
          if ((uVar22 < *(ulonglong *)(lVar8 + 0x40 + uVar17 * 0x48)) &&
             (uVar22 = *(ulonglong *)(*(longlong *)(lVar8 + 0x38 + uVar17 * 0x48) + uVar22 * 8),
             uVar22 != 0)) {
            lStack_78 = *(longlong *)
                         (*(longlong *)(lVar8 + uVar17 * 0x48 + 0x18) + 0x10 + ~uVar22 * 0x30);
          }
          else {
            lStack_78 = 0;
          }
        }
        else {
          bVar10 = false;
        }
        lStack_90 = *(longlong *)(lVar19 + 0x50);
        uVar22 = 0;
      }
      uVar14 = *(undefined8 *)(lStack_90 + uVar22 * 0x10);
      if (!bVar10) goto LAB_140929d70;
      uVar17 = (ulonglong)*(uint *)(uVar22 * 0x10 + lStack_90 + 8);
LAB_140929f3f:
      lVar19 = uVar17 * 0x1c + lStack_78;
      uVar22 = uVar22 + 1;
      if ((lVar19 == 0) || ((fVar3 = *(float *)(lVar19 + 0x14), fVar3 == 0.0 && (!NAN(fVar3))))) {
LAB_140929d73:
        uStack_f8 = plStack_70;
        uStack_f0 = uStack_b0;
        uStack_e8 = uStack_a0;
        uStack_e0 = uStack_a8;
        uStack_d8 = uVar14;
        FUN_140d14770(&uStack_f8);
      }
    }
    if (uVar22 == uVar18) {
      do {
        if (puVar20 == puVar2) goto LAB_140929f73;
        uVar16 = *puVar20;
        puVar20 = puVar20 + 1;
        uVar18 = *(ulonglong *)(*(longlong *)(lVar11 + 0x1a8) + 0x10 + (ulonglong)uVar16 * 0x48);
      } while (uVar18 == 0);
      lVar19 = *(longlong *)(lVar11 + 0x1a8) + (ulonglong)uVar16 * 0x48;
      if ((*(ulonglong *)(lStack_80 + 0x118) < *(ulonglong *)(lVar19 + 0x40)) &&
         (uVar22 = *(ulonglong *)
                    (*(longlong *)(lVar19 + 0x38) + *(ulonglong *)(lStack_80 + 0x118) * 8),
         uVar22 != 0)) {
        lStack_78 = *(longlong *)(*(longlong *)(lVar19 + 0x18) + 0x10 + ~uVar22 * 0x30);
        bVar10 = true;
      }
      else {
        bVar10 = false;
      }
      lStack_88 = *(longlong *)(lVar19 + 8);
      uVar22 = 0;
    }
    uVar14 = *(undefined8 *)(lStack_88 + uVar22 * 8);
    if (bVar10) {
      uVar17 = uVar22 & 0xffffffff;
      goto LAB_140929f3f;
    }
LAB_140929d70:
    uVar22 = uVar22 + 1;
    goto LAB_140929d73;
  }
  goto LAB_14092a005;
LAB_140929f73:
  if (cStack_61 == '\0') {
    return;
  }
  plVar15 = (longlong *)func_0x000140c18130();
  if (*plVar15 == 1) {
    plVar15 = plVar15 + 1;
  }
  else if ((*plVar15 != 0) ||
          (plVar15 = (longlong *)FUN_1428f87d0(plVar15,0), plVar15 == (longlong *)0x0)) {
LAB_140929ff9:
    FUN_1428d80d0(&UNK_142c3aaa0);
    goto LAB_14092a005;
  }
  if (*plVar15 == 0) {
    *plVar15 = -1;
    plStack_70 = plVar15;
    FUN_140c170f0(plVar15 + 1,uStack_98);
    *plStack_70 = *plStack_70 + 1;
    return;
  }
LAB_14092a005:
  FUN_1428d9250(&UNK_142c3ab70);
  pcVar9 = (code *)swi(3);
  (*pcVar9)();
  return;
}

