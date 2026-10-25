
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140f801d0(undefined8 *param_1,undefined4 *param_2,undefined1 param_3)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined1 *puVar3;
  undefined1 uStack_188;
  undefined7 uStack_187;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 *puStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  puStack_30 = (undefined8 *)
               func_0x000140613c10(0x150,8,param_2[8],param_2[0xc],*param_2,param_2[2],param_2[4],
                                   param_2[6],param_2[8],param_2[10],param_2[0xc],param_2[0xe],
                                   *(undefined8 *)(param_2 + 0x10),*(undefined8 *)(param_2 + 0x12),
                                   *(undefined8 *)(param_2 + 0x14),*(undefined8 *)(param_2 + 0x16),
                                   *(undefined8 *)(param_2 + 0x18),*(undefined8 *)(param_2 + 0x1a));
  if (puStack_30 != (undefined8 *)0x0) {
    uStack_b8 = *(undefined8 *)(param_2 + 0x18);
    uStack_b0 = *(undefined8 *)(param_2 + 0x1a);
    uStack_c8 = *(undefined8 *)(param_2 + 0x14);
    uStack_c0 = *(undefined8 *)(param_2 + 0x16);
    uStack_d8 = *(undefined8 *)(param_2 + 0x10);
    uStack_d0 = *(undefined8 *)(param_2 + 0x12);
    uStack_118 = *param_2;
    uStack_114 = param_2[1];
    uStack_110 = param_2[2];
    uStack_10c = param_2[3];
    uStack_108 = param_2[4];
    uStack_104 = param_2[5];
    uStack_100 = param_2[6];
    uStack_fc = param_2[7];
    uStack_f8 = param_2[8];
    uStack_f4 = param_2[9];
    uStack_f0 = param_2[10];
    uStack_ec = param_2[0xb];
    uStack_e8 = param_2[0xc];
    uStack_e4 = param_2[0xd];
    uStack_e0 = param_2[0xe];
    uStack_dc = param_2[0xf];
    FUN_141db2d10(&uStack_188);
    FUN_141db5920(&uStack_a0);
    puStack_30[0xc] = uStack_b8;
    puStack_30[0xd] = uStack_b0;
    puStack_30[10] = uStack_c8;
    puStack_30[0xb] = uStack_c0;
    puStack_30[8] = uStack_d8;
    puStack_30[9] = uStack_d0;
    puStack_30[6] = CONCAT44(uStack_e4,uStack_e8);
    puStack_30[7] = CONCAT44(uStack_dc,uStack_e0);
    puStack_30[4] = CONCAT44(uStack_f4,uStack_f8);
    puStack_30[5] = CONCAT44(uStack_ec,uStack_f0);
    puStack_30[2] = CONCAT44(uStack_104,uStack_108);
    puStack_30[3] = CONCAT44(uStack_fc,uStack_100);
    *puStack_30 = CONCAT44(uStack_114,uStack_118);
    puStack_30[1] = CONCAT44(uStack_10c,uStack_110);
    puStack_30[0xe] = CONCAT71(uStack_187,uStack_188);
    puStack_30[0xf] = uStack_180;
    puStack_30[0x10] = uStack_178;
    puStack_30[0x11] = uStack_170;
    puStack_30[0x12] = uStack_168;
    puStack_30[0x13] = uStack_160;
    puStack_30[0x14] = uStack_158;
    puStack_30[0x15] = uStack_150;
    puStack_30[0x16] = uStack_148;
    puStack_30[0x17] = uStack_140;
    puStack_30[0x18] = uStack_138;
    puStack_30[0x19] = uStack_130;
    puStack_30[0x1a] = uStack_128;
    puStack_30[0x1b] = uStack_120;
    puStack_30[0x1c] = uStack_a0;
    puStack_30[0x1d] = uStack_98;
    *(undefined4 *)(puStack_30 + 0x1e) = (undefined4)uStack_90;
    *(undefined4 *)((longlong)puStack_30 + 0xf4) = uStack_90._4_4_;
    *(undefined4 *)(puStack_30 + 0x1f) = (undefined4)uStack_88;
    *(undefined4 *)((longlong)puStack_30 + 0xfc) = uStack_88._4_4_;
    *(undefined4 *)(puStack_30 + 0x20) = (undefined4)uStack_80;
    *(undefined4 *)((longlong)puStack_30 + 0x104) = uStack_80._4_4_;
    *(undefined4 *)(puStack_30 + 0x21) = (undefined4)uStack_78;
    *(undefined4 *)((longlong)puStack_30 + 0x10c) = uStack_78._4_4_;
    *(undefined4 *)(puStack_30 + 0x22) = (undefined4)uStack_70;
    *(undefined4 *)((longlong)puStack_30 + 0x114) = uStack_70._4_4_;
    *(undefined4 *)(puStack_30 + 0x23) = (undefined4)uStack_68;
    *(undefined4 *)((longlong)puStack_30 + 0x11c) = uStack_68._4_4_;
    puStack_30[0x24] = uStack_60;
    puStack_30[0x25] = uStack_58;
    puStack_30[0x26] = uStack_50;
    puStack_30[0x27] = uStack_48;
    puStack_30[0x28] = uStack_40;
    puStack_30[0x29] = uStack_38;
    uStack_98 = 3;
    uStack_90 = puStack_30;
    uStack_88 = 3;
    uStack_80 = 0;
    uStack_78 = 8;
    uStack_70 = 0;
    uStack_68 = 0;
    uStack_a0 = 0x8000000000000000;
    puVar3 = &uStack_188;
    uStack_188 = param_3;
    uVar2 = FUN_141d926f0(_DAT_14362c370,puVar3,&UNK_142f6b940);
    FUN_140f80590(&uStack_a0,uVar2,puVar3);
    param_1[0xc] = uStack_40;
    param_1[0xd] = uStack_38;
    param_1[10] = uStack_50;
    param_1[0xb] = uStack_48;
    param_1[8] = uStack_60;
    param_1[9] = uStack_58;
    param_1[6] = uStack_70;
    param_1[7] = uStack_68;
    param_1[4] = uStack_80;
    param_1[5] = uStack_78;
    param_1[2] = uStack_90;
    param_1[3] = uStack_88;
    *param_1 = uStack_a0;
    param_1[1] = uStack_98;
    return param_1;
  }
  FUN_1428d9000(8,0x150);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

