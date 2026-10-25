// 0x141a39490
// void __cdecl do_ui_edit_roof_ridge_dir(void)
// do_ui_edit_roof_ridge_dir


void __cdecl system_roof::ui_edit_roof_ridge_dir::do_ui_edit_roof_ridge_dir(void)

{
  byte *pbVar1;
  uint uVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined1 uVar6;
  ulonglong *puVar7;
  undefined8 uVar8;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  uint uVar9;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 *extraout_RAX_09;
  undefined8 *in_RCX;
  longlong *plVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  undefined8 unaff_RBX;
  ulonglong uVar15;
  undefined8 *puVar16;
  longlong *in_R8;
  undefined *puVar17;
  ulonglong uVar18;
  undefined4 uVar19;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  undefined **ppuVar25;
  longlong lVar26;
  bool bVar27;
  bool bVar28;
  uint unaff_XMM6_Da;
  float unaff_XMM7_Da;
  float unaff_XMM8_Da;
  float unaff_XMM8_Db;
  uint unaff_XMM9_Da;
  uint unaff_XMM9_Db;
  longlong in_stack_00000038;
  longlong *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined **local_308;
  code *pcStack_300;
  undefined8 *puStack_2f8;
  undefined8 local_2f0;
  uint *local_2e8;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  longlong local_240;
  undefined8 local_238;
  undefined8 local_228;
  code *pcStack_220;
  undefined ***local_218;
  float fStack_210;
  undefined4 uStack_20c;
  undefined8 uStack_208;
  code *local_200;
  longlong local_1f8;
  code *local_1f0;
  undefined *local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  uint local_148 [4];
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  uint local_118;
  float fStack_114;
  float fStack_110;
  float fStack_10c;
  undefined8 local_108;
  float local_100;
  longlong *local_f0;
  uint local_e8;
  uint uStack_e4;
  longlong local_e0;
  undefined *local_d8;
  longlong *local_d0;
  undefined8 local_c8;
  longlong *local_c0;
  undefined1 local_b2;
  undefined1 local_b1;
  undefined8 local_b0;
  
  local_b0 = 0xfffffffffffffffe;
  local_d0 = in_R8;
  puffin::are_scopes_on();
  local_c0 = in_RCX;
  if (extraout_AL == '\0') {
    local_e0 = 0;
  }
  else {
    uVar13 = 0x3e;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x3e) {
          if ((char)(&DAT_142ebaede)[uVar13] < -0x40) {
LAB_141a3954c:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 != 0x3e) goto LAB_141a3954c;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar13 != 0xfffffffffffffffe)) {
        if (uVar13 + 2 < 0x3e) {
          if ((char)(&UNK_142ebaee0)[uVar13] < -0x40) {
LAB_141a395aa:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 + 2 != 0x3e) goto LAB_141a395aa;
      }
    }
    puVar17 = &DAT_142ebb109;
    do {
      bVar3 = puVar17[-1];
      uVar9 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar17[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar17[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar17 + -4;
            puVar17 = puVar17 + -4;
            uVar9 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar17 = puVar17 + -3;
            uVar9 = bVar5 & 0xf;
          }
          uVar9 = bVar4 & 0x3f | uVar9 << 6;
        }
        else {
          puVar17 = puVar17 + -2;
          uVar9 = bVar4 & 0x1f;
        }
        uVar9 = (uint)(bVar3 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_141a39664;
LAB_141a39675:
        if (puVar17 == &UNK_142ebb0d7) break;
        if (puVar17 + -0x142ebb0d7 < (undefined *)0x31) {
          if (-0x41 < (char)puVar17[1]) break;
        }
        else if (puVar17 + -0x142ebb0d7 == (undefined *)0x31) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar17 = puVar17 + -1;
      if (uVar9 == 0x5c) goto LAB_141a39675;
LAB_141a39664:
      if (uVar9 == 0x2f) goto LAB_141a39675;
    } while (puVar17 != &anon_cb06c9136c6217bb1323d596e337a917_27_llvm_16635302344178984818);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar10 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_141a39d57;
    if (*plVar10 != 0) goto LAB_141a3a018;
    *plVar10 = -1;
    local_f0 = plVar10;
    puffin::ThreadProfiler::begin_scope();
    *local_f0 = *local_f0 + 1;
    local_e0 = 1;
  }
  puVar7 = (ulonglong *)*in_stack_00000058;
  lVar12 = in_stack_00000058[1];
  uVar13 = *puVar7;
  uVar15 = 0;
  uVar14 = uVar13 - *(ulonglong *)(lVar12 + 0x18);
  if (uVar13 < *(ulonglong *)(lVar12 + 0x18)) {
    uVar14 = uVar15;
  }
  uVar11 = uVar13 - *(ulonglong *)(lVar12 + 0x38);
  if (uVar13 < *(ulonglong *)(lVar12 + 0x38)) {
    uVar11 = uVar15;
  }
  lVar20 = *(longlong *)(lVar12 + 8);
  uVar13 = *(ulonglong *)(lVar12 + 0x10);
  uVar18 = *(ulonglong *)(lVar12 + 0x30);
  bVar27 = uVar18 < uVar11;
  uVar23 = uVar18 - uVar11;
  uVar24 = uVar23;
  if (bVar27) {
    uVar24 = uVar15;
  }
  bVar28 = uVar13 < uVar14;
  uVar21 = uVar13 - uVar14;
  uVar22 = uVar21;
  if (bVar28) {
    uVar22 = uVar15;
  }
  lVar26 = *(longlong *)(lVar12 + 0x28);
  uVar15 = *(ulonglong *)(lVar12 + 0x40);
  *puVar7 = (uVar15 - uVar24) - uVar22;
  if (!bVar28 && uVar21 != 0 || !bVar27 && uVar23 != 0) {
    lVar26 = lVar26 + uVar11 * 0x30;
    lVar20 = lVar20 + uVar14 * 0x30;
    if (uVar13 < uVar14) {
      lVar20 = 8;
    }
    lVar12 = 0;
    if (uVar14 < uVar13) {
      lVar12 = lVar20 + -0x30 + uVar22 * 0x30;
    }
    if (uVar18 < uVar11) {
      lVar26 = 8;
    }
    lVar20 = lVar26 + -0x30 + uVar24 * 0x30;
    if (uVar18 <= uVar11) {
      lVar20 = lVar12;
    }
    *puVar7 = uVar15;
    unaff_RBX = *(undefined8 *)(lVar20 + 8);
    unaff_XMM8_Da = (float)*(undefined8 *)(lVar20 + 0x20);
    unaff_XMM8_Db = (float)((ulonglong)*(undefined8 *)(lVar20 + 0x20) >> 0x20);
    unaff_XMM7_Da = *(float *)(lVar20 + 0x28);
    unaff_XMM6_Da = *(uint *)(lVar20 + 0x2c);
    unaff_XMM9_Da = (uint)*(undefined8 *)(lVar20 + 0x10);
    unaff_XMM9_Db = (uint)((ulonglong)*(undefined8 *)(lVar20 + 0x10) >> 0x20);
  }
  if (1 < *(int *)*local_c0 - 1U) goto LAB_141a39ba2;
  country_core::resources::walls::public_walls::PublicWalls::roof_entity();
  if (extraout_RAX_02 >> 0x20 != 0) {
    lVar12 = local_d0[1];
    uVar13 = extraout_RAX_02 & 0xffffffff;
    if ((uVar13 < *(ulonglong *)(lVar12 + 0x10)) &&
       (uVar18 = *(ulonglong *)(lVar12 + 8),
       *(int *)(uVar18 + uVar13 * 0x14) == (int)(extraout_RAX_02 >> 0x20))) {
      lVar20 = uVar18 + uVar13 * 0x14;
      uVar9 = *(uint *)(lVar20 + 4);
      uVar18 = (ulonglong)uVar9;
      if (uVar18 != 0xffffffff) {
        lVar26 = *local_d0;
        uVar19 = 0;
        if ((*(ulonglong *)(lVar26 + 0x38) <= uVar18) ||
           ((*(ulonglong *)(*(longlong *)(lVar26 + 0x28) + (ulonglong)(uVar9 >> 6) * 8) >>
             (uVar18 & 0x3f) & 1) == 0)) goto LAB_141a3a034;
        if (*(ulonglong *)(lVar26 + 0x110) <
            *(ulonglong *)
             (*(longlong *)(lVar12 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar20 + 0xc) * 0x48)) {
          lVar12 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)*(uint *)(lVar20 + 0xc) * 0x48;
          uVar13 = *(ulonglong *)(*(longlong *)(lVar12 + 0x38) + *(ulonglong *)(lVar26 + 0x110) * 8)
          ;
          if (uVar13 == 0) goto LAB_141a39916;
          lVar12 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
        }
        else {
LAB_141a39916:
          lVar12 = 0;
        }
        ppuVar25 = (undefined **)((ulonglong)*(uint *)(lVar20 + 0x10) * 0x58 + lVar12);
        country_core::resources::roofs::roof_shape::Roof::ty();
        if ((extraout_AL_00 == '\0') || (*(int *)(ppuVar25 + 1) != 1)) {
LAB_141a39ba2:
          if ((bVar28 || uVar21 == 0) && (bVar27 || uVar23 == 0)) goto LAB_141a39c92;
        }
        else {
          uVar9 = *(uint *)((longlong)ppuVar25 + 0xc);
          uVar2 = *(uint *)(ppuVar25 + 2);
          country_core::resources::roofs::roof_ridge::RoofRidgeDims::from();
          country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space();
          if ((local_118 & 1) == 0) goto LAB_141a39ba2;
          uStack_e4 = uVar2;
          local_e8 = uVar9;
          if (*(char *)((longlong)ppuVar25 + 0x3c) != '\0') {
            local_e8 = uVar2 ^ 0x80000000;
            uStack_e4 = uVar9;
          }
          ::utils::swizzlers::impl$0::x0y();
          ::utils::xz_projective_rotation_bad();
          unaff_XMM7_Da = (fStack_10c + local_100) * 0.5;
          unaff_XMM8_Da = (fStack_114 + (float)local_108) * 0.5;
          unaff_XMM8_Db = (fStack_110 + (float)((ulonglong)local_108 >> 0x20)) * 0.5;
          local_218 = (undefined ***)CONCAT44(unaff_XMM8_Db,unaff_XMM8_Da);
          uStack_20c = 0x3f800000;
          uStack_208._0_4_ = 0x3f800000;
          uStack_208._4_4_ = 0x3f800000;
          unaff_XMM6_Da = 0x3f800000;
          fStack_210 = unaff_XMM7_Da;
          country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
          box_trigger_volume();
          pcStack_300 = country_core::resources::walls::impl$7::fmt;
          local_228 = &PTR_s_roof_ridge_dir_focused_142ebb1a8;
          pcStack_220 = (code *)0x1;
          fStack_210 = 1.4013e-45;
          uStack_20c = 0;
          uStack_208._0_4_ = 0;
          uStack_208._4_4_ = 0;
          local_b1 = 1;
          local_308 = ppuVar25;
          local_218 = &local_308;
          alloc::fmt::format::format_inner();
          local_138 = local_258;
          uStack_130 = uStack_250;
          local_128 = local_248;
          local_2f0 = local_150;
          puStack_2f8 = (undefined8 *)uStack_158;
          local_308 = (undefined **)CONCAT71(local_308._1_7_,2);
          local_148[0] = local_148[0] & 0xffffff00;
          local_b2 = 1;
          country_core::systems::ui::resources::widget_params::WidgetParams::new();
          local_b2 = 1;
          country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
          local_b2 = 0;
          puVar16 = &local_138;
          country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
          local_b1 = 0;
          pcStack_220 = (code *)puVar16;
          country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
          local_b1 = 0;
          toolbar::mei::core::NodeContext::id();
          unaff_RBX = extraout_RAX_03;
          unaff_XMM9_Da = local_e8;
          unaff_XMM9_Db = uStack_e4;
          if (extraout_AL_01 != '\0') {
            unaff_XMM6_Da = 0x3f99999a;
          }
        }
        local_138 = CONCAT44(unaff_XMM9_Db,unaff_XMM9_Da);
        local_148[0] = unaff_XMM6_Da;
        country_core::resources::walls::public_walls::PublicWalls::roof_entity();
        if (extraout_RAX_04 >> 0x20 != 0) {
          lVar12 = local_d0[1];
          uVar13 = extraout_RAX_04 & 0xffffffff;
          if ((uVar13 < *(ulonglong *)(lVar12 + 0x10)) &&
             (*(int *)(*(longlong *)(lVar12 + 8) + uVar13 * 0x14) == (int)(extraout_RAX_04 >> 0x20))
             ) {
            lVar20 = *(longlong *)(lVar12 + 8) + uVar13 * 0x14;
            uVar13 = (ulonglong)*(uint *)(lVar20 + 4);
            if ((uVar13 != 0xffffffff) &&
               ((lVar26 = *local_d0, uVar13 < *(ulonglong *)(lVar26 + 0x38) &&
                ((*(ulonglong *)
                   (*(longlong *)(lVar26 + 0x28) + (ulonglong)(*(uint *)(lVar20 + 4) >> 6) * 8) >>
                  (uVar13 & 0x3f) & 1) != 0)))) {
              if (*(ulonglong *)(lVar26 + 0x110) <
                  *(ulonglong *)
                   (*(longlong *)(lVar12 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar20 + 0xc) * 0x48
                   )) {
                lVar12 = *(longlong *)(lVar12 + 0x1a8) + (ulonglong)*(uint *)(lVar20 + 0xc) * 0x48;
                uVar13 = *(ulonglong *)
                          (*(longlong *)(lVar12 + 0x38) + *(ulonglong *)(lVar26 + 0x110) * 8);
                if (uVar13 == 0) {
                  lVar12 = 0;
                }
                else {
                  lVar12 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
                }
              }
              else {
                lVar12 = 0;
              }
              uVar9 = *(uint *)(lVar20 + 0x10);
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              puVar16 = (undefined8 *)(lVar12 + (ulonglong)uVar9 * 0x58);
              local_118 = (uint)extraout_RAX_08;
              fStack_114 = (float)((ulonglong)extraout_RAX_08 >> 0x20);
              fStack_110 = (float)unaff_RBX;
              fStack_10c = (float)((ulonglong)unaff_RBX >> 0x20);
              country_core::resources::roofs::roof_shape::Roof::height();
              local_1f8 = *in_stack_00000050 + 0x2c;
              local_228 = &local_160;
              pcStack_220 = core::fmt::float::impl$4::fmt;
              local_218 = (undefined ***)&local_138;
              fStack_210 = 10.506409;
              uStack_20c = 1;
              uStack_208 = local_148;
              local_200 = core::fmt::float::impl$5::fmt;
              local_1f0 = core::fmt::num::imp::impl$6::fmt;
              local_308 = &PTR_s_roof_ridge_dir__142ebb1b8;
              pcStack_300 = (code *)0x4;
              puStack_2f8 = &local_228;
              local_2f0 = 4;
              local_2e8 = (uint *)0x0;
              alloc::fmt::format::format_inner();
              uVar8 = local_138;
              uVar9 = local_148[0];
              local_c0 = (longlong *)local_240;
              std::alloc::__default_lib_allocator::__rust_alloc();
              local_c8 = local_238;
              if (extraout_RAX_09 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                alloc::alloc::handle_alloc_error();
              }
              *extraout_RAX_09 = 1;
              extraout_RAX_09[1] = 1;
              extraout_RAX_09[2] = uVar8;
              *(uint *)(extraout_RAX_09 + 3) = uVar9;
              *(ulonglong *)((longlong)extraout_RAX_09 + 0x1c) =
                   CONCAT44(unaff_XMM8_Db,unaff_XMM8_Da);
              *(float *)((longlong)extraout_RAX_09 + 0x24) = unaff_XMM7_Da;
              country_core::systems::ui::signifiers::SignifierFactory::new();
              local_2e8 = uStack_208;
              local_2f0 = CONCAT44(uStack_20c,fStack_210);
              puStack_2f8 = local_218;
              local_308 = local_228;
              pcStack_300 = pcStack_220;
              country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
              if (local_c0 != (longlong *)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
              if (extraout_AL_02 == '\x03') {
                **(undefined4 **)(in_stack_00000038 + 0x10) =
                     *(undefined4 *)(in_stack_00000038 + 0x1c);
                country_core::resources::walls::decorator_archivist::DecoratorArchivist::
                decorator_cull_allow_wall();
              }
              country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
              if (extraout_AL_03 != '\0') {
                uVar6 = *(undefined1 *)((longlong)puVar16 + 0x3c);
                bevy_ecs::event::writer::EventWriter::
                write<system_roof::ui_edit_roof_ridge_dir::SwitchRoofRidgeDirCmd>();
                local_228 = (undefined **)*puVar16;
                pcStack_220 = (code *)(CONCAT62(pcStack_220._2_6_,
                                                CONCAT11(uVar6,*(undefined1 *)
                                                                ((longlong)puVar16 + 0x3c))) ^ 0x100
                                      );
                bevy_ecs::event::writer::EventWriter::
                write<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>();
              }
              if (local_e0 == 0) {
                return;
              }
              local_308 = &local_d8;
              local_228 = (undefined **)&local_308;
              std::thread::local::LocalKey::
              with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl_6::call::closure_env_0<tuple_<>,puffin::impl_10::drop::closure_env_0>,tuple_<>_>
                        ();
              return;
            }
          }
        }
        goto LAB_141a39c92;
      }
    }
    uVar9 = (uint)uVar18;
    uVar19 = 1;
LAB_141a3a034:
    local_228 = (undefined **)CONCAT44(uVar9,uVar19);
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
LAB_141a39c92:
  if (local_e0 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_05 == 1) {
      local_c0 = extraout_RAX_05 + 1;
    }
    else if ((*extraout_RAX_05 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_c0 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_141a39d57:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*local_c0 != 0) {
LAB_141a3a018:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_c0 = -1;
    puffin::ThreadProfiler::end_scope();
    *local_c0 = *local_c0 + 1;
  }
  return;
}


