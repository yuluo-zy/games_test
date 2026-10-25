
void __cdecl
system_decorator::decorator_visual::chimney_assemble_bricks::
construct_crossbrick_chimney<system_decorator::decorator_visual::chimney_assemble_bricks::chimney_assemble_bricks::closure_0::closure_0::closure_env_0>
          (void)

{
  float *pfVar1;
  longlong lVar2;
  float *pfVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  ulonglong uVar11;
  longlong lVar12;
  bool bVar13;
  char extraout_AL;
  longlong extraout_RAX;
  ulonglong uVar14;
  ulonglong uVar15;
  float in_XMM2_Da;
  float in_XMM3_Da;
  undefined1 *in_stack_00000060;
  undefined4 local_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  uint local_238;
  undefined4 uStack_234;
  longlong local_128;
  
  ceilf((in_XMM2_Da - in_XMM3_Da) / 0.4);
  ::utils::random_splits();
  lVar2 = CONCAT44(uStack_23c,uStack_240);
  uVar11 = CONCAT44(uStack_234,local_238);
  if (uVar11 == 0) {
    local_128 = 4;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_128 = extraout_RAX;
    if (uVar11 < 8 || (ulonglong)(extraout_RAX - lVar2) < 0x20) {
      uVar14 = 0;
    }
    else {
      uVar14 = uVar11 & 0xfffffffffffffff8;
      uVar15 = 0;
      do {
        pfVar1 = (float *)(lVar2 + uVar15 * 4);
        fVar4 = pfVar1[1];
        fVar5 = pfVar1[2];
        fVar6 = pfVar1[3];
        pfVar3 = (float *)(lVar2 + 0x10 + uVar15 * 4);
        fVar7 = *pfVar3;
        fVar8 = pfVar3[1];
        fVar9 = pfVar3[2];
        fVar10 = pfVar3[3];
        pfVar3 = (float *)(extraout_RAX + uVar15 * 4);
        *pfVar3 = *pfVar1 * in_XMM2_Da + (1.0 - *pfVar1) * in_XMM3_Da;
        pfVar3[1] = fVar4 * in_XMM2_Da + (1.0 - fVar4) * in_XMM3_Da;
        pfVar3[2] = fVar5 * in_XMM2_Da + (1.0 - fVar5) * in_XMM3_Da;
        pfVar3[3] = fVar6 * in_XMM2_Da + (1.0 - fVar6) * in_XMM3_Da;
        pfVar1 = (float *)(extraout_RAX + 0x10 + uVar15 * 4);
        *pfVar1 = fVar7 * in_XMM2_Da + (1.0 - fVar7) * in_XMM3_Da;
        pfVar1[1] = fVar8 * in_XMM2_Da + (1.0 - fVar8) * in_XMM3_Da;
        pfVar1[2] = fVar9 * in_XMM2_Da + (1.0 - fVar9) * in_XMM3_Da;
        pfVar1[3] = fVar10 * in_XMM2_Da + (1.0 - fVar10) * in_XMM3_Da;
        uVar15 = uVar15 + 8;
      } while (uVar14 != uVar15);
      if (uVar11 == uVar14) goto LAB_14200bf47;
    }
    uVar15 = uVar14 | 1;
    if ((local_238 & 1) != 0) {
      fVar4 = *(float *)(lVar2 + uVar14 * 4);
      *(float *)(extraout_RAX + uVar14 * 4) = fVar4 * in_XMM2_Da + (1.0 - fVar4) * in_XMM3_Da;
      uVar14 = uVar15;
    }
    while (uVar11 != uVar15) {
      fVar4 = *(float *)(lVar2 + uVar14 * 4);
      *(float *)(extraout_RAX + uVar14 * 4) = fVar4 * in_XMM2_Da + (1.0 - fVar4) * in_XMM3_Da;
      fVar4 = *(float *)(lVar2 + 4 + uVar14 * 4);
      *(float *)(extraout_RAX + 4 + uVar14 * 4) = fVar4 * in_XMM2_Da + (1.0 - fVar4) * in_XMM3_Da;
      uVar15 = uVar14 + 2;
      uVar14 = uVar15;
    }
  }
LAB_14200bf47:
  if (CONCAT44(uStack_244,local_248) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (uVar11 != 0) {
    lVar2 = local_128 + uVar11 * 4;
    bVar13 = true;
    do {
      lVar12 = local_128 + 4;
      if (bVar13) {
        if (local_128 + 4 == lVar2) break;
        lVar12 = local_128 + 8;
      }
      local_128 = lVar12;
      ::utils::swizzlers::impl$1::xvy();
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::rectangle::Rectangle2d::new();
      uVar14 = 0;
      while (uVar14 < 4) {
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        uVar14 = uVar14 + 1;
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        fastrand::Rng::f32();
        ::utils::swizzlers::impl$1::xvy();
        ::utils::geometry::rectangle::Rectangle2d::get_edge_normal();
        ::utils::xz_rotation_bad();
        country_core::resources::render::impl$1::from();
        country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        country_core::geometry::instanced_wall::impl$3::default();
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
        if (extraout_AL != '\0') {
          *in_stack_00000060 = 1;
        }
      }
      bVar13 = false;
    } while (local_128 != lVar2);
    if (uVar11 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

