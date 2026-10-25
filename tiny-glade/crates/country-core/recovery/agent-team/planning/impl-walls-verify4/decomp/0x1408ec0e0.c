
// WARNING: Removing unreachable block (ram,0x0001408ec700)
// WARNING: Removing unreachable block (ram,0x0001408ec83e)
// WARNING: Removing unreachable block (ram,0x0001408ec72a)
// WARNING: Removing unreachable block (ram,0x0001408ec6bd)
// WARNING: Removing unreachable block (ram,0x0001408ec502)
// WARNING: Removing unreachable block (ram,0x0001408ec560)
// WARNING: Removing unreachable block (ram,0x0001408ec84f)
// WARNING: Removing unreachable block (ram,0x0001408ec56b)
// WARNING: Removing unreachable block (ram,0x0001408ec85d)
// WARNING: Removing unreachable block (ram,0x0001408ec583)
// WARNING: Removing unreachable block (ram,0x0001408ec5ab)
// WARNING: Removing unreachable block (ram,0x0001408ec5af)
// WARNING: Removing unreachable block (ram,0x0001408ec5b7)
// WARNING: Removing unreachable block (ram,0x0001408ec5ba)
// WARNING: Removing unreachable block (ram,0x0001408ec7d2)

void __cdecl country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug(void)

{
  undefined8 uVar1;
  longlong lVar3;
  undefined8 uVar4;
  float *extraout_RAX;
  float *extraout_RAX_00;
  longlong *in_RCX;
  int iVar5;
  int iVar6;
  longlong *in_R9;
  undefined8 uVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  longlong local_180;
  undefined8 uVar2;
  
  if (*(char *)(*in_R9 + 0xf1) == '\x01') {
    lVar3 = *in_RCX;
    uVar1 = *(undefined8 *)(lVar3 + 0x50);
    uVar2 = *(undefined8 *)(lVar3 + 0x50);
    ::utils::geometry::rectangle::impl$2::from();
    country_core::utils::debug_viz::DebugRectangle2d::show();
    uVar7 = *(undefined8 *)(lVar3 + 0x48);
    if (1 < (int)uVar7) {
      iVar5 = 1;
      do {
        uVar4 = *(undefined8 *)(lVar3 + 0x48);
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (float *)0x0) goto LAB_1408ec82f;
        fVar8 = (float)iVar5 / (float)(int)uVar4;
        fVar8 = fVar8 * (float)uVar2 * 0.5 + (1.0 - fVar8) * (float)uVar1 * -0.5;
        iVar5 = iVar5 + 1;
        *extraout_RAX = fVar8;
        extraout_RAX[1] = 0.0;
        extraout_RAX[2] = (float)((ulonglong)uVar1 >> 0x20) * -0.5;
        extraout_RAX[3] = fVar8;
        extraout_RAX[4] = 0.0;
        extraout_RAX[5] = (float)((ulonglong)uVar2 >> 0x20) * 0.5;
        country_core::utils::debug_viz::DebugLine::show();
      } while ((int)uVar7 != iVar5);
      uVar7 = *(undefined8 *)(lVar3 + 0x48);
    }
    iVar5 = (int)((ulonglong)uVar7 >> 0x20);
    if (1 < iVar5) {
      iVar6 = 1;
      do {
        uVar1 = *(undefined8 *)(lVar3 + 0x48);
        fVar8 = *(float *)(lVar3 + 0x50);
        fVar10 = *(float *)(lVar3 + 0x54);
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == (float *)0x0) {
LAB_1408ec82f:
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        fVar10 = fVar10 * 0.5;
        fVar9 = (float)iVar6 / (float)(int)uVar1;
        fVar10 = fVar9 * fVar10 - (1.0 - fVar9) * fVar10;
        iVar6 = iVar6 + 1;
        *extraout_RAX_00 = fVar8 * -0.5;
        extraout_RAX_00[1] = 0.0;
        extraout_RAX_00[2] = fVar10;
        extraout_RAX_00[3] = fVar8 * 0.5;
        extraout_RAX_00[4] = 0.0;
        extraout_RAX_00[5] = fVar10;
        country_core::utils::debug_viz::DebugLine::show();
      } while (iVar5 != iVar6);
    }
    ::utils::geometry::circle::Circle2d::new();
    wall_spatial_hash::WallSpatialHash::
    get_cells_from_circle<country_core::systems::wall::wall_state::spatial_hash_debug::spatial_hash_debug::closure_env_2>
              ();
    ::utils::geometry::circle::Circle2d::as_curve3();
    country_core::utils::debug_viz::DebugLine::show();
    if (local_180 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    wall_spatial_hash::WallSpatialHash::query_with_circle();
  }
  return;
}

