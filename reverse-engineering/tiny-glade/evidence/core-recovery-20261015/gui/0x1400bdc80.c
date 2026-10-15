
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_13::closure_env_1,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  undefined4 uVar1;
  longlong *plVar2;
  undefined4 *puVar3;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  void *_Src;
  undefined8 *extraout_RAX_02;
  longlong lVar4;
  longlong *extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  undefined8 *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  undefined8 *extraout_RAX_08;
  longlong *in_RCX;
  undefined *_Size;
  uint uVar5;
  undefined8 *_Dst;
  longlong *local_368;
  longlong *plStack_360;
  longlong *local_358;
  longlong *plStack_350;
  longlong lStack_348;
  longlong *plStack_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 uStack_328;
  undefined8 local_320;
  ulonglong uStack_318;
  ulonglong local_310;
  undefined8 uStack_308;
  undefined8 local_300;
  undefined8 uStack_2f8;
  longlong *local_220;
  longlong *plStack_218;
  longlong *local_210;
  longlong lStack_208;
  longlong *local_200;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 local_1e0;
  ulonglong uStack_1d8;
  ulonglong local_1d0;
  undefined8 uStack_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 *local_1b0;
  longlong local_1a8;
  longlong *local_1a0;
  longlong *plStack_198;
  longlong local_190;
  longlong *local_188;
  longlong *plStack_180;
  longlong *local_178;
  longlong *local_170;
  longlong *plStack_168;
  ulonglong local_160;
  longlong *plStack_158;
  longlong *local_150;
  undefined1 uStack_148;
  undefined4 uStack_147;
  undefined4 uStack_143;
  undefined3 uStack_13f;
  undefined4 uStack_13c;
  longlong lStack_138;
  ulonglong local_130;
  ulonglong uStack_128;
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
  undefined8 local_d0;
  undefined8 uStack_c8;
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
  undefined4 local_5b;
  undefined4 uStack_57;
  undefined3 uStack_53;
  longlong *local_50;
  undefined1 local_42;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  plVar2 = (longlong *)*in_RCX;
  uVar1 = *(undefined4 *)((longlong)plVar2 + 0x1c);
  puVar3 = (undefined4 *)plVar2[2];
  *puVar3 = uVar1;
  lVar4 = *plVar2;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    lStack_138 = lVar4 + 0xf8;
    *(undefined8 *)((longlong)extraout_RAX + 10) = 0x656c616373206e61;
    *(undefined8 *)((longlong)extraout_RAX + 0x12) = 0x2952282066657220;
    *extraout_RAX = 0x6820656c62616e45;
    extraout_RAX[1] = 0x616373206e616d75;
    local_178 = (longlong *)0x1a;
    plStack_168 = (longlong *)0x1a;
    local_160 = local_160 & 0xffffffff00000000;
    local_150 = (longlong *)0x0;
    plStack_158 = (longlong *)((ulonglong)plStack_158 & 0xffffffff00000000);
    local_1a8 = 3;
    local_190 = 6;
    uStack_148 = 0;
    uStack_143 = 0;
    uStack_13f = 0;
    local_130 = local_130 & 0xffffffffffffff00;
    local_170 = extraout_RAX;
    egui::widgets::button::impl$3::ui();
    LOCK();
    *local_358 = *local_358 + -1;
    UNLOCK();
    if (*local_358 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    *puVar3 = uVar1;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      lStack_138 = lVar4 + 0xf3;
      *(undefined8 *)((longlong)extraout_RAX_00 + 10) = 0x6f68736572687420;
      *(undefined8 *)((longlong)extraout_RAX_00 + 0x12) = 0x657672756320646c;
      *extraout_RAX_00 = 0x62656420776f6853;
      extraout_RAX_00[1] = 0x7365726874206775;
      local_178 = (longlong *)0x1a;
      plStack_168 = (longlong *)0x1a;
      local_160 = local_160 & 0xffffffff00000000;
      local_150 = (longlong *)0x0;
      plStack_158 = (longlong *)((ulonglong)plStack_158 & 0xffffffff00000000);
      local_1a8 = 3;
      local_190 = 6;
      uStack_148 = 0;
      uStack_143 = 0;
      uStack_13f = 0;
      local_130 = local_130 & 0xffffffffffffff00;
      egui::widgets::button::impl$3::ui();
      LOCK();
      *local_358 = *local_358 + -1;
      UNLOCK();
      if (*local_358 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      *puVar3 = uVar1;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        ::alloc::raw_vec::handle_error();
      }
      lStack_138 = lVar4 + 0xf1;
      *extraout_RAX_01 = 0x62656420776f6853;
      extraout_RAX_01[1] = 0x6974617073206775;
      *(undefined8 *)((longlong)extraout_RAX_01 + 0xf) = 0x64697267206c6169;
      local_178 = (longlong *)0x17;
      plStack_168 = (longlong *)0x17;
      local_160 = local_160 & 0xffffffff00000000;
      local_150 = (longlong *)0x0;
      plStack_158 = (longlong *)((ulonglong)plStack_158 & 0xffffffff00000000);
      local_1a8 = 3;
      local_190 = 6;
      uStack_148 = 0;
      uStack_143 = 0;
      uStack_13f = 0;
      local_130 = local_130 & 0xffffffffffffff00;
      egui::widgets::button::impl$3::ui();
      LOCK();
      *local_358 = *local_358 + -1;
      UNLOCK();
      if (*local_358 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      _Size = &DAT_142935553;
      egui::containers::combo_box::ComboBox::from_label<ref_<str_>_>();
      local_41 = 1;
      country_core::resources::gui::ShowDebugRaycastGeo::as_string();
      if ((longlong)_Size < 0) {
LAB_1400be8bc:
                    // WARNING: Subroutine does not return
        ::alloc::raw_vec::handle_error();
      }
      if (_Size == (undefined *)0x0) {
        _Dst = (undefined8 *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        _Dst = extraout_RAX_02;
        if (extraout_RAX_02 == (undefined8 *)0x0) goto LAB_1400be8bc;
      }
      memcpy(_Dst,_Src,(size_t)_Size);
      uStack_57 = 0;
      uStack_53 = 0;
      uVar5 = (uint)local_1a8;
      lVar4 = 0;
      if ((uVar5 & 0xfffffffe) == 4) {
        lVar4 = local_1a8 + -3;
      }
      local_1b0 = _Dst;
      local_50 = (longlong *)_Size;
      if (lVar4 == 0) {
        if (local_178 != (longlong *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if ((uVar5 != 3) && (1 < uVar5)) {
          LOCK();
          *local_1a0 = *local_1a0 + -1;
          UNLOCK();
          if (*local_1a0 == 0) {
            ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
          }
        }
        if ((local_190 != 6) && (4 < (uint)local_190)) {
          LOCK();
          *local_188 = *local_188 + -1;
          UNLOCK();
          if (*local_188 == 0) {
            ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
          }
        }
      }
      else if (lVar4 == 1) {
        if (local_1a0 != (longlong *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        ptr::
        drop_in_place<alloc::vec::Vec<epaint::text::text_layout_types::LayoutSection,alloc::alloc::Global>_>
                  ();
      }
      else {
        LOCK();
        *local_1a0 = *local_1a0 + -1;
        UNLOCK();
        if (*local_1a0 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::text::text_layout_types::Galley,alloc::alloc::Global>();
        }
      }
      local_1a8 = 3;
      local_1a0 = local_368;
      plStack_198 = plStack_360;
      local_190 = 6;
      local_188 = local_220;
      plStack_180 = plStack_218;
      local_178 = local_50;
      local_170 = local_1b0;
      plStack_168 = local_50;
      local_160 = local_160 & 0xffffffff00000000;
      plStack_158 = (longlong *)((ulonglong)plStack_158 & 0xffffffff00000000);
      local_150 = (longlong *)0x0;
      uStack_148 = 0;
      uStack_143 = uStack_57;
      uStack_13f = uStack_53;
      memcpy(&local_368,&local_1a8,0x110);
      memcpy(&local_1a8,&local_368,0x110);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        ::alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_03 = (longlong)plVar2;
      egui::containers::combo_box::ComboBox::show_ui_dyn<tuple_<>_>();
      LOCK();
      *local_210 = *local_210 + -1;
      UNLOCK();
      if (*local_210 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      *(undefined4 *)plVar2[2] = *(undefined4 *)((longlong)plVar2 + 0x1c);
      lVar4 = *plVar2;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_04 != (undefined8 *)0x0) {
        lStack_138 = lVar4 + 0xf2;
        *(undefined8 *)((longlong)extraout_RAX_04 + 10) = 0x7265676769727420;
        *(undefined8 *)((longlong)extraout_RAX_04 + 0x12) = 0x736569786f727020;
        *extraout_RAX_04 = 0x62656420776f6853;
        extraout_RAX_04[1] = 0x6767697274206775;
        local_178 = (longlong *)0x1a;
        plStack_168 = (longlong *)0x1a;
        local_160 = local_160 & 0xffffffff00000000;
        local_150 = (longlong *)0x0;
        plStack_158 = (longlong *)((ulonglong)plStack_158 & 0xffffffff00000000);
        local_1a8 = 3;
        local_190 = 6;
        uStack_148 = 0;
        uStack_143 = 0;
        uStack_13f = 0;
        local_130 = local_130 & 0xffffffffffffff00;
        egui::widgets::button::impl$3::ui();
        LOCK();
        *local_358 = *local_358 + -1;
        UNLOCK();
        if (*local_358 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        *(undefined4 *)plVar2[2] = *(undefined4 *)((longlong)plVar2 + 0x1c);
        lVar4 = *plVar2;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_05 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          ::alloc::raw_vec::handle_error();
        }
        lStack_138 = lVar4 + 0xf4;
        *(undefined8 *)((longlong)extraout_RAX_05 + 6) = 0x6469726720797a61;
        *extraout_RAX_05 = 0x7a616c20776f6853;
        local_178 = (longlong *)0xe;
        plStack_168 = (longlong *)0xe;
        local_160 = local_160 & 0xffffffff00000000;
        local_150 = (longlong *)0x0;
        plStack_158 = (longlong *)((ulonglong)plStack_158 & 0xffffffff00000000);
        local_1a8 = 3;
        local_190 = 6;
        uStack_148 = 0;
        uStack_143 = 0;
        uStack_13f = 0;
        local_130 = local_130 & 0xffffffffffffff00;
        egui::widgets::button::impl$3::ui();
        LOCK();
        *local_358 = *local_358 + -1;
        UNLOCK();
        if (*local_358 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        if (*(char *)(*plVar2 + 0xf2) != '\0') {
          egui::containers::combo_box::ComboBox::from_label<ref_<str_>_>();
          local_42 = 1;
          country_core::resources::gui::DebugTriggerProxyVis::as_string();
          local_42 = 0;
          egui::containers::combo_box::ComboBox::selected_text<alloc::string::String>();
          memcpy(&local_1a8,&local_368,0x110);
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_06 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
            ::alloc::alloc::handle_alloc_error();
          }
          *extraout_RAX_06 = (longlong)plVar2;
          egui::containers::combo_box::ComboBox::show_ui_dyn<tuple_<>_>();
          LOCK();
          *local_210 = *local_210 + -1;
          UNLOCK();
          if (*local_210 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
          lVar4 = *plVar2;
          if (*(char *)(lVar4 + 0x102) == '\x03') {
            *(undefined4 *)plVar2[2] = *(undefined4 *)((longlong)plVar2 + 0x1c);
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_07 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
              ::alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_07 = lVar4 + 0xe8;
            local_50 = extraout_RAX_07;
            egui::widget_text::impl$6::default();
            local_108 = 0;
            local_100 = 1;
            local_f8 = 0;
            uStack_f0 = 0;
            local_e8 = 1;
            local_e0 = 0;
            plStack_360 = local_220;
            local_358 = plStack_218;
            plStack_350 = local_210;
            lStack_348 = lStack_208;
            plStack_340 = local_200;
            uStack_338 = uStack_1f8;
            local_330 = local_1f0;
            uStack_328 = uStack_1e8;
            local_320 = local_1e0;
            uStack_318 = uStack_1d8;
            local_310 = local_1d0;
            uStack_308 = uStack_1c8;
            local_300 = local_1c0;
            uStack_2f8 = uStack_1b8;
            local_90 = 0;
            uStack_88 = 0;
            local_1a8 = 0;
            plStack_198 = (longlong *)0x0;
            local_188 = (longlong *)0x0;
            plStack_180 = local_368;
            local_178 = local_220;
            local_170 = plStack_218;
            plStack_168 = local_210;
            local_160 = lStack_208;
            plStack_158 = local_200;
            local_150 = (longlong *)uStack_1f8;
            uStack_148 = (undefined1)local_1f0;
            uStack_147 = (undefined4)((ulonglong)local_1f0 >> 8);
            uStack_143._0_3_ = (undefined3)((ulonglong)local_1f0 >> 0x28);
            uStack_143._3_1_ = (byte)uStack_1e8;
            uStack_13f = (undefined3)((ulonglong)uStack_1e8 >> 8);
            uStack_13c = (undefined4)((ulonglong)uStack_1e8 >> 0x20);
            lStack_138 = local_1e0;
            local_130 = uStack_1d8;
            uStack_128 = local_1d0;
            local_120 = uStack_1c8;
            uStack_118 = (undefined1)local_1c0;
            uStack_117 = (undefined7)((ulonglong)local_1c0 >> 8);
            uStack_110 = (undefined4)uStack_1b8;
            uStack_10c = (undefined4)((ulonglong)uStack_1b8 >> 0x20);
            local_d8 = 2;
            local_d0 = 0x3fc99999a0000000;
            uStack_c8 = 0x4010000000000000;
            local_c0 = 0;
            local_b8 = 0x3eb0c6f7a0b5ed8d;
            uStack_b0 = 0x7ff0000000000000;
            local_a8 = 0;
            local_a0 = local_50;
            local_98 = &anon_aa39661ce9781c29d9a9638471876d74_42_llvm_6754927731931653273;
            local_78 = 0;
            local_68 = 0x10101;
            local_64 = 2;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_08 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              ::alloc::raw_vec::handle_error();
            }
            *(undefined8 *)((longlong)extraout_RAX_08 + 5) = 0x6874676e656c206c;
            *extraout_RAX_08 = 0x6c206c616d726f4e;
            uStack_53 = 0;
            uStack_57 = 0;
            uVar5 = (uint)local_178;
            lVar4 = 0;
            if ((uVar5 & 0xfffffffe) == 4) {
              lVar4 = (longlong)local_178 + -3;
            }
            local_50 = extraout_RAX_08;
            if (lVar4 == 0) {
              if (CONCAT35((undefined3)uStack_143,CONCAT41(uStack_147,uStack_148)) != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if ((uVar5 != 3) && (1 < uVar5)) {
                LOCK();
                *local_170 = *local_170 + -1;
                UNLOCK();
                if (*local_170 == 0) {
                  ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
                }
              }
              if ((local_160 != 6) && (4 < (uint)local_160)) {
                LOCK();
                *plStack_158 = *plStack_158 + -1;
                UNLOCK();
                if (*plStack_158 == 0) {
                  ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
                }
              }
            }
            else if (lVar4 == 1) {
              if (local_170 != (longlong *)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              ptr::
              drop_in_place<alloc::vec::Vec<epaint::text::text_layout_types::LayoutSection,alloc::alloc::Global>_>
                        ();
            }
            else {
              LOCK();
              *local_170 = *local_170 + -1;
              UNLOCK();
              if (*local_170 == 0) {
                ::alloc::sync::Arc::
                drop_slow<epaint::text::text_layout_types::Galley,alloc::alloc::Global>();
              }
            }
            local_178 = (longlong *)0x3;
            local_170 = local_368;
            plStack_168 = plStack_360;
            local_160 = 6;
            plStack_158 = local_220;
            local_150 = plStack_218;
            uStack_148 = 0xd;
            uStack_147 = 0;
            uStack_143._3_1_ = (byte)local_50;
            uStack_143 = (uint)uStack_143._3_1_ << 0x18;
            uStack_13f = (undefined3)((ulonglong)local_50 >> 8);
            uStack_13c = (undefined4)((ulonglong)local_50 >> 0x20);
            lStack_138 = 0xd;
            local_130 = local_130 & 0xffffffff00000000;
            uStack_128 = uStack_128 & 0xffffffff00000000;
            local_120 = 0;
            uStack_118 = 0;
            uStack_117 = (undefined7)CONCAT44(uStack_57,local_5b);
            uStack_110 = CONCAT31(uStack_53,uStack_57._3_1_);
            memcpy(&local_368,&local_1a8,0x148);
            egui::widgets::slider::impl$2::ui();
            LOCK();
            *plStack_198 = *plStack_198 + -1;
            UNLOCK();
            if (*plStack_198 == 0) {
              ::alloc::sync::Arc::
              drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                        ();
            }
          }
        }
        return;
      }
    }
  }
                    // WARNING: Subroutine does not return
  ::alloc::raw_vec::handle_error();
}


