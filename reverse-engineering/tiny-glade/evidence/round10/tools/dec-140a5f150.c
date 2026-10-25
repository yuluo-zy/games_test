
undefined8 FUN_140a5f150(undefined8 param_1)

{
  code *pcVar1;
  longlong lVar2;
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
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined4 uStack_110;
  undefined8 uStack_108;
  undefined *puStack_100;
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
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined4 uStack_78;
  undefined4 uStack_70;
  undefined2 uStack_6c;
  undefined8 uStack_68;
  undefined4 uStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  uStack_180 = 0;
  uStack_178 = 0;
  uStack_168 = 0;
  uStack_160 = 0;
  uStack_150 = 0;
  uStack_148 = 0;
  uStack_138 = 0;
  uStack_130 = 0;
  uStack_120 = 0;
  uStack_118 = 0;
  uStack_110 = 0;
  uStack_e8 = 0;
  uStack_e0 = 0;
  uStack_d0 = 0;
  uStack_c8 = 0;
  uStack_b8 = 0;
  uStack_b0 = 0;
  uStack_a0 = 0;
  uStack_98 = 0;
  uStack_88 = 0;
  uStack_80 = 0;
  uStack_78 = 0;
  uStack_1a0 = 0;
  uStack_198 = 8;
  uStack_190 = 0;
  uStack_188 = 0x10;
  uStack_170 = 0x10;
  uStack_158 = 0x10;
  uStack_140 = 0x10;
  uStack_128 = 0x10;
  uStack_108 = 0x8000000000000000;
  puStack_100 = &UNK_142ad3db0;
  uStack_f8 = 0x2d;
  uStack_f0 = 0x10;
  uStack_d8 = 0x10;
  uStack_c0 = 0x10;
  uStack_a8 = 0x10;
  uStack_90 = 0x10;
  uStack_70 = 0;
  uStack_6c = 1;
  uStack_68 = 0x8000000000000000;
  uStack_28 = 0;
  lVar2 = func_0x000140613c10(0x180,8);
  if (lVar2 != 0) {
    func_0x000142923c00(lVar2,&uStack_1a0,0x180);
    FUN_140ce4b10(param_1,lVar2,&UNK_142ad3970);
    return param_1;
  }
  FUN_1428d9000(8,0x180);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

