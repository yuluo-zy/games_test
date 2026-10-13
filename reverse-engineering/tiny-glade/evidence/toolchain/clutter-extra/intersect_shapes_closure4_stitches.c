
/* WARNING: Removing unreachable block (ram,0x000140edc175) */
/* WARNING: Removing unreachable block (ram,0x000140edc8d9) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_clutter::inter_shape_stitches::detect_intra_shape_corners::
intersect_shapes<system_clutter::inter_shape_stitches::detect_intra_shape_corners::detect_intra_shape_corners::closure_env_4>
          (void)

{
  undefined8 uVar1;
  uint uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  uint uVar7;
  char extraout_AL;
  uint *in_RCX;
  uint *in_RDX;
  longlong lVar8;
  char in_R9B;
  float fVar9;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float fVar10;
  float fVar11;
  float fVar12;
  undefined1 extraout_XMM0 [16];
  float fVar13;
  undefined1 auVar14 [16];
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  undefined1 auVar20 [16];
  float fVar21;
  float fVar22;
  undefined1 auVar23 [16];
  longlong *in_stack_00000028;
  float local_2b8;
  float fStack_2b4;
  float fStack_2b0;
  float fStack_2ac;
  float fStack_2a8;
  float local_268;
  float fStack_264;
  float fStack_260;
  float fStack_25c;
  float fStack_258;
  float fStack_254;
  float local_248;
  float fStack_244;
  undefined8 uStack_240;
  float local_238;
  float fStack_234;
  undefined8 uStack_230;
  ulonglong local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  longlong local_210;
  float *pfStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined4 local_1d0;
  uint local_1ac;
  undefined8 local_1a8;
  undefined8 local_1a0;
  longlong local_198 [4];
  undefined8 local_178;
  longlong lStack_170;
  ulonglong local_168;
  undefined8 uStack_160;
  float local_158;
  float fStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  float *local_140;
  undefined8 uStack_138;
  float local_130 [4];
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 uStack_100;
  
  uVar2 = *in_RCX;
  if (uVar2 == 0) {
    if ((*in_RDX & 1) == 0) {
      fStack_260 = (float)in_RCX[3];
      local_268 = (float)*(undefined8 *)(in_RCX + 1);
      fVar22 = local_268;
      fStack_264 = (float)((ulonglong)*(undefined8 *)(in_RCX + 1) >> 0x20);
      fVar17 = fStack_264;
      fStack_25c = 0.0;
      uStack_240._0_4_ = (float)in_RDX[3];
      local_248 = (float)*(undefined8 *)(in_RDX + 1);
      fVar18 = local_248;
      fStack_244 = (float)((ulonglong)*(undefined8 *)(in_RDX + 1) >> 0x20);
      fVar15 = fStack_244;
      uStack_240._4_4_ = 0.0;
      ::utils::geometry::circle::Circle2d::intersect_circle();
      fVar21 = __real_3f800000;
      uVar2 = ___xmm_80000000800000008000000080000000;
      if (local_2b8 != 1.4013e-45) {
        return;
      }
      fVar9 = fVar18 - fStack_2b4;
      uStack_240._0_4_ = fVar15 - fStack_2b0;
      fVar19 = fStack_2b4 - fVar22;
      local_248 = fStack_2b0 - fVar17;
      fStack_244 = __real_3f800000 / SQRT(local_248 * local_248 + fVar19 * fVar19);
      uStack_240._4_4_ =
           __real_3f800000 / SQRT((float)uStack_240 * (float)uStack_240 + fVar9 * fVar9);
      local_248 = (float)((uint)fStack_244 ^ ___xmm_80000000800000008000000080000000) * local_248;
      fStack_244 = fStack_244 * fVar19;
      uStack_240._0_4_ =
           (float)((uint)uStack_240._4_4_ ^ ___xmm_80000000800000008000000080000000) *
           (float)uStack_240;
      uStack_240._4_4_ = uStack_240._4_4_ * fVar9;
      uStack_230 = 0;
      local_228 = local_228 & 0xffffffff00000000;
      detect_intra_shape_corners::closure_4();
      fVar17 = fVar17 - fStack_2a8;
      fVar22 = fVar22 - fStack_2ac;
      fVar15 = fStack_2a8 - fVar15;
      fVar18 = fStack_2ac - fVar18;
      fStack_244 = fVar21 / SQRT(fVar22 * fVar22 + fVar17 * fVar17);
      fVar21 = fVar21 / SQRT(fVar18 * fVar18 + fVar15 * fVar15);
      local_248 = (float)((uint)fStack_244 ^ uVar2) * fVar17;
      fStack_244 = fStack_244 * fVar22;
      uStack_240._0_4_ = (float)(uVar2 ^ (uint)fVar21) * fVar15;
      uStack_240._4_4_ = fVar21 * fVar18;
      uStack_230 = 1;
      local_228 = local_228 & 0xffffffff00000000;
      detect_intra_shape_corners::closure_4();
      if (in_R9B == '\0') {
        return;
      }
      lVar8 = in_stack_00000028[2];
      if (lVar8 == *in_stack_00000028) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar4 = in_stack_00000028[1];
      *(undefined4 *)(lVar4 + lVar8 * 0xc) = 0;
      *(ulonglong *)(lVar4 + 4 + lVar8 * 0xc) = CONCAT44(fStack_2b0,fStack_2b4);
      lVar4 = lVar8 + 1;
      in_stack_00000028[2] = lVar4;
      if (lVar4 == *in_stack_00000028) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar3 = in_stack_00000028[1];
      *(undefined4 *)(lVar3 + lVar4 * 0xc) = 1;
      *(ulonglong *)(lVar3 + 4 + lVar4 * 0xc) = CONCAT44(fStack_2a8,fStack_2ac);
      in_stack_00000028[2] = lVar8 + 2;
      return;
    }
  }
  else {
    in_RCX = in_RDX;
    if ((*in_RDX & 1) != 0) {
      ::utils::geometry::rectangle::Rectangle2d::as_transformation_and_size();
      fVar19 = fStack_234;
      fVar15 = local_238;
      fVar18 = uStack_240._4_4_;
      fVar17 = (float)uStack_240;
      fVar22 = fStack_244;
      fVar21 = local_248;
      ::utils::geometry::rectangle::Rectangle2d::as_transformation_and_size();
      local_168 = local_228;
      uStack_160 = 0;
      ::utils::geometry::aabb::Aabb2::from_center_dims();
      auVar14._0_4_ = fVar18 * fVar21 - fVar17 * fVar22;
      auVar14._4_4_ = auVar14._0_4_;
      auVar14._8_4_ = auVar14._0_4_;
      auVar14._12_4_ = auVar14._0_4_;
      auVar14 = divps(___xmm_3f800000bf800000bf8000003f800000,auVar14);
      fVar9 = fVar18 * auVar14._0_4_;
      fVar10 = fVar22 * auVar14._4_4_;
      fVar11 = fVar17 * auVar14._8_4_;
      fVar12 = fVar21 * auVar14._12_4_;
      local_158 = fVar15;
      fStack_154 = fVar19;
      uStack_150 = 0;
      uStack_14c = 0;
      local_268 = local_248 * fVar9 + fStack_244 * fVar11;
      fStack_264 = local_248 * fVar10 + fStack_244 * fVar12;
      fStack_260 = (float)uStack_240 * fVar9 + uStack_240._4_4_ * fVar11;
      fStack_25c = (float)uStack_240 * fVar10 + uStack_240._4_4_ * fVar12;
      fStack_258 = (fVar11 * fStack_234 + fVar9 * local_238) - (fVar19 * fVar11 + fVar15 * fVar9);
      fStack_254 = (fVar12 * fStack_234 + fVar10 * local_238) - (fVar19 * fVar12 + fVar15 * fVar10);
      ::utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned();
      uStack_118 = 0;
      local_110 = 4;
      pfStack_208 = &local_268;
      lStack_170 = 0;
      lVar8 = 0;
      do {
        local_198[lVar8] = *(longlong *)(local_130 + lVar8 * 2 + -2);
        local_198[lVar8 + 1] = *(longlong *)(local_130 + lVar8 * 2);
        local_210 = lStack_170 + 2;
        lVar8 = lVar8 + 2;
        lStack_170 = local_210;
      } while (lVar8 != 4);
      local_178 = 0;
      local_248 = 5.60519e-45;
      fStack_244 = 0.0;
      local_238 = (float)local_198[0];
      fStack_234 = (float)((ulonglong)local_198[0] >> 0x20);
      local_218 = 0;
      uStack_200 = uStack_138;
      local_1e8 = local_120;
      uStack_1e0 = 0;
      local_1d8 = 4;
      local_1d0 = 0;
      fVar15 = local_130[0];
      local_140 = pfStack_208;
      uStack_240 = pfStack_208;
      itertools::tuple_impl::impl$10::
      next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl_1::intersects::closure_env_0>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                ();
      if (local_1ac != 1) {
        return;
      }
      do {
        fVar11 = (float)local_1a8;
        fVar9 = (float)local_1a0 - fVar11;
        fVar12 = (float)((ulonglong)local_1a8 >> 0x20);
        fVar10 = (float)((ulonglong)local_1a0 >> 0x20) - fVar12;
        uStack_100 = CONCAT44(fVar10,fVar9);
        local_108 = local_1a8;
        fVar19 = fVar15;
        ::utils::geometry::aabb::Aabb2::intersect_ray();
        fVar15 = fVar19;
        ::utils::geometry::aabb::RayBoxIntersection::is_hit();
        if (extraout_AL != '\0') {
          fVar15 = fVar10 * fVar17 + fVar9 * fVar21;
          fVar16 = fVar10 * fVar18 + fVar9 * fVar22;
          fVar15 = SQRT(fVar16 * fVar16 + fVar15 * fVar15);
          if ((__real_00000000 <= extraout_XMM0_Da) &&
             (fVar15 = __real_3f800000, extraout_XMM0_Da <= __real_3f800000)) {
            auVar20._0_4_ = extraout_XMM0_Da * fVar9 + fVar11;
            auVar20._4_4_ = extraout_XMM0_Da * fVar10 + fVar12;
            auVar20._8_8_ = 0;
            fVar16 = auVar20._4_4_ * fVar17 + auVar20._0_4_ * fVar21 + local_158;
            fVar15 = auVar20._4_4_ * fVar18 + auVar20._0_4_ * fVar22 + fStack_154;
            if (in_R9B != '\0') {
              lVar8 = in_stack_00000028[2];
              if (lVar8 == *in_stack_00000028) {
                alloc::raw_vec::RawVec::
                grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                          ();
              }
              lVar4 = in_stack_00000028[1];
              *(undefined4 *)(lVar4 + lVar8 * 0xc) = 1;
              *(ulonglong *)(lVar4 + 4 + lVar8 * 0xc) = CONCAT44(fVar15,fVar16);
              in_stack_00000028[2] = lVar8 + 1;
            }
            auVar5._8_8_ = uStack_160;
            auVar5._0_8_ = local_168;
            divps(auVar20,auVar5);
            ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
            ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
            fVar13 = extraout_XMM0_Da_00;
            ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
            fVar15 = extraout_XMM0_Da_01;
            ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
            fVar15 = fVar15 * fVar13 + extraout_XMM0_Da_03 * extraout_XMM0_Da_02;
            if (fVar15 < __real_3f666666) {
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              fVar15 = fVar16;
              detect_intra_shape_corners::closure_4();
            }
          }
          if ((__real_00000000 <= fVar19) && (fVar19 <= __real_3f800000)) {
            auVar23._0_4_ = fVar11 + fVar9 * fVar19;
            auVar23._4_4_ = fVar12 + fVar10 * fVar19;
            auVar23._8_8_ = 0;
            fVar19 = auVar23._4_4_ * fVar17 + auVar23._0_4_ * fVar21 + local_158;
            fVar15 = auVar23._4_4_ * fVar18 + auVar23._0_4_ * fVar22 + fStack_154;
            if (in_R9B != '\0') {
              lVar8 = in_stack_00000028[2];
              if (lVar8 == *in_stack_00000028) {
                alloc::raw_vec::RawVec::
                grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                          ();
              }
              lVar4 = in_stack_00000028[1];
              *(undefined4 *)(lVar4 + lVar8 * 0xc) = 0;
              *(ulonglong *)(lVar4 + 4 + lVar8 * 0xc) = CONCAT44(fVar15,fVar19);
              in_stack_00000028[2] = lVar8 + 1;
            }
            auVar6._8_8_ = uStack_160;
            auVar6._0_8_ = local_168;
            divps(auVar23,auVar6);
            ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
            ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
            fVar9 = extraout_XMM0_Da_04;
            ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
            fVar15 = extraout_XMM0_Da_05;
            ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
            fVar15 = fVar15 * fVar9 + extraout_XMM0_Da_06 * extraout_XMM0._0_4_;
            if (fVar15 < __real_3f666666) {
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              ::utils::geometry::rectangle::Rectangle2d::get_point();
              fVar15 = fVar19;
              detect_intra_shape_corners::closure_4();
            }
          }
        }
        itertools::tuple_impl::impl$10::
        next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl_1::intersects::closure_env_0>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                  ();
      } while ((local_1ac & 1) != 0);
      return;
    }
  }
  ::utils::geometry::rectangle::Rectangle2d::as_points2();
  uStack_1f0 = 0;
  local_1e8 = 4;
  uStack_220 = 0;
  local_218 = 4;
  uStack_1e0 = (ulonglong)uStack_1e0._4_4_ << 0x20;
  local_248 = 5.60519e-45;
  fStack_244 = 0.0;
  itertools::tuple_impl::impl$10::
  next<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
            ();
  fVar21 = __real_3f800000;
  uVar7 = ___xmm_80000000800000008000000080000000;
  if (local_268 == 1.4013e-45) {
    do {
      fVar18 = fStack_260;
      fVar17 = fStack_264;
      fVar22 = fStack_2b0;
      fVar15 = fStack_25c - fStack_264;
      fVar19 = fStack_258 - fStack_260;
      ::utils::geometry::circle::Circle2d::intersect_ray();
      if (((uint)local_2b8 & 1) != 0) {
        fVar9 = (fVar21 / SQRT(fVar19 * fVar19 + fVar15 * fVar15)) * fVar15;
        if ((0.0 <= fStack_2b4) && (fStack_2b4 <= fVar21)) {
          fVar10 = fStack_2b4 * fVar15 + fVar17;
          fVar11 = fStack_2b4 * fVar19 + fVar18;
          if (in_R9B != '\0') {
            lVar8 = in_stack_00000028[2];
            if (lVar8 == *in_stack_00000028) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                        ();
            }
            lVar4 = in_stack_00000028[1];
            *(uint *)(lVar4 + lVar8 * 0xc) = uVar2 ^ 1;
            *(ulonglong *)(lVar4 + 4 + lVar8 * 0xc) = CONCAT44(fVar11,fVar10);
            in_stack_00000028[2] = lVar8 + 1;
          }
          uVar1 = *(undefined8 *)(in_RCX + 1);
          fVar10 = (float)uVar1 - fVar10;
          fVar11 = (float)((ulonglong)uVar1 >> 0x20) - fVar11;
          fVar12 = fVar21 / SQRT(fVar11 * fVar11 + fVar10 * fVar10);
          local_2b8 = fVar11 * (float)((uint)fVar12 ^ uVar7);
          fStack_2b4 = fVar10 * fVar12;
          fStack_2b0 = (float)((uint)fVar9 ^ uVar7);
          detect_intra_shape_corners::closure_4();
        }
        if ((0.0 <= fVar22) && (fVar22 <= fVar21)) {
          fVar17 = fVar17 + fVar15 * fVar22;
          fVar18 = fVar18 + fVar19 * fVar22;
          if (in_R9B != '\0') {
            lVar8 = in_stack_00000028[2];
            if (lVar8 == *in_stack_00000028) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                        ();
            }
            lVar4 = in_stack_00000028[1];
            *(uint *)(lVar4 + lVar8 * 0xc) = uVar2;
            *(ulonglong *)(lVar4 + 4 + lVar8 * 0xc) = CONCAT44(fVar18,fVar17);
            in_stack_00000028[2] = lVar8 + 1;
          }
          uVar1 = *(undefined8 *)(in_RCX + 1);
          fVar17 = fVar17 - (float)uVar1;
          fVar18 = fVar18 - (float)((ulonglong)uVar1 >> 0x20);
          fVar22 = fVar21 / SQRT(fVar18 * fVar18 + fVar17 * fVar17);
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          local_2b8 = fVar18 * (float)((uint)fVar22 ^ uVar7);
          fStack_2b4 = fVar17 * fVar22;
          detect_intra_shape_corners::closure_4();
          fStack_2b0 = fVar9;
        }
      }
      itertools::tuple_impl::impl$10::
      next<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                ();
    } while (((uint)local_268 & 1) != 0);
  }
  return;
}

