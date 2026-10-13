
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows(void)

{
  byte *pbVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined1 auVar4 [16];
  byte bVar5;
  byte bVar6;
  byte bVar7;
  undefined1 uVar8;
  undefined4 uVar9;
  ulonglong uVar10;
  ushort uVar11;
  uint uVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint uVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong lVar17;
  longlong *extraout_RAX_03;
  undefined1 (*extraout_RAX_04) [16];
  longlong *extraout_RAX_05;
  ulonglong uVar18;
  longlong *plVar19;
  ulonglong in_RDX;
  undefined *puVar20;
  longlong lVar21;
  undefined1 (*pauVar22) [16];
  undefined8 *in_R8;
  undefined **ppuVar23;
  longlong lVar24;
  uint uVar25;
  ulonglong uVar26;
  undefined1 (*pauVar27) [16];
  undefined1 (*pauVar28) [16];
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float local_248;
  float fStack_244;
  float fStack_240;
  float fStack_23c;
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  longlong local_1a8;
  undefined8 local_1a0;
  undefined4 *local_198;
  undefined8 local_190;
  longlong local_188;
  uint local_17c;
  float local_178;
  float fStack_174;
  float local_170;
  longlong local_168;
  undefined8 local_160;
  undefined1 (*local_158) [16];
  undefined4 local_14c;
  undefined *local_148;
  undefined1 (*local_140) [16];
  code *local_138;
  undefined1 local_130 [8];
  undefined8 uStack_128;
  undefined8 local_120;
  float local_118;
  undefined4 local_114;
  undefined1 local_110;
  undefined1 local_10f;
  undefined2 local_10e;
  undefined1 local_10c;
  uint local_104;
  undefined1 (*local_100) [16];
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_f1 = extraout_AL;
  if (extraout_AL != '\0') {
    pauVar27 = (undefined1 (*) [16])0x5b;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x5b) {
          if ((char)(&DAT_143042760)[in_RDX] < -0x40) {
LAB_1422cf0ac:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x5b) goto LAB_1422cf0ac;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (in_RDX != 0xfffffffffffffffe) {
          if (in_RDX + 2 < 0x5b) {
            if ("stem_clutter::autoclutter::add_autoclutter_around_windows::add_autoclutter_around_windows::f"
                [in_RDX] < -0x40) {
LAB_1422cf100:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (in_RDX + 2 != 0x5b) goto LAB_1422cf100;
        }
        pauVar27 = (undefined1 (*) [16])(0x59 - in_RDX);
      }
    }
    ppuVar23 = &PTR_DAT_1430428d0;
    do {
      bVar5 = *(byte *)((longlong)ppuVar23 + -1);
      uVar16 = (uint)(char)bVar5;
      local_100 = pauVar27;
      if ((char)bVar5 < '\0') {
        bVar6 = *(byte *)((longlong)ppuVar23 + -2);
        if ((char)bVar6 < -0x40) {
          bVar7 = *(byte *)((longlong)ppuVar23 + -3);
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar23 + -4);
            ppuVar23 = (undefined **)((longlong)ppuVar23 + -4);
            uVar16 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar23 = (undefined **)((longlong)ppuVar23 + -3);
            uVar16 = bVar7 & 0xf;
          }
          uVar16 = bVar6 & 0x3f | uVar16 << 6;
        }
        else {
          ppuVar23 = (undefined **)((longlong)ppuVar23 + -2);
          uVar16 = bVar6 & 0x1f;
        }
        uVar16 = (uint)(bVar5 & 0x3f) | uVar16 << 6;
        if (uVar16 != 0x5c) goto LAB_1422cf1c4;
LAB_1422cf1d5:
        if (ppuVar23 == (undefined **)&UNK_143042887) break;
        if ((undefined *)((longlong)ppuVar23 + -0x143042887) < (undefined *)0x48) {
          if (-0x41 < *(char *)((longlong)ppuVar23 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar23 + -0x143042887) == (undefined *)0x48) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      ppuVar23 = (undefined **)((longlong)ppuVar23 + -1);
      if (uVar16 == 0x5c) goto LAB_1422cf1d5;
LAB_1422cf1c4:
      if (uVar16 == 0x2f) goto LAB_1422cf1d5;
    } while (ppuVar23 != (undefined **)&DAT_143042888);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      pauVar27 = (undefined1 (*) [16])(extraout_RAX_01 + 1);
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), pauVar27 = extraout_RAX_04, extraout_RAX_04 == (undefined1 (*) [16])0x0))
    goto LAB_1422cfda3;
    if (*(longlong *)*pauVar27 != 0) goto LAB_1422cfdc9;
    *(undefined8 *)*pauVar27 = 0xffffffffffffffff;
    local_158 = pauVar27;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)*local_158 = *(longlong *)*local_158 + 1;
  }
  country_core::resources::walls::decorator_subtype::impl$8::from();
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  fVar15 = _UNK_142a171f4;
  fVar14 = ___xmm_0000000000000000bf000000bf000000;
  fVar13 = __real_3fc00000;
  local_14c = *(undefined4 *)((longlong)in_R8 + 0x1c);
  local_190 = *in_R8;
  local_198 = (undefined4 *)in_R8[2];
  puVar20 = local_148;
  pauVar27 = local_140;
  lVar24 = uStack_128;
  uVar16 = (uint)(ushort)local_130._0_2_;
  while (lVar24 != 0) {
    if ((short)uVar16 == 0) {
      do {
        auVar4 = *pauVar27;
        uVar11 = (ushort)(SUB161(auVar4 >> 7,0) & 1) | (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar4[0xf] >> 7) << 0xf;
        puVar20 = (undefined *)((longlong)puVar20 + -0x980);
        pauVar27 = pauVar27 + 1;
      } while (uVar11 == 0xffff);
      uVar16 = ~(uint)uVar11;
      uVar25 = uVar16 & -(uint)uVar11 - 2;
    }
    else {
      if (puVar20 == (undefined *)0x0) break;
      uVar25 = uVar16 - 1 & uVar16;
    }
    lVar24 = lVar24 + -1;
    uVar12 = 0;
    for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
      uVar12 = uVar12 + 1;
    }
    lVar21 = (ulonglong)uVar12 * -0x98;
    uVar16 = uVar25;
    if (*(longlong *)((longlong)puVar20 + -0x48 + lVar21) != 0) {
      local_158 = *(undefined1 (**) [16])((longlong)puVar20 + -0x98 + lVar21);
      local_17c = uVar25;
      country_core::resources::walls::public_walls::PublicWalls::get();
      uVar16 = local_17c;
      if ((extraout_RAX_02 != 0) && (*(uint *)(extraout_RAX_02 + 600) < 2)) {
        pauVar22 = *(undefined1 (**) [16])(lVar21 + (longlong)puVar20 + -0x60);
        auVar4 = *pauVar22;
        pauVar28 = pauVar22 + 1;
        lVar21 = *(longlong *)(lVar21 + (longlong)puVar20 + -0x48);
        uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar4 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar4[0xf] >> 7) << 0xf);
        local_188 = extraout_RAX_02;
        while (uVar16 = local_17c, lVar21 != 0) {
          if ((short)uVar25 == 0) {
            do {
              auVar4 = *pauVar28;
              uVar11 = (ushort)(SUB161(auVar4 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar4[0xf] >> 7) << 0xf;
              pauVar22 = pauVar22 + -0x50;
              pauVar28 = pauVar28 + 1;
            } while (uVar11 == 0xffff);
            uVar25 = ~(uint)uVar11;
            local_104 = uVar25 & -(uint)uVar11 - 2;
          }
          else {
            local_104 = uVar25 - 1 & uVar25;
          }
          uVar16 = 0;
          for (; (uVar25 & 1) == 0; uVar25 = uVar25 >> 1 | 0x80000000) {
            uVar16 = uVar16 + 1;
          }
          country_core::resources::walls::decorator::DecoratorType::spawns_clutter();
          lVar21 = lVar21 + -1;
          uVar25 = local_104;
          if (extraout_AL_00 != '\0') {
            local_100 = pauVar22 + (ulonglong)uVar16 * -5;
            lVar17 = (ulonglong)(byte)local_100[-3][10] - 1;
            if (2 < (byte)(local_100[-3][10] - 2)) {
              lVar17 = 0;
            }
            if (lVar17 == 0) {
              lVar17 = *(longlong *)(local_188 + 0x48);
              if (lVar17 == 1) {
                    /* WARNING: Subroutine does not return */
                core::option::unwrap_failed();
              }
              if (lVar17 == 0) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic();
              }
              local_168 = CONCAT44(local_168._4_4_,*(undefined4 *)(local_100[-5] + 0xc));
              local_148 = *(undefined **)(local_188 + 0x40);
              local_138 = core::ops::function::FnOnce::
                          call_once<utils::curve::curve_slice::impl_5::remap_2d::closure_env_0<slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>,tuple_<usize,ref_<glam::f32::vec3::Vec3>_>_>
              ;
              _local_130 = (undefined1  [16])0x0;
              local_140 = (undefined1 (*) [16])lVar17;
              ::utils::curve::curve_slice::CurveSlice::
              extend_right_until<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
                        ();
              uVar18 = (ulonglong)(uint)uStack_128;
              uVar10 = *(ulonglong *)(local_188 + 0x48);
              if (uVar10 <= uVar18) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              if (uVar10 <= uVar18 + 1) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              uVar26 = 0;
              if (uVar18 != 0) {
                uVar26 = uVar18 - 1;
              }
              if (uVar10 <= uVar26) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              fVar32 = uStack_128._4_4_;
              lVar17 = *(longlong *)(local_188 + 0x40);
              fVar29 = *(float *)(lVar17 + 8 + (uVar18 + 1) * 0xc);
              uVar3 = *(undefined8 *)(lVar17 + 8 + uVar18 * 0xc);
              fVar34 = (float)((ulonglong)uVar3 >> 0x20);
              fVar30 = *(float *)(lVar17 + uVar18 * 0xc);
              fVar31 = fVar34 - *(float *)(lVar17 + uVar26 * 0xc);
              fVar33 = fVar29 - *(float *)(lVar17 + 8 + uVar26 * 0xc);
              fVar40 = __real_3f800000 / SQRT(fVar33 * fVar33 + fVar31 * fVar31);
              *local_198 = local_14c;
              local_1b8 = (ulonglong)*(uint *)local_100[-5];
              local_1c8 = &DAT_1430428e8;
              local_1c0 = 0xf;
              local_1b0._0_4_ = *(undefined4 *)(local_100[-2] + 4);
              ::utils::calculate_hash<tuple_<ref_<str_>,usize,u32>_>();
              fVar35 = fVar32 * fVar29 + (float)uVar3 * (__real_3f800000 - fVar32);
              fVar36 = (__real_3f800000 - fVar32) * fVar30 + fVar34 * fVar32;
              uVar16 = (uint)((int)fVar40 - 1U < 0x7fffff ||
                             (int)ABS(fVar40) - 0x800000U < 0x7f000000 && -1 < (int)fVar40);
              fVar29 = (float)((int)(uVar16 << 0x1f) >> 0x1f & (uint)(fVar31 * fVar40));
              fVar30 = (float)((int)(uVar16 << 0x1f) >> 0x1f & (uint)(fVar33 * fVar40));
              fVar37 = fVar30 * ___xmm_000000000000000040970a3d40970a3d;
              fVar38 = fVar29 * _UNK_143042684;
              fVar34 = fVar29 * ___xmm_0000000000000000c000000040000000;
              fVar40 = fVar30 * _UNK_143042694;
              fVar32 = fVar29 * ___xmm_0000000000000000be8f5c293e8f5c29;
              fVar31 = fVar30 * _UNK_1430426a4;
              local_160 = CONCAT44((fVar37 + fVar34) * (float)__xmm_00000000000000003f0000003f000000
                                   + fVar35 + fVar32,
                                   (fVar38 + fVar40) * __xmm_00000000000000003f0000003f000000._4_4_
                                   + fVar36 + fVar31);
              ::utils::swizzlers::impl$1::xvy();
              uVar9 = *(undefined4 *)local_100[-5];
              uVar8 = local_100[-1][0xd];
              fVar39 = fVar29 * __real_40800000 - (fVar30 + fVar30);
              fVar33 = fVar30 * __real_40800000 + fVar34;
              local_178 = fVar39;
              fStack_174 = fVar33;
              ::utils::swizzlers::impl$0::x0y();
              local_120 = CONCAT44(fStack_244 + (float)((ulonglong)local_148 >> 0x20) * fVar15,
                                   local_248 + SUB84(local_148,0) * fVar14);
              local_118 = local_140._0_4_ * __real_bf000000 + fStack_240;
              local_148 = (undefined *)0x0;
              local_140 = local_158;
              uStack_128._4_4_ = (float)__xmm_00000000000000004000000040800000._4_4_;
              uStack_128._0_4_ = (undefined4)__xmm_00000000000000004000000040800000;
              local_130._4_4_ = fVar30;
              local_130._0_4_ = fVar29;
              local_110 = 0;
              local_10e = 0xffff;
              local_10c = 0;
              local_114 = uVar9;
              local_10f = uVar8;
              local_138._0_4_ = uVar9;
              AutoClutterState::add_region();
              *local_198 = local_14c;
              local_1b8 = (ulonglong)*(uint *)local_100[-5];
              local_1c8 = "decorator_left";
              local_1c0 = 0xe;
              local_1b0 = CONCAT44(local_1b0._4_4_,*(undefined4 *)(local_100[-2] + 4));
              ::utils::calculate_hash<tuple_<ref_<str_>,usize,u32>_>();
              local_160 = CONCAT44(fVar32 + fVar35 + (fVar34 - fVar37) *
                                                     (float)__xmm_00000000000000003f0000003f000000,
                                   fVar31 + fVar36 + (fVar40 - fVar38) *
                                                     __xmm_00000000000000003f0000003f000000._4_4_);
              ::utils::swizzlers::impl$1::xvy();
              uVar9 = *(undefined4 *)local_100[-5];
              uVar8 = local_100[-1][0xd];
              local_178 = fVar39;
              fStack_174 = fVar33;
              ::utils::swizzlers::impl$0::x0y();
              fVar32 = SUB84(local_148,0);
              uVar10 = (ulonglong)local_148 >> 0x20;
              local_118 = local_140._0_4_ * __real_bf000000 + fStack_240;
              local_148 = (undefined *)0x0;
              local_140 = local_158;
              local_138 = (code *)CONCAT44(local_138._4_4_,uVar9);
              uStack_128._0_4_ = (undefined4)__xmm_00000000000000004000000040800000;
              uStack_128._4_4_ = (float)__xmm_00000000000000004000000040800000._4_4_;
              local_120 = CONCAT44(fStack_244 + (float)uVar10 * fVar15,local_248 + fVar32 * fVar14);
              local_110 = 0;
              local_10e = 0xffff;
              local_10c = 0;
              local_114 = uVar9;
              local_10f = uVar8;
              AutoClutterState::add_region();
              uVar25 = local_104;
            }
            else {
              local_168 = lVar21;
              if (lVar17 != 1) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic();
              }
              country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
              if ((extraout_AL_01 == '\n') ||
                 (lVar21 = local_168, uVar25 = local_104, extraout_AL_01 == '\x05')) {
                country_core::resources::walls::cached_decorator_transforms::
                CachedDecoratorTransforms::transform();
                fVar32 = fStack_240 * fStack_240 + fStack_244 * fStack_244 + local_248 * local_248;
                fVar30 = fStack_240 * 0.0 + fStack_244 * 0.0 + local_248 * __real_3f800000;
                fVar30 = fVar30 + fVar30;
                fVar29 = (fStack_23c + fStack_23c) *
                         (fStack_244 * 0.0 - fStack_240 * ___xmm_000000003f8000003f80000000000000) +
                         fVar30 * local_248 + (fStack_23c * fStack_23c - fVar32) * __real_3f800000;
                fVar30 = (fStack_23c + fStack_23c) * (local_248 * 0.0 - fStack_244 * _UNK_142a82408)
                         + fVar30 * fStack_240 + (fStack_23c * fStack_23c - fVar32) * 0.0;
                local_1c8 = (char *)CONCAT44(fVar30,fVar29);
                local_1c0 = 0x3f800000;
                local_1b8 = CONCAT44(fVar30,fVar29);
                local_1b0 = 0x1bf800000;
                local_1a0 = 2;
                lVar17 = 0;
                do {
                  lVar2 = lVar17 + 1;
                  local_1a8 = lVar2;
                  fVar29 = *(float *)(&local_1c8 + lVar17 * 2);
                  fVar30 = *(float *)((longlong)&local_1c8 + lVar17 * 0x10 + 4);
                  local_138 = (code *)(ulonglong)*(uint *)local_100[-5];
                  local_148 = &DAT_143042920;
                  local_140 = (undefined1 (*) [16])0x9;
                  local_130._4_4_ = *(undefined4 *)(local_100[-2] + 4);
                  local_130._0_4_ = *(undefined4 *)((longlong)&local_1c0 + lVar17 * 0x10 + 4);
                  ::utils::calculate_hash<tuple_<ref_<str_>,i32,usize,u32>_>();
                  ::utils::swizzlers::impl$1::xvy();
                  uVar9 = *(undefined4 *)local_100[-5];
                  uVar8 = local_100[-1][0xd];
                  fVar32 = fVar29 * fVar13;
                  fVar31 = fVar30 * fVar13;
                  local_160 = CONCAT44(fVar31 + fVar32,fVar32 - fVar31);
                  ::utils::swizzlers::impl$0::x0y();
                  local_120 = CONCAT44(fStack_174 + (float)((ulonglong)local_148 >> 0x20) * fVar15,
                                       local_178 + SUB84(local_148,0) * fVar14);
                  local_118 = local_140._0_4_ * __real_bf000000 + local_170;
                  *local_198 = local_14c;
                  local_148 = (undefined *)0x0;
                  local_140 = local_158;
                  local_138 = (code *)CONCAT44(local_138._4_4_,uVar9);
                  local_130._4_4_ = fVar30;
                  local_130._0_4_ = fVar29;
                  uStack_128._0_4_ = (int)__xmm_00000000000000003fc000003fc00000;
                  uStack_128._4_4_ =
                       (float)(int)((ulonglong)__xmm_00000000000000003fc000003fc00000 >> 0x20);
                  local_110 = 1;
                  local_10e = 0xffff;
                  local_10c = 0;
                  local_114 = uVar9;
                  local_10f = uVar8;
                  AutoClutterState::add_region();
                  lVar17 = lVar2;
                  lVar21 = local_168;
                  uVar25 = local_104;
                } while (lVar2 != 2);
              }
            }
          }
        }
      }
    }
  }
  if (local_f1 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar19 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar19 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1422cfda3:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar19 != 0) {
LAB_1422cfdc9:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar19 = -1;
    local_100 = (undefined1 (*) [16])plVar19;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_100 = *(longlong *)local_100 + 1;
  }
  return;
}

