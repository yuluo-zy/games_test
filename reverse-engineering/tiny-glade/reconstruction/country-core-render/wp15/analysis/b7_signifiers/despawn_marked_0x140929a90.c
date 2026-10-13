// addr=0x140929a90 name=despawn_marked
// signature=void __cdecl despawn_marked(void)


void __cdecl country_core::systems::ui::signifiers::signifier_mark_for_delete::despawn_marked(void)

{
  byte *pbVar1;
  uint *puVar2;
  float fVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  char cVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  bool bVar11;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar12;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  uint uVar13;
  longlong *in_RCX;
  ulonglong uVar14;
  longlong lVar15;
  longlong *plVar16;
  ulonglong uVar17;
  uint *puVar18;
  ulonglong uVar19;
  longlong local_90;
  longlong local_78;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar17 = 0x50;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar17 != 0) {
        if (uVar17 < 0x50) {
          if ((char)(&DAT_142aae665)[uVar17] < -0x40) {
LAB_140929b08:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 != 0x50) goto LAB_140929b08;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar17 != 0xfffffffffffffffe)) {
        if (uVar17 + 2 < 0x50) {
          if ((char)(&UNK_142aae667)[uVar17] < -0x40) {
LAB_140929b66:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 + 2 != 0x50) goto LAB_140929b66;
      }
    }
    puVar12 = &DAT_142aaf586;
    do {
      bVar4 = puVar12[-1];
      uVar13 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar12[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar12[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar12 + -4;
            puVar12 = puVar12 + -4;
            uVar13 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar12 = puVar12 + -3;
            uVar13 = bVar6 & 0xf;
          }
          uVar13 = bVar5 & 0x3f | uVar13 << 6;
        }
        else {
          puVar12 = puVar12 + -2;
          uVar13 = bVar5 & 0x1f;
        }
        uVar13 = (uint)(bVar4 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_140929c29;
LAB_140929c3f:
        if (puVar12 == &UNK_142aaf53b) break;
        if (puVar12 + -0x142aaf53b < (undefined *)0x4a) {
          if (-0x41 < (char)puVar12[1]) break;
        }
        else if (puVar12 + -0x142aaf53b == (undefined *)0x4a) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar12 = puVar12 + -1;
      if (uVar13 == 0x5c) goto LAB_140929c3f;
LAB_140929c29:
      if (uVar13 == 0x2f) goto LAB_140929c3f;
    } while (puVar12 != &DAT_142aaf53c);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar16 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_140929ff9;
    if (*plVar16 != 0) goto LAB_14092a005;
    *plVar16 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar16 = *plVar16 + 1;
  }
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  puVar18 = *(uint **)(lVar8 + 0xf8);
  puVar2 = puVar18 + *(longlong *)(lVar8 + 0x100);
  cVar7 = *(char *)(lVar8 + 0x108);
  uVar17 = 0;
  local_90 = 8;
  uVar19 = 0;
  local_78 = 0;
  bVar11 = false;
  while (cVar7 != '\0') {
    if (uVar19 == uVar17) {
      do {
        if (puVar18 == puVar2) goto LAB_140929f73;
        uVar13 = *puVar18;
        puVar18 = puVar18 + 1;
        uVar17 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar13 * 0x48);
      } while (uVar17 == 0);
      lVar15 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar13 * 0x48;
      if ((*(ulonglong *)(lVar8 + 0x118) < *(ulonglong *)(lVar15 + 0x40)) &&
         (uVar19 = *(ulonglong *)(*(longlong *)(lVar15 + 0x38) + *(ulonglong *)(lVar8 + 0x118) * 8),
         uVar19 != 0)) {
        local_78 = *(longlong *)(*(longlong *)(lVar15 + 0x18) + 0x10 + ~uVar19 * 0x30);
        bVar11 = true;
      }
      else {
        bVar11 = false;
      }
      uVar19 = 0;
    }
    if (!bVar11) goto LAB_140929d70;
    uVar14 = uVar19 & 0xffffffff;
LAB_140929f3f:
    lVar15 = uVar14 * 0x1c + local_78;
    uVar19 = uVar19 + 1;
    if ((lVar15 == 0) || ((fVar3 = *(float *)(lVar15 + 0x14), fVar3 == 0.0 && (!NAN(fVar3))))) {
LAB_140929d73:
      bevy_ecs::system::commands::EntityCommands::despawn();
    }
  }
  if (uVar19 == uVar17) {
    do {
      if (puVar18 == puVar2) goto LAB_140929f73;
      uVar13 = *puVar18;
      puVar18 = puVar18 + 1;
      uVar17 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar13 * 0xa0);
    } while (uVar17 == 0);
    lVar15 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar13 * 0xa0;
    uVar19 = *(ulonglong *)(lVar8 + 0x118);
    if ((uVar19 < *(ulonglong *)(lVar15 + 0x88)) &&
       (*(longlong *)(*(longlong *)(lVar15 + 0x80) + uVar19 * 8) != 0)) {
      uVar14 = (ulonglong)*(uint *)(lVar15 + 0x94);
      lVar10 = *(longlong *)(lVar9 + 0x1a8);
      bVar11 = true;
      if ((uVar19 < *(ulonglong *)(lVar10 + 0x40 + uVar14 * 0x48)) &&
         (uVar19 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38 + uVar14 * 0x48) + uVar19 * 8),
         uVar19 != 0)) {
        local_78 = *(longlong *)
                    (*(longlong *)(lVar10 + uVar14 * 0x48 + 0x18) + 0x10 + ~uVar19 * 0x30);
      }
      else {
        local_78 = 0;
      }
    }
    else {
      bVar11 = false;
    }
    local_90 = *(longlong *)(lVar15 + 0x50);
    uVar19 = 0;
  }
  if (bVar11) {
    uVar14 = (ulonglong)*(uint *)(uVar19 * 0x10 + local_90 + 8);
    goto LAB_140929f3f;
  }
LAB_140929d70:
  uVar19 = uVar19 + 1;
  goto LAB_140929d73;
LAB_140929f73:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar16 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_140929ff9:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar16 != 0) {
LAB_14092a005:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar16 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar16 = *plVar16 + 1;
  }
  return;
}


