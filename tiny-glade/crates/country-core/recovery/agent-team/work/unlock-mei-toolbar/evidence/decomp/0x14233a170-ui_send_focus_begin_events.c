
void __cdecl toolbar::ui_interaction::ui_send_focus_begin_events(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  undefined4 uVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  undefined4 *puVar11;
  undefined8 uVar12;
  longlong lVar13;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar14;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong *puVar15;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *in_RCX;
  ulonglong uVar16;
  longlong *plVar17;
  longlong *in_RDX;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  uint *puVar21;
  char *pcVar22;
  longlong local_a0;
  longlong local_98;
  longlong local_58;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar18 = 0x33;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar18 != 0) {
        if (uVar18 < 0x33) {
          if ((char)(&DAT_14304da3d)[uVar18] < -0x40) {
LAB_14233a1e8:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar18 != 0x33) goto LAB_14233a1e8;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar18 != 0xfffffffffffffffe)) {
        if (uVar18 + 2 < 0x33) {
          if ((char)(&UNK_14304da3f)[uVar18] < -0x40) {
LAB_14233a246:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar18 + 2 != 0x33) goto LAB_14233a246;
      }
    }
    pcVar22 = "toolbar_button_";
    do {
      bVar3 = pcVar22[-1];
      uVar14 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = pcVar22[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = pcVar22[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = (byte *)(pcVar22 + -4);
            pcVar22 = pcVar22 + -4;
            uVar14 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar22 = pcVar22 + -3;
            uVar14 = bVar5 & 0xf;
          }
          uVar14 = bVar4 & 0x3f | uVar14 << 6;
        }
        else {
          pcVar22 = pcVar22 + -2;
          uVar14 = bVar4 & 0x1f;
        }
        uVar14 = (uint)(bVar3 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto LAB_14233a304;
LAB_14233a315:
        if (pcVar22 == "") break;
        if (pcVar22 + -0x14304dcd7 < (char *)0x2c) {
          if (-0x41 < pcVar22[1]) break;
        }
        else if (pcVar22 + -0x14304dcd7 == (char *)0x2c) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar22 = pcVar22 + -1;
      if (uVar14 == 0x5c) goto LAB_14233a315;
LAB_14233a304:
      if (uVar14 == 0x2f) goto LAB_14233a315;
    } while (pcVar22 != "crates/systems/toolbar/src/ui_interaction.rstoolbar_button_");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar17 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar17 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_14233a6ec;
    if (*plVar17 != 0) goto LAB_14233a6f8;
    *plVar17 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar17 = *plVar17 + 1;
  }
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  puVar21 = *(uint **)(lVar8 + 0xf8);
  puVar2 = puVar21 + *(longlong *)(lVar8 + 0x100);
  cVar6 = *(char *)(lVar8 + 0x108);
  uVar7 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar10 = *in_RDX;
  puVar11 = (undefined4 *)in_RDX[2];
  local_a0 = 8;
  local_58 = 0;
  local_98 = 8;
  uVar18 = 0;
  uVar19 = 0;
LAB_14233a43b:
  do {
    if (cVar6 == '\0') {
      if (uVar19 == uVar18) {
        do {
          if (puVar21 == puVar2) goto LAB_14233a660;
          uVar14 = *puVar21;
          puVar21 = puVar21 + 1;
          uVar18 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar14 * 0xa0);
        } while (uVar18 == 0);
        lVar20 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar14 * 0xa0;
        uVar19 = (ulonglong)*(uint *)(lVar20 + 0x94);
        if (*(ulonglong *)(lVar8 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x40 + uVar19 * 0x48)) {
          lVar13 = *(longlong *)(lVar9 + 0x1a8) + uVar19 * 0x48;
          uVar19 = *(ulonglong *)(*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8);
          if (uVar19 == 0) goto LAB_14233a567;
          local_58 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar19 * 0x30);
        }
        else {
LAB_14233a567:
          local_58 = 0;
        }
        local_98 = *(longlong *)(lVar20 + 0x50);
        uVar19 = 0;
      }
      puVar15 = (ulonglong *)(local_98 + uVar19 * 0x10);
      uVar16 = (ulonglong)*(uint *)(local_98 + 8 + uVar19 * 0x10);
    }
    else {
      if (uVar19 == uVar18) {
        do {
          if (puVar21 == puVar2) goto LAB_14233a660;
          uVar14 = *puVar21;
          puVar21 = puVar21 + 1;
          uVar18 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar14 * 0x48);
        } while (uVar18 == 0);
        lVar20 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar14 * 0x48;
        if ((*(ulonglong *)(lVar8 + 0x110) < *(ulonglong *)(lVar20 + 0x40)) &&
           (uVar19 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8),
           uVar19 != 0)) {
          local_58 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar19 * 0x30);
        }
        else {
          local_58 = 0;
        }
        local_a0 = *(longlong *)(lVar20 + 8);
        uVar19 = 0;
      }
      puVar15 = (ulonglong *)(local_a0 + uVar19 * 8);
      uVar16 = uVar19 & 0xffffffff;
    }
    lVar20 = uVar16 * 3 + local_58;
    uVar16 = *puVar15;
    if (uVar16 >> 0x20 == 0) goto LAB_14233a660;
    uVar19 = uVar19 + 1;
    if (*(char *)(lVar20 + 1) == '\0') break;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    if (extraout_AL_00 == '\0') goto LAB_14233a5f7;
  } while( true );
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  hashbrown::raw::RawTable::
  remove_entry<tuple_<bevy_ecs::entity::Entity,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<bevy_ecs::entity::Entity,bevy_ecs::entity::Entity,tuple_<>_>_>
            ();
  if (extraout_RAX_02 >> 0x20 == 0) {
LAB_14233a5f7:
    if (*(char *)(lVar20 + 1) == '\x01') {
      *puVar11 = uVar7;
      lVar20 = *(longlong *)(lVar10 + 0x30);
      uVar12 = *(undefined8 *)(lVar10 + 0x40);
      if (lVar20 == *(longlong *)(lVar10 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar13 = *(longlong *)(lVar10 + 0x28);
      *(undefined8 *)(lVar13 + lVar20 * 0x10) = uVar12;
      *(ulonglong *)(lVar13 + 8 + lVar20 * 0x10) = uVar16;
      *(longlong *)(lVar10 + 0x30) = lVar20 + 1;
      *(longlong *)(lVar10 + 0x40) = *(longlong *)(lVar10 + 0x40) + 1;
    }
  }
  goto LAB_14233a43b;
LAB_14233a660:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar17 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar17 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_14233a6ec:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar17 != 0) {
LAB_14233a6f8:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar17 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar17 = *plVar17 + 1;
  }
  return;
}

