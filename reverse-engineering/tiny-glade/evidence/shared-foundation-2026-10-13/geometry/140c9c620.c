utils::geometry::rectangle::Rectangle2d::expand_along_edge

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c9c620(undefined8 *param_1,ulonglong param_2,float param_3,ulonglong param_4,
                  float param_5)

{
  code *pcVar1;
  undefined8 uVar2;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar3;
  
  fVar3 = (float)param_4;
  if ((param_2 | 2) == 3) {
    if (fVar3 <= param_5) {
      fVar4 = *(float *)((longlong)param_1 + 0x14);
      if (fVar3 <= param_3 + fVar4) {
        param_4 = (ulonglong)(uint)(param_3 + fVar4);
      }
      if ((float)param_4 <= param_5) {
        param_5 = (float)param_4;
      }
      *(float *)((longlong)param_1 + 0x14) = param_5;
LAB_140c9c681:
      uVar2 = _DAT_1429ddd00;
      if (param_2 < 3) {
        uVar2 = CONCAT44(*(undefined4 *)(&UNK_142b2ea84 + param_2 * 4),
                         *(undefined4 *)(&UNK_142b2ea90 + param_2 * 4));
      }
      fVar3 = (float)((ulonglong)uVar2 >> 0x20);
      fVar6 = (float)((ulonglong)*param_1 >> 0x20);
      fVar5 = (float)*param_1;
      fVar4 = (param_5 - fVar4) * _DAT_142925870;
      param_1[1] = CONCAT44(fVar4 * (fVar5 * fVar3 + fVar6 * (float)uVar2) +
                            (float)((ulonglong)param_1[1] >> 0x20),
                            fVar4 * ((float)((uint)fVar6 ^ _DAT_1429258f0) * fVar3 +
                                    fVar5 * (float)uVar2) + (float)param_1[1]);
      return;
    }
  }
  else if (fVar3 <= param_5) {
    fVar4 = *(float *)(param_1 + 2);
    if (fVar3 <= param_3 + fVar4) {
      param_4 = (ulonglong)(uint)(param_3 + fVar4);
    }
    if ((float)param_4 <= param_5) {
      param_5 = (float)param_4;
    }
    *(float *)(param_1 + 2) = param_5;
    if (param_2 < 4) goto LAB_140c9c681;
    goto LAB_140c9c709;
  }
  FUN_140d8b480(param_4,param_5,&UNK_142b2e5e0);
LAB_140c9c709:
  FUN_1428d9430(&UNK_142b2e8b0,0x2b,&UNK_142b2e8e0);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

