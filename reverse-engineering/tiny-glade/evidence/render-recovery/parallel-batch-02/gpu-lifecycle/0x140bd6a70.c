
void FUN_140bd6a70(longlong param_1,int *param_2)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  code *pcVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  bool bVar7;
  int *piStack_40;
  code *pcStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  int **ppiStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  puVar1 = *(ulonglong **)(param_1 + 8);
  if (*(int *)((longlong)puVar1 + 0xc) == *(int *)(param_1 + 0x10)) {
    if (*param_2 != 1) {
      pcStack_38 = FUN_140bed620;
      puStack_30 = &UNK_142b14bb0;
      uStack_28 = 1;
      uStack_10 = 0;
      ppiStack_20 = &piStack_40;
      uStack_18 = 1;
      piStack_40 = param_2;
      FUN_1428d9390(&puStack_30,&UNK_142b14bf0);
      pcVar3 = (code *)swi(3);
      (*pcVar3)();
      return;
    }
    uVar2 = *(ulonglong *)(param_2 + 2);
    uVar4 = *puVar1;
    do {
      uVar6 = uVar2;
      if (uVar2 < uVar4) {
        uVar6 = uVar4;
      }
      LOCK();
      uVar5 = *puVar1;
      bVar7 = uVar4 == uVar5;
      if (bVar7) {
        *puVar1 = uVar6;
        uVar5 = uVar4;
      }
      UNLOCK();
      uVar4 = uVar5;
    } while (!bVar7);
  }
  return;
}

