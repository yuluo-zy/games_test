// name: add_skirts
// addr: 0x1412221f0
// size: 5146
// signature: void __cdecl add_skirts(void)


// WARNING: Removing unreachable block (ram,0x000141223378)
// WARNING: Removing unreachable block (ram,0x0001412226a1)
// WARNING: Removing unreachable block (ram,0x00014122274c)
// WARNING: Removing unreachable block (ram,0x000141222782)
// WARNING: Removing unreachable block (ram,0x0001412226c5)
// WARNING: Removing unreachable block (ram,0x0001412226e0)
// WARNING: Removing unreachable block (ram,0x0001412226f0)
// WARNING: Removing unreachable block (ram,0x0001412235ae)
// WARNING: Removing unreachable block (ram,0x0001412226fa)
// WARNING: Removing unreachable block (ram,0x000141222731)

void __cdecl system_wall_constructor::utils::wall_skirts::add_skirts(void)

{
  float *pfVar1;
  longlong lVar2;
  undefined8 *puVar3;
  float fVar4;
  float fVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  longlong lVar10;
  ulonglong uVar11;
  float *pfVar12;
  undefined1 auVar13 [16];
  undefined4 uVar14;
  undefined4 uVar15;
  bool bVar16;
  char extraout_AL;
  uint extraout_EAX;
  uint extraout_EAX_00;
  undefined4 extraout_EAX_01;
  float *extraout_RAX;
  float *extraout_RAX_00;
  float *extraout_RAX_01;
  ulonglong uVar17;
  longlong lVar18;
  ulonglong uVar19;
  undefined8 *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong in_RCX;
  longlong lVar20;
  longlong lVar21;
  float *pfVar22;
  float *pfVar23;
  longlong *in_R8;
  longlong in_R9;
  float *unaff_R13;
  float *pfVar24;
  ulonglong uVar25;
  uint uVar26;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float fVar27;
  float fVar28;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float extraout_XMM0_Da_11;
  float extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  undefined4 extraout_XMM0_Da_14;
  float fVar29;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  longlong in_stack_00000028;
  longlong in_stack_00000038;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined1 local_288 [8];
  float fStack_280;
  float fStack_27c;
  undefined1 local_278 [12];
  undefined4 uStack_26c;
  longlong local_218;
  longlong local_210;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  longlong local_198;
  int local_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  float *local_120;
  float *local_108;
  
  country_core::resources::walls::public_wall_state::PublicWallState::max_y();
  country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle();
  if (local_188 == 1) {
    local_1f8 = CONCAT44(uStack_170,uStack_174);
    uStack_200 = CONCAT44(uStack_178,uStack_17c);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (float *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    local_198 = 4;
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    *extraout_RAX = extraout_XMM0_Da;
    extraout_RAX[1] = extraout_XMM0_Da_00;
    extraout_RAX[2] = extraout_XMM0_Da_01;
    extraout_RAX[3] = extraout_XMM0_Da_02;
    extraout_RAX[4] = extraout_XMM0_Da_03;
    extraout_RAX[5] = extraout_XMM0_Da_04;
    extraout_RAX[6] = extraout_XMM0_Da_05;
    extraout_RAX[7] = extraout_XMM0_Da_06;
    local_108 = extraout_RAX;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (float *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    extraout_RAX_00[0] = 0.0;
    extraout_RAX_00[1] = 1.0;
    local_198 = 1;
    local_108 = extraout_RAX_00;
  }
  country_core::resources::walls::split_height_into_skirts();
  pfVar22 = (float *)CONCAT44(uStack_17c,uStack_180);
  pfVar1 = pfVar22 + CONCAT44(uStack_174,uStack_178);
  fVar4 = *(float *)(in_R9 + 0x30);
  lVar10 = *(longlong *)(in_R9 + 8);
  uVar11 = *(ulonglong *)(in_R9 + 0x10);
  lVar2 = lVar10 + uVar11 * 8;
  pfVar12 = *(float **)(in_R9 + 0x20);
  do {
    do {
      if (pfVar22 == pfVar1) {
        if (CONCAT44(uStack_184,local_188) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
        return;
      }
      fVar5 = *pfVar22;
      pfVar22 = pfVar22 + 1;
    } while (fVar5 < *(float *)(in_RCX + 0x280) + 3.5);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (float *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = 0.0;
    for (pfVar23 = local_108; pfVar23 != local_108 + local_198 * 2; pfVar23 = pfVar23 + 2) {
      ceilf(((pfVar23[1] - *pfVar23) * fVar4) / 0.69);
      ::utils::random_splits_into();
    }
    if (uVar11 == 1) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if (uVar11 == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar27 = pfVar12[1] - *pfVar12;
    fVar28 = ABS(fVar27);
    if (NAN(fVar27)) {
      uVar25 = 0;
      fVar27 = 1.0;
      if (uVar11 != 2) {
        fVar27 = ABS(pfVar12[2] - pfVar12[1]);
        fVar28 = fVar28 + fVar27;
        if (0.0 <= fVar28) {
          uVar25 = 1;
        }
        else {
          lVar20 = uVar11 * 8 + -0x10;
          lVar18 = 3;
          fVar29 = pfVar12[2];
          do {
            lVar21 = lVar18;
            lVar20 = lVar20 + -8;
            if (lVar20 == 0) {
              fVar27 = 1.0;
              uVar25 = (ulonglong)((int)(uVar11 * 8 + 0x7ffffffe8 >> 3) + 1);
              goto LAB_1412227f8;
            }
            fVar27 = ABS(pfVar12[lVar21] - fVar29);
            fVar28 = fVar28 + fVar27;
            lVar18 = lVar21 + 1;
            fVar29 = pfVar12[lVar21];
          } while (fVar28 < 0.0);
          uVar25 = lVar21 - 1;
        }
        goto LAB_1412227e5;
      }
    }
    else {
      uVar25 = 0;
      fVar27 = fVar28;
LAB_1412227e5:
      fVar27 = 1.0 - fVar28 / fVar27;
    }
LAB_1412227f8:
    uVar17 = uVar25 & 0xffffffff;
    _local_278 = ZEXT416((uint)fVar27);
    local_288._0_4_ =
         fVar27 * *(float *)(in_stack_00000038 + 4 + uVar17 * 4) +
         *(float *)(in_stack_00000038 + uVar17 * 4) * (1.0 - fVar27);
    local_288._4_4_ =
         fVar27 * *(float *)(in_stack_00000028 + 4 + uVar17 * 4) +
         *(float *)(in_stack_00000028 + uVar17 * 4) * (1.0 - fVar27);
    register0x00001248 = 0;
    bVar16 = true;
    local_120 = extraout_RAX_01;
LAB_141222860:
    while( true ) {
      uVar14 = local_288._0_4_;
      uVar15 = local_288._4_4_;
      if (local_120 == extraout_RAX_01 + 1) break;
      pfVar23 = local_120 + 1;
      pfVar24 = local_120;
      if (bVar16) {
        if (pfVar23 == extraout_RAX_01 + 1) break;
        unaff_R13 = local_120;
        pfVar24 = pfVar23;
        pfVar23 = local_120 + 2;
      }
      local_120 = pfVar23;
      fVar27 = *unaff_R13;
      fVar28 = (*pfVar24 - fVar27) * 0.5;
      if (fVar28 < 0.0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      uVar17 = uVar25 & 0xffffffff;
      if ((int)uVar25 == 0) {
        lVar20 = lVar10;
        if (lVar10 + 8 == lVar2) goto LAB_1412229c0;
LAB_141222912:
        fVar30 = ABS(pfVar12[uVar17 + 1] - pfVar12[uVar17]);
        fVar29 = (1.0 - (float)local_278._0_4_) * fVar30 - fVar28;
        if (0.0 <= fVar29) {
LAB_141222988:
          local_278._8_4_ = 0.0;
          uStack_26c = 0;
          local_278._0_4_ = 1.0 - fVar29 / fVar30;
        }
        else {
          lVar20 = lVar20 + 0x10;
          if (lVar20 != lVar2) {
            lVar18 = uVar17 + 2;
            fVar30 = pfVar12[uVar17 + 1];
            do {
              pfVar23 = pfVar12 + lVar18;
              fVar30 = ABS(*pfVar23 - fVar30);
              fVar29 = fVar29 + fVar30;
              if (0.0 <= fVar29) {
                uVar25 = (ulonglong)((int)uVar25 + 1);
                goto LAB_141222988;
              }
              lVar20 = lVar20 + 8;
              uVar25 = (ulonglong)((int)uVar25 + 1);
              lVar18 = lVar18 + 1;
              fVar30 = *pfVar23;
            } while (lVar20 != lVar2);
          }
          local_278._0_4_ = 1.0;
          local_278._8_4_ = 0.0;
          uStack_26c = 0;
        }
      }
      else {
        if ((uVar17 < uVar11) && (lVar20 = lVar10 + uVar17 * 8, lVar20 + 8 != lVar2))
        goto LAB_141222912;
LAB_1412229c0:
      }
      uVar17 = uVar25 & 0xffffffff;
      fVar30 = 1.0 - (float)local_278._0_4_;
      lVar20 = lVar10;
      fVar29 = (float)local_278._0_4_;
      if ((int)uVar25 == 0) {
joined_r0x000141222a94:
        if (lVar20 + 8 != lVar2) {
          fVar32 = ABS(pfVar12[uVar17 + 1] - pfVar12[uVar17]);
          fVar29 = fVar30 * fVar32 - fVar28;
          if (0.0 <= fVar29) {
LAB_141222a6a:
            local_278._8_4_ = 0.0;
            uStack_26c = 0;
            fVar29 = 1.0 - fVar29 / fVar32;
          }
          else {
            lVar20 = lVar20 + 0x10;
            if (lVar20 != lVar2) {
              lVar18 = uVar17 + 2;
              fVar32 = pfVar12[uVar17 + 1];
              do {
                pfVar23 = pfVar12 + lVar18;
                fVar32 = ABS(*pfVar23 - fVar32);
                fVar29 = fVar29 + fVar32;
                if (0.0 <= fVar29) {
                  uVar25 = (ulonglong)((int)uVar25 + 1);
                  goto LAB_141222a6a;
                }
                lVar20 = lVar20 + 8;
                uVar25 = (ulonglong)((int)uVar25 + 1);
                lVar18 = lVar18 + 1;
                fVar32 = *pfVar23;
              } while (lVar20 != lVar2);
            }
            local_278._8_4_ = 0.0;
            uStack_26c = 0;
            fVar29 = 1.0;
          }
        }
      }
      else if (uVar17 < uVar11) {
        lVar20 = lVar10 + uVar17 * 8;
        goto joined_r0x000141222a94;
      }
      fVar31 = *(float *)(in_stack_00000038 + 4 + uVar17 * 4) * (float)local_278._0_4_;
      fVar32 = (float)local_278._0_4_ * *(float *)(in_stack_00000028 + 4 + uVar17 * 4) +
               fVar30 * *(float *)(in_stack_00000028 + uVar17 * 4);
      uVar19 = uVar25 & 0xffffffff;
      local_278._4_4_ = 0;
      local_278._0_4_ = fVar29;
      fVar33 = fVar29 * *(float *)(in_stack_00000038 + 4 + uVar19 * 4) +
               *(float *)(in_stack_00000038 + uVar19 * 4) * (1.0 - fVar29);
      local_288._4_4_ =
           fVar29 * *(float *)(in_stack_00000028 + 4 + uVar19 * 4) +
           *(float *)(in_stack_00000028 + uVar19 * 4) * (1.0 - fVar29);
      local_288._0_4_ = fVar33;
      fStack_280 = (float)local_278._8_4_ * 0.0 + 0.0;
      fStack_27c = (float)local_278._8_4_ * 0.0 + 0.0;
      fVar29 = (float)local_288._4_4_;
      if ((float)uVar15 <= (float)local_288._4_4_) {
        fVar29 = (float)uVar15;
      }
      fVar29 = (float)(~-(uint)NAN((float)uVar15) & (uint)fVar29 |
                      -(uint)NAN((float)uVar15) & local_288._4_4_);
      unaff_R13 = pfVar24;
      if (1 < *(uint *)(in_RCX + 600)) goto LAB_141222b70;
      bVar16 = false;
      if (fVar29 <= fVar5) goto LAB_141222ba2;
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
  } while( true );
LAB_141222b70:
  bVar16 = false;
  if ((fVar29 <= fVar5) &&
     (fVar29 = fVar5 - fVar32,
     fVar29 < *(float *)(in_RCX + 0x284) || fVar29 == *(float *)(in_RCX + 0x284))) {
LAB_141222ba2:
    fVar31 = *(float *)(in_stack_00000038 + uVar17 * 4) * fVar30 + fVar31;
    fVar29 = (float)uVar14 + fVar31 + fVar33;
    if (fVar29 <= 0.0) {
      fVar33 = -100.0;
      fVar31 = -100.0;
    }
    uVar26 = -(uint)(0.0 < fVar29);
    fVar29 = (float)uVar15 + (float)(~uVar26 & 0xc2c80000 | uVar26 & uVar14);
    fVar32 = fVar32 + fVar31;
    if ((((float)local_288._4_4_ + fVar33 <= fVar5) || (fVar29 <= fVar5)) ||
       (bVar16 = false, fVar32 <= fVar5)) {
      fVar27 = (fVar27 + *pfVar24) * 0.5;
      ::utils::geometry::aabb::Aabb2::from_center_dims();
      lVar20 = 4;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *(undefined1 (*) [16])(extraout_RAX_02 + 2) = ZEXT816(0);
      *extraout_RAX_02 = local_358;
      extraout_RAX_02[1] = uStack_350;
      *(undefined4 *)(extraout_RAX_02 + 4) = 0;
      *(undefined2 *)((longlong)extraout_RAX_02 + 0x24) = 0;
      local_218 = 0;
      local_210 = 1;
      country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates();
      for (lVar18 = extraout_RAX_03; (lVar18 != lVar20 && (lVar18 != 0)); lVar18 = lVar18 + 0x14) {
        ::utils::geometry::aabb::Aabb2::width();
        ::utils::geometry::aabb::Aabb2::height();
        ::utils::geometry::aabb::Aabb2::center();
        ::utils::geometry::aabb::Aabb2::from_center_dims();
        ::utils::geometry::aabb::Aabb2::intersects();
        if (extraout_AL != '\0') {
          resolve_hole_overlap::resolve_hole_overlap();
        }
        if (*(int *)(in_RCX + 600) != 2) {
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::width();
          if (extraout_XMM0_Da_07 + extraout_XMM0_Da_08 * 0.5 <= fVar4) {
            ::utils::geometry::aabb::Aabb2::center();
            ::utils::geometry::aabb::Aabb2::width();
            if (extraout_XMM0_Da_09 < extraout_XMM0_Da_10 * 0.5) {
              ::utils::geometry::aabb::Aabb2::center();
              ::utils::geometry::aabb::Aabb2::center();
              ::utils::geometry::aabb::Aabb2::from_center_dims();
              resolve_hole_overlap::resolve_hole_overlap();
            }
          }
          else {
            ::utils::geometry::aabb::Aabb2::center();
            ::utils::geometry::aabb::Aabb2::center();
            ::utils::geometry::aabb::Aabb2::from_center_dims();
            resolve_hole_overlap::resolve_hole_overlap();
          }
        }
      }
      do {
        local_210 = local_210 + -1;
        if (*(char *)((longlong)extraout_RAX_02 + local_218 * 0x28 + 0x25) == '\x02') break;
        ::utils::geometry::aabb::Aabb2::height();
        if (0.1 <= extraout_XMM0_Da_11) {
          ::utils::curve::Curve::
          get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                    ();
          uVar6 = *(undefined8 *)(lVar10 + (ulonglong)extraout_EAX * 8);
          uVar7 = *(undefined8 *)(lVar10 + 8 + (ulonglong)extraout_EAX * 8);
          ::utils::curve::Curve::
          get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                    ();
          uVar8 = *(undefined8 *)(lVar10 + (ulonglong)extraout_EAX_00 * 8);
          uVar9 = *(undefined8 *)(lVar10 + 8 + (ulonglong)extraout_EAX_00 * 8);
          fVar30 = (extraout_XMM0_Da_13 * (float)uVar9 + (1.0 - extraout_XMM0_Da_13) * (float)uVar8)
                   - (extraout_XMM0_Da_12 * (float)uVar7 +
                     (1.0 - extraout_XMM0_Da_12) * (float)uVar6);
          fVar31 = (extraout_XMM0_Da_13 * (float)((ulonglong)uVar9 >> 0x20) +
                   (1.0 - extraout_XMM0_Da_13) * (float)((ulonglong)uVar8 >> 0x20)) -
                   (extraout_XMM0_Da_12 * (float)((ulonglong)uVar7 >> 0x20) +
                   (1.0 - extraout_XMM0_Da_12) * (float)((ulonglong)uVar6 >> 0x20));
          ::utils::swizzlers::impl$0::x0y();
          fVar30 = -((1.0 / SQRT(fVar31 * fVar31 + fVar30 * fVar30)) * fVar31);
          ::utils::swizzlers::impl$0::x0y();
          ::utils::geometry::aabb::Aabb2::width();
          ::utils::geometry::aabb::Aabb2::height();
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::center();
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          ::utils::wall_space::WallSpaceCoord::to_world_space();
          country_core::resources::render::impl$1::from();
          country_core::geometry::instanced_wall::impl$1::from();
          uStack_170 = (undefined4)uStack_1f0;
          uStack_16c = (undefined4)((ulonglong)uStack_1f0 >> 0x20);
          lVar20 = in_R8[2];
          if (lVar20 == *in_R8) {
            alloc::raw_vec::RawVec::
            grow_one<country_core::geometry::instanced_wall::BrickTransformSsbo,alloc::alloc::Global>
                      ();
          }
          lVar18 = in_R8[1];
          lVar21 = lVar20 * 0x60;
          auVar13._8_4_ = uStack_170;
          auVar13._0_8_ = local_1f8;
          auVar13._12_4_ = uStack_16c;
          puVar3 = (undefined8 *)(lVar18 + 0x30 + lVar21);
          *puVar3 = 0;
          puVar3[1] = 0;
          puVar3 = (undefined8 *)(lVar18 + 0x20 + lVar21);
          *puVar3 = local_1e8;
          puVar3[1] = uStack_1e0;
          *(undefined1 (*) [16])(lVar18 + 0x10 + lVar21) = auVar13;
          *(undefined8 *)(lVar18 + lVar21) = CONCAT44(fVar30,extraout_XMM0_Da_14);
          ((undefined8 *)(lVar18 + lVar21))[1] = uStack_200;
          *(float *)(lVar18 + 0x40 + lVar21) = fVar29;
          *(float *)(lVar18 + 0x44 + lVar21) = fVar32;
          *(float *)(lVar18 + 0x48 + lVar21) = (float)local_288._4_4_ + fVar33;
          *(undefined4 *)(lVar18 + 0x4c + lVar21) = 0;
          *(undefined4 *)(lVar18 + 0x50 + lVar21) = extraout_EAX_01;
          *(float *)(lVar18 + 0x54 + lVar21) = (fVar27 - fVar28) * fVar4;
          *(float *)(lVar18 + 0x58 + lVar21) = (fVar27 + fVar28) * fVar4;
          *(undefined4 *)(lVar18 + 0x5c + lVar21) = 4;
          in_R8[2] = lVar20 + 1;
        }
        local_218 = (local_218 - (ulonglong)(local_218 != -1)) + 1;
      } while (local_210 != 0);
      std::alloc::__default_lib_allocator::__rust_dealloc();
      bVar16 = false;
    }
  }
  goto LAB_141222860;
}

