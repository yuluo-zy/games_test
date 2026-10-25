
void FUN_140bd4f80(undefined8 *param_1,undefined8 param_2,longlong param_3)

{
  code *pcVar1;
  
  if (param_1[1] == param_3) {
                    /* WARNING: Could not recover jumptable at 0x000142923c00. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    memcpy(param_2,*param_1);
    return;
  }
  FUN_1428d98e0(param_3,param_1[1],&UNK_142b0f8e8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

