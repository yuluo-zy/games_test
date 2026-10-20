
undefined8 FUN_1408c6e60(undefined8 param_1)

{
  code *pcVar1;
  longlong lVar2;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
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
  undefined4 uStack_120;
  undefined8 uStack_118;
  undefined *puStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined4 uStack_88;
  undefined4 uStack_80;
  undefined2 uStack_7c;
  undefined8 uStack_68;
  undefined4 uStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  uStack_190 = 0;
  uStack_188 = 0;
  uStack_178 = 0;
  uStack_170 = 0;
  uStack_160 = 0;
  uStack_158 = 0;
  uStack_148 = 0;
  uStack_140 = 0;
  uStack_130 = 0;
  uStack_128 = 0;
  uStack_120 = 0;
  uStack_f8 = 0;
  uStack_f0 = 0;
  uStack_e0 = 0;
  uStack_d8 = 0;
  uStack_c8 = 0;
  uStack_c0 = 0;
  uStack_b0 = 0;
  uStack_a8 = 0;
  uStack_98 = 0;
  uStack_90 = 0;
  uStack_88 = 0;
  uStack_1b0 = 0;
  uStack_1a8 = 8;
  uStack_1a0 = 0;
  uStack_198 = 0x10;
  uStack_180 = 0x10;
  uStack_168 = 0x10;
  uStack_150 = 0x10;
  uStack_138 = 0x10;
  uStack_118 = 0x8000000000000000;
  puStack_110 = &UNK_142aa2ded;
  uStack_108 = 0x38;
  uStack_100 = 0x10;
  uStack_e8 = 0x10;
  uStack_d0 = 0x10;
  uStack_b8 = 0x10;
  uStack_a0 = 0x10;
  uStack_80 = 0;
  uStack_7c = 1;
  uStack_68 = 0x8000000000000000;
  uStack_28 = 0;
  lVar2 = func_0x000140613c10(400,8);
  if (lVar2 != 0) {
    func_0x000142923c00(lVar2,&uStack_1b0,400);
    FUN_140ce4b10(param_1,lVar2,&UNK_142aa2b80);
    return param_1;
  }
  FUN_1428d9000(8,400);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

