
void __cdecl system_decorator::decorator_visual::stairs_railings_assemble::railings_tips(void)

{
  undefined8 *puVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined8 *puVar6;
  undefined8 *puVar7;
  undefined8 *extraout_RAX;
  longlong extraout_RAX_00;
  bool bVar8;
  longlong lVar9;
  int iVar10;
  ulonglong uVar11;
  ulonglong *in_R9;
  ulonglong uVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  byte in_stack_00000038;
  longlong in_stack_00000048;
  ulonglong in_stack_00000050;
  longlong in_stack_00000058;
  ulonglong in_stack_00000060;
  longlong in_stack_00000068;
  ulonglong in_stack_00000070;
  longlong local_348;
  undefined8 *puStack_340;
  longlong local_2a8;
  longlong local_298;
  longlong local_290;
  undefined8 local_264;
  longlong local_248;
  undefined8 *puStack_240;
  longlong local_238;
  longlong lStack_230;
  undefined8 local_1f0;
  float local_1e8;
  undefined8 local_1c8;
  undefined8 local_1b8;
  undefined8 *puStack_1b0;
  int local_f8;
  char local_f2 [178];
  
  local_f2[2] = -2;
  local_f2[3] = -1;
  local_f2[4] = -1;
  local_f2[5] = -1;
  local_f2[6] = -1;
  local_f2[7] = -1;
  local_f2[8] = -1;
  local_f2[9] = -1;
  local_f2[0] = '\0';
  local_f2[1] = '\0';
  railings_tips::closure_0();
  railings_tips::closure_0();
  lVar9 = 0;
  do {
    if (local_f2[lVar9] == '\x01') {
      uVar11 = in_stack_00000070 - 1;
      if (lVar9 == 0) {
        uVar11 = 0;
      }
      if (in_stack_00000070 <= uVar11) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (in_stack_00000060 <= uVar11) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      fVar13 = *(float *)(&__real_3f800000bf800000 + (ulonglong)(lVar9 == 0) * 4);
      fVar17 = *(float *)(in_stack_00000068 + uVar11 * 0xc);
      fVar15 = *(float *)(in_stack_00000068 + 4 + uVar11 * 0xc);
      fVar16 = *(float *)(in_stack_00000068 + 8 + uVar11 * 0xc);
      ::utils::swizzlers::impl$0::x0y();
      fVar20 = fVar13 * local_1b8._4_4_;
      fVar21 = (float)local_1b8 * fVar13;
      fVar19 = *(float *)(in_stack_00000058 + uVar11 * 8);
      fVar14 = *(float *)(in_stack_00000058 + 4 + uVar11 * 8);
      fVar18 = 1.0 / SQRT(fVar19 * fVar19 + fVar14 * fVar14);
      local_1b8 = CONCAT44(fVar19 * fVar18,-fVar14 * fVar18);
      ::utils::swizzlers::impl$0::x0y();
      if (in_stack_00000050 <= uVar11) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      fVar15 = fVar15 + fVar20 * 0.0975;
      fVar16 = fVar16 + puStack_1b0._0_4_ * fVar13 * 0.0975;
      fVar17 = fVar17 + fVar21 * 0.0975;
      fVar13 = *(float *)(in_stack_00000048 + uVar11 * 4);
      iVar10 = 0;
      ::utils::xz_rotation_mat3_bad();
      local_f8 = 1;
      if (1 < in_stack_00000038) {
        if (in_stack_00000038 != 2) {
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
            alloc::alloc::handle_alloc_error();
          }
          fVar19 = fVar13 + -0.5 + 0.19;
          fVar14 = local_1e8 * fVar19 * 0.5;
          fVar13 = (float)local_1f0 * fVar19 * 0.5;
          fVar19 = (float)((ulonglong)local_1f0 >> 0x20) * fVar19 * 0.5;
          *extraout_RAX = CONCAT44(fVar15 + fVar19,fVar17 + fVar13);
          *(float *)(extraout_RAX + 1) = fVar14 + fVar16;
          *(ulonglong *)((longlong)extraout_RAX + 0xc) = CONCAT44(fVar15 - fVar19,fVar17 - fVar13);
          *(float *)((longlong)extraout_RAX + 0x14) = fVar16 - fVar14;
          local_1b8 = 2;
          ::utils::curve::Curve::
          try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
          puStack_1b0 = extraout_RAX;
          if (!SBORROW8(0,local_248)) {
            local_1b8 = local_248;
            puStack_1b0 = puStack_240;
            ::utils::curve::Curve::
            try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
            if (local_248 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (lStack_230 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (!SBORROW8(0,local_2a8)) {
              if (local_290 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if ((local_298 != 0) &&
                 (std::alloc::__default_lib_allocator::__rust_alloc_zeroed(), extraout_RAX_00 == 0))
              {
                    // WARNING: Subroutine does not return
                alloc::raw_vec::handle_error();
              }
              construct_wooden_railing();
              if (local_298 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_2a8 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
          }
          goto LAB_1407fcde7;
        }
        local_f8 = 2;
      }
      if (local_f8 != 0) {
        local_1c8 = CONCAT44((float)((ulonglong)local_264 >> 0x20) * 0.46,(float)local_264 * 0.46);
        do {
          ::utils::random_splits();
          if (local_238 != 0) {
            puVar1 = (undefined8 *)((longlong)puStack_240 + local_238 * 4);
            bVar8 = true;
            puVar7 = puStack_240;
            do {
              puVar6 = (undefined8 *)((longlong)puVar7 + 4);
              if (bVar8) {
                if ((undefined8 *)((longlong)puVar7 + 4) == puVar1) break;
                puVar6 = puVar7 + 1;
              }
              puVar7 = puVar6;
              local_2a8 = local_1c8;
              country_core::resources::render::Affine3Packed::from_cols();
              country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
              uVar12 = *in_R9;
              uVar11 = uVar12 + 0xa0761d6478bd642f;
              *in_R9 = uVar11;
              auVar2._8_8_ = 0;
              auVar2._0_8_ = uVar11 ^ 0xe7037ed1a0b428db;
              auVar4._8_8_ = 0;
              auVar4._0_8_ = uVar11;
              if ((SUB164(auVar2 * auVar4,8) ^ SUB164(auVar2 * auVar4,0)) * 1000 < 0x128) {
                uVar12 = uVar12 + 0x40ec3ac8f17ac85e;
                do {
                  uVar11 = uVar11 + 0xa0761d6478bd642f;
                  auVar3._8_8_ = 0;
                  auVar3._0_8_ = uVar12 ^ 0xe7037ed1a0b428db;
                  auVar5._8_8_ = 0;
                  auVar5._0_8_ = uVar12;
                  uVar12 = uVar12 + 0xa0761d6478bd642f;
                } while ((SUB164(auVar3 * auVar5,8) ^ SUB164(auVar3 * auVar5,0)) * 1000 < 0x128);
                *in_R9 = uVar11;
              }
              local_1b8 = local_348;
              puStack_1b0 = puStack_340;
              country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
              push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
              bVar8 = false;
            } while (puVar7 != puVar1);
          }
          if (local_248 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          iVar10 = iVar10 + 1;
        } while (iVar10 != local_f8);
      }
    }
LAB_1407fcde7:
    lVar9 = lVar9 + 1;
    if (lVar9 == 2) {
      return;
    }
  } while( true );
}

