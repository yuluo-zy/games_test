// signifier_animation_t 0x140a69340

void __cdecl country_core::systems::ui::signifiers::animation::signifier_animation_t(void)

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
  longlong lVar11;
  bool bVar12;
  bool bVar13;
  bool bVar14;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar15;
  longlong *extraout_RAX_01;
  longlong lVar16;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  uint uVar17;
  longlong *in_RCX;
  longlong *plVar18;
  ulonglong uVar19;
  longlong lVar20;
  ulonglong uVar21;
  undefined4 *puVar22;
  longlong *in_RDX;
  ulonglong uVar23;
  float *pfVar24;
  ulonglong uVar25;
  longlong lVar26;
  uint *puVar27;
  float fVar28;
  undefined4 extraout_XMM0_Da;
  float fVar29;
  undefined4 *local_108;
  undefined4 *local_100;
  longlong local_e0;
  longlong local_d8;
  longlong local_d0;
  longlong local_c8;
  longlong local_b8;
  longlong local_a8;
  undefined4 *local_98;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar12 = false;
  }
  else {
    uVar23 = 0x47;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar23 != 0) {
        if (uVar23 < 0x47) {
          if ((char)(&DAT_142ad4b20)[uVar23] < -0x40) {
LAB_140a693c1:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar23 != 0x47) goto LAB_140a693c1;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar23 != 0xfffffffffffffffe)) {
        if (uVar23 + 2 < 0x47) {
          if ((char)(&UNK_142ad4b22)[uVar23] < -0x40) {
LAB_140a6941f:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar23 + 2 != 0x47) goto LAB_140a6941f;
      }
    }
    puVar15 = &DAT_142ad51de;
    do {
      bVar3 = puVar15[-1];
      uVar17 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar15[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar15[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar15 + -4;
            puVar15 = puVar15 + -4;
            uVar17 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar15 = puVar15 + -3;
            uVar17 = bVar5 & 0xf;
          }
          uVar17 = bVar4 & 0x3f | uVar17 << 6;
        }
        else {
          puVar15 = puVar15 + -2;
          uVar17 = bVar4 & 0x1f;
        }
        uVar17 = (uint)(bVar3 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_140a694e9;
LAB_140a694ff:
        if (puVar15 == &UNK_142ad51a3) break;
        if (puVar15 + -0x142ad51a3 < (undefined *)0x3a) {
          if (-0x41 < (char)puVar15[1]) break;
        }
        else if (puVar15 + -0x142ad51a3 == (undefined *)0x3a) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar15 = puVar15 + -1;
      if (uVar17 == 0x5c) goto LAB_140a694ff;
LAB_140a694e9:
      if (uVar17 == 0x2f) goto LAB_140a694ff;
    } while (puVar15 != &DAT_142ad51a4);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_140a69ad2;
    if (*plVar18 != 0) goto LAB_140a69ade;
    *plVar18 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar18 = *plVar18 + 1;
    bVar12 = true;
  }
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar27 = *(uint **)(lVar8 + 0xf8);
  puVar2 = puVar27 + *(longlong *)(lVar8 + 0x100);
  cVar6 = *(char *)(lVar8 + 0x108);
  lVar10 = *in_RDX;
  local_e0 = 8;
  local_c8 = 0;
  bVar13 = false;
  local_a8 = 0;
  uVar25 = 0;
  uVar23 = 0;
  local_b8 = 0;
  bVar14 = false;
  do {
    if (cVar6 == '\0') {
      if (uVar25 == uVar23) {
        do {
          if (puVar27 == puVar2) goto LAB_140a69a3f;
          uVar17 = *puVar27;
          puVar27 = puVar27 + 1;
          uVar23 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar17 * 0xa0);
        } while (uVar23 == 0);
        lVar26 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar17 * 0xa0;
        uVar19 = (ulonglong)*(uint *)(lVar26 + 0x94);
        lVar20 = *(longlong *)(lVar9 + 0x1a8);
        lVar16 = *(longlong *)(lVar20 + 0x18 + uVar19 * 0x48);
        lVar11 = *(longlong *)(lVar20 + 0x38 + uVar19 * 0x48);
        uVar25 = *(ulonglong *)(lVar8 + 0x118);
        uVar21 = *(ulonglong *)(lVar26 + 0x88);
        if ((uVar25 < uVar21) && (*(longlong *)(*(longlong *)(lVar26 + 0x80) + uVar25 * 8) != 0)) {
          bVar13 = true;
          if ((*(ulonglong *)(lVar20 + 0x40 + uVar19 * 0x48) <= uVar25) ||
             (uVar25 = *(ulonglong *)(lVar11 + uVar25 * 8), uVar25 == 0)) {
            local_a8 = 0;
            uVar25 = *(ulonglong *)(lVar8 + 0x120);
            goto joined_r0x000140a6983b;
          }
          local_a8 = *(longlong *)(lVar16 + 0x10 + ~uVar25 * 0x30);
          uVar25 = *(ulonglong *)(lVar8 + 0x120);
          if (uVar25 < uVar21) goto LAB_140a698c2;
LAB_140a698fa:
          bVar14 = false;
        }
        else {
          bVar13 = false;
          uVar25 = *(ulonglong *)(lVar8 + 0x120);
joined_r0x000140a6983b:
          if (uVar21 <= uVar25) goto LAB_140a698fa;
LAB_140a698c2:
          if (*(longlong *)(*(longlong *)(lVar26 + 0x80) + uVar25 * 8) == 0) goto LAB_140a698fa;
          lVar20 = ~*(ulonglong *)(lVar11 + uVar25 * 8) * 0x30;
          local_b8 = *(longlong *)(lVar16 + 0x10 + lVar20);
          local_d0 = *(longlong *)(lVar16 + 0x28 + lVar20);
          bVar14 = true;
        }
        lVar20 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
        local_c8 = *(longlong *)(lVar16 + 0x10 + lVar20);
        local_d8 = *(longlong *)(lVar16 + 0x28 + lVar20);
        local_e0 = *(longlong *)(lVar26 + 0x50);
        uVar25 = 0;
      }
      uVar21 = (ulonglong)*(uint *)(local_e0 + 8 + uVar25 * 0x10);
      lVar16 = local_c8 + uVar21 * 0x1c;
      local_98 = (undefined4 *)(local_d8 + uVar21 * 4);
      lVar20 = 0;
      if (bVar13) {
        lVar20 = local_a8;
      }
      if (bVar14) {
        pfVar24 = (float *)(local_b8 + uVar21 * 8);
        puVar22 = (undefined4 *)(local_d0 + uVar21 * 4);
        local_100 = puVar22;
      }
      else {
        pfVar24 = (float *)0x0;
        puVar22 = local_100;
      }
    }
    else {
      if (uVar25 == uVar23) {
        do {
          if (puVar27 == puVar2) goto LAB_140a69a3f;
          uVar17 = *puVar27;
          puVar27 = puVar27 + 1;
          uVar23 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar17 * 0x48);
        } while (uVar23 == 0);
        lVar20 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar17 * 0x48;
        lVar16 = *(longlong *)(lVar20 + 0x18);
        lVar11 = *(longlong *)(lVar20 + 0x38);
        if ((*(ulonglong *)(lVar8 + 0x118) < *(ulonglong *)(lVar20 + 0x40)) &&
           (uVar25 = *(ulonglong *)(lVar11 + *(ulonglong *)(lVar8 + 0x118) * 8), uVar25 != 0)) {
          local_a8 = *(longlong *)(lVar16 + 0x10 + ~uVar25 * 0x30);
          bVar13 = true;
        }
        else {
          bVar13 = false;
        }
        lVar26 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30;
        local_c8 = *(longlong *)(lVar16 + 0x10 + lVar26);
        local_d8 = *(longlong *)(lVar16 + 0x28 + lVar26);
        if ((*(ulonglong *)(lVar8 + 0x120) < *(ulonglong *)(lVar20 + 0x40)) &&
           (uVar25 = *(ulonglong *)(lVar11 + *(ulonglong *)(lVar8 + 0x120) * 8), uVar25 != 0)) {
          lVar20 = ~uVar25 * 0x30;
          local_b8 = *(longlong *)(lVar16 + 0x10 + lVar20);
          local_d0 = *(longlong *)(lVar16 + 0x28 + lVar20);
          bVar14 = true;
          uVar25 = 0;
        }
        else {
          uVar25 = 0;
          bVar14 = false;
        }
      }
      uVar21 = uVar25 & 0xffffffff;
      lVar16 = local_c8 + uVar21 * 0x1c;
      local_98 = (undefined4 *)(local_d8 + uVar21 * 4);
      lVar20 = 0;
      if (bVar13) {
        lVar20 = local_a8;
      }
      if (bVar14) {
        pfVar24 = (float *)(local_b8 + uVar21 * 8);
        local_108 = (undefined4 *)(local_d0 + uVar21 * 4);
        puVar22 = local_108;
      }
      else {
        pfVar24 = (float *)0x0;
        puVar22 = local_108;
      }
    }
    fVar28 = *(float *)(lVar16 + 0x10);
    if ((fVar28 != 0.0) || (NAN(fVar28))) {
      fVar29 = *(float *)(lVar10 + 0x28);
      if (lVar20 != 0) {
        fVar29 = -fVar29;
      }
      *local_98 = uVar7;
      fVar29 = (1.0 / fVar28) * fVar29 + *(float *)(lVar16 + 0x14);
      fVar28 = 0.0;
      if (0.0 <= fVar29) {
        fVar28 = fVar29;
      }
      fVar29 = 1.0;
      if (fVar28 <= 1.0) {
        fVar29 = fVar28;
      }
    }
    else {
      fVar29 = 1.0;
      if (lVar20 != 0) {
        fVar29 = 0.0;
      }
      *local_98 = uVar7;
    }
    *(float *)(lVar16 + 0x14) = fVar29;
    ::utils::easing_functions::EasingFunction::apply();
    uVar25 = uVar25 + 1;
    *local_98 = uVar7;
    *(undefined4 *)(lVar16 + 0x18) = extraout_XMM0_Da;
    if (pfVar24 != (float *)0x0) {
      fVar28 = *(float *)(lVar10 + 0x28);
      *puVar22 = uVar7;
      *pfVar24 = fVar28 + *pfVar24;
    }
  } while( true );
LAB_140a69a3f:
  if (bVar12) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar18 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0)) {
LAB_140a69ad2:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_140a69ade:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar18 = *plVar18 + 1;
  }
  return;
}

