
/* WARNING: Removing unreachable block (ram,0x000140edd445) */
/* WARNING: Removing unreachable block (ram,0x000140edd809) */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_clutter::inter_shape_stitches::detect_intra_shape_corners::
intersect_shapes<system_clutter::inter_roof_merlons::detect_inter_roof_merlons::detect_inter_roof_merlons::closure_env_4>
          (void)

{
  uint uVar1;
  longlong lVar2;
  longlong lVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  char extraout_AL;
  uint *in_RCX;
  byte *in_RDX;
  byte in_R8B;
  longlong lVar8;
  longlong *in_R9;
  float fVar9;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  undefined1 auVar14 [16];
  float fVar15;
  float fVar16;
  float fVar17;
  undefined8 local_258;
  float fStack_250;
  float local_24c;
  float fStack_248;
  float local_238;
  float fStack_234;
  float fStack_230;
  float fStack_22c;
  float fStack_228;
  float fStack_224;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 local_210;
  longlong local_208 [4];
  undefined8 local_1e8;
  longlong lStack_1e0;
  float local_1d8;
  float fStack_1d4;
  undefined8 uStack_1d0;
  float fStack_1c8;
  float fStack_1c4;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  longlong local_1a0;
  float *pfStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined4 local_160;
  float *local_140;
  undefined8 uStack_138;
  float local_130 [4];
  undefined8 local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  float local_108;
  float fStack_100;
  float fStack_fc;
  
  uVar1 = *in_RCX;
  if (uVar1 == 0) {
    if ((*in_RDX & 1) == 0) {
      fStack_230 = *(float *)(in_RDX + 0xc);
      local_238 = (float)*(undefined8 *)(in_RDX + 4);
      fStack_234 = (float)((ulonglong)*(undefined8 *)(in_RDX + 4) >> 0x20);
      fStack_22c = 0.0;
      ::utils::geometry::circle::Circle2d::intersect_circle();
      if (local_1d8 != 1.4013e-45) {
        return;
      }
      if (in_R8B == 0) {
        return;
      }
      lVar8 = in_R9[2];
      if (lVar8 == *in_R9) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar3 = in_R9[1];
      *(undefined4 *)(lVar3 + lVar8 * 0xc) = 0;
      *(ulonglong *)(lVar3 + 4 + lVar8 * 0xc) = CONCAT44((float)uStack_1d0,fStack_1d4);
      lVar3 = lVar8 + 1;
      in_R9[2] = lVar3;
      if (lVar3 == *in_R9) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                  ();
      }
      lVar2 = in_R9[1];
      *(undefined4 *)(lVar2 + lVar3 * 0xc) = 1;
      *(ulonglong *)(lVar2 + 4 + lVar3 * 0xc) = CONCAT44(fStack_1c8,uStack_1d0._4_4_);
      in_R9[2] = lVar8 + 2;
      return;
    }
  }
  else if ((*in_RDX & 1) != 0) {
    ::utils::geometry::rectangle::Rectangle2d::as_transformation_and_size();
    fVar7 = fStack_1c4;
    fVar16 = fStack_1c8;
    fVar15 = uStack_1d0._4_4_;
    fVar6 = (float)uStack_1d0;
    fVar5 = fStack_1d4;
    fVar4 = local_1d8;
    ::utils::geometry::rectangle::Rectangle2d::as_transformation_and_size();
    ::utils::geometry::aabb::Aabb2::from_center_dims();
    fVar9 = fVar15 * fVar4 - fVar6 * fVar5;
    auVar14._4_4_ = fVar9;
    auVar14._0_4_ = fVar9;
    auVar14._8_4_ = fVar9;
    auVar14._12_4_ = fVar9;
    auVar14 = divps(___xmm_3f800000bf800000bf8000003f800000,auVar14);
    fVar9 = fVar15 * auVar14._0_4_;
    fVar10 = fVar5 * auVar14._4_4_;
    fVar12 = fVar6 * auVar14._8_4_;
    fVar13 = fVar4 * auVar14._12_4_;
    local_238 = local_1d8 * fVar9 + fStack_1d4 * fVar12;
    fStack_234 = local_1d8 * fVar10 + fStack_1d4 * fVar13;
    fStack_230 = (float)uStack_1d0 * fVar9 + uStack_1d0._4_4_ * fVar12;
    fStack_22c = (float)uStack_1d0 * fVar10 + uStack_1d0._4_4_ * fVar13;
    fStack_228 = (fVar12 * fStack_1c4 + fVar9 * fStack_1c8) - (fVar7 * fVar12 + fVar16 * fVar9);
    fStack_224 = (fVar13 * fStack_1c4 + fVar10 * fStack_1c8) - (fVar7 * fVar13 + fVar16 * fVar10);
    ::utils::geometry::rectangle::Rectangle2d::as_points2_centered_aligned();
    uStack_118 = 0;
    local_110 = 4;
    pfStack_198 = &local_238;
    lStack_1e0 = 0;
    lVar8 = 0;
    do {
      local_208[lVar8] = *(longlong *)(local_130 + lVar8 * 2 + -2);
      local_208[lVar8 + 1] = *(longlong *)(local_130 + lVar8 * 2);
      lStack_1e0 = lStack_1e0 + 2;
      lVar8 = lVar8 + 2;
    } while (lVar8 != 4);
    local_1e8 = 0;
    local_1d8 = 5.60519e-45;
    fStack_1d4 = 0.0;
    fStack_1c8 = (float)local_208[0];
    fStack_1c4 = (float)((ulonglong)local_208[0] >> 0x20);
    local_1a8 = 0;
    uStack_190 = uStack_138;
    local_178 = local_120;
    uStack_170 = 0;
    local_168 = 4;
    local_160 = 0;
    fVar10 = local_130[0];
    local_1a0 = lStack_1e0;
    local_140 = pfStack_198;
    uStack_1d0 = pfStack_198;
    itertools::tuple_impl::impl$10::
    next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl_1::intersects::closure_env_0>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
              ();
    fVar9 = __real_3f800000;
    if ((int)local_258 != 1) {
      return;
    }
    do {
      fVar13 = local_24c - local_258._4_4_;
      fVar17 = fStack_248 - fStack_250;
      local_108 = local_258._4_4_;
      fStack_100 = fVar13;
      fStack_fc = fVar17;
      ::utils::geometry::aabb::Aabb2::intersect_ray();
      fVar12 = fVar10;
      ::utils::geometry::aabb::RayBoxIntersection::is_hit();
      if (extraout_AL != '\0') {
        if ((0.0 <= extraout_XMM0_Da) && (extraout_XMM0_Da <= fVar9)) {
          fVar12 = extraout_XMM0_Da * fVar13 + local_258._4_4_;
          fVar11 = extraout_XMM0_Da * fVar17 + fStack_250;
          if (in_R8B != 0) {
            lVar8 = in_R9[2];
            if (lVar8 == *in_R9) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                        ();
            }
            lVar3 = in_R9[1];
            *(undefined4 *)(lVar3 + lVar8 * 0xc) = 1;
            *(ulonglong *)(lVar3 + 4 + lVar8 * 0xc) =
                 CONCAT44(fVar11 * fVar15 + fVar12 * fVar5 + fVar7,
                          fVar11 * fVar6 + fVar12 * fVar4 + fVar16);
            in_R9[2] = lVar8 + 1;
          }
          ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
          ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
          fVar11 = extraout_XMM0_Da_00;
          ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
          fVar12 = extraout_XMM0_Da_01;
          ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
          fVar12 = fVar12 * fVar11 + extraout_XMM0_Da_03 * extraout_XMM0_Da_02;
          if (fVar12 < __real_3f666666) {
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
          }
        }
        if ((0.0 <= fVar10) && (fVar10 <= fVar9)) {
          fVar12 = local_258._4_4_ + fVar13 * fVar10;
          fVar10 = fStack_250 + fVar17 * fVar10;
          if (in_R8B != 0) {
            lVar8 = in_R9[2];
            if (lVar8 == *in_R9) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                        ();
            }
            lVar3 = in_R9[1];
            *(undefined4 *)(lVar3 + lVar8 * 0xc) = 0;
            *(ulonglong *)(lVar3 + 4 + lVar8 * 0xc) =
                 CONCAT44(fVar10 * fVar15 + fVar12 * fVar5 + fVar7,
                          fVar10 * fVar6 + fVar12 * fVar4 + fVar16);
            in_R9[2] = lVar8 + 1;
          }
          ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
          ::utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos();
          fVar12 = extraout_XMM0_Da_04;
          ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
          fVar10 = extraout_XMM0_Da_05;
          ::utils::geometry::rectangle::Rectangle2d::get_normal_at_u();
          fVar12 = fVar10 * fVar12 + extraout_XMM0_Da_07 * extraout_XMM0_Da_06;
          if (fVar12 < __real_3f666666) {
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
            ::utils::geometry::rectangle::Rectangle2d::get_point();
          }
        }
      }
      itertools::tuple_impl::impl$10::
      next<core::iter::adapters::map::Map<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,utils::geometry::rectangle::impl_1::intersects::closure_env_0>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                ();
      fVar10 = fVar12;
    } while ((local_258 & 1) != 0);
    return;
  }
  ::utils::geometry::rectangle::Rectangle2d::as_points2();
  local_218 = 0;
  local_210 = 4;
  uStack_188 = uStack_220;
  uStack_180 = 0;
  local_178 = 4;
  uStack_1d0._0_4_ = local_238;
  uStack_1d0._4_4_ = fStack_234;
  fStack_1c8 = fStack_230;
  fStack_1c4 = fStack_22c;
  local_1b8 = uStack_220;
  uStack_1b0 = 0;
  local_1a8 = 4;
  uStack_170 = (ulonglong)uStack_170._4_4_ << 0x20;
  local_1d8 = 5.60519e-45;
  fStack_1d4 = 0.0;
  itertools::tuple_impl::impl$10::
  next<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
            ();
  fVar4 = __real_3f800000;
  if (local_238 == 1.4013e-45) {
    do {
      fVar6 = fStack_230;
      fVar5 = fStack_234;
      fVar15 = fStack_22c - fStack_234;
      fVar16 = fStack_228 - fStack_230;
      ::utils::geometry::circle::Circle2d::intersect_ray();
      if ((local_258 & 1) != 0) {
        if ((-(local_258._4_4_ <= fVar4 && 0.0 <= local_258._4_4_) & in_R8B) == 1) {
          lVar8 = in_R9[2];
          if (lVar8 == *in_R9) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                      ();
          }
          lVar3 = in_R9[1];
          *(uint *)(lVar3 + lVar8 * 0xc) = uVar1 ^ 1;
          *(ulonglong *)(lVar3 + 4 + lVar8 * 0xc) =
               CONCAT44(local_258._4_4_ * fVar16 + fVar6,local_258._4_4_ * fVar15 + fVar5);
          in_R9[2] = lVar8 + 1;
        }
        if ((0.0 <= fStack_250) && (fStack_250 <= fVar4)) {
          if (in_R8B != 0) {
            lVar8 = in_R9[2];
            if (lVar8 == *in_R9) {
              alloc::raw_vec::RawVec::
              grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,u8>,alloc::alloc::Global>
                        ();
            }
            lVar3 = in_R9[1];
            *(uint *)(lVar3 + lVar8 * 0xc) = uVar1;
            *(ulonglong *)(lVar3 + 4 + lVar8 * 0xc) =
                 CONCAT44(fVar6 + fVar16 * fStack_250,fVar5 + fVar15 * fStack_250);
            in_R9[2] = lVar8 + 1;
          }
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
        }
      }
      itertools::tuple_impl::impl$10::
      next<core::array::iter::IntoIter<glam::f32::vec2::Vec2,4>,tuple_<glam::f32::vec2::Vec2,glam::f32::vec2::Vec2>_>
                ();
    } while (((uint)local_238 & 1) != 0);
  }
  return;
}

