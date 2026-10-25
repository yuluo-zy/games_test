
undefined8 * FUN_1416a3620(undefined8 *param_1,longlong param_2)

{
  char *pcVar1;
  char cVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  bool bVar8;
  
  lVar7 = *(longlong *)(param_2 + 0x678);
  pcVar1 = (char *)(lVar7 + 0x10);
  LOCK();
  bVar8 = *(char *)(lVar7 + 0x10) == '\0';
  if (bVar8) {
    *(char *)(lVar7 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar8) {
    FUN_1428cd210(pcVar1,param_2,1000000000);
  }
  FUN_140c81c70(pcVar1);
  lVar7 = lVar7 + 0x18;
  uVar3 = func_0x000140c0e630(lVar7);
  uVar4 = func_0x000140c0e640(lVar7);
  uVar5 = func_0x000140c0e600(lVar7);
  uVar6 = func_0x000140c0e610(lVar7);
  FUN_140a4ca70(param_1 + 3,uVar6);
  *param_1 = uVar3;
  param_1[1] = uVar4;
  param_1[2] = uVar5;
  FUN_140c81e10();
  LOCK();
  cVar2 = *pcVar1;
  if (cVar2 == '\x01') {
    *pcVar1 = '\0';
  }
  UNLOCK();
  if (cVar2 != '\x01') {
    FUN_1428cdb90(pcVar1,0);
  }
  return param_1;
}

