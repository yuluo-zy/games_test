
undefined8 FUN_14179fa20(undefined8 param_1)

{
  code *pcVar1;
  longlong lVar2;
  undefined8 auStack_190 [6];
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
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined4 uStack_d0;
  undefined8 uStack_c8;
  undefined *puStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
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
  undefined4 uStack_38;
  undefined4 uStack_30;
  undefined2 uStack_2c;
  undefined4 uStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  uStack_140 = 0;
  uStack_138 = 0;
  uStack_128 = 0;
  uStack_120 = 0;
  uStack_110 = 0;
  uStack_108 = 0;
  uStack_f8 = 0;
  uStack_f0 = 0;
  uStack_e0 = 0;
  uStack_d8 = 0;
  uStack_d0 = 0;
  uStack_a8 = 0;
  uStack_a0 = 0;
  uStack_90 = 0;
  uStack_88 = 0;
  uStack_78 = 0;
  uStack_70 = 0;
  uStack_60 = 0;
  uStack_58 = 0;
  uStack_48 = 0;
  uStack_40 = 0;
  uStack_38 = 0;
  auStack_190[0] = 0;
  uStack_160 = 0;
  uStack_158 = 8;
  uStack_150 = 0;
  uStack_148 = 0x10;
  uStack_130 = 0x10;
  uStack_118 = 0x10;
  uStack_100 = 0x10;
  uStack_e8 = 0x10;
  uStack_c8 = 0x8000000000000000;
  puStack_c0 = &UNK_142cdc046;
  uStack_b8 = 0x31;
  uStack_b0 = 0x10;
  uStack_98 = 0x10;
  uStack_80 = 0x10;
  uStack_68 = 0x10;
  uStack_50 = 0x10;
  uStack_30 = 0;
  uStack_2c = 1;
  uStack_28 = 0;
  lVar2 = func_0x000140613c10(0x170,8);
  if (lVar2 != 0) {
    func_0x000142923c00(lVar2,auStack_190,0x170);
    FUN_140ce4b10(param_1,lVar2,&UNK_142cdbe98);
    return param_1;
  }
  FUN_1428d9000(8,0x170);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

