
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_tod_sequence_entry::closure_env_0,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  undefined4 uVar1;
  longlong lVar2;
  undefined4 *puVar3;
  char *pcVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  char cVar10;
  longlong *local_350;
  longlong *local_1a8;
  char local_165;
  char local_164;
  longlong *local_118;
  char local_d5;
  char local_d4;
  
  convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if ((local_d4 == '\0') &&
     ((local_d5 == '\0' ||
      (egui::context::Context::
       write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                 (), extraout_AL == '\0')))) {
    LOCK();
    *local_118 = *local_118 + -1;
    UNLOCK();
    if (*local_118 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
    LOCK();
    *local_118 = *local_118 + -1;
    UNLOCK();
    if (*local_118 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    puVar3 = (undefined4 *)*in_RCX;
    uVar1 = *(undefined4 *)in_RCX[1];
    *puVar3 = 0;
    puVar3[1] = uVar1;
  }
  pcVar4 = (char *)in_RCX[4];
  if (*pcVar4 != '\0') {
    convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
    egui::widgets::button::Button::opt_image_and_text();
    egui::widgets::button::impl$1::ui();
    if ((local_d4 == '\0') &&
       ((local_d5 == '\0' ||
        (egui::context::Context::
         write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                   (), extraout_AL_01 == '\0')))) {
      LOCK();
      *local_118 = *local_118 + -1;
      UNLOCK();
      if (*local_118 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      cVar10 = '\x01';
      goto LAB_1401448ef;
    }
    LOCK();
    *local_118 = *local_118 + -1;
    UNLOCK();
    if (*local_118 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  cVar10 = '\x01';
  if (local_164 == '\0') {
    if (local_165 == '\x01') {
      egui::context::Context::
      write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                ();
      cVar10 = extraout_AL_00;
      goto LAB_140144883;
    }
    cVar10 = '\0';
    LOCK();
    *local_1a8 = *local_1a8 + -1;
    lVar2 = *local_1a8;
    UNLOCK();
  }
  else {
LAB_140144883:
    LOCK();
    *local_1a8 = *local_1a8 + -1;
    lVar2 = *local_1a8;
    UNLOCK();
  }
  if (lVar2 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
LAB_1401448ef:
  *pcVar4 = cVar10;
  uVar8 = in_RCX[1];
  uVar9 = in_RCX[2];
  uVar5 = in_RCX[3];
  uVar6 = in_RCX[5];
  uVar7 = *in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = uVar8;
    extraout_RAX[1] = uVar9;
    extraout_RAX[2] = uVar5;
    extraout_RAX[3] = uVar6;
    extraout_RAX[4] = pcVar4;
    extraout_RAX[5] = uVar7;
    egui::ui::Ui::with_layout_dyn<tuple_<>_>();
    LOCK();
    *local_350 = *local_350 + -1;
    UNLOCK();
    if (*local_350 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


