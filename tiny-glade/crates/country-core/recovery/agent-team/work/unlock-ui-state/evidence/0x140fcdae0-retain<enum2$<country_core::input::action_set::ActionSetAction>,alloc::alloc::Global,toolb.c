
void __cdecl
alloc::vec::Vec::
retain<enum2_<country_core::input::action_set::ActionSetAction>,alloc::alloc::Global,toolbar::mei::core::mei_pre_update::closure_env_2>
          (void)

{
  undefined8 *puVar1;
  byte bVar2;
  longlong lVar3;
  longlong lVar4;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  char extraout_AL_08;
  longlong in_RCX;
  undefined1 *in_RDX;
  longlong lVar5;
  byte *in_R8;
  longlong lVar6;
  
  lVar3 = *(longlong *)(in_RCX + 0x10);
  if (lVar3 == 0) {
    return;
  }
  *(undefined8 *)(in_RCX + 0x10) = 0;
  bVar2 = *in_R8;
  lVar4 = *(longlong *)(in_RCX + 8);
  lVar5 = 0;
  lVar6 = 0;
  while (((((country_core::input::action_set::ActionSetAction::just_pressed(), extraout_AL == '\0'
            && (country_core::input::action_set::ActionSetAction::just_pressed(),
               extraout_AL_00 == '\0')) &&
           (country_core::input::action_set::ActionSetAction::just_pressed(), extraout_AL_01 == '\0'
           )) && ((country_core::input::action_set::ActionSetAction::just_pressed(),
                  extraout_AL_02 == '\0' &&
                  (country_core::input::action_set::ActionSetAction::just_pressed(),
                  extraout_AL_03 == '\0')))) || (*in_RDX = 1, (bVar2 & 1) != 0))) {
    lVar6 = lVar6 + 1;
    if (lVar6 == lVar3) goto LAB_140fcdbd6;
  }
  lVar5 = 1;
  while (lVar6 = lVar6 + 1, lVar6 != lVar3) {
    puVar1 = (undefined8 *)(lVar4 + lVar6 * 0xc);
    country_core::input::action_set::ActionSetAction::just_pressed();
    if (((extraout_AL_04 == '\0') &&
        (country_core::input::action_set::ActionSetAction::just_pressed(), extraout_AL_05 == '\0'))
       && ((country_core::input::action_set::ActionSetAction::just_pressed(), extraout_AL_06 == '\0'
           && ((country_core::input::action_set::ActionSetAction::just_pressed(),
               extraout_AL_07 == '\0' &&
               (country_core::input::action_set::ActionSetAction::just_pressed(),
               extraout_AL_08 == '\0')))))) {
      *(undefined4 *)(lVar4 + 8 + (lVar6 - lVar5) * 0xc) = *(undefined4 *)(puVar1 + 1);
      *(undefined8 *)(lVar4 + (lVar6 - lVar5) * 0xc) = *puVar1;
    }
    else {
      *in_RDX = 1;
      lVar5 = lVar5 + 1;
    }
  }
LAB_140fcdbd6:
  *(longlong *)(in_RCX + 0x10) = lVar3 - lVar5;
  return;
}

