
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_140c909a0(uint param_1,uint param_2)

{
  code *pcVar1;
  float fVar2;
  ulonglong uVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  
  if (param_1 == 0) {
    return 0;
  }
  if (param_2 != 0) {
    fVar4 = _DAT_142925904 / (float)param_2;
    fVar2 = 0.0;
    fVar5 = fVar4;
    do {
      uVar3 = (ulonglong)param_1;
      fVar6 = (float)param_1 * fVar4;
      param_1 = (uint)(longlong)fVar6;
      if (fVar6 < 0.0) {
        param_1 = 0;
      }
      if (_DAT_1429a07d0 < fVar6) {
        param_1 = 0xffffffff;
      }
      fVar2 = fVar2 + (float)(uVar3 % (ulonglong)param_2) * fVar5;
      fVar5 = fVar5 * fVar4;
    } while (param_1 != 0);
    return (ulonglong)(uint)fVar2;
  }
  FUN_1428d9a20(&UNK_142b2d8b8);
  pcVar1 = (code *)swi(3);
  uVar3 = (*pcVar1)();
  return uVar3;
}

