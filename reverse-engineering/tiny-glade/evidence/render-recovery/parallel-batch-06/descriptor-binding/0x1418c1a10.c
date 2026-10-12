
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Instruction at (ram,0x000142d06398) overlaps instruction at (ram,0x000142d06397)
    */

undefined1 (*) [16]
FUN_1418c1a10(undefined1 (*param_1) [16],longlong param_2,undefined8 *param_3,undefined4 param_4,
             longlong *param_5)

{
  longlong lVar1;
  byte bVar2;
  undefined1 auVar3 [16];
  code *pcVar4;
  ushort uVar5;
  undefined1 (*pauVar6) [16];
  char cVar7;
  uint uVar8;
  int iVar9;
  ulonglong uVar10;
  undefined1 (*pauVar11) [16];
  undefined1 (*pauVar12) [16];
  undefined8 uVar13;
  undefined8 *puVar14;
  undefined1 (*pauVar15) [16];
  ulonglong uVar16;
  undefined1 (**ppauVar17) [16];
  char *pcVar18;
  ulonglong uVar19;
  undefined *puVar20;
  undefined1 (*pauVar21) [16];
  undefined1 *puVar22;
  undefined1 *puVar23;
  undefined1 (*pauVar24) [16];
  uint uVar25;
  byte *pbVar26;
  undefined1 (*pauVar27) [16];
  undefined1 (*pauVar28) [16];
  uint uVar29;
  uint uVar30;
  undefined8 *puVar31;
  undefined4 uVar32;
  bool bVar33;
  undefined1 auStack_128 [8];
  undefined1 auStack_120 [24];
  undefined *puStack_108;
  undefined *puStack_100;
  undefined1 (**ppauStack_f8) [16];
  undefined1 (**ppauStack_f0) [16];
  undefined1 (*pauStack_e8) [16];
  undefined1 (*pauStack_e0) [16];
  undefined8 uStack_d8;
  undefined1 (**ppauStack_d0) [16];
  undefined1 (*pauStack_c8) [16];
  undefined1 (*pauStack_c0) [16];
  code *pcStack_b8;
  ulonglong uStack_b0;
  undefined1 (*pauStack_a8) [16];
  longlong lStack_a0;
  ulonglong uStack_98;
  undefined1 (*pauStack_90) [16];
  ulonglong uStack_88;
  undefined1 (*pauStack_80) [16];
  undefined1 (*pauStack_78) [16];
  undefined1 (*pauStack_70) [16];
  undefined1 (*pauStack_68) [16];
  undefined1 (*pauStack_60) [16];
  undefined1 (*pauStack_58) [16];
  undefined4 uStack_4c;
  undefined8 uStack_48;
  
  puVar22 = auStack_128;
  uStack_48 = 0xfffffffffffffffe;
  uStack_4c = param_4;
  lStack_a0 = param_2;
  pauVar21 = (undefined1 (*) [16])param_3[4];
  pauVar27 = (undefined1 (*) [16])0x0;
  auVar3._8_8_ = 0;
  auVar3._0_8_ = pauVar21;
  uVar10 = SUB168(auVar3 * ZEXT816(0x18),0);
  if ((SUB168(auVar3 * ZEXT816(0x18),8) != 0) || (0x7ffffffffffffff8 < uVar10)) {
code_r0x0001418c2035:
    FUN_1428d8fe3(pauVar27,uVar10,&UNK_142d06298);
code_r0x0001418c2047:
    FUN_1428d9000(8,0x58);
    goto code_r0x0001418c20a7;
  }
  if (uVar10 == 0) {
    pauVar11 = (undefined1 (*) [16])0x8;
    pauVar12 = (undefined1 (*) [16])0x0;
  }
  else {
    pauVar27 = (undefined1 (*) [16])0x8;
    pauVar11 = (undefined1 (*) [16])func_0x000140613c10(uVar10,8);
    pauVar12 = pauVar21;
    if (pauVar11 == (undefined1 (*) [16])0x0) goto code_r0x0001418c2035;
  }
  pauStack_68 = pauVar12;
  pauStack_60 = pauVar11;
  pauStack_58 = (undefined1 (*) [16])0x0;
  lVar1 = (longlong)pauVar21 * 4;
  if (pauVar21 == (undefined1 (*) [16])0x0) {
    pauStack_70 = (undefined1 (*) [16])0x4;
    pauStack_90 = (undefined1 (*) [16])0x0;
code_r0x0001418c1b28:
    pauVar12 = pauStack_70;
    pauStack_c8 = param_1;
    if (pauVar21 != (undefined1 (*) [16])0x0) goto code_r0x0001418c1b31;
  }
  else {
    pauVar12 = (undefined1 (*) [16])func_0x000140613c10(lVar1,4);
    pauStack_70 = pauVar12;
    if (pauVar12 == (undefined1 (*) [16])0x0) {
      FUN_1428d8fe3(4,lVar1,&UNK_142d062b0);
      goto code_r0x0001418c20a7;
    }
    pauStack_90 = pauVar21;
    if (pauVar21 < (undefined1 (*) [16])0x2) goto code_r0x0001418c1b28;
    pauStack_c8 = param_1;
    func_0x000142923c20(pauVar12,0,lVar1 + -4);
    pauVar12 = (undefined1 (*) [16])(pauVar12[-1] + lVar1 + 0xc);
code_r0x0001418c1b31:
    *(undefined4 *)*pauVar12 = 0;
  }
  pauVar12 = (undefined1 (*) [16])*param_3;
  pauVar11 = (undefined1 (*) [16])param_3[1];
  pbVar26 = (byte *)(ulonglong)*(ushort *)(param_3 + 3);
  uStack_98 = 0;
  pauStack_78 = pauVar21;
  while( true ) {
    pauVar28 = pauStack_58;
    pauVar6 = pauStack_60;
    uVar25 = (uint)pbVar26;
    bVar33 = pauVar21 == (undefined1 (*) [16])0x0;
    pauVar21 = (undefined1 (*) [16])(pauVar21[-1] + 0xf);
    if (bVar33) break;
    if ((short)pbVar26 == 0) {
      do {
        auVar3 = *pauVar11;
        uVar5 = (ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar3[0xf] >> 7) << 0xf;
        pauVar12 = pauVar12 + -0x30;
        pauVar11 = pauVar11 + 1;
      } while (uVar5 == 0xffff);
      uVar25 = ~(uint)uVar5;
      uVar8 = uVar25 & -(uint)uVar5 - 2;
    }
    else {
      if (pauVar12 == (undefined1 (*) [16])0x0) break;
      uVar8 = uVar25 - 1 & uVar25;
    }
    uVar29 = 0;
    for (; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
      uVar29 = uVar29 + 1;
    }
    uVar10 = (ulonglong)uVar29;
    pbVar26 = (byte *)(ulonglong)uVar8;
    pauVar15 = pauVar12 + uVar10 * -3;
    uStack_88 = (ulonglong)*(uint *)pauVar12[uVar10 * -3 + -3];
    pauStack_80 = (undefined1 (*) [16])(pauVar12[uVar10 * -3 + -3] + 8);
    cVar7 = (char)pauVar15;
    pauVar24 = pauVar12;
    if ((pauVar12[uVar10 * -3 + -1][0] & 1) == 0) {
      uVar10 = (ulonglong)*(uint *)(pauVar15[-1] + 4);
      if (*(uint *)(pauVar15[-1] + 4) == 1) goto code_r0x0001418c1c9e;
code_r0x0001418c1c52:
      if ((int)uVar10 == 0) {
        pauStack_c0 = (undefined1 (*) [16])&pauStack_80;
        pcStack_b8 = FUN_14213e060;
        ppauStack_f8 = (undefined1 (**) [16])&UNK_142d062e0;
        ppauStack_f0 = (undefined1 (**) [16])0x1;
        uStack_d8 = 0;
        pauStack_e8 = (undefined1 (*) [16])&pauStack_c0;
        pauStack_e0 = (undefined1 (*) [16])0x1;
        FUN_1428d9390(&ppauStack_f8,&UNK_142d062f0);
      }
      else {
        if (pauStack_58 < pauStack_78) {
          *(undefined4 *)(*pauStack_70 + (longlong)pauStack_58 * 4) = 4;
          bVar2 = pauVar15[-1][8];
          uVar30 = (uint)bVar2;
          uVar29 = (uint)bVar2;
          uVar19 = (ulonglong)*(int *)(&UNK_142d06398 + (ulonglong)bVar2 * 4);
          puVar20 = &UNK_142d06398 + uVar19;
          puVar23 = auStack_128;
          uVar25 = (uint)bVar2;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
          switch(bVar2) {
          case 0:
            goto code_r0x0001418c1cb3;
          case 1:
            goto code_r0x0001418c1d5e;
          case 2:
            goto code_r0x0001418c1dc3;
          case 3:
            goto code_r0x0001418c1da0;
          case 4:
            goto code_r0x0001418c1cf6;
          case 5:
            goto code_r0x0001418c1e00;
          case 6:
            goto code_r0x0001418c2350;
          case 7:
            goto code_r0x0001418c23f3;
          case 8:
            goto code_r0x0001418c245b;
          case 9:
            goto code_r0x0001418c2438;
          case 10:
            goto code_r0x0001418c238e;
          case 0xb:
            goto code_r0x0001418c2495;
          case 0xc:
          case 0xd:
            goto code_r0x000142d06398;
          default:
            goto code_r0x000142d06397;
          }
        }
        FUN_1428d9518(pauStack_58,pauStack_78,&UNK_142d06308);
      }
      goto code_r0x0001418c20a7;
    }
    uVar19 = (ulonglong)*(uint *)(*(longlong *)(lStack_a0 + 0x658) + 0x680);
    uVar10 = 0;
    if (0x1f < uVar19) {
      uVar10 = uVar19 - 0x20;
    }
    if (0x1fff < uVar10) {
      uVar10 = 0x2000;
    }
    if ((int)uVar10 != 1) goto code_r0x0001418c1c52;
code_r0x0001418c1c9e:
    bVar2 = pauVar15[-1][8];
    uVar30 = (uint)bVar2;
    uVar29 = (uint)bVar2;
    uVar19 = (ulonglong)*(int *)(&UNK_142d06398 + (ulonglong)bVar2 * 4);
    puVar20 = &UNK_142d06398 + uVar19;
    puVar23 = auStack_128;
    uVar25 = (uint)bVar2;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
    switch(bVar2) {
    case 0:
code_r0x0001418c1cb3:
      uVar29 = 7;
      if (3 < *(ulonglong *)(pauVar15[-2] + 8)) {
        uVar25 = *(uint *)(*(longlong *)pauVar15[-2] + -4 + *(ulonglong *)(pauVar15[-2] + 8));
        uVar29 = (uint)((uVar25 >> 0x18 | (uVar25 & 0xff0000) >> 8 | (uVar25 & 0xff00) << 8 |
                        uVar25 << 0x18) == 0x5f64796e) * 2 + 7;
      }
      break;
    case 1:
code_r0x0001418c1d5e:
      uVar29 = 6;
      if (3 < *(ulonglong *)(pauVar15[-2] + 8)) {
        uVar25 = *(uint *)(*(longlong *)pauVar15[-2] + -4 + *(ulonglong *)(pauVar15[-2] + 8));
        uVar29 = (uint)((uVar25 >> 0x18 | (uVar25 & 0xff0000) >> 8 | (uVar25 & 0xff00) << 8 |
                        uVar25 << 0x18) == 0x5f64796e) * 2 + 6;
      }
      break;
    case 2:
code_r0x0001418c1dc3:
      uVar32 = (undefined4)uVar10;
      if (pauStack_58 == pauStack_68) {
        FUN_140562db0(&pauStack_68,&UNK_142d06320);
      }
      *(int *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18) = (int)uStack_88;
      *(undefined4 *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18 + 4) = 1;
      goto code_r0x0001418c1b7b;
    case 3:
      break;
    case 4:
code_r0x0001418c1cf6:
      uVar32 = (undefined4)uVar10;
      if (*(int *)pauVar15[-1] != 0) {
        if (pauStack_78 <= pauStack_58) {
          FUN_1428d9518(pauStack_58,pauStack_78,&UNK_142d06350);
          goto code_r0x0001418c20a7;
        }
        *(undefined4 *)(*pauStack_70 + (longlong)pauStack_58 * 4) = 0xf;
        uStack_98 = (ulonglong)((uint)uStack_98 | 2);
      }
      if (pauStack_58 == pauStack_68) {
        FUN_140562db0(&pauStack_68,&UNK_142d06368);
      }
      *(int *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18) = (int)uStack_88;
      *(undefined4 *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18 + 4) = 2;
      goto code_r0x0001418c1b7b;
    case 5:
code_r0x0001418c1e00:
      uVar13 = FUN_140bfc5b0(lStack_a0,*(undefined8 *)pauVar15[-2]);
      lVar1 = *param_5;
      uVar25 = *(uint *)(lVar1 + 0x50);
      if ((ulonglong)uVar25 < 8) {
        *(undefined8 *)(lVar1 + 0x10 + (ulonglong)uVar25 * 8) = uVar13;
        *(uint *)(lVar1 + 0x50) = uVar25 + 1;
        uVar10 = (ulonglong)*(uint *)(*param_5 + 0x50);
        if (uVar10 == 0) {
          FUN_1428d9518(0xffffffffffffffff,0,&UNK_142feb2c8);
          goto code_r0x0001418c20a7;
        }
        puVar31 = (undefined8 *)(*param_5 + (uVar10 - 1) * 8 + 0x10);
        if (pauStack_58 == pauStack_68) goto code_r0x0001418c1eab;
      }
      else {
        puVar14 = (undefined8 *)func_0x000140613c10(0x58,8);
        if (puVar14 == (undefined8 *)0x0) goto code_r0x0001418c2047;
        puVar31 = puVar14 + 2;
        puVar14[2] = uVar13;
        *(undefined4 *)(puVar14 + 10) = 1;
        lVar1 = *param_5;
        *param_5 = (longlong)puVar14;
        *puVar14 = 1;
        puVar14[1] = lVar1;
        if (pauStack_58 == pauStack_68) {
code_r0x0001418c1eab:
          pauVar27 = pauStack_58;
          FUN_140562db0(&pauStack_68,&UNK_142d06380);
          pauStack_58 = pauVar27;
        }
      }
      *(ulonglong *)((longlong)pauStack_60 + (longlong)pauStack_58 * 0x18) = uStack_88;
      *(undefined4 *)((longlong)pauStack_60 + (longlong)pauStack_58 * 0x18 + 8) = 1;
      *(undefined4 *)((longlong)pauStack_60 + (longlong)pauStack_58 * 0x18 + 0xc) = uStack_4c;
      *(undefined8 **)((longlong)pauStack_60 + (longlong)pauStack_58 * 0x18 + 0x10) = puVar31;
      pauVar28 = pauStack_58;
      goto code_r0x0001418c1b90;
    case 6:
      goto code_r0x0001418c2350;
    case 7:
code_r0x0001418c23f3:
      uVar30 = uVar25;
      (*pauVar15)[0] = (*pauVar15)[0] + cVar7;
      (*pauVar15)[0] = (*pauVar15)[0] + cVar7;
      puVar20 = puVar20 + (longlong)pauVar27;
      goto code_r0x0001418c23f4;
    case 8:
code_r0x0001418c245b:
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    case 9:
code_r0x0001418c2438:
      *pbVar26 = *pbVar26 >> 6 | *pbVar26 << 2;
      pauVar12[6][6] = pauVar12[6][6] + (char)((ulonglong)pauVar15 >> 8);
      puVar22 = auStack_128;
code_r0x0001418c2450:
      if (pauStack_60 == pauStack_70) {
        *(undefined **)(puVar22 + -8) = &UNK_1418c246e;
        FUN_140562db0(&pauStack_70,&UNK_142d06338);
      }
      puVar20 = (undefined *)((longlong)pauVar6 * 3);
      *(uint *)((longlong)pauStack_68 + (longlong)pauVar6 * 0x18) = uVar30;
      *(int *)((longlong)pauStack_68 + (longlong)pauVar6 * 0x18 + 4) = (int)pauVar21;
      pauVar15 = pauStack_68;
      pauVar27 = pauVar6;
      goto code_r0x0001418c22b7;
    case 10:
code_r0x0001418c238e:
      uVar30 = uVar25;
      if ((longlong)puVar20 < 0) {
        uVar8 = auStack_128._0_4_;
        pauVar21 = (undefined1 (*) [16])
                   (ulonglong)
                   ((uint)((int)puVar20 + -1 + (uint)(0xfffffffebd2f9c67 < uVar19) == 0) * 2 + 7);
        puVar22 = auStack_120;
        goto code_r0x0001418c2450;
      }
code_r0x0001418c23f4:
      (*pauVar15)[0] = (*pauVar15)[0] + cVar7;
      (puVar20 + -0x17)[(longlong)pauVar15] = (puVar20 + -0x17)[(longlong)pauVar15] + (char)puVar20;
      *(uint *)(*pauVar15 + (longlong)puVar20 * 8) = uVar30;
      *(undefined4 *)(*pauVar15 + (longlong)puVar20 * 8 + 4) = 2;
      goto code_r0x0001418c22b7;
    case 0xb:
code_r0x0001418c2495:
      (*pauVar15)[0] = (*pauVar15)[0] + cVar7;
      (*pauVar15)[0] = (*pauVar15)[0] + cVar7;
      puVar20 = puVar20 + (longlong)pauVar27;
      *(uint *)(*pauVar15 + (longlong)puVar20 * 8) = uVar30;
      *(undefined4 *)(*pauVar15 + (longlong)puVar20 * 8 + 4) = 1;
      puVar22 = auStack_128;
code_r0x0001418c22b7:
      uVar19 = uStack_88;
      *(uint *)(*pauVar15 + (longlong)puVar20 * 8 + 8) = uVar8;
      *(undefined4 *)(*pauVar15 + (longlong)puVar20 * 8 + 0xc) = pauStack_78._4_4_;
      *(undefined8 *)(pauVar15[1] + (longlong)puVar20 * 8) = 0;
      pauVar24 = (undefined1 (*) [16])(pauVar12[2] + 8);
      pauStack_60 = (undefined1 (*) [16])(*pauVar27 + 1);
      if (pauVar24 == pauVar11) {
        pauStack_c8 = (undefined1 (*) [16])CONCAT44(pauStack_c8._4_4_,0x3b9d3ee8);
        pauStack_c0 = (undefined1 (*) [16])0x0;
        pcStack_b8 = (code *)CONCAT44(pcStack_b8._4_4_,(int)pauStack_80);
        uStack_b0 = uStack_88;
        pauVar21 = (undefined1 (*) [16])&pauStack_c8;
        do {
          pauVar21 = *(undefined1 (**) [16])((longlong)*pauVar21 + 8);
        } while (pauVar21 != (undefined1 (*) [16])0x0);
        puStack_100 = (undefined *)CONCAT44(puStack_100._4_4_,0x20);
        ppauStack_f8 = &pauStack_c8;
        ppauStack_f0 = (undefined1 (**) [16])
                       CONCAT44((int)(undefined1 (*) [16])(*pauVar27 + 1),(int)lStack_a0);
        pauStack_e8 = pauStack_68;
        uVar13 = *(undefined8 *)pauStack_a8[100];
        pcVar4 = *(code **)pauStack_a8[0x23];
        *(undefined **)(puVar22 + -8) = &UNK_1418c2612;
        iVar9 = (*pcVar4)(uVar13,&puStack_100,0,&pauStack_90);
        if (iVar9 == 0) {
          ppauStack_d0[2] = pauStack_60;
          *ppauStack_d0 = pauStack_70;
          ppauStack_d0[1] = pauStack_68;
          ppauStack_d0[3] = pauStack_90;
          if (0x27 < uStack_98) {
            *(undefined **)(puVar22 + -8) = &UNK_1418c2652;
            func_0x000140613c20(uVar19,(longlong)pauStack_80 << 2,4);
          }
          return (undefined1 (*) [16])ppauStack_d0;
        }
        pauStack_90 = (undefined1 (*) [16])CONCAT44(pauStack_90._4_4_,iVar9);
        *(undefined **)(puVar22 + 0x20) = &UNK_142d062c8;
        *(undefined **)(puVar22 + -8) = &UNK_1418c2745;
        FUN_1428d9760(&UNK_142d06248,0x2b,&pauStack_90,&UNK_142d06228);
      }
      else {
        puVar23 = puVar22;
        pauStack_90 = pauVar24;
        if ((pauVar12[4][0] & 1) == 0) {
code_r0x0001418c2350:
          iVar9 = *(int *)(pauVar24[1] + 0xc);
        }
        else {
          uVar16 = (ulonglong)*(uint *)(*(longlong *)(pauStack_a8[0x65] + 8) + 0x680);
          uVar19 = 0;
          if (0x1f < uVar16) {
            uVar19 = uVar16 - 0x20;
          }
          if (0x1fff < uVar19) {
            uVar19 = 0x2000;
          }
          iVar9 = (int)uVar19;
        }
        if (iVar9 == 1) {
                    /* WARNING: Could not recover jumptable at 0x0001418c2363. Too many branches */
                    /* WARNING: Treating indirect jump as call */
          ppauVar17 = (undefined1 (**) [16])
                      (*(code *)((longlong)*(int *)(uVar10 + (ulonglong)(byte)pauVar24[2][0] * 4) +
                                uVar10))();
          return (undefined1 (*) [16])ppauVar17;
        }
        if (iVar9 == 0) {
          pauStack_c8 = (undefined1 (*) [16])&pauStack_90;
          pauStack_c0 = (undefined1 (*) [16])FUN_14213e060;
          puStack_100 = &UNK_142d062e0;
          ppauStack_f8 = (undefined1 (**) [16])0x1;
          pauStack_e0 = (undefined1 (*) [16])0x0;
          ppauStack_f0 = &pauStack_c8;
          pauStack_e8 = (undefined1 (*) [16])0x1;
          *(undefined **)(puVar23 + -8) = &UNK_1418c26b7;
          FUN_1428d9390(&puStack_100,&UNK_142d062f0);
        }
        else {
          if (pauStack_60 < pauStack_80) {
            *(undefined4 *)(uStack_88 + (longlong)pauStack_60 * 4) = 4;
                    /* WARNING: Could not recover jumptable at 0x0001418c234b. Too many branches */
                    /* WARNING: Treating indirect jump as call */
            ppauVar17 = (undefined1 (**) [16])
                        (*(code *)((longlong)*(int *)(uVar10 + (ulonglong)(byte)pauVar24[2][0] * 4)
                                  + uVar10))();
            return (undefined1 (*) [16])ppauVar17;
          }
          *(undefined **)(puVar23 + -8) = &UNK_1418c26cc;
          FUN_1428d9518(pauStack_60,pauStack_80,&UNK_142d06308);
        }
      }
                    /* WARNING: Does not return */
      pcVar4 = (code *)invalidInstructionException();
      (*pcVar4)();
    case 0xc:
    case 0xd:
code_r0x000142d06398:
code_r0x000142d0639e:
      (*pauVar15)[0] = (*pauVar15)[0] + -1;
      cVar7 = cRamfdfebbbff6febbc0;
      cRam00000000777b2cfc = cRam00000000777b2cfc >> 0x1e;
      pcVar18 = (char *)CONCAT71((int7)((ulonglong)pauVar15 >> 8),cRamfdfebbbff6febbc0);
      *pcVar18 = *pcVar18 + cRamfdfebbbff6febbc0;
      *pcVar18 = *pcVar18 + cVar7;
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    default:
code_r0x000142d06397:
      (*pauVar21)[0] = (*pauVar21)[0] + (char)pauVar21;
      goto code_r0x000142d0639e;
    }
code_r0x0001418c1da0:
    uVar32 = (undefined4)uVar10;
    if (pauStack_58 == pauStack_68) {
      FUN_140562db0(&pauStack_68,&UNK_142d06338);
    }
    *(int *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18) = (int)uStack_88;
    *(uint *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18 + 4) = uVar29;
code_r0x0001418c1b7b:
    *(undefined4 *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18 + 8) = uVar32;
    *(undefined4 *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18 + 0xc) = uStack_4c;
    *(undefined8 *)((longlong)pauStack_60 + (longlong)pauVar28 * 0x18 + 0x10) = 0;
code_r0x0001418c1b90:
    pauVar27 = (undefined1 (*) [16])(*pauVar28 + 1);
    pauStack_58 = pauVar27;
  }
  pauStack_c0 = (undefined1 (*) [16])CONCAT44(pauStack_c0._4_4_,0x3b9d3ee8);
  pcStack_b8 = (code *)0x0;
  uStack_b0 = CONCAT44(uStack_b0._4_4_,(int)pauStack_78);
  pauStack_a8 = pauStack_70;
  pauVar21 = (undefined1 (*) [16])&pauStack_c0;
  do {
    pauVar21 = *(undefined1 (**) [16])((longlong)*pauVar21 + 8);
  } while (pauVar21 != (undefined1 (*) [16])0x0);
  ppauStack_f8 = (undefined1 (**) [16])CONCAT44(ppauStack_f8._4_4_,0x20);
  ppauStack_f0 = &pauStack_c0;
  pauStack_e8 = (undefined1 (*) [16])CONCAT44((int)pauStack_58,(uint)uStack_98);
  pauStack_e0 = pauStack_60;
  iVar9 = (**(code **)(lStack_a0 + 0x230))
                    (*(undefined8 *)(lStack_a0 + 0x640),&ppauStack_f8,0,&pauStack_80);
  pauVar21 = pauStack_c8;
  if (iVar9 == 0) {
    *(undefined1 (**) [16])pauStack_c8[1] = pauStack_58;
    *(undefined1 (**) [16])*pauStack_c8 = pauStack_68;
    *(undefined1 (**) [16])((longlong)*pauStack_c8 + 8) = pauStack_60;
    *(undefined1 (**) [16])((longlong)pauStack_c8[1] + 8) = pauStack_80;
    if (pauStack_90 != (undefined1 (*) [16])0x0) {
      func_0x000140613c20(pauStack_70,(longlong)pauStack_90 << 2,4);
    }
    return pauVar21;
  }
  pauStack_80 = (undefined1 (*) [16])CONCAT44(pauStack_80._4_4_,iVar9);
  puStack_108 = &UNK_142d062c8;
  FUN_1428d9760(&UNK_142d06248,0x2b,&pauStack_80,&UNK_142d06228);
code_r0x0001418c20a7:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

