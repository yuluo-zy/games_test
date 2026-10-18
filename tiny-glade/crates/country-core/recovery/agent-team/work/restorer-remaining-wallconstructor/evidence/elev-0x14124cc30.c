
void __cdecl
system_wall_constructor::construct_elevation_supports::circle_turret::construct_turret(void)

{
  byte *pbVar1;
  ulonglong *puVar2;
  undefined8 uVar3;
  byte bVar4;
  byte bVar5;
  byte bVar6;
  longlong lVar7;
  undefined8 uVar8;
  longlong lVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  uint uVar14;
  uint extraout_EAX;
  int iVar15;
  uint extraout_EAX_00;
  uint extraout_EAX_01;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong lVar16;
  ulonglong uVar17;
  float *extraout_RAX_04;
  float *pfVar18;
  longlong *plVar19;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  undefined8 in_RCX;
  ulonglong uVar20;
  float *pfVar21;
  float *pfVar22;
  longlong *plVar23;
  undefined8 *in_RDX;
  undefined8 *puVar24;
  ulonglong uVar25;
  longlong in_R8;
  undefined *puVar26;
  float **ppfVar27;
  longlong *plVar28;
  float *pfVar29;
  longlong *plVar30;
  float *pfVar31;
  bool bVar32;
  float fVar33;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  longlong in_stack_00000030;
  longlong *in_stack_00000040;
  longlong local_370;
  longlong lStack_368;
  ulonglong local_360;
  ulonglong uStack_358;
  undefined8 local_350;
  undefined8 uStack_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined4 local_328;
  float *local_298;
  code *pcStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined4 uStack_260;
  undefined4 uStack_25c;
  undefined4 local_258;
  undefined4 uStack_254;
  float local_248;
  undefined8 local_238;
  undefined8 uStack_230;
  longlong local_228;
  ulonglong local_218;
  longlong *local_210;
  float *local_208;
  longlong local_200;
  float local_1f8;
  float local_1f4;
  float local_1f0;
  float local_1ec;
  float local_1e8;
  float fStack_1e4;
  float fStack_1e0;
  undefined4 uStack_1dc;
  float **local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined4 uStack_1a8;
  undefined8 uStack_1a4;
  undefined4 local_19c;
  undefined8 local_194;
  float local_180;
  float local_17c;
  float local_178;
  float local_174;
  undefined4 local_170;
  float local_16c;
  undefined8 local_168;
  float *local_160;
  undefined8 local_158;
  longlong local_150;
  longlong *local_148;
  longlong local_140;
  longlong *local_138;
  longlong local_130;
  longlong *local_128;
  longlong local_120;
  float local_118;
  float local_114;
  float local_110;
  float local_10c;
  longlong local_108;
  longlong *local_100;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar24 = in_RDX;
  local_158 = in_RCX;
  local_150 = in_R8;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_228 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar24 != (undefined8 *)0x0) {
        if (puVar24 < (undefined8 *)0x56) {
          if (*(char *)(puVar24 + 0x285840d7) < -0x40) {
LAB_14124ccfd:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (puVar24 != (undefined8 *)0x56) goto LAB_14124ccfd;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar24 != (undefined8 *)0xfffffffffffffffe)) {
        if ((longlong)puVar24 + 2U < 0x56) {
          if (*(char *)((longlong)puVar24 + 0x142c206ba) < -0x40) {
LAB_14124cd51:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((longlong)puVar24 + 2U != 0x56) goto LAB_14124cd51;
      }
    }
    puVar26 = &DAT_142c20831;
    do {
      bVar4 = puVar26[-1];
      uVar14 = (uint)(char)bVar4;
      if ((char)bVar4 < '\0') {
        bVar5 = puVar26[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar26[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar26 + -4;
            puVar26 = puVar26 + -4;
            uVar14 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar26 = puVar26 + -3;
            uVar14 = bVar6 & 0xf;
          }
          uVar14 = bVar5 & 0x3f | uVar14 << 6;
        }
        else {
          puVar26 = puVar26 + -2;
          uVar14 = bVar5 & 0x1f;
        }
        uVar14 = (uint)(bVar4 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto LAB_14124ce14;
LAB_14124ce26:
        if (puVar26 == &UNK_142c207df) break;
        if (puVar26 + -0x142c207df < (undefined *)0x51) {
          if (-0x41 < (char)puVar26[1]) break;
        }
        else if (puVar26 + -0x142c207df == (undefined *)0x51) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar26 = puVar26 + -1;
      if (uVar14 == 0x5c) goto LAB_14124ce26;
LAB_14124ce14:
      if (uVar14 == 0x2f) goto LAB_14124ce26;
    } while (puVar26 != &DAT_142c207e0);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar23 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_14124e083;
    if (*plVar23 != 0) goto LAB_14124e08f;
    *plVar23 = -1;
    local_100 = plVar23;
    puffin::ThreadProfiler::begin_scope();
    *local_100 = *local_100 + 1;
    local_228 = 1;
  }
  local_130 = 0;
  local_128 = (longlong *)&DAT_00000010;
  local_120 = 0;
  local_148 = (longlong *)0x0;
  local_140 = 4;
  local_138 = (longlong *)0x0;
  local_114 = *(float *)(local_150 + 0x280) + -0.27600002;
  local_110 = *(float *)(in_RDX + 1) + 0.056;
  local_17c = local_110 * 0.4;
  if (local_17c <= 0.5) {
    local_17c = 0.5;
  }
  pfVar22 = *(float **)(in_stack_00000030 + 8);
  pfVar21 = pfVar22 + *(longlong *)(in_stack_00000030 + 0x10) * 0x14;
  local_238 = *in_RDX;
  uStack_230 = 0;
  fVar35 = *(float *)(in_RDX + 1) - local_17c;
  bVar32 = true;
  fVar37 = 1000.0;
  local_200 = local_228;
  while (pfVar22 != pfVar21) {
    while( true ) {
      local_f1 = 1;
      local_f2 = 1;
      country_core::resources::walls::public_walls::PublicWalls::get();
      if (extraout_RAX_02 == 0) {
        local_f1 = 1;
        local_f2 = 1;
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      local_f1 = 1;
      local_f2 = 1;
      country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
      fVar33 = (float)local_238;
      fVar34 = local_238._4_4_;
      if (*(char *)(pfVar22 + 0x12) == '\0') {
        local_f1 = 1;
        local_f2 = 1;
        country_core::resources::glade_border::GladeBorder::is_pos_inside();
        fVar33 = (float)local_238;
        fVar34 = local_238._4_4_;
        if (extraout_AL_00 == '\0') {
          local_f1 = 1;
          local_f2 = 1;
          ::utils::curve::Curve::
          get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                    ();
          uVar25 = (ulonglong)extraout_EAX;
          if (*(ulonglong *)(pfVar22 + 4) <= uVar25) {
            local_f1 = 1;
            local_f2 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          if (*(ulonglong *)(pfVar22 + 4) <= uVar25 + 1) {
            local_f1 = 1;
            local_f2 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          uVar8 = *(undefined8 *)(*(longlong *)(pfVar22 + 2) + uVar25 * 8);
          uVar3 = *(undefined8 *)(*(longlong *)(pfVar22 + 2) + 8 + uVar25 * 8);
          fVar40 = extraout_XMM0_Da * (float)uVar3 + (1.0 - extraout_XMM0_Da) * (float)uVar8;
          fVar38 = extraout_XMM0_Da * (float)((ulonglong)uVar3 >> 0x20) +
                   (1.0 - extraout_XMM0_Da) * (float)((ulonglong)uVar8 >> 0x20);
          fVar33 = (float)local_238 - fVar40;
          fVar34 = local_238._4_4_ - fVar38;
          fVar34 = (fVar35 + -0.46) / SQRT(fVar34 * fVar34 + fVar33 * fVar33);
          fVar33 = 0.0;
          if (0.0 <= fVar34) {
            fVar33 = fVar34;
          }
          fVar34 = 1.0;
          if (fVar33 <= 1.0) {
            fVar34 = fVar33;
          }
          fVar33 = fVar34 * fVar40 + (1.0 - fVar34) * (float)local_238;
          fVar34 = fVar34 * fVar38 + (1.0 - fVar34) * local_238._4_4_;
        }
      }
      pfVar22 = pfVar22 + 0x14;
      fVar34 = SQRT((fVar34 - local_238._4_4_) * (fVar34 - local_238._4_4_) +
                    (fVar33 - (float)local_238) * (fVar33 - (float)local_238));
      if (fVar34 < fVar37) break;
      if (pfVar22 == pfVar21) goto LAB_14124d16b;
    }
    bVar32 = false;
    fVar37 = fVar34;
    local_178 = fVar33;
  }
LAB_14124d16b:
  if (!bVar32) {
    local_f1 = 1;
    local_f2 = 1;
    minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
    local_108 = local_150 + 0x299;
    local_174 = local_110 + -0.1;
    local_170 = *(undefined4 *)((longlong)in_RDX + 0xc);
    local_16c = local_114 + -1.3800001;
    uVar25 = 0;
    while (uVar25 < 5) {
      fVar37 = (float)(longlong)uVar25 * 0.25;
      local_f1 = 1;
      local_f2 = 1;
      country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
      powf(fVar37,1.5);
      local_f1 = 1;
      local_f2 = 1;
      ::utils::geometry::circle::Circle2d::new_rotated();
      local_f1 = 1;
      local_f2 = 1;
      country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
      uVar25 = uVar25 + 1;
      if (extraout_AL_01 == '\0') {
        local_f1 = 1;
        local_f2 = 1;
        local_118 = fVar37 * local_16c + (1.0 - fVar37) * local_114;
        ::utils::geometry::circle::Circle2d::as_curve2();
        ::utils::geometry::circle::Circle2d::perimeter();
        local_248 = extraout_XMM0_Da_00;
        fVar37 = ceilf(extraout_XMM0_Da_00 / 0.46);
        uVar17 = (longlong)(fVar37 - 9.223372e+18) & (longlong)fVar37 >> 0x3f | (longlong)fVar37;
        if (fVar37 < 0.0) {
          uVar17 = 0;
        }
        if (1.8446743e+19 < fVar37) {
          uVar17 = 0xffffffffffffffff;
        }
        uVar20 = uVar17 + 1;
        if ((uVar20 >> 0x3e != 0) ||
           (pfVar22 = (float *)(uVar17 * 4 + 4), (float *)0x7ffffffffffffffc < pfVar22)) {
LAB_14124e1cd:
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        local_218 = uVar25;
        if (pfVar22 == (float *)0x0) {
          pfVar18 = (float *)&DAT_00000004;
          uVar20 = 0;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          pfVar18 = extraout_RAX_04;
          local_208 = pfVar22;
          if (extraout_RAX_04 == (float *)0x0) goto LAB_14124e1cd;
        }
        local_208 = pfVar18;
        ::utils::random_splits_into();
        fVar37 = local_248;
        local_180 = local_118 + 0.41400003;
        pfVar31 = (float *)0x0;
        plVar23 = (longlong *)0x0;
        pfVar22 = pfVar18;
        while (pfVar29 = pfVar22, plVar30 = local_210, pfVar29 != pfVar18) {
          pfVar22 = pfVar29 + 1;
          plVar30 = (longlong *)((longlong)plVar23 + 1);
          bVar32 = pfVar31 == (float *)0x0;
          plVar19 = plVar30;
          plVar28 = plVar23;
          pfVar31 = pfVar21;
          if (bVar32) {
            if (pfVar22 == pfVar18) {
              ppfVar27 = (float **)&local_1e8;
              pfVar21 = (float *)0x0;
              pfVar22 = pfVar18;
            }
            else {
              local_1e8 = SUB84(pfVar29,0);
              fStack_1e4 = (float)((ulonglong)pfVar29 >> 0x20);
              ppfVar27 = &local_298;
              pfVar21 = pfVar22;
              plVar19 = (longlong *)((longlong)plVar23 + 2);
              pfVar22 = pfVar29 + 2;
            }
            *ppfVar27 = pfVar21;
            plVar28 = plVar30;
            local_210 = plVar23;
            pfVar29 = local_298;
            pfVar31 = (float *)CONCAT44(fStack_1e4,local_1e8);
            pfVar21 = local_298;
            if ((float *)CONCAT44(fStack_1e4,local_1e8) == (float *)0x0) break;
          }
          pfVar21 = pfVar29;
          local_160 = pfVar31;
          local_100 = local_210;
          if (pfVar31 == (float *)0x0) goto LAB_14124dbb0;
          fVar35 = *pfVar31;
          fVar33 = *pfVar29;
          local_1f4 = local_118;
          local_1ec = local_180;
          local_210 = plVar28;
          local_1f8 = fVar35;
          local_1f0 = fVar33;
          ::utils::geometry::aabb::Aabb2::width();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::center();
          local_10c = fVar35;
          if ((fVar35 < 0.0) || (1.0 < fVar35)) {
            local_298 = &local_10c;
            pcStack_290 = core::fmt::float::impl$5::fmt;
            local_1e8 = 96.82391;
            fStack_1e4 = 1.4013e-45;
            fStack_1e0 = 1.4013e-45;
            uStack_1dc = 0;
            local_1c8 = 0;
            local_1d8 = &local_298;
            uStack_1d0 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          if (NAN(fVar35)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          ::utils::curve::Curve::
          get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                    ();
          uVar25 = (ulonglong)extraout_EAX_00;
          if (local_360 <= uVar25) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          if (local_360 <= uVar25 + 1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          uVar8 = *(undefined8 *)(lStack_368 + uVar25 * 8);
          uVar3 = *(undefined8 *)(lStack_368 + 8 + uVar25 * 8);
          local_10c = fVar33;
          if ((fVar33 < 0.0) || (1.0 < fVar33)) {
            local_298 = &local_10c;
            pcStack_290 = core::fmt::float::impl$5::fmt;
            local_1e8 = 96.82391;
            fStack_1e4 = 1.4013e-45;
            fStack_1e0 = 1.4013e-45;
            uStack_1dc = 0;
            local_1c8 = 0;
            local_1d8 = &local_298;
            uStack_1d0 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          if (NAN(fVar33)) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          ::utils::curve::Curve::
          get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                    ();
          uVar25 = (ulonglong)extraout_EAX_01;
          if (local_360 <= uVar25) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          if (local_360 <= uVar25 + 1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          fVar40 = extraout_XMM0_Da_03 * (float)uVar3 + (1.0 - extraout_XMM0_Da_03) * (float)uVar8;
          fVar41 = extraout_XMM0_Da_03 * (float)((ulonglong)uVar3 >> 0x20) +
                   (1.0 - extraout_XMM0_Da_03) * (float)((ulonglong)uVar8 >> 0x20);
          uVar8 = *(undefined8 *)(lStack_368 + uVar25 * 8);
          uVar3 = *(undefined8 *)(lStack_368 + 8 + uVar25 * 8);
          fVar38 = extraout_XMM0_Da_04 * (float)uVar3 + (1.0 - extraout_XMM0_Da_04) * (float)uVar8;
          fVar39 = extraout_XMM0_Da_04 * (float)((ulonglong)uVar3 >> 0x20) +
                   (1.0 - extraout_XMM0_Da_04) * (float)((ulonglong)uVar8 >> 0x20);
          fVar36 = extraout_XMM0_Da_01 * local_248 + 0.1;
          local_298 = (float *)CONCAT44((fVar41 + fVar39) * 0.5,(fVar40 + fVar38) * 0.5);
          ::utils::swizzlers::impl$1::xvy();
          fVar11 = fStack_1e0;
          fVar10 = fStack_1e4;
          fVar34 = local_1e8;
          fVar38 = fVar38 - fVar40;
          fVar39 = fVar39 - fVar41;
          fVar40 = 1.0 / SQRT(fVar39 * fVar39 + fVar38 * fVar38);
          local_168 = CONCAT44(fVar40 * fVar39,fVar40 * fVar38);
          ::utils::swizzlers::impl$0::x0y();
          fVar12 = fStack_1e0;
          fVar39 = fStack_1e4;
          fVar40 = local_1e8;
          local_298 = (float *)(CONCAT44((undefined4)local_168,local_168._4_4_) ^ 0x80000000);
          ::utils::swizzlers::impl$0::x0y();
          fVar13 = fStack_1e0;
          fVar41 = fStack_1e4;
          fVar38 = local_1e8;
          country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
          lVar16 = local_120;
          if (extraout_AL_02 != '\0') {
            fVar36 = fVar36 + 0.2;
          }
          if (local_120 == local_130) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<glam::f32::affine3a::Affine3A,u32,array_<f32,2>_>,alloc::alloc::Global>
                      ();
          }
          pfVar29 = (float *)(local_128 + lVar16 * 10);
          *pfVar29 = fVar36 * fVar40;
          pfVar29[1] = fVar36 * fVar39;
          pfVar29[2] = fVar36 * fVar12;
          pfVar29[3] = fVar36 * fVar12;
          pfVar29 = (float *)(local_128 + lVar16 * 10 + 2);
          *pfVar29 = extraout_XMM0_Da_02 * 0.0;
          pfVar29[1] = extraout_XMM0_Da_02 * 1.0;
          pfVar29[2] = extraout_XMM0_Da_02 * 0.0;
          pfVar29[3] = extraout_XMM0_Da_02 * 0.0;
          pfVar29 = (float *)(local_128 + lVar16 * 10 + 4);
          *pfVar29 = fVar38 * 0.56;
          pfVar29[1] = fVar41 * 0.56;
          pfVar29[2] = fVar13 * 0.56;
          pfVar29[3] = fVar13 * 0.56;
          pfVar29 = (float *)(local_128 + lVar16 * 10 + 6);
          *pfVar29 = fVar34;
          pfVar29[1] = fVar10;
          pfVar29[2] = fVar11;
          pfVar29[3] = fVar11;
          *(int *)(local_128 + lVar16 * 10 + 8) = (int)local_218 * ((int)local_100 + 1) + 1000;
          *(ulonglong *)((longlong)local_128 + lVar16 * 0x50 + 0x44) =
               CONCAT44(fVar33 * fVar37,fVar35 * fVar37);
          local_120 = lVar16 + 1;
          plVar23 = plVar19;
        }
        local_210 = plVar30;
        local_160 = (float *)0x0;
        plVar28 = local_210;
LAB_14124dbb0:
        local_210 = plVar28;
        if (uVar20 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        uVar25 = local_218;
        if (local_370 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (uStack_358 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      else {
        local_f1 = 1;
        local_f2 = 1;
        ::utils::swizzlers::impl$1::xvy();
        local_f1 = 1;
        local_f2 = 1;
        country_core::render::nani::instance_data::CurvedHalfTimberInstanceData::new_curved_xy();
        plVar23 = local_138;
        if (local_138 == local_148) {
          local_f1 = 1;
          local_f2 = 1;
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::nani::instance_data::CurvedHalfTimberInstanceData,alloc::alloc::Global>
                    ();
        }
        lVar16 = (longlong)plVar23 * 0x40;
        puVar24 = (undefined8 *)(local_140 + 0x30 + lVar16);
        *puVar24 = local_340;
        puVar24[1] = uStack_338;
        puVar24 = (undefined8 *)(local_140 + 0x20 + lVar16);
        *puVar24 = local_350;
        puVar24[1] = uStack_348;
        puVar2 = (ulonglong *)(local_140 + 0x10 + lVar16);
        *puVar2 = local_360;
        puVar2[1] = uStack_358;
        *(longlong *)(local_140 + lVar16) = local_370;
        ((longlong *)(local_140 + lVar16))[1] = lStack_368;
        local_138 = (longlong *)((longlong)plVar23 + 1);
      }
    }
  }
  plVar23 = local_138;
  local_100 = local_148;
  local_108 = local_140;
  *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  lVar16 = *in_stack_00000040;
  lVar7 = *(longlong *)(lVar16 + 0x30);
  uVar8 = *(undefined8 *)(lVar16 + 0x40);
  if (lVar7 == *(longlong *)(lVar16 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
              ();
  }
  lVar9 = *(longlong *)(lVar16 + 0x28);
  *(longlong **)(lVar9 + lVar7 * 0x28) = local_100;
  *(longlong *)(lVar9 + 8 + lVar7 * 0x28) = local_108;
  *(longlong **)(lVar9 + 0x10 + lVar7 * 0x28) = plVar23;
  *(undefined8 *)(lVar9 + 0x18 + lVar7 * 0x28) = local_158;
  *(undefined8 *)(lVar9 + 0x20 + lVar7 * 0x28) = uVar8;
  *(longlong *)(lVar16 + 0x30) = lVar7 + 1;
  *(longlong *)(lVar16 + 0x40) = *(longlong *)(lVar16 + 0x40) + 1;
  if (local_120 == 0) {
    lVar16 = local_200;
    if (local_130 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      lVar16 = local_200;
    }
  }
  else {
    local_f1 = 1;
    local_f2 = 0;
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_03 == (undefined8 *)0x0) {
LAB_14124e1e5:
      local_f1 = 1;
      local_f2 = 0;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    local_1e8 = 96.83134;
    fStack_1e4 = 1.4013e-45;
    fStack_1e0 = 8.12753e-44;
    uStack_1dc = 0;
    if ((extraout_RAX_03[3] != 0x3a) ||
       (iVar15 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                        (void *)extraout_RAX_03[2],0x3a), iVar15 != 0)) {
      local_f1 = 1;
      local_f2 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_f1 = 1;
    local_f2 = 0;
    lVar16 = (**(code **)(extraout_RAX_03[1] + 0x18))(*extraout_RAX_03);
    if (lVar16 == 0) goto LAB_14124e1e5;
    lVar7 = *(longlong *)(local_150 + 0x150);
    if (lVar7 == 0) {
      local_f1 = 1;
      local_f2 = 0;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    LOCK();
    bVar32 = *(char *)(lVar7 + 0x18) == '\0';
    if (bVar32) {
      *(char *)(lVar7 + 0x18) = '\x01';
    }
    UNLOCK();
    if (!bVar32) {
      local_f1 = 1;
      local_f2 = 0;
      parking_lot::raw_mutex::RawMutex::lock_slow();
    }
    local_f1 = 1;
    local_f2 = 0;
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    if ((*(longlong *)(lVar7 + 0x10) * 4 + *(longlong *)(lVar7 + 8) == 4 ||
         *(longlong *)(lVar7 + 0x10) == 0) && (*(longlong *)(lVar16 + 0x28) == 0)) {
      local_f1 = 1;
      local_f2 = 0;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    local_108 = local_130;
    local_100 = local_128;
    if (local_120 != 0) {
      plVar30 = local_128 + local_120 * 10;
      plVar23 = local_128;
      do {
        lVar16 = plVar23[8];
        uVar8 = *(undefined8 *)((longlong)plVar23 + 0x44);
        country_core::resources::render::impl$1::from();
        country_core::geometry::instanced_wall::impl$1::from();
        country_core::geometry::instanced_wall::impl$3::default();
        local_298 = (float *)CONCAT44(fStack_1e4,local_1e8);
        pcStack_290 = (code *)CONCAT44(uStack_1dc,fStack_1e0);
        local_278 = local_1c8;
        uStack_270 = uStack_1c0;
        local_288 = local_1d8;
        uStack_280 = uStack_1d0;
        uStack_260 = (undefined4)uStack_338;
        uStack_25c = (undefined4)((ulonglong)uStack_338 >> 0x20);
        local_258 = (undefined4)local_330;
        uStack_254 = (undefined4)((ulonglong)local_330 >> 0x20);
        uStack_1a4 = CONCAT44(local_328,uStack_254);
        uStack_1ac = uStack_25c;
        uStack_1a8 = local_258;
        uStack_1b0 = uStack_260;
        local_19c = (int)lVar16;
        local_194 = uVar8;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
        plVar23 = plVar23 + 10;
      } while (plVar23 != plVar30);
    }
    if (local_108 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_f1 = 0;
    local_f2 = 0;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    bVar32 = *(char *)(lVar7 + 0x18) == '\x01';
    if (bVar32) {
      *(char *)(lVar7 + 0x18) = '\0';
    }
    UNLOCK();
    lVar16 = local_228;
    if (!bVar32) {
      local_f1 = 0;
      local_f2 = 0;
      parking_lot::raw_mutex::RawMutex::unlock_slow();
      lVar16 = local_228;
    }
  }
  if (lVar16 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_05 == 1) {
      plVar23 = extraout_RAX_05 + 1;
    }
    else if ((*extraout_RAX_05 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar23 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0)) {
LAB_14124e083:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar23 != 0) {
LAB_14124e08f:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar23 = -1;
    local_100 = plVar23;
    puffin::ThreadProfiler::end_scope();
    *local_100 = *local_100 + 1;
  }
  return;
}

