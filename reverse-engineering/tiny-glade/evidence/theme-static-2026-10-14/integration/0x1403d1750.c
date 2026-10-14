
uint FUN_1403d1750(undefined8 *param_1,undefined8 *param_2)

{
  byte bVar1;
  undefined4 uVar2;
  byte *pbVar3;
  undefined4 *puVar4;
  longlong lVar5;
  code *pcVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  char cVar10;
  uint uVar11;
  undefined8 uVar12;
  undefined *puVar13;
  undefined8 uVar14;
  undefined *puVar15;
  undefined *puVar16;
  undefined8 *puVar17;
  undefined *puVar18;
  undefined4 uStack_2f8;
  undefined4 uStack_2f4;
  undefined4 uStack_2f0;
  undefined4 uStack_2ec;
  undefined *puStack_2c8;
  undefined *puStack_2c0;
  undefined *puStack_2b8;
  longlong lStack_2b0;
  longlong lStack_2a8;
  longlong lStack_2a0;
  undefined4 uStack_298;
  undefined4 uStack_294;
  undefined4 uStack_290;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  undefined4 uStack_280;
  undefined4 uStack_27c;
  undefined *puStack_278;
  undefined *puStack_270;
  longlong lStack_268;
  longlong lStack_260;
  undefined4 uStack_258;
  uint uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  longlong lStack_238;
  longlong lStack_230;
  longlong lStack_228;
  undefined *puStack_1e8;
  undefined *puStack_1e0;
  undefined *puStack_1d8;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  uint uStack_1bc;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  longlong lStack_198;
  longlong lStack_190;
  undefined *puStack_188;
  longlong lStack_180;
  undefined *puStack_178;
  undefined *puStack_170;
  undefined8 uStack_168;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined *puStack_138;
  undefined *puStack_130;
  undefined *puStack_118;
  undefined *puStack_110;
  undefined *puStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined4 uStack_f0;
  uint uStack_ec;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  longlong lStack_c8;
  longlong lStack_c0;
  undefined **ppuStack_b8;
  undefined *puStack_b0;
  undefined *puStack_a8;
  undefined *puStack_a0;
  undefined *puStack_98;
  uint uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined *puStack_78;
  undefined *puStack_70;
  undefined *puStack_68;
  byte *pbStack_60;
  undefined *puStack_58;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  pbVar3 = (byte *)*param_1;
  LOCK();
  bVar1 = *pbVar3;
  if (bVar1 == 0) {
    *pbVar3 = 1;
  }
  UNLOCK();
  if (bVar1 != 0) {
    FUN_1428cd210(pbVar3,param_2,1000000000);
  }
  FUN_140c81c70(pbVar3);
  pbStack_60 = pbVar3;
  if (*(longlong *)(pbVar3 + 8) == 0) {
    FUN_1404f2380(&puStack_118,*(undefined8 *)(pbVar3 + 0x10));
  }
  else if ((int)*(longlong *)(pbVar3 + 8) == 1) {
    FUN_140267200(&puStack_118);
  }
  else {
    FUN_14051fa20(&puStack_118);
  }
  pbVar3 = pbStack_60;
  uVar11 = uStack_ec;
  uVar9 = uStack_f0;
  puVar13 = uStack_100;
  puVar18 = puStack_108;
  puVar16 = puStack_110;
  puVar15 = puStack_118;
  if (puStack_118 == (undefined *)0x8000000000000001) {
    FUN_140c81e10();
    uVar11 = 1;
    LOCK();
    bVar1 = *pbVar3;
    if (bVar1 == 1) {
      *pbVar3 = 0;
    }
    else {
      uVar11 = (uint)bVar1;
    }
    UNLOCK();
    if (bVar1 == 1) {
      return uVar11;
    }
    uVar11 = FUN_1428cdb90(pbVar3,0);
    return uVar11;
  }
  uVar7 = (undefined4)uStack_f8;
  uVar8 = uStack_f8._4_4_;
  uStack_49 = 1;
  FUN_140c81e10();
  LOCK();
  bVar1 = *pbVar3;
  if (bVar1 == 1) {
    *pbVar3 = 0;
  }
  UNLOCK();
  if (bVar1 != 1) {
    uStack_49 = 1;
    FUN_1428cdb90(pbVar3,0);
  }
  uVar2 = *(undefined4 *)((longlong)param_2 + 0x1c);
  puVar4 = (undefined4 *)param_2[2];
  *puVar4 = uVar2;
  pbStack_60 = (byte *)*param_2;
  if (SBORROW8(0,*(longlong *)pbStack_60)) {
    lVar5 = *(longlong *)((longlong)pbStack_60 + 8);
    if ((-0x7ffffffffffffffe < lVar5 || lVar5 == -0x7fffffffffffffff) && (lVar5 != 0)) {
      func_0x000140613c20(*(longlong *)((longlong)pbStack_60 + 0x10),lVar5,1);
    }
  }
  else {
    FUN_1405fa460();
  }
  pbVar3 = pbStack_60;
  uVar14 = 0x8000000000000000;
  pbStack_60[0] = 0;
  pbStack_60[1] = 0;
  pbStack_60[2] = 0;
  pbStack_60[3] = 0;
  pbStack_60[4] = 0;
  pbStack_60[5] = 0;
  pbStack_60[6] = 0;
  pbStack_60[7] = 0x80;
  *(longlong *)((longlong)pbStack_60 + 8) = -0x8000000000000000;
  puStack_138 = puVar15;
  puStack_130 = puVar16;
  uStack_7c = uVar7;
  uStack_80 = uVar8;
  uStack_84 = uVar9;
  uStack_88 = uVar11;
  if (SBORROW8(0,(longlong)puVar15)) {
    puStack_78 = (undefined *)0x8000000000000000;
    goto LAB_1403d1f0e;
  }
  uVar12 = FUN_140d76ed0(&puStack_138);
  FUN_140d62810(&puStack_118,uVar12,uVar14,&UNK_1429d1d78,10);
  uVar14 = FUN_140d76ed0(&puStack_118);
  FUN_140d58eb0(&uStack_168,uVar14,uVar12);
  if (puStack_118 != (undefined *)0x0) {
    func_0x000140613c20(puStack_110,puStack_118,1);
  }
  puVar13 = (undefined *)CONCAT44(uStack_15c,uStack_160);
  if (SBORROW8(0,(longlong)uStack_168)) {
    uStack_4a = 1;
    puStack_188 = puVar13;
    FUN_140d56700(&puStack_118);
    uStack_1c8 = (undefined4)uStack_f8;
    uStack_1c4 = uStack_f8._4_4_;
    uStack_1c0 = uStack_f0;
    uStack_1bc = uStack_ec;
    uStack_1d0 = (undefined4)uStack_100;
    uStack_1cc = uStack_100._4_4_;
    puStack_1e8 = puStack_118;
    puStack_1e0 = puStack_110;
    puStack_118 = &UNK_1429d1d82;
    puStack_110 = (undefined *)0x12;
    uStack_4a = 0;
    puStack_108 = puVar13;
    puVar13 = (undefined *)FUN_1428a3530(&puStack_118,&puStack_1e8);
  }
  else {
    puStack_58 = uStack_168;
    puStack_68 = puVar13;
    FUN_1400570b0(&puStack_a8,puVar13,CONCAT44(uStack_154,uStack_158));
    puVar18 = puStack_98;
    puVar16 = puStack_a0;
    if (SBORROW8(0,(longlong)puStack_a8)) {
      uStack_168 = puStack_a0;
      uStack_4b = 1;
      FUN_140d56700(&puStack_118);
      uStack_1c8 = (undefined4)uStack_f8;
      uStack_1c4 = uStack_f8._4_4_;
      uStack_1c0 = uStack_f0;
      uStack_1bc = uStack_ec;
      uStack_1d0 = (undefined4)uStack_100;
      uStack_1cc = uStack_100._4_4_;
      puStack_1e8 = puStack_118;
      puStack_1e0 = puStack_110;
      puStack_118 = &UNK_1429d1d94;
      puStack_110 = (undefined *)0x12;
      puStack_108 = puVar16;
      uStack_4b = 0;
      puVar13 = (undefined *)FUN_1428a3450(&puStack_118,&puStack_1e8);
      if (puStack_58 != (undefined *)0x0) {
        func_0x000140613c20(puStack_68,puStack_58,1);
      }
    }
    else {
      puStack_78 = puStack_a8;
      puVar15 = puStack_58;
      if (puStack_58 != (undefined *)0x0) {
        func_0x000140613c20(puStack_68,puStack_58,1);
      }
      uVar14 = FUN_140d76ed0(&puStack_138);
      FUN_140d62810(&puStack_a8,uVar14,puVar15,&UNK_1429d1da6,9);
      uVar12 = FUN_140d76ed0(&puStack_a8);
      FUN_140b46480(&puStack_118,uVar12,uVar14);
      if (SBORROW8(0,(longlong)puStack_118)) {
        puStack_188 = &UNK_1429d1daf;
        lStack_180 = 0x14;
        puStack_178 = puStack_110;
        uStack_168 = (undefined *)0x3;
        puVar13 = (undefined *)FUN_1428a3c20(&puStack_188,&uStack_168);
        if (puStack_a8 != (undefined *)0x0) {
          func_0x000140613c20(puStack_a0,puStack_a8,1);
        }
      }
      else {
        lStack_198 = lStack_c8;
        lStack_190 = lStack_c0;
        uStack_1a8 = uStack_d8;
        uStack_1a4 = uStack_d4;
        uStack_1a0 = uStack_d0;
        uStack_19c = uStack_cc;
        uStack_1b8 = (undefined4)uStack_e8;
        uStack_1b4 = uStack_e8._4_4_;
        uStack_1b0 = (undefined4)uStack_e0;
        uStack_1ac = uStack_e0._4_4_;
        uStack_1c8 = (undefined4)uStack_f8;
        uStack_1c4 = uStack_f8._4_4_;
        uStack_1c0 = uStack_f0;
        uStack_1bc = uStack_ec;
        puStack_1d8 = puStack_108;
        uStack_1d0 = (undefined4)uStack_100;
        uStack_1cc = uStack_100._4_4_;
        puStack_1e8 = puStack_118;
        puStack_1e0 = puStack_110;
        puVar15 = puStack_a8;
        if (puStack_a8 != (undefined *)0x0) {
          func_0x000140613c20(puStack_a0,puStack_a8,1);
        }
        uVar14 = FUN_140d76ed0(&puStack_138);
        FUN_140d62810(&uStack_168,uVar14,puVar15,&UNK_1429d1dc3,0xc);
        uVar12 = FUN_140d76ed0(&uStack_168);
        cVar10 = FUN_140d62cf0(uVar12);
        if (cVar10 != '\0') {
          uVar12 = FUN_140d76ed0(&puStack_138);
          FUN_140d62810(&puStack_118,uVar12,uVar14,&UNK_1429d1df8,0xc);
          FUN_140310740(&puStack_a8,&puStack_118);
          if (SBORROW8(0,(longlong)puStack_a8)) {
            FUN_1405e7a00(&puStack_a0);
LAB_1403d2120:
            puStack_b0 = (undefined *)0x8000000000000000;
            puStack_70 = (undefined *)0x8000000000000000;
            ppuStack_b8 = (undefined **)0x8000000000000000;
          }
          else {
            puStack_58 = puStack_a8;
            puStack_68 = puStack_a0;
            FUN_140058e20(&puStack_118,puStack_a0,puStack_98);
            puStack_70 = puStack_118;
            if (puStack_118 == (undefined *)0x8000000000000001) {
              ppuStack_b8 = (undefined **)FUN_1405e88a0();
              puStack_b0 = puVar13;
            }
            else {
              puStack_a8 = puStack_110;
              puStack_a0 = puStack_108;
              lStack_180 = CONCAT44(uStack_ec,uStack_f0);
              puStack_188 = uStack_f8;
              uStack_2f8 = (undefined4)uStack_e0;
              uStack_2f4 = uStack_e0._4_4_;
              uStack_2f0 = uStack_d8;
              uStack_2ec = uStack_d4;
              ppuStack_b8 = uStack_e8;
              puStack_b0 = uStack_100;
            }
            if (puStack_58 != (undefined *)0x0) {
              func_0x000140613c20(puStack_68,puStack_58,1);
            }
            if (puStack_70 == (undefined *)0x8000000000000001) goto LAB_1403d2120;
            puStack_2c8 = puStack_a8;
            puStack_2c0 = puStack_a0;
            puStack_2b8 = puStack_188;
            lStack_2b0 = lStack_180;
            lStack_2a8 = CONCAT44(uStack_2f4,uStack_2f8);
            lStack_2a0 = CONCAT44(uStack_2ec,uStack_2f0);
          }
          puVar13 = puStack_1e8;
          puStack_278 = puStack_1e0;
          puStack_270 = puStack_1d8;
          lStack_268 = CONCAT44(uStack_1cc,uStack_1d0);
          lStack_260 = CONCAT44(uStack_1c4,uStack_1c8);
          uStack_258 = uStack_1c0;
          uStack_254 = uStack_1bc;
          uStack_250 = uStack_1b8;
          uStack_24c = uStack_1b4;
          uStack_248 = uStack_1b0;
          uStack_244 = uStack_1ac;
          uStack_240 = uStack_1a8;
          uStack_23c = uStack_1a4;
          lStack_238 = CONCAT44(uStack_19c,uStack_1a0);
          lStack_230 = lStack_198;
          lStack_228 = lStack_190;
          uStack_298 = (undefined4)uStack_168;
          uStack_294 = uStack_168._4_4_;
          uStack_290 = uStack_160;
          uStack_28c = uStack_15c;
          uStack_288 = uStack_158;
          uStack_284 = uStack_154;
          uStack_280 = uStack_150;
          uStack_27c = uStack_14c;
          if (puStack_138 != (undefined *)0x0) {
            func_0x000140613c20(puStack_130,puStack_138,1);
          }
          goto LAB_1403d1f0e;
        }
        puStack_118 = &UNK_1429d1de8;
        puStack_110 = (undefined *)0x1;
        puStack_108 = (undefined *)0x8;
        uStack_100 = (undefined *)0x0;
        uStack_f8 = (undefined *)0x0;
        puVar13 = (undefined *)FUN_1428a2020(&puStack_118);
        if (uStack_168 != (undefined *)0x0) {
          func_0x000140613c20(CONCAT44(uStack_15c,uStack_160),uStack_168,1);
        }
        FUN_14014b4d0(&puStack_1e8);
      }
      if (puVar18 != (undefined *)0x0) {
        puVar17 = (undefined8 *)(puVar16 + 8);
        do {
          if (puVar17[-1] != 0) {
            func_0x000140613c20(*puVar17,puVar17[-1],1);
          }
          puVar17 = puVar17 + 3;
          puVar18 = puVar18 + -1;
        } while (puVar18 != (undefined *)0x0);
      }
      if (puStack_78 != (undefined *)0x0) {
        func_0x000140613c20(puVar16,(longlong)puStack_78 * 0x18,8);
      }
    }
  }
  if (puStack_138 != (undefined *)0x0) {
    func_0x000140613c20(puStack_130,puStack_138,1);
  }
  puStack_1e8 = (undefined *)0x0;
  puStack_1e0 = (undefined *)0x1;
  puStack_1d8 = (undefined *)0x0;
  puStack_118 = (undefined *)0x0;
  puStack_108 = (undefined *)0x0;
  uStack_f8 = (undefined *)0x20;
  uStack_f0 = CONCAT31(uStack_f0._1_3_,3);
  uStack_e8 = &puStack_1e8;
  uStack_e0 = &UNK_1429d18d8;
  uStack_168 = puVar13;
  cVar10 = func_0x000140d4cc10(&uStack_168,&puStack_118);
  puVar13 = puStack_1d8;
  puVar18 = puStack_1e0;
  puVar16 = puStack_1e8;
  if (cVar10 != '\0') {
    FUN_1428d9760(&UNK_1429d1908,0x37,&puStack_a8,&UNK_1429d19c8,&UNK_1429d1990);
                    /* WARNING: Does not return */
    pcVar6 = (code *)invalidInstructionException();
    (*pcVar6)();
  }
  uStack_49 = 0;
  func_0x000140d4cc30(&uStack_168);
  puStack_78 = (undefined *)0x8000000000000000;
LAB_1403d1f0e:
  *puVar4 = uVar2;
  puStack_170 = puVar13;
  puStack_68 = puVar18;
  puStack_58 = puVar16;
  if (SBORROW8(0,*(longlong *)pbVar3)) {
    lVar5 = *(longlong *)((longlong)pbVar3 + 8);
    if ((-0x7ffffffffffffffe < lVar5 || lVar5 == -0x7fffffffffffffff) && (lVar5 != 0)) {
      func_0x000140613c20(*(longlong *)((longlong)pbStack_60 + 0x10),lVar5,1);
    }
  }
  else {
    FUN_1405fa460(pbVar3);
  }
  *(undefined **)pbStack_60 = puStack_78;
  *(undefined **)((longlong)pbStack_60 + 8) = puStack_58;
  *(undefined **)((longlong)pbStack_60 + 0x10) = puStack_68;
  *(undefined **)((longlong)pbStack_60 + 0x18) = puStack_170;
  *(undefined **)((longlong)pbStack_60 + 0x20) = puStack_278;
  *(undefined **)((longlong)pbStack_60 + 0x28) = puStack_270;
  *(longlong *)((longlong)pbStack_60 + 0x30) = lStack_268;
  *(longlong *)((longlong)pbStack_60 + 0x38) = lStack_260;
  *(longlong *)((longlong)pbStack_60 + 0x40) = CONCAT44(uStack_254,uStack_258);
  *(longlong *)((longlong)pbStack_60 + 0x48) = CONCAT44(uStack_24c,uStack_250);
  *(longlong *)((longlong)pbStack_60 + 0x50) = CONCAT44(uStack_244,uStack_248);
  *(longlong *)((longlong)pbStack_60 + 0x58) = CONCAT44(uStack_23c,uStack_240);
  *(longlong *)((longlong)pbStack_60 + 0x60) = lStack_238;
  *(longlong *)((longlong)pbStack_60 + 0x68) = lStack_230;
  *(longlong *)((longlong)pbStack_60 + 0x70) = lStack_228;
  *(longlong *)((longlong)pbStack_60 + 0x78) = CONCAT44(uStack_294,uStack_298);
  *(longlong *)((longlong)pbStack_60 + 0x80) = CONCAT44(uStack_28c,uStack_290);
  *(longlong *)((longlong)pbStack_60 + 0x88) = CONCAT44(uStack_284,uStack_288);
  *(longlong *)((longlong)pbStack_60 + 0x90) = CONCAT44(uStack_27c,uStack_280);
  *(undefined **)((longlong)pbStack_60 + 0x98) = puStack_70;
  *(undefined **)((longlong)pbStack_60 + 0xa0) = puStack_2c8;
  *(undefined **)((longlong)pbStack_60 + 0xa8) = puStack_2c0;
  *(undefined **)((longlong)pbStack_60 + 0xb0) = puStack_b0;
  *(undefined **)((longlong)pbStack_60 + 0xb8) = puStack_2b8;
  *(longlong *)((longlong)pbStack_60 + 0xc0) = lStack_2b0;
  *(undefined ***)((longlong)pbStack_60 + 200) = ppuStack_b8;
  *(longlong *)((longlong)pbStack_60 + 0xd0) = lStack_2a8;
  *(longlong *)((longlong)pbStack_60 + 0xd8) = lStack_2a0;
  *(undefined4 *)((longlong)pbStack_60 + 0xe0) = uStack_7c;
  *(undefined4 *)((longlong)pbStack_60 + 0xe4) = uStack_80;
  *(undefined4 *)((longlong)pbStack_60 + 0xe8) = uStack_84;
  *(uint *)((longlong)pbStack_60 + 0xec) = uStack_88;
  return uStack_88;
}

