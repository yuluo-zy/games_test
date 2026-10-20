
undefined8 FUN_141740a20(undefined8 param_1)

{
  code *pcVar1;
  longlong lVar2;
  undefined1 auStack_278 [112];
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined8 uStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined4 uStack_178;
  undefined8 uStack_170;
  undefined *puStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_d8;
  undefined2 uStack_d4;
  undefined8 uStack_68;
  undefined4 uStack_30;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  uStack_1e8 = 0;
  uStack_1e0 = 0;
  uStack_1d0 = 0;
  uStack_1c8 = 0;
  uStack_1b8 = 0;
  uStack_1b0 = 0;
  uStack_1a0 = 0;
  uStack_198 = 0;
  uStack_188 = 0;
  uStack_180 = 0;
  uStack_178 = 0;
  uStack_150 = 0;
  uStack_148 = 0;
  uStack_138 = 0;
  uStack_130 = 0;
  uStack_120 = 0;
  uStack_118 = 0;
  uStack_108 = 0;
  uStack_100 = 0;
  uStack_f0 = 0;
  uStack_e8 = 0;
  uStack_e0 = 0;
  uStack_208 = 0;
  uStack_200 = 8;
  uStack_1f8 = 0;
  uStack_1f0 = 0x10;
  uStack_1d8 = 0x10;
  uStack_1c0 = 0x10;
  uStack_1a8 = 0x10;
  uStack_190 = 0x10;
  uStack_170 = 0x8000000000000000;
  puStack_168 = &UNK_142ccfb99;
  uStack_160 = 0x39;
  uStack_158 = 0x10;
  uStack_140 = 0x10;
  uStack_128 = 0x10;
  uStack_110 = 0x10;
  uStack_f8 = 0x10;
  uStack_d8 = 0;
  uStack_d4 = 1;
  uStack_68 = 0x8000000000000000;
  uStack_30 = 0;
  lVar2 = func_0x000140613c10(0x1e0,8);
  if (lVar2 != 0) {
    func_0x000142923c00(lVar2,&uStack_208,0x1e0);
    FUN_140ce4b10(auStack_278,lVar2,&UNK_142ccf9d0);
    FUN_14144f1f0(param_1,auStack_278);
    return param_1;
  }
  FUN_1428d9000(8,0x1e0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

