
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_2,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong lVar1;
  char extraout_AL;
  char extraout_AL_00;
  undefined8 *extraout_RAX;
  longlong *in_RCX;
  longlong *local_330;
  longlong *local_88;
  char local_45;
  char local_44;
  
  lVar1 = *in_RCX;
  country_core::resources::history::History::debug_display();
  egui::widgets::label::impl$2::ui();
  LOCK();
  *local_330 = *local_330 + -1;
  UNLOCK();
  if (*local_330 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  country_core::resources::walls::decorator::DecoratorRank::new();
  convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if (local_44 == '\0') {
    if (local_45 != '\0') {
      egui::context::Context::
      write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                ();
      if (extraout_AL != '\0') goto LAB_1400b7a22;
    }
    LOCK();
    *local_88 = *local_88 + -1;
    UNLOCK();
    if (*local_88 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
LAB_1400b7a22:
    LOCK();
    *local_88 = *local_88 + -1;
    UNLOCK();
    if (*local_88 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    **(undefined4 **)(lVar1 + 0x10) = *(undefined4 *)(lVar1 + 0x1c);
    country_core::resources::history::History::debug_set_history_start();
  }
  convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if (local_44 == '\0') {
    if (local_45 != '\0') {
      egui::context::Context::
      write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                ();
      if (extraout_AL_00 != '\0') goto LAB_1400b7b3a;
    }
    LOCK();
    *local_88 = *local_88 + -1;
    UNLOCK();
    if (*local_88 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
LAB_1400b7b3a:
    LOCK();
    *local_88 = *local_88 + -1;
    UNLOCK();
    if (*local_88 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    **(undefined4 **)(lVar1 + 0x10) = *(undefined4 *)(lVar1 + 0x1c);
    country_core::resources::history::History::debug_set_history_start();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    extraout_RAX[2] = 0x6920736565727420;
    extraout_RAX[3] = 0x70616e73206f746e;
    *extraout_RAX = 0x7265732070696b53;
    extraout_RAX[1] = 0x676e697a696c6169;
    *(undefined4 *)(extraout_RAX + 4) = 0x746f6873;
    egui::widgets::button::impl$3::ui();
    LOCK();
    *local_330 = *local_330 + -1;
    UNLOCK();
    if (*local_330 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  ::alloc::raw_vec::handle_error();
}


