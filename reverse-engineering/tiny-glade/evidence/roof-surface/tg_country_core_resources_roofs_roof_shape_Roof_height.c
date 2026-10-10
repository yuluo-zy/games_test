
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_1408e3260(longlong param_1)

{
  code *pcVar1;
  ulonglong uVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  
  fVar5 = *(float *)(param_1 + 0x30);
  uVar2 = 0;
  if (_DAT_142925984 < fVar5) {
    if ((*(byte *)(param_1 + 8) & 1) == 0) {
      fVar3 = (*(float *)(param_1 + 0x14) + _DAT_142aa508c) / _DAT_142aa5090;
      fVar4 = _DAT_142934604;
    }
    else {
      fVar4 = *(float *)(param_1 + 0x1c);
      fVar3 = *(float *)(param_1 + 0x20);
      fVar6 = fVar3;
      if (fVar3 <= fVar4) {
        fVar6 = fVar4;
      }
      fVar3 = ((float)(~-(uint)NAN(fVar4) & (uint)fVar6 | -(uint)NAN(fVar4) & (uint)fVar3) +
              _DAT_142aa5084) / _DAT_142aa5088;
      fVar4 = _DAT_142a7ba84;
    }
    fVar6 = 0.0;
    if (0.0 <= fVar3 / _DAT_142925970) {
      fVar6 = fVar3 / _DAT_142925970;
    }
    fVar3 = _DAT_142925904;
    if (fVar6 <= _DAT_142925904) {
      fVar3 = fVar6;
    }
    fVar3 = (_DAT_1429cd078 - (fVar3 + fVar3)) * fVar3 * fVar3;
    fVar4 = fVar3 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar3) * fVar4;
    uVar2 = (ulonglong)(uint)fVar4;
    if (fVar4 < 0.0) {
      FUN_140d8b480(0,uVar2,&UNK_142aa5190);
      pcVar1 = (code *)swi(3);
      uVar2 = (*pcVar1)();
      return uVar2;
    }
    fVar3 = fVar5 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar5) * _DAT_142925970;
    fVar5 = 0.0;
    if (0.0 <= fVar3) {
      fVar5 = fVar3;
    }
    if (fVar5 <= fVar4) {
      uVar2 = (ulonglong)(uint)fVar5;
    }
  }
  return uVar2;
}

