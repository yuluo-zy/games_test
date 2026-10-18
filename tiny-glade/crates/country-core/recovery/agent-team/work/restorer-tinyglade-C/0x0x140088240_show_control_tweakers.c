// show_control_tweakers @ 0x0x140088240
// sig: void __cdecl show_control_tweakers(void)
// size: 883


void __cdecl tiny_glade::gui::gui_controls::show_control_tweakers(void)

{
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  byte extraout_AL_02;
  byte extraout_AL_03;
  byte extraout_AL_04;
  byte extraout_AL_05;
  byte *in_RCX;
  longlong in_R9;
  byte bVar1;
  
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar1 = *in_RCX;
  *in_RCX = extraout_AL | bVar1;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
  bVar1 = extraout_AL_00 | extraout_AL | bVar1;
  *in_RCX = bVar1;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar1 = extraout_AL_01 | bVar1;
  *in_RCX = bVar1;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar1 = extraout_AL_02 | bVar1;
  *in_RCX = bVar1;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  if (*(char *)(in_R9 + 0x11c) == '\x06') {
    *(byte *)(in_R9 + 0x11c) = 0;
  }
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot>_>
            ();
  bVar1 = bVar1 | extraout_AL_03;
  *in_RCX = bVar1;
  lang_lib::lookup();
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  if (*(char *)(in_R9 + 0x11d) == '\x06') {
    *(undefined1 *)(in_R9 + 0x11d) = 0;
  }
  toolbar::mei::widgets::tweaker::impl$13::
  tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::systems::main_camera::CameraRotationPivot>_>
            ();
  bVar1 = bVar1 | extraout_AL_04;
  *in_RCX = bVar1;
  if ((*(byte *)(in_R9 + 0x11c) | 2) == 3) {
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
    *in_RCX = bVar1 | extraout_AL_05;
  }
  return;
}

