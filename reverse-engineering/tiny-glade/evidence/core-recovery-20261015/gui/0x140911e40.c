   1	
   2	void FUN_140911e40(undefined8 param_1,longlong *param_2,undefined8 param_3,longlong *param_4,
   3	                  longlong *param_5)
   4	
   5	{
   6	  ulonglong uVar1;
   7	  
   8	  *(undefined4 *)param_4[2] = *(undefined4 *)((longlong)param_4 + 0x1c);
   9	  uVar1 = (ulonglong)*(uint *)(*param_4 + 0x18);
  10	                    /* WARNING: Could not recover jumptable at 0x000140911ea8. Too many branches */
  11	                    /* WARNING: Treating indirect jump as call */
  12	  (*(code *)(&UNK_142aabc88 + *(int *)(&UNK_142aabc88 + uVar1 * 4)))
  13	            (*(undefined4 *)(*param_4 + 0x1c),
  14	             *(float *)(*param_2 + 0x28) * *(float *)(*param_5 + 0xec),uVar1,
  15	             &UNK_142aabc88 + *(int *)(&UNK_142aabc88 + uVar1 * 4));
  16	  return;
  17	}
  18	
