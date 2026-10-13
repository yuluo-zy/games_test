
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_1411fb2b0(ulonglong param_1,float param_2,ulonglong param_3)

{
  code *pcVar1;
  ulonglong uVar2;
  float fStack_64;
  ulonglong uStack_60;
  ulonglong uStack_58;
  undefined *puStack_50;
  undefined8 uStack_48;
  undefined8 **ppuStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  ulonglong *puStack_20;
  code *pcStack_18;
  ulonglong *puStack_10;
  code *pcStack_8;
  
  fStack_64 = param_2;
  uStack_60 = param_1;
  uStack_58 = param_3;
  if ((param_2 < 0.0) || (_DAT_142925904 < param_2)) {
    FUN_1428f15b0(&fStack_64,&UNK_142c192c8);
LAB_1411fb32e:
    puStack_20 = &uStack_60;
    pcStack_18 = FUN_140d8d6c0;
    puStack_10 = &uStack_58;
    pcStack_8 = FUN_140d8d6c0;
    puStack_50 = &UNK_142c192f8;
    uStack_48 = 2;
    uStack_30 = 0;
    ppuStack_40 = &puStack_20;
    uStack_38 = 2;
    FUN_1428d9390(&puStack_50,&UNK_142c19318);
  }
  else {
    if (param_3 <= param_1) goto LAB_1411fb32e;
    if (param_1 + 1 < param_3) {
      return param_1 & 0xffffffff;
    }
    if ((param_2 == 0.0) && (!NAN(param_2))) {
      if (param_1 != 0) {
        return param_1 - 1 & 0xffffffff;
      }
      goto LAB_1411fb3cb;
    }
  }
  puStack_50 = (undefined *)0x0;
  FUN_1428ce04f(0,&fStack_64,&UNK_142c18d00,&puStack_50,&UNK_142c19330);
LAB_1411fb3cb:
  FUN_1428d9310(&UNK_142c19348);
  pcVar1 = (code *)swi(3);
  uVar2 = (*pcVar1)();
  return uVar2;
}

