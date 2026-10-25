
void FUN_14205eff0(void)

{
  longlong lVar1;
  uint *in_R9;
  
  lVar1 = 0;
  if (*in_R9 - 3 < 3) {
    lVar1 = (ulonglong)*in_R9 - 2;
  }
                    /* WARNING: Could not emulate address calculation at 0x00014205f02e */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&UNK_142fcf75c + *(int *)(&UNK_142fcf75c + lVar1 * 4)))();
  return;
}

