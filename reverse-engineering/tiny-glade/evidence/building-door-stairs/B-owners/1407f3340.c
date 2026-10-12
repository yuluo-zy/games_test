
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_decorator::decorator_visual::gothic_balcony_door::check_for_door_stairs(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  bool bVar4;
  float fVar5;
  float *extraout_RAX;
  float *in_R8;
  longlong in_R9;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar6;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  longlong in_stack_00000028;
  
  fVar1 = in_R8[4];
  fVar6 = in_R8[5];
  fVar2 = in_R8[6];
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>();
  fVar12 = extraout_XMM0_Da;
  if (extraout_XMM0_Da <= 0.0) {
    fVar12 = 0.0;
  }
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  fVar6 = (*(float *)(in_R9 + 0x1c) - *(float *)(in_R9 + 0x18)) * __real_bf000000 + fVar6;
  if (extraout_XMM0_Da_00 <= 0.0) {
    bVar4 = false;
  }
  else {
    bVar4 = fVar6 <= __real_3dcccccd;
    if (bVar4) goto LAB_1407f34ae;
  }
  if (fVar12 <= fVar6 + __real_be4ccccd) {
    return;
  }
LAB_1407f34ae:
  fVar6 = *in_R8;
  fVar10 = in_R8[1];
  fVar11 = in_R8[2];
  fVar5 = in_R8[3];
  fVar7 = fVar11 * fVar11 + fVar10 * fVar10 + fVar6 * fVar6;
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>();
  if (__real_3dcccccd < fVar12 - extraout_XMM0_Da_01) {
    fVar3 = *(float *)(in_R9 + 0x10);
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>();
    ::utils::float_grid::impl$0::
    bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>();
    fVar14 = extraout_XMM0_Da_03;
    if (extraout_XMM0_Da_02 <= extraout_XMM0_Da_03) {
      fVar14 = extraout_XMM0_Da_02;
    }
    fVar13 = (float)(~-(uint)NAN(extraout_XMM0_Da_02) & (uint)fVar14 |
                    -(uint)NAN(extraout_XMM0_Da_02) & (uint)extraout_XMM0_Da_03);
    fVar14 = extraout_XMM0_Da_01;
    if (fVar13 <= extraout_XMM0_Da_01) {
      fVar14 = fVar13;
    }
    fVar14 = (float)(-(uint)NAN(fVar13) & (uint)extraout_XMM0_Da_01 |
                    ~-(uint)NAN(fVar13) & (uint)fVar14);
    if (!(bool)(~bVar4 & __real_3fc00000 <= fVar12 - fVar14)) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      fVar13 = __real_bf4ccccd - fVar14;
      if (__real_bf4ccccd - fVar14 <= 0.0) {
        fVar13 = 0.0;
      }
      fVar8 = _UNK_1429cd0c8 * fVar11 +
              _UNK_1429cd0c4 * fVar10 + ___xmm_000000003f8000000000000000000000 * fVar6;
      fVar8 = fVar8 + fVar8;
      fVar9 = fVar8 * fVar6 + (fVar5 * fVar5 - fVar7) * ___xmm_000000003f8000000000000000000000 +
              (fVar5 + fVar5) * (fVar10 * _UNK_1429cd0c8 - fVar11 * 0.0);
      fVar11 = fVar8 * fVar11 + (fVar5 * fVar5 - fVar7) * _UNK_1429cd0c8 +
               (fVar5 + fVar5) * (fVar6 * _UNK_1429cd0c4 - fVar10 * 0.0);
      fVar10 = __real_3f800000 / SQRT(fVar11 * fVar11 + fVar9 * fVar9);
      fVar6 = __real_bf4ccccd;
      if (!bVar4) {
        fVar6 = fVar14;
      }
      *extraout_RAX = fVar1;
      extraout_RAX[1] = fVar2;
      *(ulonglong *)(extraout_RAX + 2) = CONCAT44(fVar11 * fVar10,fVar9 * fVar10);
      extraout_RAX[4] = fVar12;
      extraout_RAX[5] = fVar6;
      extraout_RAX[6] = fVar13;
      extraout_RAX[7] = fVar3;
      if (*(longlong *)(in_stack_00000028 + 0x120) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *(float **)(in_stack_00000028 + 0x120) = extraout_RAX;
    }
  }
  return;
}

