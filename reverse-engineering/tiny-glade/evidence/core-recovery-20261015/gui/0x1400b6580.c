
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_5,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined *puVar4;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  undefined8 *extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  void *_Dst;
  undefined8 *in_RCX;
  byte bVar5;
  byte local_2c0 [144];
  undefined1 local_230 [16];
  longlong *local_220;
  int local_1f0;
  longlong *local_1c0;
  undefined1 *local_1a0;
  undefined *local_190;
  undefined8 local_160 [3];
  undefined8 local_148;
  undefined8 local_130;
  undefined8 local_120;
  undefined4 local_118;
  undefined4 local_110;
  undefined8 local_108;
  undefined1 local_100;
  undefined4 local_fb;
  undefined3 uStack_f7;
  byte *local_f0;
  undefined1 local_e8;
  longlong *local_88;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *(undefined8 *)((longlong)extraout_RAX + 6) = 0x72657469746a2041;
  *extraout_RAX = 0x2041415420657355;
  local_130 = 0xe;
  local_120 = 0xe;
  local_118 = 0;
  local_108 = 0;
  local_110 = 0;
  local_160[0] = 3;
  local_148 = 6;
  local_100 = 0;
  local_fb = 0;
  uStack_f7 = 0;
  local_f0 = 
  __imp__ZN12country_core7systems7backend23prepare_frame_constants14USE_TAA_JITTER17hcb25419c22158c3aE
  ;
  local_e8 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_220 = *local_220 + -1;
  UNLOCK();
  if (*local_220 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *extraout_RAX_00 = 0x4154206563726f46;
  extraout_RAX_00[1] = 0x742d737361702041;
  *(undefined4 *)((longlong)extraout_RAX_00 + 0xf) = 0x75726874;
  local_130 = 0x13;
  local_120 = 0x13;
  local_118 = 0;
  local_108 = 0;
  local_110 = 0;
  local_160[0] = 3;
  local_148 = 6;
  local_100 = 0;
  local_fb = 0;
  uStack_f7 = 0;
  local_f0 = 
  __imp__ZN12country_core7systems11render_loop14compute_passes3taa21FORCE_TAA_PASSTHROUGH17h7d5b5739bd9a7eb5E
  ;
  local_e8 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_220 = *local_220 + -1;
  UNLOCK();
  if (*local_220 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *(undefined8 *)((longlong)extraout_RAX_01 + 5) = 0x466f44207365722d;
  *extraout_RAX_01 = 0x65722d69746c754d;
  local_130 = 0xd;
  local_120 = 0xd;
  local_118 = 0;
  local_108 = 0;
  local_110 = 0;
  local_160[0] = 3;
  local_148 = 6;
  local_100 = 0;
  local_fb = 0;
  uStack_f7 = 0;
  local_f0 = 
  __imp__ZN12country_core7systems11render_loop14compute_passes3dof16USE_DOF_MULTIRES17h1a387e81d61754edE
  ;
  local_e8 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_220 = *local_220 + -1;
  UNLOCK();
  if (*local_220 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *extraout_RAX_02 = 0x6f20657a65657246;
  extraout_RAX_02[1] = 0x6e6f6973756c6363;
  extraout_RAX_02[2] = 0x676e696c6c756320;
  local_130 = 0x18;
  local_120 = 0x18;
  local_118 = 0;
  local_108 = 0;
  local_110 = 0;
  local_160[0] = 3;
  local_148 = 6;
  local_100 = 0;
  local_fb = 0;
  uStack_f7 = 0;
  local_f0 = 
  __imp__ZN12country_core7systems11render_loop9resources24FREEZE_OCCLUSION_CULLING17h90f467abb0e1e73eE
  ;
  local_e8 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_220 = *local_220 + -1;
  UNLOCK();
  if (*local_220 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  plVar1 = (longlong *)*in_RCX;
  lVar2 = *plVar1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  local_f0 = (byte *)(lVar2 + 0x48);
  *extraout_RAX_03 = 0x6d656d20776f6853;
  extraout_RAX_03[1] = 0x616572622079726f;
  *(undefined8 *)((longlong)extraout_RAX_03 + 0xd) = 0x6e776f646b616572;
  local_130 = 0x15;
  local_120 = 0x15;
  local_118 = 0;
  local_108 = 0;
  local_110 = 0;
  local_160[0] = 3;
  local_148 = 6;
  local_100 = 0;
  local_fb = 0;
  uStack_f7 = 0;
  local_e8 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_220 = *local_220 + -1;
  UNLOCK();
  if (*local_220 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  lVar2 = *plVar1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_04 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  local_f0 = (byte *)(lVar2 + 0x49);
  *extraout_RAX_04 = 0x6d656d20776f6853;
  extraout_RAX_04[1] = 0x636f6c622079726f;
  *(undefined2 *)(extraout_RAX_04 + 2) = 0x736b;
  local_130 = 0x12;
  local_120 = 0x12;
  local_118 = 0;
  local_108 = 0;
  local_110 = 0;
  local_160[0] = 3;
  local_148 = 6;
  local_100 = 0;
  local_fb = 0;
  uStack_f7 = 0;
  local_e8 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_220 = *local_220 + -1;
  UNLOCK();
  if (*local_220 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_05 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  *extraout_RAX_05 = 0x7361726320555047;
  extraout_RAX_05[1] = 0x72656b72616d2068;
  *(undefined1 *)(extraout_RAX_05 + 2) = 0x73;
  local_130 = 0x11;
  local_120 = 0x11;
  local_118 = 0;
  local_108 = 0;
  local_110 = 0;
  local_160[0] = 3;
  local_148 = 6;
  local_100 = 0;
  local_fb = 0;
  uStack_f7 = 0;
  local_f0 = __imp__ZN8rhapsody2rg8compiled24ENABLE_GPU_CRASH_MARKERS17h7c576a80ed65beeeE;
  local_e8 = 0;
  egui::widgets::button::impl$3::ui();
  LOCK();
  *local_220 = *local_220 + -1;
  UNLOCK();
  if (*local_220 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  puVar4 = __imp__ZN8rhapsody2rg8compiled25USE_PARALLEL_CB_RECORDING17h0342141fe0fd4c86E;
  bVar5 = *(byte *)(**(longlong **)in_RCX[1] + 0xacb) ^ 1;
  if (*__imp__ZN8rhapsody2rg8compiled25USE_PARALLEL_CB_RECORDING17h0342141fe0fd4c86E != 2) {
    bVar5 = *__imp__ZN8rhapsody2rg8compiled25USE_PARALLEL_CB_RECORDING17h0342141fe0fd4c86E;
  }
  local_2c0[0] = bVar5 & 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_06 != (undefined8 *)0x0) {
    extraout_RAX_06[2] = 0x2072656666756220;
    extraout_RAX_06[3] = 0x6e6964726f636572;
    *extraout_RAX_06 = 0x6c656c6c61726150;
    extraout_RAX_06[1] = 0x646e616d6d6f6320;
    *(undefined1 *)(extraout_RAX_06 + 4) = 0x67;
    local_130 = 0x21;
    local_120 = 0x21;
    local_118 = 0;
    local_108 = 0;
    local_110 = 0;
    local_160[0] = 3;
    local_148 = 6;
    local_100 = 0;
    local_fb = 0;
    uStack_f7 = 0;
    local_f0 = local_2c0;
    local_e8 = 0;
    egui::widgets::button::impl$3::ui();
    LOCK();
    *local_220 = *local_220 + -1;
    UNLOCK();
    if (*local_220 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    if ((bVar5 & 1) != local_2c0[0]) {
      *puVar4 = local_2c0[0];
    }
    uVar3 = in_RCX[2];
    egui::containers::collapsing_header::CollapsingHeader::new<ref_<str_>_>();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_07 != (undefined8 *)0x0) {
      *extraout_RAX_07 = uVar3;
      local_31 = 1;
      memcpy(local_230,local_2c0,0x90);
      local_1a0 = &local_31;
      local_190 = &DAT_142934710;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst != (void *)0x0) {
        memcpy(_Dst,local_230,0xa8);
        egui::ui::Ui::
        with_layout_dyn<egui::containers::collapsing_header::CollapsingResponse<tuple_<>_>_>();
        memcpy(local_230,local_160,200);
        LOCK();
        *local_88 = *local_88 + -1;
        UNLOCK();
        if (*local_88 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        LOCK();
        *local_1c0 = *local_1c0 + -1;
        UNLOCK();
        if (*local_1c0 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        if (local_1f0 != 2) {
          LOCK();
          *local_220 = *local_220 + -1;
          UNLOCK();
          if (*local_220 == 0) {
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


