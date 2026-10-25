// FUN_1409cc270  0x1409cc270

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409cc270(undefined8 param_1,longlong param_2,longlong param_3)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  ulonglong *puVar5;
  code *pcVar6;
  float fVar7;
  char cVar8;
  uint uVar9;
  longlong lVar10;
  undefined *puVar11;
  undefined *puVar12;
  undefined8 uVar13;
  longlong *plVar14;
  longlong *plVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  longlong lVar18;
  undefined *puVar19;
  undefined *puVar20;
  undefined1 uVar21;
  longlong lStack_9a8;
  undefined1 *puStack_9a0;
  undefined8 uStack_998;
  undefined1 auStack_5c0 [1000];
  longlong lStack_1d8;
  longlong lStack_1d0;
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
  longlong lStack_138;
  uint uStack_130;
  undefined4 uStack_12c;
  undefined2 uStack_128;
  undefined1 auStack_dc [4];
  undefined1 auStack_d8 [48];
  ulonglong uStack_a8;
  undefined8 uStack_a0;
  longlong lStack_98;
  longlong *plStack_90;
  float fStack_88;
  char cStack_84;
  longlong lStack_80;
  undefined8 uStack_78;
  longlong *plStack_70;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  uStack_a0 = param_1;
  cVar8 = func_0x000140c16a00();
  if (cVar8 == '\0') {
code_r0x0001409cc312:
    lStack_80 = 0;
code_r0x0001409cc4ed:
    lStack_98 = param_2 + 0x38;
    uVar16 = 0xa68561b396784eaa;
    lVar10 = FUN_1400dd6f0(lStack_98,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
    if (((lVar10 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 != 0)) {
      lVar10 = ~uVar16 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar10) != 0) {
        FUN_14093cf30(auStack_dc,*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar10 + 0x10),
                      0x11);
        fVar7 = _UNK_1429a7640;
        plStack_90 = *(longlong **)(param_3 + 8);
        plStack_70 = plStack_90 + *(longlong *)(param_3 + 0x10) * 0x10;
        uVar16 = 0;
        do {
          uVar13 = func_0x000140bd5790(auStack_d8);
          lStack_9a8 = uVar16 << 0x20;
          puStack_9a0 = auStack_dc;
          uStack_998 = 0x3f80000000000001;
          FUN_140867970(auStack_5c0,&UNK_142ac0ce0,7,&lStack_1d8,0,&lStack_9a8,uVar13,param_2,
                        uStack_a0);
          uStack_61 = 1;
          uVar17 = 0x4bc362195b80f729;
          lVar10 = FUN_1400dd6f0(lStack_98,0x4bc362195b80f729,0x982798ce80cbcefb);
          if (((lVar10 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar17)) ||
             (uVar17 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar17 * 8), uVar17 == 0))
          goto code_r0x0001409cc973;
          lVar10 = ~uVar17 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar10) == 0)
          goto code_r0x0001409cc973;
          puVar5 = *(ulonglong **)(*(longlong *)(param_2 + 0x1e0) + lVar10 + 0x10);
          uStack_a8 = uVar16 + 1;
          cStack_84 = (char)uVar16 + '\x02';
          fStack_88 = (float)(int)puVar5[1] / fVar7 + (float)*puVar5;
          plVar15 = plStack_90;
          if (plStack_90 != plStack_70) {
            do {
              lVar10 = plVar15[10];
              lVar18 = plVar15[0xb];
              uStack_61 = 1;
              uVar9 = func_0x000140be6750();
              uStack_61 = 1;
              plVar14 = (longlong *)FUN_1400d9b20(0);
              if (plVar14 == (longlong *)0x0) goto code_r0x0001409cc952;
              uStack_128 = 0;
              uStack_12c = 0x3f800000;
              lStack_1d8 = 1;
              uStack_1c8 = 0;
              uStack_1c0 = 4;
              uStack_1b8 = 0;
              uStack_148 = (undefined4)*plVar14;
              uStack_144 = *(undefined4 *)((longlong)plVar14 + 4);
              uStack_140 = (undefined4)plVar14[1];
              uStack_13c = *(undefined4 *)((longlong)plVar14 + 0xc);
              *plVar14 = *plVar14 + 1;
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
              uStack_61 = 1;
              lStack_1d0 = lVar10;
              lStack_138 = lVar18;
              uStack_130 = uVar9;
              FUN_140905be0(&lStack_9a8,&lStack_1d8,auStack_5c0,1);
              uStack_61 = 1;
              FUN_14090b460(&lStack_1d8,&lStack_9a8,&fStack_88,plVar15);
              uStack_130 = uStack_130 & 0xffffffcf;
              pcVar6 = (code *)plVar15[0xe];
              func_0x000142923c00(&lStack_9a8,&lStack_1d8,0xf8);
              uStack_61 = 1;
              (*pcVar6)(&lStack_9a8,plVar15,param_2);
              plVar15 = plVar15 + 0x10;
            } while (plVar15 != plStack_70);
          }
          func_0x000142923c00(&lStack_9a8,auStack_5c0,1000);
          uStack_61 = 0;
          FUN_140868370(&lStack_9a8);
          uVar16 = uStack_a8;
        } while (uStack_a8 < 3);
        if (lStack_80 == 0) {
          return;
        }
        plVar15 = (longlong *)func_0x000140c18130();
        if (*plVar15 == 1) {
          plVar15 = plVar15 + 1;
        }
        else if ((*plVar15 != 0) ||
                (plVar15 = (longlong *)FUN_1428f87d0(plVar15,0), plVar15 == (longlong *)0x0))
        goto code_r0x0001409cc946;
        if (*plVar15 == 0) {
          *plVar15 = -1;
          plStack_70 = plVar15;
          FUN_140c170f0(plVar15 + 1,uStack_78);
          *plStack_70 = *plStack_70 + 1;
          return;
        }
        goto code_r0x0001409cc967;
      }
    }
    FUN_1428d9310(&UNK_142ac0cc8);
  }
  else {
    puVar19 = &UNK_142ac0758;
    lVar18 = 0x49;
    uVar16 = 0x49;
    lVar10 = FUN_1409c8970(&UNK_142ac0758);
    if (lVar10 == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x49) {
          if ((char)(&UNK_142ac0758)[uVar16] < -0x40) {
code_r0x0001409cc2f2:
            FUN_1428d9940(&UNK_142ac0758,0x49,0,uVar16,&UNK_142ac0920);
            goto code_r0x0001409cc312;
          }
        }
        else if (uVar16 != 0x49) goto code_r0x0001409cc2f2;
      }
      puVar19 = &UNK_142ac0758;
      lVar10 = FUN_1409c8970(&UNK_142ac0758);
      if (lVar10 == 1) {
        uVar17 = uVar16 + 2;
        if (uVar17 != 0) {
          uVar21 = uVar17 == 0x49;
          if (0x48 < uVar17) goto code_r0x0001409cc373;
          uVar21 = (&UNK_142ac075a)[uVar16] == -0x41;
          if ((char)(&UNK_142ac075a)[uVar16] < -0x40) {
            do {
              uVar16 = 0x49;
              FUN_1428d9940(&UNK_142ac0758,0x49);
code_r0x0001409cc373:
            } while (!(bool)uVar21);
          }
        }
        lVar18 = 0x47 - uVar16;
        puVar19 = &UNK_142ac0758 + uVar17;
      }
    }
    puVar11 = &UNK_142ac0cc6;
    puVar20 = &UNK_142ac0c80;
    do {
      bVar2 = puVar11[-1];
      uVar9 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar11[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar11[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar11 + -4;
            puVar11 = puVar11 + -4;
            uVar9 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar11 = puVar11 + -3;
            uVar9 = bVar4 & 0xf;
          }
          uVar9 = bVar3 & 0x3f | uVar9 << 6;
        }
        else {
          puVar11 = puVar11 + -2;
          uVar9 = bVar3 & 0x1f;
        }
        uVar9 = (uint)(bVar2 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto code_r0x0001409cc419;
code_r0x0001409cc42e:
        puVar12 = puVar11 + -0x142ac0c80;
        puVar20 = puVar11 + -0x142ac0c7f;
        if (puVar20 != (undefined *)0x0) {
          uVar21 = puVar20 == (undefined *)0x46;
          if ((undefined *)0x45 < puVar20) goto code_r0x0001409cc470;
          uVar21 = puVar11[1] == -0x41;
          if ((char)puVar11[1] < -0x40) {
            do {
              puVar12 = (undefined *)FUN_1428d9940(&UNK_142ac0c80,0x46);
code_r0x0001409cc470:
            } while (!(bool)uVar21);
          }
        }
        lVar10 = 0x45 - (longlong)puVar12;
        puVar20 = &UNK_142ac0c80 + (longlong)puVar20;
        goto code_r0x0001409cc47d;
      }
      puVar11 = puVar11 + -1;
      if (uVar9 == 0x5c) goto code_r0x0001409cc42e;
code_r0x0001409cc419:
      if (uVar9 == 0x2f) goto code_r0x0001409cc42e;
    } while (puVar11 != &UNK_142ac0c80);
    lVar10 = 0x46;
code_r0x0001409cc47d:
    plVar15 = (longlong *)func_0x000140c18130();
    if (*plVar15 != 1) {
      if ((*plVar15 == 0) &&
         (plVar15 = (longlong *)FUN_1428f87d0(plVar15,0), plVar15 != (longlong *)0x0))
      goto code_r0x0001409cc496;
code_r0x0001409cc946:
      FUN_1428d80d0(&UNK_142c3aaa0);
code_r0x0001409cc952:
      uStack_61 = 1;
      FUN_1428d80d0(&UNK_142c3aaa0);
      goto code_r0x0001409cc994;
    }
    plVar15 = plVar15 + 1;
code_r0x0001409cc496:
    if (*plVar15 == 0) {
      *plVar15 = -1;
      plStack_70 = plVar15;
      uStack_78 = FUN_140c16d40(plVar15 + 1,puVar19,lVar18,puVar20,lVar10,1,0);
      *plStack_70 = *plStack_70 + 1;
      lStack_80 = 1;
      goto code_r0x0001409cc4ed;
    }
code_r0x0001409cc967:
    FUN_1428d9250(&UNK_142c3ab70);
code_r0x0001409cc973:
    uStack_61 = 1;
    FUN_1428d9310(&UNK_142aa5bd8);
  }
code_r0x0001409cc994:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

