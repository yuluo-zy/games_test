
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float * FUN_1407d8fe0(float *param_1,longlong param_2,float *param_3,float *param_4,
                     undefined8 param_5,undefined8 param_6,undefined4 param_7,undefined4 param_8)

{
  char cVar1;
  longlong lVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined8 uStack_d0;
  float afStack_c4 [33];
  
  fVar9 = param_3[1];
  fVar6 = param_4[1];
  if (fVar9 <= fVar6) {
    lVar2 = func_0x000140959ef0(param_2 + 0x20);
    fVar12 = 0.0;
    if ((lVar2 != 0) && (lVar2 = param_2, *(char *)(param_2 + 0x38) == '\x01'))
    goto code_r0x0001407d9079;
  }
  else {
    lVar2 = func_0x000140959ef0(param_2);
    fVar12 = 0.0;
    if ((lVar2 != 0) && (*(char *)(param_2 + 0x18) == '\x01')) {
      lVar2 = param_2 + 0x20;
code_r0x0001407d9079:
      lVar2 = func_0x000140959ef0(lVar2);
      fVar12 = _UNK_142a80a4c;
      if (lVar2 != 0) {
        fVar12 = 0.0;
      }
    }
  }
  fVar8 = *param_3;
  fVar10 = param_3[2];
  fVar11 = *param_4;
  fVar7 = param_4[2];
  fVar3 = fVar8 - fVar11;
  fVar4 = fVar10 - fVar7;
  fVar5 = _UNK_142925904 / SQRT(fVar4 * fVar4 + fVar3 * fVar3);
  uStack_d0 = CONCAT44(fVar5 * fVar4,fVar5 * fVar3);
  func_0x000140c9cdc0(afStack_c4,&uStack_d0);
  if (((fVar6 <= fVar9) || (*(byte *)(param_2 + 0x18) < 2)) ||
     (cVar1 = FUN_1407d9240(param_5,param_6,param_7,param_8,1), cVar1 != '\0')) {
    fVar3 = 0.0;
    if ((fVar9 <= fVar6) || (*(byte *)(param_2 + 0x38) < 2)) goto code_r0x0001407d916e;
    cVar1 = FUN_1407d9240(param_5,param_6,param_8,param_7,0);
    fVar3 = 0.0;
    if (cVar1 != '\0') goto code_r0x0001407d916e;
  }
  fVar3 = _UNK_1429258d8;
code_r0x0001407d916e:
  fVar5 = SUB84(afStack_c4._4_8_,4);
  fVar4 = (float)afStack_c4._4_8_;
  if (fVar9 <= fVar6) {
    fVar8 = fVar8 + afStack_c4[0] * fVar3;
    fVar9 = fVar9 + fVar4 * fVar3;
    fVar10 = fVar10 + fVar5 * fVar3;
    fVar11 = fVar11 + afStack_c4[0] * fVar12;
    fVar6 = fVar6 + fVar12 * fVar4;
    fVar7 = fVar7 + fVar12 * fVar5;
  }
  else {
    fVar8 = fVar8 - afStack_c4[0] * fVar12;
    fVar9 = fVar9 - fVar4 * fVar12;
    fVar10 = fVar10 - fVar5 * fVar12;
    fVar11 = fVar11 - afStack_c4[0] * fVar3;
    fVar6 = fVar6 - fVar3 * fVar4;
    fVar7 = fVar7 - fVar3 * fVar5;
  }
  *param_1 = fVar8;
  param_1[1] = fVar9;
  param_1[2] = fVar10;
  param_1[3] = fVar11;
  *(ulonglong *)(param_1 + 4) = CONCAT44(fVar7,fVar6);
  return param_1;
}

