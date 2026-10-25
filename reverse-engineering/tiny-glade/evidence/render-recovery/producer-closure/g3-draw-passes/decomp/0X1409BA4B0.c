// FUN_1409ba4b0  0x1409ba4b0

void FUN_1409ba4b0(longlong param_1)

{
  undefined8 uVar1;
  ulonglong uVar2;
  longlong lVar3;
  undefined1 uStack_29;
  undefined1 *puStack_28;
  
  lVar3 = 0;
  do {
    while( true ) {
      uVar1 = *(undefined8 *)(param_1 + 8 + lVar3);
      uVar2 = *(ulonglong *)(param_1 + 0x10 + lVar3);
      if (1 < uVar2) break;
      lVar3 = lVar3 + 0x18;
      if (lVar3 == 0x1f8) {
        return;
      }
    }
    puStack_28 = &uStack_29;
    if (uVar2 < 0x15) {
      FUN_1413396b0(uVar1,uVar2,1,&puStack_28);
    }
    else {
      FUN_140b4c720(uVar1,uVar2,&puStack_28);
    }
    lVar3 = lVar3 + 0x18;
  } while (lVar3 != 0x1f8);
  return;
}

