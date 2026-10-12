
ulonglong FUN_140bda840(longlong param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  char cVar3;
  ulonglong uVar4;
  int aiStack_40 [10];
  
  FUN_140bda990(aiStack_40,param_1);
  if (aiStack_40[0] == 5) {
    uVar1 = *(undefined8 *)(param_1 + 0x18);
    uVar2 = *(undefined8 *)(param_1 + 0x20);
    cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b1188b,10);
    if ((cVar3 == '\0') && (cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b11895,0x1d), cVar3 == '\0'))
    {
      if (*(char *)(param_1 + 0x99) == '\0') {
        if (*(ulonglong *)(param_1 + 0x88) < 1000000000) {
          return 0;
        }
        if (*(ulonglong *)(param_1 + 0x90) < 400000000) {
          return 0;
        }
      }
      else if (*(ulonglong *)(param_1 + 0x88) + *(longlong *)(param_1 + 0x90) < 1400000000) {
        return 0;
      }
      uVar4 = FUN_140bdaf20(param_1);
      return uVar4 ^ 1;
    }
  }
  return 0;
}

