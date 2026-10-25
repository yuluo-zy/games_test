
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_10,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong *plVar1;
  longlong *plVar2;
  longlong **pplVar3;
  undefined8 uVar4;
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  char *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong lVar5;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *in_RCX;
  uint uVar6;
  undefined1 *puVar7;
  longlong *local_360;
  longlong **pplStack_358;
  longlong *local_350;
  longlong *plStack_348;
  longlong *local_340;
  undefined8 uStack_338;
  longlong *local_330;
  longlong *plStack_328;
  ulonglong local_320;
  longlong *plStack_318;
  longlong **local_310;
  undefined8 uStack_308;
  undefined8 local_300;
  undefined8 uStack_2f8;
  longlong *local_2f0;
  longlong *plStack_2e8;
  longlong **local_2e0;
  longlong *plStack_2d8;
  longlong *local_2d0;
  longlong *plStack_2c8;
  undefined8 local_2c0;
  longlong *plStack_2b8;
  longlong *local_2b0;
  undefined8 uStack_2a8;
  undefined8 local_2a0;
  longlong **pplStack_298;
  undefined1 local_290;
  undefined4 uStack_28f;
  undefined4 uStack_28b;
  undefined3 uStack_287;
  undefined4 uStack_284;
  undefined8 local_280;
  longlong *local_1a8;
  longlong *plStack_1a0;
  longlong **local_198;
  longlong *plStack_190;
  longlong *local_188;
  longlong *plStack_180;
  longlong *local_178;
  longlong **pplStack_170;
  longlong *local_168;
  longlong *plStack_160;
  longlong *local_158;
  longlong **pplStack_150;
  undefined1 local_148;
  undefined4 uStack_147;
  undefined4 uStack_143;
  undefined3 uStack_13f;
  undefined4 uStack_13c;
  undefined8 local_138;
  longlong *local_130;
  longlong **pplStack_128;
  undefined8 local_120;
  undefined1 uStack_118;
  undefined7 uStack_117;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined4 local_d8;
  undefined4 local_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined1 local_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined1 local_a8;
  longlong *local_a0;
  undefined *local_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_78;
  undefined4 local_68;
  undefined1 local_64;
  undefined4 local_53;
  undefined4 uStack_4f;
  undefined3 uStack_4b;
  longlong *local_48;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  **(undefined4 **)(*in_RCX + 0x10) = *(undefined4 *)(*in_RCX + 0x1c);
  country_core::resources::image_formation::ImageFormationConfigs::current_mut();
  plVar1 = *(longlong **)in_RCX[1];
  if (SBORROW8(0,*plVar1)) {
    local_2f0 = (longlong *)((ulonglong)local_2f0 & 0xffffffffffffff00);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (longlong *)0x0) goto LAB_1400bfbf4;
    *extraout_RAX_00 = (longlong)&local_2f0;
    egui::ui::Ui::scope_dyn<tuple_<>_>();
    LOCK();
    *local_198 = (longlong *)((longlong)*local_198 + -1);
    plVar1 = *local_198;
    UNLOCK();
joined_r0x0001400bf3f3:
    if (plVar1 == (longlong *)0x0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
    plVar2 = (longlong *)in_RCX[2];
    *(undefined4 *)plVar2[2] = *(undefined4 *)((longlong)plVar2 + 0x1c);
    lVar5 = *plVar2;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (char *)0x0) goto LAB_1400bfc03;
    local_138 = lVar5 + 0xc1;
    builtin_strncpy(extraout_RAX_01,"Use color management",0x14);
    local_178 = (longlong *)0x14;
    local_168 = (longlong *)&DAT_00000014;
    plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
    pplStack_150 = (longlong **)0x0;
    local_158 = (longlong *)((ulonglong)local_158 & 0xffffffff00000000);
    local_1a8 = (longlong *)0x3;
    plStack_190 = (longlong *)&DAT_00000006;
    local_148 = 0;
    uStack_143 = 0;
    uStack_13f = 0;
    local_130 = (longlong *)((ulonglong)local_130 & 0xffffffffffffff00);
    egui::widgets::button::impl$3::ui();
    LOCK();
    *local_2e0 = (longlong *)((longlong)*local_2e0 + -1);
    UNLOCK();
    if (*local_2e0 == (longlong *)0x0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    if (*(char *)(*plVar2 + 0xc1) != '\0') {
      pplStack_358 = (longlong **)::alloc::string::impl$23::fmt;
      local_2f0 = (longlong *)&DAT_1429352f0;
      plStack_2e8 = (longlong *)0x1;
      local_2e0 = &local_360;
      plStack_2d8 = (longlong *)0x1;
      local_2d0 = (longlong *)0x0;
      local_360 = plVar1;
      ::alloc::fmt::format::format_inner();
      lVar5 = local_138;
      local_148 = 0;
      uStack_143 = 0;
      uStack_13f = 0;
      local_1a8 = (longlong *)0x3;
      plStack_190 = (longlong *)&DAT_00000006;
      plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
      local_158 = (longlong *)((ulonglong)local_158 & 0xffffffff00000000);
      pplStack_150 = (longlong **)0x0;
      local_138 = CONCAT53(local_138._3_5_,0x20200);
      uVar4 = local_138;
      local_138._6_2_ = SUB82(lVar5,6);
      local_138._0_6_ = CONCAT15(2,(int5)uVar4);
      egui::widgets::label::impl$2::ui();
      pplVar3 = local_2e0;
      LOCK();
      *local_2e0 = (longlong *)((longlong)*local_2e0 + -1);
      plVar1 = *pplVar3;
      UNLOCK();
      goto joined_r0x0001400bf3f3;
    }
  }
  local_2f0 = (longlong *)0x40c00000;
  plStack_2e8 = (longlong *)CONCAT71(plStack_2e8._1_7_,2);
  egui::widgets::separator::impl$2::ui();
  LOCK();
  *local_198 = (longlong *)((longlong)*local_198 + -1);
  UNLOCK();
  if (*local_198 == (longlong *)0x0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
LAB_1400bfc03:
                    // WARNING: Subroutine does not return
    ::alloc::raw_vec::handle_error();
  }
  local_138 = extraout_RAX + 0x28;
  *extraout_RAX_02 = 0x6320737361707942;
  extraout_RAX_02[1] = 0x61726720726f6c6f;
  *(undefined4 *)(extraout_RAX_02 + 2) = 0x676e6964;
  local_178 = (longlong *)0x14;
  local_168 = (longlong *)0x14;
  plStack_160 = (longlong *)((ulonglong)plStack_160 & 0xffffffff00000000);
  pplStack_150 = (longlong **)0x0;
  local_158 = (longlong *)((ulonglong)local_158 & 0xffffffff00000000);
  local_1a8 = (longlong *)0x3;
  plStack_190 = (longlong *)0x6;
  local_148 = 0;
  uStack_143 = 0;
  uStack_13f = 0;
  local_130 = (longlong *)((ulonglong)local_130 & 0xffffffffffffff00);
  egui::widgets::button::impl$3::ui();
  pplVar3 = local_2e0;
  LOCK();
  *local_2e0 = (longlong *)((longlong)*local_2e0 + -1);
  UNLOCK();
  if (*pplVar3 == (longlong *)0x0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  egui::containers::combo_box::ComboBox::from_label<ref_<str_>_>();
  ::alloc::string::impl$6::clone();
  local_290 = 0;
  uStack_28b = 0;
  uStack_287 = 0;
  local_2f0 = (longlong *)0x3;
  plStack_2d8 = (longlong *)&DAT_00000006;
  uStack_2a8 = (ulonglong)uStack_2a8._4_4_ << 0x20;
  local_2a0 = (longlong *)((ulonglong)local_2a0._4_4_ << 0x20);
  pplStack_298 = (longlong **)0x0;
  uVar6 = (uint)local_1a8;
  lVar5 = 0;
  if ((uVar6 & 0xfffffffe) == 4) {
    lVar5 = (longlong)local_1a8 + -3;
  }
  if (lVar5 == 0) {
    if (local_178 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((uVar6 != 3) && (1 < uVar6)) {
      LOCK();
      *plStack_1a0 = *plStack_1a0 + -1;
      UNLOCK();
      if (*plStack_1a0 == 0) {
        ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
      }
    }
    if ((plStack_190 != (longlong *)0x6) && (4 < (uint)plStack_190)) {
      LOCK();
      *local_188 = *local_188 + -1;
      UNLOCK();
      if (*local_188 == 0) {
        ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
      }
    }
  }
  else if (lVar5 == 1) {
    if (plStack_1a0 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    ptr::
    drop_in_place<alloc::vec::Vec<epaint::text::text_layout_types::LayoutSection,alloc::alloc::Global>_>
              ();
  }
  else {
    LOCK();
    *plStack_1a0 = *plStack_1a0 + -1;
    UNLOCK();
    if (*plStack_1a0 == 0) {
      ::alloc::sync::Arc::drop_slow<epaint::text::text_layout_types::Galley,alloc::alloc::Global>();
    }
  }
  local_148 = local_290;
  uStack_147 = uStack_28f;
  uStack_143 = uStack_28b;
  uStack_13f = uStack_287;
  uStack_13c = uStack_284;
  local_158 = local_2a0;
  pplStack_150 = pplStack_298;
  local_168 = local_2b0;
  plStack_160 = (longlong *)uStack_2a8;
  local_178 = (longlong *)local_2c0;
  pplStack_170 = (longlong **)plStack_2b8;
  local_188 = local_2d0;
  plStack_180 = plStack_2c8;
  local_198 = local_2e0;
  plStack_190 = plStack_2d8;
  local_1a8 = local_2f0;
  plStack_1a0 = plStack_2e8;
  memcpy(&local_2f0,&local_1a8,0x110);
  lVar5 = in_RCX[3];
  memcpy(&local_1a8,&local_2f0,0x110);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_03 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    ::alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX_03 = lVar5;
  extraout_RAX_03[1] = extraout_RAX;
  egui::containers::combo_box::ComboBox::show_ui_dyn<tuple_<>_>();
  LOCK();
  *local_350 = *local_350 + -1;
  UNLOCK();
  if (*local_350 == 0) {
    ::alloc::sync::Arc::
    drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
              ();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_04 != (longlong *)0x0) {
    *extraout_RAX_04 = extraout_RAX + 0x18;
    local_48 = extraout_RAX_04;
    egui::widget_text::impl$6::default();
    local_108 = 0;
    local_100 = 1;
    local_f8 = 0;
    uStack_f0 = 0;
    local_e8 = 1;
    local_e0 = 0;
    plStack_2e8 = local_360;
    local_2e0 = pplStack_358;
    plStack_2d8 = local_350;
    local_2d0 = plStack_348;
    plStack_2c8 = local_340;
    local_2c0 = uStack_338;
    plStack_2b8 = local_330;
    local_2b0 = plStack_328;
    uStack_2a8 = local_320;
    local_2a0 = plStack_318;
    pplStack_298 = local_310;
    local_290 = (undefined1)uStack_308;
    uStack_28f = (undefined4)((ulonglong)uStack_308 >> 8);
    uStack_28b._0_3_ = (undefined3)((ulonglong)uStack_308 >> 0x28);
    uStack_28b._3_1_ = (undefined1)local_300;
    uStack_287 = (undefined3)((ulonglong)local_300 >> 8);
    uStack_284 = (undefined4)((ulonglong)local_300 >> 0x20);
    local_280 = uStack_2f8;
    local_90 = 0;
    uStack_88 = 0;
    local_1a8 = (longlong *)0x0;
    local_198 = (longlong **)0x0;
    local_188 = (longlong *)0x0;
    plStack_180 = local_2f0;
    local_178 = local_360;
    pplStack_170 = pplStack_358;
    local_168 = local_350;
    plStack_160 = plStack_348;
    local_158 = local_340;
    pplStack_150 = (longlong **)uStack_338;
    local_148 = SUB81(local_330,0);
    uStack_147 = (undefined4)((ulonglong)local_330 >> 8);
    uStack_143._0_3_ = (undefined3)((ulonglong)local_330 >> 0x28);
    uStack_143._3_1_ = (byte)plStack_328;
    uStack_13f = (undefined3)((ulonglong)plStack_328 >> 8);
    uStack_13c = (undefined4)((ulonglong)plStack_328 >> 0x20);
    local_138 = local_320;
    local_130 = plStack_318;
    pplStack_128 = local_310;
    uStack_117 = (undefined7)((ulonglong)local_300 >> 8);
    uStack_110 = (undefined4)uStack_2f8;
    uStack_10c = (undefined4)((ulonglong)uStack_2f8 >> 0x20);
    local_d8 = 2;
    local_d0 = 0;
    uStack_cc = 0;
    uStack_c8 = 0;
    uStack_c4 = 0x3ff00000;
    local_c0 = 0;
    local_b8 = 0x3eb0c6f7a0b5ed8d;
    uStack_b0 = 0x7ff0000000000000;
    local_a8 = 0;
    local_a0 = local_48;
    local_98 = &anon_aa39661ce9781c29d9a9638471876d74_42_llvm_6754927731931653273;
    local_78 = 0;
    local_68 = 0x10101;
    local_64 = 2;
    uStack_118 = uStack_28b._3_1_;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_05 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      ::alloc::raw_vec::handle_error();
    }
    *extraout_RAX_05 = 0x657274732054554c;
    *(undefined4 *)(extraout_RAX_05 + 1) = 0x6874676e;
    uStack_4b = 0;
    uStack_4f = 0;
    uVar6 = (uint)local_178;
    puVar7 = (undefined1 *)0x0;
    if ((uVar6 & 0xfffffffe) == 4) {
      puVar7 = (undefined1 *)((longlong)local_178 + -3);
    }
    local_48 = extraout_RAX_05;
    if (puVar7 == (undefined1 *)0x0) {
      if (CONCAT35((undefined3)uStack_143,CONCAT41(uStack_147,local_148)) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if ((uVar6 != 3) && (1 < uVar6)) {
        LOCK();
        *pplStack_170 = (longlong *)((longlong)*pplStack_170 + -1);
        UNLOCK();
        if (*pplStack_170 == (longlong *)0x0) {
          ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
        }
      }
      if ((plStack_160 != (longlong *)&DAT_00000006) && (4 < (uint)plStack_160)) {
        LOCK();
        *local_158 = *local_158 + -1;
        UNLOCK();
        if (*local_158 == 0) {
          ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
        }
      }
    }
    else if (puVar7 == (undefined1 *)0x1) {
      if (pplStack_170 != (longlong **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      ptr::
      drop_in_place<alloc::vec::Vec<epaint::text::text_layout_types::LayoutSection,alloc::alloc::Global>_>
                ();
    }
    else {
      LOCK();
      *pplStack_170 = (longlong *)((longlong)*pplStack_170 + -1);
      UNLOCK();
      if (*pplStack_170 == (longlong *)0x0) {
        ::alloc::sync::Arc::drop_slow<epaint::text::text_layout_types::Galley,alloc::alloc::Global>
                  ();
      }
    }
    local_178 = (longlong *)0x3;
    pplStack_170 = (longlong **)local_2f0;
    local_168 = plStack_2e8;
    plStack_160 = (longlong *)0x6;
    local_158 = local_360;
    pplStack_150 = pplStack_358;
    local_148 = 0xc;
    uStack_147 = 0;
    uStack_143._3_1_ = (byte)local_48;
    uStack_143 = (uint)uStack_143._3_1_ << 0x18;
    uStack_13f = (undefined3)((ulonglong)local_48 >> 8);
    uStack_13c = (undefined4)((ulonglong)local_48 >> 0x20);
    local_138 = 0xc;
    local_130 = (longlong *)((ulonglong)local_130 & 0xffffffff00000000);
    pplStack_128 = (longlong **)((ulonglong)pplStack_128 & 0xffffffff00000000);
    local_120 = 0;
    uStack_118 = 0;
    uStack_117 = (undefined7)CONCAT44(uStack_4f,local_53);
    uStack_110 = CONCAT31(uStack_4b,uStack_4f._3_1_);
    memcpy(&local_2f0,&local_1a8,0x148);
    egui::widgets::slider::impl$2::ui();
    LOCK();
    *local_198 = (longlong *)((longlong)*local_198 + -1);
    UNLOCK();
    if (*local_198 == (longlong *)0x0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_06 != (longlong *)0x0) {
      *extraout_RAX_06 = extraout_RAX + 0x1c;
      egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
      LOCK();
      *local_198 = (longlong *)((longlong)*local_198 + -1);
      UNLOCK();
      if (*local_198 == (longlong *)0x0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_07 != (longlong *)0x0) {
        *extraout_RAX_07 = extraout_RAX + 0x20;
        egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
        LOCK();
        *local_198 = (longlong *)((longlong)*local_198 + -1);
        UNLOCK();
        if (*local_198 == (longlong *)0x0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_08 != (longlong *)0x0) {
          *extraout_RAX_08 = extraout_RAX + 0x24;
          egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
          LOCK();
          *local_198 = (longlong *)((longlong)*local_198 + -1);
          UNLOCK();
          if (*local_198 == (longlong *)0x0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
          return;
        }
      }
    }
  }
LAB_1400bfbf4:
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


