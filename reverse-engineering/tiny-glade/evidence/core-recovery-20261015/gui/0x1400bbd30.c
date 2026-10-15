
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_11,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong lVar1;
  longlong *plVar2;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  void *_Dst;
  longlong *in_RCX;
  undefined4 extraout_XMM0_Da;
  undefined1 local_320 [16];
  longlong *local_310;
  longlong *local_248;
  longlong local_1f8;
  code *local_1f0;
  longlong *local_1e8;
  undefined *local_150;
  undefined8 local_148;
  longlong *local_140;
  undefined8 local_138;
  undefined8 local_130;
  int local_110;
  longlong *local_e0;
  undefined1 *local_c0;
  undefined *local_b0;
  undefined1 local_32;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  lVar1 = *in_RCX;
  country_core::systems::camera_rig::MainCamera::target_vertical_fov_degrees();
  local_150 = (undefined *)CONCAT44(local_150._4_4_,extraout_XMM0_Da);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = (longlong)&local_150;
  extraout_RAX[1] = lVar1;
  egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
  LOCK();
  *local_310 = *local_310 + -1;
  UNLOCK();
  if (*local_310 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  egui::containers::combo_box::ComboBox::from_label<ref_<str_>_>();
  plVar2 = (longlong *)in_RCX[1];
  local_1f8 = *plVar2 + 0x103;
  local_1f0 = country_core::systems::camera_rig::impl$9::fmt;
  local_150 = &DAT_142935040;
  local_148 = 1;
  local_140 = &local_1f8;
  local_138 = 1;
  local_130 = 0;
  local_31 = 1;
  ::alloc::fmt::format::format_inner();
  local_31 = 0;
  egui::containers::combo_box::ComboBox::selected_text<alloc::string::String>();
  memcpy(local_320,&local_150,0x110);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (longlong *)0x0) {
    *extraout_RAX_00 = (longlong)plVar2;
    egui::containers::combo_box::ComboBox::show_ui_dyn<tuple_<>_>();
    LOCK();
    *local_1e8 = *local_1e8 + -1;
    UNLOCK();
    if (*local_1e8 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    lVar1 = in_RCX[2];
    egui::containers::collapsing_header::CollapsingHeader::new<ref_<str_>_>();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 != (longlong *)0x0) {
      *extraout_RAX_01 = (longlong)plVar2;
      extraout_RAX_01[1] = lVar1;
      local_32 = 1;
      memcpy(&local_150,&local_1f8,0x90);
      local_c0 = &local_32;
      local_b0 = &DAT_1429347b0;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst != (void *)0x0) {
        memcpy(_Dst,&local_150,0xa8);
        egui::ui::Ui::
        with_layout_dyn<egui::containers::collapsing_header::CollapsingResponse<tuple_<>_>_>();
        memcpy(&local_150,local_320,200);
        LOCK();
        *local_248 = *local_248 + -1;
        UNLOCK();
        if (*local_248 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        LOCK();
        *local_e0 = *local_e0 + -1;
        UNLOCK();
        if (*local_e0 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        plVar2 = local_140;
        if (local_110 != 2) {
          LOCK();
          *local_140 = *local_140 + -1;
          UNLOCK();
          if (*plVar2 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
        }
        return;
      }
                    // WARNING: Subroutine does not return
      ::alloc::alloc::handle_alloc_error();
    }
                    // WARNING: Subroutine does not return
    ::alloc::alloc::handle_alloc_error();
  }
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


