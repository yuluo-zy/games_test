// ui_settings_developer
// sig: void __cdecl ui_settings_developer(void)
// size: 695

void __cdecl tiny_glade::gui::settings::ui_settings_developer(void)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  byte extraout_AL;
  byte extraout_AL_00;
  byte extraout_AL_01;
  byte extraout_AL_02;
  byte extraout_AL_03;
  byte extraout_AL_04;
  byte *in_RCX;
  longlong in_R9;
  byte bVar3;
  
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  uVar1 = *(undefined4 *)(in_R9 + 0x1c);
  puVar2 = *(undefined4 **)(in_R9 + 0x10);
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar3 = *in_RCX;
  *in_RCX = extraout_AL | bVar3;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar3 = extraout_AL_00 | extraout_AL | bVar3;
  *in_RCX = bVar3;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar3 = extraout_AL_01 | bVar3;
  *in_RCX = bVar3;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar3 = extraout_AL_02 | bVar3;
  *in_RCX = bVar3;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  bVar3 = extraout_AL_03 | bVar3;
  *in_RCX = bVar3;
  toolbar::mei::widgets::tweaker::HintTracker::text();
  toolbar::mei::widgets::tweaker::impl$11::default();
  *puVar2 = uVar1;
  toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>();
  *in_RCX = (extraout_AL_04 | bVar3) & 1;
  toolbar::mei::widgets::impl$26::label();
  toolbar::mei::widgets::LabelView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  toolbar::mei::widgets::LabelView::color();
  toolbar::mei::widgets::ActionHintView::into_inner();
  return;
}

