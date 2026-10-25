
// WARNING: Restarted to delay deadcode elimination for space: stack

void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_13::closure_env_0,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  undefined *puVar1;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  char *extraout_RAX_02;
  longlong *in_RCX;
  longlong *local_78;
  
  **(undefined4 **)(*in_RCX + 0x10) = *(undefined4 *)(*in_RCX + 0x1c);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *extraout_RAX = 0x6220656c62616e45;
  *(undefined4 *)(extraout_RAX + 1) = 0x73647269;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_78 = *local_78 + -1;
  UNLOCK();
  if (*local_78 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *extraout_RAX_00 = 0x6f707320776f6853;
  *(undefined2 *)(extraout_RAX_00 + 1) = 0x7374;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_78 = *local_78 + -1;
  UNLOCK();
  if (*local_78 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *(undefined8 *)((longlong)extraout_RAX_01 + 5) = 0x7374736163796172;
  *extraout_RAX_01 = 0x79617220776f6853;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_78 = *local_78 + -1;
  UNLOCK();
  if (*local_78 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 != (char *)0x0) {
    builtin_strncpy(extraout_RAX_02,"Show haven population",0x15);
    egui::widgets::button::impl$3::ui();
    LOCK();
    *local_78 = *local_78 + -1;
    UNLOCK();
    if (*local_78 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    puVar1 = __imp__ZN15system_critters25DEBUG_AIRBORNE_BIRD_COUNT17h7a9dbaaecf259204E;
    ::alloc::fmt::format::format_inner();
    egui::widgets::label::impl$2::ui();
    LOCK();
    UNLOCK();
    if (puVar1 == (undefined *)0x1) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    puVar1 = __imp__ZN15system_critters22DEBUG_HAVEN_BIRD_COUNT17hf16b9444df521ebbE;
    ::alloc::fmt::format::format_inner();
    egui::widgets::label::impl$2::ui();
    LOCK();
    UNLOCK();
    if (puVar1 == (undefined *)0x1) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  ::alloc::raw_vec::handle_error();
}


