
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_4,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong *plVar1;
  char extraout_AL;
  longlong *extraout_RAX;
  longlong *in_RCX;
  longlong *local_2a0;
  longlong *local_80;
  char local_3d;
  char local_3c;
  
  plVar1 = (longlong *)*in_RCX;
  if (*plVar1 == 0) {
    return;
  }
  convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if ((local_3c == '\0') &&
     ((local_3d == '\0' ||
      (egui::context::Context::
       write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                 (), extraout_AL == '\0')))) {
    LOCK();
    *local_80 = *local_80 + -1;
    UNLOCK();
    if (*local_80 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
    LOCK();
    *local_80 = *local_80 + -1;
    UNLOCK();
    if (*local_80 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    *(undefined4 *)plVar1[2] = *(undefined4 *)((longlong)plVar1 + 0x1c);
    country_core::resources::logo_animation::LogoAnimation::play();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (longlong *)0x0) {
    *extraout_RAX = (longlong)plVar1;
    egui::ui::Ui::with_layout_dyn<tuple_<>_>();
    LOCK();
    *local_2a0 = *local_2a0 + -1;
    UNLOCK();
    if (*local_2a0 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


