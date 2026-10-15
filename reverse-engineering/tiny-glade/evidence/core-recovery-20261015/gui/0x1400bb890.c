
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_3,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong *extraout_RAX_01;
  void *_Dst;
  longlong *in_RCX;
  undefined1 local_2e0 [144];
  undefined1 local_250 [16];
  longlong *local_240;
  int local_210;
  longlong *local_1e0;
  undefined1 *local_1c0;
  undefined *local_1b0;
  undefined8 local_188 [3];
  undefined8 local_170;
  undefined8 local_158;
  undefined8 local_148;
  undefined4 local_140;
  undefined4 local_138;
  undefined8 local_130;
  undefined1 local_128;
  undefined4 local_123;
  undefined3 uStack_11f;
  longlong local_118;
  undefined1 local_110;
  longlong *local_b0;
  undefined1 local_51;
  undefined8 local_50;
  
  local_50 = 0xfffffffffffffffe;
  plVar1 = (longlong *)*in_RCX;
  *(undefined4 *)plVar1[2] = *(undefined4 *)((longlong)plVar1 + 0x1c);
  lVar2 = *plVar1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  local_118 = lVar2 + 0x100;
  *(undefined8 *)((longlong)extraout_RAX + 0xb) = 0x73656e6863695220;
  *(undefined8 *)((longlong)extraout_RAX + 0x13) = 0x302e31206f742073;
  *extraout_RAX = 0x754d206563726f46;
  extraout_RAX[1] = 0x6863695220636973;
  local_158 = 0x1b;
  local_148 = 0x1b;
  local_140 = 0;
  local_130 = 0;
  local_138 = 0;
  local_188[0] = 3;
  local_170 = 6;
  local_128 = 0;
  local_123 = 0;
  uStack_11f = 0;
  local_110 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_240 = *local_240 + -1;
  UNLOCK();
  if (*local_240 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  *(undefined4 *)plVar1[2] = *(undefined4 *)((longlong)plVar1 + 0x1c);
  lVar2 = *plVar1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    local_118 = lVar2 + 0xf6;
    *(undefined8 *)((longlong)extraout_RAX_00 + 7) = 0x6f666e6920636973;
    *extraout_RAX_00 = 0x73756d20776f6853;
    local_158 = 0xf;
    local_148 = 0xf;
    local_140 = 0;
    local_130 = 0;
    local_138 = 0;
    local_188[0] = 3;
    local_170 = 6;
    local_128 = 0;
    local_123 = 0;
    uStack_11f = 0;
    local_110 = 0;
    egui::widgets::button::impl$3::ui();
    LOCK();
    *local_240 = *local_240 + -1;
    UNLOCK();
    if (*local_240 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    lVar2 = in_RCX[1];
    lVar3 = in_RCX[2];
    lVar4 = in_RCX[3];
    lVar5 = in_RCX[4];
    egui::containers::collapsing_header::CollapsingHeader::new<ref_<str_>_>();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 != (longlong *)0x0) {
      *extraout_RAX_01 = lVar2;
      extraout_RAX_01[1] = lVar3;
      extraout_RAX_01[2] = lVar4;
      extraout_RAX_01[3] = lVar5;
      local_51 = 1;
      memcpy(local_250,local_2e0,0x90);
      local_1c0 = &local_51;
      local_1b0 = &DAT_142934930;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst != (void *)0x0) {
        memcpy(_Dst,local_250,0xa8);
        egui::ui::Ui::
        with_layout_dyn<egui::containers::collapsing_header::CollapsingResponse<tuple_<>_>_>();
        memcpy(local_250,local_188,200);
        LOCK();
        *local_b0 = *local_b0 + -1;
        UNLOCK();
        if (*local_b0 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        LOCK();
        *local_1e0 = *local_1e0 + -1;
        UNLOCK();
        if (*local_1e0 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        if (local_210 != 2) {
          LOCK();
          *local_240 = *local_240 + -1;
          UNLOCK();
          if (*local_240 == 0) {
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
  ::alloc::raw_vec::handle_error();
}


