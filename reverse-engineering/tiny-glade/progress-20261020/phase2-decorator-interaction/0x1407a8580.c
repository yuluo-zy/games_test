
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1407a8580(undefined8 *param_1,undefined8 *param_2,float *param_3,undefined8 *param_4,
                  longlong *param_5,undefined8 param_6,undefined8 *param_7,longlong *param_8,
                  longlong *param_9,longlong *param_10,undefined8 *param_11,undefined8 *param_12,
                  longlong *param_13,longlong *param_14)

{
  ulonglong *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  code *pcVar7;
  undefined8 uVar8;
  float fVar9;
  ulonglong *puVar10;
  char cVar11;
  uint uVar12;
  ulonglong uVar13;
  undefined8 *puVar14;
  byte bVar15;
  undefined1 uVar16;
  undefined *puVar17;
  undefined1 uVar18;
  longlong lVar19;
  int iVar20;
  bool bVar21;
  float fVar22;
  undefined4 uVar23;
  float fVar24;
  float fVar25;
  undefined4 uVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  ulonglong uStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  ulonglong uStack_280;
  ulonglong *puStack_278;
  undefined8 uStack_270;
  ulonglong **ppuStack_268;
  code *pcStack_260;
  undefined8 *puStack_258;
  code *pcStack_250;
  float fStack_240;
  uint uStack_23c;
  undefined *puStack_238;
  ulonglong *puStack_230;
  undefined8 *puStack_228;
  undefined8 *puStack_220;
  undefined8 uStack_218;
  ulonglong uStack_210;
  undefined4 uStack_204;
  undefined4 uStack_200;
  uint uStack_1fc;
  undefined8 uStack_1f8;
  undefined *puStack_1f0;
  ulonglong *puStack_1e8;
  undefined *puStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  float fStack_1c8;
  undefined1 uStack_1b6;
  undefined1 uStack_1b0;
  undefined4 uStack_1a8;
  undefined4 uStack_1a0;
  undefined1 uStack_198;
  undefined4 uStack_196;
  undefined4 uStack_190;
  undefined1 uStack_18c;
  float fStack_17c;
  undefined4 uStack_178;
  float fStack_174;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  ulonglong *puStack_158;
  undefined8 uStack_150;
  ulonglong **ppuStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined4 *puStack_120;
  undefined4 *puStack_118;
  undefined8 uStack_110;
  ulonglong uStack_108;
  undefined8 uStack_100;
  uint uStack_f8;
  ulonglong uStack_f0;
  undefined4 uStack_e8;
  undefined1 uStack_e2;
  byte bStack_e1;
  undefined8 uStack_e0;
  
  uStack_e0 = 0xfffffffffffffffe;
  puVar1 = (ulonglong *)*param_8;
  uVar13 = *puVar1 - 2;
  if (uVar13 != 1 || (ulonglong)(uVar13 < 5) <= (puVar1[1] - 1) + (ulonglong)(1 < *puVar1)) {
    return;
  }
  uVar2 = *param_11;
  puStack_120 = (undefined4 *)param_11[2];
  uStack_110 = CONCAT44(uStack_110._4_4_,*(undefined4 *)((longlong)param_11 + 0x1c));
  uVar3 = param_11[4];
  puStack_118 = (undefined4 *)param_11[6];
  uStack_e8 = *(undefined4 *)((longlong)param_11 + 0x3c);
  uVar4 = param_11[8];
  uStack_108 = (ulonglong)(uint)puVar1[0xb];
  uVar13 = puVar1[2];
  uStack_218 = func_0x00014067dd80(*param_1);
  uStack_f0 = (ulonglong)(byte)puVar1[0xc];
  uStack_210 = uVar13;
  cVar11 = func_0x00014092e680(&uStack_218);
  if (cVar11 == '\0') {
    return;
  }
  bStack_e1 = func_0x00014092df10(&uStack_218);
  uVar13 = uStack_108;
  if (bStack_e1 < 2) {
    *(undefined4 *)param_13[2] = *(undefined4 *)((longlong)param_13 + 0x1c);
    *(undefined1 *)(*param_13 + 0x48) = 1;
  }
  puVar17 = (undefined *)(uStack_108 & 0xffffffff);
  uStack_168 = uVar3;
  uStack_160 = uVar4;
  puStack_230 = (ulonglong *)FUN_140a1c3e0(uVar2);
  puStack_238 = puVar17;
  uStack_170 = uVar2;
  puVar14 = (undefined8 *)FUN_140a1c5f0(uVar2,puStack_230,puVar17,uVar13 & 0xffffffff);
  if (puVar14 == (undefined8 *)0x0) {
    FUN_1428d9310(&UNK_142a82aa0);
  }
  else {
    uStack_204 = *(undefined4 *)puVar14;
    uStack_200 = *(undefined4 *)((longlong)puVar14 + 4);
    fVar25 = *(float *)(puVar14 + 1);
    uVar26 = *(undefined4 *)((longlong)puVar14 + 0xc);
    bVar15 = *(char *)((longlong)puVar14 + 0x22) - 2;
    iVar20 = 0;
    if (bVar15 < 3) {
      iVar20 = bVar15 + 1;
    }
    uStack_e2 = *(undefined1 *)((longlong)puVar14 + 0x44);
                    /* WARNING: Switch is manually overridden */
    switch(iVar20) {
    case 0:
      goto override_jmp_1407a8764_case_0;
    case 1:
      FUN_140a1aa20(&uStack_1f8,*param_7,uStack_108,&UNK_142a82af8);
      uStack_150 = (undefined8 *)CONCAT44(uStack_150._4_4_,puStack_1e0._0_4_);
      puStack_158 = puStack_1e8;
      fVar29 = fVar25;
      break;
    case 2:
      uStack_270 = (undefined *)CONCAT44(uStack_270._4_4_,fVar25);
      lVar19 = *param_5;
      uStack_1f8 = *(ulonglong **)(lVar19 + 8);
      puStack_1f0 = (undefined *)(*(longlong *)(lVar19 + 0x10) * 2);
      puStack_1e8 = *(ulonglong **)(lVar19 + 0x50);
      puStack_278 = (ulonglong *)*puVar14;
      FUN_140257ba0(&uStack_1f8,puStack_1f0,uStack_200);
      func_0x000140c9cde0(&puStack_158,&puStack_278);
      fVar29 = (float)uStack_270;
      break;
    case 3:
      fVar29 = *(float *)(puVar14 + 2);
      FUN_140a1aa20(&uStack_1f8,*param_7,uStack_108,&UNK_142a82b10);
      uStack_150 = (undefined8 *)CONCAT44(uStack_150._4_4_,puStack_1e0._0_4_);
      puStack_158 = puStack_1e8;
    }
    uStack_f8 = (uint)uStack_150;
    uStack_100 = puStack_158;
    puVar10 = uStack_100;
    uStack_100._0_4_ = SUB84(puStack_158,0);
    fVar24 = (float)uStack_100;
    fStack_240 = (float)uStack_100;
    uVar12 = (int)uStack_f0 - 0x38;
    uStack_1fc = 6;
    if ((byte)uVar12 < 8) {
      uStack_1fc = uVar12 & 0xff;
    }
    uStack_23c = (uint)uStack_150;
    uStack_100 = puVar10;
    if (((byte)uStack_f0 & 0x3e) == 0x3c) {
      func_0x000140b02490(&uStack_1f8,&puStack_158);
      func_0x000140b02500(&puStack_158,&uStack_1f8);
      FUN_140b02520(&puStack_158,*param_2);
    }
    uStack_100 = (ulonglong *)
                 CONCAT44(*(float *)((longlong)puVar1 + 0x5c) + uStack_100._4_4_,(float)uStack_100);
    cVar11 = func_0x00014092df10(&uStack_218);
    bVar15 = 3;
    if ((byte)(cVar11 - 2U) < 3) {
      bVar15 = cVar11 - 2U;
    }
    if (1 < bVar15 - 2) {
      if (bVar15 == 1) {
        FUN_140a1a9f0(&uStack_1f8,*param_7,uStack_108,&UNK_142a82b28);
        fVar25 = uStack_1f8._4_4_;
        if (uStack_1f8._4_4_ <= 0.0) {
          fVar25 = 0.0;
        }
        fVar24 = (float)func_0x000140833a00(*(float *)(puVar1 + 6) - fVar24);
        func_0x0001408ef180(&uStack_1f8,*param_4);
        param_3[2] = puStack_1f0._0_4_;
        *(ulonglong **)param_3 = uStack_1f8;
        param_3[3] = fVar25;
        param_3[4] = fVar24;
        param_3[5] = fVar29;
      }
      param_3[6] = fVar29;
code_r0x0001407a8ec8:
      fVar29 = (fVar29 + _UNK_1429edba4) * _UNK_142925870;
      fVar25 = (float)func_0x000142923fd0(fVar29);
      uVar26 = func_0x000142923e80(fVar29);
      uStack_290 = CONCAT44(uVar26,fVar25 * 0.0);
      uStack_298 = CONCAT44(fVar25,fVar25 * 0.0);
      uStack_1f8 = (ulonglong *)((ulonglong)uStack_1f8 & 0xffffffffffffff00);
      puStack_1f0 = &UNK_142a82b58;
      puStack_1e8 = (ulonglong *)0xc;
      puStack_1e0 = (undefined *)CONCAT71(puStack_1e0._1_7_,1);
      uStack_1d8 = &UNK_142a82b64;
      uStack_1d0 = 0x14;
      puStack_118 = (undefined4 *)*param_2;
      lVar19 = 0;
      while ((lVar19 != 2 && (cVar11 = *(char *)(&uStack_1f8 + lVar19 * 3), cVar11 != '\x02'))) {
        puStack_120 = (undefined4 *)lVar19;
        puVar17 = (&puStack_1f0)[lVar19 * 3];
        puVar1 = (&puStack_1e8)[lVar19 * 3];
        uStack_280 = uStack_108;
        puStack_278 = &uStack_280;
        uStack_270 = &UNK_1400c84c0;
        ppuStack_268 = (ulonglong **)&uStack_100;
        pcStack_260 = FUN_140d4ba00;
        puStack_258 = &uStack_298;
        pcStack_250 = FUN_140d4af90;
        puStack_158 = (ulonglong *)&UNK_142a82b88;
        uStack_150 = (undefined8 *)0x3;
        uStack_140 = 3;
        uStack_138 = 0;
        ppuStack_148 = &puStack_278;
        FUN_140d79f90(&uStack_2b0,&puStack_158);
        uVar8 = uStack_290;
        uVar4 = uStack_298;
        uVar3 = uStack_2a0;
        uVar2 = uStack_2a8;
        uStack_f0 = uStack_2b0;
        uStack_150._0_4_ = uStack_f8;
        puStack_158 = uStack_100;
        puVar14 = (undefined8 *)func_0x000140613c10(0x40,0x10);
        uStack_110 = uVar2;
        if (puVar14 == (undefined8 *)0x0) {
          FUN_1428d9000(0x10,0x40);
                    /* WARNING: Does not return */
          pcVar7 = (code *)invalidInstructionException();
          (*pcVar7)();
        }
        *puVar14 = 1;
        puVar14[1] = 1;
        puVar14[2] = puVar17;
        puVar14[3] = puVar1;
        puVar14[4] = uVar4;
        puVar14[5] = uVar8;
        puVar14[6] = puStack_158;
        *(uint *)(puVar14 + 7) = (uint)uStack_150;
        *(char *)((longlong)puVar14 + 0x3c) = cVar11;
        FUN_14089d8b0(&puStack_158,uVar2,uVar3,puVar14,&UNK_142a82bb8);
        puStack_258 = (undefined8 *)uStack_138;
        ppuStack_268 = ppuStack_148;
        pcStack_260 = (code *)uStack_140;
        puStack_278 = puStack_158;
        uStack_270 = (undefined *)CONCAT44(uStack_150._4_4_,(uint)uStack_150);
        func_0x000140b2cf90(puStack_118,&puStack_278);
        lVar19 = (longlong)puStack_120 + 1;
        if (uStack_f0 != 0) {
          func_0x000140613c20(uStack_110,uStack_f0,1);
        }
      }
      return;
    }
    bVar15 = func_0x00014092df10(&uStack_218);
    if (bVar15 < 2) {
      *(undefined4 *)param_14[2] = *(undefined4 *)((longlong)param_14 + 0x1c);
      *(undefined1 *)(*param_14 + 0x280) = 1;
    }
    func_0x0001408ef180(&fStack_17c,*param_4);
    fVar9 = fStack_17c;
    fVar27 = *param_3 - fStack_17c;
    fVar28 = param_3[2] - fStack_174;
    fVar22 = (float)func_0x00014127c950(*(undefined4 *)(param_12[0x10] + 4),*param_12,
                                        (float)*(uint *)(param_12[4] + 4));
    if (((fVar22 * SQRT(fVar28 * fVar28 + fVar27 * fVar27) < _UNK_14292e6f0) &&
        (bVar15 = func_0x00014092df10(&uStack_218), bVar15 < 2)) &&
       (cVar11 = func_0x000140833150(uStack_f0), cVar11 != '\0')) {
      bVar21 = 1 < bStack_e1;
      uStack_1f8 = puStack_230;
      puStack_1f0 = puStack_238;
      puStack_1e8 = (ulonglong *)0x3;
      uStack_1a8 = 0;
      uStack_1a0 = 0;
      uStack_198 = 0;
      uStack_196 = 0x40020101;
      uStack_190 = (undefined4)uStack_108;
      uStack_18c = (undefined1)uStack_f0;
      FUN_140a1a9f0(&puStack_158,*param_7,uStack_108,&UNK_142a82b40);
      uStack_150._0_5_ = (uint5)(uint)uStack_150;
      *puStack_120 = (undefined4)uStack_110;
      *puStack_118 = uStack_e8;
      *(undefined4 *)param_13[2] = *(undefined4 *)((longlong)param_13 + 0x1c);
      FUN_141276e30(&uStack_1f8,CONCAT71((int7)((ulonglong)param_4 >> 8),bVar21) & 0xffffffff,
                    &puStack_158,uStack_170,uStack_168,*param_13,param_6,uStack_160);
      fVar29 = param_3[6];
      goto code_r0x0001407a8ec8;
    }
    uVar23 = func_0x000140833a00(fVar9 - fVar24);
    puStack_220 = (undefined8 *)func_0x000140613c10(0x18,4);
    if (puStack_220 != (undefined8 *)0x0) {
      puStack_228 = param_2;
      func_0x000140c9cde0(&uStack_1f8,&fStack_240);
      *(float *)(puStack_220 + 1) = fStack_174;
      *puStack_220 = CONCAT44(uStack_178,fStack_17c);
      *(float *)((longlong)puStack_220 + 0x14) = puStack_1f0._0_4_;
      *(ulonglong **)((longlong)puStack_220 + 0xc) = uStack_1f8;
      puStack_158 = (ulonglong *)0x2;
      uStack_150 = puStack_220;
      ppuStack_148 = (ulonglong **)0x2;
      func_0x000140b04e70(&uStack_1f8,&puStack_158);
      FUN_140b04f70(&uStack_1f8,*puStack_228);
      fVar24 = (float)func_0x000140833b80(uVar23);
      if (iVar20 == 1) {
        uStack_1b6 = 2;
        fVar25 = fVar24;
      }
      else if (iVar20 == 3) {
        uStack_1b6 = 4;
      }
      else {
        uStack_1b6 = 3;
        fVar25 = fVar24;
      }
      uStack_1f8 = puStack_230;
      puStack_1f0 = puStack_238;
      puStack_1e8 = puStack_230;
      puStack_1e0 = puStack_238;
      uStack_1d8 = (undefined *)CONCAT44(uStack_200,uStack_204);
      uStack_1d0 = CONCAT44(uVar26,fVar25);
      uStack_1b0 = uStack_e2;
      uStack_1a8 = 0;
      uStack_1a0 = 0;
      uStack_198 = 0;
      uStack_196._0_2_ = (ushort)uStack_196 & 0xff00;
      uStack_196 = CONCAT22(0x4002,(ushort)uStack_196);
      uStack_190 = (undefined4)uStack_108;
      uStack_18c = (undefined1)uStack_f0;
      puStack_158 = uStack_100;
      uStack_150._0_5_ = (uint5)uStack_f8;
      *puStack_120 = (undefined4)uStack_110;
      *puStack_118 = uStack_e8;
      *(undefined4 *)param_13[2] = *(undefined4 *)((longlong)param_13 + 0x1c);
      fStack_1c8 = fVar24;
      FUN_141276e30(&uStack_1f8,1 < bStack_e1,&puStack_158,uStack_170,uStack_168,*param_13,param_6,
                    uStack_160);
      param_3[6] = fVar24;
      fVar29 = (float)((uint)(fVar29 - fVar24) & _UNK_14292e980);
      fVar25 = *(float *)(*param_10 + 0x28);
      uVar16 = 0x18;
      if ((char)uStack_1fc == '\x06') {
        uVar16 = 0x22;
      }
      uVar18 = 0xd;
      if ((char)uStack_1fc != '\x04') {
        uVar18 = uVar16;
      }
      puStack_1e8 = (ulonglong *)CONCAT44(puStack_1e8._4_4_,fStack_174);
      puStack_1f0 = (undefined *)CONCAT44(uStack_178,fStack_17c);
      *(undefined4 *)param_9[2] = *(undefined4 *)((longlong)param_9 + 0x1c);
      lVar19 = *param_9;
      lVar5 = *(longlong *)(lVar19 + 0x30);
      uVar2 = *(undefined8 *)(lVar19 + 0x40);
      if (lVar5 == *(longlong *)(lVar19 + 0x20)) {
        FUN_140562ab0(lVar19 + 0x20,&UNK_142c21f80);
      }
      lVar6 = *(longlong *)(lVar19 + 0x28);
      *(undefined8 *)(lVar6 + lVar5 * 0x28) = uVar2;
      *(undefined1 *)(lVar6 + 8 + lVar5 * 0x28) = uVar18;
      *(float *)(lVar6 + 0xc + lVar5 * 0x28) = fVar29 / fVar25;
      puVar1 = (ulonglong *)(lVar6 + 0x10 + lVar5 * 0x28);
      *puVar1 = (ulonglong)uStack_1f8;
      puVar1[1] = (ulonglong)puStack_1f0;
      *(undefined4 *)(lVar6 + 0x20 + lVar5 * 0x28) = puStack_1e8._0_4_;
      *(undefined1 *)(lVar6 + 0x24 + lVar5 * 0x28) = 0;
      *(longlong *)(lVar19 + 0x30) = lVar5 + 1;
      *(longlong *)(lVar19 + 0x40) = *(longlong *)(lVar19 + 0x40) + 1;
      param_2 = puStack_228;
      fVar29 = fVar24;
      goto code_r0x0001407a8ec8;
    }
  }
  FUN_1428d9000(4,0x18);
override_jmp_1407a8764_case_0:
  FUN_1428d9430(&UNK_142a82ab8,0x28,&UNK_142a82ae0);
  pcVar7 = (code *)swi(3);
  (*pcVar7)();
  return;
}

