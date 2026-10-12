// FUN_1409cb7b0  0x1409cb7b0

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1409cb7b0(undefined8 param_1,longlong param_2)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  code *pcVar5;
  char cVar6;
  undefined4 uVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 uVar10;
  undefined *puVar11;
  undefined *puVar12;
  longlong *plVar13;
  longlong *plVar14;
  uint uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  undefined *puVar18;
  undefined *puVar19;
  longlong lVar20;
  undefined1 uVar21;
  undefined1 auStack_f48 [1000];
  undefined4 uStack_b60;
  undefined8 uStack_b5c;
  undefined8 uStack_b54;
  undefined4 uStack_b4c;
  undefined4 *puStack_b48;
  undefined1 auStack_778 [168];
  byte bStack_6d0;
  undefined4 auStack_680 [62];
  undefined4 uStack_588;
  undefined4 uStack_584;
  undefined4 uStack_580;
  undefined4 uStack_57c;
  undefined4 uStack_578;
  undefined4 uStack_574;
  undefined1 *puStack_570;
  undefined8 uStack_568;
  undefined8 uStack_560;
  undefined8 uStack_558;
  undefined8 uStack_550;
  undefined8 uStack_548;
  undefined4 uStack_518;
  undefined4 uStack_514;
  undefined4 uStack_510;
  undefined4 uStack_50c;
  undefined8 uStack_508;
  undefined8 uStack_500;
  undefined4 uStack_4f8;
  undefined4 uStack_4f4;
  undefined4 uStack_4f0;
  undefined4 uStack_4ec;
  undefined *puStack_4e8;
  undefined4 uStack_4e0;
  undefined4 uStack_4dc;
  undefined2 uStack_4d8;
  undefined4 auStack_1a0 [63];
  undefined1 auStack_a4 [4];
  undefined1 auStack_a0 [48];
  longlong lStack_70;
  undefined8 uStack_68;
  longlong *plStack_60;
  longlong *plStack_58;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar9 = param_2 + 0x38;
  uVar16 = 0x1e9ac9b6648cf0a6;
  lVar8 = FUN_1400dd6f0(lVar9,0x1e9ac9b6648cf0a6,0xe6222c4af0fcfd90);
  if (((lVar8 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar16)) ||
     (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 == 0)) {
code_r0x0001409cb8c4:
    if (cRam0000000143634cf8 == '\0') {
      return;
    }
    uVar16 = 0xa68561b396784eaa;
    lVar9 = FUN_1400dd6f0(lVar9,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
    if (((lVar9 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 != 0)) {
      lVar9 = ~uVar16 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) != 0) {
        FUN_14093cf30(&uStack_588,*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10),
                      0x10);
        uVar10 = func_0x000140bd5790(&uStack_584);
        uStack_b60 = 1;
        uStack_b5c = 0;
        uStack_b54 = 0;
        uStack_b4c = 0;
        auStack_1a0[0] = 2;
        puStack_b48 = &uStack_588;
        FUN_140867970(auStack_f48,&UNK_142ac0c50,0x11,&uStack_b60,1,auStack_1a0,uVar10,param_2,
                      param_1);
        FUN_140868370(auStack_f48);
        cRam0000000143634cf8 = 0;
        return;
      }
    }
code_r0x0001409cc027:
    FUN_1428d9310(&UNK_142ac0c68);
  }
  else {
    lVar8 = ~uVar16 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) == 0)
    goto code_r0x0001409cb8c4;
    plVar13 = *(longlong **)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10);
    cVar6 = func_0x000140c16a00();
    if (cVar6 != '\0') {
      puVar18 = &UNK_142ac07eb;
      lVar20 = 0x69;
      uVar16 = 0x69;
      lVar8 = FUN_1409c8970(&UNK_142ac07eb);
      if (lVar8 == 1) {
        if (uVar16 != 0) {
          if (uVar16 < 0x69) {
            if ((char)(&UNK_142ac07eb)[uVar16] < -0x40) {
code_r0x0001409cb8a4:
              FUN_1428d9940(&UNK_142ac07eb,0x69,0,uVar16,&UNK_142ac0920);
              goto code_r0x0001409cb8c4;
            }
          }
          else if (uVar16 != 0x69) goto code_r0x0001409cb8a4;
        }
        puVar18 = &UNK_142ac07eb;
        lVar8 = FUN_1409c8970(&UNK_142ac07eb);
        if (lVar8 == 1) {
          uVar17 = uVar16 + 2;
          if (uVar17 != 0) {
            uVar21 = uVar17 == 0x69;
            if (0x68 < uVar17) goto code_r0x0001409cba5b;
            uVar21 = (&UNK_142ac07ed)[uVar16] == -0x41;
            if ((char)(&UNK_142ac07ed)[uVar16] < -0x40) {
              do {
                uVar16 = 0x69;
                FUN_1428d9940(&UNK_142ac07eb,0x69);
code_r0x0001409cba5b:
              } while (!(bool)uVar21);
            }
          }
          lVar20 = 0x67 - uVar16;
          puVar18 = &UNK_142ac07eb + uVar17;
        }
      }
      puVar11 = &UNK_142ac0be9;
      puVar19 = &UNK_142ac0b98;
      do {
        bVar2 = puVar11[-1];
        uVar15 = (uint)(char)bVar2;
        plStack_58 = plVar13;
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
          if (uVar15 != 0x5c) goto code_r0x0001409cbb09;
code_r0x0001409cbb1e:
          puVar12 = puVar11 + -0x142ac0b98;
          puVar19 = puVar11 + -0x142ac0b97;
          if (puVar19 != (undefined *)0x0) {
            uVar21 = puVar19 == (undefined *)0x51;
            if ((undefined *)0x50 < puVar19) goto code_r0x0001409cbb60;
            uVar21 = puVar11[1] == -0x41;
            if ((char)puVar11[1] < -0x40) {
              do {
                puVar12 = (undefined *)FUN_1428d9940(&UNK_142ac0b98,0x51);
code_r0x0001409cbb60:
              } while (!(bool)uVar21);
            }
          }
          lVar8 = 0x50 - (longlong)puVar12;
          puVar19 = &UNK_142ac0b98 + (longlong)puVar19;
          goto code_r0x0001409cbb6d;
        }
        puVar11 = puVar11 + -1;
        if (uVar15 == 0x5c) goto code_r0x0001409cbb1e;
code_r0x0001409cbb09:
        if (uVar15 == 0x2f) goto code_r0x0001409cbb1e;
      } while (puVar11 != &UNK_142ac0b98);
      lVar8 = 0x51;
code_r0x0001409cbb6d:
      plVar13 = (longlong *)func_0x000140c18130();
      if (*plVar13 == 1) {
        plVar13 = plVar13 + 1;
code_r0x0001409cbb86:
        if (*plVar13 == 0) {
          *plVar13 = -1;
          plStack_60 = plVar13;
          uStack_68 = FUN_140c16d40(plVar13 + 1,puVar18,lVar20,puVar19,lVar8,1,0);
          *plStack_60 = *plStack_60 + 1;
          lStack_70 = 1;
          plVar13 = plStack_58;
          goto code_r0x0001409cbbe4;
        }
      }
      else {
        if ((*plVar13 == 0) &&
           (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0), plVar13 != (longlong *)0x0))
        goto code_r0x0001409cbb86;
code_r0x0001409cc00f:
        FUN_1428d80d0(&UNK_142c3aaa0);
      }
code_r0x0001409cc01b:
      FUN_1428d9250(&UNK_142c3ab70);
      goto code_r0x0001409cc027;
    }
    lStack_70 = 0;
code_r0x0001409cbbe4:
    uVar16 = 0x8597ee0727b461b3;
    lVar8 = FUN_1400dd6f0(lVar9,0x8597ee0727b461b3,0x8b297ec5ae15f15c);
    if (((lVar8 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 != 0)) {
      lVar8 = ~uVar16 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar8) != 0) {
        puVar18 = &UNK_142ac0c08;
        uVar16 = FUN_140937aa0(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar8 + 0x10),
                               &UNK_142ac0c08,0x18);
        if ((uVar16 & 1) == 0) {
          FUN_1428d9310(&UNK_142ac0c20);
          goto code_r0x0001409cc070;
        }
        uVar16 = 0xa68561b396784eaa;
        lVar9 = FUN_1400dd6f0(lVar9,0xa68561b396784eaa,0xf5fd74e6faf4ed14);
        if (((lVar9 != 0) && (uVar16 < *(ulonglong *)(param_2 + 0x218))) &&
           (uVar16 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar16 * 8), uVar16 != 0)) {
          lVar9 = ~uVar16 * 0x60;
          if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar9) != 0) {
            FUN_14093cf30(auStack_a4,*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar9 + 0x10),
                          0x10);
            uVar10 = func_0x000140bd5790(auStack_a0);
            uStack_588 = 1;
            uStack_584 = 0;
            uStack_580 = 0;
            uStack_57c = 0;
            uStack_578 = 0;
            uStack_574 = 0;
            auStack_1a0[0] = 2;
            puStack_570 = auStack_a4;
            FUN_140867970(&uStack_b60,&UNK_142ac0c50,0x11,&uStack_588,1,auStack_1a0,uVar10,param_2,
                          param_1);
            uStack_49 = 1;
            uVar7 = func_0x000140be6750();
            uStack_49 = 1;
            plVar14 = (longlong *)FUN_1400d9b20(0);
            if (plVar14 == (longlong *)0x0) {
              uStack_49 = 1;
              FUN_1428d80d0(&UNK_142c3aaa0);
              goto code_r0x0001409cc070;
            }
            uStack_4d8 = 0;
            uStack_4dc = 0x3f800000;
            uStack_588 = 0;
            uStack_584 = 0;
            uStack_578 = 0;
            uStack_574 = 0;
            puStack_570 = (undefined1 *)0x4;
            uStack_568 = 0;
            uStack_4f8 = (undefined4)*plVar14;
            uStack_4f4 = *(undefined4 *)((longlong)plVar14 + 4);
            uStack_4f0 = (undefined4)plVar14[1];
            uStack_4ec = *(undefined4 *)((longlong)plVar14 + 0xc);
            *plVar14 = *plVar14 + 1;
            uStack_508 = _UNK_142a96730;
            uStack_500 = _UNK_142a96738;
            uStack_518 = _UNK_142a96720;
            uStack_514 = _UNK_142a96724;
            uStack_510 = _UNK_142a96728;
            uStack_50c = _UNK_142a9672c;
            uStack_548 = 0x100000003;
            uStack_560 = 0;
            uStack_558 = 1;
            uStack_550 = 0;
            uStack_49 = 1;
            puStack_4e8 = puVar18;
            uStack_4e0 = uVar7;
            FUN_140905be0(auStack_778,&uStack_588,&uStack_b60,0);
            bStack_6d0 = bStack_6d0 & 0x73;
            func_0x000142923c00(&uStack_588,auStack_778,0xf8);
            uStack_49 = 1;
            FUN_140906400(auStack_1a0,&uStack_588,0x92930b0d61fef16e,plVar13);
            auStack_680[0] = CONCAT31(auStack_680[0]._1_3_,9);
            uStack_588 = 2;
            FUN_140908c50(auStack_680,auStack_1a0,0x61b08b5e0470c5f2,0x1d,&uStack_588);
            func_0x000142923c00(&uStack_588,auStack_1a0,0xf8);
            uStack_4a = 10;
            auStack_680[0] = 2;
            FUN_140908c50(&uStack_4a,&uStack_588,0x297d1e4cdae8f657,0x1d,auStack_680);
            func_0x000142923c00(auStack_680,&uStack_588,0xf8);
            uStack_49 = 1;
            FUN_140906a90(auStack_680,*(undefined4 *)((longlong)plVar13 + 0x28));
            func_0x000142923c00(&uStack_588,&uStack_b60,1000);
            uStack_49 = 0;
            FUN_140868370(&uStack_588);
            cRam0000000143634cf8 = '\x01';
            if (lStack_70 == 0) {
              cRam0000000143634cf8 = 1;
              return;
            }
            plVar13 = (longlong *)func_0x000140c18130();
            if (*plVar13 == 1) {
              plVar13 = plVar13 + 1;
            }
            else if ((*plVar13 != 0) ||
                    (plVar13 = (longlong *)FUN_1428f87d0(plVar13,0), plVar13 == (longlong *)0x0))
            goto code_r0x0001409cc00f;
            if (*plVar13 == 0) {
              *plVar13 = -1;
              plStack_58 = plVar13;
              FUN_140c170f0(plVar13 + 1,uStack_68);
              *plStack_58 = *plStack_58 + 1;
              return;
            }
            goto code_r0x0001409cc01b;
          }
        }
        FUN_1428d9310(&UNK_142ac0c38);
        goto code_r0x0001409cc070;
      }
    }
  }
  FUN_1428d9310(&UNK_142ac0bf0);
code_r0x0001409cc070:
                    /* WARNING: Does not return */
  pcVar5 = (code *)invalidInstructionException();
  (*pcVar5)();
}

