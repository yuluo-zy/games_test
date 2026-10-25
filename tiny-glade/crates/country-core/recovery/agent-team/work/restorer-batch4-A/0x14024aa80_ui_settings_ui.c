// ui_settings_ui
// sig: void __cdecl ui_settings_ui(void)
// size: 986

void __cdecl tiny_glade::gui::settings::ui_settings_ui(void)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  byte extraout_AL_02;
  byte extraout_AL_03;
  byte extraout_AL_04;
  byte bVar7;
  byte *in_RCX;
  longlong *in_R9;
  longlong lVar8;
  longlong *in_stack_00000028;
  byte local_95;
  
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  uVar1 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  puVar2 = (undefined4 *)in_R9[2];
  *puVar2 = uVar1;
  lVar8 = *in_R9;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar7 = *in_RCX;
  *in_RCX = extraout_AL | bVar7;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>_>
            ();
  local_95 = extraout_AL_00 | extraout_AL | bVar7;
  *in_RCX = local_95;
  lang_lib::lookup();
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  local_95 = extraout_AL_01 | local_95;
  *in_RCX = local_95;
  lang_lib::lookup();
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar7 = *(byte *)(lVar8 + 0xf4);
  if (bVar7 == 1) {
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *puVar2 = uVar1;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
    if ((extraout_AL_02 | extraout_AL_03) != 1) goto LAB_14024adca;
    bVar7 = *(byte *)(lVar8 + 0xf4);
  }
  else if (extraout_AL_02 == 0) goto LAB_14024adca;
  *in_RCX = 1;
  if ((bVar7 & 1) == 0) {
    lVar8 = 0;
  }
  else {
    lVar8 = ((ulonglong)*(uint *)(lVar8 + 0xec) << 0x20) + 1;
  }
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  lVar3 = *in_stack_00000028;
  lVar4 = *(longlong *)(lVar3 + 0x30);
  uVar5 = *(undefined8 *)(lVar3 + 0x40);
  if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar6 = *(longlong *)(lVar3 + 0x28);
  *(longlong *)(lVar6 + lVar4 * 0x10) = lVar8;
  *(undefined8 *)(lVar6 + 8 + lVar4 * 0x10) = uVar5;
  *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
  *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
  local_95 = 1;
LAB_14024adca:
  if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') {
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *puVar2 = uVar1;
    toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>
              ();
    *in_RCX = local_95 | extraout_AL_04;
  }
  return;
}

