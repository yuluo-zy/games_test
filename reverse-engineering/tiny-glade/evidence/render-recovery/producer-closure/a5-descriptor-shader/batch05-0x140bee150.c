
void FUN_140bee150(longlong param_1,int *param_2,longlong param_3,undefined4 param_4,
                  undefined8 param_5)

{
  code *pcVar1;
  longlong lVar2;
  undefined8 uStack_28;
  
  lVar2 = FUN_1418aee50(*(undefined8 *)(param_1 + 0x648),param_5,param_2);
  if (lVar2 != 0) {
    if (*param_2 == 1) {
      uStack_28 = *(undefined8 *)(lVar2 + 8);
      (**(code **)(param_1 + 0x328))
                (*(undefined8 *)(param_2 + 0x12),*(undefined4 *)(param_3 + 0x30),
                 *(undefined8 *)(param_3 + 0x20),param_4,1,&uStack_28,0,4);
      return;
    }
    FUN_1428d9430(&UNK_142b15b78,0x4c,&UNK_142b15bf0);
  }
  FUN_1428d9310(&UNK_142b157d8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

