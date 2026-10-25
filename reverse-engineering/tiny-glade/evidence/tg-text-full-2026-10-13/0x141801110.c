
void FUN_141801110(longlong param_1,undefined8 *param_2)

{
  char cVar1;
  undefined8 uVar2;
  char *pcVar3;
  longlong lVar4;
  undefined4 *puVar5;
  longlong lVar6;
  longlong lStack_30;
  undefined8 uStack_28;
  
  lVar4 = 0;
  if (*(longlong *)(param_1 + 0x38) < -0x7ffffffffffffffe) {
    lVar4 = *(longlong *)(param_1 + 0x38) + -0x7fffffffffffffff;
  }
  if (lVar4 == 0) {
    lStack_30 = param_1 + 0x38;
    uStack_28 = *param_2;
    puVar5 = (undefined4 *)param_2[2];
    cVar1 = *(char *)param_2[1];
    if (cVar1 == '\x05') {
      return;
    }
    if (*(longlong *)(param_1 + 0x48) != 0) {
      lVar6 = *(longlong *)(param_1 + 0x48) * 0xb0;
      lVar4 = *(longlong *)(param_1 + 0x40);
      do {
        FUN_141a96310(lVar4,cVar1);
        lVar6 = lVar6 + -0xb0;
        lVar4 = lVar4 + 0xb0;
      } while (lVar6 != 0);
    }
  }
  else if (lVar4 == 1) {
    uStack_28 = *param_2;
    puVar5 = (undefined4 *)param_2[2];
    lStack_30 = *(longlong *)(param_1 + 0x40);
    cVar1 = *(char *)param_2[1];
    if (cVar1 == '\x05') {
      return;
    }
    if (*(longlong *)(lStack_30 + 0x10) != 0) {
      lVar6 = *(longlong *)(lStack_30 + 0x10) * 0xb0;
      lVar4 = *(longlong *)(lStack_30 + 8);
      do {
        FUN_141a96310(lVar4,cVar1);
        lVar6 = lVar6 + -0xb0;
        lVar4 = lVar4 + 0xb0;
      } while (lVar6 != 0);
    }
  }
  else {
    uVar2 = *param_2;
    pcVar3 = (char *)param_2[1];
    puVar5 = (undefined4 *)param_2[2];
    lStack_30 = FUN_141801600(param_1 + 0x40);
    cVar1 = *pcVar3;
    if (cVar1 == '\x05') {
      return;
    }
    uStack_28 = uVar2;
    if (*(longlong *)(lStack_30 + 0x10) != 0) {
      lVar6 = *(longlong *)(lStack_30 + 0x10) * 0xb0;
      lVar4 = *(longlong *)(lStack_30 + 8);
      do {
        FUN_141a96310(lVar4,cVar1);
        lVar6 = lVar6 + -0xb0;
        lVar4 = lVar4 + 0xb0;
      } while (lVar6 != 0);
    }
  }
  FUN_1417fbe00(&lStack_30,*puVar5,puVar5[1]);
  return;
}

