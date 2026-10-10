
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1408e2eb0(undefined8 *param_1,longlong param_2,undefined4 param_3,undefined4 param_4)

{
  float fVar1;
  float fVar2;
  float fVar3;
  code *pcVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined1 auStack_98 [8];
  float fStack_90;
  
  fVar1 = *(float *)(param_2 + 0xc + (ulonglong)*(uint *)(param_2 + 8) * 8);
  fVar2 = *(float *)(param_2 + 0x10 + (ulonglong)*(uint *)(param_2 + 8) * 8);
  fVar3 = *(float *)(param_2 + 0x4c);
  uStack_a0 = param_3;
  uStack_9c = param_4;
  tg_x0y(auStack_98,&uStack_a0);
  fVar7 = *(float *)(param_2 + 0x30);
  fVar6 = 0.0;
  if (_DAT_142925984 < fVar7) {
    if ((*(byte *)(param_2 + 8) & 1) == 0) {
      fVar5 = (*(float *)(param_2 + 0x14) + _DAT_142aa508c) / _DAT_142aa5090;
      fVar6 = _DAT_142934604;
    }
    else {
      fVar6 = *(float *)(param_2 + 0x1c);
      fVar5 = *(float *)(param_2 + 0x20);
      fVar8 = fVar5;
      if (fVar5 <= fVar6) {
        fVar8 = fVar6;
      }
      fVar5 = ((float)(~-(uint)NAN(fVar6) & (uint)fVar8 | -(uint)NAN(fVar6) & (uint)fVar5) +
              _DAT_142aa5084) / _DAT_142aa5088;
      fVar6 = _DAT_142a7ba84;
    }
    fVar8 = 0.0;
    if (0.0 <= fVar5 / _DAT_142925970) {
      fVar8 = fVar5 / _DAT_142925970;
    }
    fVar5 = _DAT_142925904;
    if (fVar8 <= _DAT_142925904) {
      fVar5 = fVar8;
    }
    fVar5 = (_DAT_1429cd078 - (fVar5 + fVar5)) * fVar5 * fVar5;
    fVar6 = fVar5 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar5) * fVar6;
    if (fVar6 < 0.0) {
      FUN_140d8b480(0,fVar6,&UNK_142aa5190);
      pcVar4 = (code *)swi(3);
      (*pcVar4)();
      return;
    }
    fVar5 = fVar7 * _DAT_1429cd2d4 + (_DAT_142925904 - fVar7) * _DAT_142925970;
    fVar7 = 0.0;
    if (0.0 <= fVar5) {
      fVar7 = fVar5;
    }
    if (fVar7 <= fVar6) {
      fVar6 = fVar7;
    }
  }
  *param_1 = CONCAT44(fVar6 + fVar3 + auStack_98._4_4_,fVar6 * 0.0 + fVar1 + auStack_98._0_4_);
  *(float *)(param_1 + 1) = fVar2 + fStack_90 + fVar6 * 0.0;
  return;
}

