// addr=0x140a69c30 name=signifier_animation
// signature=void __cdecl signifier_animation(void)


// WARNING: Control flow encountered bad instruction data

void __cdecl country_core::systems::ui::signifiers::animation::signifier_animation(void)

{
  byte *pbVar1;
  uint *puVar2;
  undefined4 *puVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  int *piVar9;
  longlong lVar10;
  longlong lVar11;
  undefined8 uVar12;
  code *pcVar13;
  bool bVar14;
  int iVar15;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar16;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  uint uVar17;
  undefined8 *in_RCX;
  longlong *plVar18;
  ulonglong uVar19;
  longlong lVar20;
  longlong *in_RDX;
  ulonglong uVar21;
  ulonglong uVar22;
  bool bVar23;
  longlong lVar24;
  ulonglong uVar25;
  uint *puVar26;
  longlong lVar27;
  undefined8 *puVar28;
  longlong lVar29;
  ulonglong uVar30;
  bool bVar31;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar32;
  float in_XMM1_Da;
  undefined4 in_XMM1_Db;
  float fVar33;
  longlong local_70;
  longlong local_68;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar14 = false;
    fVar32 = extraout_XMM0_Da;
  }
  else {
    uVar21 = 0x45;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar21 != 0) {
        if (uVar21 < 0x45) {
          if ((char)(&DAT_142ad4bac)[uVar21] < -0x40) {
LAB_140a69ca1:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar21 != 0x45) goto LAB_140a69ca1;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar21 != 0xfffffffffffffffe)) {
        if (uVar21 + 2 < 0x45) {
          if ((char)(&UNK_142ad4bae)[uVar21] < -0x40) {
LAB_140a69cff:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar21 + 2 != 0x45) goto LAB_140a69cff;
      }
    }
    puVar16 = &DAT_142ad51de;
    do {
      bVar4 = puVar16[-1];
      uVar17 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar16[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar16[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar16 + -4;
            puVar16 = puVar16 + -4;
            uVar17 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar16 = puVar16 + -3;
            uVar17 = bVar6 & 0xf;
          }
          uVar17 = bVar5 & 0x3f | uVar17 << 6;
        }
        else {
          puVar16 = puVar16 + -2;
          uVar17 = bVar5 & 0x1f;
        }
        uVar17 = (uint)(bVar4 & 0x3f) | uVar17 << 6;
        if (uVar17 != 0x5c) goto LAB_140a69dc9;
LAB_140a69ddf:
        if (puVar16 == &UNK_142ad51a3) break;
        if (puVar16 + -0x142ad51a3 < (undefined *)0x3a) {
          if (-0x41 < (char)puVar16[1]) break;
        }
        else if (puVar16 + -0x142ad51a3 == (undefined *)0x3a) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar16 = puVar16 + -1;
      if (uVar17 == 0x5c) goto LAB_140a69ddf;
LAB_140a69dc9:
      if (uVar17 == 0x2f) goto LAB_140a69ddf;
    } while (puVar16 != &DAT_142ad51a4);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_140a6a371:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_140a6a37d:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar18 = *plVar18 + 1;
    bVar14 = true;
    fVar32 = extraout_XMM0_Da_00;
  }
  piVar9 = (int *)*in_RCX;
  lVar10 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar26 = *(uint **)(piVar9 + 0x3e);
  puVar2 = puVar26 + *(longlong *)(piVar9 + 0x40);
  iVar15 = piVar9[0x42];
  lVar11 = *in_RDX;
  lVar29 = 8;
  lVar24 = 0;
  bVar23 = false;
  uVar25 = 0;
  uVar30 = 0;
  uVar21 = 0;
  local_68 = 0;
  bVar31 = false;
LAB_140a69eec:
  do {
    if ((char)iVar15 == '\0') {
      if (uVar30 == uVar21) {
        do {
          if (puVar26 == puVar2) goto LAB_140a6a2ee;
          uVar17 = *puVar26;
          puVar26 = puVar26 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar17 * 0xa0);
        } while (uVar21 == 0);
        lVar27 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar17 * 0xa0;
        uVar30 = (ulonglong)*(uint *)(lVar27 + 0x94);
        lVar29 = *(longlong *)(lVar10 + 0x1a8);
        lVar24 = lVar29 + uVar30 * 0x48;
        lVar20 = *(longlong *)(lVar29 + 0x38 + uVar30 * 0x48);
        uVar30 = *(ulonglong *)(lVar29 + 0x40 + uVar30 * 0x48);
        if (*(ulonglong *)(piVar9 + 0x44) < uVar30) {
          uVar19 = *(ulonglong *)(lVar20 + *(ulonglong *)(piVar9 + 0x44) * 8);
          lVar29 = *(longlong *)(lVar24 + 0x18);
          if (uVar19 == 0) goto LAB_140a6a03b;
          lVar24 = *(longlong *)(lVar29 + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(piVar9 + 0x48);
          uVar22 = *(ulonglong *)(lVar27 + 0x88);
          if (uVar19 < uVar22) goto LAB_140a6a051;
LAB_140a6a08e:
          bVar23 = false;
          uVar30 = *(ulonglong *)(piVar9 + 0x4a);
          if (uVar30 < uVar22) goto LAB_140a6a09c;
LAB_140a6a0c1:
          bVar31 = false;
        }
        else {
          lVar29 = *(longlong *)(lVar24 + 0x18);
LAB_140a6a03b:
          lVar24 = 0;
          uVar19 = *(ulonglong *)(piVar9 + 0x48);
          uVar22 = *(ulonglong *)(lVar27 + 0x88);
          if (uVar22 <= uVar19) goto LAB_140a6a08e;
LAB_140a6a051:
          if (*(longlong *)(*(longlong *)(lVar27 + 0x80) + uVar19 * 8) == 0) goto LAB_140a6a08e;
          bVar23 = true;
          if ((uVar19 < uVar30) && (uVar25 = *(ulonglong *)(lVar20 + uVar19 * 8), uVar25 != 0)) {
            uVar25 = *(ulonglong *)(lVar29 + 0x10 + ~uVar25 * 0x30);
            uVar30 = *(ulonglong *)(piVar9 + 0x4a);
            if (uVar30 < uVar22) goto LAB_140a6a09c;
            goto LAB_140a6a0c1;
          }
          uVar25 = 0;
          uVar30 = *(ulonglong *)(piVar9 + 0x4a);
          if (uVar22 <= uVar30) goto LAB_140a6a0c1;
LAB_140a6a09c:
          bVar31 = *(longlong *)(*(longlong *)(lVar27 + 0x80) + uVar30 * 8) != 0;
        }
        lVar20 = ~*(ulonglong *)(lVar20 + *(longlong *)(piVar9 + 0x46) * 8) * 0x30;
        local_68 = *(longlong *)(lVar29 + 0x10 + lVar20);
        local_70 = *(longlong *)(lVar29 + 0x28 + lVar20);
        lVar29 = *(longlong *)(lVar27 + 0x50);
        uVar30 = 0;
      }
      uVar19 = (ulonglong)*(uint *)(lVar29 + 8 + uVar30 * 0x10);
    }
    else {
      if (uVar30 == uVar21) {
        do {
          if (puVar26 == puVar2) goto LAB_140a6a2ee;
          uVar17 = *puVar26;
          puVar26 = puVar26 + 1;
          uVar21 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar17 * 0x48);
        } while (uVar21 == 0);
        lVar24 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar17 * 0x48;
        lVar20 = *(longlong *)(lVar24 + 0x38);
        uVar30 = *(ulonglong *)(lVar24 + 0x40);
        if (*(ulonglong *)(piVar9 + 0x44) < uVar30) {
          uVar19 = *(ulonglong *)(lVar20 + *(ulonglong *)(piVar9 + 0x44) * 8);
          lVar27 = *(longlong *)(lVar24 + 0x18);
          if (uVar19 == 0) {
            uVar19 = *(ulonglong *)(piVar9 + 0x48);
            goto joined_r0x000140a6a02c;
          }
          lVar24 = *(longlong *)(lVar27 + 0x10 + ~uVar19 * 0x30);
          uVar19 = *(ulonglong *)(piVar9 + 0x48);
          if (uVar19 < uVar30) goto LAB_140a6a11e;
LAB_140a6a14f:
          bVar23 = false;
          uVar19 = *(ulonglong *)(piVar9 + 0x4a);
          if (uVar19 < uVar30) goto LAB_140a6a15e;
LAB_140a6a146:
          bVar31 = false;
        }
        else {
          lVar27 = *(longlong *)(lVar24 + 0x18);
          uVar19 = *(ulonglong *)(piVar9 + 0x48);
joined_r0x000140a6a02c:
          lVar24 = 0;
          if (uVar30 <= uVar19) goto LAB_140a6a14f;
LAB_140a6a11e:
          uVar19 = *(ulonglong *)(lVar20 + uVar19 * 8);
          if (uVar19 == 0) goto LAB_140a6a14f;
          uVar25 = *(ulonglong *)(lVar27 + 0x10 + ~uVar19 * 0x30);
          bVar23 = true;
          uVar19 = *(ulonglong *)(piVar9 + 0x4a);
          if (uVar30 <= uVar19) goto LAB_140a6a146;
LAB_140a6a15e:
          bVar31 = *(longlong *)(lVar20 + uVar19 * 8) != 0;
        }
        lVar20 = ~*(ulonglong *)(lVar20 + *(longlong *)(piVar9 + 0x46) * 8) * 0x30;
        local_68 = *(longlong *)(lVar27 + 0x10 + lVar20);
        local_70 = *(longlong *)(lVar27 + 0x28 + lVar20);
        uVar30 = 0;
      }
      uVar19 = uVar30 & 0xffffffff;
    }
    lVar27 = uVar19 * 0x1c + lVar24;
    uVar22 = uVar19 * 0x30 + local_68;
    puVar3 = (undefined4 *)(local_70 + uVar19 * 4);
    lVar20 = uVar19 * 0x10 + uVar25;
    if (!bVar23) {
      lVar20 = 0;
    }
    if (*(char *)(lVar27 + 8) == '\0') {
      in_XMM1_Da = *(float *)(lVar27 + 0xc);
      in_XMM1_Db = 0;
      fVar32 = *(float *)(lVar27 + 0x18) * in_XMM1_Da;
      fVar33 = fVar32;
      if (*(char *)(lVar27 + 9) == '\0') {
        fVar33 = in_XMM1_Da;
      }
      *puVar3 = uVar7;
      *(float *)(uVar22 + 0x1c) = fVar33;
      *(float *)(uVar22 + 0x20) = fVar32;
      *(float *)(uVar22 + 0x24) = fVar33;
    }
    uVar30 = uVar30 + 1;
  } while ((((lVar20 == 0) || (!bVar31)) || (*(char *)(lVar11 + 0x18) == '\0')) ||
          (*(longlong *)(lVar11 + 0x38) != *(longlong *)(lVar20 + 8)));
  lVar20 = *(longlong *)(lVar11 + 0x20);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(byte *)(lVar20 + 0x98)) {
  case 0:
    lVar27 = 8;
    break;
  case 1:
    if (*(ulonglong *)(lVar20 + 0xb0) <= *(ulonglong *)(lVar11 + 0x40)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    puVar28 = (undefined8 *)(*(ulonglong *)(lVar11 + 0x40) * 0xc + *(longlong *)(lVar20 + 0xa8));
    goto code_r0x000140a6a2c3;
  default:
    goto LAB_140a69eec;
  case 6:
    lVar27 = 4;
    break;
  case 8:
    goto code_r0x000140a6acfb;
  case 9:
    uVar22 = (ulonglong)*(byte *)(piVar9 + 6);
    uVar25 = (ulonglong)*(byte *)((longlong)piVar9 + 1);
    goto code_r0x000140a6ad51;
  case 10:
    pcVar13 = (code *)swi(3);
    (*pcVar13)();
    return;
  case 0xb:
code_r0x000140a6ad51:
    **(undefined4 **)
      (&UNK_142ad55bc + *(int *)(&DAT_142ad55ac + (ulonglong)*(byte *)(lVar20 + 0x98) * 4)) =
         *(undefined4 *)
          (&UNK_142ad55c8 + *(int *)(&DAT_142ad55ac + (ulonglong)*(byte *)(lVar20 + 0x98) * 4));
    *(char *)(piVar9 + 8) = (char)uVar22;
    *(char *)((longlong)piVar9 + 0x21) = (char)uVar25;
    *(ulonglong *)(piVar9 + 9) = CONCAT44(in_XMM1_Db,in_XMM1_Da);
    piVar9[0xb] = (int)fVar32;
    *(longlong *)(piVar9 + 0xc) = lVar10;
    *(char *)(piVar9 + 0xe) = (char)puVar26;
    return;
  case 0xc:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0xd:
  case 0x10:
    pcVar13 = (code *)swi(1);
    (*pcVar13)();
    return;
  case 0xe:
  case 0xf:
    *piVar9 = *piVar9 + (int)piVar9;
    *(char *)piVar9 = (char)*piVar9 + (char)piVar9;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  puVar28 = (undefined8 *)(lVar20 + 0x98 + lVar27);
code_r0x000140a6a2c3:
  uVar8 = *(undefined4 *)(puVar28 + 1);
  uVar12 = *puVar28;
  *puVar3 = uVar7;
  *(undefined4 *)(uVar22 + 0x18) = uVar8;
  *(undefined8 *)(uVar22 + 0x10) = uVar12;
  goto LAB_140a69eec;
code_r0x000140a6acfb:
  pcVar13 = (code *)swi(3);
  (*pcVar13)();
  return;
LAB_140a6a2ee:
  if (bVar14) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar18 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_140a6a371;
    if (*plVar18 != 0) goto LAB_140a6a37d;
    *plVar18 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar18 = *plVar18 + 1;
  }
  return;
}


