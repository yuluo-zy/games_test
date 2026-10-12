
undefined8 FUN_140bda8f0(longlong param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  char cVar3;
  ulonglong uVar4;
  bool bVar5;
  int aiStack_40 [10];
  
  FUN_140bda990(aiStack_40,param_1);
  if (aiStack_40[0] == 5) {
    uVar1 = *(undefined8 *)(param_1 + 0x18);
    uVar2 = *(undefined8 *)(param_1 + 0x20);
    cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b1188b,10);
    if ((cVar3 == '\0') && (cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b11895,0x1d), cVar3 == '\0'))
    {
      uVar4 = *(ulonglong *)(param_1 + 0x88);
      if (*(char *)(param_1 + 0x99) == '\0') {
        if (uVar4 < 1000000000) {
          return 0;
        }
        bVar5 = *(ulonglong *)(param_1 + 0x90) < 400000000;
      }
      else {
        uVar4 = uVar4 + *(longlong *)(param_1 + 0x90);
        bVar5 = uVar4 < 1400000000;
      }
      return CONCAT71((int7)(uVar4 >> 8),!bVar5);
    }
  }
  return 0;
}

