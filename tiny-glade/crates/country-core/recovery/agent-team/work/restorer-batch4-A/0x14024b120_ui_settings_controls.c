// ui_settings_controls
// sig: void __cdecl ui_settings_controls(void)
// size: 524

void __cdecl tiny_glade::gui::settings::ui_settings_controls(void)

{
  longlong lVar1;
  byte *pbVar2;
  longlong *plVar3;
  char extraout_AL;
  char extraout_AL_00;
  ulonglong uVar4;
  longlong *in_RCX;
  longlong in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  
  lVar1 = *in_stack_00000030;
  pbVar2 = (byte *)*in_stack_00000038;
  toolbar::mei::core::impl$0::as_id();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  toolbar::mei::core::NodeContext::child_with_global_id();
  toolbar::mei::config::impl$18::template();
  toolbar::mei::config::TemplateView::into_slot<ref_<str_>_>();
  **(undefined4 **)(in_stack_00000028 + 0x10) = *(undefined4 *)(in_stack_00000028 + 0x1c);
  gui_controls::show_control_tweakers();
  toolbar::mei::widgets::impl$26::space();
  lang_lib::lookup();
  toolbar::mei::widgets::impl$26::menu_button();
  toolbar::mei::widgets::impl$37::deref();
  toolbar::mei::core::Node::used();
  if (extraout_AL != '\0') {
    if ((*in_RCX != 0xb) && (uVar4 = *in_RCX - 2, 8 < uVar4 || uVar4 == 4)) {
      plVar3 = (longlong *)in_RCX[6];
      LOCK();
      *plVar3 = *plVar3 + -1;
      UNLOCK();
      if (*plVar3 == 0) {
        alloc::sync::Arc::
        drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,country_core::resources::gui::LoadMenuThumbnailMetrics>,alloc::alloc::Global>
                  ();
      }
    }
    *in_RCX = 7;
    *(undefined1 *)(in_RCX + 1) = 10;
  }
  if ((pbVar2 != (byte *)0x0 && lVar1 != 0) && ((*pbVar2 & 1) != 0)) {
    lang_lib::lookup();
    toolbar::mei::widgets::impl$26::menu_button();
    toolbar::mei::widgets::impl$37::deref();
    toolbar::mei::core::Node::used();
    if (extraout_AL_00 != '\0') {
      country_core::systems::steam::SteamClientResource::input_show_binding_panel();
    }
  }
  return;
}

