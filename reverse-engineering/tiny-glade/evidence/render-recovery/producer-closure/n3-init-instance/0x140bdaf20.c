
ulonglong FUN_140bdaf20(longlong param_1)

{
  undefined8 uVar1;
  undefined8 uVar2;
  char cVar3;
  ulonglong uVar4;
  
  if (*(int *)(param_1 + 0x58) == 1) {
    uVar1 = *(undefined8 *)(param_1 + 0x18);
    uVar2 = *(undefined8 *)(param_1 + 0x20);
    cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118b2,0xb);
    if ((((((cVar3 != '\0') || (cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118bd,7), cVar3 != '\0'))
          && (cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118c4,2), cVar3 == '\0')) &&
         ((cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118c6,4), cVar3 == '\0' &&
          (cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118ca,5), cVar3 == '\0')))) &&
        ((cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118cf,4), cVar3 == '\0' &&
         ((cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118d3,5), cVar3 == '\0' &&
          (cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118d8,4), cVar3 == '\0')))))) &&
       ((cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118dc,5), cVar3 == '\0' &&
        (cVar3 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118e1,10), cVar3 == '\0')))) {
      uVar4 = FUN_140bd7250(uVar1,uVar2,&UNK_142b118eb,0xb);
      return uVar4 ^ 1;
    }
  }
  return 0;
}

