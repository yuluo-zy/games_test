
ulonglong * FUN_1418aeba0(longlong param_1,longlong param_2,int *param_3)

{
  ulonglong uVar1;
  code *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  undefined *puVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  bool bVar8;
  undefined *puStack_40;
  undefined8 uStack_38;
  int **ppiStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  int *piStack_10;
  code *pcStack_8;
  
  if (param_3 == (int *)0x0) {
    if (*(short *)(param_2 + 0x14) != *(short *)(param_1 + 0xa50)) {
      puVar5 = &UNK_142d022a0;
      goto code_r0x0001418aec9a;
    }
    if (*(char *)(param_2 + 0x16) == '\x01') {
      puVar6 = (ulonglong *)0x0;
      if (*(int *)(*(longlong *)(param_2 + 8) + 0xc) == *(int *)(param_2 + 0x10)) {
        puVar6 = (ulonglong *)(*(longlong *)(param_2 + 8) + 0x10);
      }
      return puVar6;
    }
code_r0x0001418aecae:
    puVar5 = &UNK_142d022c0;
  }
  else {
    if (*(short *)(param_2 + 0x14) != *(short *)(param_1 + 0xa50)) {
      puVar5 = &UNK_142d022e0;
code_r0x0001418aec9a:
      puStack_40 = (undefined *)0x0;
      param_2 = param_2 + 0x14;
      FUN_1428ca816(0,param_2,param_1 + 0xa50,&puStack_40,puVar5);
      goto code_r0x0001418aecae;
    }
    if (*(char *)(param_2 + 0x16) == '\x01') {
      puVar6 = *(ulonglong **)(param_2 + 8);
      if (*(int *)((longlong)puVar6 + 0xc) != *(int *)(param_2 + 0x10)) {
        return (ulonglong *)0x0;
      }
      if (*param_3 == 1) {
        uVar1 = *(ulonglong *)(param_3 + 2);
        uVar3 = *puVar6;
        do {
          uVar7 = uVar1;
          if (uVar1 < uVar3) {
            uVar7 = uVar3;
          }
          LOCK();
          uVar4 = *puVar6;
          bVar8 = uVar3 == uVar4;
          if (bVar8) {
            *puVar6 = uVar7;
            uVar4 = uVar3;
          }
          UNLOCK();
          uVar3 = uVar4;
        } while (!bVar8);
        return puVar6 + 2;
      }
      goto code_r0x0001418aecda;
    }
    puVar5 = &UNK_142d022f8;
  }
  puStack_40 = (undefined *)0x0;
  param_3 = (int *)&UNK_142d022d9;
  FUN_1428ca902(0,param_2 + 0x16,&UNK_142d022d9,&puStack_40,puVar5);
code_r0x0001418aecda:
  pcStack_8 = FUN_140bed620;
  puStack_40 = &UNK_142b14bb0;
  uStack_38 = 1;
  uStack_20 = 0;
  ppiStack_30 = &piStack_10;
  uStack_28 = 1;
  piStack_10 = param_3;
  FUN_1428d9390(&puStack_40,&UNK_142b14bf0);
  pcVar2 = (code *)swi(3);
  puVar6 = (ulonglong *)(*pcVar2)();
  return puVar6;
}

