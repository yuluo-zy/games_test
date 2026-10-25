
void FUN_1416a2320(longlong param_1,undefined8 param_2,longlong *param_3)

{
  undefined4 *puVar1;
  longlong lVar2;
  undefined8 *puVar3;
  code *pcVar4;
  longlong lVar5;
  undefined8 *puVar6;
  ulonglong uVar7;
  undefined8 *puVar8;
  longlong lVar9;
  undefined4 uStack_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined4 uStack_238;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 uStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  undefined4 uStack_218;
  undefined4 uStack_214;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  longlong lStack_1c8;
  longlong lStack_1c0;
  undefined4 uStack_1b8;
  undefined4 uStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined8 *puStack_1a8;
  undefined8 *puStack_1a0;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
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
  undefined8 *puStack_158;
  undefined1 *puStack_150;
  longlong *plStack_148;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined1 auStack_128 [16];
  undefined8 *puStack_118;
  longlong lStack_110;
  longlong lStack_108;
  longlong lStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 *puStack_e8;
  undefined8 *puStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 *puStack_98;
  undefined1 *puStack_90;
  longlong *plStack_88;
  longlong *plStack_78;
  longlong lStack_70;
  longlong lStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uStack_49 = 1;
  plStack_78 = param_3;
  FUN_140be2a40();
  uStack_49 = 1;
  uVar7 = FUN_140bff400(param_1);
  if ((uVar7 & 1) == 0) {
    uStack_49 = 1;
    FUN_1428d9310(&UNK_142c88848);
                    /* WARNING: Does not return */
    pcVar4 = (code *)invalidInstructionException();
    (*pcVar4)();
  }
  uStack_49 = 1;
  func_0x000140d4a3b0(auStack_128);
  uVar7 = (ulonglong)(plStack_78[1] - *plStack_78) >> 3;
  lStack_1c8 = plStack_78[8];
  lStack_1c0 = plStack_78[9];
  uStack_208 = (undefined4)*plStack_78;
  uStack_204 = *(undefined4 *)((longlong)plStack_78 + 4);
  uStack_200 = (undefined4)plStack_78[1];
  uStack_1fc = *(undefined4 *)((longlong)plStack_78 + 0xc);
  uStack_1f8 = (undefined4)plStack_78[2];
  uStack_1f4 = *(undefined4 *)((longlong)plStack_78 + 0x14);
  uStack_1f0 = (undefined4)plStack_78[3];
  uStack_1ec = *(undefined4 *)((longlong)plStack_78 + 0x1c);
  uStack_1e8 = (undefined4)plStack_78[4];
  uStack_1e4 = *(undefined4 *)((longlong)plStack_78 + 0x24);
  uStack_1e0 = (undefined4)plStack_78[5];
  uStack_1dc = *(undefined4 *)((longlong)plStack_78 + 0x2c);
  uStack_1d8 = (undefined4)plStack_78[6];
  uStack_1d4 = *(undefined4 *)((longlong)plStack_78 + 0x34);
  uStack_1d0 = (undefined4)plStack_78[7];
  uStack_1cc = *(undefined4 *)((longlong)plStack_78 + 0x3c);
  uStack_4a = 1;
  uStack_f0 = param_2;
  FUN_1417c69c0(&lStack_110,uVar7,0,auStack_128);
  uStack_f8 = 0;
  uStack_4c = 1;
  FUN_1417c6af0(&lStack_70,uVar7,0,auStack_128);
  puStack_1a8 = (undefined8 *)*plStack_78;
  puStack_1a0 = (undefined8 *)plStack_78[1];
  uStack_198 = (undefined4)plStack_78[2];
  uStack_194 = *(undefined4 *)((longlong)plStack_78 + 0x14);
  uStack_190 = (undefined4)plStack_78[3];
  uStack_18c = *(undefined4 *)((longlong)plStack_78 + 0x1c);
  uStack_188 = (undefined4)plStack_78[4];
  uStack_184 = *(undefined4 *)((longlong)plStack_78 + 0x24);
  uStack_180 = (undefined4)plStack_78[5];
  uStack_17c = *(undefined4 *)((longlong)plStack_78 + 0x2c);
  uStack_178 = (undefined4)plStack_78[6];
  uStack_174 = *(undefined4 *)((longlong)plStack_78 + 0x34);
  uStack_170 = (undefined4)plStack_78[7];
  uStack_16c = *(undefined4 *)((longlong)plStack_78 + 0x3c);
  uStack_168 = (undefined4)plStack_78[8];
  uStack_164 = *(undefined4 *)((longlong)plStack_78 + 0x44);
  uStack_160 = (undefined4)plStack_78[9];
  uStack_15c = *(undefined4 *)((longlong)plStack_78 + 0x4c);
  puStack_158 = &uStack_f0;
  uStack_58 = 0;
  uVar7 = (ulonglong)((longlong)puStack_1a0 - (longlong)puStack_1a8) >> 3;
  puStack_150 = auStack_128;
  plStack_148 = &lStack_110;
  if (uStack_60 < uVar7) {
    uStack_4b = 1;
    FUN_1417c6c10(&lStack_70,0,uVar7,1);
  }
  plStack_88 = plStack_148;
  puStack_98 = puStack_158;
  puStack_90 = puStack_150;
  uStack_a8 = CONCAT44(uStack_164,uStack_168);
  uStack_a0 = CONCAT44(uStack_15c,uStack_160);
  uStack_b8 = uStack_178;
  uStack_b4 = uStack_174;
  uStack_b0 = uStack_170;
  uStack_ac = uStack_16c;
  uStack_c8 = uStack_188;
  uStack_c4 = uStack_184;
  uStack_c0 = uStack_180;
  uStack_bc = uStack_17c;
  uStack_d8 = uStack_198;
  uStack_d4 = uStack_194;
  uStack_d0 = uStack_190;
  uStack_cc = uStack_18c;
  puStack_e0 = puStack_1a0;
  if (puStack_1a8 != puStack_1a0) {
    puVar8 = puStack_1a8;
    do {
      puStack_e8 = puVar8 + 1;
      FUN_140a96740(&uStack_138,&uStack_c0,*puVar8);
      if (CONCAT44(uStack_12c,uStack_130) == 0) {
        lVar9 = CONCAT44(uStack_c4,uStack_c8);
        goto joined_r0x0001416a2647;
      }
      uStack_1b8 = uStack_138;
      uStack_1b4 = uStack_134;
      uStack_1b0 = uStack_130;
      uStack_1ac = uStack_12c;
      FUN_140b06020(&uStack_248,&puStack_98,&uStack_1b8);
      if (uStack_58 == uStack_60) {
        FUN_1417c6c10(&lStack_70,uStack_58,1,1);
      }
      lVar9 = uStack_58 * 0x40;
      puVar1 = (undefined4 *)(lStack_70 + 0x30 + lVar9);
      *puVar1 = uStack_218;
      puVar1[1] = uStack_214;
      puVar1[2] = uStack_210;
      puVar1[3] = uStack_20c;
      puVar1 = (undefined4 *)(lStack_70 + 0x20 + lVar9);
      *puVar1 = uStack_228;
      puVar1[1] = uStack_224;
      puVar1[2] = uStack_220;
      puVar1[3] = uStack_21c;
      puVar1 = (undefined4 *)(lStack_70 + 0x10 + lVar9);
      *puVar1 = uStack_238;
      puVar1[1] = uStack_234;
      puVar1[2] = uStack_230;
      puVar1[3] = uStack_22c;
      puVar1 = (undefined4 *)(lStack_70 + lVar9);
      *puVar1 = uStack_248;
      puVar1[1] = uStack_244;
      puVar1[2] = uStack_240;
      puVar1[3] = uStack_23c;
      uStack_58 = uStack_58 + 1;
      puVar8 = puStack_e8;
    } while (puStack_e8 != puStack_e0);
  }
  puStack_e8 = (undefined8 *)0x8;
  puStack_e0 = (undefined8 *)0x8;
  lVar9 = CONCAT44(uStack_c4,uStack_c8);
joined_r0x0001416a2647:
  if (lVar9 != 0) {
    lVar5 = CONCAT44(uStack_d4,uStack_d8);
    lVar2 = *(longlong *)(lVar5 + 0x10);
    if (CONCAT44(uStack_cc,uStack_d0) != lVar2) {
      func_0x000142923c10(*(longlong *)(lVar5 + 8) + lVar2 * 8,
                          *(longlong *)(lVar5 + 8) + CONCAT44(uStack_cc,uStack_d0) * 8,lVar9 * 8);
    }
    *(longlong *)(lVar5 + 0x10) = lVar9 + lVar2;
  }
  (**(code **)(param_1 + 0x268))
            (*(undefined8 *)(param_1 + 0x640),uStack_58 & 0xffffffff,lStack_70,0,8);
  LOCK();
  *(longlong *)(param_1 + 0xab0) = *(longlong *)(param_1 + 0xab0) + 1;
  UNLOCK();
  if (uStack_60 != 0) {
    lVar9 = *(longlong *)(*(longlong *)(lStack_68 + 0x10) + 0x20);
    if (lVar9 == lStack_70) {
      *(ulonglong *)(*(longlong *)(lStack_68 + 0x10) + 0x20) = lVar9 + uStack_60 * 0x40;
    }
  }
  puVar8 = puRam000000014362c3e0;
  if (lStack_100 != 0) {
    lVar9 = *(longlong *)(*(longlong *)(lStack_108 + 0x10) + 0x20);
    if (lVar9 == lStack_110) {
      *(longlong *)(*(longlong *)(lStack_108 + 0x10) + 0x20) = lVar9 + lStack_100 * 4;
      puVar8 = puRam000000014362c3e0;
    }
  }
  while (puVar6 = puRam000000014362c3e0, puStack_118 != puRam000000014362c3e0) {
    puVar3 = (undefined8 *)puStack_118[3];
    puRam000000014362c3e0 = puVar8;
    func_0x000140613c20(*puStack_118,puStack_118[2],puStack_118[1]);
    puVar8 = puRam000000014362c3e0;
    puStack_118 = puVar3;
    puRam000000014362c3e0 = puVar6;
  }
  puRam000000014362c3e0 = puVar8;
  return;
}

