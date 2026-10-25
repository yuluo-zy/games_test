
undefined4 *
FUN_140bef4a0(undefined4 *param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
             uint param_5)

{
  longlong lVar1;
  char cVar2;
  ulonglong uVar3;
  code *pcVar4;
  undefined *puVar5;
  undefined1 *puVar6;
  int iVar7;
  longlong lVar8;
  undefined8 *puVar9;
  ulonglong uVar10;
  char *pcVar11;
  bool bVar12;
  int iStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined *puStack_140;
  undefined8 uStack_138;
  int iStack_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined *puStack_e8;
  longlong lStack_d8;
  undefined1 *puStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined *puStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  char *pcStack_48;
  undefined8 uStack_40;
  
  uStack_40 = 0xfffffffffffffffe;
  pcVar11 = (char *)(param_2 + 0x9b0);
  LOCK();
  bVar12 = *(char *)(param_2 + 0x9b0) == '\0';
  if (bVar12) {
    *(char *)(param_2 + 0x9b0) = '\x01';
  }
  UNLOCK();
  if (!bVar12) {
    FUN_1428cd210(pcVar11,param_2,1000000000);
  }
  pcStack_48 = pcVar11;
  FUN_140c81c70(pcVar11);
  uVar10 = (ulonglong)param_5;
  if (*(ulonglong *)(param_2 + 0x9c8) <= uVar10) {
    FUN_1428d9518(uVar10,*(ulonglong *)(param_2 + 0x9c8),&UNK_142b15c08);
    goto code_r0x000140bef957;
  }
  lVar8 = *(longlong *)(*(longlong *)(param_2 + 0x9c0) + 0x10 + uVar10 * 0x18);
  if (lVar8 == 0) {
code_r0x000140bef58c:
    uStack_148 = CONCAT44(uStack_7c,uStack_80);
    puStack_140 = puStack_78;
    uStack_158 = (undefined4)uStack_90;
    uStack_154 = uStack_90._4_4_;
    uStack_150 = (undefined4)uStack_88;
    uStack_14c = uStack_88._4_4_;
    uStack_168 = uStack_a0;
    uStack_164 = uStack_9c;
    uStack_160 = (undefined4)uStack_98;
    uStack_15c = uStack_98._4_4_;
    uStack_178 = (undefined4)uStack_b0;
    uStack_174 = uStack_b0._4_4_;
    uStack_170 = (undefined4)uStack_a8;
    uStack_16c = uStack_a8._4_4_;
    iStack_188 = (int)uStack_c0;
    uStack_184 = uStack_c0._4_4_;
    uStack_180 = (undefined4)uStack_b8;
    uStack_17c = uStack_b8._4_4_;
    if (4 < *puRam000000014362c3a8) {
      puVar9 = (undefined8 *)func_0x00014067dd80(&UNK_142b15c80);
      uStack_a8 = (undefined1 *)*puVar9;
      uStack_a0 = *(undefined4 *)(puVar9 + 1);
      uStack_9c = *(undefined4 *)((longlong)puVar9 + 0xc);
      uStack_7c = *(undefined4 *)(puVar9 + 2);
      uStack_98 = 5;
      uStack_90 = &UNK_142b15c98;
      uStack_88 = 0x1d;
      puStack_78 = &UNK_142b15c70;
      uStack_70 = 1;
      uStack_68 = 8;
      uStack_c8 = (undefined *)0x0;
      uStack_60 = 0;
      uStack_58 = 0;
      uStack_c0 = &UNK_142b15c98;
      uStack_b8 = 0x1d;
      uStack_b0 = 0;
      uStack_80 = 1;
      func_0x000140d4a330(&uStack_138,&uStack_c8);
    }
    uVar3 = *(ulonglong *)(*(longlong *)(param_2 + 0x658) + 0x40);
    if (uVar3 <= uVar10) {
      FUN_1428d9518(uVar10,uVar3,&UNK_142b15cb8);
      goto code_r0x000140bef957;
    }
    FUN_140bed050(&uStack_c8,param_2,
                  uVar10 * 0x1c + *(longlong *)(*(longlong *)(param_2 + 0x658) + 0x38));
    if ((int)uStack_c8 == 4) {
      uStack_138 = uStack_c0;
      FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_138,&UNK_142b15168,&UNK_142b15cd0);
      goto code_r0x000140bef957;
    }
    uStack_f0 = CONCAT44(uStack_7c,uStack_80);
    lVar8 = *(longlong *)(param_2 + 0x870);
    uStack_108 = (undefined4)uStack_98;
    uStack_104 = uStack_98._4_4_;
    uStack_100 = (undefined4)uStack_90;
    uStack_fc = uStack_90._4_4_;
    uStack_118 = (undefined4)uStack_a8;
    uStack_114 = uStack_a8._4_4_;
    uStack_128 = (undefined4)uStack_b8;
    uStack_124 = uStack_b8._4_4_;
    uStack_120 = (undefined4)uStack_b0;
    uStack_11c = uStack_b0._4_4_;
    iStack_130 = (int)uStack_c0;
    uStack_12c = uStack_c0._4_4_;
    uStack_110 = uStack_a0;
    uStack_10c = uStack_9c;
    uStack_f8 = uStack_88;
    puStack_e8 = puStack_78;
    pcVar11 = pcStack_48;
    uStack_138 = uStack_c8;
  }
  else {
    lVar1 = *(longlong *)(param_2 + 0x9c0) + uVar10 * 0x18;
    lVar8 = lVar8 + -1;
    *(longlong *)(lVar1 + 0x10) = lVar8;
    lVar1 = *(longlong *)(lVar1 + 8);
    lVar8 = lVar8 * 0x58;
    puVar5 = *(undefined **)(lVar1 + lVar8);
    uStack_c0 = (undefined *)((undefined8 *)(lVar1 + lVar8))[1];
    puVar9 = (undefined8 *)(lVar1 + 0x10 + lVar8);
    uStack_b8 = *puVar9;
    uStack_b0 = puVar9[1];
    puVar9 = (undefined8 *)(lVar1 + 0x20 + lVar8);
    uStack_a8 = (undefined1 *)*puVar9;
    uStack_a0 = *(undefined4 *)(puVar9 + 1);
    uStack_9c = *(undefined4 *)((longlong)puVar9 + 0xc);
    puVar9 = (undefined8 *)(lVar1 + 0x30 + lVar8);
    uStack_98 = *puVar9;
    uStack_90 = (undefined *)puVar9[1];
    puStack_78 = *(undefined **)(lVar1 + 0x50 + lVar8);
    puVar9 = (undefined8 *)(lVar1 + 0x40 + lVar8);
    uStack_88 = *puVar9;
    uStack_80 = *(undefined4 *)(puVar9 + 1);
    uStack_7c = *(undefined4 *)((longlong)puVar9 + 0xc);
    uStack_c8 = puVar5;
    if (puVar5 == (undefined *)0x4) goto code_r0x000140bef58c;
    uStack_138 = (undefined *)CONCAT71(uStack_138._1_7_,0x16);
    FUN_1418c1380(0x143634e40,&uStack_138,1);
    uStack_148 = CONCAT44(uStack_7c,uStack_80);
    lVar8 = *(longlong *)(param_2 + 0x870);
    uStack_108 = (undefined4)uStack_98;
    uStack_104 = uStack_98._4_4_;
    uStack_100 = (undefined4)uStack_90;
    uStack_fc = uStack_90._4_4_;
    uStack_118 = (undefined4)uStack_a8;
    uStack_114 = uStack_a8._4_4_;
    uStack_128 = (undefined4)uStack_b8;
    uStack_124 = uStack_b8._4_4_;
    uStack_120 = (undefined4)uStack_b0;
    uStack_11c = uStack_b0._4_4_;
    iStack_130 = (int)uStack_c0;
    uStack_12c = uStack_c0._4_4_;
    iStack_188 = (int)uStack_c0;
    uStack_184 = uStack_c0._4_4_;
    uStack_180 = (undefined4)uStack_b8;
    uStack_17c = uStack_b8._4_4_;
    uStack_178 = (undefined4)uStack_b0;
    uStack_174 = uStack_b0._4_4_;
    uStack_170 = (undefined4)uStack_a8;
    uStack_16c = uStack_a8._4_4_;
    uStack_168 = uStack_a0;
    uStack_164 = uStack_9c;
    uStack_160 = (undefined4)uStack_98;
    uStack_15c = uStack_98._4_4_;
    uStack_158 = (undefined4)uStack_90;
    uStack_154 = uStack_90._4_4_;
    uStack_150 = (undefined4)uStack_88;
    uStack_14c = uStack_88._4_4_;
    puStack_140 = puStack_78;
    uStack_110 = uStack_a0;
    uStack_10c = uStack_9c;
    uStack_f8 = uStack_88;
    uStack_f0 = uStack_148;
    puStack_e8 = puStack_78;
    pcVar11 = pcStack_48;
    uStack_138 = puVar5;
  }
  uStack_a0 = uStack_110;
  uStack_9c = uStack_10c;
  uStack_88 = uStack_f8;
  puStack_78 = puStack_e8;
  pcStack_48 = pcVar11;
  if (lVar8 != 0) {
    FUN_140d797d0(&iStack_188,param_3,param_4);
    if (SBORROW8(0,CONCAT44(uStack_184,iStack_188))) {
      puVar6 = (undefined1 *)CONCAT44(uStack_17c,uStack_180);
      lVar8 = CONCAT44(uStack_174,uStack_178);
      uStack_c8 = (undefined *)CONCAT44(uStack_c8._4_4_,0x3b9cbe00);
      uStack_c0 = (undefined *)0x0;
      uStack_b8 = CONCAT44(uStack_b8._4_4_,6);
      uStack_b0 = uStack_f0;
      uStack_a8 = puVar6;
      iVar7 = (**(code **)(param_2 + 0x870))(*(undefined8 *)(param_2 + 0x8b0),&uStack_c8);
      pcVar11 = pcStack_48;
      if (iVar7 == 0) {
        *puVar6 = 0;
        if (lVar8 != 0) {
          func_0x000140613c20(puVar6,lVar8,1);
        }
        goto code_r0x000140bef818;
      }
      iStack_188 = iVar7;
      lStack_d8 = lVar8;
      puStack_d0 = puVar6;
      FUN_1428d9760(&UNK_142b15850,0x1b,&iStack_188,&UNK_142b15128,&UNK_142b15c38);
    }
    else {
      uStack_b8 = CONCAT44(uStack_174,uStack_178);
      uStack_b0 = CONCAT44(uStack_16c,uStack_170);
      uStack_c8 = (undefined *)CONCAT44(uStack_184,iStack_188);
      uStack_c0 = (undefined *)CONCAT44(uStack_17c,uStack_180);
      FUN_1428d9760(&UNK_142b15188,0x2b,&uStack_c8,&UNK_142b151d8,&UNK_142b15c20);
    }
code_r0x000140bef957:
                    /* WARNING: Does not return */
    pcVar4 = (code *)invalidInstructionException();
    (*pcVar4)();
  }
code_r0x000140bef818:
  *(undefined **)(param_1 + 0x14) = puStack_e8;
  *(undefined8 *)(param_1 + 0x10) = uStack_f8;
  *(undefined8 *)(param_1 + 0x12) = uStack_f0;
  param_1[0xc] = uStack_108;
  param_1[0xd] = uStack_104;
  param_1[0xe] = uStack_100;
  param_1[0xf] = uStack_fc;
  param_1[8] = uStack_118;
  param_1[9] = uStack_114;
  param_1[10] = uStack_110;
  param_1[0xb] = uStack_10c;
  param_1[4] = uStack_128;
  param_1[5] = uStack_124;
  param_1[6] = uStack_120;
  param_1[7] = uStack_11c;
  *param_1 = (undefined4)uStack_138;
  param_1[1] = uStack_138._4_4_;
  param_1[2] = iStack_130;
  param_1[3] = uStack_12c;
  FUN_140c81e10();
  LOCK();
  cVar2 = *pcVar11;
  if (cVar2 == '\x01') {
    *pcVar11 = '\0';
  }
  UNLOCK();
  if (cVar2 != '\x01') {
    FUN_1428cdb90(pcVar11,0);
  }
  return param_1;
}

