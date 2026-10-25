
undefined8 *
FUN_140631ea0(undefined8 *param_1,undefined8 *param_2,undefined8 param_3,longlong param_4)

{
  code *pcVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  
  if (param_4 < 0) {
    uVar5 = 0;
LAB_140631f81:
    FUN_1428d8fe3(uVar5,param_4,&UNK_142a0cbc0);
                    /* WARNING: Does not return */
    pcVar1 = (code *)invalidInstructionException();
    (*pcVar1)();
  }
  if (param_4 == 0) {
    lVar4 = 1;
  }
  else {
    uVar5 = 1;
    lVar4 = func_0x000140613c10(param_4,1);
    if (lVar4 == 0) goto LAB_140631f81;
  }
  func_0x000142923c00(lVar4,param_3,param_4);
  if (param_2[3] != 0) {
    func_0x000140613c20(param_2[4],param_2[3],1);
  }
  param_2[3] = param_4;
  param_2[4] = lVar4;
  param_2[5] = param_4;
  uVar5 = *param_2;
  uVar2 = param_2[1];
  uVar3 = param_2[7];
  param_1[6] = param_2[6];
  param_1[7] = uVar3;
  *param_1 = uVar5;
  param_1[1] = uVar2;
  param_1[4] = param_2[4];
  param_1[5] = param_2[5];
  param_1[2] = param_2[2];
  param_1[3] = param_2[3];
  return param_1;
}

