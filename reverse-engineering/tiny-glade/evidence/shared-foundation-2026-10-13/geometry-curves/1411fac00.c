
/* WARNING: Removing unreachable block (ram,0x0001411fafad) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1411fac00(undefined8 param_1,longlong param_2,ulonglong param_3,undefined4 param_4)

{
  float fVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  code *pcVar4;
  undefined8 *puVar5;
  longlong lVar6;
  longlong lVar7;
  longlong unaff_RBX;
  undefined4 unaff_ESI;
  undefined8 unaff_RDI;
  ulonglong uVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  ulonglong unaff_R13;
  ulonglong uVar11;
  float fVar12;
  float fVar13;
  uint uVar14;
  float fVar15;
  uint uVar16;
  float fVar17;
  float fVar18;
  float unaff_XMM6_Da;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  ulonglong uStack_148;
  undefined1 auStack_138 [16];
  undefined8 *puStack_120;
  longlong lStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  ulonglong uStack_100;
  undefined1 uStack_f1;
  undefined8 uStack_f0;
  
  uStack_f0 = 0xfffffffffffffffe;
  if (1 < param_3) {
    unaff_XMM6_Da = (float)param_3;
    uVar10 = param_3 * 8;
    uVar9 = 0;
    if ((param_3 >> 0x3d == 0) && (uVar10 < 0x7ffffffffffffffd)) {
      uVar9 = 4;
      puVar5 = (undefined8 *)func_0x000140613c10(uVar10,4);
      if (puVar5 != (undefined8 *)0x0) {
        uStack_100 = 0;
        unaff_R13 = *(ulonglong *)(param_2 + 0x10);
        unaff_RBX = param_2;
        unaff_RDI = param_1;
        uStack_110 = param_3;
        uStack_108 = puVar5;
        unaff_ESI = param_4;
        if (unaff_R13 == 0) {
          uStack_f1 = 1;
          FUN_1428d9518(0,0,&UNK_142c190c0);
          goto LAB_1411fb1b6;
        }
        goto LAB_1411fad34;
      }
    }
    FUN_1428d8fe3(uVar9,uVar10,&UNK_142c190a8);
LAB_1411fb152:
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c191c8);
LAB_1411fb1b6:
                    /* WARNING: Does not return */
    pcVar4 = (code *)invalidInstructionException();
    (*pcVar4)();
  }
  puVar5 = (undefined8 *)FUN_1428d9430(&UNK_142c19070,0x1f);
LAB_1411fad34:
  fVar21 = _DAT_142925904 / (unaff_XMM6_Da + _DAT_14295a0e4);
  puVar3 = *(undefined8 **)(unaff_RBX + 8);
  auStack_138 = ZEXT416((uint)(_DAT_14295a0e8 * fVar21 + _DAT_142925904));
  uVar9 = *puVar3;
  fVar22 = (float)uVar9;
  fVar23 = (float)((ulonglong)uVar9 >> 0x20);
  *puVar5 = uVar9;
  uStack_100 = 1;
  puStack_120 = puVar3 + unaff_R13;
  if (*(ulonglong *)(unaff_RBX + 0x28) < unaff_R13) {
    unaff_R13 = *(ulonglong *)(unaff_RBX + 0x28);
  }
  lVar6 = *(longlong *)(unaff_RBX + 0x20);
  lStack_118 = -unaff_R13;
  fVar25 = 0.0;
  fVar20 = 0.0;
  fVar13 = 0.0;
  fVar19 = 0.0;
  uVar10 = 1;
  uVar8 = (ulonglong)(unaff_R13 != 0);
  if (uVar8 < unaff_R13) {
    lVar7 = 0;
    do {
      if (uVar8 - 1 == lVar7) {
        uVar8 = uVar8 - lVar7;
        uVar11 = uVar8 + 1;
        fVar15 = fVar22;
        fVar24 = fVar23;
        goto LAB_1411faddf;
      }
      lVar7 = lVar7 + -1;
    } while (lStack_118 + uVar8 != lVar7);
  }
LAB_1411fafe1:
  if (puVar5 + (uVar10 - 1) == (undefined8 *)0x0) {
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c190d8);
    goto LAB_1411fb1b6;
  }
  if (puStack_120 + -1 == (undefined8 *)0x0) {
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c190f0);
    goto LAB_1411fb1b6;
  }
  uVar9 = puStack_120[-1];
  fVar19 = (float)((ulonglong)uVar9 >> 0x20);
  uVar2 = puVar5[uVar10 - 1];
  fVar21 = (float)uVar9 - (float)uVar2;
  fVar13 = fVar19 - (float)((ulonglong)uVar2 >> 0x20);
  fVar21 = SQRT(fVar13 * fVar13 + fVar21 * fVar21);
  if (fVar21 <= 0.0) {
    if (fVar20 <= fVar21) goto LAB_1411fb0d0;
    if (uVar10 == uStack_110) {
      uStack_f1 = 1;
      FUN_140b81f90(&uStack_110,&UNK_142c19108);
      puVar5 = uStack_108;
    }
    puVar5[uVar10] = CONCAT44(fVar23,fVar22);
    uVar8 = uVar10 + 1;
    fVar22 = fVar22 - (float)uVar9;
    fVar23 = fVar23 - fVar19;
    uStack_100 = uVar8;
    if (SQRT(fVar23 * fVar23 + fVar22 * fVar22) <= 0.0) {
      uStack_f1 = 1;
      FUN_1428d9430(&UNK_142c19120,0x47,&UNK_142c19168);
      goto LAB_1411fb1b6;
    }
    if (uVar8 == uStack_110) {
      uStack_f1 = 1;
      FUN_140b81f90(&uStack_110,&UNK_142c19180);
    }
    puVar5 = uStack_108 + uVar8;
    lVar6 = 2;
  }
  else {
    if (uVar10 == uStack_110) {
      uStack_f1 = 1;
      FUN_140b81f90(&uStack_110,&UNK_142c19198);
      puVar5 = uStack_108;
    }
    puVar5 = puVar5 + uVar10;
    lVar6 = 1;
  }
  *puVar5 = uVar9;
  uStack_100 = lVar6 + uVar10;
LAB_1411fb0d0:
  uStack_148 = uStack_100;
  uStack_158 = (undefined4)uStack_110;
  uStack_154 = uStack_110._4_4_;
  uStack_150 = (undefined4)uStack_108;
  uStack_14c = uStack_108._4_4_;
  uStack_f1 = 0;
  FUN_1411f9920(unaff_RDI,&uStack_158,unaff_ESI);
  return;
LAB_1411faddf:
  fVar26 = (float)puVar3[uVar8];
  fVar27 = (float)((ulonglong)puVar3[uVar8] >> 0x20);
  fVar1 = *(float *)(lVar6 + uVar8 * 4);
  fVar25 = fVar25 + (fVar1 - fVar13);
  fVar12 = (float)auStack_138._0_4_;
  if (fVar25 <= (float)auStack_138._0_4_) {
    fVar12 = fVar25;
  }
  fVar25 = (float)(-(uint)NAN(fVar25) & auStack_138._0_4_ | ~-(uint)NAN(fVar25) & (uint)fVar12);
LAB_1411fae20:
  if (fVar21 <= fVar25 - fVar19) {
    fVar19 = fVar19 + fVar21;
    uStack_f1 = 1;
    fVar12 = (float)func_0x000140c904a0(fVar13);
    uVar10 = uStack_100;
    if ((uStack_100 != 0) && (uStack_108 + (uStack_100 - 1) != (undefined8 *)0x0)) {
      uVar9 = uStack_108[uStack_100 - 1];
      fVar17 = fVar26 - (float)uVar9;
      fVar18 = fVar27 - (float)((ulonglong)uVar9 >> 0x20);
      if (SQRT(fVar18 * fVar18 + fVar17 * fVar17) <= fVar20) goto code_r0x0001411fae8c;
      fVar22 = 0.0;
      if (0.0 <= fVar12) {
        fVar22 = fVar12;
      }
      fVar23 = _DAT_142925904;
      if (fVar22 <= _DAT_142925904) {
        fVar23 = fVar22;
      }
      fVar22 = (_DAT_142925904 - fVar23) * fVar15 + fVar23 * fVar26;
      fVar23 = (_DAT_142925904 - fVar23) * fVar24 + fVar23 * fVar27;
      if (uStack_100 == uStack_110) {
        uStack_f1 = 1;
        FUN_140b81f90(&uStack_110);
      }
      goto LAB_1411faefb;
    }
    goto LAB_1411fb152;
  }
  if ((uVar10 == 0) || (uStack_108 + (uVar10 - 1) == (undefined8 *)0x0)) {
    uStack_f1 = 1;
    FUN_1428d9310(&UNK_142c191b0);
    goto LAB_1411fb1b6;
  }
  uVar9 = uStack_108[uVar10 - 1];
  fVar13 = fVar26 - (float)uVar9;
  fVar15 = fVar27 - (float)((ulonglong)uVar9 >> 0x20);
  fVar15 = SQRT(fVar15 * fVar15 + fVar13 * fVar13);
  fVar13 = fVar15;
  if (fVar15 <= fVar20) {
    fVar13 = fVar20;
  }
  uVar14 = (int)((uint)(fVar20 < fVar15) << 0x1f) >> 0x1f;
  uVar16 = (int)((uint)(fVar20 < fVar15) << 0x1f) >> 0x1f;
  fVar22 = (float)(~uVar14 & (uint)fVar22 | (uint)fVar26 & uVar14);
  fVar23 = (float)(~uVar16 & (uint)fVar23 | (uint)fVar27 & uVar16);
  puVar5 = uStack_108;
  fVar20 = fVar13;
  if (unaff_R13 <= uVar11) goto LAB_1411fafe1;
  uVar8 = uVar11;
  uVar11 = (uVar11 < unaff_R13) + uVar11;
  fVar13 = fVar1;
  fVar15 = fVar26;
  fVar24 = fVar27;
  goto LAB_1411faddf;
code_r0x0001411fae8c:
  if (0.0 < fVar20) {
    if (uStack_100 == uStack_110) {
      uStack_f1 = 1;
      FUN_140b81f90(&uStack_110);
    }
LAB_1411faefb:
    uStack_108[uVar10] = CONCAT44(fVar23,fVar22);
    uVar10 = uVar10 + 1;
    fVar20 = 0.0;
    uStack_100 = uVar10;
  }
  goto LAB_1411fae20;
}

