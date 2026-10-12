
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 *
FUN_1417c93d0(undefined4 *param_1,undefined1 (*param_2) [16],ulonglong param_3,undefined8 *param_4,
             ulonglong *param_5)

{
  undefined1 auVar1 [16];
  code *pcVar2;
  unkuint9 Var3;
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined8 *puVar6;
  ulonglong uVar7;
  longlong lVar8;
  undefined4 *puVar9;
  uint uVar10;
  undefined4 uVar11;
  uint uVar12;
  undefined1 (*pauVar13) [16];
  undefined1 *puVar14;
  int iVar15;
  undefined1 (*pauVar16) [16];
  uint uVar17;
  ulonglong uVar18;
  undefined8 uVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar24;
  undefined1 auVar23 [16];
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined8 in_stack_fffffffffffffdd8;
  undefined4 uVar28;
  undefined8 *puVar27;
  undefined8 in_stack_fffffffffffffde8;
  undefined4 uVar29;
  undefined1 auStack_1f8 [16];
  longlong lStack_1e8;
  undefined8 uStack_1e0;
  longlong lStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined8 uStack_1a8;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined4 *puStack_138;
  undefined1 (*pauStack_130) [16];
  undefined8 *puStack_128;
  ulonglong uStack_120;
  ulonglong uStack_118;
  undefined8 uStack_110;
  ulonglong uStack_108;
  ulonglong uStack_100;
  longlong lStack_f8;
  ulonglong uStack_f0;
  undefined1 uStack_e1;
  undefined8 uStack_e0;
  
  uStack_e0 = 0xfffffffffffffffe;
  uVar18 = param_3 << 5;
  uVar19 = 0;
  if ((param_3 >> 0x3b != 0) || (0x7ffffffffffffff0 < uVar18)) {
code_r0x0001417c99a3:
    FUN_1428d8fe3(uVar19,uVar18,&UNK_142ce2c98);
    pcVar2 = (code *)swi(3);
    puVar9 = (undefined4 *)(*pcVar2)();
    return puVar9;
  }
  if (uVar18 == 0) {
    lStack_f8 = 0x10;
    uStack_100 = 0;
    pauVar13 = param_2;
  }
  else {
    uVar19 = 0x10;
    pauVar13 = (undefined1 (*) [16])0x10;
    lStack_f8 = func_0x000140613c10(uVar18);
    uStack_100 = param_3;
    if (lStack_f8 == 0) goto code_r0x0001417c99a3;
  }
  uStack_f0 = 0;
  puStack_138 = param_1;
  puStack_128 = param_4;
  if (param_3 == 0) {
    lVar8 = 4;
  }
  else {
    pauVar13 = (undefined1 (*) [16])0x4;
    lVar8 = func_0x000140613c10(param_3 * 4);
    if (lVar8 == 0) {
      FUN_1428d8fe3(4,param_3 * 4,&UNK_142ce2cb0);
      goto code_r0x0001417c99a1;
    }
  }
  auVar5 = _UNK_142afcb20;
  auVar4 = _UNK_142afcb10;
  uStack_108 = 0;
  pauVar16 = param_2 + param_3 * 3;
  fVar20 = 0.0;
  iVar15 = 0;
  fVar21 = 0.0;
  pauStack_130 = param_2;
  uStack_120 = param_3;
  uStack_118 = param_3;
  uStack_110 = lVar8;
  while( true ) {
    uVar18 = uStack_f0;
    uVar29 = (undefined4)((ulonglong)in_stack_fffffffffffffde8 >> 0x20);
    uVar28 = (undefined4)((ulonglong)in_stack_fffffffffffffdd8 >> 0x20);
    uVar11 = SUB84(pauVar13,0);
    if (param_2 == pauVar16) break;
    auVar23 = *param_2;
    auVar26 = param_2[1];
    auVar1 = param_2[2];
    if (uStack_f0 == uStack_100) {
      uStack_e1 = 1;
      pauVar13 = (undefined1 (*) [16])&UNK_142ce2d00;
      FUN_1420e6d90(&uStack_100);
    }
    uVar7 = uStack_108;
    auVar25 = minps(auVar4,auVar23);
    auVar23 = maxps(auVar5,auVar23);
    auVar25 = minps(auVar25,auVar26);
    auVar23 = maxps(auVar23,auVar26);
    auVar26 = minps(auVar25,auVar1);
    auVar23 = maxps(auVar23,auVar1);
    lVar8 = uVar18 * 0x20;
    *(undefined1 (*) [16])(lStack_f8 + lVar8) = auVar26;
    *(undefined1 (*) [16])(lStack_f8 + 0x10 + lVar8) = auVar23;
    uStack_f0 = uVar18 + 1;
    if (uStack_108 == uStack_118) {
      uStack_e1 = 1;
      pauVar13 = (undefined1 (*) [16])&UNK_142ce2d18;
      FUN_140650050(&uStack_118);
    }
    param_2 = param_2 + 3;
    *(int *)(uStack_110 + uVar7 * 4) = iVar15;
    iVar15 = iVar15 + 1;
    fVar22 = auVar23._0_4_ - auVar26._0_4_;
    fVar24 = auVar23._4_4_ - auVar26._4_4_;
    fVar24 = fVar22 * fVar24 + (auVar23._8_4_ - auVar26._8_4_) * (fVar22 + fVar24);
    fVar22 = fVar21;
    if (fVar21 <= fVar24) {
      fVar22 = fVar24;
    }
    fVar20 = fVar20 + fVar24;
    fVar21 = (float)(-(uint)NAN(fVar24) & (uint)fVar21 | ~-(uint)NAN(fVar24) & (uint)fVar22);
    uStack_108 = uVar7 + 1;
  }
  Var3 = (unkuint9)uStack_120;
  uStack_e1 = 1;
  uStack_148 = FUN_140d635e0();
  uStack_140 = uVar11;
  if (*(char *)((longlong)puStack_128 + 0x19) != '\0') {
    fVar20 = fVar20 / (float)(unkint9)Var3;
    fVar22 = _UNK_142934694 * fVar20;
    fVar24 = fVar21 * _UNK_142925984 + fVar20 * _UNK_142a7c88c;
    fVar21 = fVar24;
    if (fVar24 <= fVar22) {
      fVar21 = fVar22;
    }
    uStack_e1 = 1;
    FUN_1420e8890(&uStack_100,&uStack_118,pauStack_130,uStack_120,
                  CONCAT44(uVar28,_UNK_1429cd078 * fVar20),
                  ~-(uint)NAN(fVar22) & (uint)fVar21 | -(uint)NAN(fVar22) & (uint)fVar24,
                  CONCAT44(uVar29,0x3fe66666),0x3fcccccd,0xc,0xc);
  }
  puVar6 = puStack_128;
  uStack_1a8 = uStack_108;
  uStack_1b8 = uStack_118;
  uStack_1b0 = (undefined4)uStack_110;
  uStack_1ac = uStack_110._4_4_;
  uStack_e1 = 0;
  puVar27 = &uStack_1b8;
  FUN_1420e9710(auStack_1f8,puStack_128 + 3,lStack_f8,uStack_f0,puVar27,
                *(undefined1 *)((longlong)puStack_128 + 0x1a),*puStack_128);
  uStack_1b8 = 0x8000000000000000;
  FUN_1420e5450(auStack_1f8,*(undefined4 *)(puVar6 + 1),&uStack_1b8);
  uVar12 = *(uint *)(puVar6 + 2);
  uVar10 = 3;
  if (uVar12 < 3) {
    uVar10 = uVar12;
  }
  uVar17 = 1;
  if (uVar12 != 0) {
    uVar17 = uVar10;
  }
  puVar14 = auStack_1f8;
  FUN_1420f2480(&uStack_1b8,puVar14,uVar17,1,(ulonglong)puVar27 & 0xffffffffffffff00);
  iVar15 = (int)puVar14;
  uVar18 = FUN_140d63910(&uStack_148);
  puVar9 = puStack_138;
  if (!CARRY8(*param_5,uVar18)) {
    uVar18 = *param_5 + uVar18;
    uVar12 = iVar15 + (int)param_5[1];
    if (uVar12 < 1000000000) {
code_r0x0001417c9890:
      *param_5 = uVar18;
      *(uint *)(param_5 + 1) = uVar12;
      *(undefined8 *)(puStack_138 + 0x18) = uStack_158;
      *(undefined8 *)(puStack_138 + 0x1a) = uStack_150;
      *(undefined8 *)(puStack_138 + 0x14) = uStack_168;
      *(undefined8 *)(puStack_138 + 0x16) = uStack_160;
      *(undefined8 *)(puStack_138 + 0x10) = uStack_178;
      *(undefined8 *)(puStack_138 + 0x12) = uStack_170;
      puStack_138[0xc] = uStack_188;
      puStack_138[0xd] = uStack_184;
      puStack_138[0xe] = uStack_180;
      puStack_138[0xf] = uStack_17c;
      *(undefined8 *)(puStack_138 + 8) = uStack_198;
      *(undefined8 *)(puStack_138 + 10) = uStack_190;
      puStack_138[4] = (undefined4)uStack_1a8;
      puStack_138[5] = uStack_1a8._4_4_;
      puStack_138[6] = uStack_1a0;
      puStack_138[7] = uStack_19c;
      *puStack_138 = (undefined4)uStack_1b8;
      puStack_138[1] = uStack_1b8._4_4_;
      puStack_138[2] = uStack_1b0;
      puStack_138[3] = uStack_1ac;
      if (lStack_1e8 != 0) {
        func_0x000140613c20(uStack_1e0,lStack_1e8 * 0x30,0x10);
      }
      if (lStack_1d0 != 0) {
        func_0x000140613c20(uStack_1c8,lStack_1d0 << 2,4);
      }
      if (uStack_100 != 0) {
        func_0x000140613c20(lStack_f8,uStack_100 << 5,0x10);
      }
      return puVar9;
    }
    if ((uVar18 != 0xffffffffffffffff) && (uVar12 = uVar12 + 0xc4653600, uVar12 != 1000000000)) {
      uVar18 = uVar18 + 1;
      goto code_r0x0001417c9890;
    }
  }
  FUN_1428d9330(&UNK_142ce2cc8,0x1e,&UNK_142ce2ce8);
code_r0x0001417c99a1:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

