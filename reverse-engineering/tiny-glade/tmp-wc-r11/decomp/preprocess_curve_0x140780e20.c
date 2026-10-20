// name: preprocess_curve
// addr: 0x140780e20
// size: 5331
// signature: void __cdecl preprocess_curve(void)


// WARNING: Type propagation algorithm not settling

void __cdecl system_wall_constructor::construct_playermade_stairs::preprocess_curve(void)

{
  undefined8 *puVar1;
  uint uVar2;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined8 *extraout_RAX;
  ulonglong uVar3;
  undefined8 ****ppppuVar4;
  undefined8 *in_RCX;
  ulonglong uVar5;
  undefined8 ****ppppuVar6;
  char *in_R8;
  undefined8 ****ppppuVar7;
  uint uVar8;
  undefined8 *****pppppuVar9;
  undefined8 ****ppppuVar10;
  undefined8 ****_Src;
  bool bVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  longlong in_stack_00000028;
  char in_stack_00000030;
  ushort in_stack_00000038;
  undefined8 ****in_stack_00000040;
  undefined8 ****in_stack_00000048;
  undefined8 ****in_stack_00000050;
  undefined8 ****local_338;
  undefined8 ****ppppuStack_330;
  undefined8 ****local_328;
  longlong lStack_320;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 ****local_2f8;
  undefined8 ****ppppuStack_2f0;
  undefined8 ****local_2e8;
  undefined8 ****local_2e0;
  undefined8 ****ppppuStack_2d8;
  float *local_2d0;
  undefined8 ****ppppuStack_2c8;
  float **local_2c0;
  ulonglong uStack_2b8;
  undefined8 ***local_2b0;
  undefined8 ****local_2a8;
  undefined8 ****ppppuStack_2a0;
  undefined8 ****local_298;
  float *pfStack_290;
  undefined8 ****local_288;
  float **ppfStack_280;
  ulonglong local_278;
  undefined8 ****local_268;
  undefined8 ****ppppuStack_260;
  undefined8 *****local_258;
  longlong lStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 ****local_228;
  undefined8 ****ppppuStack_220;
  undefined8 ****ppppuStack_218;
  float *pfStack_210;
  undefined8 ****ppppuStack_208;
  float **local_200;
  ulonglong uStack_1f8;
  undefined8 local_1f0;
  undefined8 ****local_1e0;
  float local_1d4;
  undefined8 *local_1d0;
  float ****local_1c8;
  undefined8 ****ppppuStack_1c0;
  undefined8 ****local_1b8;
  longlong lStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  float local_18c;
  undefined8 ****local_188;
  undefined8 ****ppppuStack_180;
  undefined8 ****local_178;
  undefined8 ****local_168;
  undefined8 ****ppppuStack_160;
  undefined8 ****local_158;
  float *pfStack_150;
  undefined8 ****local_148;
  float **ppfStack_140;
  ulonglong local_138;
  float local_12c;
  undefined8 ****local_128;
  undefined8 ****ppppuStack_120;
  undefined8 ****local_118;
  float *local_110;
  undefined8 ****local_108;
  undefined8 ****local_100;
  undefined8 ****local_f8;
  undefined8 *puStack_f0;
  undefined8 ****local_e8;
  undefined8 ****local_e0;
  undefined1 local_d6;
  undefined1 local_d5;
  undefined1 local_d4;
  undefined1 local_d3;
  undefined1 local_d2;
  undefined1 local_d1;
  undefined8 local_d0;
  
  local_d0 = 0xfffffffffffffffe;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>_>
            ();
  ::utils::curve::Curve::
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  local_2a8 = ppppuStack_220;
  ppppuStack_2a0 = ppppuStack_218;
  if (SBORROW8(0,(longlong)local_228)) {
    in_RCX[1] = ppppuStack_220;
    in_RCX[2] = ppppuStack_218;
    *in_RCX = 0x8000000000000000;
    return;
  }
  local_2c0 = local_200;
  uStack_2b8 = uStack_1f8;
  uVar3 = uStack_2b8;
  local_2d0 = pfStack_210;
  ppppuStack_2c8 = ppppuStack_208;
  local_2e0 = ppppuStack_220;
  ppppuStack_2d8 = ppppuStack_218;
  local_2e8 = local_228;
  uStack_2b8._0_4_ = (float)uStack_1f8;
  fVar12 = (float)uStack_2b8;
  fVar14 = 0.368 / (float)uStack_2b8;
  if (0.2 <= fVar14) {
    fVar14 = 0.2;
  }
  local_18c = fVar14;
  if (*in_R8 == '\0') {
    local_18c = 0.0;
  }
  local_12c = 1.0;
  if (in_R8[1] != '\0') {
    local_12c = 1.0 - fVar14;
  }
  local_128 = (undefined8 ****)0x0;
  ppppuStack_120 = (undefined8 ****)&DAT_00000004;
  local_118 = (undefined8 ****)0x0;
  uStack_2b8 = uVar3;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
    local_d2 = 1;
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX = 0;
  *(undefined1 *)(extraout_RAX + 1) = 0;
  local_f8 = (undefined8 ****)0x1;
  local_e8 = (undefined8 ****)0x1;
  uVar8 = (in_stack_00000038 & 0x100) >> 8;
  puStack_f0 = extraout_RAX;
  if (in_stack_00000030 == '\0') {
    local_1c8 = (float ****)in_stack_00000040;
    ppppuStack_1c0 = in_stack_00000048;
    local_1b8 = in_stack_00000050;
    lStack_1b0 = in_stack_00000028;
    local_d1 = 1;
    preprocess_curve::closure_0();
    if (extraout_AL == '\0') {
      local_d1 = 1;
      preprocess_curve::closure_0();
      if (extraout_AL_01 == '\0') {
        uVar2 = uVar8;
        if ((in_stack_00000038 & 1) != 0) goto LAB_1407810b2;
        if ((short)(in_stack_00000038 & 0x100) != 0) goto LAB_140781dce;
        local_e0 = (undefined8 ****)&DAT_00000004;
        fVar14 = 0.0;
        in_stack_00000038 = 0;
        ppppuVar10 = (undefined8 ****)0x0;
      }
      else {
        uVar8 = 1;
        if ((in_stack_00000038 & 1) != 0) goto LAB_1407817b0;
LAB_140781dce:
        fVar14 = 0.9 - fVar14;
        local_e0 = (undefined8 ****)&DAT_00000004;
        in_stack_00000038 = 0;
        ppppuVar10 = (undefined8 ****)0x0;
      }
LAB_140782060:
      if ((char)uVar8 != '\0') {
        fVar13 = -2.0 / (float)uStack_2b8 + 1.0;
        fVar14 = 1.0 - fVar14;
        fVar12 = fVar14;
        if (fVar14 <= fVar13) {
          fVar12 = fVar13;
        }
        local_12c = (float)(~-(uint)NAN(fVar13) & (uint)fVar12 | -(uint)NAN(fVar13) & (uint)fVar14);
      }
      goto LAB_140780ffd;
    }
    local_d1 = 1;
    preprocess_curve::closure_0();
    uVar2 = 1;
    if (extraout_AL_00 == '\0') {
      uVar2 = uVar8;
    }
LAB_1407810b2:
    uVar8 = uVar2;
    if ((char)uVar8 == '\0') {
      fVar14 = 0.9 - fVar14;
    }
    else {
LAB_1407817b0:
      fVar14 = 0.45;
    }
    fVar12 = 2.0 / fVar12;
    fVar13 = fVar14;
    if (fVar12 <= fVar14) {
      fVar13 = fVar12;
    }
    fVar12 = (float)(-(uint)NAN(fVar12) & (uint)fVar14 | ~-(uint)NAN(fVar12) & (uint)fVar13);
    local_1d4 = fVar12;
    if ((fVar12 < 0.0) || (1.0 < fVar12)) {
      local_188 = (undefined8 ****)&local_1d4;
      ppppuStack_180 = (undefined8 ****)core::fmt::float::impl$5::fmt;
      local_268 = (undefined8 ****)
                  &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
      ppppuStack_260 = (undefined8 ****)0x1;
      local_248 = 0;
      local_258 = &local_188;
      lStack_250 = 1;
      local_d1 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    local_188 = (undefined8 ****)0x0;
    ppppuStack_180 = (undefined8 ****)&DAT_00000004;
    local_178 = (undefined8 ****)0x0;
    local_d3 = 1;
    ::utils::curve::Curve::
    slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
              ();
    local_258 = (undefined8 *****)local_178;
    local_268 = local_188;
    ppppuStack_260 = ppppuStack_180;
    local_d3 = 0;
    ::utils::curve::Curve::
    try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    ppppuVar10 = local_228;
    local_2f8 = ppppuStack_220;
    ppppuStack_2f0 = ppppuStack_218;
    if (SBORROW8(0,(longlong)local_228)) {
      in_RCX[1] = ppppuStack_220;
      in_RCX[2] = ppppuStack_218;
      *in_RCX = 0x8000000000000000;
    }
    else {
      ppfStack_140 = local_200;
      local_138 = uStack_1f8;
      uVar3 = local_138;
      pfStack_150 = pfStack_210;
      local_148 = ppppuStack_208;
      ppppuStack_160 = ppppuStack_220;
      local_158 = ppppuStack_218;
      local_168 = local_228;
      if (ppppuStack_218 < (undefined8 ****)0x2) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      local_138._0_4_ = (float)uStack_1f8;
      local_138 = uVar3;
      if (((float)local_138 == 0.0) && (!NAN((float)local_138))) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if ((float)local_138 <= 1e-08) {
        local_138._0_4_ = 1e-08;
      }
      fVar13 = 0.5 / (float)local_138;
      if (1.0 <= fVar13) {
        fVar13 = 1.0;
      }
      roundf(1.0 / fVar13);
      ::utils::curve::Curve::
      try_resample_approx_pt_count_remap<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(glam::f32::vec2::Vec2)>
                ();
      ppppuVar7 = local_298;
      ppppuVar6 = ppppuStack_2a0;
      ppppuVar4 = local_2a8;
      if (!SBORROW8(0,(longlong)local_2a8)) {
        local_110 = pfStack_290;
        local_108 = local_288;
        if (ppppuVar10 != (undefined8 ****)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (pfStack_150 != (float *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_e0 = ppppuVar6;
        if (ppppuVar7 == (undefined8 ****)0x0) {
          ppppuVar6 = (undefined8 ****)&DAT_00000004;
          ppppuVar10 = (undefined8 ****)0x0;
        }
        else {
          local_100 = ppppuVar4;
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          ppppuVar4 = local_100;
          ppppuVar6 = ppppuStack_120;
          ppppuVar10 = local_118;
        }
        memcpy(ppppuVar6 + (longlong)ppppuVar10,local_e0,(longlong)ppppuVar7 * 8);
        ppppuVar10 = (undefined8 ****)((longlong)ppppuVar10 + (longlong)ppppuVar7);
        local_118 = ppppuVar10;
        if (ppppuVar4 != (undefined8 ****)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        ppppuVar4 = local_e8;
        if (ppppuVar10 == (undefined8 ****)0x0) {
          ppppuVar10 = (undefined8 ****)0x0;
        }
        else {
          ppppuVar10 = (undefined8 ****)((longlong)ppppuVar10 + -1);
          local_118 = ppppuVar10;
        }
        if (local_e8 == (undefined8 ****)0x0) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        *(undefined1 *)(puStack_f0 + 1) = 2;
        local_e0 = ppppuVar6;
        if (local_e8 == local_f8) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        puStack_f0[(longlong)ppppuVar4 * 2] = ppppuVar10;
        *(undefined1 *)(puStack_f0 + (longlong)ppppuVar4 * 2 + 1) = 2;
        local_e8 = (undefined8 ****)((longlong)ppppuVar4 + 1);
        in_stack_00000038 = 1;
        local_18c = fVar12;
        if (local_110 != (float *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        goto LAB_140782060;
      }
      in_RCX[1] = ppppuStack_2a0;
      in_RCX[2] = local_298;
      *in_RCX = 0x8000000000000000;
      if (ppppuVar10 != (undefined8 ****)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (pfStack_150 != (float *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
LAB_140781ade:
    ppppuVar10 = local_128;
    if (local_f8 != (undefined8 ****)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      ppppuVar10 = local_128;
    }
  }
  else {
    local_e0 = (undefined8 ****)&DAT_00000004;
    ppppuVar10 = (undefined8 ****)0x0;
LAB_140780ffd:
    local_110 = (float *)CONCAT44(local_110._4_4_,uVar8);
    if ((char)uVar8 == '\0') {
      fVar12 = 0.0;
      if ((in_stack_00000038 & 1) != 0) goto LAB_1407810d6;
    }
    else if ((in_stack_00000038 & 1) == 0) {
LAB_1407810d6:
      fVar12 = 1.6;
    }
    else {
      fVar12 = 3.2;
    }
    local_1d0 = in_RCX;
    fVar12 = (float)roundf(((float)uStack_2b8 - fVar12) * 0.25);
    uVar3 = 0;
    if (0.0 <= fVar12) {
      uVar3 = (longlong)(fVar12 - 9.223372e+18) & (longlong)fVar12 >> 0x3f | (longlong)fVar12;
    }
    uVar5 = 0xffffffffffffffff;
    if (fVar12 <= 1.8446743e+19) {
      uVar5 = uVar3;
    }
    local_2b0 = (undefined8 ***)(uVar5 + (uVar5 == 0));
    local_228 = (undefined8 ****)0x0;
    ppppuStack_220 = &local_2b0;
    ppppuStack_218 = (undefined8 ****)&local_18c;
    pfStack_210 = &local_12c;
    ppppuStack_208 = (undefined8 ****)0x0;
    uStack_1f8 = uStack_1f8 & 0xffffffffffffff00;
    local_1f0 = (undefined8 ****)((ulonglong)local_1f0._4_4_ << 0x20);
    local_200 = (float **)local_2b0;
    while( true ) {
      local_d1 = 1;
      itertools::tuple_impl::impl$7::
      next<core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::construct_playermade_stairs::preprocess_curve::closure_env_1>,tuple_<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>_>
                ();
      ppppuVar7 = local_e8;
      ppppuVar6 = local_128;
      ppppuVar4 = local_228;
      if ((int)local_168 != 1) break;
      ppppuVar6 = (undefined8 ****)((longlong)local_228 + 1);
      bVar11 = local_228 != (undefined8 ****)0x0;
      local_228 = ppppuVar6;
      if (bVar11) {
        if (local_e8 == local_f8) {
          local_d1 = 1;
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        puStack_f0[(longlong)ppppuVar7 * 2] = ppppuVar10;
        *(char *)(puStack_f0 + (longlong)ppppuVar7 * 2 + 1) = in_stack_00000030;
        local_e8 = (undefined8 ****)((longlong)ppppuVar7 + 1);
      }
      local_188 = (undefined8 ****)0x0;
      ppppuStack_180 = (undefined8 ****)&DAT_00000004;
      local_178 = (undefined8 ****)0x0;
      local_d5 = 1;
      ::utils::curve::Curve::
      slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
                ();
      local_158 = local_178;
      local_168 = local_188;
      ppppuStack_160 = ppppuStack_180;
      local_d5 = 0;
      ::utils::curve::Curve::
      try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
      if (!SBORROW8(0,(longlong)local_2a8)) {
        local_158 = local_298;
        pfStack_150 = pfStack_290;
        local_138 = local_278;
        uVar3 = local_138;
        local_148 = local_288;
        ppfStack_140 = ppfStack_280;
        local_168 = local_2a8;
        ppppuStack_160 = ppppuStack_2a0;
        if (local_298 < (undefined8 ****)0x2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        local_138._0_4_ = (float)local_278;
        local_138 = uVar3;
        if (((float)local_138 == 0.0) && (!NAN((float)local_138))) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        if ((float)local_138 <= 1e-08) {
          local_138._0_4_ = 1e-08;
        }
        fVar12 = 0.5 / (float)local_138;
        if (1.0 <= 0.5 / (float)local_138) {
          fVar12 = 1.0;
        }
        roundf(1.0 / fVar12);
        ::utils::curve::Curve::
        try_resample_approx_pt_count_remap<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(glam::f32::vec2::Vec2)>
                  ();
        if (local_168 != (undefined8 ****)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (pfStack_150 != (float *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        pppppuVar9 = local_258;
        if (!SBORROW8(0,(longlong)local_268)) {
          local_198 = local_238;
          local_1a8 = local_248;
          uStack_1a0 = uStack_240;
          local_1b8 = local_258;
          lStack_1b0 = lStack_250;
          local_1c8 = (float ****)local_268;
          ppppuStack_1c0 = ppppuStack_260;
          if (local_258 < &DAT_00000005) {
            local_d6 = 1;
            ::utils::curve::Curve::
            try_resample_approx_pt_count_remap<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(glam::f32::vec2::Vec2)>
                      ();
            if (!SBORROW8(0,(longlong)local_338)) {
              if (local_1c8 != (float ****)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (lStack_1b0 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              local_1b8 = local_328;
              lStack_1b0 = lStack_320;
              local_198 = local_308;
              local_1a8 = local_318;
              uStack_1a0 = uStack_310;
              local_1c8 = (float ****)local_338;
              ppppuStack_1c0 = ppppuStack_330;
              pppppuVar9 = (undefined8 *****)local_328;
            }
          }
          local_100 = (undefined8 ****)local_1c8;
          local_108 = ppppuStack_1c0;
          if ((undefined8 ****)((longlong)local_128 - (longlong)ppppuVar10) < pppppuVar9) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
            local_e0 = ppppuStack_120;
            ppppuVar10 = local_118;
          }
          memcpy(local_e0 + (longlong)ppppuVar10,local_108,(longlong)pppppuVar9 * 8);
          ppppuVar10 = (undefined8 ****)((longlong)ppppuVar10 + (longlong)pppppuVar9);
          local_118 = ppppuVar10;
          if (local_100 != (undefined8 ****)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if ((ppppuVar4 < (undefined8 ****)((longlong)local_2b0 + -1)) &&
             (ppppuVar10 != (undefined8 ****)0x0)) {
            ppppuVar10 = (undefined8 ****)((longlong)ppppuVar10 + -1);
            local_118 = ppppuVar10;
          }
          if (lStack_1b0 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
      }
    }
    if (ppppuVar10 == (undefined8 ****)0x0) {
      *(undefined1 *)(local_1d0 + 1) = 1;
LAB_14078176b:
      *local_1d0 = 0x8000000000000000;
      goto LAB_140781ade;
    }
    ppppuVar10 = (undefined8 ****)((longlong)ppppuVar10 + -1);
    if ((char)local_110 == '\0') {
      if (local_e8 == local_f8) {
        local_d1 = 1;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      puStack_f0[(longlong)ppppuVar7 * 2] = ppppuVar10;
      *(char *)(puStack_f0 + (longlong)ppppuVar7 * 2 + 1) = in_stack_00000030;
      local_e8 = (undefined8 ****)((longlong)ppppuVar7 + 1);
    }
    else {
      local_118 = ppppuVar10;
      if (local_e8 == local_f8) {
        local_d1 = 1;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      puStack_f0[(longlong)ppppuVar7 * 2] = ppppuVar10;
      *(undefined1 *)(puStack_f0 + (longlong)ppppuVar7 * 2 + 1) = 1;
      local_e8 = (undefined8 ****)((longlong)ppppuVar7 + 1);
      local_2f8 = (undefined8 ****)CONCAT44(local_2f8._4_4_,local_12c);
      if ((local_12c < 0.0) || (1.0 < local_12c)) {
        local_1c8 = (float ****)&local_2f8;
        ppppuStack_1c0 = (undefined8 ****)core::fmt::float::impl$5::fmt;
        local_268 = (undefined8 ****)
                    &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
        ppppuStack_260 = (undefined8 ****)0x1;
        local_248 = 0;
        local_258 = (undefined8 *****)&local_1c8;
        lStack_250 = 1;
        local_d1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(local_12c)) {
        local_d1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      local_1c8 = (float ****)0x0;
      ppppuStack_1c0 = (undefined8 ****)&DAT_00000004;
      local_1b8 = (undefined8 ****)0x0;
      local_d4 = 1;
      ::utils::curve::Curve::
      slice_into<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,alloc::vec::Vec<glam::f32::vec2::Vec2,alloc::alloc::Global>_>
                ();
      puVar1 = local_1d0;
      local_258 = (undefined8 *****)local_1b8;
      local_268 = (undefined8 ****)local_1c8;
      ppppuStack_260 = ppppuStack_1c0;
      local_d4 = 0;
      ::utils::curve::Curve::
      try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
      ppppuVar4 = local_228;
      local_188 = ppppuStack_220;
      ppppuStack_180 = ppppuStack_218;
      if (SBORROW8(0,(longlong)local_228)) {
        puVar1[1] = ppppuStack_220;
        puVar1[2] = ppppuStack_218;
        local_1d0 = puVar1;
        goto LAB_14078176b;
      }
      ppfStack_140 = local_200;
      fVar12 = (float)uStack_1f8;
      local_138 = uStack_1f8;
      pfStack_150 = pfStack_210;
      local_148 = ppppuStack_208;
      ppppuStack_160 = ppppuStack_220;
      local_158 = ppppuStack_218;
      local_168 = local_228;
      if (ppppuStack_218 < (undefined8 ****)0x2) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if ((fVar12 == 0.0) && (!NAN(fVar12))) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if (fVar12 <= 1e-08) {
        fVar12 = 1e-08;
      }
      fVar14 = 0.5 / fVar12;
      if (1.0 <= 0.5 / fVar12) {
        fVar14 = 1.0;
      }
      roundf(1.0 / fVar14);
      ::utils::curve::Curve::
      try_resample_approx_pt_count_remap<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(glam::f32::vec2::Vec2)>
                ();
      _Src = ppppuStack_2a0;
      ppppuVar7 = local_2a8;
      if (SBORROW8(0,(longlong)local_2a8)) {
        puVar1[1] = ppppuStack_2a0;
        puVar1[2] = local_298;
        *puVar1 = 0x8000000000000000;
        if (ppppuVar4 != (undefined8 ****)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (pfStack_150 != (float *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        goto LAB_140781ade;
      }
      local_110 = pfStack_290;
      local_108 = local_288;
      if (ppppuVar4 != (undefined8 ****)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (pfStack_150 != (float *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      ppppuVar4 = local_e0;
      if ((undefined8 ****)((longlong)ppppuVar6 - (longlong)ppppuVar10) < local_298) {
        local_100 = _Src;
        local_e0 = ppppuVar7;
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        ppppuVar4 = ppppuStack_120;
        ppppuVar7 = local_e0;
        _Src = local_100;
        ppppuVar10 = local_118;
      }
      memcpy(ppppuVar4 + (longlong)ppppuVar10,_Src,(longlong)local_298 * 8);
      ppppuVar10 = (undefined8 ****)((longlong)ppppuVar10 + (longlong)local_298);
      local_118 = ppppuVar10;
      if (ppppuVar7 != (undefined8 ****)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      ppppuVar4 = local_e8;
      if (local_e8 == local_f8) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      puStack_f0[(longlong)ppppuVar4 * 2] = (undefined1 *)((longlong)ppppuVar10 + -1);
      *(undefined1 *)(puStack_f0 + (longlong)ppppuVar4 * 2 + 1) = 1;
      local_e8 = (undefined8 ****)((longlong)ppppuVar4 + 1);
      if (local_110 != (float *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    ppppuStack_218 = local_118;
    local_228 = local_128;
    ppppuStack_220 = ppppuStack_120;
    local_d1 = 0;
    ::utils::curve::Curve::
    try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    if (!SBORROW8(0,(longlong)local_168)) {
      local_1d0[2] = local_158;
      local_1d0[3] = pfStack_150;
      *local_1d0 = local_168;
      local_1d0[1] = ppppuStack_160;
      local_1d0[4] = local_148;
      local_1d0[5] = ppfStack_140;
      local_1d0[8] = puStack_f0;
      local_1d0[9] = local_e8;
      local_1d0[6] = local_138;
      local_1d0[7] = local_f8;
      local_228 = local_168;
      ppppuStack_220 = ppppuStack_160;
      ppppuStack_218 = local_158;
      pfStack_210 = pfStack_150;
      ppppuStack_208 = local_148;
      local_200 = ppfStack_140;
      uStack_1f8 = local_138;
      local_1f0 = local_f8;
      local_1e0 = local_e8;
      goto joined_r0x000140781b25;
    }
    local_1d0[1] = ppppuStack_160;
    local_1d0[2] = local_158;
    *local_1d0 = 0x8000000000000000;
    ppppuVar10 = local_f8;
  }
  if (ppppuVar10 != (undefined8 ****)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
joined_r0x000140781b25:
  if (local_2e8 != (undefined8 ****)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_2d0 != (float *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

