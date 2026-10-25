// 0x141a386b0 void __cdecl ui_edit_roof_ridge_dir(void)

void __cdecl system_roof::ui_edit_roof_ridge_dir::ui_edit_roof_ridge_dir(void)

{
  byte *pbVar1;
  undefined1 auVar2 [16];
  byte bVar3;
  byte bVar4;
  byte bVar5;
  int iVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  ushort uVar10;
  bool bVar11;
  bool bVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar13;
  uint uVar14;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  undefined8 *in_RCX;
  longlong *plVar15;
  ulonglong uVar16;
  longlong lVar17;
  longlong *in_R8;
  undefined *puVar18;
  byte local_294;
  longlong local_278;
  undefined1 (*pauStack_270) [16];
  ushort local_260;
  undefined4 uStack_258;
  undefined4 uStack_254;
  byte local_1a4;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar11 = false;
    bVar12 = false;
    iVar6 = *(int *)*in_RCX;
  }
  else {
    uVar16 = 0x3b;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar16 != 0) {
        if (uVar16 < 0x3b) {
          if ((char)(&DAT_142ebaea0)[uVar16] < -0x40) {
LAB_141a38774:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 != 0x3b) goto LAB_141a38774;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar16 != 0xfffffffffffffffe)) {
        if (uVar16 + 2 < 0x3b) {
          if ((char)(&UNK_142ebaea2)[uVar16] < -0x40) {
LAB_141a387ee:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar16 + 2 != 0x3b) goto LAB_141a387ee;
      }
    }
    puVar18 = &DAT_142ebb109;
    do {
      bVar3 = puVar18[-1];
      uVar13 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar18[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar18[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar18 + -4;
            puVar18 = puVar18 + -4;
            uVar13 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar18 = puVar18 + -3;
            uVar13 = bVar5 & 0xf;
          }
          uVar13 = bVar4 & 0x3f | uVar13 << 6;
        }
        else {
          puVar18 = puVar18 + -2;
          uVar13 = bVar4 & 0x1f;
        }
        uVar13 = (uint)(bVar3 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_141a388b4;
LAB_141a388c5:
        if (puVar18 == &UNK_142ebb0d7) break;
        if (puVar18 + -0x142ebb0d7 < (undefined *)0x31) {
          if (-0x41 < (char)puVar18[1]) break;
        }
        else if (puVar18 + -0x142ebb0d7 == (undefined *)0x31) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar18 = puVar18 + -1;
      if (uVar13 == 0x5c) goto LAB_141a388c5;
LAB_141a388b4:
      if (uVar13 == 0x2f) goto LAB_141a388c5;
    } while (puVar18 != &anon_cb06c9136c6217bb1323d596e337a917_27_llvm_16635302344178984818);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar15 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_141a38f6c;
    if (*plVar15 != 0) goto LAB_141a38f78;
    *plVar15 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar15 = *plVar15 + 1;
    bVar11 = true;
    bVar12 = true;
    iVar6 = *(int *)*in_RCX;
  }
  if (iVar6 == 0) {
    country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
    lVar17 = CONCAT44(uStack_254,uStack_258);
    lVar7 = *in_R8;
    lVar8 = in_R8[1];
    uVar13 = (uint)local_260;
    do {
      do {
        do {
          do {
            if (lVar17 == 0) {
LAB_141a38e89:
              if (!bVar12) {
                return;
              }
              goto LAB_141a38e95;
            }
            if ((short)uVar13 == 0) {
              do {
                auVar2 = *pauStack_270;
                uVar10 = (ushort)(SUB161(auVar2 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
                local_278 = local_278 + -0x2a80;
                pauStack_270 = pauStack_270 + 1;
              } while (uVar10 == 0xffff);
              uVar13 = ~(uint)uVar10;
              uVar14 = uVar13 & -(uint)uVar10 - 2;
            }
            else {
              if (local_278 == 0) goto LAB_141a38e89;
              uVar14 = uVar13 - 1 & uVar13;
            }
            for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            }
            country_core::resources::walls::public_walls::PublicWalls::get();
            if (extraout_RAX_02 == 0) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle();
            lVar17 = lVar17 + -1;
            uVar13 = uVar14;
          } while (((((local_294 & 1) == 0) ||
                    (country_core::resources::walls::public_walls::PublicWalls::roof_entity(),
                    extraout_RAX_03 >> 0x20 == 0)) ||
                   (uVar16 = extraout_RAX_03 & 0xffffffff, *(ulonglong *)(lVar8 + 0x10) <= uVar16))
                  || (lVar9 = *(longlong *)(lVar8 + 8),
                     *(int *)(lVar9 + uVar16 * 0x14) != (int)(extraout_RAX_03 >> 0x20)));
          uVar14 = *(uint *)(lVar9 + uVar16 * 0x14 + 4);
          uVar16 = (ulonglong)uVar14;
        } while (((uVar16 == 0xffffffff) || (*(ulonglong *)(lVar7 + 0x38) <= uVar16)) ||
                (((*(ulonglong *)(*(longlong *)(lVar7 + 0x28) + (ulonglong)(uVar14 >> 6) * 8) >>
                   (uVar16 & 0x3f) & 1) == 0 ||
                 (country_core::resources::roofs::roof_shape::Roof::ty(), extraout_AL_00 == '\0'))))
        ;
        country_core::resources::roofs::roof_ridge::RoofRidgeDims::from();
        country_core::resources::roofs::roof_ridge::RoofRidgeDims::to_world_space();
      } while ((local_1a4 & 1) == 0);
      ::utils::swizzlers::impl$0::x0y();
      ::utils::xz_projective_rotation_bad();
      country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
      box_trigger_volume();
      alloc::fmt::format::format_inner();
      country_core::systems::ui::resources::widget_params::WidgetParams::new();
      country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
    } while (extraout_AL_01 == '\0');
    toolbar::mei::core::NodeContext::id();
    bevy_ecs::event::writer::EventWriter::
    write<system_roof::ui_edit_roof_ridge_dir::DoUiEditRoofRidgeDir>();
    if (bVar12) {
      std::thread::local::LocalKey::
      with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl_6::call::closure_env_0<tuple_<>,puffin::impl_10::drop::closure_env_0>,tuple_<>_>
                ();
    }
  }
  else if (bVar11) {
LAB_141a38e95:
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar15 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_141a38f6c:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar15 != 0) {
LAB_141a38f78:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar15 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar15 = *plVar15 + 1;
  }
  return;
}


