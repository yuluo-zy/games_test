
void __cdecl country_core::systems::wall::wall_state::spatial_hash_update::spatial_hash_update(void)

{
  byte *pbVar1;
  char *pcVar2;
  float *pfVar3;
  undefined8 uVar4;
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  ulonglong *puVar8;
  undefined4 *puVar9;
  uint uVar10;
  uint uVar11;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar12;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  uint uVar13;
  undefined8 *in_RCX;
  longlong *plVar14;
  undefined8 *in_RDX;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  ulonglong uVar20;
  longlong lVar21;
  longlong *in_R8;
  longlong lVar22;
  ulonglong uVar23;
  longlong in_R9;
  ushort uVar24;
  longlong lVar25;
  longlong lVar26;
  longlong lVar27;
  byte bVar28;
  char cVar29;
  char cVar35;
  float fVar30;
  float fVar31;
  char cVar33;
  char cVar34;
  undefined1 auVar32 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  ulonglong uStack_200;
  int local_130;
  float fStack_12c;
  float fStack_128;
  float fStack_124;
  float fStack_120;
  longlong local_118;
  longlong local_110;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar15 = 0x51;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar15 != 0) {
        if (uVar15 < 0x51) {
          if ((char)(&DAT_142ad8c31)[uVar15] < -0x40) {
LAB_140a8fe63:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 != 0x51) goto LAB_140a8fe63;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar15 != 0xfffffffffffffffe)) {
        if (uVar15 + 2 < 0x51) {
          if ((char)(&UNK_142ad8c33)[uVar15] < -0x40) {
LAB_140a8fec1:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar15 + 2 != 0x51) goto LAB_140a8fec1;
      }
    }
    puVar12 = &DAT_142ad93ae;
    do {
      bVar28 = puVar12[-1];
      uVar13 = (uint)(char)bVar28;
      if ((char)bVar28 < '\0') {
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
        uVar13 = (uint)(bVar28 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_140a8ff89;
LAB_140a8ff9f:
        if (puVar12 == &UNK_142ad9367) break;
        if (puVar12 + -0x142ad9367 < (undefined *)0x46) {
          if (-0x41 < (char)puVar12[1]) break;
        }
        else if (puVar12 + -0x142ad9367 == (undefined *)0x46) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar12 = puVar12 + -1;
      if (uVar13 == 0x5c) goto LAB_140a8ff9f;
LAB_140a8ff89:
      if (uVar13 == 0x2f) goto LAB_140a8ff9f;
    } while (puVar12 != &DAT_142ad9368);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar14 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar14 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_140a90a21;
    if (*plVar14 != 0) goto LAB_140a90a4a;
    *plVar14 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar14 = *plVar14 + 1;
  }
  puVar8 = (ulonglong *)*in_RCX;
  lVar18 = in_RCX[1];
  uVar15 = *puVar8;
  uVar17 = 0;
  uVar16 = uVar15 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar15 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar16 = uVar17;
  }
  uVar23 = uVar15 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar15 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar23 = uVar17;
  }
  local_110 = uVar16 * 0x10 + *(longlong *)(lVar18 + 8);
  uVar15 = *(ulonglong *)(lVar18 + 0x10) - uVar16;
  if (*(ulonglong *)(lVar18 + 0x10) < uVar16) {
    local_110 = 8;
    uVar15 = uVar17;
  }
  local_118 = uVar23 * 0x10 + *(longlong *)(lVar18 + 0x28);
  uVar16 = *(ulonglong *)(lVar18 + 0x30) - uVar23;
  if (*(ulonglong *)(lVar18 + 0x30) < uVar23) {
    local_118 = 8;
    uVar16 = uVar17;
  }
  *puVar8 = (*(longlong *)(lVar18 + 0x40) - uVar16) - uVar15;
  lVar18 = uVar15 * 0x10 + local_110;
  lVar25 = uVar16 * 0x10 + local_118;
  plVar14 = (longlong *)*in_R8;
  uVar7 = *(undefined4 *)(in_R9 + 0x1c);
  puVar9 = *(undefined4 **)(in_R9 + 0x10);
  uVar15 = CONCAT44(fStack_12c,local_130);
LAB_140a9015e:
  do {
    if ((local_110 == 0) || (local_110 == lVar18)) {
      if ((local_118 == 0) || (local_118 == lVar25)) {
        puVar8 = (ulonglong *)*in_RDX;
        lVar18 = in_RDX[1];
        uVar15 = *puVar8;
        uVar17 = 0;
        uVar16 = uVar15 - *(ulonglong *)(lVar18 + 0x18);
        if (uVar15 < *(ulonglong *)(lVar18 + 0x18)) {
          uVar16 = uVar17;
        }
        uVar23 = uVar15 - *(ulonglong *)(lVar18 + 0x38);
        if (uVar15 < *(ulonglong *)(lVar18 + 0x38)) {
          uVar23 = uVar17;
        }
        lVar25 = uVar16 * 0x10 + *(longlong *)(lVar18 + 8);
        uVar15 = *(ulonglong *)(lVar18 + 0x10) - uVar16;
        if (*(ulonglong *)(lVar18 + 0x10) < uVar16) {
          lVar25 = 8;
          uVar15 = uVar17;
        }
        lVar27 = uVar23 * 0x10 + *(longlong *)(lVar18 + 0x28);
        uVar16 = *(ulonglong *)(lVar18 + 0x30) - uVar23;
        if (*(ulonglong *)(lVar18 + 0x30) < uVar23) {
          lVar27 = 8;
          uVar16 = uVar17;
        }
        *puVar8 = (*(longlong *)(lVar18 + 0x40) - uVar16) - uVar15;
        lVar21 = uVar15 * 0x10 + lVar25;
        lVar18 = uVar16 * 0x10 + lVar27;
        break;
      }
      local_110 = 0;
      lVar27 = local_118;
      local_118 = local_118 + 0x10;
    }
    else {
      lVar27 = local_110;
      local_110 = local_110 + 0x10;
    }
    *puVar8 = *puVar8 + 1;
    lVar21 = *(longlong *)(lVar27 + 8);
    if (plVar14[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar19 = *plVar14;
      bVar28 = (byte)(extraout_RAX_02 >> 0x39);
      auVar32 = ZEXT216(CONCAT11(bVar28,bVar28));
      auVar32 = pshuflw(auVar32,auVar32,0);
      lVar22 = 0;
      uVar16 = extraout_RAX_02;
      while( true ) {
        uVar16 = plVar14[1] & uVar16;
        pcVar2 = (char *)(lVar19 + uVar16);
        cVar29 = auVar32[0];
        auVar37[0] = -(*pcVar2 == cVar29);
        cVar33 = auVar32[1];
        auVar37[1] = -(pcVar2[1] == cVar33);
        cVar34 = auVar32[2];
        auVar37[2] = -(pcVar2[2] == cVar34);
        cVar35 = auVar32[3];
        auVar37[3] = -(pcVar2[3] == cVar35);
        auVar37[4] = -(pcVar2[4] == cVar29);
        auVar37[5] = -(pcVar2[5] == cVar33);
        auVar37[6] = -(pcVar2[6] == cVar34);
        auVar37[7] = -(pcVar2[7] == cVar35);
        auVar37[8] = -(pcVar2[8] == cVar29);
        auVar37[9] = -(pcVar2[9] == cVar33);
        auVar37[10] = -(pcVar2[10] == cVar34);
        auVar37[0xb] = -(pcVar2[0xb] == cVar35);
        auVar37[0xc] = -(pcVar2[0xc] == cVar29);
        auVar37[0xd] = -(pcVar2[0xd] == cVar33);
        auVar37[0xe] = -(pcVar2[0xe] == cVar34);
        auVar37[0xf] = -(pcVar2[0xf] == cVar35);
        uVar24 = (ushort)(SUB161(auVar37 >> 7,0) & 1) | (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe | (ushort)(auVar37[0xf] >> 7) << 0xf
        ;
        uVar13 = (uint)uVar24;
        while (uVar24 != 0) {
          uVar10 = 0;
          for (uVar11 = uVar13; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
            uVar10 = uVar10 + 1;
          }
          lVar26 = (uVar10 + uVar16 & plVar14[1]) * -0x2a8;
          if (*(longlong *)(lVar19 + -0x2a8 + lVar26) == lVar21) {
            if ((*(ulonglong *)(lVar19 + -0x290 + lVar26) < 2) ||
               (*(float *)(lVar19 + -0x270 + lVar26) <= 0.0)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            *puVar9 = uVar7;
            wall_spatial_hash::WallSpatialHash::remove_wall_id();
            lVar19 = lVar19 + lVar26;
            lVar27 = *(longlong *)(lVar19 + -0x290);
            uVar16 = 0;
            if (lVar27 != 1) {
              do {
                if (*(ulonglong *)(lVar19 + -0x290) <= uVar16) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(lVar19 + -0x278) <= uVar16) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                uVar17 = uVar16 + 1;
                if (*(ulonglong *)(lVar19 + -0x290) <= uVar17) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(lVar19 + -0x278) <= uVar17) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                pfVar3 = (float *)(*(longlong *)(lVar19 + -0x298) + uVar16 * 8);
                fVar41 = *pfVar3;
                fVar31 = pfVar3[1];
                uVar4 = *(undefined8 *)(*(longlong *)(lVar19 + -0x298) + uVar17 * 8);
                fVar42 = (float)uVar4;
                fVar43 = (float)((ulonglong)uVar4 >> 0x20);
                uVar16 = *(ulonglong *)(lVar19 + -0x220);
                uStack_200 = 0;
                itertools::tuple_impl::impl$7::
                next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                          ();
                local_130 = (int)uVar15;
                fStack_12c = (float)(uVar15 >> 0x20);
                if (local_130 == 1) {
                  uVar23 = 0;
                  fVar40 = 3.4028235e+38;
                  do {
                    fVar38 = fStack_124 - fStack_12c;
                    fVar39 = fStack_120 - fStack_128;
                    fVar30 = fVar39 * fVar39 + fVar38 * fVar38;
                    if (fVar30 <= 1e-08) {
                      fVar30 = 1e-08;
                    }
                    fVar30 = ((fVar31 - fStack_128) * fVar39 + (fVar41 - fStack_12c) * fVar38) /
                             fVar30;
                    fVar38 = 0.0;
                    if (0.0 <= fVar30) {
                      fVar38 = fVar30;
                    }
                    fVar30 = 1.0;
                    if (fVar38 <= 1.0) {
                      fVar30 = fVar38;
                    }
                    fVar38 = fVar41 - (fVar30 * fStack_124 + (1.0 - fVar30) * fStack_12c);
                    fVar39 = fVar31 - (fVar30 * fStack_120 + (1.0 - fVar30) * fStack_128);
                    fVar38 = SQRT(fVar39 * fVar39 + fVar38 * fVar38);
                    if (fVar38 < fVar40) {
                      if ((fVar30 < 0.0) || (1.0 < fVar30)) {
                    // WARNING: Subroutine does not return
                        core::panicking::panic();
                      }
                      uVar23 = uStack_200 & 0xffffffff;
                      fVar40 = fVar38;
                    }
                    itertools::tuple_impl::impl$7::
                    next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                              ();
                    uStack_200 = uStack_200 + 1;
                  } while ((uVar15 & 1) != 0);
                }
                else {
                  uVar23 = 0;
                }
                if (uVar16 <= uVar23) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar16 <= uVar23 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                uStack_200 = 0;
                itertools::tuple_impl::impl$7::
                next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                          ();
                if (local_130 == 1) {
                  uVar20 = 0;
                  fVar41 = 3.4028235e+38;
                  do {
                    fVar40 = fStack_124 - fStack_12c;
                    fVar30 = fStack_120 - fStack_128;
                    fVar31 = fVar30 * fVar30 + fVar40 * fVar40;
                    if (fVar31 <= 1e-08) {
                      fVar31 = 1e-08;
                    }
                    fVar31 = ((fVar43 - fStack_128) * fVar30 + (fVar42 - fStack_12c) * fVar40) /
                             fVar31;
                    fVar40 = 0.0;
                    if (0.0 <= fVar31) {
                      fVar40 = fVar31;
                    }
                    fVar31 = 1.0;
                    if (fVar40 <= 1.0) {
                      fVar31 = fVar40;
                    }
                    fVar40 = fVar42 - (fVar31 * fStack_124 + (1.0 - fVar31) * fStack_12c);
                    fVar30 = fVar43 - (fVar31 * fStack_120 + (1.0 - fVar31) * fStack_128);
                    fVar40 = SQRT(fVar30 * fVar30 + fVar40 * fVar40);
                    if (fVar40 < fVar41) {
                      if ((fVar31 < 0.0) || (1.0 < fVar31)) {
                    // WARNING: Subroutine does not return
                        core::panicking::panic();
                      }
                      uVar20 = uStack_200 & 0xffffffff;
                      fVar41 = fVar40;
                    }
                    itertools::tuple_impl::impl$7::
                    next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                              ();
                    uStack_200 = uStack_200 + 1;
                  } while ((uVar15 & 1) != 0);
                }
                else {
                  uVar20 = 0;
                }
                if (uVar16 <= uVar20) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar16 <= uVar20 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                uVar16 = *(ulonglong *)(lVar19 + -600);
                if (uVar16 <= uVar23) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar16 <= uVar23 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar16 <= uVar20) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar16 <= uVar20 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                *puVar9 = uVar7;
                wall_spatial_hash::WallSpatialHash::add_segment();
                uVar16 = uVar17;
              } while (uVar17 < lVar27 - 1U);
            }
            goto LAB_140a9015e;
          }
          uVar24 = (ushort)(uVar13 - 1) & (ushort)uVar13;
          uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar24);
        }
        auVar36[0] = -(*pcVar2 == -1);
        auVar36[1] = -(pcVar2[1] == -1);
        auVar36[2] = -(pcVar2[2] == -1);
        auVar36[3] = -(pcVar2[3] == -1);
        auVar36[4] = -(pcVar2[4] == -1);
        auVar36[5] = -(pcVar2[5] == -1);
        auVar36[6] = -(pcVar2[6] == -1);
        auVar36[7] = -(pcVar2[7] == -1);
        auVar36[8] = -(pcVar2[8] == -1);
        auVar36[9] = -(pcVar2[9] == -1);
        auVar36[10] = -(pcVar2[10] == -1);
        auVar36[0xb] = -(pcVar2[0xb] == -1);
        auVar36[0xc] = -(pcVar2[0xc] == -1);
        auVar36[0xd] = -(pcVar2[0xd] == -1);
        auVar36[0xe] = -(pcVar2[0xe] == -1);
        auVar36[0xf] = -(pcVar2[0xf] == -1);
        if ((((((((((((((((SUB161(auVar36 >> 7,0) & 1) != 0 || (SUB161(auVar36 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar36 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar36 >> 0x1f,0) & 1) != 0) || (SUB161(auVar36 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar36 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar36 >> 0x37,0) & 1) != 0) || (SUB161(auVar36 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar36 >> 0x47,0) & 1) != 0) || (SUB161(auVar36 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar36 >> 0x57,0) & 1) != 0) || (SUB161(auVar36 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar36 >> 0x67,0) & 1) != 0) || (SUB161(auVar36 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar36 >> 0x77,0) & 1) != 0) || auVar36[0xf] < '\0') break;
        uVar16 = uVar16 + lVar22 + 0x10;
        lVar22 = lVar22 + 0x10;
      }
    }
    if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      fStack_128 = 5.603363;
      fStack_124 = 1.4013e-45;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
      uVar15 = lVar27 + 8;
    }
  } while( true );
LAB_140a90919:
  if ((lVar25 == 0) || (lVar25 == lVar21)) {
    if ((lVar27 == 0) || (lVar27 == lVar18)) goto LAB_140a90939;
    lVar27 = lVar27 + 0x10;
    lVar25 = 0;
  }
  else {
    lVar25 = lVar25 + 0x10;
  }
  *puVar8 = *puVar8 + 1;
  *puVar9 = uVar7;
  wall_spatial_hash::WallSpatialHash::remove_wall_id();
  goto LAB_140a90919;
LAB_140a90939:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar14 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar14 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_140a90a21:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar14 != 0) {
LAB_140a90a4a:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar14 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar14 = *plVar14 + 1;
  }
  return;
}

