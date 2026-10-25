
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_clutter::autoclutter::add_autoclutter_around_gates::add_autoclutter_around_gates(void)

{
  byte *pbVar1;
  longlong lVar2;
  undefined8 uVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  undefined1 uVar7;
  undefined8 *puVar8;
  undefined *puVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  char extraout_AL;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  undefined8 *in_RCX;
  longlong lVar17;
  longlong *in_RDX;
  ulonglong uVar18;
  uint *puVar19;
  ulonglong uVar20;
  longlong *plVar21;
  undefined *puVar22;
  longlong lVar23;
  uint *puVar24;
  char cVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  undefined *local_228 [2];
  float local_218;
  undefined8 local_214;
  float local_20c;
  longlong local_208;
  char *local_200;
  undefined8 local_1f8;
  undefined4 local_1f0;
  undefined8 local_1ec;
  float local_1e4;
  longlong local_1e0;
  longlong local_1d8;
  undefined8 local_1d0;
  undefined8 local_1b8;
  float local_1b0;
  undefined8 local_1a8;
  undefined4 *local_1a0;
  longlong local_198;
  ulonglong local_190;
  ulonglong local_188;
  longlong local_180;
  longlong local_178;
  uint *local_170;
  undefined8 local_168;
  undefined *local_160;
  undefined4 local_158;
  longlong local_150;
  undefined4 uStack_148;
  undefined4 uStack_144;
  undefined8 local_140;
  float local_138;
  undefined4 local_134;
  undefined1 local_130;
  undefined1 local_12f;
  undefined2 local_12e;
  undefined1 local_12c;
  float local_128;
  float local_124;
  undefined8 local_120;
  longlong local_118;
  undefined4 local_104;
  longlong *local_100;
  char local_f2;
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  plVar21 = in_RDX;
  puffin::are_scopes_on();
  local_f1 = extraout_AL;
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (plVar21 != (longlong *)0x0) {
        if (plVar21 < (longlong *)0x57) {
          if ((char)plVar21[0x2860841b] < -0x40) {
LAB_1422cc455:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (plVar21 != (longlong *)0x57) goto LAB_1422cc455;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (plVar21 != (longlong *)0xfffffffffffffffe)) {
        if ((longlong)plVar21 + 2U < 0x57) {
          if (*(char *)((longlong)plVar21 + 0x1430420da) < -0x40) {
LAB_1422cc4a9:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if ((longlong)plVar21 + 2U != 0x57) goto LAB_1422cc4a9;
      }
    }
    puVar22 = &DAT_143042246;
    do {
      bVar4 = puVar22[-1];
      uVar13 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar22[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar22[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar22 + -4;
            puVar22 = puVar22 + -4;
            uVar13 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar22 = puVar22 + -3;
            uVar13 = bVar6 & 0xf;
          }
          uVar13 = bVar5 & 0x3f | uVar13 << 6;
        }
        else {
          puVar22 = puVar22 + -2;
          uVar13 = bVar5 & 0x1f;
        }
        uVar13 = (uint)(bVar4 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_1422cc564;
LAB_1422cc575:
        if (puVar22 == &UNK_1430421ff) break;
        if (puVar22 + -0x1430421ff < (undefined *)0x46) {
          if (-0x41 < (char)puVar22[1]) break;
        }
        else if (puVar22 + -0x1430421ff == (undefined *)0x46) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar22 = puVar22 + -1;
      if (uVar13 == 0x5c) goto LAB_1422cc575;
LAB_1422cc564:
      if (uVar13 == 0x2f) goto LAB_1422cc575;
    } while (puVar22 != &DAT_143042200);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar21 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_03, extraout_RAX_03 == (longlong *)0x0))
    goto LAB_1422ccc30;
    if (*plVar21 != 0) goto LAB_1422ccc3c;
    *plVar21 = -1;
    local_100 = plVar21;
    puffin::ThreadProfiler::begin_scope();
    *local_100 = *local_100 + 1;
  }
  fVar12 = __xmm_00000000000000003f0000003f000000._4_4_;
  fVar11 = (float)__xmm_00000000000000003f0000003f000000;
  fVar10 = __real_3f000000;
  local_118 = *in_RDX;
  local_178 = in_RDX[1];
  puVar19 = *(uint **)(local_118 + 0xf8);
  local_170 = puVar19 + *(longlong *)(local_118 + 0x100);
  local_f2 = *(char *)(local_118 + 0x108);
  local_104 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_1a8 = *in_RCX;
  local_1a0 = (undefined4 *)in_RCX[2];
  lVar17 = 0;
  uVar18 = 0;
  uVar20 = 0;
  lVar14 = 8;
  lVar23 = local_178;
  puVar24 = local_170;
  cVar25 = local_f2;
  do {
    if (cVar25 == '\0') {
      if (uVar18 == uVar20) {
        do {
          if (puVar19 == puVar24) goto LAB_1422ccb3b;
          uVar13 = *puVar19;
          puVar19 = puVar19 + 1;
          uVar20 = *(ulonglong *)(*(longlong *)(lVar23 + 0x100) + 0x58 + (ulonglong)uVar13 * 0xa0);
        } while (uVar20 == 0);
        lVar14 = *(longlong *)(lVar23 + 0x100) + (ulonglong)uVar13 * 0xa0;
        uVar18 = (ulonglong)*(uint *)(lVar14 + 0x94);
        if (*(ulonglong *)(local_118 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar23 + 0x1a8) + 0x40 + uVar18 * 0x48)) {
          lVar17 = *(longlong *)(lVar23 + 0x1a8) + uVar18 * 0x48;
          uVar18 = *(ulonglong *)
                    (*(longlong *)(lVar17 + 0x38) + *(ulonglong *)(local_118 + 0x110) * 8);
          if (uVar18 == 0) goto LAB_1422cc7d0;
          lVar17 = *(longlong *)(*(longlong *)(lVar17 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
LAB_1422cc7d0:
          lVar17 = 0;
        }
        lVar14 = *(longlong *)(lVar14 + 0x50);
        uVar18 = 0;
      }
      uVar15 = (ulonglong)*(uint *)(lVar14 + 8 + uVar18 * 0x10);
    }
    else {
      if (uVar18 == uVar20) {
        do {
          if (puVar19 == puVar24) goto LAB_1422ccb3b;
          uVar13 = *puVar19;
          puVar19 = puVar19 + 1;
          uVar20 = *(ulonglong *)(*(longlong *)(lVar23 + 0x1a8) + 0x10 + (ulonglong)uVar13 * 0x48);
        } while (uVar20 == 0);
        lVar17 = *(longlong *)(lVar23 + 0x1a8) + (ulonglong)uVar13 * 0x48;
        if ((*(ulonglong *)(local_118 + 0x110) < *(ulonglong *)(lVar17 + 0x40)) &&
           (uVar18 = *(ulonglong *)
                      (*(longlong *)(lVar17 + 0x38) + *(ulonglong *)(local_118 + 0x110) * 8),
           uVar18 != 0)) {
          lVar17 = *(longlong *)(*(longlong *)(lVar17 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
          lVar17 = 0;
        }
        uVar18 = 0;
      }
      uVar15 = uVar18 & 0xffffffff;
    }
    plVar21 = (longlong *)(uVar15 * 0x38 + lVar17);
    uVar18 = uVar18 + 1;
    if (2 < (ulonglong)plVar21[2]) {
      puVar8 = (undefined8 *)plVar21[1];
      lVar23 = (longlong)puVar8 + plVar21[2] * 0xc;
      local_198 = lVar17;
      local_190 = uVar18;
      if (lVar23 == 0xc) {
                    /* WARNING: Subroutine does not return */
        core::option::unwrap_failed();
      }
      local_228[0] = &DAT_143042260;
      local_228[1] = (undefined *)0xa;
      local_218 = -1.0;
      fVar30 = (float)((ulonglong)__xmm_00000000000000004000000000000000 >> 0x20);
      local_214 = CONCAT44((float)((ulonglong)*puVar8 >> 0x20) + fVar30,
                           (float)*puVar8 + (float)__xmm_00000000000000004000000000000000);
      local_20c = *(float *)(puVar8 + 1) + 0.0;
      local_208 = plVar21[4];
      local_200 = "gate_left";
      local_1f8 = 9;
      local_1f0 = 0x3f800000;
      local_1ec = CONCAT44((float)((ulonglong)*(undefined8 *)(lVar23 + -0xc) >> 0x20) + fVar30,
                           (float)*(undefined8 *)(lVar23 + -0xc) +
                           (float)__xmm_00000000000000004000000000000000);
      local_1e4 = *(float *)(lVar23 + -4) + 0.0;
      local_1e0 = plVar21[5];
      local_1d0 = 2;
      lVar16 = 0;
      local_188 = uVar20;
      local_180 = lVar14;
      local_100 = plVar21;
      do {
        lVar2 = lVar16 + 1;
        puVar22 = local_228[lVar16 * 5];
        lVar17 = local_198;
        uVar18 = local_190;
        uVar20 = local_188;
        lVar14 = local_180;
        lVar23 = local_178;
        puVar24 = local_170;
        cVar25 = local_f2;
        local_1d8 = lVar2;
        if (puVar22 == (undefined *)0x0) break;
        puVar9 = local_228[lVar16 * 5 + 1];
        fVar30 = (&local_218)[lVar16 * 10];
        uVar3 = (&local_214)[lVar16 * 5];
        fVar27 = (&local_20c)[lVar16 * 10];
        local_120 = (&local_208)[lVar16 * 5];
        fVar28 = (float)((ulonglong)local_120 >> 0x20);
        fVar26 = (float)local_120;
        fVar29 = __real_3f800000 / SQRT(fVar26 * fVar26 + fVar28 * fVar28);
        local_168 = (undefined *)
                    CONCAT44(fVar29 * fVar26,
                             (float)((uint)fVar28 ^ ___xmm_80000000800000008000000080000000) *
                             fVar29);
        ::utils::swizzlers::impl$0::x0y();
        local_158 = (undefined4)local_100[6];
        local_150 = local_100[3];
        local_168 = puVar22;
        local_160 = puVar9;
        ::utils::calculate_hash<tuple_<ref_<str_>,u32,country_core::resources::walls::WallId>_>();
        ::utils::swizzlers::impl$0::x0y();
        lVar23 = local_120;
        fVar26 = local_1b0;
        fVar28 = local_160._0_4_;
        fVar32 = SUB84(local_168,0);
        fVar33 = (float)((ulonglong)local_168 >> 0x20);
        fVar29 = (float)local_1b8;
        fVar31 = (float)((ulonglong)local_1b8 >> 0x20);
        uVar7 = *(undefined1 *)((longlong)local_100 + 0x34);
        lVar17 = local_100[3];
        lVar14 = local_100[6];
        local_128 = ((float)local_120 + (float)local_120) - (local_120._4_4_ + local_120._4_4_);
        local_124 = local_120._4_4_ + local_120._4_4_ + (float)local_120 + (float)local_120;
        ::utils::swizzlers::impl$0::x0y();
        local_138 = ((fVar26 + fVar26) * fVar10 + fVar27 + (fVar28 + fVar28) * fVar10 * fVar30) -
                    local_160._0_4_ * fVar10;
        fVar27 = SUB84(local_168,0);
        uVar20 = (ulonglong)local_168 >> 0x20;
        *local_1a0 = local_104;
        local_168 = (undefined *)0x3;
        uStack_148 = (undefined4)__xmm_00000000000000004000000040000000;
        uStack_144 = (undefined4)((ulonglong)__xmm_00000000000000004000000040000000 >> 0x20);
        local_150 = lVar23;
        local_140 = CONCAT44(((fVar31 + fVar31) * fVar12 +
                             (float)((ulonglong)uVar3 >> 0x20) + fVar30 * (fVar33 + fVar33) * fVar12
                             ) - (float)uVar20 * fVar12,
                             ((fVar29 + fVar29) * fVar11 +
                             (float)uVar3 + fVar30 * (fVar32 + fVar32) * fVar11) - fVar27 * fVar11);
        local_134 = 0;
        local_130 = 0;
        local_12e = 0xffff;
        local_12c = 0;
        local_160 = (undefined *)lVar17;
        local_158 = (int)lVar14;
        local_12f = uVar7;
        AutoClutterState::add_region();
        lVar16 = lVar2;
        lVar17 = local_198;
        uVar18 = local_190;
        uVar20 = local_188;
        lVar14 = local_180;
        lVar23 = local_178;
        puVar24 = local_170;
        cVar25 = local_f2;
      } while (lVar2 != 2);
    }
  } while( true );
LAB_1422ccb3b:
  if (local_f1 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar21 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_1422ccc30:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar21 != 0) {
LAB_1422ccc3c:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    local_100 = plVar21;
    puffin::ThreadProfiler::end_scope();
    *local_100 = *local_100 + 1;
  }
  return;
}

