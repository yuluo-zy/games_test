
void FUN_140bf0b40(longlong param_1,int *param_2)

{
  code *pcVar1;
  
  if (*(longlong *)(param_1 + 0x870) == 0) {
    return;
  }
  if (*param_2 == 1) {
                    /* WARNING: Could not recover jumptable at 0x000140bf0b5e. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (**(code **)(param_1 + 0x8a0))(*(undefined8 *)(param_2 + 0x12));
    return;
  }
  FUN_1428d9430(&UNK_142b15b78,0x4c,&UNK_142b15bf0);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

