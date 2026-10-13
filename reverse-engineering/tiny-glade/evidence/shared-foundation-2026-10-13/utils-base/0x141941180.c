/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
undefined8 FUN_141941180(undefined8 *param_1,ulonglong param_2)
{
  undefined8 uVar1;
  longlong lVar2;
  code *pcVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  float fVar6;
  float fVar7;
  undefined8 extraout_XMM0_Qa;
  undefined8 extraout_XMM0_Qa_00;
  undefined8 extraout_XMM0_Qa_01;
  undefined8 uVar8;
  float fVar9;
  if (param_2 == 0) {
    uVar8 = 0;
    if (*(int *)*param_1 != 1) {
      return 0;
    }
    if ((ulonglong)param_1[2] < 2) goto LAB_1419412de;
    if (param_1[4] != 0) {
      fVar6 = (((float *)param_1[1])[1] - *(float *)param_1[1]) * _DAT_1429cd078 *
              *(float *)param_1[3];
      fVar7 = 0.0;
      fVar9 = _DAT_1429cd078 * (float)((int *)*param_1)[1];
LAB_14194129f:
      return CONCAT44(fVar7,fVar6 - fVar9);
    }
LAB_1419412ef:
    uVar5 = FUN_1428d9518(uVar8,0,&UNK_142e71938);
    uVar8 = extraout_XMM0_Qa;
LAB_1419412ff:
    uVar5 = FUN_1428d9518(uVar8,uVar5,&UNK_142e71950);
    uVar8 = extraout_XMM0_Qa_00;
  }
  else {
    lVar2 = *(longlong *)param_1[5];
    if (lVar2 - 1U != param_2) {
      uVar5 = param_2 + 1;
      uVar4 = param_1[2];
      if (uVar5 < uVar4) {
        if (uVar4 <= param_2) goto LAB_1419412ba;
        uVar5 = param_1[4];
        uVar4 = param_2;
        if (param_2 < uVar5) {
          uVar8 = *(undefined8 *)(param_1[1] + param_2 * 4);
          fVar7 = (float)uVar8;
          uVar1 = *(undefined8 *)(param_1[3] + -4 + param_2 * 4);
          fVar9 = (float)uVar1 *
                  (fVar7 - *(float *)(param_1[1] + -4 + param_2 * 4)) * _DAT_142b2da40;
          fVar6 = (float)((ulonglong)uVar1 >> 0x20) *
                  ((float)((ulonglong)uVar8 >> 0x20) - fVar7) * _UNK_142b2da44;
          fVar7 = fVar6;
          goto LAB_14194129f;
        }
      }
      else {
        param_2 = uVar4;
        uVar4 = FUN_1428d9518(uVar5,uVar4,&UNK_142e718d8);
LAB_1419412ba:
        uVar5 = FUN_1428d9518(param_2,uVar4,&UNK_142e718f0);
      }
      uVar8 = FUN_1428d9518(uVar4,uVar5,&UNK_142e71908);
LAB_1419412de:
      uVar8 = FUN_1428d9518(uVar8);
      goto LAB_1419412ef;
    }
    uVar8 = 0;
    if (*(int *)param_1[6] != 1) {
      return 0;
    }
    uVar5 = param_1[2];
    if (uVar5 <= param_2) goto LAB_1419412ff;
    if (lVar2 - 2U < uVar5) {
      uVar5 = param_1[4];
      if (lVar2 - 2U < uVar5) {
        fVar6 = _DAT_1429cd078 * (float)((int *)param_1[6])[1];
        fVar9 = (*(float *)(param_1[1] + param_2 * 4) - *(float *)(param_1[1] + -8 + lVar2 * 4)) *
                _DAT_1429cd078 * *(float *)(param_1[3] + -8 + lVar2 * 4);
        fVar7 = 0.0;
        goto LAB_14194129f;
      }
      goto LAB_141941323;
    }
  }
  uVar5 = FUN_1428d9518(uVar8,uVar5,&UNK_142e71968);
  uVar8 = extraout_XMM0_Qa_01;
LAB_141941323:
  FUN_1428d9518(uVar8,uVar5,&UNK_142e71980);
  pcVar3 = (code *)swi(3);
  uVar8 = (*pcVar3)();
  return uVar8;
}