// name=ui_edit_roof
// sig=void __cdecl ui_edit_roof(void)
// size=2601

void __cdecl system_roof::ui_edit_roof::ui_edit_roof(void)

{
  byte *pbVar1;
  uint *puVar2;
  longlong lVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  int iVar7;
  ulonglong *puVar8;
  longlong lVar9;
  ulonglong uVar10;
  undefined8 uVar11;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined1 extraout_AL_02;
  char extraout_AL_03;
  undefined1 extraout_AL_04;
  uint uVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong lVar13;
  undefined8 *puVar14;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  longlong *plVar15;
  undefined8 *in_RDX;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  undefined8 *puVar19;
  longlong *in_R8;
  char *pcVar20;
  undefined8 *in_R9;
  ulonglong uVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  uint *puVar24;
  longlong in_stack_00000030;
  undefined8 *local_300;
  code *local_2f8;
  undefined8 uStack_2f0;
  undefined8 **local_2e8;
  undefined1 local_250;
  char local_24f;
  undefined *local_1d8;
  undefined8 uStack_1d0;
  undefined8 **local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined4 local_128;
  undefined8 local_120;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 *local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  undefined8 *local_88;
  longlong *local_80;
  longlong *local_78;
  longlong local_70;
  undefined8 local_68;
  undefined8 local_60;
  longlong *local_58;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_58 = (longlong *)CONCAT71(local_58._1_7_,extraout_AL);
  if (extraout_AL == '\0') {
    local_f8 = 0;
    iVar7 = *(int *)*in_RCX;
  }
  else {
    uVar16 = 0x27;
    local_88 = in_RDX;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x27) {
          if ((char)(&DAT_142eaf340)[uVar16] < -0x40) {
LAB_141a18e57:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 != 0x27) goto LAB_141a18e57;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar16 != 0xfffffffffffffffe)) {
        if (uVar16 + 2 < 0x27) {
          if ((char)(&UNK_142eaf342)[uVar16] < -0x40) {
LAB_141a18ec8:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 + 2 != 0x27) goto LAB_141a18ec8;
      }
    }
    pcVar20 = "roof_edit_";
    do {
      bVar4 = pcVar20[-1];
      uVar12 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = pcVar20[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = pcVar20[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = (byte *)(pcVar20 + -4);
            pcVar20 = pcVar20 + -4;
            uVar12 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar20 = pcVar20 + -3;
            uVar12 = bVar6 & 0xf;
          }
          uVar12 = bVar5 & 0x3f | uVar12 << 6;
        }
        else {
          pcVar20 = pcVar20 + -2;
          uVar12 = bVar5 & 0x1f;
        }
        uVar12 = (uint)(bVar4 & 0x3f) | uVar12 << 6;
        if (uVar12 != 0x5c) goto LAB_141a18f84;
LAB_141a18f95:
        if (pcVar20 == "") break;
        if (pcVar20 + -0x142eaf527 < (char *)0x27) {
          if (-0x41 < pcVar20[1]) break;
        }
        else if (pcVar20 + -0x142eaf527 == (char *)0x27) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar20 = pcVar20 + -1;
      if (uVar12 == 0x5c) goto LAB_141a18f95;
LAB_141a18f84:
      if (uVar12 == 0x2f) goto LAB_141a18f95;
    } while (pcVar20 != "crates/systems/roof/src/ui_edit_roof.rsroof_edit_");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      local_78 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_78 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_141a197f9;
    if (*local_78 != 0) goto LAB_141a19805;
    *local_78 = -1;
    puffin::ThreadProfiler::begin_scope();
    *local_78 = *local_78 + 1;
    local_f8 = 1;
    iVar7 = *(int *)*in_RCX;
    in_RDX = local_88;
  }
  if (iVar7 == 0) {
    puVar8 = (ulonglong *)*in_R9;
    lVar9 = in_R9[1];
    uVar16 = *puVar8;
    uVar18 = 0;
    uVar23 = uVar16 - *(ulonglong *)(lVar9 + 0x18);
    if (uVar16 < *(ulonglong *)(lVar9 + 0x18)) {
      uVar23 = uVar18;
    }
    uVar17 = uVar16 - *(ulonglong *)(lVar9 + 0x38);
    if (uVar16 < *(ulonglong *)(lVar9 + 0x38)) {
      uVar17 = uVar18;
    }
    uVar16 = *(ulonglong *)(lVar9 + 0x30);
    uVar21 = uVar16 - uVar17;
    if (uVar17 > uVar16) {
      uVar21 = uVar18;
    }
    uVar10 = *(ulonglong *)(lVar9 + 0x10);
    uVar22 = uVar10 - uVar23;
    if (uVar23 > uVar10) {
      uVar22 = uVar18;
    }
    uVar18 = *(ulonglong *)(lVar9 + 0x40);
    *puVar8 = uVar18 - (uVar22 + uVar21);
    if (uVar23 <= uVar10 && uVar10 - uVar23 != 0 || uVar17 <= uVar16 && uVar16 - uVar17 != 0) {
      *puVar8 = uVar18;
    }
    else {
      local_90 = *in_R8;
      puVar24 = *(uint **)(local_90 + 0xf8);
      local_80 = *(longlong **)(in_stack_00000030 + 8);
      if (local_80 == (longlong *)0x0) {
        local_80 = *(longlong **)(in_stack_00000030 + 0x10);
      }
      lVar9 = in_R8[1];
      puVar2 = puVar24 + *(longlong *)(local_90 + 0x100);
      uVar16 = (ulonglong)local_88 >> 8;
      local_88 = (undefined8 *)CONCAT71((int7)uVar16,*(undefined1 *)(local_90 + 0x108));
      local_b8 = *in_RDX;
      local_a0 = 8;
      local_70 = 0;
      local_98 = 8;
      local_68 = 0;
      local_60 = 0;
      uVar16 = 0;
      uVar23 = 0;
      do {
        do {
          if ((char)local_88 == '\0') {
            if (uVar23 == uVar16) {
              do {
                if (puVar24 == puVar2) goto LAB_141a1906e;
                uVar12 = *puVar24;
                puVar24 = puVar24 + 1;
                uVar16 = *(ulonglong *)
                          (*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar12 * 0xa0);
              } while (uVar16 == 0);
              lVar13 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar12 * 0xa0;
              uVar23 = (ulonglong)*(uint *)(lVar13 + 0x94);
              lVar3 = *(longlong *)(lVar9 + 0x1a8) + uVar23 * 0x48;
              uVar23 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x40 + uVar23 * 0x48);
              if ((*(ulonglong *)(local_90 + 0x110) < uVar23) &&
                 (uVar18 = *(ulonglong *)
                            (*(longlong *)(lVar3 + 0x38) + *(ulonglong *)(local_90 + 0x110) * 8),
                 uVar18 != 0)) {
                local_70 = *(longlong *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar18 * 0x30);
                uVar18 = *(ulonglong *)(local_90 + 0x118);
                if (uVar23 <= uVar18) goto LAB_141a19401;
LAB_141a19386:
                uVar18 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + uVar18 * 8);
                if (uVar18 == 0) goto LAB_141a19401;
                local_68 = *(undefined8 *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar18 * 0x30);
                uVar18 = *(ulonglong *)(local_90 + 0x120);
                if (uVar18 < uVar23) goto LAB_141a19418;
LAB_141a1946b:
                local_60 = 0;
              }
              else {
                local_70 = 0;
                uVar18 = *(ulonglong *)(local_90 + 0x118);
                if (uVar18 < uVar23) goto LAB_141a19386;
LAB_141a19401:
                local_68 = 0;
                uVar18 = *(ulonglong *)(local_90 + 0x120);
                if (uVar23 <= uVar18) goto LAB_141a1946b;
LAB_141a19418:
                uVar23 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + uVar18 * 8);
                if (uVar23 == 0) goto LAB_141a1946b;
                local_60 = *(undefined8 *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar23 * 0x30);
              }
              local_98 = *(longlong *)(lVar13 + 0x50);
              uVar23 = 0;
            }
            puVar14 = (undefined8 *)(local_98 + uVar23 * 0x10);
            uVar18 = (ulonglong)*(uint *)(local_98 + 8 + uVar23 * 0x10);
          }
          else {
            if (uVar23 == uVar16) {
              do {
                if (puVar24 == puVar2) goto LAB_141a1906e;
                uVar12 = *puVar24;
                puVar24 = puVar24 + 1;
                uVar16 = *(ulonglong *)
                          (*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar12 * 0x48);
              } while (uVar16 == 0);
              lVar3 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar12 * 0x48;
              uVar23 = *(ulonglong *)(lVar3 + 0x40);
              if ((*(ulonglong *)(local_90 + 0x110) < uVar23) &&
                 (uVar18 = *(ulonglong *)
                            (*(longlong *)(lVar3 + 0x38) + *(ulonglong *)(local_90 + 0x110) * 8),
                 uVar18 != 0)) {
                local_70 = *(longlong *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar18 * 0x30);
                uVar18 = *(ulonglong *)(local_90 + 0x118);
                if (uVar23 <= uVar18) goto LAB_141a193c0;
LAB_141a19335:
                uVar18 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + uVar18 * 8);
                if (uVar18 == 0) goto LAB_141a193c0;
                local_68 = *(undefined8 *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar18 * 0x30);
                uVar18 = *(ulonglong *)(local_90 + 0x120);
                if (uVar18 < uVar23) goto LAB_141a193d7;
LAB_141a19442:
                local_60 = 0;
              }
              else {
                local_70 = 0;
                uVar18 = *(ulonglong *)(local_90 + 0x118);
                if (uVar18 < uVar23) goto LAB_141a19335;
LAB_141a193c0:
                local_68 = 0;
                uVar18 = *(ulonglong *)(local_90 + 0x120);
                if (uVar23 <= uVar18) goto LAB_141a19442;
LAB_141a193d7:
                uVar23 = *(ulonglong *)(*(longlong *)(lVar3 + 0x38) + uVar18 * 8);
                if (uVar23 == 0) goto LAB_141a19442;
                local_60 = *(undefined8 *)(*(longlong *)(lVar3 + 0x18) + 0x10 + ~uVar23 * 0x30);
              }
              local_a0 = *(longlong *)(lVar3 + 8);
              uVar23 = 0;
            }
            puVar14 = (undefined8 *)(local_a0 + uVar23 * 8);
            uVar18 = uVar23 & 0xffffffff;
          }
          puVar19 = (undefined8 *)(uVar18 * 0x58 + local_70);
          local_c0 = *puVar14;
          country_core::resources::roofs::roof_animation::RoofAnimation::is_being_animated();
          uVar23 = uVar23 + 1;
        } while (extraout_AL_00 != '\0');
        local_2f8 = country_core::resources::walls::impl$7::fmt;
        local_1d8 = &DAT_142eaf560;
        uStack_1d0 = 1;
        local_1c8 = &local_300;
        local_1c0 = 1;
        local_1b8 = 0;
        local_300 = puVar19;
        alloc::fmt::format::format_inner();
        local_e8 = local_110;
        uStack_e0 = uStack_108;
        local_d8 = local_100;
        local_49 = 1;
        country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::triangles();
        local_49 = 1;
        alloc::vec::spec_from_iter::impl$0::
        from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,country_core::resources::roofs::roof_trigger_volume::impl_1::triangles::closure_env_0>_>
                  ();
        local_2e8 = local_1c8;
        local_2f8 = (code *)local_1d8;
        uStack_2f0 = uStack_1d0;
        local_300 = (undefined8 *)CONCAT71(local_300._1_7_,2);
        local_4b = 1;
        country_core::resources::roofs::roof_shape::Roof::ty();
        local_24f = '\f' - extraout_AL_01;
        local_250 = 1;
        local_4b = 0;
        country_core::systems::ui::resources::widget_params::WidgetParams::new();
        local_4a = 1;
        country_core::systems::ui::resources::widget_params::UiSounds::boop();
        local_120 = 2;
        local_128 = 1;
        local_4a = 1;
        country_core::systems::ui::resources::widget_params::UiSounds::on_hover_start();
        local_4a = 0;
        country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
        local_49 = 0;
        puVar14 = &local_e8;
        country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
        local_a8 = puVar14;
        country_core::systems::ui::resources::ui_state::UiWidgetContext::hovered();
        local_1d8 = (undefined *)CONCAT71(local_1d8._1_7_,extraout_AL_02);
        country_core::resources::walls::inner_wall_state::StructureType::is_fence();
        if (extraout_AL_03 != '\0') {
          uVar11 = *puVar19;
          lVar3 = local_80[2];
          if ((ulonglong)(*local_80 - lVar3) < 0x18) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar13 = local_80[1];
          *(code **)(lVar13 + lVar3) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::send_event::closure_env_0<country_core::resources::roofs::commands::HighlightRoofCmd>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(undefined8 *)(lVar13 + 8 + lVar3) = uVar11;
          *(undefined1 *)(lVar13 + 0x10 + lVar3) = 2;
          local_80[2] = lVar3 + 0x18;
        }
        country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting();
      } while ((int)local_1d8 == 2);
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      uStack_1d0 = CONCAT71(uStack_1d0._1_7_,extraout_AL_04);
      local_1d8 = (undefined *)local_c0;
      bevy_ecs::event::writer::EventWriter::
      write<system_roof::ui_edit_roof_ridge_dir::RecordRidgeSwitchHistoryCmd>();
    }
    if ((char)local_58 == '\0') {
      return;
    }
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar15 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_141a197f9;
    if (*plVar15 != 0) goto LAB_141a19805;
    *plVar15 = -1;
    local_58 = plVar15;
    puffin::ThreadProfiler::end_scope();
  }
  else {
LAB_141a1906e:
    if ((char)local_58 == '\0') {
      return;
    }
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar15 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_141a197f9:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar15 != 0) {
LAB_141a19805:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar15 = -1;
    local_58 = plVar15;
    puffin::ThreadProfiler::end_scope();
  }
  *local_58 = *local_58 + 1;
  return;
}

