
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_tod_sequence_entry::closure_0::closure_env_0,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  longlong lVar3;
  undefined4 *puVar4;
  char extraout_AL;
  undefined8 *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  longlong *local_230;
  longlong *local_88;
  char local_45;
  
  uVar1 = *in_RCX;
  uVar2 = in_RCX[3];
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = uVar1;
  extraout_RAX[1] = uVar2;
  egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
  LOCK();
  *local_230 = *local_230 + -1;
  UNLOCK();
  if (*local_230 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  if (*(char *)in_RCX[4] != '\x01') {
    return;
  }
  lVar3 = in_RCX[1];
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (longlong *)0x0) {
    *extraout_RAX_00 = lVar3;
    egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
    LOCK();
    *local_230 = *local_230 + -1;
    UNLOCK();
    if (*local_230 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 != (longlong *)0x0) {
      *extraout_RAX_01 = lVar3 + 0x24;
      egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
      LOCK();
      *local_230 = *local_230 + -1;
      UNLOCK();
      if (*local_230 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_02 != (longlong *)0x0) {
        *extraout_RAX_02 = lVar3 + 0x18;
        egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
        LOCK();
        *local_230 = *local_230 + -1;
        UNLOCK();
        if (*local_230 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_03 != (longlong *)0x0) {
          *extraout_RAX_03 = lVar3 + 0xc;
          egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
          LOCK();
          *local_230 = *local_230 + -1;
          UNLOCK();
          if (*local_230 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_04 != (longlong *)0x0) {
            *extraout_RAX_04 = lVar3 + 0x30;
            egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
            LOCK();
            *local_230 = *local_230 + -1;
            UNLOCK();
            if (*local_230 == 0) {
              ::alloc::sync::Arc::
              drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                        ();
            }
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_05 != (longlong *)0x0) {
              *extraout_RAX_05 = lVar3 + 0x34;
              egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
              LOCK();
              *local_230 = *local_230 + -1;
              UNLOCK();
              if (*local_230 == 0) {
                ::alloc::sync::Arc::
                drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                          ();
              }
              std::alloc::__default_lib_allocator::__rust_alloc();
              if (extraout_RAX_06 != (longlong *)0x0) {
                *extraout_RAX_06 = lVar3 + 0x40;
                egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
                LOCK();
                *local_230 = *local_230 + -1;
                UNLOCK();
                if (*local_230 == 0) {
                  ::alloc::sync::Arc::
                  drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                            ();
                }
                convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
                egui::widgets::button::Button::opt_image_and_text();
                egui::widgets::button::impl$1::ui();
                if ((local_45 != '\0') &&
                   (egui::context::Context::
                    write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::double_clicked_by::closure_env_0>_>
                              (), extraout_AL != '\0')) {
                  LOCK();
                  *local_88 = *local_88 + -1;
                  UNLOCK();
                  if (*local_88 == 0) {
                    ::alloc::sync::Arc::
                    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                              ();
                  }
                  puVar4 = (undefined4 *)in_RCX[5];
                  uVar1 = *(undefined8 *)in_RCX[2];
                  *puVar4 = 1;
                  *(undefined8 *)(puVar4 + 2) = uVar1;
                  return;
                }
                LOCK();
                *local_88 = *local_88 + -1;
                UNLOCK();
                if (*local_88 != 0) {
                  return;
                }
                ::alloc::sync::Arc::
                drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                          ();
                return;
              }
            }
          }
        }
      }
    }
  }
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


