
undefined8 FUN_140faeae0(undefined8 param_1,longlong param_2)

{
  char *pcVar1;
  char cVar2;
  longlong lVar3;
  undefined8 uVar4;
  bool bVar5;
  
  lVar3 = *(longlong *)(param_2 + 0x678);
  pcVar1 = (char *)(lVar3 + 0x10);
  LOCK();
  bVar5 = *(char *)(lVar3 + 0x10) == '\0';
  if (bVar5) {
    *(char *)(lVar3 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar5) {
    FUN_1428cd210(pcVar1,param_2,1000000000);
  }
  FUN_140c81c70(pcVar1);
  uVar4 = func_0x00014067dd80(lVar3 + 0x18);
  FUN_140c15150(param_1,uVar4);
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

