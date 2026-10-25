utils::geometry::rectangle::Rectangle2d::get_point

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140c99600(undefined8 *param_1,ulonglong param_2)

{
  code *pcVar1;
  float fVar2;
  float fVar4;
  ulonglong uVar3;
  float afStack_20 [4];
  float fStack_10;
  undefined8 uStack_c;
  float fStack_4;
  
  if (param_2 < 5) {
    fVar2 = (float)param_1[2];
    fVar4 = (float)((ulonglong)param_1[2] >> 0x20);
    if (0.0 < fVar4 * fVar2) {
      uVar3 = 0;
      if (param_2 != 4) {
        uVar3 = param_2;
      }
      afStack_20[0] = fVar2 * _UNK_142aa5d64;
      afStack_20[1] = fVar4 * _DAT_142aa5d60;
      afStack_20[2] = fVar2 * _UNK_142aa5d64;
      afStack_20[3] = fVar4 * _UNK_142aa5d68;
      fStack_10 = fVar2 * _UNK_142aa5d6c;
      uStack_c = CONCAT44(fVar2 * _UNK_142aa5d6c,fVar4 * _UNK_142aa5d68);
      fStack_4 = fVar4 * _DAT_142aa5d60;
      return (ulonglong)
             (uint)(*(float *)(param_1 + 1) +
                   (float)((ulonglong)*(undefined8 *)(afStack_20 + uVar3 * 2) >> 0x20) *
                   (float)((uint)((ulonglong)*param_1 >> 0x20) ^ _DAT_1429258f0) +
                   (float)*(undefined8 *)(afStack_20 + uVar3 * 2) * (float)*param_1);
    }
  }
  else {
    FUN_1428d9430(&UNK_142b2e7c0,0x18,&UNK_142b2e7d8);
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6b8);
  pcVar1 = (code *)swi(3);
  uVar3 = (*pcVar1)();
  return uVar3;
}

