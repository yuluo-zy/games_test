
void FUN_1416f7980(longlong *param_1,undefined4 param_2,undefined8 param_3)

{
  longlong lVar1;
  longlong lVar2;
  code *pcVar3;
  longlong lVar4;
  undefined8 uStack_20;
  
  lVar1 = *param_1;
  lVar2 = param_1[1];
  lVar4 = FUN_1418aed30(*(undefined8 *)(lVar2 + 0x648),param_3,lVar1);
  if (lVar4 != 0) {
    uStack_20 = *(undefined8 *)(lVar4 + 0x20);
    (**(code **)(lVar2 + 0x338))(*(undefined8 *)(lVar1 + 0x48),param_2,1,&uStack_20,&UNK_142c91748);
    return;
  }
  FUN_1428d9310(&UNK_142c91730);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
}

