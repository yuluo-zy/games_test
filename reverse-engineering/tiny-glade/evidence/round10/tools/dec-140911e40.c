
void FUN_140911e40(undefined8 param_1,longlong *param_2,undefined8 param_3,longlong *param_4,
                  longlong *param_5)

{
  ulonglong uVar1;
  
  *(undefined4 *)param_4[2] = *(undefined4 *)((longlong)param_4 + 0x1c);
  uVar1 = (ulonglong)*(uint *)(*param_4 + 0x18);
                    /* WARNING: Could not recover jumptable at 0x000140911ea8. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&UNK_142aabc88 + *(int *)(&UNK_142aabc88 + uVar1 * 4)))
            (*(undefined4 *)(*param_4 + 0x1c),
             *(float *)(*param_2 + 0x28) * *(float *)(*param_5 + 0xec),uVar1,
             &UNK_142aabc88 + *(int *)(&UNK_142aabc88 + uVar1 * 4));
  return;
}

