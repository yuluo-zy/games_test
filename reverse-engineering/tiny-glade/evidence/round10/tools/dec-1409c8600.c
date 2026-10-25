
undefined8 FUN_1409c8600(undefined8 param_1)

{
  code *pcVar1;
  longlong lVar2;
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
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined8 uStack_168;
  undefined8 uStack_160;
  undefined8 uStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined8 uStack_138;
  undefined *puStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
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
  undefined4 uStack_a8;
  undefined4 uStack_a0;
  undefined2 uStack_9c;
  undefined8 uStack_68;
  undefined4 uStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  uStack_1b0 = 0;
  uStack_1a8 = 0;
  uStack_198 = 0;
  uStack_190 = 0;
  uStack_180 = 0;
  uStack_178 = 0;
  uStack_168 = 0;
  uStack_160 = 0;
  uStack_150 = 0;
  uStack_148 = 0;
  uStack_140 = 0;
  uStack_118 = 0;
  uStack_110 = 0;
  uStack_100 = 0;
  uStack_f8 = 0;
  uStack_e8 = 0;
  uStack_e0 = 0;
  uStack_d0 = 0;
  uStack_c8 = 0;
  uStack_b8 = 0;
  uStack_b0 = 0;
  uStack_a8 = 0;
  uStack_1d0 = 0;
  uStack_1c8 = 8;
  uStack_1c0 = 0;
  uStack_1b8 = 0x10;
  uStack_1a0 = 0x10;
  uStack_188 = 0x10;
  uStack_170 = 0x10;
  uStack_158 = 0x10;
  uStack_138 = 0x8000000000000000;
  puStack_130 = &UNK_142ac0985;
  uStack_128 = 0x31;
  uStack_120 = 0x10;
  uStack_108 = 0x10;
  uStack_f0 = 0x10;
  uStack_d8 = 0x10;
  uStack_c0 = 0x10;
  uStack_a0 = 0;
  uStack_9c = 1;
  uStack_68 = 0x8000000000000000;
  uStack_28 = 0;
  lVar2 = func_0x000140613c10(0x1b0,8);
  if (lVar2 != 0) {
    func_0x000142923c00(lVar2,&uStack_1d0,0x1b0);
    FUN_140ce4b10(param_1,lVar2,&UNK_142ac0538);
    return param_1;
  }
  FUN_1428d9000(8,0x1b0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

