
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_7,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  undefined8 uVar1;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 *in_RCX;
  uint uVar2;
  longlong lVar3;
  undefined8 local_340;
  ulonglong uStack_338;
  longlong *plStack_330;
  undefined8 local_328;
  longlong lStack_320;
  longlong *local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  undefined8 local_2f8;
  ulonglong uStack_2f0;
  ulonglong local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  ulonglong local_1f8;
  longlong *plStack_1f0;
  undefined8 local_1e8;
  longlong lStack_1e0;
  longlong *local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  ulonglong uStack_1b0;
  ulonglong local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188 [2];
  longlong *local_178;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 local_160;
  ulonglong uStack_158;
  longlong *local_150;
  ulonglong uStack_148;
  ulonglong local_140;
  longlong *plStack_138;
  longlong *local_130;
  undefined1 uStack_128;
  undefined4 uStack_127;
  undefined4 uStack_123;
  undefined3 uStack_11f;
  undefined4 uStack_11c;
  undefined1 *puStack_118;
  ulonglong local_110;
  ulonglong uStack_108;
  undefined8 local_100;
  undefined1 uStack_f8;
  undefined7 uStack_f7;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined4 local_b8;
  undefined4 local_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined1 local_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined1 local_88;
  undefined8 *local_80;
  undefined *local_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined8 local_58;
  undefined4 local_48;
  undefined1 local_44;
  undefined4 local_3b;
  undefined4 uStack_37;
  undefined3 uStack_33;
  undefined8 *local_30;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  uVar1 = *in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = uVar1;
    egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
    LOCK();
    *local_178 = *local_178 + -1;
    UNLOCK();
    if (*local_178 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    uVar1 = in_RCX[2];
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      *extraout_RAX_00 = uVar1;
      egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
      LOCK();
      *local_178 = *local_178 + -1;
      UNLOCK();
      if (*local_178 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      uVar1 = in_RCX[1];
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 != (undefined8 *)0x0) {
        *extraout_RAX_01 = uVar1;
        local_30 = extraout_RAX_01;
        egui::widget_text::impl$6::default();
        local_e8 = 0;
        local_e0 = 1;
        local_d8 = 0;
        uStack_d0 = 0;
        local_c8 = 1;
        local_c0 = 0;
        uStack_338 = local_1f8;
        plStack_330 = plStack_1f0;
        local_328 = local_1e8;
        lStack_320 = lStack_1e0;
        local_318 = local_1d8;
        uStack_310 = uStack_1d0;
        local_308 = local_1c8;
        uStack_300 = uStack_1c0;
        local_2f8 = local_1b8;
        uStack_2f0 = uStack_1b0;
        local_2e8 = local_1a8;
        uStack_2e0 = uStack_1a0;
        local_2d8 = local_198;
        uStack_2d0 = uStack_190;
        local_70 = 0;
        uStack_68 = 0;
        local_188[0] = 0;
        local_178 = (longlong *)0x0;
        local_168 = 0;
        local_160 = local_340;
        uStack_158 = local_1f8;
        local_150 = plStack_1f0;
        uStack_148 = local_1e8;
        local_140 = lStack_1e0;
        plStack_138 = local_1d8;
        local_130 = (longlong *)uStack_1d0;
        uStack_128 = (undefined1)local_1c8;
        uStack_127 = (undefined4)((ulonglong)local_1c8 >> 8);
        uStack_123._0_3_ = (undefined3)((ulonglong)local_1c8 >> 0x28);
        uStack_123._3_1_ = (byte)uStack_1c0;
        uStack_11f = (undefined3)((ulonglong)uStack_1c0 >> 8);
        uStack_11c = (undefined4)((ulonglong)uStack_1c0 >> 0x20);
        puStack_118 = (undefined1 *)local_1b8;
        local_110 = uStack_1b0;
        uStack_108 = local_1a8;
        local_100 = uStack_1a0;
        uStack_f8 = (undefined1)local_198;
        uStack_f7 = (undefined7)((ulonglong)local_198 >> 8);
        uStack_f0 = (undefined4)uStack_190;
        uStack_ec = (undefined4)((ulonglong)uStack_190 >> 0x20);
        local_b8 = 2;
        local_b0 = 0;
        uStack_ac = 0;
        uStack_a8 = 0;
        uStack_a4 = 0x40568000;
        local_a0 = 0;
        local_98 = 0x3eb0c6f7a0b5ed8d;
        uStack_90 = 0x7ff0000000000000;
        local_88 = 0;
        local_80 = local_30;
        local_78 = &anon_aa39661ce9781c29d9a9638471876d74_42_llvm_6754927731931653273;
        local_58 = 0;
        local_48 = 0x10101;
        local_44 = 2;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          ::alloc::raw_vec::handle_error();
        }
        *extraout_RAX_02 = 0x656475746974614c;
        uStack_33 = 0;
        uStack_37 = 0;
        uVar2 = (uint)uStack_158;
        lVar3 = 0;
        if ((uVar2 & 0xfffffffe) == 4) {
          lVar3 = uStack_158 - 3;
        }
        local_30 = extraout_RAX_02;
        if (lVar3 == 0) {
          if (CONCAT35((undefined3)uStack_123,CONCAT41(uStack_127,uStack_128)) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if ((uVar2 != 3) && (1 < uVar2)) {
            LOCK();
            *local_150 = *local_150 + -1;
            UNLOCK();
            if (*local_150 == 0) {
              ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
            }
          }
          if ((local_140 != 6) && (4 < (uint)local_140)) {
            LOCK();
            *plStack_138 = *plStack_138 + -1;
            UNLOCK();
            if (*plStack_138 == 0) {
              ::alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
            }
          }
        }
        else if (lVar3 == 1) {
          if (local_150 != (longlong *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          ptr::
          drop_in_place<alloc::vec::Vec<epaint::text::text_layout_types::LayoutSection,alloc::alloc::Global>_>
                    ();
        }
        else {
          LOCK();
          *local_150 = *local_150 + -1;
          UNLOCK();
          if (*local_150 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::text::text_layout_types::Galley,alloc::alloc::Global>();
          }
        }
        uStack_158 = 3;
        local_150 = (longlong *)local_340;
        uStack_148 = uStack_338;
        local_140 = 6;
        plStack_138 = (longlong *)local_1f8;
        local_130 = plStack_1f0;
        uStack_128 = 8;
        uStack_127 = 0;
        uStack_123._3_1_ = (byte)local_30;
        uStack_123 = (uint)uStack_123._3_1_ << 0x18;
        uStack_11f = (undefined3)((ulonglong)local_30 >> 8);
        uStack_11c = (undefined4)((ulonglong)local_30 >> 0x20);
        puStack_118 = (undefined1 *)0x8;
        local_110 = local_110 & 0xffffffff00000000;
        uStack_108 = uStack_108 & 0xffffffff00000000;
        local_100 = 0;
        uStack_f8 = 0;
        uStack_f7 = (undefined7)CONCAT44(uStack_37,local_3b);
        uStack_f0 = CONCAT31(uStack_33,uStack_37._3_1_);
        memcpy(&local_340,local_188,0x148);
        egui::widgets::slider::impl$2::ui();
        LOCK();
        *local_178 = *local_178 + -1;
        UNLOCK();
        if (*local_178 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_03 != (undefined8 *)0x0) {
          *extraout_RAX_03 = 0x79617220776f6853;
          extraout_RAX_03[1] = 0x676e696361727420;
          *(undefined8 *)((longlong)extraout_RAX_03 + 0xd) = 0x7765697620676e69;
          uStack_158 = 0x15;
          uStack_148 = 0x15;
          local_140 = local_140 & 0xffffffff00000000;
          local_130 = (longlong *)0x0;
          plStack_138 = (longlong *)((ulonglong)plStack_138 & 0xffffffff00000000);
          local_188[0] = 3;
          local_170 = 6;
          uStack_128 = 0;
          uStack_123 = 0;
          uStack_11f = 0;
          puStack_118 = &tiny_glade::game_loop::RENDER_OVERRIDE_SHOW_DEBUG_RT_VIEW;
          local_110 = local_110 & 0xffffffffffffff00;
          egui::widgets::button::impl$3::ui();
          LOCK();
          *plStack_330 = *plStack_330 + -1;
          UNLOCK();
          if (*plStack_330 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
          return;
        }
                    // WARNING: Subroutine does not return
        ::alloc::raw_vec::handle_error();
      }
    }
  }
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


