
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
  float fVar12;
  float fVar13;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar14;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  uint uVar15;
  undefined8 *in_RCX;
  longlong *plVar16;
  undefined8 *in_RDX;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  longlong lVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong *in_R8;
  longlong lVar24;
  ulonglong uVar25;
  longlong in_R9;
  ushort uVar26;
  longlong lVar27;
  longlong lVar28;
  longlong lVar29;
  byte bVar30;
  char cVar31;
  char cVar37;
  float fVar32;
  float fVar33;
  char cVar35;
  char cVar36;
  undefined1 auVar34 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
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
    uVar17 = 0x51;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar17 != 0) {
        if (uVar17 < 0x51) {
          if ((char)(&DAT_142ad8c31)[uVar17] < -0x40) {
LAB_140a8fe63:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 != 0x51) goto LAB_140a8fe63;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar17 != 0xfffffffffffffffe)) {
        if (uVar17 + 2 < 0x51) {
          if ((char)(&UNK_142ad8c33)[uVar17] < -0x40) {
LAB_140a8fec1:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 + 2 != 0x51) goto LAB_140a8fec1;
      }
    }
    puVar14 = &DAT_142ad93ae;
    do {
      bVar30 = puVar14[-1];
      uVar15 = (uint)(char)bVar30;
      if ((char)bVar30 < '\0') {
        bVar5 = puVar14[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar14[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar14 + -4;
            puVar14 = puVar14 + -4;
            uVar15 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar14 = puVar14 + -3;
            uVar15 = bVar6 & 0xf;
          }
          uVar15 = bVar5 & 0x3f | uVar15 << 6;
        }
        else {
          puVar14 = puVar14 + -2;
          uVar15 = bVar5 & 0x1f;
        }
        uVar15 = (uint)(bVar30 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_140a8ff89;
LAB_140a8ff9f:
        if (puVar14 == &UNK_142ad9367) break;
        if (puVar14 + -0x142ad9367 < (undefined *)0x46) {
          if (-0x41 < (char)puVar14[1]) break;
        }
        else if (puVar14 + -0x142ad9367 == (undefined *)0x46) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar14 = puVar14 + -1;
      if (uVar15 == 0x5c) goto LAB_140a8ff9f;
LAB_140a8ff89:
      if (uVar15 == 0x2f) goto LAB_140a8ff9f;
    } while (puVar14 != &DAT_142ad9368);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar16 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_140a90a21;
    if (*plVar16 != 0) goto LAB_140a90a4a;
    *plVar16 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar16 = *plVar16 + 1;
  }
  puVar8 = (ulonglong *)*in_RCX;
  lVar20 = in_RCX[1];
  uVar17 = *puVar8;
  uVar19 = 0;
  uVar18 = uVar17 - *(ulonglong *)(lVar20 + 0x18);
  if (uVar17 < *(ulonglong *)(lVar20 + 0x18)) {
    uVar18 = uVar19;
  }
  uVar25 = uVar17 - *(ulonglong *)(lVar20 + 0x38);
  if (uVar17 < *(ulonglong *)(lVar20 + 0x38)) {
    uVar25 = uVar19;
  }
  local_110 = uVar18 * 0x10 + *(longlong *)(lVar20 + 8);
  uVar17 = *(ulonglong *)(lVar20 + 0x10) - uVar18;
  if (*(ulonglong *)(lVar20 + 0x10) < uVar18) {
    local_110 = 8;
    uVar17 = uVar19;
  }
  local_118 = uVar25 * 0x10 + *(longlong *)(lVar20 + 0x28);
  uVar18 = *(ulonglong *)(lVar20 + 0x30) - uVar25;
  if (*(ulonglong *)(lVar20 + 0x30) < uVar25) {
    local_118 = 8;
    uVar18 = uVar19;
  }
  *puVar8 = (*(longlong *)(lVar20 + 0x40) - uVar18) - uVar17;
  fVar13 = __real_322bcc77;
  fVar12 = __real_3f800000;
  lVar20 = uVar17 * 0x10 + local_110;
  lVar27 = uVar18 * 0x10 + local_118;
  plVar16 = (longlong *)*in_R8;
  uVar7 = *(undefined4 *)(in_R9 + 0x1c);
  puVar9 = *(undefined4 **)(in_R9 + 0x10);
  uVar17 = CONCAT44(fStack_12c,local_130);
LAB_140a9015e:
  do {
    if ((local_110 == 0) || (local_110 == lVar20)) {
      if ((local_118 == 0) || (local_118 == lVar27)) {
        puVar8 = (ulonglong *)*in_RDX;
        lVar20 = in_RDX[1];
        uVar17 = *puVar8;
        uVar19 = 0;
        uVar18 = uVar17 - *(ulonglong *)(lVar20 + 0x18);
        if (uVar17 < *(ulonglong *)(lVar20 + 0x18)) {
          uVar18 = uVar19;
        }
        uVar25 = uVar17 - *(ulonglong *)(lVar20 + 0x38);
        if (uVar17 < *(ulonglong *)(lVar20 + 0x38)) {
          uVar25 = uVar19;
        }
        lVar27 = uVar18 * 0x10 + *(longlong *)(lVar20 + 8);
        uVar17 = *(ulonglong *)(lVar20 + 0x10) - uVar18;
        if (*(ulonglong *)(lVar20 + 0x10) < uVar18) {
          lVar27 = 8;
          uVar17 = uVar19;
        }
        lVar29 = uVar25 * 0x10 + *(longlong *)(lVar20 + 0x28);
        uVar18 = *(ulonglong *)(lVar20 + 0x30) - uVar25;
        if (*(ulonglong *)(lVar20 + 0x30) < uVar25) {
          lVar29 = 8;
          uVar18 = uVar19;
        }
        *puVar8 = (*(longlong *)(lVar20 + 0x40) - uVar18) - uVar17;
        lVar23 = uVar17 * 0x10 + lVar27;
        lVar20 = uVar18 * 0x10 + lVar29;
        break;
      }
      local_110 = 0;
      lVar29 = local_118;
      local_118 = local_118 + 0x10;
    }
    else {
      lVar29 = local_110;
      local_110 = local_110 + 0x10;
    }
    *puVar8 = *puVar8 + 1;
    lVar23 = *(longlong *)(lVar29 + 8);
    if (plVar16[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar21 = *plVar16;
      bVar30 = (byte)(extraout_RAX_02 >> 0x39);
      auVar34 = ZEXT216(CONCAT11(bVar30,bVar30));
      auVar34 = pshuflw(auVar34,auVar34,0);
      lVar24 = 0;
      uVar18 = extraout_RAX_02;
      while( true ) {
        uVar18 = plVar16[1] & uVar18;
        pcVar2 = (char *)(lVar21 + uVar18);
        cVar31 = auVar34[0];
        auVar39[0] = -(*pcVar2 == cVar31);
        cVar35 = auVar34[1];
        auVar39[1] = -(pcVar2[1] == cVar35);
        cVar36 = auVar34[2];
        auVar39[2] = -(pcVar2[2] == cVar36);
        cVar37 = auVar34[3];
        auVar39[3] = -(pcVar2[3] == cVar37);
        auVar39[4] = -(pcVar2[4] == cVar31);
        auVar39[5] = -(pcVar2[5] == cVar35);
        auVar39[6] = -(pcVar2[6] == cVar36);
        auVar39[7] = -(pcVar2[7] == cVar37);
        auVar39[8] = -(pcVar2[8] == cVar31);
        auVar39[9] = -(pcVar2[9] == cVar35);
        auVar39[10] = -(pcVar2[10] == cVar36);
        auVar39[0xb] = -(pcVar2[0xb] == cVar37);
        auVar39[0xc] = -(pcVar2[0xc] == cVar31);
        auVar39[0xd] = -(pcVar2[0xd] == cVar35);
        auVar39[0xe] = -(pcVar2[0xe] == cVar36);
        auVar39[0xf] = -(pcVar2[0xf] == cVar37);
        uVar26 = (ushort)(SUB161(auVar39 >> 7,0) & 1) | (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe | (ushort)(auVar39[0xf] >> 7) << 0xf
        ;
        uVar15 = (uint)uVar26;
        while (uVar26 != 0) {
          uVar10 = 0;
          for (uVar11 = uVar15; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
            uVar10 = uVar10 + 1;
          }
          lVar28 = (uVar10 + uVar18 & plVar16[1]) * -0x2a8;
          if (*(longlong *)(lVar21 + -0x2a8 + lVar28) == lVar23) {
            if ((*(ulonglong *)(lVar21 + -0x290 + lVar28) < 2) ||
               (*(float *)(lVar21 + -0x270 + lVar28) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic();
            }
            *puVar9 = uVar7;
            wall_spatial_hash::WallSpatialHash::remove_wall_id();
            lVar21 = lVar21 + lVar28;
            lVar29 = *(longlong *)(lVar21 + -0x290);
            uVar18 = 0;
            if (lVar29 != 1) {
              do {
                if (*(ulonglong *)(lVar21 + -0x290) <= uVar18) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(lVar21 + -0x278) <= uVar18) {
                    /* WARNING: Subroutine does not return */
                  core::option::unwrap_failed();
                }
                uVar19 = uVar18 + 1;
                if (*(ulonglong *)(lVar21 + -0x290) <= uVar19) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(lVar21 + -0x278) <= uVar19) {
                    /* WARNING: Subroutine does not return */
                  core::option::unwrap_failed();
                }
                pfVar3 = (float *)(*(longlong *)(lVar21 + -0x298) + uVar18 * 8);
                fVar43 = *pfVar3;
                fVar33 = pfVar3[1];
                uVar4 = *(undefined8 *)(*(longlong *)(lVar21 + -0x298) + uVar19 * 8);
                fVar44 = (float)uVar4;
                fVar45 = (float)((ulonglong)uVar4 >> 0x20);
                uVar18 = *(ulonglong *)(lVar21 + -0x220);
                uStack_200 = 0;
                itertools::tuple_impl::impl$7::
                next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                          ();
                local_130 = (int)uVar17;
                fStack_12c = (float)(uVar17 >> 0x20);
                if (local_130 == 1) {
                  uVar25 = 0;
                  fVar41 = __real_7f7fffff;
                  do {
                    fVar40 = fStack_124 - fStack_12c;
                    fVar42 = fStack_120 - fStack_128;
                    fVar32 = fVar42 * fVar42 + fVar40 * fVar40;
                    if (fVar32 <= fVar13) {
                      fVar32 = fVar13;
                    }
                    fVar32 = ((fVar33 - fStack_128) * fVar42 + (fVar43 - fStack_12c) * fVar40) /
                             fVar32;
                    fVar40 = 0.0;
                    if (0.0 <= fVar32) {
                      fVar40 = fVar32;
                    }
                    fVar32 = fVar12;
                    if (fVar40 <= fVar12) {
                      fVar32 = fVar40;
                    }
                    fVar40 = fVar43 - (fVar32 * fStack_124 + (fVar12 - fVar32) * fStack_12c);
                    fVar42 = fVar33 - (fVar32 * fStack_120 + (fVar12 - fVar32) * fStack_128);
                    fVar40 = SQRT(fVar42 * fVar42 + fVar40 * fVar40);
                    if (fVar40 < fVar41) {
                      if ((fVar32 < 0.0) || (fVar12 < fVar32)) {
                    /* WARNING: Subroutine does not return */
                        core::panicking::panic();
                      }
                      uVar25 = uStack_200 & 0xffffffff;
                      fVar41 = fVar40;
                    }
                    itertools::tuple_impl::impl$7::
                    next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                              ();
                    uStack_200 = uStack_200 + 1;
                  } while ((uVar17 & 1) != 0);
                }
                else {
                  uVar25 = 0;
                }
                if (uVar18 <= uVar25) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (uVar18 <= uVar25 + 1) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                uStack_200 = 0;
                itertools::tuple_impl::impl$7::
                next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                          ();
                if (local_130 == 1) {
                  uVar22 = 0;
                  fVar43 = __real_7f7fffff;
                  do {
                    fVar41 = fStack_124 - fStack_12c;
                    fVar32 = fStack_120 - fStack_128;
                    fVar33 = fVar32 * fVar32 + fVar41 * fVar41;
                    if (fVar33 <= fVar13) {
                      fVar33 = fVar13;
                    }
                    fVar33 = ((fVar45 - fStack_128) * fVar32 + (fVar44 - fStack_12c) * fVar41) /
                             fVar33;
                    fVar41 = 0.0;
                    if (0.0 <= fVar33) {
                      fVar41 = fVar33;
                    }
                    fVar33 = fVar12;
                    if (fVar41 <= fVar12) {
                      fVar33 = fVar41;
                    }
                    fVar41 = fVar44 - (fVar33 * fStack_124 + (fVar12 - fVar33) * fStack_12c);
                    fVar32 = fVar45 - (fVar33 * fStack_120 + (fVar12 - fVar33) * fStack_128);
                    fVar41 = SQRT(fVar32 * fVar32 + fVar41 * fVar41);
                    if (fVar41 < fVar43) {
                      if ((fVar33 < 0.0) || (fVar12 < fVar33)) {
                    /* WARNING: Subroutine does not return */
                        core::panicking::panic();
                      }
                      uVar22 = uStack_200 & 0xffffffff;
                      fVar43 = fVar41;
                    }
                    itertools::tuple_impl::impl$7::
                    next<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                              ();
                    uStack_200 = uStack_200 + 1;
                  } while ((uVar17 & 1) != 0);
                }
                else {
                  uVar22 = 0;
                }
                if (uVar18 <= uVar22) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (uVar18 <= uVar22 + 1) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                uVar18 = *(ulonglong *)(lVar21 + -600);
                if (uVar18 <= uVar25) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (uVar18 <= uVar25 + 1) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (uVar18 <= uVar22) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (uVar18 <= uVar22 + 1) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                *puVar9 = uVar7;
                wall_spatial_hash::WallSpatialHash::add_segment();
                uVar18 = uVar19;
              } while (uVar19 < lVar29 - 1U);
            }
            goto LAB_140a9015e;
          }
          uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
          uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
        }
        auVar38[0] = -(*pcVar2 == __xmm_ffffffffffffffffffffffffffffffff);
        auVar38[1] = -(pcVar2[1] == UNK_142925991);
        auVar38[2] = -(pcVar2[2] == UNK_142925992);
        auVar38[3] = -(pcVar2[3] == UNK_142925993);
        auVar38[4] = -(pcVar2[4] == UNK_142925994);
        auVar38[5] = -(pcVar2[5] == UNK_142925995);
        auVar38[6] = -(pcVar2[6] == UNK_142925996);
        auVar38[7] = -(pcVar2[7] == UNK_142925997);
        auVar38[8] = -(pcVar2[8] == UNK_142925998);
        auVar38[9] = -(pcVar2[9] == UNK_142925999);
        auVar38[10] = -(pcVar2[10] == UNK_14292599a);
        auVar38[0xb] = -(pcVar2[0xb] == UNK_14292599b);
        auVar38[0xc] = -(pcVar2[0xc] == UNK_14292599c);
        auVar38[0xd] = -(pcVar2[0xd] == UNK_14292599d);
        auVar38[0xe] = -(pcVar2[0xe] == UNK_14292599e);
        auVar38[0xf] = -(pcVar2[0xf] == UNK_14292599f);
        if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar38 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar38 >> 0x1f,0) & 1) != 0) || (SUB161(auVar38 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar38 >> 0x37,0) & 1) != 0) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') break;
        uVar18 = uVar18 + lVar24 + 0x10;
        lVar24 = lVar24 + 0x10;
      }
    }
    if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      fStack_128 = 5.603363;
      fStack_124 = 1.4013e-45;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
      uVar17 = lVar29 + 8;
    }
  } while( true );
LAB_140a90919:
  if ((lVar27 == 0) || (lVar27 == lVar23)) {
    if ((lVar29 == 0) || (lVar29 == lVar20)) goto LAB_140a90939;
    lVar29 = lVar29 + 0x10;
    lVar27 = 0;
  }
  else {
    lVar27 = lVar27 + 0x10;
  }
  *puVar8 = *puVar8 + 1;
  *puVar9 = uVar7;
  wall_spatial_hash::WallSpatialHash::remove_wall_id();
  goto LAB_140a90919;
LAB_140a90939:
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar16 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar16 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_140a90a21:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar16 != 0) {
LAB_140a90a4a:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar16 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar16 = *plVar16 + 1;
  }
  return;
}

