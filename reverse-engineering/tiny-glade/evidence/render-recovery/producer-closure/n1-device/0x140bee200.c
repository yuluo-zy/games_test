
undefined8 *
FUN_140bee200(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
             char *param_5,undefined8 *param_6,undefined8 *param_7,undefined8 *param_8)

{
  char cVar1;
  undefined2 uVar2;
  uint uVar3;
  undefined4 uVar4;
  undefined8 *puVar5;
  code *pcVar6;
  int iVar7;
  ulonglong uVar8;
  undefined8 *puVar9;
  undefined8 uVar10;
  longlong lVar11;
  longlong lVar12;
  undefined8 *puVar13;
  undefined8 uVar14;
  undefined8 *puVar15;
  undefined8 auStack_210 [2];
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined8 uStack_1e0;
  undefined1 uStack_1d8;
  undefined1 uStack_1d7;
  undefined4 auStack_1d0 [2];
  undefined8 uStack_1c8;
  undefined4 uStack_1c0;
  undefined8 *puStack_1b8;
  uint uStack_1b0;
  undefined8 uStack_1ac;
  undefined8 uStack_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined8 uStack_188;
  undefined8 *puStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 *puStack_130;
  undefined8 *puStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined8 uStack_f0;
  undefined8 *puStack_e8;
  undefined8 *puStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined8 *puStack_d0;
  undefined8 *puStack_c8;
  undefined8 *puStack_c0;
  undefined8 *puStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 uStack_80;
  undefined8 *puStack_78;
  undefined8 *puStack_70;
  undefined8 *puStack_68;
  undefined8 *puStack_60;
  undefined8 *puStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  puVar5 = (undefined8 *)*param_6;
  uVar3 = *(uint *)(param_6 + 1);
  auStack_1d0[0] = 0xc;
  uStack_1c8 = 0;
  uStack_1c0 = 0;
  uStack_1ac = 0;
  uStack_1a0 = 0;
  puStack_60 = (undefined8 *)param_2[0xb9];
  puStack_1b8 = puVar5;
  uStack_1b0 = uVar3;
  puStack_70 = param_4;
  iVar7 = (*(code *)param_2[0x21])(puStack_60,auStack_1d0,0,&puStack_e8);
  puVar15 = puStack_e8;
  if (iVar7 == 0) {
    if (param_3 != (undefined8 *)0x0) {
      puStack_58 = param_2;
      FUN_140d797d0(&puStack_130,param_7,param_8);
      puVar9 = uStack_120;
      puVar13 = puStack_128;
      if (!SBORROW8(0,(longlong)puStack_130)) {
        uStack_d8 = (undefined4)uStack_120;
        uStack_d4 = uStack_120._4_4_;
        puStack_d0 = uStack_118;
        puStack_e8 = puStack_130;
        puStack_e0 = puStack_128;
        FUN_1428d9760(&UNK_142b15188,0x2b,&puStack_e8,&UNK_142b151d8,&UNK_142b15838);
        goto code_r0x000140bee8c3;
      }
      puStack_e8 = (undefined8 *)CONCAT44(puStack_e8._4_4_,0x3b9cbe00);
      puStack_e0 = (undefined8 *)0x0;
      uStack_d8 = 9;
      puStack_d0 = puVar15;
      puStack_c8 = puStack_128;
      iVar7 = (*(code *)*param_3)(param_3[8],&puStack_e8);
      param_2 = puStack_58;
      if (iVar7 != 0) {
        puStack_70 = puVar13;
        puStack_60 = puVar9;
        puStack_130 = (undefined8 *)CONCAT44(puStack_130._4_4_,iVar7);
        FUN_1428d9760(&UNK_142b15850,0x1b,&puStack_130,&UNK_142b15128,&UNK_142b15870);
        goto code_r0x000140bee8c3;
      }
      *(undefined1 *)puVar13 = 0;
      if (puVar9 != (undefined8 *)0x0) {
        func_0x000140613c20(puVar13,puVar9,1);
      }
    }
    (*(code *)param_2[0xe])(puStack_60,puVar15,&uStack_160);
    uVar8 = uStack_158;
    if (uStack_158 < *(ushort *)((longlong)param_6 + 0xc)) {
      uVar8 = (ulonglong)*(ushort *)((longlong)param_6 + 0xc);
    }
    uStack_158 = 0x40;
    if ((uVar3 & 0x400) == 0) {
      uStack_158 = 0x10;
    }
    if (uStack_158 <= uVar8) {
      uStack_158 = uVar8;
    }
    LOCK();
    cVar1 = *param_5;
    if (cVar1 == '\0') {
      *param_5 = '\x01';
    }
    UNLOCK();
    if (cVar1 != '\0') {
      FUN_1428cd210(param_5,uStack_158,1000000000);
    }
    FUN_140c81c70(param_5);
    uStack_1e0 = uStack_150;
    uStack_1f0 = uStack_160;
    uStack_1ec = uStack_15c;
    uStack_1e8 = (undefined4)uStack_158;
    uStack_1e4 = uStack_158._4_4_;
    uStack_1d7 = *(undefined1 *)((longlong)param_6 + 0xe);
    uStack_200 = 1;
    uStack_1f8 = 0;
    uStack_1d8 = 1;
    auStack_210[0] = 2;
    FUN_140c0da40(&puStack_e8,param_5 + 8,auStack_210);
    puVar13 = puStack_d0;
    uStack_188 = uStack_150;
    uStack_198 = uStack_160;
    uStack_194 = uStack_15c;
    uStack_190 = (undefined4)uStack_158;
    uStack_18c = uStack_158._4_4_;
    if ((char)uStack_a4 == '\x02') {
      puStack_68 = puStack_e0;
      puStack_78 = (undefined8 *)CONCAT44(uStack_d4,uStack_d8);
      puStack_58 = puStack_e8;
      if ((longlong)param_8 < 0) {
        uVar14 = 0;
code_r0x000140bee8ad:
        FUN_1428d8fe3(uVar14,param_8,&UNK_142b15288);
        goto code_r0x000140bee8c3;
      }
      puStack_180 = puVar15;
      if (param_8 == (undefined8 *)0x0) {
        puVar9 = (undefined8 *)0x1;
      }
      else {
        uVar14 = 1;
        puVar9 = (undefined8 *)func_0x000140613c10(param_8,1);
        if (puVar9 == (undefined8 *)0x0) goto code_r0x000140bee8ad;
      }
      func_0x000142923c00(puVar9,param_7,param_8);
      uStack_178 = CONCAT44(uStack_194,uStack_198);
      uStack_170 = CONCAT44(uStack_18c,uStack_190);
      uStack_138 = uStack_188;
      puStack_c0 = param_8;
      uStack_118 = param_8;
      puStack_128 = puStack_78;
      puStack_130 = puStack_68;
      puVar15 = puStack_180;
      uStack_148 = uStack_178;
      uStack_140 = uStack_170;
      if (puStack_58 != (undefined8 *)0xb) {
        uStack_168 = uStack_188;
        param_1[1] = puStack_68;
        param_1[2] = puStack_78;
        param_1[3] = puVar13;
        *(undefined4 *)(param_1 + 7) = uStack_198;
        *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_194;
        *(undefined4 *)(param_1 + 8) = uStack_190;
        *(undefined4 *)((longlong)param_1 + 0x44) = uStack_18c;
        param_1[9] = uStack_188;
        *param_1 = puStack_58;
        param_1[4] = param_8;
        param_1[5] = puVar9;
        param_1[6] = param_8;
        FUN_140c81e10();
        LOCK();
        cVar1 = *param_5;
        if (cVar1 == '\x01') {
          *param_5 = '\0';
        }
        UNLOCK();
        if (cVar1 == '\x01') {
          return param_1;
        }
        FUN_1428cdb90(param_5,0);
        return param_1;
      }
    }
    else {
      puVar13 = (undefined8 *)CONCAT44(uStack_d4,uStack_d8);
      uStack_140 = CONCAT44(uStack_ac,uStack_b0);
      uStack_148 = puStack_b8;
      uStack_138 = CONCAT44(uStack_a4,uStack_a8);
      uStack_118 = puStack_d0;
      puStack_128 = puStack_e0;
      puStack_130 = puStack_e8;
      puVar9 = puStack_c8;
    }
    uStack_168 = uStack_138;
    uStack_178 = uStack_148;
    uStack_170 = uStack_140;
    uStack_100 = (undefined4)uStack_148;
    uStack_fc = uStack_148._4_4_;
    uStack_f8 = (undefined4)uStack_140;
    uStack_f4 = uStack_140._4_4_;
    uStack_f0 = uStack_138;
    uStack_49 = 1;
    uStack_120 = puVar13;
    uStack_110 = puVar9;
    uStack_108 = puStack_c0;
    FUN_140c81e10();
    puVar13 = puStack_70;
    LOCK();
    cVar1 = *param_5;
    if (cVar1 == '\x01') {
      *param_5 = '\0';
    }
    UNLOCK();
    if (cVar1 != '\x01') {
      uStack_49 = 1;
      FUN_1428cdb90(param_5,0);
    }
    uStack_49 = 1;
    uVar14 = func_0x0001406d0cb0(&puStack_130);
    uStack_49 = 1;
    uVar10 = func_0x0001406868b0(&puStack_130);
    puVar9 = puVar15;
    iVar7 = (*(code *)param_2[0xc])(puStack_60,puVar15,uVar14,uVar10);
    if (iVar7 == 0) {
      uStack_49 = 1;
      lVar11 = func_0x000140c113d0(&puStack_130);
      if (puVar5 <= puVar9 || lVar11 == 0) {
        uStack_80 = uStack_f0;
        uStack_90 = uStack_100;
        uStack_8c = uStack_fc;
        uStack_88 = uStack_f8;
        uStack_84 = uStack_f4;
        uStack_a0 = (undefined4)uStack_110;
        uStack_9c = uStack_110._4_4_;
        uStack_98 = (undefined4)uStack_108;
        uStack_94 = uStack_108._4_4_;
        uStack_b0 = (undefined4)uStack_120;
        uStack_ac = uStack_120._4_4_;
        uStack_a8 = (undefined4)uStack_118;
        uStack_a4 = uStack_118._4_4_;
        puStack_c0 = puStack_130;
        puStack_b8 = puStack_128;
        uStack_d8 = *(undefined4 *)param_6;
        uStack_d4 = *(undefined4 *)((longlong)param_6 + 4);
        puStack_d0 = (undefined8 *)param_6[1];
        puStack_e8 = param_7;
        puStack_e0 = param_8;
        uStack_49 = 0;
        puStack_c8 = puVar15;
        lVar12 = FUN_1418ae8a0(puVar13 + 0x30,&puStack_e8);
        uVar4 = *(undefined4 *)(lVar12 + 0xc);
        uVar2 = *(undefined2 *)(puVar13 + 0x14a);
        uVar14 = *puVar13;
        param_1[1] = lVar12 + 0x10;
        param_1[2] = uVar14;
        param_1[3] = lVar12;
        *(undefined4 *)(param_1 + 4) = uVar4;
        *(undefined2 *)((longlong)param_1 + 0x24) = uVar2;
        *(undefined1 *)((longlong)param_1 + 0x26) = 1;
        uVar14 = param_6[1];
        param_1[5] = *param_6;
        param_1[6] = uVar14;
        param_1[7] = lVar11;
        param_1[8] = puVar5;
        *param_1 = 0xb;
        return param_1;
      }
      uStack_49 = 1;
      FUN_1428d9840(puVar5);
      goto code_r0x000140bee8c3;
    }
  }
  else {
    puStack_e8 = (undefined8 *)CONCAT44(puStack_e8._4_4_,iVar7);
    iVar7 = FUN_1428d9760(&UNK_142b157f0,0xd,&puStack_e8,&UNK_142b15128,&UNK_142b15820);
  }
  puStack_e8 = (undefined8 *)CONCAT44(puStack_e8._4_4_,iVar7);
  uStack_49 = 1;
  FUN_1428d9760(&UNK_142b15888,0x12,&puStack_e8,&UNK_142b15128,&UNK_142b158a0);
code_r0x000140bee8c3:
                    /* WARNING: Does not return */
  pcVar6 = (code *)invalidInstructionException();
  (*pcVar6)();
}

