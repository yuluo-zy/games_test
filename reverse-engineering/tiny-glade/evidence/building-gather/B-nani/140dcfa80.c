
void __cdecl bevy_ecs::error::handler::panic(void)

{
  uint *in_RDX;
  
                    /* WARNING: Could not recover jumptable at 0x000140dcfaac. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&DAT_142b73838 + *(int *)(&DAT_142b73838 + (ulonglong)*in_RDX * 4)))();
  return;
}

