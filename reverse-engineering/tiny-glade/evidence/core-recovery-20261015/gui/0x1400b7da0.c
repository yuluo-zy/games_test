
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_12,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  undefined4 uVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined4 *puVar4;
  longlong lVar5;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong *extraout_RAX_03;
  void *_Dst;
  longlong *in_RCX;
  undefined1 local_2d0 [144];
  undefined1 local_240 [16];
  longlong *local_230;
  int local_200;
  longlong *local_1d0;
  undefined1 *local_1b0;
  undefined *local_1a0;
  undefined8 local_178 [3];
  undefined8 local_160;
  undefined8 local_148;
  undefined8 local_138;
  undefined4 local_130;
  undefined4 local_128;
  undefined8 local_120;
  undefined1 local_118;
  undefined4 local_113;
  undefined3 uStack_10f;
  longlong local_108;
  undefined1 local_100;
  longlong *local_a0;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  plVar2 = (longlong *)*in_RCX;
  lVar3 = in_RCX[1];
  uVar1 = *(undefined4 *)((longlong)plVar2 + 0x1c);
  puVar4 = (undefined4 *)plVar2[2];
  *puVar4 = uVar1;
  lVar5 = *plVar2;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  local_108 = lVar5 + 0xf9;
  *(undefined8 *)((longlong)extraout_RAX + 0xf) = 0x68207265766f6820;
  *(undefined8 *)((longlong)extraout_RAX + 0x17) = 0x746867696c686769;
  *extraout_RAX = 0x20656c6261736944;
  extraout_RAX[1] = 0x206e696172726574;
  local_148 = 0x1f;
  local_138 = 0x1f;
  local_130 = 0;
  local_120 = 0;
  local_128 = 0;
  local_178[0] = 3;
  local_160 = 6;
  local_118 = 0;
  local_113 = 0;
  uStack_10f = 0;
  local_100 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_230 = *local_230 + -1;
  UNLOCK();
  if (*local_230 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  *puVar4 = uVar1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  local_108 = lVar5 + 0xfd;
  *extraout_RAX_00 = 0x20656c6261736944;
  extraout_RAX_00[1] = 0x616e73206c6c6177;
  *(undefined8 *)((longlong)extraout_RAX_00 + 0xd) = 0x676e697070616e73;
  local_148 = 0x15;
  local_138 = 0x15;
  local_130 = 0;
  local_120 = 0;
  local_128 = 0;
  local_178[0] = 3;
  local_160 = 6;
  local_118 = 0;
  local_113 = 0;
  uStack_10f = 0;
  local_100 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_230 = *local_230 + -1;
  UNLOCK();
  if (*local_230 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  *puVar4 = uVar1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  local_108 = lVar5 + 0xfe;
  *extraout_RAX_01 = 0x20656c6261736944;
  extraout_RAX_01[1] = 0x6820726573617265;
  *(undefined8 *)((longlong)extraout_RAX_01 + 0xf) = 0x746867696c676968;
  local_148 = 0x17;
  local_138 = 0x17;
  local_130 = 0;
  local_120 = 0;
  local_128 = 0;
  local_178[0] = 3;
  local_160 = 6;
  local_118 = 0;
  local_113 = 0;
  uStack_10f = 0;
  local_100 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_230 = *local_230 + -1;
  UNLOCK();
  if (*local_230 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  *puVar4 = uVar1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 != (undefined8 *)0x0) {
    local_108 = lVar5 + 0xfa;
    *extraout_RAX_02 = 0x7572622065646948;
    extraout_RAX_02[1] = 0x6976657270206873;
    *(undefined4 *)((longlong)extraout_RAX_02 + 0xf) = 0x73776569;
    local_148 = 0x13;
    local_138 = 0x13;
    local_130 = 0;
    local_120 = 0;
    local_128 = 0;
    local_178[0] = 3;
    local_160 = 6;
    local_118 = 0;
    local_113 = 0;
    uStack_10f = 0;
    local_100 = 0;
    egui::widgets::button::impl$3::ui();
    LOCK();
    *local_230 = *local_230 + -1;
    UNLOCK();
    if (*local_230 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    egui::containers::collapsing_header::CollapsingHeader::new<ref_<str_>_>();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_03 != (longlong *)0x0) {
      *extraout_RAX_03 = (longlong)plVar2;
      extraout_RAX_03[1] = lVar3;
      local_41 = 1;
      memcpy(local_240,local_2d0,0x90);
      local_1b0 = &local_41;
      local_1a0 = &DAT_1429347f0;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst != (void *)0x0) {
        memcpy(_Dst,local_240,0xa8);
        egui::ui::Ui::
        with_layout_dyn<egui::containers::collapsing_header::CollapsingResponse<tuple_<>_>_>();
        memcpy(local_240,local_178,200);
        LOCK();
        *local_a0 = *local_a0 + -1;
        UNLOCK();
        if (*local_a0 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        LOCK();
        *local_1d0 = *local_1d0 + -1;
        UNLOCK();
        if (*local_1d0 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        if (local_200 != 2) {
          LOCK();
          *local_230 = *local_230 + -1;
          UNLOCK();
          if (*local_230 == 0) {
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


