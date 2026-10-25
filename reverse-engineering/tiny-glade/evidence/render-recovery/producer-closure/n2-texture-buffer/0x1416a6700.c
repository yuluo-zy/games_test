
uint * FUN_1416a6700(uint *param_1,longlong param_2,uint *param_3,uint *param_4,uint *param_5,
                    undefined8 param_6,undefined8 param_7)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  uint *puVar4;
  uint *puVar5;
  int iVar6;
  undefined8 *puVar7;
  uint *puVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  uint *unaff_R12;
  longlong unaff_R13;
  bool bVar11;
  undefined1 auStack_418 [188];
  char cStack_35c;
  undefined2 uStack_35b;
  undefined1 uStack_359;
  undefined1 auStack_358 [28];
  int iStack_33c;
  int iStack_338;
  int iStack_334;
  uint *apuStack_300 [3];
  uint *puStack_2e8;
  uint *puStack_2e0;
  uint *puStack_2d8;
  uint *puStack_2d0;
  undefined8 uStack_2b8;
  undefined2 uStack_2b0;
  undefined6 uStack_2ae;
  undefined8 uStack_228;
  undefined8 uStack_220;
  uint *puStack_218;
  uint *puStack_210;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  uint uStack_1f0;
  uint uStack_1ec;
  undefined8 uStack_1e8;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  uint uStack_1c0;
  uint uStack_1bc;
  uint *puStack_1b8;
  uint *puStack_1b0;
  uint *puStack_1a8;
  uint *puStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  uint uStack_180;
  uint uStack_17c;
  undefined8 uStack_178;
  uint *puStack_170;
  uint *puStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  uint uStack_130;
  uint uStack_12c;
  uint uStack_128;
  uint uStack_124;
  undefined8 uStack_120;
  char cStack_ac;
  undefined2 uStack_ab;
  undefined1 uStack_a9;
  longlong lStack_a8;
  undefined8 uStack_a0;
  longlong lStack_98;
  undefined4 uStack_90;
  undefined2 uStack_8c;
  undefined1 uStack_8a;
  undefined1 uStack_89;
  uint uStack_88;
  uint uStack_84;
  uint uStack_80;
  uint uStack_7c;
  undefined8 uStack_78;
  uint *puStack_70;
  uint *puStack_68;
  uint *puStack_60;
  uint *puStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_140bd5a40(apuStack_300,param_6,param_7);
  if (*param_5 == 0) {
    FUN_1428d9430(&UNK_142c88dda,0x3c,&UNK_142c88e18);
code_r0x0001416a6e7b:
    FUN_1428d9430(&UNK_142c88e30,0x2e,&UNK_142c88e60);
code_r0x0001416a6e93:
    FUN_1428d9430(&UNK_142c88e78,0x2f,&UNK_142c88ea8);
code_r0x0001416a6eab:
    FUN_1428d9430(&UNK_142c88ec0,0x2e,&UNK_142c88ef0);
    puStack_60 = param_5;
code_r0x0001416a6ec3:
    FUN_1428cd210(puStack_60);
code_r0x0001416a68cc:
    FUN_140c81c70(puStack_60);
    uStack_2b8 = uStack_78;
    _uStack_2b0 = CONCAT62(uStack_2ae,0x100);
    puStack_2e8 = (uint *)0x2;
    puStack_2d8 = param_3;
    puStack_2d0 = param_4;
    FUN_140c0da40(&puStack_168,unaff_R13 + 0x18,&puStack_2e8);
    puVar4 = uStack_150;
    puStack_58 = puStack_168;
    if ((char)uStack_124 == '\x02') {
      puStack_68 = uStack_160;
      puStack_70 = uStack_158;
      puVar8 = (uint *)func_0x000140613c10(0xf,1);
      puVar5 = puStack_60;
      if (puVar8 == (uint *)0x0) {
        FUN_1428d8fe3(1,0xf,&UNK_142c88780);
        goto code_r0x0001416a6ffa;
      }
      builtin_strncpy((char *)((longlong)puVar8 + 7)," texture",8);
      puVar8[0] = 0x4f757047;
      puVar8[1] = 0x20796c6e;
      puStack_218 = (uint *)uStack_78;
      uStack_228 = (uint *)CONCAT44(uStack_84,uStack_88);
      uStack_220._0_4_ = uStack_80;
      uStack_220._4_4_ = uStack_7c;
      if (puStack_58 != (uint *)0xb) {
        uStack_1c8 = uStack_78;
        *(uint **)(param_1 + 2) = puStack_68;
        *(uint **)(param_1 + 4) = puStack_70;
        *(uint **)(param_1 + 6) = puVar4;
        param_1[0xe] = uStack_88;
        param_1[0xf] = uStack_84;
        param_1[0x10] = uStack_80;
        param_1[0x11] = uStack_7c;
        *(undefined8 *)(param_1 + 0x12) = uStack_78;
        *(uint **)param_1 = puStack_58;
        param_1[8] = 0xf;
        param_1[9] = 0;
        *(uint **)(param_1 + 10) = puVar8;
        param_1[0xc] = 0xf;
        param_1[0xd] = 0;
        FUN_140c81e10();
        LOCK();
        bVar11 = (char)*puVar5 == '\x01';
        if (bVar11) {
          *(char *)puVar5 = '\0';
        }
        UNLOCK();
        if (bVar11) {
          return param_1;
        }
        FUN_1428cdb90(puVar5,0);
        return param_1;
      }
      puStack_58 = puStack_68;
      uStack_198 = puVar8;
      puStack_1a8 = puVar4;
      uStack_190 = 0xf;
      puStack_1b0 = puStack_70;
      puStack_1a0 = (uint *)0xf;
    }
    else {
      uStack_228 = (uint *)uStack_138;
      uStack_220._0_4_ = uStack_130;
      uStack_220._4_4_ = uStack_12c;
      puStack_218 = (uint *)CONCAT44(uStack_124,uStack_128);
      uStack_198 = uStack_148;
      puStack_1a8 = uStack_158;
      uStack_190 = uStack_140;
      puStack_1b0 = uStack_160;
      puStack_1a0 = uStack_150;
    }
    puVar4 = puStack_60;
    uStack_1c8 = puStack_218;
    uStack_1d8 = uStack_228;
    puStack_1b8 = puStack_58;
    uStack_188 = (undefined4)uStack_228;
    uStack_184 = uStack_228._4_4_;
    uStack_180 = (uint)uStack_220;
    uStack_17c = uStack_220._4_4_;
    uStack_178 = puStack_218;
    uStack_49 = 1;
    FUN_140c81e10();
    LOCK();
    bVar11 = (char)*puVar4 == '\x01';
    if (bVar11) {
      *(char *)puVar4 = '\0';
    }
    UNLOCK();
    if (!bVar11) {
      uStack_49 = 1;
      FUN_1428cdb90(puVar4,0);
    }
    uStack_49 = 1;
    uVar9 = func_0x0001406d0cb0(&puStack_1b8);
    uStack_49 = 1;
    uVar10 = func_0x0001406868b0(&puStack_1b8);
    iVar6 = (**(code **)(param_2 + 0xe0))(*(undefined8 *)(param_2 + 0x640),unaff_R12,uVar9,uVar10);
    if (iVar6 == 0) {
      puVar7 = *(undefined8 **)(param_2 + 0x648);
      uStack_1e8 = uStack_178;
      uStack_1f8 = uStack_188;
      uStack_1f4 = uStack_184;
      uStack_1f0 = uStack_180;
      uStack_1ec = uStack_17c;
      uStack_208 = (undefined4)uStack_198;
      uStack_204 = uStack_198._4_4_;
      uStack_200 = (undefined4)uStack_190;
      uStack_1fc = uStack_190._4_4_;
      puStack_218 = puStack_1a8;
      puStack_210 = puStack_1a0;
      uStack_228 = puStack_1b8;
      uStack_220 = puStack_1b0;
      uStack_1d8 = *(undefined8 *)param_5;
      uStack_1d0 = *(undefined8 *)(param_5 + 2);
      uStack_1c8 = *(undefined8 *)(param_5 + 4);
      uStack_1c0 = param_5[6];
      uStack_1bc = param_5[7];
      uStack_49 = 0;
      FUN_140bdc2d0(&puStack_168,param_3,param_4,unaff_R12,param_2 + 0x78,&uStack_228,&uStack_1d8);
      if (cStack_ac == '\x03') {
        *(ulonglong *)(param_1 + 0x10) = CONCAT44(uStack_124,uStack_128);
        *(undefined8 *)(param_1 + 0x12) = uStack_120;
        *(undefined8 *)(param_1 + 0xc) = uStack_138;
        *(ulonglong *)(param_1 + 0xe) = CONCAT44(uStack_12c,uStack_130);
        *(uint **)(param_1 + 8) = uStack_148;
        *(undefined8 *)(param_1 + 10) = uStack_140;
        *(uint **)(param_1 + 4) = uStack_158;
        *(uint **)(param_1 + 6) = uStack_150;
        *(uint **)param_1 = puStack_168;
        *(uint **)(param_1 + 2) = uStack_160;
      }
      else {
        func_0x000142923c00(&puStack_2e8,&puStack_168,0xbc);
        uStack_35b = uStack_ab;
        uStack_359 = uStack_a9;
        func_0x000142923c00(auStack_418,&puStack_2e8,0xbc);
        cStack_35c = cStack_ac;
        uStack_49 = 0;
        lStack_98 = FUN_140bdbd50(puVar7,auStack_418);
        uStack_90 = *(undefined4 *)(lStack_98 + 0xc);
        uStack_8c = *(undefined2 *)(puVar7 + 0x14a);
        uStack_a0 = *puVar7;
        lStack_a8 = lStack_98 + 0x10;
        uStack_8a = 0;
        if (puStack_170 != (uint *)0x0) {
          puStack_2e8 = *(uint **)param_5;
          puStack_2e0 = *(uint **)(param_5 + 2);
          puStack_2d8 = *(uint **)(param_5 + 4);
          puStack_2d0 = *(uint **)(param_5 + 6);
          FUN_140bfe270(&puStack_168,param_2,&puStack_2e8,apuStack_300,&lStack_a8);
          if (puStack_168 != (uint *)0xb) {
            *(undefined8 *)(param_1 + 0x12) = uStack_120;
            param_1[0xe] = uStack_130;
            param_1[0xf] = uStack_12c;
            param_1[0x10] = uStack_128;
            param_1[0x11] = uStack_124;
            param_1[10] = (uint)uStack_140;
            param_1[0xb] = uStack_140._4_4_;
            param_1[0xc] = (uint)uStack_138;
            param_1[0xd] = uStack_138._4_4_;
            param_1[6] = (uint)uStack_150;
            param_1[7] = uStack_150._4_4_;
            param_1[8] = (uint)uStack_148;
            param_1[9] = uStack_148._4_4_;
            param_1[2] = (uint)uStack_160;
            param_1[3] = uStack_160._4_4_;
            param_1[4] = (uint)uStack_158;
            param_1[5] = uStack_158._4_4_;
            *(uint **)param_1 = puStack_168;
            uStack_49 = 0;
            FUN_140be24b0(&uStack_a0);
            return param_1;
          }
        }
        uVar9 = *(undefined8 *)param_5;
        uVar10 = *(undefined8 *)(param_5 + 2);
        uVar2 = *(undefined8 *)(param_5 + 4);
        uVar3 = *(undefined8 *)(param_5 + 6);
        *(undefined8 *)(param_1 + 2) = uStack_a0;
        *(longlong *)(param_1 + 4) = lStack_98;
        *(undefined8 *)(param_1 + 0xe) = uVar3;
        *(undefined8 *)(param_1 + 10) = uVar10;
        *(undefined8 *)(param_1 + 0xc) = uVar2;
        *(ulonglong *)(param_1 + 6) =
             CONCAT17(uStack_89,CONCAT16(uStack_8a,CONCAT24(uStack_8c,uStack_90)));
        *(undefined8 *)(param_1 + 8) = uVar9;
        param_1[0] = 0xb;
        param_1[1] = 0;
      }
      return param_1;
    }
  }
  else {
    if (apuStack_300[0] != (uint *)0x0) {
      *param_5 = *param_5 | 2;
    }
    FUN_140bd5ed0(auStack_358,param_5,apuStack_300[0] != (uint *)0x0);
    unaff_R12 = apuStack_300[0];
    if (iStack_33c == 0) goto code_r0x0001416a6e7b;
    if (iStack_338 == 0) goto code_r0x0001416a6e93;
    if (iStack_334 == 0) goto code_r0x0001416a6eab;
    puStack_170 = apuStack_300[0];
    iVar6 = (**(code **)(param_2 + 0x1a0))
                      (*(undefined8 *)(param_2 + 0x640),auStack_358,0,&puStack_168);
    unaff_R12 = puStack_168;
    if (iVar6 == 0) {
      puVar7 = (undefined8 *)func_0x000140bf8960(param_2);
      if (puVar7 != (undefined8 *)0x0) {
        puStack_58 = param_1;
        FUN_140d797d0(&puStack_2e8,param_3,param_4);
        puVar4 = puStack_2d8;
        if (!SBORROW8(0,(longlong)puStack_2e8)) {
          uStack_158 = puStack_2d8;
          uStack_150 = puStack_2d0;
          puStack_168 = puStack_2e8;
          uStack_160 = puStack_2e0;
          FUN_1428d9760(&UNK_142c88708,0x2b,&puStack_168,&UNK_142c886e8,&UNK_142c88f30);
          goto code_r0x0001416a6ffa;
        }
        puStack_168 = (uint *)CONCAT44(puStack_168._4_4_,0x3b9cbe00);
        uStack_160 = (uint *)0x0;
        uStack_158 = (uint *)CONCAT44(uStack_158._4_4_,10);
        uStack_150 = unaff_R12;
        uStack_148 = puStack_2e0;
        iVar6 = (*(code *)*puVar7)(puVar7[8],&puStack_168);
        param_1 = puStack_58;
        if (iVar6 != 0) {
          puStack_60 = puStack_2e0;
          puStack_58 = puVar4;
          puStack_2e8 = (uint *)CONCAT44(puStack_2e8._4_4_,iVar6);
          FUN_1428d9760(&UNK_142c88f48,0x1b,&puStack_2e8,&UNK_142c886c8,&UNK_142c88f68);
          goto code_r0x0001416a6ffa;
        }
        *(char *)puStack_2e0 = '\0';
        if (puVar4 != (uint *)0x0) {
          func_0x000140613c20(puStack_2e0,puVar4,1);
        }
      }
      (**(code **)(param_2 + 0xf0))(*(undefined8 *)(param_2 + 0x640),unaff_R12,&uStack_88);
      unaff_R13 = *(longlong *)(param_2 + 0x678);
      puStack_60 = (uint *)(unaff_R13 + 0x10);
      LOCK();
      bVar11 = *(char *)(unaff_R13 + 0x10) == '\0';
      if (bVar11) {
        *(char *)(unaff_R13 + 0x10) = '\x01';
      }
      UNLOCK();
      if (!bVar11) goto code_r0x0001416a6ec3;
      goto code_r0x0001416a68cc;
    }
    puStack_168 = (uint *)CONCAT44(puStack_168._4_4_,iVar6);
    iVar6 = FUN_1428d9760(&UNK_142c88f08,0xc,&puStack_168,&UNK_142c886c8,&UNK_142c88f18);
  }
  puStack_168 = (uint *)CONCAT44(puStack_168._4_4_,iVar6);
  uStack_49 = 1;
  FUN_1428d9760(&UNK_142c88f80,0x11,&puStack_168,&UNK_142c886c8,&UNK_142c88f98);
code_r0x0001416a6ffa:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

