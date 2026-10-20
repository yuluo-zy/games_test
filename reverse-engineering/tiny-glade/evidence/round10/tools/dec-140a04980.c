
undefined8 FUN_140a04980(undefined8 param_1)

{
  code *pcVar1;
  longlong lVar2;
  undefined8 uStack_2b8;
  undefined8 uStack_2b0;
  undefined8 uStack_2a8;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined8 uStack_288;
  undefined8 uStack_280;
  undefined8 uStack_278;
  undefined8 uStack_270;
  undefined8 uStack_268;
  undefined8 uStack_260;
  undefined8 uStack_258;
  undefined8 uStack_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined4 uStack_228;
  undefined8 uStack_220;
  undefined *puStack_218;
  undefined8 uStack_210;
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
  undefined4 uStack_190;
  undefined4 uStack_188;
  undefined2 uStack_184;
  undefined8 uStack_88;
  undefined4 uStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  uStack_298 = 0;
  uStack_290 = 0;
  uStack_280 = 0;
  uStack_278 = 0;
  uStack_268 = 0;
  uStack_260 = 0;
  uStack_250 = 0;
  uStack_248 = 0;
  uStack_238 = 0;
  uStack_230 = 0;
  uStack_228 = 0;
  uStack_200 = 0;
  uStack_1f8 = 0;
  uStack_1e8 = 0;
  uStack_1e0 = 0;
  uStack_1d0 = 0;
  uStack_1c8 = 0;
  uStack_1b8 = 0;
  uStack_1b0 = 0;
  uStack_1a0 = 0;
  uStack_198 = 0;
  uStack_190 = 0;
  uStack_2b8 = 0;
  uStack_2b0 = 8;
  uStack_2a8 = 0;
  uStack_2a0 = 0x10;
  uStack_288 = 0x10;
  uStack_270 = 0x10;
  uStack_258 = 0x10;
  uStack_240 = 0x10;
  uStack_220 = 0x8000000000000000;
  puStack_218 = &UNK_142ac6471;
  uStack_210 = 0x36;
  uStack_208 = 0x10;
  uStack_1f0 = 0x10;
  uStack_1d8 = 0x10;
  uStack_1c0 = 0x10;
  uStack_1a8 = 0x10;
  uStack_188 = 0;
  uStack_184 = 1;
  uStack_88 = 0x8000000000000000;
  uStack_28 = 0;
  lVar2 = func_0x000140613c10(0x298,8);
  if (lVar2 != 0) {
    func_0x000142923c00(lVar2,&uStack_2b8,0x298);
    FUN_140ce4b10(param_1,lVar2,&UNK_142ac6160);
    return param_1;
  }
  FUN_1428d9000(8,0x298);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

