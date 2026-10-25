// ui_settings_game
// sig: void __cdecl ui_settings_game(void)
// size: 1392

void __cdecl tiny_glade::gui::settings::ui_settings_game(void)

{
  byte bVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined4 *puVar7;
  byte bVar8;
  char extraout_AL;
  char extraout_AL_00;
  byte extraout_AL_01;
  byte extraout_AL_02;
  byte extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  char extraout_AL_08;
  byte *in_RCX;
  byte *in_RDX;
  byte bVar9;
  longlong in_stack_00000028;
  longlong *in_stack_00000030;
  
  bVar1 = *in_RCX;
  bVar9 = bVar1 - 2;
  bVar8 = 1;
  if (bVar9 < 3) {
    bVar8 = bVar9;
  }
  if (bVar8 == 0) {
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    uVar2 = *(undefined4 *)(in_stack_00000028 + 0x1c);
    puVar7 = *(undefined4 **)(in_stack_00000028 + 0x10);
    *puVar7 = uVar2;
    toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>
              ();
    bVar9 = *in_RDX;
    *in_RDX = extraout_AL_01 | bVar9;
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *puVar7 = uVar2;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep>();
    bVar9 = extraout_AL_02 | extraout_AL_01 | bVar9;
    *in_RDX = bVar9;
    lang_lib::lookup();
    toolbar::mei::widgets::tweaker::HintTracker::text();
    toolbar::mei::widgets::tweaker::impl$11::default();
    *puVar7 = uVar2;
    toolbar::mei::widgets::tweaker::impl$13::
    tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat>_>
              ();
    *in_RDX = extraout_AL_03 | bVar9;
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::label();
    toolbar::mei::widgets::LabelView::content_mut();
    tg_text::TextFragment::set_metrics<f32>();
    toolbar::mei::widgets::LabelView::weight();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::hover();
    if (extraout_AL_04 != '\0') {
      lang_lib::lookup();
      toolbar::mei::widgets::tweaker::HintTracker::text();
      toolbar::mei::widgets::tweaker::MaybeHint::confirm();
    }
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::menu_button();
    toolbar::mei::widgets::MenuButtonView::content_mut();
    tg_text::TextFragment::set_metrics<f32>();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::hover();
    if (extraout_AL_05 != '\0') {
      lang_lib::lookup();
      toolbar::mei::widgets::tweaker::HintTracker::text();
      toolbar::mei::widgets::tweaker::MaybeHint::confirm();
    }
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::used();
    if (extraout_AL_06 != '\0') {
      *in_RCX = 0;
    }
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::menu_button();
    toolbar::mei::widgets::MenuButtonView::content_mut();
    tg_text::TextFragment::set_metrics<f32>();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::hover();
    if (extraout_AL_07 != '\0') {
      lang_lib::lookup();
      toolbar::mei::widgets::tweaker::HintTracker::text();
      toolbar::mei::widgets::tweaker::MaybeHint::confirm();
    }
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::used();
    if (extraout_AL_08 != '\0') {
      *in_RCX = 1;
    }
  }
  else if (bVar8 == 1) {
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::label();
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::menu_button();
    toolbar::mei::widgets::MenuButtonView::content_mut();
    tg_text::TextFragment::set_metrics<f32>();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::used();
    if (extraout_AL != '\0') {
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      lVar3 = *in_stack_00000030;
      lVar4 = *(longlong *)(lVar3 + 0x30);
      uVar5 = *(undefined8 *)(lVar3 + 0x40);
      if (lVar4 == *(longlong *)(lVar3 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar6 = *(longlong *)(lVar3 + 0x28);
      *(undefined8 *)(lVar6 + lVar4 * 0x10) = uVar5;
      *(byte *)(lVar6 + 8 + lVar4 * 0x10) = bVar1 & 1;
      *(longlong *)(lVar3 + 0x30) = lVar4 + 1;
      *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
      *in_RCX = 4;
      *in_RDX = 1;
    }
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::menu_button();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::used();
    if (extraout_AL_00 != '\0') {
      *in_RCX = 2;
    }
  }
  else {
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::label();
  }
  return;
}

