
ulonglong FUN_140c03d70(longlong param_1,ulonglong *param_2,longlong param_3,undefined8 param_4)

{
  char *pcVar1;
  char cVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  uint uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  bool bVar8;
  undefined1 uStack_3c;
  undefined1 uStack_3b;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  uVar7 = *param_2;
  if (uVar7 < 0xfffffffffffffff0 && (char)(((uint)uVar7 & 8) >> 3) == '\0') {
    LOCK();
    uVar6 = *param_2;
    if (uVar7 == uVar6) {
      *param_2 = uVar7 + 0x10;
    }
    UNLOCK();
    if (uVar7 != uVar6) goto code_r0x000140c03dab;
  }
  else {
code_r0x000140c03dab:
    FUN_1428cc4b0(param_2,0,param_3,1000000000);
  }
  FUN_140c81c70(param_2);
  FUN_140c81c70((ulonglong)param_2 | 1);
  pcVar1 = (char *)(param_1 + 0x70);
  LOCK();
  bVar8 = *(char *)(param_1 + 0x70) == '\0';
  if (bVar8) {
    *(char *)(param_1 + 0x70) = '\x01';
  }
  UNLOCK();
  if (!bVar8) {
    uStack_3b = 1;
    FUN_1428cd210(pcVar1);
  }
  uStack_3b = 1;
  FUN_140c81c70(pcVar1);
  uVar4 = *(undefined8 *)(param_1 + 0x78);
  uStack_3c = 0x21;
  uStack_39 = 1;
  uStack_3a = 1;
  uVar7 = 1;
  FUN_1418c1380(0x143634e40,&uStack_3c,1);
  uVar5 = (**(code **)(param_3 + 0x6d0))(uVar4,param_4);
  if (uVar5 == 0x3b9acdeb) {
    uVar6 = 0x100;
  }
  else {
    uVar6 = 0;
    if (uVar5 != 0) goto code_r0x000140c03e55;
  }
  uVar7 = 0;
code_r0x000140c03e55:
  uStack_39 = 0;
  uStack_3a = 1;
  FUN_140c81e10();
  LOCK();
  cVar2 = *pcVar1;
  if (cVar2 == '\x01') {
    *pcVar1 = '\0';
  }
  UNLOCK();
  if (cVar2 != '\x01') {
    uStack_39 = 0;
    uStack_3a = 1;
    FUN_1428cdb90(pcVar1,0);
  }
  uStack_39 = 0;
  uStack_3a = 0;
  FUN_140c81e10(param_2);
  uStack_39 = 0;
  uStack_3a = 0;
  FUN_140c81e10((ulonglong)param_2 | 1);
  LOCK();
  uVar3 = *param_2;
  *param_2 = *param_2 - 0x10;
  UNLOCK();
  if ((uVar3 & 0xfffffffffffffff2) == 0x12) {
    uStack_39 = 0;
    uStack_3a = 0;
    FUN_1428ccf50(param_2);
  }
  return uVar6 | (ulonglong)uVar5 << 0x20 | uVar7;
}

