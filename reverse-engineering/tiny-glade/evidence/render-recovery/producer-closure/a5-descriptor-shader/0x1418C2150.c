
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x000142d063b0) overlaps instruction at (ram,0x000142d063af)
    */

undefined4 *
FUN_1418c2150(undefined4 *param_1,longlong param_2,longlong *param_3,undefined4 param_4,
             longlong *param_5)

{
  longlong lVar1;
  byte bVar2;
  longlong lVar3;
  code *pcVar4;
  undefined4 *puVar5;
  char cVar6;
  int iVar7;
  longlong lVar8;
  undefined4 *puVar9;
  ulonglong uVar10;
  undefined *puVar11;
  undefined8 *puVar12;
  char *pcVar13;
  ulonglong uVar14;
  longlong **pplVar15;
  uint uVar16;
  longlong lVar17;
  undefined4 uVar18;
  ulonglong uVar19;
  undefined8 *puVar20;
  undefined8 uVar21;
  undefined4 uVar22;
  ulonglong uVar23;
  undefined *puStack_f0;
  longlong **pplStack_e8;
  longlong **pplStack_e0;
  longlong lStack_d8;
  undefined8 uStack_d0;
  undefined4 *puStack_c0;
  longlong *plStack_b8;
  code *pcStack_b0;
  undefined4 uStack_a8;
  undefined4 *puStack_a0;
  longlong lStack_98;
  ulonglong uStack_90;
  ulonglong uStack_88;
  longlong lStack_80;
  undefined4 *puStack_78;
  ulonglong uStack_70;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined8 uStack_58;
  ulonglong uStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_64 = param_4;
  lStack_98 = param_2;
  lVar17 = *param_3;
  lVar3 = param_3[1];
  uStack_88 = lVar3 - lVar17;
  uStack_70 = uStack_88 / 0x28;
  lVar1 = (uStack_88 / 0x28) * 0x18;
  puStack_c0 = param_1;
  if (uStack_88 < 0xd555555555555549) {
    if (lVar3 == lVar17) {
      uStack_60 = 0;
      uStack_58 = 8;
      uStack_50 = 0;
      puStack_78 = (undefined4 *)0x4;
    }
    else {
      uVar21 = 8;
      lVar8 = func_0x000140613c10(lVar1,8);
      uVar23 = uStack_70;
      if (lVar8 == 0) goto code_r0x0001418c2708;
      uStack_60 = uStack_70;
      uStack_50 = 0;
      lVar1 = uStack_70 * 4;
      uStack_58 = lVar8;
      puVar9 = (undefined4 *)func_0x000140613c10(lVar1,4);
      if (puVar9 == (undefined4 *)0x0) {
        FUN_1428d8fe3(4,lVar1,&UNK_142d062b0);
        goto code_r0x0001418c275b;
      }
      puStack_78 = puVar9;
      if (0x28 < uStack_88) {
        func_0x000142923c20(puVar9,0,lVar1 + -4);
        puVar9 = puVar9 + (uVar23 - 1);
      }
      *puVar9 = 0;
    }
    uVar23 = param_3[2];
    uStack_90 = 0;
    for (; uVar14 = uStack_50, puVar9 = puStack_78, lVar17 != lVar3; lVar17 = lVar17 + 0x28) {
      lStack_80 = lVar17;
      uVar22 = (undefined4)uVar23;
      if ((*(byte *)(lVar17 + 0x18) & 1) != 0) {
        uVar10 = (ulonglong)*(uint *)(*(longlong *)(lStack_98 + 0x658) + 0x680);
        uVar19 = 0;
        if (0x1f < uVar10) {
          uVar19 = uVar10 - 0x20;
        }
        if (0x1fff < uVar19) {
          uVar19 = 0x2000;
        }
        if ((int)uVar19 == 1) goto code_r0x0001418c2358;
code_r0x0001418c231f:
        if ((int)uVar19 == 0) {
          plStack_b8 = &lStack_80;
          pcStack_b0 = FUN_14213e060;
          puStack_f0 = &UNK_142d062e0;
          pplStack_e8 = (longlong **)0x1;
          uStack_d0 = 0;
          pplStack_e0 = &plStack_b8;
          lStack_d8 = 1;
          FUN_1428d9390(&puStack_f0,&UNK_142d062f0);
        }
        else {
          if (uStack_50 < uStack_70) {
            puStack_78[uStack_50] = 4;
            bVar2 = *(byte *)(lVar17 + 0x20);
            uVar10 = (ulonglong)bVar2;
            uVar16 = (uint)bVar2;
            puVar11 = &UNK_142d063b0 + *(int *)(&UNK_142d063b0 + uVar10 * 4);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
            switch(bVar2) {
            case 0:
              goto code_r0x0001418c2365;
            case 1:
              goto code_r0x0001418c240b;
            case 2:
              goto code_r0x0001418c2473;
            case 3:
              goto code_r0x0001418c2450;
            case 4:
              goto code_r0x0001418c23a6;
            case 5:
              goto code_r0x0001418c24ad;
            case 6:
            case 7:
              goto code_r0x000142d063b0;
            default:
              goto code_r0x000142d063af;
            }
          }
          FUN_1428d9518(uStack_50,uStack_70,&UNK_142d06308);
        }
        goto code_r0x0001418c275b;
      }
      uVar19 = (ulonglong)*(uint *)(lVar17 + 0x1c);
      if (*(uint *)(lVar17 + 0x1c) != 1) goto code_r0x0001418c231f;
code_r0x0001418c2358:
      bVar2 = *(byte *)(lVar17 + 0x20);
      uVar10 = (ulonglong)bVar2;
      uVar16 = (uint)bVar2;
      puVar11 = &UNK_142d063b0 + *(int *)(&UNK_142d063b0 + uVar10 * 4);
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
      switch(bVar2) {
      case 0:
code_r0x0001418c2365:
        uVar16 = 7;
        if (3 < *(ulonglong *)(lVar17 + 0x10)) {
          uVar16 = *(uint *)(*(longlong *)(lVar17 + 8) + -4 + *(ulonglong *)(lVar17 + 0x10));
          uVar16 = (uint)((uVar16 >> 0x18 | (uVar16 & 0xff0000) >> 8 | (uVar16 & 0xff00) << 8 |
                          uVar16 << 0x18) == 0x5f64796e) * 2 + 7;
        }
        break;
      case 1:
code_r0x0001418c240b:
        uVar16 = 6;
        if (3 < *(ulonglong *)(lVar17 + 0x10)) {
          uVar16 = *(uint *)(*(longlong *)(lVar17 + 8) + -4 + *(ulonglong *)(lVar17 + 0x10));
          uVar16 = (uint)((uVar16 >> 0x18 | (uVar16 & 0xff0000) >> 8 | (uVar16 & 0xff00) << 8 |
                          uVar16 << 0x18) == 0x5f64796e) * 2 + 6;
        }
        break;
      case 2:
code_r0x0001418c2473:
        uVar18 = (undefined4)uVar19;
        if (uStack_50 == uStack_60) {
          FUN_140562db0(&uStack_60,&UNK_142d06320);
        }
        *(undefined4 *)(uStack_58 + uVar14 * 0x18) = uVar22;
        *(undefined4 *)(uStack_58 + 4 + uVar14 * 0x18) = 1;
        goto code_r0x0001418c22b7;
      case 3:
        break;
      case 4:
code_r0x0001418c23a6:
        uVar18 = (undefined4)uVar19;
        if (*(int *)(lVar17 + 0x18) != 0) {
          if (uStack_70 <= uStack_50) {
            FUN_1428d9518(uStack_50,uStack_70,&UNK_142d06350);
            goto code_r0x0001418c275b;
          }
          puStack_78[uStack_50] = 0xf;
          uStack_90 = (ulonglong)((uint)uStack_90 | 2);
        }
        if (uStack_50 == uStack_60) {
          FUN_140562db0(&uStack_60,&UNK_142d06368);
        }
        *(undefined4 *)(uStack_58 + uVar14 * 0x18) = uVar22;
        *(undefined4 *)(uStack_58 + 4 + uVar14 * 0x18) = 2;
        goto code_r0x0001418c22b7;
      case 5:
code_r0x0001418c24ad:
        uVar21 = FUN_140bfc5b0(lStack_98,*(undefined8 *)(lVar17 + 8));
        lVar1 = *param_5;
        uVar16 = *(uint *)(lVar1 + 0x50);
        if ((ulonglong)uVar16 < 8) {
          *(undefined8 *)(lVar1 + 0x10 + (ulonglong)uVar16 * 8) = uVar21;
          *(uint *)(lVar1 + 0x50) = uVar16 + 1;
          uVar14 = (ulonglong)*(uint *)(*param_5 + 0x50);
          if (uVar14 == 0) {
            FUN_1428d9518(0xffffffffffffffff,0,&UNK_142feb2c8);
            goto code_r0x0001418c275b;
          }
          puVar20 = (undefined8 *)(*param_5 + (uVar14 - 1) * 8 + 0x10);
          if (uStack_50 == uStack_60) {
code_r0x0001418c2558:
            uVar14 = uStack_50;
            FUN_140562db0(&uStack_60,&UNK_142d06380);
            uStack_50 = uVar14;
          }
        }
        else {
          puVar12 = (undefined8 *)func_0x000140613c10(0x58,8);
          if (puVar12 == (undefined8 *)0x0) {
            FUN_1428d9000(8,0x58);
            goto code_r0x0001418c275b;
          }
          puVar20 = puVar12 + 2;
          puVar12[2] = uVar21;
          *(undefined4 *)(puVar12 + 10) = 1;
          lVar1 = *param_5;
          *param_5 = (longlong)puVar12;
          *puVar12 = 1;
          puVar12[1] = lVar1;
          if (uStack_50 == uStack_60) goto code_r0x0001418c2558;
        }
        *(ulonglong *)(uStack_58 + uStack_50 * 0x18) = uVar23 & 0xffffffff;
        *(undefined4 *)(uStack_58 + 8 + uStack_50 * 0x18) = 1;
        *(undefined4 *)(uStack_58 + 0xc + uStack_50 * 0x18) = uStack_64;
        *(undefined8 **)(uStack_58 + 0x10 + uStack_50 * 0x18) = puVar20;
        uVar14 = uStack_50;
        goto code_r0x0001418c22cb;
      case 6:
      case 7:
code_r0x000142d063b0:
        cRam000000007780fcfd = cRam000000007780fcfd >> 0x1e;
        cVar6 = (char)((short)puVar11 / (short)*(char *)(uVar19 - 0x3f020145));
        pcVar13 = (char *)CONCAT62((int6)((ulonglong)puVar11 >> 0x10),
                                   CONCAT11((char)((short)puVar11 %
                                                  (short)*(char *)(uVar19 - 0x3f020145)),cVar6));
        *pcVar13 = *pcVar13 + cVar6;
        *pcVar13 = *pcVar13 + cVar6;
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      default:
code_r0x000142d063af:
        *(char *)(uVar10 - 0x443f3c02) = *(char *)(uVar10 - 0x443f3c02) >> 0x1e;
        cVar6 = cRamfdfebbbff6febbc0;
        pcVar13 = (char *)CONCAT71((int7)((ulonglong)puVar11 >> 8),cRamfdfebbbff6febbc0);
        *(undefined1 *)(uVar10 + 0xfe) = *(undefined1 *)(uVar10 + 0xfe);
        *pcVar13 = *pcVar13 + cVar6;
        *pcVar13 = *pcVar13 + cVar6;
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      }
code_r0x0001418c2450:
      uVar18 = (undefined4)uVar19;
      if (uStack_50 == uStack_60) {
        FUN_140562db0(&uStack_60,&UNK_142d06338);
      }
      *(undefined4 *)(uStack_58 + uVar14 * 0x18) = uVar22;
      *(uint *)(uStack_58 + 4 + uVar14 * 0x18) = uVar16;
code_r0x0001418c22b7:
      *(undefined4 *)(uStack_58 + 8 + uVar14 * 0x18) = uVar18;
      *(undefined4 *)(uStack_58 + 0xc + uVar14 * 0x18) = uStack_64;
      *(undefined8 *)(uStack_58 + 0x10 + uVar14 * 0x18) = 0;
code_r0x0001418c22cb:
      uVar23 = uVar23 + 1;
      uStack_50 = uVar14 + 1;
    }
    plStack_b8 = (longlong *)CONCAT44(plStack_b8._4_4_,0x3b9d3ee8);
    pcStack_b0 = (code *)0x0;
    uStack_a8 = (int)uStack_70;
    puStack_a0 = puStack_78;
    pplVar15 = &plStack_b8;
    do {
      pplVar15 = (longlong **)pplVar15[1];
    } while (pplVar15 != (longlong **)0x0);
    puStack_f0 = (undefined *)CONCAT44(puStack_f0._4_4_,0x20);
    pplStack_e8 = &plStack_b8;
    pplStack_e0 = (longlong **)CONCAT44((int)uStack_50,(uint)uStack_90);
    lStack_d8 = uStack_58;
    iVar7 = (**(code **)(lStack_98 + 0x230))
                      (*(undefined8 *)(lStack_98 + 0x640),&puStack_f0,0,&lStack_80);
    puVar5 = puStack_c0;
    if (iVar7 == 0) {
      *(ulonglong *)(puStack_c0 + 4) = uStack_50;
      *puStack_c0 = (undefined4)uStack_60;
      puStack_c0[1] = uStack_60._4_4_;
      puStack_c0[2] = (undefined4)uStack_58;
      puStack_c0[3] = uStack_58._4_4_;
      *(longlong *)(puStack_c0 + 6) = lStack_80;
      if (0x27 < uStack_88) {
        func_0x000140613c20(puVar9,uStack_70 << 2,4);
      }
      return puVar5;
    }
  }
  else {
    uVar21 = 0;
code_r0x0001418c2708:
    iVar7 = FUN_1428d8fe3(uVar21,lVar1,&UNK_142d06298);
  }
  lStack_80 = CONCAT44(lStack_80._4_4_,iVar7);
  FUN_1428d9760(&UNK_142d06248,0x2b,&lStack_80,&UNK_142d06228,&UNK_142d062c8);
code_r0x0001418c275b:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

