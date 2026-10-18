void __cdecl despawn_prefabs(void)


void __cdecl country_core::systems::prefab::despawn_prefabs(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  ulonglong *puVar8;
  longlong lVar9;
  longlong lVar10;
  undefined4 *puVar11;
  undefined4 *puVar12;
  char extraout_AL;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong uVar14;
  longlong lVar15;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *in_RCX;
  longlong *plVar16;
  undefined8 *in_RDX;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  longlong lVar20;
  longlong lVar21;
  longlong in_R8;
  undefined *puVar22;
  ulonglong uVar23;
  longlong in_R9;
  ulonglong uVar24;
  longlong in_stack_00000028;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar17 = 0x2e;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar17 != 0) {
        if (uVar17 < 0x2e) {
          if ((char)(&DAT_142a93faf)[uVar17] < -0x40) {
LAB_140842ddb:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 != 0x2e) goto LAB_140842ddb;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar17 != 0xfffffffffffffffe)) {
        if (uVar17 + 2 < 0x2e) {
          if ((char)(&UNK_142a93fb1)[uVar17] < -0x40) {
LAB_140842e39:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 + 2 != 0x2e) goto LAB_140842e39;
      }
    }
    puVar22 = &DAT_142a94181;
    do {
      bVar2 = puVar22[-1];
      uVar13 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar22[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar22[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar22 + -4;
            puVar22 = puVar22 + -4;
            uVar13 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar22 = puVar22 + -3;
            uVar13 = bVar4 & 0xf;
          }
          uVar13 = bVar3 & 0x3f | uVar13 << 6;
        }
        else {
          puVar22 = puVar22 + -2;
          uVar13 = bVar3 & 0x1f;
        }
        uVar13 = (uint)(bVar2 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_140842f04;
LAB_140842f15:
        if (puVar22 == &UNK_142a94157) break;
        if (puVar22 + -0x142a94157 < (undefined *)0x29) {
          if (-0x41 < (char)puVar22[1]) break;
        }
        else if (puVar22 + -0x142a94157 == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar22 = puVar22 + -1;
      if (uVar13 == 0x5c) goto LAB_140842f15;
LAB_140842f04:
      if (uVar13 == 0x2f) goto LAB_140842f15;
    } while (puVar22 != &DAT_142a94158);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar16 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_140843295:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar16 != 0) {
LAB_1408432a1:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar16 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar16 = *plVar16 + 1;
  }
  **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
  puVar8 = (ulonglong *)*in_RDX;
  lVar9 = in_RDX[1];
  uVar17 = *puVar8;
  uVar23 = 0;
  uVar14 = uVar17 - *(ulonglong *)(lVar9 + 0x18);
  if (uVar17 < *(ulonglong *)(lVar9 + 0x18)) {
    uVar14 = uVar23;
  }
  uVar24 = uVar17 - *(ulonglong *)(lVar9 + 0x38);
  if (uVar17 < *(ulonglong *)(lVar9 + 0x38)) {
    uVar24 = uVar23;
  }
  lVar20 = uVar14 * 0x10 + *(longlong *)(lVar9 + 8);
  uVar17 = *(ulonglong *)(lVar9 + 0x10) - uVar14;
  if (*(ulonglong *)(lVar9 + 0x10) < uVar14) {
    lVar20 = 8;
    uVar17 = uVar23;
  }
  lVar15 = uVar24 * 0x10 + *(longlong *)(lVar9 + 0x28);
  uVar14 = *(ulonglong *)(lVar9 + 0x30) - uVar24;
  if (*(ulonglong *)(lVar9 + 0x30) < uVar24) {
    lVar15 = 8;
    uVar14 = uVar23;
  }
  *puVar8 = (*(longlong *)(lVar9 + 0x40) - uVar14) - uVar17;
  lVar19 = uVar17 * 0x10 + lVar20;
  lVar18 = uVar14 * 0x10 + lVar15;
  lVar9 = *in_RCX;
  lVar10 = in_RCX[1];
  uVar5 = *(undefined4 *)((longlong)in_RCX + 0x14);
  uVar6 = *(undefined4 *)(in_R9 + 0x1c);
  puVar11 = *(undefined4 **)(in_R9 + 0x10);
  uVar7 = *(undefined4 *)(in_stack_00000028 + 0x1c);
  puVar12 = *(undefined4 **)(in_stack_00000028 + 0x10);
  if (lVar20 == 0) goto LAB_1408430e0;
  do {
    if (lVar20 == lVar19) goto LAB_1408430e0;
    lVar21 = lVar20;
    lVar20 = lVar20 + 0x10;
    while( true ) {
      *puVar8 = *puVar8 + 1;
      uVar17 = *(ulonglong *)(lVar21 + 8) & 0xffffffff;
      if ((uVar17 < *(ulonglong *)(lVar10 + 0x10)) &&
         (*(int *)(*(longlong *)(lVar10 + 8) + uVar17 * 0x14) ==
          (int)(*(ulonglong *)(lVar21 + 8) >> 0x20))) {
        lVar21 = *(longlong *)(lVar10 + 8) + uVar17 * 0x14;
        uVar17 = (ulonglong)*(uint *)(lVar21 + 4);
        if ((uVar17 != 0xffffffff) &&
           ((uVar17 < *(ulonglong *)(lVar9 + 0x38) &&
            ((*(ulonglong *)
               (*(longlong *)(lVar9 + 0x28) + (ulonglong)(*(uint *)(lVar21 + 4) >> 6) * 8) >>
              (uVar17 & 0x3f) & 1) != 0)))) {
          *(undefined4 *)
           (*(longlong *)
             (*(longlong *)
               (*(longlong *)(lVar10 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar21 + 0xc) * 0x48) +
              0x28 + ~*(ulonglong *)
                       (*(longlong *)
                         (*(longlong *)(lVar10 + 0x1a8) + 0x38 +
                         (ulonglong)*(uint *)(lVar21 + 0xc) * 0x48) +
                       *(longlong *)(lVar9 + 0x110) * 8) * 0x30) +
           (ulonglong)*(uint *)(lVar21 + 0x10) * 4) = uVar5;
          *puVar11 = uVar6;
          *puVar12 = uVar7;
          despawn_prefab_contents();
        }
      }
      bevy_ecs::system::commands::EntityCommands::despawn();
      if (lVar20 != 0) break;
LAB_1408430e0:
      if ((lVar15 == 0) || (lVar15 == lVar18)) {
        if (extraout_AL != '\0') {
          puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
          if (*extraout_RAX_02 == 1) {
            plVar16 = extraout_RAX_02 + 1;
          }
          else if ((*extraout_RAX_02 != 0) ||
                  (std::sys::thread_local::native::lazy::Storage::
                   initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                             (), plVar16 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
          goto LAB_140843295;
          if (*plVar16 != 0) goto LAB_1408432a1;
          *plVar16 = -1;
          puffin::ThreadProfiler::end_scope();
          *plVar16 = *plVar16 + 1;
        }
        return;
      }
      lVar20 = 0;
      lVar21 = lVar15;
      lVar15 = lVar15 + 0x10;
    }
  } while( true );
}

