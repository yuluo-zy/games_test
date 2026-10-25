
void FUN_140bd4c00(longlong param_1,longlong param_2)

{
  code *pcVar1;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  
  if (*(short *)(param_1 + 0x14) == *(short *)(*(longlong *)(param_2 + 0x648) + 0xa50)) {
    if (*(char *)(param_1 + 0x16) == '\x01') {
      if (*(int *)(*(longlong *)(param_1 + 8) + 0xc) == *(int *)(param_1 + 0x10)) {
        uStack_20 = *(undefined8 *)(*(longlong *)(param_1 + 8) + 0x30);
        uStack_30 = CONCAT44(uStack_30._4_4_,0x3b9e8321);
        uStack_28 = 0;
        (**(code **)(param_2 + 0x508))(*(undefined8 *)(param_2 + 0x640),&uStack_30);
        return;
      }
      goto code_r0x000140bd4cc1;
    }
  }
  else {
    uStack_30 = 0;
    param_1 = param_1 + 0x14;
    FUN_1428ca816(0,param_1,*(longlong *)(param_2 + 0x648) + 0xa50,&uStack_30,&UNK_142d022a0);
  }
  uStack_30 = 0;
  FUN_1428ca902(0,param_1 + 0x16,&UNK_142d022d9,&uStack_30,&UNK_142d022c0);
code_r0x000140bd4cc1:
  FUN_1428d9310(&UNK_142b0f7d8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

