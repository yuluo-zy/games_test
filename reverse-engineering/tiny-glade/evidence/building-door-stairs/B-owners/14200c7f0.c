
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::door_stairs_assemble_bricks::
construct_door_stairs<ref_mut_<system_decorator::decorator_visual::door_stairs_assemble_bricks::door_stairs_assemble_bricks::closure_0::closure_0::closure_env_0>_>
          (void)

{
  longlong lVar1;
  float *pfVar2;
  float *pfVar3;
  float fVar4;
  float fVar5;
  undefined1 *puVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  bool bVar16;
  char extraout_AL;
  char extraout_AL_00;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong lVar22;
  longlong lVar23;
  float fVar24;
  float in_XMM2_Da;
  float fVar25;
  float in_XMM3_Da;
  float fVar26;
  float fVar27;
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
  
  uVar14 = local_1b8;
  ceilf(((in_XMM2_Da - in_XMM3_Da) + in_stack_00000030) / __real_3e4ccccd);
  ::utils::random_splits();
  if (local_1b8 == 0) {
    local_130 = 4;
    fVar24 = __real_3f800000;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    fVar25 = _UNK_14292593c;
    fVar4 = _UNK_142925938;
    fVar13 = _UNK_142925934;
    fVar24 = ___xmm_3f8000003f8000003f8000003f800000;
    if (extraout_RAX == 0) {
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    fVar26 = in_XMM3_Da - in_stack_00000030;
    local_130 = extraout_RAX;
    if (local_1b8 < 8 || (ulonglong)(extraout_RAX - lStack_1c0) < 0x20) {
      uVar21 = 0;
    }
    else {
      uVar21 = local_1b8 & 0xfffffffffffffff8;
      uVar18 = 0;
      do {
        pfVar2 = (float *)(lStack_1c0 + uVar18 * 4);
        fVar27 = pfVar2[1];
        fVar5 = pfVar2[2];
        fVar7 = pfVar2[3];
        pfVar3 = (float *)(lStack_1c0 + 0x10 + uVar18 * 4);
        fVar8 = *pfVar3;
        fVar9 = pfVar3[1];
        fVar10 = pfVar3[2];
        fVar11 = pfVar3[3];
        pfVar3 = (float *)(extraout_RAX + uVar18 * 4);
        *pfVar3 = *pfVar2 * fVar26 + (fVar24 - *pfVar2) * in_XMM2_Da;
        pfVar3[1] = fVar27 * fVar26 + (fVar13 - fVar27) * in_XMM2_Da;
        pfVar3[2] = fVar5 * fVar26 + (fVar4 - fVar5) * in_XMM2_Da;
        pfVar3[3] = fVar7 * fVar26 + (fVar25 - fVar7) * in_XMM2_Da;
        pfVar2 = (float *)(extraout_RAX + 0x10 + uVar18 * 4);
        *pfVar2 = fVar8 * fVar26 + (fVar24 - fVar8) * in_XMM2_Da;
        pfVar2[1] = fVar9 * fVar26 + (fVar13 - fVar9) * in_XMM2_Da;
        pfVar2[2] = fVar10 * fVar26 + (fVar4 - fVar10) * in_XMM2_Da;
        pfVar2[3] = fVar11 * fVar26 + (fVar25 - fVar11) * in_XMM2_Da;
        uVar18 = uVar18 + 8;
      } while (uVar21 != uVar18);
      fVar24 = __real_3f800000;
      if (local_1b8 == uVar21) goto LAB_14200ca8b;
    }
    uVar18 = uVar21 | 1;
    fVar24 = __real_3f800000;
    if ((local_1b8 & 1) != 0) {
      fVar24 = *(float *)(lStack_1c0 + uVar21 * 4);
      *(float *)(extraout_RAX + uVar21 * 4) =
           fVar24 * fVar26 + (__real_3f800000 - fVar24) * in_XMM2_Da;
      uVar21 = uVar18;
      fVar24 = __real_3f800000;
    }
    while (fVar13 = __real_3f800000, local_1b8 != uVar18) {
      fVar4 = *(float *)(lStack_1c0 + uVar21 * 4);
      fVar25 = __real_3f800000 - fVar4;
      __real_3f800000 = fVar24;
      *(float *)(extraout_RAX + uVar21 * 4) = fVar4 * fVar26 + fVar25 * in_XMM2_Da;
      fVar24 = *(float *)(lStack_1c0 + 4 + uVar21 * 4);
      *(float *)(extraout_RAX + 4 + uVar21 * 4) = fVar24 * fVar26 + (fVar13 - fVar24) * in_XMM2_Da;
      uVar18 = uVar21 + 2;
      uVar21 = uVar18;
      fVar24 = __real_3f800000;
      __real_3f800000 = fVar13;
    }
  }
LAB_14200ca8b:
  __real_3f800000 = fVar24;
  if (local_1c8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  fVar24 = ceilf((in_XMM2_Da - in_XMM3_Da) / __real_3e4ccccd);
  uVar21 = (longlong)fVar24 >> 0x3f & (longlong)(fVar24 - __real_5f000000) | (longlong)fVar24;
  if (fVar24 < 0.0) {
    uVar21 = 0;
  }
  if (__real_5f7fffff < fVar24) {
    uVar21 = 0xffffffffffffffff;
  }
  uVar18 = 5;
  if (uVar21 < 5) {
    uVar18 = uVar21;
  }
  uVar21 = 2;
  if (2 < uVar18) {
    uVar21 = uVar18;
  }
  ceilf(in_stack_00000028 / __real_3ee66666);
  fVar25 = _UNK_14292593c;
  fVar4 = _UNK_142925938;
  fVar13 = _UNK_142925934;
  fVar24 = ___xmm_3f8000003f8000003f8000003f800000;
  uVar18 = 0;
  lVar1 = local_130 + local_1b8 * 4;
  fVar27 = in_stack_00000028 * __real_3f000000;
  puVar6 = *(undefined1 **)(in_stack_00000058 + 8);
  bVar16 = true;
  fVar26 = __real_3f800000;
LAB_14200cc79:
  if (local_130 != lVar1) {
    lVar23 = local_130 + 4;
    if (bVar16) {
      if (local_130 + 4 == lVar1) goto LAB_14200d5bd;
      lVar23 = local_130 + 8;
    }
    local_130 = lVar23;
    uVar18 = uVar18 + 1;
    uVar19 = uVar18;
    if (uVar21 < uVar18) {
      uVar19 = uVar21;
    }
    local_148 = 0;
LAB_14200cd95:
    do {
      uVar15 = local_1b8;
      if (uVar19 <= local_148) goto LAB_14200cc70;
      ::utils::random_splits();
      if (local_1b8 == 0) {
        lVar23 = 4;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == 0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        lVar23 = extraout_RAX_00;
        if (local_1b8 < 8 || (ulonglong)(extraout_RAX_00 - lStack_1c0) < 0x20) {
          uVar17 = 0;
        }
        else {
          uVar17 = local_1b8 & 0xfffffffffffffff8;
          uVar20 = 0;
          do {
            pfVar2 = (float *)(lStack_1c0 + uVar20 * 4);
            fVar5 = pfVar2[1];
            fVar7 = pfVar2[2];
            fVar8 = pfVar2[3];
            pfVar3 = (float *)(lStack_1c0 + 0x10 + uVar20 * 4);
            fVar9 = *pfVar3;
            fVar10 = pfVar3[1];
            fVar11 = pfVar3[2];
            fVar12 = pfVar3[3];
            pfVar3 = (float *)(extraout_RAX_00 + uVar20 * 4);
            *pfVar3 = *pfVar2 * fVar27 - (fVar24 - *pfVar2) * fVar27;
            pfVar3[1] = fVar5 * fVar27 - (fVar13 - fVar5) * fVar27;
            pfVar3[2] = fVar7 * fVar27 - (fVar4 - fVar7) * fVar27;
            pfVar3[3] = fVar8 * fVar27 - (fVar25 - fVar8) * fVar27;
            pfVar2 = (float *)(extraout_RAX_00 + 0x10 + uVar20 * 4);
            *pfVar2 = fVar9 * fVar27 - (fVar24 - fVar9) * fVar27;
            pfVar2[1] = fVar10 * fVar27 - (fVar13 - fVar10) * fVar27;
            pfVar2[2] = fVar11 * fVar27 - (fVar4 - fVar11) * fVar27;
            pfVar2[3] = fVar12 * fVar27 - (fVar25 - fVar12) * fVar27;
            uVar20 = uVar20 + 8;
          } while (uVar17 != uVar20);
          if (local_1b8 == uVar17) goto LAB_14200cf2f;
        }
        uVar20 = uVar17 | 1;
        if ((local_1b8 & 1) != 0) {
          fVar5 = *(float *)(lStack_1c0 + uVar17 * 4);
          *(float *)(extraout_RAX_00 + uVar17 * 4) = fVar5 * fVar27 - (fVar26 - fVar5) * fVar27;
          uVar17 = uVar20;
        }
        while (local_1b8 != uVar20) {
          fVar5 = *(float *)(lStack_1c0 + uVar17 * 4);
          *(float *)(extraout_RAX_00 + uVar17 * 4) = fVar5 * fVar27 - (fVar26 - fVar5) * fVar27;
          fVar5 = *(float *)(lStack_1c0 + 4 + uVar17 * 4);
          *(float *)(extraout_RAX_00 + 4 + uVar17 * 4) = fVar5 * fVar27 - (fVar26 - fVar5) * fVar27;
          uVar20 = uVar17 + 2;
          uVar17 = uVar20;
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
            *puVar6 = 1;
          }
          fVar26 = __real_3f800000;
          if (uVar15 != 2) {
            lVar22 = lVar23 + 8;
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
                *puVar6 = 1;
              }
              lVar22 = lVar22 + 4;
            } while (lVar22 != lVar23 + uVar15 * 4);
            fVar26 = __real_3f800000;
            if (uVar15 == 0) goto LAB_14200cd95;
          }
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    } while( true );
  }
LAB_14200d5bd:
  if (uVar14 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
LAB_14200cc70:
  bVar16 = false;
  goto LAB_14200cc79;
}

