
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c90050(ulonglong param_1,float param_2,undefined8 param_3,longlong *param_4,
                  undefined8 param_5)

{
  float fVar1;
  float fVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  ulonglong uStack_98;
  
  fVar1 = _DAT_142925904;
  if (param_1 < 3) {
    lVar3 = param_4[2];
    if ((ulonglong)(*param_4 - lVar3) < 2) {
      FUN_1428fc1a0(param_4,lVar3,2,4,4);
      lVar3 = param_4[2];
    }
    *(undefined8 *)(param_4[1] + lVar3 * 4) = 0x3f80000000000000;
  }
  else {
    fVar7 = (_DAT_142925904 / (float)(param_1 - 1)) * _DAT_142b2d878;
    fVar8 = fVar7;
    if (param_2 <= fVar7) {
      fVar8 = param_2;
    }
    lVar5 = param_4[2];
    if (lVar5 == *param_4) {
      FUN_140b5f870(param_4,param_5);
    }
    *(undefined4 *)(param_4[1] + lVar5 * 4) = 0;
    lVar5 = lVar5 + 1;
    param_4[2] = lVar5;
    fVar9 = (float)param_1 + _DAT_14295a0e4;
    param_1 = param_1 - 2;
    if ((ulonglong)(*param_4 - lVar5) < param_1) {
      FUN_1428fc1a0(param_4,lVar5,param_1,4,4);
      lVar5 = param_4[2];
    }
    fVar2 = _DAT_14295a0e8;
    lVar3 = param_4[1];
    uStack_98 = 0;
    do {
      fVar6 = (float)tiles(param_3);
      uVar4 = uStack_98 + 1;
      *(float *)(lVar3 + lVar5 * 4 + uStack_98 * 4) =
           (fVar6 + fVar2) *
           (float)(~-(uint)NAN(param_2) & (uint)fVar8 | (uint)fVar7 & -(uint)NAN(param_2)) +
           (float)uVar4 * (fVar1 / fVar9);
      uStack_98 = uVar4;
    } while (param_1 != uVar4);
    param_4[2] = lVar5 + uVar4;
    if (lVar5 + uVar4 == *param_4) {
      FUN_140b5f870(param_4,param_5);
    }
    lVar3 = lVar5 + uVar4 + -1;
    *(undefined4 *)(lVar5 * 4 + param_4[1] + uVar4 * 4) = 0x3f800000;
  }
  param_4[2] = lVar3 + 2;
  return;
}

