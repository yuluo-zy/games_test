// date_spinner @ 0x0x140148b40
// sig: void __cdecl date_spinner(void)
// size: 352


void __cdecl tiny_glade::gui::gui_debug::date_spinner(void)

{
  undefined8 *extraout_RAX;
  undefined8 in_RDX;
  undefined8 local_68;
  
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = in_RDX;
    egui::widgets::drag_value::impl$1::ui();
    LOCK();
    *local_68 = *local_68 + -1;
    UNLOCK();
    if (*local_68 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

