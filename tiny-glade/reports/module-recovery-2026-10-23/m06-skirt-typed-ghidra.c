/* B patched seed，独立 worker 研究签名；原始反编译不是作者源码。 */

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_wall_constructor::utils::wall_skirts::add_skirts
          (void *wall,ulonglong wall_id,void *out,void *curve,float *bottom,ulonglong bottom_len,
          float *profile,ulonglong profile_len,void *holes)

{
  float *pfVar1;
  float *pfVar2;
  undefined8 *puVar3;
  float fVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined1 auVar9 [16];
  char cVar10;
  float fVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  bool bVar14;
  char extraout_AL;
  uint extraout_EAX;
  uint extraout_EAX_00;
  undefined4 extraout_EAX_01;
  float *extraout_RAX;
  float *extraout_RAX_00;
  float *extraout_RAX_01;
  longlong lVar15;
  ulonglong uVar16;
  undefined8 *extraout_RAX_02;
  undefined1 *extraout_RAX_03;
  longlong lVar17;
  longlong lVar18;
  undefined1 *puVar19;
  ulonglong uVar20;
  float *pfVar21;
  float *pfVar22;
  longlong lVar23;
  undefined1 *aabb;
  float *unaff_R13;
  float *pfVar24;
  float *pfVar25;
  longlong lVar26;
  ulonglong uVar27;
  uint uVar28;
  float fVar29;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float fVar30;
  float extraout_XMM0_Da_08;
  float fVar31;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float extraout_XMM0_Da_11;
  undefined4 extraout_XMM0_Da_12;
  float fVar32;
  float fVar33;
  float fVar34;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_318;
  undefined5 uStack_310;
  undefined3 uStack_30b;
  undefined5 uStack_308;
  undefined8 local_2b8;
  undefined8 uStack_2b0;
  undefined8 local_2a8;
  undefined5 uStack_2a0;
  undefined3 uStack_29b;
  undefined5 uStack_298;
  char local_293;
  undefined2 local_292;
  undefined1 local_288 [8];
  float fStack_280;
  float fStack_27c;
  undefined1 local_278 [12];
  undefined4 uStack_26c;
  undefined8 *local_260;
  ulonglong local_258;
  float *local_250;
  longlong local_248;
  longlong local_238;
  ulonglong local_230;
  ulonglong local_228;
  undefined8 *local_220;
  longlong local_218;
  longlong local_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  float local_1d4;
  float local_1d0;
  float local_1cc;
  undefined8 local_1c8;
  longlong local_1c0;
  float *local_1b8;
  longlong local_1b0;
  longlong *local_1a8;
  void *local_1a0;
  longlong local_198;
  ulonglong local_190;
  int local_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  float local_140;
  float local_13c;
  float *local_138;
  longlong local_130;
  longlong local_128;
  float *local_120;
  float *local_118;
  void *local_110;
  float *local_108;
  float local_fc;
  float local_f8;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_1a8 = out;
  local_1a0 = curve;
  fVar29 = country_core::resources::walls::public_wall_state::PublicWallState::wall_max_y(wall);
  country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle();
  if (local_188 == 1) {
    local_1f8 = CONCAT44(uStack_170,uStack_174);
    local_208 = CONCAT44(uStack_180,uStack_184);
    uStack_200 = CONCAT44(uStack_178,uStack_17c);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    local_108 = extraout_RAX;
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    local_198 = 4;
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    *local_108 = extraout_XMM0_Da;
    local_108[1] = extraout_XMM0_Da_00;
    local_108[2] = extraout_XMM0_Da_01;
    local_108[3] = extraout_XMM0_Da_02;
    local_108[4] = extraout_XMM0_Da_03;
    local_108[5] = extraout_XMM0_Da_04;
    local_108[6] = extraout_XMM0_Da_05;
    local_108[7] = extraout_XMM0_Da_06;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *(ulonglong *)extraout_RAX_00 =
         CONCAT44(__xmm_00000000000000003f80000000000000._4_4_,
                  (undefined4)__xmm_00000000000000003f80000000000000);
    local_198 = 1;
    local_108 = extraout_RAX_00;
  }
  country_core::resources::walls::split_height_into_skirts();
  fVar11 = __real_3f800000;
  local_248 = CONCAT44(uStack_184,local_188);
  pfVar21 = (float *)CONCAT44(uStack_17c,uStack_180);
  pfVar1 = pfVar21 + CONCAT44(uStack_174,uStack_178);
  local_f8 = *(float *)((longlong)local_1a0 + 0x30);
  local_1cc = fVar29 + local_f8;
  pfVar2 = local_108 + local_198 * 2;
  local_128 = *(longlong *)((longlong)local_1a0 + 8);
  local_190 = *(ulonglong *)((longlong)local_1a0 + 0x10);
  lVar23 = local_128 + local_190 * 8;
  pfVar24 = *(float **)((longlong)local_1a0 + 0x20);
  uVar27 = (ulonglong)((int)(local_190 * 8 + 0x7ffffffe8 >> 3) + 1);
  lVar17 = local_190 * 8 + -0x10;
  fVar29 = __real_40600000;
  do {
    do {
      if (pfVar21 == pfVar1) {
        if (local_248 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
        return;
      }
      fVar4 = *pfVar21;
      pfVar21 = pfVar21 + 1;
    } while (fVar4 < *(float *)((longlong)wall + 0x280) + fVar29);
    local_238 = lVar17;
    local_230 = uVar27;
    local_138 = pfVar24;
    local_130 = lVar23;
    local_118 = unaff_R13;
    local_110 = wall;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = 0.0;
    local_1c0 = 1;
    local_1b0 = 1;
    lVar23 = 1;
    pfVar24 = extraout_RAX_01;
    local_1b8 = extraout_RAX_01;
    for (pfVar22 = local_108; pfVar22 != pfVar2; pfVar22 = pfVar22 + 2) {
      ceilf(((pfVar22[1] - *pfVar22) * local_f8) / __real_3f30a3d7);
      local_188 = 0;
      uStack_184 = 0;
      uStack_180 = 4;
      uStack_17c = 0;
      uStack_178 = 0;
      uStack_174 = 0;
      ::utils::random_splits_into();
      uVar27 = CONCAT44(uStack_174,uStack_178);
      uVar20 = 0;
      if (uVar27 != 0) {
        uVar20 = uVar27 - 1;
      }
      lVar17 = CONCAT44(uStack_17c,uStack_180);
      if ((ulonglong)(local_1c0 - lVar23) < uVar20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        pfVar24 = local_1b8;
        lVar23 = local_1b0;
      }
      if (1 < uVar27) {
        uVar20 = 0;
        do {
          fVar29 = *(float *)(lVar17 + 4 + uVar20 * 4);
          if ((fVar29 < 0.0) || (fVar11 < fVar29)) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          ::utils::lerp();
          pfVar24[lVar23] = extraout_XMM0_Da_07;
          lVar23 = lVar23 + 1;
          uVar20 = uVar20 + 1;
        } while (uVar20 != (uVar27 - 1 & 0x3fffffffffffffff));
      }
      local_1b0 = lVar23;
      if (CONCAT44(uStack_184,local_188) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    if (local_190 == 1) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    if (local_190 == 0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    fVar29 = local_138[1] - *local_138;
    fVar30 = (float)((uint)fVar29 & ___xmm_7fffffff7fffffff7fffffff7fffffff);
    if (NAN(fVar29)) {
      uVar27 = 0;
      fVar29 = fVar11;
      if (local_190 != 2) {
        fVar32 = (float)((uint)(local_138[2] - local_138[1]) &
                        ___xmm_7fffffff7fffffff7fffffff7fffffff);
        fVar30 = fVar30 + fVar32;
        if (0.0 <= fVar30) {
          uVar27 = 1;
        }
        else {
          lVar17 = local_238;
          lVar15 = 3;
          fVar33 = local_138[2];
          do {
            lVar26 = lVar15;
            lVar17 = lVar17 + -8;
            uVar27 = local_230;
            if (lVar17 == 0) goto LAB_1412227f8;
            fVar32 = (float)((uint)(local_138[lVar26] - fVar33) &
                            ___xmm_7fffffff7fffffff7fffffff7fffffff);
            fVar30 = fVar30 + fVar32;
            lVar15 = lVar26 + 1;
            fVar33 = local_138[lVar26];
          } while (fVar30 < 0.0);
          uVar27 = lVar26 - 1;
        }
        goto LAB_1412227e5;
      }
    }
    else {
      uVar27 = 0;
      fVar32 = fVar30;
LAB_1412227e5:
      fVar29 = fVar11 - fVar30 / fVar32;
    }
LAB_1412227f8:
    uVar20 = uVar27 & 0xffffffff;
    _local_278 = ZEXT416((uint)fVar29);
    local_288._0_4_ = fVar29 * profile[uVar20 + 1] + profile[uVar20] * (fVar11 - fVar29);
    local_288._4_4_ = fVar29 * bottom[uVar20 + 1] + bottom[uVar20] * (fVar11 - fVar29);
    register0x00001248 = 0;
    local_250 = local_1b8 + lVar23;
    bVar14 = true;
    lVar23 = local_130;
    wall = local_110;
    unaff_R13 = local_118;
    pfVar24 = local_138;
LAB_141222860:
    while( true ) {
      fVar29 = __real_40600000;
      uVar12 = local_288._0_4_;
      uVar13 = local_288._4_4_;
      if (local_1b8 == local_250) break;
      pfVar22 = local_1b8 + 1;
      pfVar25 = local_1b8;
      if (bVar14) {
        if (pfVar22 == local_250) break;
        unaff_R13 = local_1b8;
        pfVar25 = pfVar22;
        pfVar22 = local_1b8 + 2;
      }
      local_120 = pfVar22;
      local_140 = *unaff_R13;
      local_13c = *pfVar25;
      fVar29 = (local_13c - local_140) * __real_3f000000;
      if (fVar29 < 0.0) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      uVar20 = uVar27 & 0xffffffff;
      if ((int)uVar27 == 0) {
        lVar17 = local_128;
        if (local_128 + 8 == lVar23) goto LAB_1412229c0;
LAB_141222912:
        fVar32 = (float)((uint)(pfVar24[uVar20 + 1] - pfVar24[uVar20]) &
                        ___xmm_7fffffff7fffffff7fffffff7fffffff);
        fVar30 = (fVar11 - (float)local_278._0_4_) * fVar32 - fVar29;
        if (0.0 <= fVar30) {
LAB_141222988:
          local_278._4_4_ = 0;
          local_278._8_4_ = 0.0;
          uStack_26c = 0;
          local_278._0_4_ = fVar11 - fVar30 / fVar32;
        }
        else {
          lVar17 = lVar17 + 0x10;
          if (lVar17 != lVar23) {
            lVar15 = uVar20 + 2;
            fVar32 = pfVar24[uVar20 + 1];
            do {
              pfVar22 = pfVar24 + lVar15;
              fVar32 = (float)((uint)(*pfVar22 - fVar32) & ___xmm_7fffffff7fffffff7fffffff7fffffff);
              fVar30 = fVar30 + fVar32;
              if (0.0 <= fVar30) {
                uVar27 = (ulonglong)((int)uVar27 + 1);
                goto LAB_141222988;
              }
              lVar17 = lVar17 + 8;
              uVar27 = (ulonglong)((int)uVar27 + 1);
              lVar15 = lVar15 + 1;
              fVar32 = *pfVar22;
            } while (lVar17 != lVar23);
          }
          local_278._4_4_ = 0;
          local_278._8_4_ = 0.0;
          uStack_26c = 0;
          local_278._0_4_ = fVar11;
        }
      }
      else {
        if ((uVar20 < local_190) && (lVar17 = local_128 + uVar20 * 8, lVar17 + 8 != lVar23))
        goto LAB_141222912;
LAB_1412229c0:
      }
      uVar20 = uVar27 & 0xffffffff;
      fVar32 = fVar11 - (float)local_278._0_4_;
      lVar17 = local_128;
      fVar30 = (float)local_278._0_4_;
      if ((int)uVar27 == 0) {
joined_r0x000141222a94:
        if (lVar17 + 8 != lVar23) {
          fVar33 = (float)((uint)(pfVar24[uVar20 + 1] - pfVar24[uVar20]) &
                          ___xmm_7fffffff7fffffff7fffffff7fffffff);
          fVar30 = fVar32 * fVar33 - fVar29;
          if (0.0 <= fVar30) {
LAB_141222a6a:
            local_278._4_4_ = 0;
            local_278._8_4_ = 0.0;
            uStack_26c = 0;
            fVar30 = fVar11 - fVar30 / fVar33;
          }
          else {
            lVar17 = lVar17 + 0x10;
            if (lVar17 != lVar23) {
              lVar15 = uVar20 + 2;
              fVar33 = pfVar24[uVar20 + 1];
              do {
                pfVar22 = pfVar24 + lVar15;
                fVar33 = (float)((uint)(*pfVar22 - fVar33) & ___xmm_7fffffff7fffffff7fffffff7fffffff
                                );
                fVar30 = fVar30 + fVar33;
                if (0.0 <= fVar30) {
                  uVar27 = (ulonglong)((int)uVar27 + 1);
                  goto LAB_141222a6a;
                }
                lVar17 = lVar17 + 8;
                uVar27 = (ulonglong)((int)uVar27 + 1);
                lVar15 = lVar15 + 1;
                fVar33 = *pfVar22;
              } while (lVar17 != lVar23);
            }
            local_278._4_4_ = 0;
            local_278._8_4_ = 0.0;
            uStack_26c = 0;
            fVar30 = fVar11;
          }
        }
      }
      else if (uVar20 < local_190) {
        lVar17 = local_128 + uVar20 * 8;
        goto joined_r0x000141222a94;
      }
      fVar33 = profile[uVar20 + 1] * (float)local_278._0_4_;
      local_fc = (float)local_278._0_4_ * bottom[uVar20 + 1] + fVar32 * bottom[uVar20];
      uVar16 = uVar27 & 0xffffffff;
      local_278._0_4_ = fVar30;
      local_288._0_4_ = fVar30 * profile[uVar16 + 1] + profile[uVar16] * (fVar11 - fVar30);
      fVar34 = fVar30 * bottom[uVar16 + 1] + bottom[uVar16] * (fVar11 - fVar30);
      local_288._4_4_ = fVar34;
      fStack_280 = (float)local_278._8_4_ * 0.0 + 0.0;
      fStack_27c = (float)local_278._8_4_ * 0.0 + 0.0;
      fVar30 = fVar34;
      if ((float)uVar13 <= fVar34) {
        fVar30 = (float)uVar13;
      }
      fVar30 = (float)(~-(uint)NAN((float)uVar13) & (uint)fVar30 |
                      -(uint)NAN((float)uVar13) & (uint)fVar34);
      unaff_R13 = pfVar25;
      local_1b8 = local_120;
      if (1 < *(uint *)((longlong)wall + 600)) goto LAB_141222b70;
      bVar14 = false;
      if (fVar30 <= fVar4) goto LAB_141222ba2;
    }
    uVar27 = local_230;
    lVar17 = local_238;
    if (local_1c0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      uVar27 = local_230;
      lVar17 = local_238;
    }
  } while( true );
LAB_141222b70:
  bVar14 = false;
  if ((fVar4 < fVar30) ||
     (*(float *)((longlong)wall + 0x284) <= fVar4 - local_fc &&
      fVar4 - local_fc != *(float *)((longlong)wall + 0x284))) goto LAB_141222860;
LAB_141222ba2:
  fVar33 = profile[uVar20] * fVar32 + fVar33;
  fVar32 = (float)uVar12 + fVar33 + (float)local_288._0_4_;
  fVar30 = (float)local_288._0_4_;
  if (fVar32 <= 0.0) {
    fVar33 = __real_c2c80000;
    fVar30 = __real_c2c80000;
  }
  uVar28 = -(uint)(0.0 < fVar32);
  fVar32 = (float)uVar13 + (float)(~uVar28 & (uint)__real_c2c80000 | uVar28 & uVar12);
  local_fc = local_fc + fVar33;
  if (((fVar4 < fVar34 + fVar30) && (fVar4 < fVar32)) && (bVar14 = false, fVar4 < local_fc))
  goto LAB_141222860;
  fVar33 = (local_140 + local_13c) * __real_3f000000;
  ::utils::geometry::aabb::Aabb2::from_center_dims();
  puVar19 = &DAT_00000004;
  local_118 = pfVar25;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_02 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  local_188 = (int)local_358;
  uStack_184 = (undefined4)((ulonglong)local_358 >> 0x20);
  uStack_180 = (undefined4)uStack_350;
  uStack_17c = (undefined4)((ulonglong)uStack_350 >> 0x20);
  uStack_178 = 0;
  uStack_174 = 0;
  uStack_170 = 0;
  uStack_16c = 0;
  *(undefined1 (*) [16])(extraout_RAX_02 + 2) = ZEXT816(0);
  *extraout_RAX_02 = local_358;
  extraout_RAX_02[1] = uStack_350;
  *(undefined4 *)(extraout_RAX_02 + 4) = 0;
  *(undefined2 *)((longlong)extraout_RAX_02 + 0x24) = 0;
  local_218 = 0;
  local_210 = 1;
  local_228 = 1;
  local_f1 = 1;
  local_220 = extraout_RAX_02;
  country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates();
  for (aabb = extraout_RAX_03; (aabb != puVar19 && (aabb != (undefined1 *)0x0)); aabb = aabb + 0x14)
  {
    local_f1 = 1;
    ::utils::geometry::aabb::Aabb2::aabb_width(aabb);
    local_f1 = 1;
    ::utils::geometry::aabb::Aabb2::aabb_height(aabb);
    local_f1 = 1;
    ::utils::geometry::aabb::Aabb2::center();
    local_f1 = 1;
    ::utils::geometry::aabb::Aabb2::from_center_dims();
    local_f1 = 1;
    ::utils::geometry::aabb::Aabb2::intersects();
    if (extraout_AL != '\0') {
      local_f1 = 1;
      resolve_hole_overlap::resolve_hole_overlap();
    }
    if (*(int *)((longlong)wall + 600) != 2) {
      local_f1 = 1;
      ::utils::geometry::aabb::Aabb2::center();
      local_f1 = 1;
      fVar31 = ::utils::geometry::aabb::Aabb2::aabb_width(&local_208);
      if (extraout_XMM0_Da_08 + fVar31 * __real_3f000000 <= local_f8) {
        local_f1 = 1;
        ::utils::geometry::aabb::Aabb2::center();
        local_f1 = 1;
        fVar31 = ::utils::geometry::aabb::Aabb2::aabb_width(&local_208);
        if (extraout_XMM0_Da_09 < fVar31 * __real_3f000000) {
          local_f1 = 1;
          ::utils::geometry::aabb::Aabb2::center();
          local_f1 = 1;
          ::utils::geometry::aabb::Aabb2::center();
          local_f1 = 1;
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          local_f1 = 1;
          resolve_hole_overlap::resolve_hole_overlap();
        }
      }
      else {
        local_f1 = 1;
        ::utils::geometry::aabb::Aabb2::center();
        local_f1 = 1;
        ::utils::geometry::aabb::Aabb2::center();
        local_f1 = 1;
        ::utils::geometry::aabb::Aabb2::from_center_dims();
        local_f1 = 1;
        resolve_hole_overlap::resolve_hole_overlap();
      }
    }
  }
  if (local_210 == 0) {
    if (local_228 == 0) goto LAB_141223390;
  }
  else {
    local_1d0 = (fVar33 - fVar29) * local_f8;
    local_1d4 = (fVar33 + fVar29) * local_f8;
    local_260 = local_220;
    local_258 = local_228;
    lVar17 = local_218;
    lVar23 = local_210;
    do {
      lVar23 = lVar23 + -1;
      uVar20 = 0;
      if (local_258 <= lVar17 + 1U) {
        uVar20 = local_258;
      }
      uStack_308 = (undefined5)
                   ((ulonglong)*(undefined8 *)((longlong)local_260 + lVar17 * 0x28 + 0x1d) >> 0x18);
      auVar9 = *(undefined1 (*) [16])(local_260 + lVar17 * 5 + 2);
      local_318 = auVar9._0_8_;
      uStack_310 = auVar9._8_5_;
      uStack_30b = auVar9._13_3_;
      cVar10 = *(char *)((longlong)local_260 + lVar17 * 0x28 + 0x25);
      wall = local_110;
      if (cVar10 == '\x02') break;
      local_292 = *(undefined2 *)((longlong)local_260 + lVar17 * 0x28 + 0x26);
      uStack_298 = uStack_308;
      local_2a8 = local_318;
      local_2b8 = local_260[lVar17 * 5];
      uStack_2b0 = (local_260 + lVar17 * 5)[1];
      uStack_2a0 = uStack_310;
      uStack_29b = uStack_30b;
      local_293 = cVar10;
      fVar29 = ::utils::geometry::aabb::Aabb2::aabb_height(&local_2b8);
      if (__real_3dcccccd <= fVar29) {
        ::utils::curve::Curve::
        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                  ();
        uVar5 = *(undefined8 *)(local_128 + (ulonglong)extraout_EAX * 8);
        uVar6 = *(undefined8 *)(local_128 + 8 + (ulonglong)extraout_EAX * 8);
        ::utils::curve::Curve::
        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                  ();
        uVar7 = *(undefined8 *)(local_128 + (ulonglong)extraout_EAX_00 * 8);
        uVar8 = *(undefined8 *)(local_128 + 8 + (ulonglong)extraout_EAX_00 * 8);
        fVar29 = (extraout_XMM0_Da_11 * (float)uVar8 + (fVar11 - extraout_XMM0_Da_11) * (float)uVar7
                 ) - (extraout_XMM0_Da_10 * (float)uVar6 +
                     (fVar11 - extraout_XMM0_Da_10) * (float)uVar5);
        fVar33 = (extraout_XMM0_Da_11 * (float)((ulonglong)uVar8 >> 0x20) +
                 (fVar11 - extraout_XMM0_Da_11) * (float)((ulonglong)uVar7 >> 0x20)) -
                 (extraout_XMM0_Da_10 * (float)((ulonglong)uVar6 >> 0x20) +
                 (fVar11 - extraout_XMM0_Da_10) * (float)((ulonglong)uVar5 >> 0x20));
        fVar31 = fVar11 / SQRT(fVar33 * fVar33 + fVar29 * fVar29);
        local_1c8 = CONCAT44(fVar31 * fVar33,fVar31 * fVar29);
        ::utils::swizzlers::impl$0::x0y();
        uVar28 = local_1c8._4_4_ ^ ___xmm_80000000800000008000000080000000;
        local_208 = CONCAT44((undefined4)local_1c8,uVar28);
        ::utils::swizzlers::impl$0::x0y();
        ::utils::geometry::aabb::Aabb2::aabb_width(&local_2b8);
        ::utils::geometry::aabb::Aabb2::aabb_height(&local_2b8);
        ::utils::geometry::aabb::Aabb2::center();
        ::utils::geometry::aabb::Aabb2::center();
        bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
        local_208 = CONCAT44(uVar28,extraout_XMM0_Da_12);
        ::utils::wall_space::WallSpaceCoord::to_world_space();
        country_core::resources::render::impl$1::from();
        country_core::geometry::instanced_wall::impl$1::from();
        local_168 = local_1e8;
        uStack_160 = uStack_1e0;
        uStack_178 = (undefined4)local_1f8;
        uStack_174 = (undefined4)((ulonglong)local_1f8 >> 0x20);
        uStack_170 = (undefined4)uStack_1f0;
        uStack_16c = (undefined4)((ulonglong)uStack_1f0 >> 0x20);
        local_188 = (int)local_208;
        uStack_184 = (undefined4)((ulonglong)local_208 >> 0x20);
        uStack_180 = (undefined4)uStack_200;
        uStack_17c = (undefined4)((ulonglong)uStack_200 >> 0x20);
        local_158 = 0;
        uStack_150 = 0;
        lVar15 = local_1a8[2];
        uVar5 = 0;
        uVar6 = 0;
        if (lVar15 == *local_1a8) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::geometry::instanced_wall::BrickTransformSsbo,alloc::alloc::Global>
                    ();
          uVar5 = local_158;
          uVar6 = uStack_150;
        }
        uStack_150 = uVar6;
        local_158 = uVar5;
        lVar26 = local_1a8[1];
        lVar18 = lVar15 * 0x60;
        auVar9._4_4_ = uStack_174;
        auVar9._0_4_ = uStack_178;
        auVar9._8_4_ = uStack_170;
        auVar9._12_4_ = uStack_16c;
        puVar3 = (undefined8 *)(lVar26 + 0x30 + lVar18);
        *puVar3 = local_158;
        puVar3[1] = uStack_150;
        puVar3 = (undefined8 *)(lVar26 + 0x20 + lVar18);
        *puVar3 = local_168;
        puVar3[1] = uStack_160;
        *(undefined1 (*) [16])(lVar26 + 0x10 + lVar18) = auVar9;
        *(undefined8 *)(lVar26 + lVar18) = CONCAT44(uStack_184,local_188);
        ((undefined8 *)(lVar26 + lVar18))[1] = CONCAT44(uStack_17c,uStack_180);
        *(float *)(lVar26 + 0x40 + lVar18) = fVar32;
        *(float *)(lVar26 + 0x44 + lVar18) = local_fc;
        *(float *)(lVar26 + 0x48 + lVar18) = fVar34 + fVar30;
        *(undefined4 *)(lVar26 + 0x4c + lVar18) = 0;
        *(undefined4 *)(lVar26 + 0x50 + lVar18) = extraout_EAX_01;
        *(float *)(lVar26 + 0x54 + lVar18) = local_1d0;
        *(float *)(lVar26 + 0x58 + lVar18) = local_1d4;
        *(undefined4 *)(lVar26 + 0x5c + lVar18) = 4;
        local_1a8[2] = lVar15 + 1;
      }
      lVar17 = (lVar17 - uVar20) + 1;
      wall = local_110;
    } while (lVar23 != 0);
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
LAB_141223390:
  bVar14 = false;
  local_1b8 = local_120;
  lVar23 = local_130;
  unaff_R13 = local_118;
  pfVar24 = local_138;
  goto LAB_141222860;
}

