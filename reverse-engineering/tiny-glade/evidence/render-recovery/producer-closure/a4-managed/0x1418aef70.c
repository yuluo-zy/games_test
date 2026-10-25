
ulonglong * FUN_1418aef70(longlong param_1,longlong param_2,int *param_3)

{
  ulonglong uVar1;
  code *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  bool bVar7;
  undefined *puStack_40;
  undefined8 uStack_38;
  int **ppiStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  int *piStack_10;
  code *pcStack_8;
  
  if (*(short *)(param_2 + 0x14) == *(short *)(param_1 + 0xa50)) {
    if (*(char *)(param_2 + 0x16) == '\x04') {
      puVar5 = *(ulonglong **)(param_2 + 8);
      if (*(int *)((longlong)puVar5 + 0xc) != *(int *)(param_2 + 0x10)) {
        return (ulonglong *)0x0;
      }
      if (*param_3 == 1) {
        uVar1 = *(ulonglong *)(param_3 + 2);
        uVar3 = *puVar5;
        do {
          uVar6 = uVar1;
          if (uVar1 < uVar3) {
            uVar6 = uVar3;
          }
          LOCK();
          uVar4 = *puVar5;
          bVar7 = uVar3 == uVar4;
          if (bVar7) {
            *puVar5 = uVar6;
            uVar4 = uVar3;
          }
          UNLOCK();
          uVar3 = uVar4;
        } while (!bVar7);
        return puVar5 + 2;
      }
      goto code_r0x0001418af032;
    }
  }
  else {
    puStack_40 = (undefined *)0x0;
    param_2 = param_2 + 0x14;
    FUN_1428ca816(0,param_2,param_1 + 0xa50,&puStack_40,&UNK_142d022e0);
  }
  puStack_40 = (undefined *)0x0;
  param_3 = (int *)&UNK_142d022db;
  FUN_1428ca902(0,param_2 + 0x16,&UNK_142d022db,&puStack_40,&UNK_142d022f8);
code_r0x0001418af032:
  pcStack_8 = FUN_140bed620;
  puStack_40 = &UNK_142b14bb0;
  uStack_38 = 1;
  uStack_20 = 0;
  ppiStack_30 = &piStack_10;
  uStack_28 = 1;
  piStack_10 = param_3;
  FUN_1428d9390(&puStack_40,&UNK_142b14bf0);
  pcVar2 = (code *)swi(3);
  puVar5 = (ulonglong *)(*pcVar2)();
  return puVar5;
}

