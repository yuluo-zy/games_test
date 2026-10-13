undefined8 * FUN_14179a010(undefined8 *param_1,undefined8 param_2,undefined8 param_3)
{
  code *pcVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined8 *puVar4;
  uVar2 = FUN_1420d2b70(param_2,param_3);
  lVar3 = func_0x000140613c10(0x10,4);
  if (lVar3 != 0) {
    param_1[3] = 0;
    param_1[4] = 0;
    *param_1 = uVar2;
    param_1[1] = 4;
    param_1[2] = lVar3;
    param_1[5] = 0x3c88888900000001;
    return param_1;
  }
  FUN_1428d8fe3(4,0x10,&UNK_142cda3d0);
  pcVar1 = (code *)swi(3);
  puVar4 = (undefined8 *)(*pcVar1)();
  return puVar4;
}