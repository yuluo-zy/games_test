
void __cdecl tiny_glade::gui::gui_debug::do_camera_snapshot_ui(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  char extraout_AL;
  longlong *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RDX;
  uint uVar4;
  undefined8 *puVar5;
  longlong *in_R8;
  longlong lVar6;
  undefined8 *local_420;
  ulonglong uStack_418;
  longlong *plStack_410;
  undefined8 local_408;
  longlong lStack_400;
  longlong *local_3f8;
  undefined8 uStack_3f0;
  undefined8 local_3e8;
  undefined8 uStack_3e0;
  undefined8 local_3d8;
  ulonglong uStack_3d0;
  ulonglong local_3c8;
  undefined8 uStack_3c0;
  undefined8 local_3b8;
  undefined8 uStack_3b0;
  ulonglong local_2d8;
  longlong *plStack_2d0;
  longlong *local_2c8;
  longlong lStack_2c0;
  longlong *local_2b8;
  undefined8 *puStack_2b0;
  undefined8 local_2a8;
  undefined8 *puStack_2a0;
  ulonglong local_298;
  ulonglong uStack_290;
  ulonglong local_288;
  longlong *plStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined4 local_268;
  undefined4 uStack_264;
  undefined3 uStack_260;
  longlong *local_258;
  char local_215;
  char local_214;
  ulonglong local_208;
  undefined8 uStack_200;
  longlong *local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 *puStack_1e0;
  ulonglong uStack_1d8;
  longlong *plStack_1d0;
  ulonglong uStack_1c8;
  ulonglong local_1c0;
  longlong *plStack_1b8;
  longlong *local_1b0;
  undefined1 uStack_1a8;
  undefined4 uStack_1a7;
  undefined4 uStack_1a3;
  undefined3 uStack_19f;
  undefined4 uStack_19c;
  undefined8 uStack_198;
  ulonglong local_190;
  ulonglong uStack_188;
  undefined8 local_180;
  undefined1 uStack_178;
  undefined7 uStack_177;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined8 local_168;
  undefined8 local_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 local_140;
  undefined4 local_138;
  undefined4 local_130;
  undefined4 uStack_12c;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined1 local_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined1 local_108;
  longlong *local_100;
  undefined *local_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_d8;
  undefined4 local_c8;
  undefined1 local_c4;
  longlong *local_60;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = (longlong)in_RDX + 0xec;
  local_60 = extraout_RAX;
  egui::widget_text::impl$6::default();
  local_168 = 0;
  local_160 = 1;
  local_158 = 0;
  uStack_150 = 0;
  local_148 = 1;
  local_140 = 0;
  uStack_418 = local_2d8;
  plStack_410 = plStack_2d0;
  local_408 = local_2c8;
  lStack_400 = lStack_2c0;
  local_3f8 = local_2b8;
  uStack_3f0 = puStack_2b0;
  local_3e8 = local_2a8;
  uStack_3e0 = puStack_2a0;
  local_3d8 = local_298;
  uStack_3d0 = uStack_290;
  local_3c8 = local_288;
  uStack_3c0 = plStack_280;
  local_3b8 = local_278;
  uStack_3b0 = uStack_270;
  local_f0 = 0;
  uStack_e8 = 0;
  local_208 = 0;
  local_1f8 = (longlong *)0x0;
  local_1e8 = 0;
  puStack_1e0 = local_420;
  uStack_1d8 = local_2d8;
  plStack_1d0 = plStack_2d0;
  uStack_1c8 = (ulonglong)local_2c8;
  local_1c0 = lStack_2c0;
  plStack_1b8 = local_2b8;
  local_1b0 = puStack_2b0;
  uStack_1a8 = (undefined1)local_2a8;
  uStack_1a7 = (undefined4)((ulonglong)local_2a8 >> 8);
  uStack_1a3._0_3_ = (undefined3)((ulonglong)local_2a8 >> 0x28);
  uStack_1a3._3_1_ = (byte)puStack_2a0;
  uStack_19f = (undefined3)((ulonglong)puStack_2a0 >> 8);
  uStack_19c = (undefined4)((ulonglong)puStack_2a0 >> 0x20);
  uStack_198 = local_298;
  local_190 = uStack_290;
  uStack_188 = local_288;
  local_180 = plStack_280;
  uStack_178 = (undefined1)local_278;
  uStack_177 = (undefined7)((ulonglong)local_278 >> 8);
  uStack_170 = (undefined4)uStack_270;
  uStack_16c = (undefined4)((ulonglong)uStack_270 >> 0x20);
  local_138 = 2;
  local_130 = 0;
  uStack_12c = 0;
  uStack_128 = 0;
  uStack_124 = 0x40240000;
  local_120 = 0;
  local_118 = 0x3eb0c6f7a0b5ed8d;
  uStack_110 = 0x7ff0000000000000;
  local_108 = 0;
  local_100 = local_60;
  local_f8 = &anon_aa39661ce9781c29d9a9638471876d74_42_llvm_6754927731931653273;
  local_d8 = 0;
  local_c8 = 0x10101;
  local_c4 = 2;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *extraout_RAX_00 = 0x6c6f707265746e49;
  extraout_RAX_00[1] = 0x7053206e6f697461;
  *(undefined4 *)((longlong)extraout_RAX_00 + 0xf) = 0x64656570;
  uStack_264 = 0;
  uStack_260 = 0;
  uVar4 = (uint)uStack_1d8;
  lVar6 = 0;
  if ((uVar4 & 0xfffffffe) == 4) {
    lVar6 = uStack_1d8 - 3;
  }
  local_60 = extraout_RAX_00;
  if (lVar6 == 0) {
    if (CONCAT35((undefined3)uStack_1a3,CONCAT41(uStack_1a7,uStack_1a8)) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((uVar4 != 3) && (1 < uVar4)) {
      LOCK();
      *plStack_1d0 = *plStack_1d0 + -1;
      UNLOCK();
      if (*plStack_1d0 == 0) {
        alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
      }
    }
    if ((local_1c0 != 6) && (4 < (uint)local_1c0)) {
      LOCK();
      *plStack_1b8 = *plStack_1b8 + -1;
      UNLOCK();
      if (*plStack_1b8 == 0) {
        alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
      }
    }
  }
  else if (lVar6 == 1) {
    if (plStack_1d0 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    core::ptr::
    drop_in_place<alloc::vec::Vec<epaint::text::text_layout_types::LayoutSection,alloc::alloc::Global>_>
              ();
  }
  else {
    LOCK();
    *plStack_1d0 = *plStack_1d0 + -1;
    UNLOCK();
    if (*plStack_1d0 == 0) {
      alloc::sync::Arc::drop_slow<epaint::text::text_layout_types::Galley,alloc::alloc::Global>();
    }
  }
  uStack_1d8 = 3;
  plStack_1d0 = local_420;
  uStack_1c8 = uStack_418;
  local_1c0 = 6;
  plStack_1b8 = (longlong *)local_2d8;
  local_1b0 = plStack_2d0;
  uStack_1a8 = 0x13;
  uStack_1a7 = 0;
  uStack_1a3._3_1_ = (byte)local_60;
  uStack_1a3 = (uint)uStack_1a3._3_1_ << 0x18;
  uStack_19f = (undefined3)((ulonglong)local_60 >> 8);
  uStack_19c = (undefined4)((ulonglong)local_60 >> 0x20);
  uStack_198 = 0x13;
  local_190 = local_190 & 0xffffffff00000000;
  uStack_188 = uStack_188 & 0xffffffff00000000;
  local_180 = 0;
  uStack_178 = 0;
  uStack_177 = (undefined7)CONCAT44(uStack_264,local_268);
  uStack_170 = CONCAT31(uStack_260,uStack_264._3_1_);
  memcpy(&local_420,&local_208,0x148);
  egui::widgets::slider::impl$2::ui();
  LOCK();
  *local_1f8 = *local_1f8 + -1;
  UNLOCK();
  if (*local_1f8 == 0) {
    alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  local_420 = (undefined8 *)0x4;
  local_49 = 0;
  core::convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  local_278 = CONCAT35((undefined3)uStack_1a3,CONCAT41(uStack_1a7,uStack_1a8));
  uStack_270 = CONCAT44(uStack_19c,CONCAT31(uStack_19f,uStack_1a3._3_1_));
  local_288 = (ulonglong)plStack_1b8;
  plStack_280 = local_1b0;
  local_298 = uStack_1c8;
  uStack_290 = local_1c0;
  local_2a8 = uStack_1d8;
  puStack_2a0 = plStack_1d0;
  local_2b8 = (longlong *)local_1e8;
  puStack_2b0 = puStack_1e0;
  local_2c8 = local_1f8;
  lStack_2c0 = uStack_1f0;
  local_2d8 = local_208;
  plStack_2d0 = (longlong *)uStack_200;
  local_49 = 1;
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if (local_214 == '\0') {
    local_208 = local_208 & 0xffffffffffffff00;
    if (local_215 != '\0') {
      egui::context::Context::
      write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                ();
      if (extraout_AL != '\0') goto LAB_14014915d;
    }
    LOCK();
    *local_258 = *local_258 + -1;
    UNLOCK();
    if (*local_258 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
LAB_14014915d:
    LOCK();
    *local_258 = *local_258 + -1;
    UNLOCK();
    if (*local_258 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    *in_RDX = 1;
  }
  puVar1 = *(undefined8 **)((longlong)in_R8 + 0x10);
  if (puVar1 != (undefined8 *)0x0) {
    puVar5 = (undefined8 *)0x0;
    local_60 = in_R8;
    do {
      local_420 = puVar5;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_01 = &local_420;
      extraout_RAX_01[1] = (longlong)in_R8 + 0x20;
      extraout_RAX_01[2] = in_RDX;
      egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
      LOCK();
      *local_1f8 = *local_1f8 + -1;
      UNLOCK();
      if (*local_1f8 == 0) {
        alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      puVar5 = (undefined8 *)((longlong)puVar5 + 1);
    } while (puVar1 != puVar5);
    in_R8 = local_60;
    if (*(ulonglong *)((longlong)local_60 + 0x10) < 2) {
      if (*(ulonglong *)((longlong)local_60 + 0x10) == 0) goto LAB_1401495bd;
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar2 = uStack_198;
      if (extraout_RAX_04 == (undefined8 *)0x0) {
LAB_14014967b:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      *extraout_RAX_04 = 0x73206172656d6143;
      extraout_RAX_04[1] = 0x20746f687370616e;
      *(undefined8 *)((longlong)extraout_RAX_04 + 0xe) = 0x2e64657661732074;
      uStack_1a8 = 0;
      uStack_1a3 = 0;
      uStack_19f = 0;
      local_208 = 3;
      uStack_1f0 = 6;
      uStack_1d8 = 0x16;
      uStack_1c8 = 0x16;
      local_1c0 = local_1c0 & 0xffffffff00000000;
      plStack_1b8 = (longlong *)((ulonglong)plStack_1b8 & 0xffffffff00000000);
      local_1b0 = (longlong *)0x0;
      uStack_198 = CONCAT53(uStack_198._3_5_,0x20200);
      uVar3 = uStack_198;
      uStack_198._6_2_ = SUB82(uVar2,6);
      uStack_198._0_5_ = (undefined5)uVar3;
      uStack_198._0_6_ = CONCAT15(2,(undefined5)uStack_198);
      plStack_1d0 = extraout_RAX_04;
      egui::widgets::label::impl$2::ui();
      LOCK();
      *plStack_410 = *plStack_410 + -1;
      UNLOCK();
      if (*plStack_410 == 0) {
        alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_05 == (undefined8 *)0x0) goto LAB_14014967b;
      *extraout_RAX_05 = 0x3846207373657250;
      extraout_RAX_05[1] = 0x6f736572206f7420;
      *(undefined8 *)((longlong)extraout_RAX_05 + 0xe) = 0x2e74692065726f73;
      uStack_1d8 = 0x16;
      uStack_1c8 = 0x16;
      plStack_1d0 = extraout_RAX_05;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar2 = uStack_198;
      if (extraout_RAX_02 == (undefined8 *)0x0) goto LAB_14014967b;
      *extraout_RAX_02 = 0x73206172656d6143;
      extraout_RAX_02[1] = 0x20746f687370616e;
      *(undefined8 *)((longlong)extraout_RAX_02 + 0xe) = 0x2e64657661732074;
      uStack_1a8 = 0;
      uStack_1a3 = 0;
      uStack_19f = 0;
      local_208 = 3;
      uStack_1f0 = 6;
      uStack_1d8 = 0x16;
      uStack_1c8 = 0x16;
      local_1c0 = local_1c0 & 0xffffffff00000000;
      plStack_1b8 = (longlong *)((ulonglong)plStack_1b8 & 0xffffffff00000000);
      local_1b0 = (longlong *)0x0;
      uStack_198 = CONCAT53(uStack_198._3_5_,0x20200);
      uVar3 = uStack_198;
      uStack_198._6_2_ = SUB82(uVar2,6);
      uStack_198._0_5_ = (undefined5)uVar3;
      uStack_198._0_6_ = CONCAT15(2,(undefined5)uStack_198);
      egui::widgets::label::impl$2::ui();
      LOCK();
      *plStack_410 = *plStack_410 + -1;
      UNLOCK();
      if (*plStack_410 == 0) {
        alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      extraout_RAX_03[2] = 0x6577746562206863;
      extraout_RAX_03[3] = 0x7370616e73206e65;
      *extraout_RAX_03 = 0x3846207373657250;
      extraout_RAX_03[1] = 0x74697773206f7420;
      *(undefined8 *)((longlong)extraout_RAX_03 + 0x1d) = 0x2e73746f68737061;
      uStack_1d8 = 0x25;
      uStack_1c8 = 0x25;
      plStack_1d0 = extraout_RAX_03;
    }
    uVar2 = uStack_198;
    uStack_1a3 = 0;
    uStack_19f = 0;
    uStack_1a8 = 0;
    uStack_1f0 = 6;
    local_208 = 3;
    local_1c0 = local_1c0 & 0xffffffff00000000;
    plStack_1b8 = (longlong *)((ulonglong)plStack_1b8 & 0xffffffff00000000);
    local_1b0 = (longlong *)0x0;
    uStack_198 = CONCAT53(uStack_198._3_5_,0x20200);
    uVar3 = uStack_198;
    uStack_198._6_2_ = SUB82(uVar2,6);
    uStack_198._0_5_ = (undefined5)uVar3;
    uStack_198._0_6_ = CONCAT15(2,(undefined5)uStack_198);
    egui::widgets::label::impl$2::ui();
    LOCK();
    *plStack_410 = *plStack_410 + -1;
    UNLOCK();
    if (*plStack_410 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
LAB_1401495bd:
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_06 != (longlong *)0x0) {
    *extraout_RAX_06 = (longlong)in_R8;
    extraout_RAX_06[1] = (longlong)in_RDX;
    egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
    LOCK();
    *local_1f8 = *local_1f8 + -1;
    UNLOCK();
    if (*local_1f8 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}


