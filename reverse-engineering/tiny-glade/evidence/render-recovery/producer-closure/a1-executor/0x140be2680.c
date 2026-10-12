
void FUN_140be2680(longlong param_1)

{
  code *pcVar1;
  char cVar2;
  undefined *puStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined8 uStack_18;
  
  if (((*puRam000000014362c420 & 0x7fffffffffffffff) != 0) &&
     (cVar2 = FUN_1428d86b0(), cVar2 == '\0')) {
    return;
  }
  if (*(int *)(param_1 + 0x108) == 2) {
    if (*(longlong *)(param_1 + 0x18) == 0) {
      return;
    }
  }
  else {
    puStack_38 = &UNK_142b12d30;
    uStack_30 = 1;
    uStack_28 = 8;
    uStack_20 = 0;
    uStack_18 = 0;
    FUN_1428d9390(&puStack_38,&UNK_142b12d68);
  }
  FUN_1428d9430(&UNK_142b12d80,0x39,&UNK_142b12dc0);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

