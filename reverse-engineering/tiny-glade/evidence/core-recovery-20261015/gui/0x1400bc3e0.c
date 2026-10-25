
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_8,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  undefined8 *puVar1;
  uint uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  undefined *puVar12;
  char extraout_AL;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined8 extraout_RAX_04;
  longlong *extraout_RAX_05;
  undefined8 *in_RCX;
  uint *puVar13;
  longlong lVar14;
  longlong *plVar15;
  longlong lVar16;
  longlong *plVar17;
  longlong lVar18;
  longlong lVar19;
  longlong *plVar20;
  longlong lVar21;
  undefined4 uVar22;
  float fVar23;
  undefined8 ****local_398;
  undefined8 uStack_390;
  longlong *local_388;
  undefined8 uStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  undefined8 uStack_360;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 ****local_320;
  undefined8 uStack_318;
  longlong *local_310;
  undefined8 uStack_308;
  undefined8 local_300;
  undefined8 uStack_2f8;
  undefined8 local_2f0;
  undefined8 uStack_2e8;
  undefined8 local_2e0;
  undefined8 uStack_2d8;
  undefined8 local_2d0;
  undefined8 uStack_2c8;
  undefined8 local_2c0;
  undefined8 uStack_2b8;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined4 local_138;
  longlong *local_d8;
  char local_95;
  char local_94;
  int local_88;
  float local_84;
  longlong local_80;
  uint *local_78;
  longlong *local_70;
  undefined1 local_61;
  undefined8 local_60;
  
  puVar12 = __imp__ZN12country_core9resources11time_of_day17TIME_OF_DAY_SPEED17hb4e7385447106f7aE;
  local_60 = 0xfffffffffffffffe;
  uVar22 = log2f(*(float *)
                  __imp__ZN12country_core9resources11time_of_day17TIME_OF_DAY_SPEED17hb4e7385447106f7aE
                 / 0.0016666666);
  local_178 = CONCAT44(local_178._4_4_,uVar22);
  uVar3 = *in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = uVar3;
    extraout_RAX[1] = &local_178;
    egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
    LOCK();
    *local_310 = *local_310 + -1;
    UNLOCK();
    if (*local_310 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    fVar23 = (float)exp2f((undefined4)local_178);
    *(float *)puVar12 = fVar23 * 0.0016666666;
    puVar13 = (uint *)in_RCX[1];
    uVar3 = in_RCX[5];
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      *extraout_RAX_00 = puVar13;
      extraout_RAX_00[1] = uVar3;
      egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
      LOCK();
      *local_310 = *local_310 + -1;
      UNLOCK();
      if (*local_310 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      uVar4 = in_RCX[4];
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 != (undefined8 *)0x0) {
        *extraout_RAX_01 = uVar4;
        egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
        LOCK();
        *local_310 = *local_310 + -1;
        UNLOCK();
        if (*local_310 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        uVar4 = in_RCX[2];
        uVar5 = in_RCX[3];
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_02 == (undefined8 *)0x0) goto LAB_1400bcaa0;
        *extraout_RAX_02 = uVar4;
        extraout_RAX_02[1] = uVar5;
        egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
        LOCK();
        *local_310 = *local_310 + -1;
        UNLOCK();
        if (*local_310 == 0) {
          ::alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
        local_88 = 2;
        plVar20 = (longlong *)in_RCX[6];
        *(undefined4 *)plVar20[2] = *(undefined4 *)((longlong)plVar20 + 0x1c);
        lVar14 = *(longlong *)(*plVar20 + 0x10);
        if (lVar14 != 0) {
          lVar21 = *(longlong *)(*plVar20 + 8);
          uVar4 = in_RCX[7];
          lVar14 = lVar14 * 0x50;
          local_78 = puVar13;
          local_70 = plVar20;
          do {
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_03 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              ::alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_03 = uVar4;
            extraout_RAX_03[1] = lVar21;
            extraout_RAX_03[2] = uVar3;
            extraout_RAX_03[3] = &local_88;
            egui::ui::Ui::scope_dyn<tuple_<>_>();
            LOCK();
            *local_310 = *local_310 + -1;
            UNLOCK();
            if (*local_310 == 0) {
              ::alloc::sync::Arc::
              drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                        ();
            }
            lVar21 = lVar21 + 0x50;
            lVar14 = lVar14 + -0x50;
          } while (lVar14 != 0);
          if (local_88 == 0) {
            fVar23 = fmodf(local_84,1.0);
            *local_78 = ~-(uint)(fVar23 < 0.0) & (uint)fVar23 |
                        (uint)(fVar23 + 1.0) & -(uint)(fVar23 < 0.0);
            puVar13 = local_78;
            plVar20 = local_70;
          }
          else {
            puVar13 = local_78;
            plVar20 = local_70;
            if ((local_88 == 1) && (lVar14 = *local_70, 1 < *(ulonglong *)(lVar14 + 0x10))) {
              *(undefined4 *)local_70[2] = *(undefined4 *)((longlong)local_70 + 0x1c);
              lVar21 = *(longlong *)(lVar14 + 0x10);
              if (lVar21 != 0) {
                plVar15 = *(longlong **)(lVar14 + 8);
                lVar16 = 1;
                lVar19 = 1;
                lVar18 = *plVar15;
                plVar17 = plVar15;
                while (local_80 != lVar18) {
                  plVar17 = plVar17 + 10;
                  if (lVar21 == lVar19) {
                    lVar16 = 0;
                    goto LAB_1400bc838;
                  }
                  lVar19 = lVar19 + 1;
                  lVar18 = *plVar17;
                }
                lVar18 = lVar21 - lVar19;
                if (lVar18 != 0) {
                  plVar15 = plVar15 + lVar19 * 10;
                  lVar16 = 1;
                  do {
                    while (local_80 != *plVar15) {
                      plVar17 = plVar15 + lVar16 * -10;
                      lVar19 = plVar15[9];
                      plVar15[lVar16 * -10 + 8] = plVar15[8];
                      (plVar15 + lVar16 * -10 + 8)[1] = lVar19;
                      lVar19 = *plVar15;
                      lVar6 = plVar15[1];
                      lVar7 = plVar15[2];
                      lVar8 = plVar15[3];
                      lVar9 = plVar15[4];
                      lVar10 = plVar15[5];
                      lVar11 = plVar15[7];
                      plVar17[6] = plVar15[6];
                      plVar17[7] = lVar11;
                      plVar17[4] = lVar9;
                      plVar17[5] = lVar10;
                      plVar17[2] = lVar7;
                      plVar17[3] = lVar8;
                      *plVar17 = lVar19;
                      plVar17[1] = lVar6;
                      plVar15 = plVar15 + 10;
                      lVar18 = lVar18 + -1;
                      if (lVar18 == 0) goto LAB_1400bc838;
                    }
                    lVar16 = lVar16 + 1;
                    plVar15 = plVar15 + 10;
                    lVar18 = lVar18 + -1;
                  } while (lVar18 != 0);
                }
LAB_1400bc838:
                *(longlong *)(lVar14 + 0x10) = lVar21 - lVar16;
              }
            }
          }
        }
        local_178 = 4;
        local_61 = 0;
        convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
        local_338 = local_2c0;
        uStack_330 = uStack_2b8;
        local_348 = local_2d0;
        uStack_340 = uStack_2c8;
        local_358 = local_2e0;
        uStack_350 = uStack_2d8;
        local_368 = local_2f0;
        uStack_360 = uStack_2e8;
        local_378 = local_300;
        uStack_370 = uStack_2f8;
        local_388 = local_310;
        uStack_380 = uStack_308;
        local_398 = local_320;
        uStack_390 = uStack_318;
        local_61 = 1;
        egui::widgets::button::Button::opt_image_and_text();
        egui::widgets::button::impl$1::ui();
        if ((local_94 == '\0') &&
           ((local_320 = (undefined8 ****)((ulonglong)local_320 & 0xffffffffffffff00),
            local_95 == '\0' ||
            (egui::context::Context::
             write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                       (), extraout_AL == '\0')))) {
          LOCK();
          *local_d8 = *local_d8 + -1;
          UNLOCK();
          if (*local_d8 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
        }
        else {
          LOCK();
          *local_d8 = *local_d8 + -1;
          UNLOCK();
          if (*local_d8 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
          uVar2 = *puVar13;
          country_core::resources::time_of_day::TodSequence::snapshot_at_tod();
          country_core::resources::time_of_day::impl$5::default();
          *(undefined4 *)plVar20[2] = *(undefined4 *)((longlong)plVar20 + 0x1c);
          plVar15 = (longlong *)*plVar20;
          lVar14 = plVar15[2];
          if (lVar14 == *plVar15) {
            ::alloc::raw_vec::RawVec::
            grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global>();
          }
          lVar21 = plVar15[1];
          lVar18 = lVar14 * 0x50;
          *(undefined8 *)(lVar21 + lVar18) = extraout_RAX_04;
          *(uint *)(lVar21 + 8 + lVar18) = uVar2;
          puVar1 = (undefined8 *)(lVar21 + 0xc + lVar18);
          *puVar1 = local_178;
          puVar1[1] = uStack_170;
          puVar1 = (undefined8 *)(lVar21 + 0x1c + lVar18);
          *puVar1 = local_168;
          puVar1[1] = uStack_160;
          puVar1 = (undefined8 *)(lVar21 + 0x2c + lVar18);
          *puVar1 = local_158;
          puVar1[1] = uStack_150;
          puVar1 = (undefined8 *)(lVar21 + 0x3c + lVar18);
          *puVar1 = local_148;
          puVar1[1] = uStack_140;
          *(undefined4 *)(lVar21 + 0x4c + lVar18) = local_138;
          plVar15[2] = lVar14 + 1;
        }
        *(undefined4 *)plVar20[2] = *(undefined4 *)((longlong)plVar20 + 0x1c);
        local_320 = &local_398;
        if (1 < *(ulonglong *)(*plVar20 + 0x10)) {
          if (*(ulonglong *)(*plVar20 + 0x10) < 0x15) {
            slice::sort::shared::smallsort::
            insertion_sort_shift_left<country_core::resources::time_of_day::TodSequenceEntry,alloc::slice::impl_0::sort_by::closure_env_0<country_core::resources::time_of_day::TodSequenceEntry,tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_8::closure_env_6>_>
                      ();
          }
          else {
            slice::sort::stable::
            driftsort_main<country_core::resources::time_of_day::TodSequenceEntry,alloc::slice::impl_0::sort_by::closure_env_0<country_core::resources::time_of_day::TodSequenceEntry,tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_8::closure_env_6>,alloc::vec::Vec<country_core::resources::time_of_day::TodSequenceEntry,alloc::alloc::Global>_>
                      ();
          }
        }
        *(undefined4 *)plVar20[2] = *(undefined4 *)((longlong)plVar20 + 0x1c);
        lVar14 = *plVar20;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_05 != (longlong *)0x0) {
          *extraout_RAX_05 = lVar14;
          egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
          LOCK();
          *local_310 = *local_310 + -1;
          UNLOCK();
          if (*local_310 == 0) {
            ::alloc::sync::Arc::
            drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                      ();
          }
          return;
        }
      }
                    // WARNING: Subroutine does not return
      ::alloc::alloc::handle_alloc_error();
    }
  }
LAB_1400bcaa0:
                    // WARNING: Subroutine does not return
  ::alloc::alloc::handle_alloc_error();
}


