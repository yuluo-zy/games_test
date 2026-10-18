// ui_settings_audio @ 0x1402475d0
// sig: void __cdecl ui_settings_audio(void)
// size: 658


void __cdecl tiny_glade::gui::settings::ui_settings_audio(void)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  byte extraout_AL_02;
  byte *in_RCX;
  longlong in_R9;
  byte bVar3;
  
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  uVar1 = *(undefined4 *)(in_R9 + 0x1c);
  puVar2 = *(undefined4 **)(in_R9 + 0x10);
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar3 = *in_RCX;
  *in_RCX = extraout_AL | bVar3;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar3 = extraout_AL_00 | extraout_AL | bVar3;
  *in_RCX = bVar3;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar3 = extraout_AL_01 | bVar3;
  *in_RCX = bVar3;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  *in_RCX = extraout_AL_02 | bVar3;
  return;
}

