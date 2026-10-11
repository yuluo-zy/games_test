
void __cdecl
country_core::systems::history::replay_wall::WallReplayEditCmd::
process<country_core::systems::history::replay_wall::replay_wall::closure_env_0>(void)

{
  byte *in_RCX;
  longlong lVar1;
  
  if (in_RCX[0x15] != 0) {
    return;
  }
  if ((*in_RCX & 1) != 0) {
    ::utils::easing_functions::cubic_in_out();
    lVar1 = 0;
    if (*(longlong *)(*(longlong *)(in_RCX + 8) + 0x10) < -0x7ffffffffffffff0) {
      lVar1 = *(longlong *)(*(longlong *)(in_RCX + 8) + 0x10) + -0x7fffffffffffffff;
    }
                    /* WARNING: Could not emulate address calculation at 0x000140abfc1c */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)((longlong)&DAT_142adf44c + (longlong)(int)(&DAT_142adf44c)[lVar1]))
              ((code *)((longlong)&DAT_142adf44c + (longlong)(int)(&DAT_142adf44c)[lVar1]));
    return;
  }
  in_RCX[0x15] = 1;
  lVar1 = 0;
  if (*(longlong *)(*(longlong *)(in_RCX + 8) + 0x10) < -0x7ffffffffffffff0) {
    lVar1 = *(longlong *)(*(longlong *)(in_RCX + 8) + 0x10) + -0x7fffffffffffffff;
  }
                    /* WARNING: Could not emulate address calculation at 0x000140abfc69 */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)((longlong)&DAT_142adf408 + (longlong)(int)(&DAT_142adf408)[lVar1]))
            ((code *)((longlong)&DAT_142adf408 + (longlong)(int)(&DAT_142adf408)[lVar1]));
  return;
}

