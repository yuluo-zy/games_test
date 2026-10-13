
void FUN_14123ad70(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4)

{
  *(undefined4 *)(param_1 + 1) = *(undefined4 *)(param_2 + 1);
  *param_1 = *param_2;
  *(undefined8 *)((longlong)param_1 + 0xc) = *param_3;
  *(undefined4 *)((longlong)param_1 + 0x14) = *(undefined4 *)(param_3 + 1);
  param_1[3] = *param_4;
  *(undefined4 *)(param_1 + 4) = *(undefined4 *)(param_4 + 1);
  return;
}

