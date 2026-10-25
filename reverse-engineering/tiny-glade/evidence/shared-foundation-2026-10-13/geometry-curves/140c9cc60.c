
void FUN_140c9cc60(undefined8 *param_1,undefined8 *param_2)

{
  undefined4 uVar1;
  undefined8 uVar2;
  
  param_1[5] = param_2[5];
  uVar2 = param_2[4];
  param_1[3] = param_2[3];
  param_1[4] = uVar2;
  uVar1 = *(undefined4 *)(param_2 + 6);
  uVar2 = param_2[1];
  *param_1 = *param_2;
  param_1[1] = uVar2;
  param_1[2] = param_2[2];
  *(undefined4 *)(param_1 + 6) = uVar1;
  return;
}

