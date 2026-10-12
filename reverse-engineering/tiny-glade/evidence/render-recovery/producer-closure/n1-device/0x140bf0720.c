
ulonglong FUN_140bf0720(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  ulonglong *puVar1;
  code *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  
  puVar1 = (ulonglong *)(param_1 + 0xa88);
  uVar5 = *(ulonglong *)(param_1 + 0xa88);
  if (uVar5 < 0xfffffffffffffff0 && (char)(((uint)uVar5 & 8) >> 3) == '\0') {
    LOCK();
    uVar3 = *puVar1;
    if (uVar5 == uVar3) {
      *puVar1 = uVar5 + 0x10;
    }
    UNLOCK();
    if (uVar5 == uVar3) goto code_r0x000140bf0776;
  }
  FUN_1428cc4b0(puVar1,0,param_3,1000000000);
code_r0x000140bf0776:
  FUN_140c81c70(puVar1);
  uVar5 = (ulonglong)puVar1 | 1;
  FUN_140c81c70(uVar5);
  if (*(longlong *)(param_1 + 0xa90) == 0) {
    FUN_140c81e10(puVar1);
    FUN_140c81e10(uVar5);
    LOCK();
    uVar5 = *puVar1;
    *puVar1 = *puVar1 - 0x10;
    UNLOCK();
    uVar4 = 0;
    uVar3 = 0;
  }
  else {
    if (*(ulonglong *)(param_1 + 0xa98) < 2) {
      FUN_1428d9518(1,*(ulonglong *)(param_1 + 0xa98),&UNK_142b15ec8);
                    /* WARNING: Does not return */
      pcVar2 = (code *)invalidInstructionException();
      (*pcVar2)();
    }
    uVar4 = CONCAT71((int7)((ulonglong)param_1 >> 8),
                     *(int *)(*(longlong *)(param_1 + 0xa90) + 4) != 0);
    FUN_140c81e10(puVar1);
    FUN_140c81e10(uVar5);
    LOCK();
    uVar5 = *puVar1;
    *puVar1 = *puVar1 - 0x10;
    UNLOCK();
    uVar3 = uVar4 & 0xffffffff;
  }
  if ((uVar5 & 0xfffffffffffffff2) == 0x12) {
    FUN_1428ccf50(puVar1);
    uVar3 = uVar4 & 0xffffffff;
  }
  return uVar3;
}

