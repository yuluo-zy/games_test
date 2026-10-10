
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_1408e30f0(float param_1,char param_2,float param_3,float param_4)

{
  code *pcVar1;
  float fVar2;
  ulonglong uVar3;
  float fVar4;
  
  if (param_2 == '\0') {
    param_4 = param_3;
  }
  if ((param_4 != _DAT_142925984) || (NAN(param_4) || NAN(_DAT_142925984))) {
    fVar2 = (param_1 + _DAT_1429eb6f8) / (param_4 + _DAT_1429eb6f8);
    fVar4 = 0.0;
    if (0.0 <= fVar2) {
      fVar4 = fVar2;
    }
    if ((0x7f7fffff < (uint)ABS(fVar2) && 0.0 <= fVar2) && fVar4 <= _DAT_142925904) {
      FUN_1428d9430(&UNK_142aa5848,0x29,&UNK_142aa5878);
    }
    else {
      fVar2 = _DAT_142925904;
      if (fVar4 <= _DAT_142925904) {
        fVar2 = fVar4;
      }
      if (!NAN(fVar2)) {
        return (ulonglong)(uint)fVar2;
      }
    }
    FUN_1428d9430(&UNK_142aa5890,0x27,&UNK_142aa58b8);
  }
  FUN_1428d9310(&UNK_142aa5830);
  pcVar1 = (code *)swi(3);
  uVar3 = (*pcVar1)();
  return uVar3;
}

