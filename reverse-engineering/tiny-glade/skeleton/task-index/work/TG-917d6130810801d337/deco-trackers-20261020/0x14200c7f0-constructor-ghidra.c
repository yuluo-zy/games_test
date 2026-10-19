
void __cdecl
system_decorator::decorator_visual::door_stairs_assemble_bricks::
construct_door_stairs<ref_mut_<system_decorator::decorator_visual::door_stairs_assemble_bricks::door_stairs_assemble_bricks::closure_0::closure_0::closure_env_0>_>
          (void)

{
  longlong lVar1;
  float *pfVar2;
  float *pfVar3;
  float fVar4;
  undefined1 *puVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  bool bVar14;
  char extraout_AL;
  char extraout_AL_00;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  longlong lVar21;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar22;
  float in_stack_00000028;
  float in_stack_00000030;
  longlong in_stack_00000058;
  longlong local_2b8;
  longlong lStack_2b0;
  ulonglong local_2a8;
  longlong local_1c8;
  longlong lStack_1c0;
  ulonglong local_1b8;
  ulonglong local_148;
  longlong local_130;
  
  uVar12 = local_1b8;
  ceilf(((in_XMM2_Da - in_XMM3_Da) + in_stack_00000030) / 0.2);
  ::utils::random_splits();
  if (local_1b8 == 0) {
    local_130 = 4;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    fVar22 = in_XMM3_Da - in_stack_00000030;
    local_130 = extraout_RAX;
    if (local_1b8 < 8 || (ulonglong)(extraout_RAX - lStack_1c0) < 0x20) {
      uVar19 = 0;
    }
    else {
      uVar19 = local_1b8 & 0xfffffffffffffff8;
      uVar16 = 0;
      do {
        pfVar2 = (float *)(lStack_1c0 + uVar16 * 4);
        fVar4 = pfVar2[1];
        fVar6 = pfVar2[2];
        fVar7 = pfVar2[3];
        pfVar3 = (float *)(lStack_1c0 + 0x10 + uVar16 * 4);
        fVar8 = *pfVar3;
        fVar9 = pfVar3[1];
        fVar10 = pfVar3[2];
        fVar11 = pfVar3[3];
        pfVar3 = (float *)(extraout_RAX + uVar16 * 4);
        *pfVar3 = *pfVar2 * fVar22 + (1.0 - *pfVar2) * in_XMM2_Da;
        pfVar3[1] = fVar4 * fVar22 + (1.0 - fVar4) * in_XMM2_Da;
        pfVar3[2] = fVar6 * fVar22 + (1.0 - fVar6) * in_XMM2_Da;
        pfVar3[3] = fVar7 * fVar22 + (1.0 - fVar7) * in_XMM2_Da;
        pfVar2 = (float *)(extraout_RAX + 0x10 + uVar16 * 4);
        *pfVar2 = fVar8 * fVar22 + (1.0 - fVar8) * in_XMM2_Da;
        pfVar2[1] = fVar9 * fVar22 + (1.0 - fVar9) * in_XMM2_Da;
        pfVar2[2] = fVar10 * fVar22 + (1.0 - fVar10) * in_XMM2_Da;
        pfVar2[3] = fVar11 * fVar22 + (1.0 - fVar11) * in_XMM2_Da;
        uVar16 = uVar16 + 8;
      } while (uVar19 != uVar16);
      if (local_1b8 == uVar19) goto LAB_14200ca8b;
    }
    uVar16 = uVar19 | 1;
    if ((local_1b8 & 1) != 0) {
      fVar4 = *(float *)(lStack_1c0 + uVar19 * 4);
      *(float *)(extraout_RAX + uVar19 * 4) = fVar4 * fVar22 + (1.0 - fVar4) * in_XMM2_Da;
      uVar19 = uVar16;
    }
    while (local_1b8 != uVar16) {
      fVar4 = *(float *)(lStack_1c0 + uVar19 * 4);
      *(float *)(extraout_RAX + uVar19 * 4) = fVar4 * fVar22 + (1.0 - fVar4) * in_XMM2_Da;
      fVar4 = *(float *)(lStack_1c0 + 4 + uVar19 * 4);
      *(float *)(extraout_RAX + 4 + uVar19 * 4) = fVar4 * fVar22 + (1.0 - fVar4) * in_XMM2_Da;
      uVar16 = uVar19 + 2;
      uVar19 = uVar16;
    }
  }
LAB_14200ca8b:
  if (local_1c8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  fVar22 = ceilf((in_XMM2_Da - in_XMM3_Da) / 0.2);
  uVar19 = (longlong)fVar22 >> 0x3f & (longlong)(fVar22 - 9.223372e+18) | (longlong)fVar22;
  if (fVar22 < 0.0) {
    uVar19 = 0;
  }
  if (1.8446743e+19 < fVar22) {
    uVar19 = 0xffffffffffffffff;
  }
  uVar16 = 5;
  if (uVar19 < 5) {
    uVar16 = uVar19;
  }
  uVar19 = 2;
  if (2 < uVar16) {
    uVar19 = uVar16;
  }
  ceilf(in_stack_00000028 / 0.45);
  uVar16 = 0;
  lVar1 = local_130 + local_1b8 * 4;
  fVar22 = in_stack_00000028 * 0.5;
  puVar5 = *(undefined1 **)(in_stack_00000058 + 8);
  bVar14 = true;
LAB_14200cc79:
  if (local_130 != lVar1) {
    lVar21 = local_130 + 4;
    if (bVar14) {
      if (local_130 + 4 == lVar1) goto LAB_14200d5bd;
      lVar21 = local_130 + 8;
    }
    local_130 = lVar21;
    uVar16 = uVar16 + 1;
    uVar17 = uVar16;
    if (uVar19 < uVar16) {
      uVar17 = uVar19;
    }
    local_148 = 0;
LAB_14200cd95:
    do {
      uVar13 = local_1b8;
      if (uVar17 <= local_148) goto LAB_14200cc70;
      ::utils::random_splits();
      if (local_1b8 == 0) {
        lVar21 = 4;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        lVar21 = extraout_RAX_00;
        if (local_1b8 < 8 || (ulonglong)(extraout_RAX_00 - lStack_1c0) < 0x20) {
          uVar15 = 0;
        }
        else {
          uVar15 = local_1b8 & 0xfffffffffffffff8;
          uVar18 = 0;
          do {
            pfVar2 = (float *)(lStack_1c0 + uVar18 * 4);
            fVar4 = pfVar2[1];
            fVar6 = pfVar2[2];
            fVar7 = pfVar2[3];
            pfVar3 = (float *)(lStack_1c0 + 0x10 + uVar18 * 4);
            fVar8 = *pfVar3;
            fVar9 = pfVar3[1];
            fVar10 = pfVar3[2];
            fVar11 = pfVar3[3];
            pfVar3 = (float *)(extraout_RAX_00 + uVar18 * 4);
            *pfVar3 = *pfVar2 * fVar22 - (1.0 - *pfVar2) * fVar22;
            pfVar3[1] = fVar4 * fVar22 - (1.0 - fVar4) * fVar22;
            pfVar3[2] = fVar6 * fVar22 - (1.0 - fVar6) * fVar22;
            pfVar3[3] = fVar7 * fVar22 - (1.0 - fVar7) * fVar22;
            pfVar2 = (float *)(extraout_RAX_00 + 0x10 + uVar18 * 4);
            *pfVar2 = fVar8 * fVar22 - (1.0 - fVar8) * fVar22;
            pfVar2[1] = fVar9 * fVar22 - (1.0 - fVar9) * fVar22;
            pfVar2[2] = fVar10 * fVar22 - (1.0 - fVar10) * fVar22;
            pfVar2[3] = fVar11 * fVar22 - (1.0 - fVar11) * fVar22;
            uVar18 = uVar18 + 8;
          } while (uVar15 != uVar18);
          if (local_1b8 == uVar15) goto LAB_14200cf2f;
        }
        uVar18 = uVar15 | 1;
        if ((local_1b8 & 1) != 0) {
          fVar4 = *(float *)(lStack_1c0 + uVar15 * 4);
          *(float *)(extraout_RAX_00 + uVar15 * 4) = fVar4 * fVar22 - (1.0 - fVar4) * fVar22;
          uVar15 = uVar18;
        }
        while (local_1b8 != uVar18) {
          fVar4 = *(float *)(lStack_1c0 + uVar15 * 4);
          *(float *)(extraout_RAX_00 + uVar15 * 4) = fVar4 * fVar22 - (1.0 - fVar4) * fVar22;
          fVar4 = *(float *)(lStack_1c0 + 4 + uVar15 * 4);
          *(float *)(extraout_RAX_00 + 4 + uVar15 * 4) = fVar4 * fVar22 - (1.0 - fVar4) * fVar22;
          uVar18 = uVar15 + 2;
          uVar15 = uVar18;
        }
      }
LAB_14200cf2f:
      if (local_1c8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_148 = local_148 + 1;
      if (local_1b8 != 0) {
        if (local_1b8 != 1) {
          ::utils::swizzlers::impl$1::xvy();
          ::utils::xz_rotation_bad();
          country_core::resources::render::impl$1::from();
          country_core::geometry::instanced_wall::impl$1::from();
          minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction()
          ;
          country_core::geometry::instanced_wall::impl$3::default();
          local_1b8 = local_2a8;
          local_1c8 = local_2b8;
          lStack_1c0 = lStack_2b0;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
          if (extraout_AL != '\0') {
            *puVar5 = 1;
          }
          if (uVar13 != 2) {
            lVar20 = lVar21 + 8;
            do {
              ::utils::swizzlers::impl$1::xvy();
              ::utils::xz_rotation_bad();
              country_core::resources::render::impl$1::from();
              country_core::geometry::instanced_wall::impl$1::from();
              minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::
              get_instruction();
              country_core::geometry::instanced_wall::impl$3::default();
              country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
              push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
              if (extraout_AL_00 != '\0') {
                *puVar5 = 1;
              }
              lVar20 = lVar20 + 4;
            } while (lVar20 != lVar21 + uVar13 * 4);
            if (uVar13 == 0) goto LAB_14200cd95;
          }
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    } while( true );
  }
LAB_14200d5bd:
  if (uVar12 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
LAB_14200cc70:
  bVar14 = false;
  goto LAB_14200cc79;
}

