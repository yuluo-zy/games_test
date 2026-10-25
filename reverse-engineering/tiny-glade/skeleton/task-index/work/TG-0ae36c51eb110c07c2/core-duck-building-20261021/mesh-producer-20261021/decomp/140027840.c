
// WARNING: Removing unreachable block (ram,0x000140027fab)
// WARNING: Removing unreachable block (ram,0x000140027fd4)

void __cdecl
system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_25<false>
          (void)

{
  ulonglong uVar1;
  undefined4 *puVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined8 uVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  uint extraout_EAX;
  ulonglong extraout_RAX;
  longlong *in_RCX;
  ulonglong uVar16;
  longlong *in_RDX;
  undefined4 *puVar17;
  char in_R8B;
  longlong lVar18;
  longlong lVar19;
  ulonglong uVar20;
  float extraout_XMM0_Da;
  float fVar21;
  float fVar22;
  float fStack_1d8;
  longlong local_1d0;
  undefined4 local_1c8;
  undefined4 uStack_1c4;
  longlong lStack_1c0;
  longlong local_1b8;
  longlong lStack_1b0;
  longlong local_1a8;
  longlong lStack_1a0;
  longlong local_148;
  longlong local_140;
  longlong local_d8;
  longlong local_a0;
  
  lVar15 = local_148;
  lVar11 = *(longlong *)(*in_RDX + 8);
  uVar12 = *(ulonglong *)(*in_RDX + 0x10);
  if (uVar12 != 0) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if ((uVar12 < 0x14) ||
       (extraout_RAX < (lVar11 + uVar12 * 0xc) - 4 && lVar11 + 4U < extraout_RAX + uVar12 * 4)) {
      uVar20 = 0;
    }
    else {
      uVar20 = uVar12 & 0xfffffffffffffff8;
      puVar17 = (undefined4 *)(lVar11 + 0x58);
      uVar16 = 0;
      do {
        uVar3 = puVar17[-0x12];
        uVar4 = puVar17[-0xf];
        uVar5 = puVar17[-0xc];
        uVar6 = *puVar17;
        uVar7 = puVar17[-3];
        uVar8 = puVar17[-6];
        uVar9 = puVar17[-9];
        puVar2 = (undefined4 *)(extraout_RAX + uVar16 * 4);
        *puVar2 = puVar17[-0x15];
        puVar2[1] = uVar3;
        puVar2[2] = uVar4;
        puVar2[3] = uVar5;
        puVar2 = (undefined4 *)(extraout_RAX + 0x10 + uVar16 * 4);
        *puVar2 = uVar9;
        puVar2[1] = uVar8;
        puVar2[2] = uVar7;
        puVar2[3] = uVar6;
        uVar16 = uVar16 + 8;
        puVar17 = puVar17 + 0x18;
      } while (uVar20 != uVar16);
      if (uVar12 == uVar20) goto LAB_140027a17;
    }
    uVar16 = uVar20;
    if ((uVar12 & 3) != 0) {
      lVar19 = 0;
      do {
        *(undefined4 *)(extraout_RAX + uVar16 * 4) =
             *(undefined4 *)(lVar11 + uVar20 * 0xc + 4 + lVar19 * 4);
        uVar16 = uVar16 + 1;
        lVar19 = lVar19 + 3;
      } while ((uVar12 & 3) * 3 != lVar19);
    }
    if (uVar20 - uVar12 < 0xfffffffffffffffd) {
      puVar17 = (undefined4 *)(lVar11 + uVar16 * 0xc + 0x28);
      do {
        *(undefined4 *)(extraout_RAX + uVar16 * 4) = puVar17[-9];
        *(undefined4 *)(extraout_RAX + 4 + uVar16 * 4) = puVar17[-6];
        *(undefined4 *)(extraout_RAX + 8 + uVar16 * 4) = puVar17[-3];
        *(undefined4 *)(extraout_RAX + 0xc + uVar16 * 4) = *puVar17;
        uVar16 = uVar16 + 4;
        puVar17 = puVar17 + 0xc;
      } while (uVar12 != uVar16);
    }
  }
LAB_140027a17:
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>_>
            ();
  ::utils::curve::Curve::
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  lVar11 = CONCAT44(uStack_1c4,local_1c8);
  if (SBORROW8(0,local_1d0)) {
    in_RCX[1] = lVar11;
    in_RCX[2] = lStack_1c0;
    *in_RCX = -0x8000000000000000;
  }
  else {
    fStack_1d8 = (float)lStack_1a0;
    fStack_1d8 = 1.3 / fStack_1d8;
    if (in_R8B == '\0') {
      fStack_1d8 = 1.0 - fStack_1d8;
      if (fStack_1d8 <= 0.5) {
        fStack_1d8 = 0.5;
      }
      if (1.0 < fStack_1d8) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
    }
    else {
      if (0.5 <= fStack_1d8) {
        fStack_1d8 = 0.5;
      }
      if (fStack_1d8 < 0.0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
    }
    ::utils::curve::Curve::try_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    if (SBORROW8(0,local_1d0)) {
      in_RCX[1] = CONCAT44(uStack_1c4,local_1c8);
      in_RCX[2] = lStack_1c0;
      *in_RCX = -0x8000000000000000;
    }
    else {
      ::utils::curve::Curve::
      try_resample_approx_pt_count<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
      if (SBORROW8(0,lVar11)) {
        in_RCX[1] = lStack_1c0;
        in_RCX[2] = local_148;
        *in_RCX = -0x8000000000000000;
        local_140 = local_1b8;
        if (local_1d0 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      else {
        if (local_1d0 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (local_1b8 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_a0 = 0;
        local_d8 = 0;
        local_148 = 0;
        lVar19 = in_RDX[1];
        lVar13 = in_RDX[2];
        for (lVar18 = lStack_1c0; lVar18 != lStack_1c0 + lVar15 * 8; lVar18 = lVar18 + 8) {
          ::utils::curve::Curve::
          get_approx_coord_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
          uVar16 = (ulonglong)extraout_EAX;
          uVar20 = *(ulonglong *)(lVar19 + 0x10);
          if (uVar20 <= uVar16) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          uVar1 = uVar16 + 1;
          if (uVar20 <= uVar1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar14 = *(longlong *)(lVar19 + 8);
          fVar21 = *(float *)(lVar14 + uVar16 * 4);
          fVar22 = *(float *)(lVar14 + 4 + uVar16 * 4);
          if (local_a0 == 0) {
            alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
          }
          *(float *)(local_a0 * 4 + 4) = (fVar22 - fVar21) * extraout_XMM0_Da + fVar21;
          local_a0 = local_a0 + 1;
          uVar20 = *(ulonglong *)(lVar13 + 0x10);
          if (uVar20 <= uVar16) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          if (uVar20 <= uVar1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar14 = *(longlong *)(lVar13 + 8);
          uVar10 = *(undefined8 *)(lVar14 + uVar16 * 8);
          fVar21 = (float)uVar10;
          fVar22 = (float)((ulonglong)uVar10 >> 0x20);
          uVar10 = *(undefined8 *)(lVar14 + 8 + uVar16 * 8);
          if (local_d8 == 0) {
            alloc::raw_vec::RawVec::grow_one<tuple_<u8,u32>,alloc::alloc::Global>();
          }
          *(ulonglong *)(local_d8 * 8 + 4) =
               CONCAT44(extraout_XMM0_Da * ((float)((ulonglong)uVar10 >> 0x20) - fVar22) + fVar22,
                        extraout_XMM0_Da * ((float)uVar10 - fVar21) + fVar21);
          local_d8 = local_d8 + 1;
          if (uVar12 <= uVar16) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          if (uVar12 <= uVar1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          ::utils::swizzlers::impl$1::xvy();
          if (local_148 == 0) {
            alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
          }
          *(undefined4 *)(local_148 * 0xc + 0xc) = local_1c8;
          *(longlong *)(local_148 * 0xc + 4) = local_1d0;
          local_148 = local_148 + 1;
        }
        ::utils::curve::Curve::
        try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
        if (SBORROW8(0,local_1d0)) {
          in_RCX[1] = CONCAT44(uStack_1c4,local_1c8);
          in_RCX[2] = lStack_1c0;
          *in_RCX = -0x8000000000000000;
          if (lVar11 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
        else {
          in_RCX[5] = local_1a8;
          in_RCX[6] = lStack_1a0;
          in_RCX[3] = local_1b8;
          in_RCX[4] = lStack_1b0;
          in_RCX[1] = CONCAT44(uStack_1c4,local_1c8);
          in_RCX[2] = lStack_1c0;
          in_RCX[7] = 0;
          in_RCX[8] = 4;
          in_RCX[9] = local_d8;
          in_RCX[0xc] = local_a0;
          in_RCX[10] = 0;
          in_RCX[0xb] = 4;
          *in_RCX = local_1d0;
          if (lVar11 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
      }
      if (local_140 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    if (local_1d0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_1b8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if (uVar12 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

