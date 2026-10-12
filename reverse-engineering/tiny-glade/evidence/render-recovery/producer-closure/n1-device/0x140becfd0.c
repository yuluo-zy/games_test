
void FUN_140becfd0(ulonglong *param_1,int *param_2)

{
  ulonglong uVar1;
  code *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  bool bVar6;
  int *piStack_40;
  code *pcStack_38;
  undefined *puStack_30;
  undefined8 uStack_28;
  int **ppiStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  if (*param_2 != 1) {
    pcStack_38 = FUN_140bed620;
    puStack_30 = &UNK_142b14bb0;
    uStack_28 = 1;
    uStack_10 = 0;
    ppiStack_20 = &piStack_40;
    uStack_18 = 1;
    piStack_40 = param_2;
    FUN_1428d9390(&puStack_30,&UNK_142b14bf0);
    pcVar2 = (code *)swi(3);
    (*pcVar2)();
    return;
  }
  uVar1 = *(ulonglong *)(param_2 + 2);
  uVar3 = *param_1;
  do {
    uVar5 = uVar1;
    if (uVar1 < uVar3) {
      uVar5 = uVar3;
    }
    LOCK();
    uVar4 = *param_1;
    bVar6 = uVar3 == uVar4;
    if (bVar6) {
      *param_1 = uVar5;
      uVar4 = uVar3;
    }
    UNLOCK();
    uVar3 = uVar4;
  } while (!bVar6);
  return;
}

