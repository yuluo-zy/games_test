
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_6,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong lVar1;
  undefined4 *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *in_RCX;
  longlong *local_e8;
  longlong *local_68;
  
  lVar1 = *in_RCX;
  **(undefined4 **)(lVar1 + 0x10) = *(undefined4 *)(lVar1 + 0x1c);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *(undefined2 *)(extraout_RAX + 1) = 0x656c;
  *extraout_RAX = 0x62616e45;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_68 = *local_68 + -1;
  UNLOCK();
  if (*local_68 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (longlong *)0x0) {
    *extraout_RAX_00 = lVar1;
    egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
    LOCK();
    *local_e8 = *local_e8 + -1;
    UNLOCK();
    if (*local_e8 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


