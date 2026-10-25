
void __cdecl
system_wall_constructor::segmentation::wall_path_segment::calculate_wall_path_segments_impl(void)

{
  longlong lVar1;
  float *pfVar2;
  longlong *plVar3;
  undefined8 *puVar4;
  byte bVar5;
  ulonglong uVar6;
  undefined8 ******ppppppuVar7;
  undefined8 ******ppppppuVar8;
  undefined1 auVar9 [16];
  undefined ******ppppppuVar10;
  ulonglong *puVar11;
  char extraout_AL;
  int iVar12;
  byte *pbVar13;
  ulonglong *puVar14;
  ulonglong *puVar15;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong uVar16;
  ulonglong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  undefined8 ******ppppppuVar17;
  longlong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  ulonglong uVar18;
  ulonglong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  bool bVar19;
  longlong *in_RCX;
  longlong in_RDX;
  undefined **ppuVar20;
  undefined8 ******ppppppuVar21;
  undefined **ppuVar22;
  longlong lVar23;
  int iVar24;
  undefined **in_R8;
  undefined8 in_R9;
  byte *pbVar25;
  longlong lVar26;
  byte *pbVar27;
  float fVar28;
  float fVar29;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar30;
  float fVar31;
  float fVar32;
  undefined **in_stack_00000028;
  ulonglong in_stack_00000030;
  undefined **in_stack_00000038;
  ulonglong in_stack_00000040;
  undefined **in_stack_00000048;
  ulonglong in_stack_00000050;
  char in_stack_00000058;
  undefined ******local_328;
  undefined ******local_320;
  longlong local_318;
  ulonglong *local_310;
  undefined8 local_308;
  undefined **local_300;
  undefined ******local_2f8;
  undefined8 local_2f0;
  float *pfStack_2e8;
  ulonglong local_2e0;
  undefined8 local_2d8;
  undefined8 *puStack_2d0;
  ulonglong local_2c8;
  undefined **local_2c0;
  undefined **ppuStack_2b8;
  longlong local_2b0;
  longlong local_2a4;
  undefined4 local_29c;
  ulonglong local_298;
  uint local_290;
  undefined **local_288;
  ulonglong uStack_280;
  undefined8 local_278;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  ulonglong local_268;
  ulonglong *local_260;
  ulonglong *local_258;
  undefined **local_250;
  undefined *******local_248;
  code *local_240;
  undefined8 local_238;
  undefined4 local_230;
  longlong local_228;
  longlong lStack_220;
  longlong local_218;
  longlong *local_210;
  ulonglong local_208;
  ulonglong *local_200;
  ulonglong local_1f8;
  longlong local_1f0;
  undefined ******local_1e8;
  int local_1dc;
  undefined **local_1d8;
  undefined **ppuStack_1d0;
  undefined1 local_1c8 [12];
  undefined4 uStack_1bc;
  undefined ******local_1b8;
  undefined8 uStack_1b0;
  undefined ******local_1a8;
  float *pfStack_1a0;
  ulonglong local_198;
  undefined ******ppppppuStack_190;
  longlong local_188;
  undefined **ppuStack_180;
  ulonglong uStack_178;
  undefined1 local_170 [16];
  undefined ******local_160;
  longlong local_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 ******local_130;
  undefined ******local_128;
  undefined ***local_120;
  undefined8 local_118;
  undefined8 local_110;
  uint local_fc;
  uint local_f8;
  undefined1 local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_228 = 0;
  lStack_220 = 8;
  local_218 = 0;
  uVar6 = *(ulonglong *)(in_RDX + 0x78);
  local_290 = *(uint *)(in_RDX + 0x80);
  local_238 = *(undefined8 *)(in_RDX + 0x78);
  local_230 = *(undefined4 *)(in_RDX + 0x80);
  local_278 = *(undefined8 *)(in_RDX + 0x78);
  uStack_270 = *(undefined4 *)(in_RDX + 0x80);
  local_288 = in_stack_00000038;
  uStack_280 = in_stack_00000040;
  local_208 = 0;
  local_200 = (ulonglong *)&DAT_00000008;
  local_1f8 = 0;
  pbVar25 = *(byte **)(in_RDX + 8);
  pbVar27 = pbVar25 + *(longlong *)(in_RDX + 0x10);
  local_250 = (undefined **)(in_RDX + 0x78);
  lVar26 = 0;
  local_308 = in_R9;
  local_300 = in_R8;
  local_298 = uVar6;
  local_210 = in_RCX;
  local_f8 = local_290;
  do {
    uVar18 = local_1f8;
    lVar23 = 0;
    pbVar13 = pbVar25;
    do {
      if (pbVar13 == pbVar27) {
        if (local_1f8 < 2) {
          *local_210 = 0;
          local_210[1] = 8;
          local_210[2] = 0;
          if (local_208 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          core::ptr::
          drop_in_place<alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>_>
                    ();
          return;
        }
        local_258 = local_200 + local_1f8;
        local_1f0 = *(longlong *)(in_RDX + 0x20);
        local_1e8 = *(undefined *******)(in_RDX + 0x28);
        local_fc = -local_f8;
        if ((int)-local_f8 < 0) {
          local_fc = local_f8;
        }
        local_268 = uVar6 & 0xffffffff;
        local_1dc = (int)uVar6 + 1;
        bVar19 = true;
        puVar14 = local_200;
        goto LAB_14123027e;
      }
      pbVar25 = pbVar13 + 1;
      lVar23 = lVar23 + -1;
      bVar5 = *pbVar13;
      pbVar13 = pbVar25;
    } while ((bVar5 & 0xf) == 0);
    if (local_1f8 == local_208) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    lVar26 = lVar26 - lVar23;
    local_200[uVar18] = lVar26 - 1;
    local_1f8 = uVar18 + 1;
  } while( true );
LAB_14123027e:
  if (puVar14 == local_258) {
LAB_141230f41:
    local_210[2] = local_218;
    *local_210 = local_228;
    local_210[1] = lStack_220;
    if (local_208 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
  puVar11 = puVar14 + 1;
  puVar15 = puVar14;
  if (bVar19) {
    if (puVar11 == local_258) goto LAB_141230f41;
    puVar15 = puVar11;
    local_310 = puVar14;
    puVar11 = puVar14 + 2;
  }
  local_260 = puVar11;
  ppppppuVar7 = (undefined8 ******)*local_310;
  ppppppuVar8 = (undefined8 ******)*puVar15;
  local_1c8._8_4_ = local_290;
  local_1c8._0_8_ = local_298;
  lVar23 = (longlong)ppppppuVar8 + (1 - (longlong)ppppppuVar7);
  local_1d8 = local_300;
  ppuStack_1d0 = (undefined **)local_308;
  local_328 = (undefined ******)ppppppuVar8;
  local_320 = (undefined ******)ppppppuVar7;
  local_310 = puVar15;
  local_1b8 = (undefined ******)ppppppuVar7;
  uStack_1b0 = lVar23;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<f32,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<f32>_>_>_>_>
            ();
  local_1c8._8_4_ = uStack_270;
  local_1c8._0_8_ = local_278;
  uStack_1bc = uStack_26c;
  fVar31 = 0.0;
  ppppppuVar17 = ppppppuVar7;
  lVar26 = lVar23;
  local_1d8 = local_288;
  ppuStack_1d0 = (undefined **)uStack_280;
  while (lVar26 != 0) {
    uStack_1b0 = lVar26 + -1;
    if (ppppppuVar17 == (undefined8 ******)0x0) {
      local_f1 = 1;
      local_1b8 = (undefined ******)ppppppuVar17;
      ::utils::loop_range::impl$2::next();
      ppuVar20 = ppuStack_1d0;
      ppuVar22 = local_1d8;
      if ((int)local_130 != 1) break;
      local_f1 = 1;
      ::utils::loop_range::LoopIdx::index();
      uVar18 = extraout_RAX;
      lVar26 = uStack_1b0;
      if (ppuVar20 <= extraout_RAX) {
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
    }
    else {
      local_1b8 = (undefined ******)0x0;
      ppppppuVar21 = (undefined8 ******)0x0;
      if (ppppppuVar17 != (undefined8 ******)0x0) {
        do {
          local_f1 = 1;
          ::utils::loop_range::impl$2::next();
          ppuVar22 = ppuStack_1d0;
          if (((ulonglong)local_130 & 1) == 0) goto LAB_141230360;
          local_f1 = 1;
          ::utils::loop_range::LoopIdx::index();
          if (ppuVar22 <= extraout_RAX_00) {
            local_f1 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          ppppppuVar21 = (undefined8 ******)((longlong)ppppppuVar21 + 1);
        } while (ppppppuVar21 != ppppppuVar17);
      }
      local_f1 = 1;
      ::utils::loop_range::impl$2::next();
      ppuVar20 = ppuStack_1d0;
      ppuVar22 = local_1d8;
      if ((int)local_130 != 1) break;
      local_f1 = 1;
      ::utils::loop_range::LoopIdx::index();
      uVar18 = extraout_RAX_01;
      lVar26 = uStack_1b0;
      if (ppuVar20 <= extraout_RAX_01) {
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
    }
    uStack_1b0._4_4_ = (undefined4)((ulonglong)lVar26 >> 0x20);
    lVar1 = (longlong)ppuVar22 + uVar18 * 0xc;
    if (lVar1 == 0) break;
    fVar32 = *(float *)(lVar1 + 4);
    fVar28 = fVar32;
    if (fVar31 <= fVar32) {
      fVar28 = fVar31;
    }
    fVar31 = (float)(~-(uint)NAN(fVar31) & (uint)fVar28 | -(uint)NAN(fVar31) & (uint)fVar32);
    ppppppuVar17 = (undefined8 ******)local_1b8;
  }
LAB_141230360:
  fVar32 = 3.4028235e+38;
  if (in_stack_00000058 != '\0') {
    local_1c8._8_4_ = local_230;
    local_1c8._0_8_ = local_238;
    fVar32 = 3.4028235e+38;
    ppppppuVar17 = ppppppuVar7;
    lVar26 = lVar23;
    local_1d8 = in_stack_00000028;
    ppuStack_1d0 = (undefined **)in_stack_00000030;
    while (lVar26 != 0) {
      uStack_1b0 = lVar26 + -1;
      if (ppppppuVar17 == (undefined8 ******)0x0) {
        local_f1 = 1;
        local_1b8 = (undefined ******)ppppppuVar17;
        ::utils::loop_range::impl$2::next();
        ppuVar20 = ppuStack_1d0;
        ppuVar22 = local_1d8;
        if ((int)local_130 != 1) break;
        local_f1 = 1;
        ::utils::loop_range::LoopIdx::index();
        uVar18 = extraout_RAX_08;
        lVar26 = uStack_1b0;
        if (ppuVar20 <= extraout_RAX_08) {
          local_f1 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
      }
      else {
        local_1b8 = (undefined ******)0x0;
        ppppppuVar21 = (undefined8 ******)0x0;
        if (ppppppuVar17 != (undefined8 ******)0x0) {
          do {
            local_f1 = 1;
            ::utils::loop_range::impl$2::next();
            ppuVar22 = ppuStack_1d0;
            if (((ulonglong)local_130 & 1) == 0) goto LAB_1412303c0;
            local_f1 = 1;
            ::utils::loop_range::LoopIdx::index();
            if (ppuVar22 <= extraout_RAX_09) {
              local_f1 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            ppppppuVar21 = (undefined8 ******)((longlong)ppppppuVar21 + 1);
          } while (ppppppuVar21 != ppppppuVar17);
        }
        local_f1 = 1;
        ::utils::loop_range::impl$2::next();
        ppuVar20 = ppuStack_1d0;
        ppuVar22 = local_1d8;
        if ((int)local_130 != 1) break;
        local_f1 = 1;
        ::utils::loop_range::LoopIdx::index();
        uVar18 = extraout_RAX_10;
        lVar26 = uStack_1b0;
        if (ppuVar20 <= extraout_RAX_10) {
          local_f1 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
      }
      uStack_1b0._4_4_ = (undefined4)((ulonglong)lVar26 >> 0x20);
      pfVar2 = (float *)((longlong)ppuVar22 + uVar18 * 4);
      if (pfVar2 == (float *)0x0) break;
      fVar28 = *pfVar2;
      fVar29 = fVar28;
      if (fVar32 <= fVar28) {
        fVar29 = fVar32;
      }
      fVar32 = (float)(~-(uint)NAN(fVar32) & (uint)fVar29 | -(uint)NAN(fVar32) & (uint)fVar28);
      ppppppuVar17 = (undefined8 ******)local_1b8;
    }
  }
LAB_1412303c0:
  if (0.0 <= fVar31) {
    fVar31 = 0.0;
  }
  local_318 = local_2b0;
  ppuStack_1d0 = (undefined **)((longlong)ppuStack_2b8 + local_2b0 * 4);
  uStack_1b0._0_4_ = local_230;
  local_1b8 = (undefined ******)local_238;
  local_170._8_4_ = uStack_270;
  local_170._0_8_ = local_278;
  local_170._12_4_ = uStack_26c;
  ppuStack_180 = local_288;
  uStack_178 = uStack_280;
  local_1c8._0_8_ = in_stack_00000028;
  stack0xfffffffffffffe40 = in_stack_00000030;
  local_198 = 0;
  ppppppuStack_190 = (undefined ******)0x0;
  local_188 = 0;
  local_150 = 0;
  uStack_148 = 0;
  local_140 = 0;
  fVar31 = ABS(fVar31) + -0.75;
  if (fVar31 <= 0.0) {
    fVar31 = 0.0;
  }
  fVar32 = fVar32 + 0.92;
  ppuVar22 = ppuStack_2b8;
  local_1a8 = (undefined ******)ppppppuVar7;
  pfStack_1a0 = (float *)lVar23;
  local_160 = (undefined ******)ppppppuVar7;
  local_158 = lVar23;
  while ((ppuVar20 = ppuVar22, ppppppuVar10 = local_1a8, ppuVar20 != ppuStack_1d0 &&
         (local_1d8 = (undefined **)((longlong)ppuVar20 + 4), pfStack_1a0 != (float *)0x0))) {
    pfStack_1a0 = (float *)((longlong)pfStack_1a0 + -1);
    if ((undefined8 ******)local_1a8 == (undefined8 ******)0x0) {
      local_f1 = 1;
      ::utils::loop_range::impl$2::next();
      uVar18 = stack0xfffffffffffffe40;
      ppuVar22 = (undefined **)local_1c8._0_8_;
      if ((int)local_130 != 1) break;
      local_f1 = 1;
      ::utils::loop_range::LoopIdx::index();
      uVar16 = extraout_RAX_02;
      if (uVar18 <= extraout_RAX_02) {
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
    }
    else {
      local_1a8 = (undefined ******)0x0;
      ppppppuVar17 = (undefined8 ******)0x0;
      if ((undefined8 ******)ppppppuVar10 != (undefined8 ******)0x0) {
        do {
          local_f1 = 1;
          ::utils::loop_range::impl$2::next();
          uVar18 = stack0xfffffffffffffe40;
          if (((ulonglong)local_130 & 1) == 0) goto LAB_141230ac0;
          local_f1 = 1;
          ::utils::loop_range::LoopIdx::index();
          if (uVar18 <= extraout_RAX_03) {
            local_f1 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          ppppppuVar17 = (undefined8 ******)((longlong)ppppppuVar17 + 1);
        } while (ppppppuVar17 != (undefined8 ******)ppppppuVar10);
      }
      local_f1 = 1;
      ::utils::loop_range::impl$2::next();
      uVar18 = stack0xfffffffffffffe40;
      ppuVar22 = (undefined **)local_1c8._0_8_;
      if ((int)local_130 != 1) break;
      local_f1 = 1;
      ::utils::loop_range::LoopIdx::index();
      uVar16 = extraout_RAX_04;
      if (uVar18 <= extraout_RAX_04) {
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
    }
    ppppppuVar10 = local_160;
    pfVar2 = (float *)((longlong)ppuVar22 + uVar16 * 4);
    if ((pfVar2 == (float *)0x0) || (local_158 == 0)) break;
    local_158 = local_158 + -1;
    if ((undefined8 ******)local_160 == (undefined8 ******)0x0) {
      local_f1 = 1;
      ::utils::loop_range::impl$2::next();
      uVar18 = uStack_178;
      ppuVar22 = ppuStack_180;
      if ((int)local_130 == 1) {
        local_f1 = 1;
        ::utils::loop_range::LoopIdx::index();
        if (uVar18 <= extraout_RAX_05) {
          local_f1 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar26 = (longlong)ppuVar22 + extraout_RAX_05 * 0xc;
      }
      else {
        lVar26 = 0;
      }
    }
    else {
      local_160 = (undefined ******)0x0;
      ppppppuVar17 = (undefined8 ******)0x0;
      if ((undefined8 ******)ppppppuVar10 != (undefined8 ******)0x0) {
        do {
          local_f1 = 1;
          local_2f8 = (undefined ******)ppppppuVar17;
          ::utils::loop_range::impl$2::next();
          uVar18 = uStack_178;
          if (((ulonglong)local_130 & 1) == 0) goto LAB_141230ac0;
          local_f1 = 1;
          ::utils::loop_range::LoopIdx::index();
          if (uVar18 <= extraout_RAX_06) {
            local_f1 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          ppppppuVar17 = (undefined8 ******)((longlong)local_2f8 + 1);
        } while (ppppppuVar17 != (undefined8 ******)ppppppuVar10);
      }
      local_f1 = 1;
      ::utils::loop_range::impl$6::next<glam::f32::vec3::Vec3>();
      lVar26 = extraout_RAX_07;
    }
    if (lVar26 == 0) break;
    fVar28 = *(float *)ppuVar20 + fVar31;
    *(float *)ppuVar20 = fVar28;
    ppuVar22 = local_1d8;
    if (in_stack_00000058 != '\0') {
      fVar30 = (*pfVar2 - *(float *)(lVar26 + 4)) + -0.96000004;
      fVar29 = fVar30;
      if (fVar28 <= fVar30) {
        fVar29 = fVar28;
      }
      fVar28 = (float)(~-(uint)NAN(fVar28) & (uint)fVar29 | -(uint)NAN(fVar28) & (uint)fVar30);
      fVar29 = fVar32;
      if (fVar28 <= fVar32) {
        fVar29 = fVar28;
      }
      *(uint *)ppuVar20 = -(uint)NAN(fVar28) & (uint)fVar32 | ~-(uint)NAN(fVar28) & (uint)fVar29;
      ppuVar22 = local_1d8;
    }
  }
LAB_141230ac0:
  local_1c8._8_4_ = uStack_270;
  uStack_1bc = uStack_26c;
  local_1c8._0_8_ = local_278;
  local_1d8 = local_288;
  ppuStack_1d0 = (undefined **)uStack_280;
  local_f1 = 1;
  local_1b8 = (undefined ******)ppppppuVar7;
  uStack_1b0 = lVar23;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>_>_>_>_>
            ();
  local_1d8 = in_stack_00000048;
  ppuStack_1d0 = (undefined **)in_stack_00000050;
  local_1c8._8_4_ = local_f8;
  local_1c8._0_8_ = uVar6;
  local_f3 = 1;
  local_1b8 = (undefined ******)ppppppuVar7;
  uStack_1b0 = lVar23;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>,core::iter::adapters::copied::Copied<core::iter::adapters::take::Take<core::iter::adapters::skip::Skip<utils::loop_range::LoopSlice<utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemanticThresholded3D>_>_>_>_>_>
            ();
  uVar18 = local_2c8;
  lVar26 = local_318;
  if (local_2c8 < 2) {
    local_248 = (undefined *******)&local_130;
    local_240 = core::fmt::impl$100::fmt<usize,usize>;
    local_1d8 = (undefined **)&DAT_142c1e1f0;
    ppuStack_1d0 = (undefined **)0x1;
    local_1b8 = (undefined ******)0x0;
    stack0xfffffffffffffe40 = 1;
    local_1c8._0_8_ = &local_248;
    local_f2 = 1;
    local_130 = ppppppuVar7;
    local_128 = (undefined ******)ppppppuVar8;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (local_1e8 <= ppppppuVar7) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  local_f2 = 1;
  ::utils::curve::impl$12::
  sample_at_coord<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>
            ();
  puVar4 = puStack_2d0;
  *(undefined4 *)(puStack_2d0 + 1) = ppuStack_1d0._0_4_;
  *puStack_2d0 = local_1d8;
  if (local_1e8 <= ppppppuVar8) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  local_f2 = 1;
  ::utils::curve::impl$12::
  sample_at_coord<glam::f32::vec3::Vec3,utils::loop_range::LoopSlice<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D>
            ();
  uVar16 = lVar26 - 1;
  if (uVar18 <= uVar16) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  *(undefined4 *)((longlong)puVar4 + uVar16 * 0xc + 8) = ppuStack_1d0._0_4_;
  *(undefined ***)((longlong)puVar4 + uVar16 * 0xc) = local_1d8;
  if (local_f8 == 0) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_rem_by_zero();
  }
  iVar24 = *(int *)(local_1f0 + (longlong)ppppppuVar7 * 8);
  iVar12 = (int)local_268 + iVar24;
  if ((local_f8 == 0xffffffff) && (SBORROW4(0,iVar12))) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_rem_overflow();
  }
  iVar12 = iVar12 % (int)local_f8;
  if (in_stack_00000050 <= (ulonglong)(longlong)(int)((iVar12 >> 0x1f & local_fc) + iVar12)) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  iVar24 = iVar24 + local_1dc;
  if ((local_f8 == 0xffffffff) && (SBORROW4(0,iVar24))) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_rem_overflow();
  }
  iVar24 = iVar24 % (int)local_f8;
  if (in_stack_00000050 <= (ulonglong)(longlong)(int)((iVar24 >> 0x1f & local_fc) + iVar24)) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  local_f2 = 1;
  ::utils::wall_space::impl$1::subtract();
  local_f2 = 1;
  ::utils::wall_space::impl$1::project_clamp();
  uVar18 = local_2e0;
  pfVar2 = pfStack_2e8;
  if ((extraout_XMM0_Da < 0.0) || (1.0 < extraout_XMM0_Da)) {
    local_130 = &local_248;
    local_128 = (undefined ******)core::fmt::float::impl$5::fmt;
    local_1d8 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    ppuStack_1d0 = (undefined **)0x1;
    local_1b8 = (undefined ******)0x0;
    stack0xfffffffffffffe40 = 1;
    local_1c8._0_8_ = &local_130;
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(extraout_XMM0_Da)) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (local_2e0 == 0) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  *pfStack_2e8 = extraout_XMM0_Da;
  iVar24 = *(int *)(local_1f0 + (longlong)ppppppuVar8 * 8);
  iVar12 = (int)local_268 + iVar24;
  if ((local_f8 == 0xffffffff) && (SBORROW4(0,iVar12))) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_rem_overflow();
  }
  iVar12 = iVar12 % (int)local_f8;
  if (in_stack_00000050 <= (ulonglong)(longlong)(int)((iVar12 >> 0x1f & local_fc) + iVar12)) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  iVar24 = iVar24 + local_1dc;
  if ((local_f8 == 0xffffffff) && (SBORROW4(0,iVar24))) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_const::panic_const_rem_overflow();
  }
  iVar24 = iVar24 % (int)local_f8;
  if (in_stack_00000050 <= (ulonglong)(longlong)(int)((iVar24 >> 0x1f & local_fc) + iVar24)) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  local_f2 = 1;
  ::utils::wall_space::impl$1::subtract();
  local_f2 = 1;
  ::utils::wall_space::impl$1::project_clamp();
  local_248 = (undefined *******)CONCAT44(local_248._4_4_,extraout_XMM0_Da_00);
  if ((extraout_XMM0_Da_00 < 0.0) || (1.0 < extraout_XMM0_Da_00)) {
    local_130 = &local_248;
    local_128 = (undefined ******)core::fmt::float::impl$5::fmt;
    local_1d8 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    ppuStack_1d0 = (undefined **)0x1;
    local_1b8 = (undefined ******)0x0;
    stack0xfffffffffffffe40 = 1;
    local_1c8._0_8_ = &local_130;
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(extraout_XMM0_Da_00)) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  if (uVar18 <= uVar16) {
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  pfVar2[lVar26 + -1] = extraout_XMM0_Da_00;
  ppuStack_1d0 = (undefined **)CONCAT44(ppuStack_1d0._4_4_,*(undefined4 *)(local_250 + 1));
  local_1d8 = (undefined **)*local_250;
  local_f2 = 1;
  ::utils::loop_range::LoopRange::subslice();
  local_f2 = 1;
  ::utils::loop_range::LoopRange::is_empty();
  lVar26 = local_218;
  if (extraout_AL != '\0') {
    local_1d8 = local_250;
    ppuStack_1d0 = (undefined **)::utils::loop_range::impl$16::fmt;
    stack0xfffffffffffffe40 = (ulonglong)core::fmt::num::impl$89::fmt;
    local_1c8._0_8_ = &local_320;
    local_1b8 = (undefined ******)&local_328;
    uStack_1b0._0_4_ = 0x400c84c0;
    uStack_1b0._4_4_ = 1;
    local_130 = (undefined8 ******)&PTR_DAT_142c1e368;
    local_128 = (undefined ******)0x3;
    local_110 = 0;
    local_120 = &local_1d8;
    local_118 = 3;
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  local_1d8 = local_2c0;
  ppuStack_1d0 = ppuStack_2b8;
  uStack_1b0._0_4_ = (undefined4)local_2c8;
  uStack_1b0._4_4_ = (undefined4)(local_2c8 >> 0x20);
  local_1b8 = (undefined ******)puStack_2d0;
  stack0xfffffffffffffe40 = local_2d8;
  local_1c8._0_8_ = local_2b0;
  local_198 = local_2e0;
  local_1a8 = (undefined ******)local_2f0;
  pfStack_1a0 = pfStack_2e8;
  uStack_178 = CONCAT44(uStack_178._4_4_,local_29c);
  ppuStack_180 = (undefined **)local_2a4;
  ppppppuStack_190 = (undefined ******)ppppppuVar7;
  local_188 = (longlong)ppppppuVar8 + 1;
  if (local_218 == local_228) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>
              ();
  }
  lVar23 = lVar26 * 0x68;
  auVar9._8_4_ = (undefined4)uStack_1b0;
  auVar9._0_8_ = local_1b8;
  auVar9._12_4_ = uStack_1b0._4_4_;
  *(ulonglong *)(lStack_220 + 0x60 + lVar23) = uStack_178;
  plVar3 = (longlong *)(lStack_220 + 0x50 + lVar23);
  *plVar3 = local_188;
  plVar3[1] = (longlong)ppuStack_180;
  puVar14 = (ulonglong *)(lStack_220 + 0x40 + lVar23);
  *puVar14 = local_198;
  puVar14[1] = (ulonglong)ppppppuStack_190;
  puVar4 = (undefined8 *)(lStack_220 + 0x30 + lVar23);
  *puVar4 = local_1a8;
  puVar4[1] = pfStack_1a0;
  *(undefined1 (*) [16])(lStack_220 + 0x20 + lVar23) = auVar9;
  puVar4 = (undefined8 *)(lStack_220 + 0x10 + lVar23);
  *puVar4 = local_1c8._0_8_;
  puVar4[1] = stack0xfffffffffffffe40;
  *(undefined8 *)(lStack_220 + lVar23) = local_1d8;
  ((undefined8 *)(lStack_220 + lVar23))[1] = ppuStack_1d0;
  local_218 = lVar26 + 1;
  bVar19 = false;
  puVar14 = local_260;
  goto LAB_14123027e;
}

