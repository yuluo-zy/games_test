// addr=0x14093fa90 name=just_pressed
// signature=void __cdecl just_pressed(void)


void __cdecl country_core::input::action_set::DigitalActionState::just_pressed(void)

{
  ulonglong in_RCX;
  
                    // WARNING: Could not recover jumptable at 0x00014093faa0. Too many branches
                    // WARNING: Treating indirect jump as call
  (*(code *)(&DAT_142ab2c3c + *(int *)(&DAT_142ab2c3c + (in_RCX & 0xffffffff) * 4)))();
  return;
}


