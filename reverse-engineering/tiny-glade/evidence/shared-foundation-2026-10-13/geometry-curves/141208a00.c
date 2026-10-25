
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141208a00(float param_1,undefined8 param_2)

{
  code *pcVar1;
  float fStack_44;
  float *pfStack_40;
  code *pcStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  undefined4 **ppuStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  fStack_44 = param_1;
  if ((param_1 < 0.0) || (_DAT_142925904 < param_1)) {
    pfStack_40 = &fStack_44;
    pcStack_38 = FUN_140d8c910;
    puStack_30 = &UNK_142c1a5d8;
    uStack_28 = 1;
    uStack_10 = 0;
    ppuStack_20 = &pfStack_40;
    uStack_18 = 1;
    FUN_1428d9390(&puStack_30,param_2);
  }
  else if (!NAN(param_1)) {
    return;
  }
  FUN_1428d9430(&UNK_142c1a5e8,0x1d);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

