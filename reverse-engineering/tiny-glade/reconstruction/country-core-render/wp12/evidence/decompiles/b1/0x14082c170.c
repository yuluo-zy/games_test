// gpu_flush_brick_buffers addr=0x14082c170 size=572 sig=void __cdecl gpu_flush_brick_buffers(void)

void __cdecl country_core::systems::wall::upload_instanced_wall_ssbo::gpu_flush_brick_buffers(void)

{
  uint *puVar1;
  longlong lVar2;
  undefined4 uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong extraout_RAX;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong in_RDX;
  longlong *in_R8;
  uint *puVar12;
  
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  asset_libraries::ssbo_library::AssetSsboLibrary::
  get_mut<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar4 = *in_R8;
  lVar5 = in_R8[1];
  uVar3 = *(undefined4 *)((longlong)in_R8 + 0x14);
  puVar12 = *(uint **)(lVar4 + 0xf8);
  puVar1 = puVar12 + *(longlong *)(lVar4 + 0x100);
  if (*(char *)(lVar4 + 0x108) == '\0') {
    while (puVar12 != puVar1) {
      while( true ) {
        lVar2 = *(longlong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*puVar12 * 0xa0);
        if (lVar2 != 0) break;
        puVar12 = puVar12 + 1;
        if (puVar12 == puVar1) goto LAB_14082c38e;
      }
      lVar9 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*puVar12 * 0xa0;
      uVar10 = (ulonglong)*(uint *)(lVar9 + 0x94);
      lVar6 = *(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x18 + uVar10 * 0x48);
      lVar11 = ~*(ulonglong *)
                 (*(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x38 + uVar10 * 0x48) +
                 *(longlong *)(lVar4 + 0x110) * 8) * 0x30;
      lVar7 = *(longlong *)(lVar6 + 0x10 + lVar11);
      lVar6 = *(longlong *)(lVar6 + 0x28 + lVar11);
      lVar9 = *(longlong *)(lVar9 + 0x50);
      puVar12 = puVar12 + 1;
      lVar11 = 0;
      while( true ) {
        *(undefined4 *)(lVar6 + (ulonglong)*(uint *)(lVar9 + 8 + lVar11 * 0x10) * 4) = uVar3;
        geometry::instanced_wall::InstancedWall::upload_to_storage_buffer();
        if (lVar11 + 1 == lVar2) break;
        lVar11 = lVar11 + 1;
        if (lVar7 == 0) goto LAB_14082c38e;
      }
    }
  }
  else {
    while (puVar12 != puVar1) {
      while( true ) {
        uVar10 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)*puVar12 * 0x48);
        if (uVar10 != 0) break;
        puVar12 = puVar12 + 1;
        if (puVar12 == puVar1) goto LAB_14082c38e;
      }
      lVar2 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*puVar12 * 0x48;
      lVar6 = *(longlong *)(lVar2 + 0x18);
      lVar7 = ~*(ulonglong *)(*(longlong *)(lVar2 + 0x38) + *(longlong *)(lVar4 + 0x110) * 8) * 0x30
      ;
      lVar2 = *(longlong *)(lVar6 + 0x10 + lVar7);
      lVar6 = *(longlong *)(lVar6 + 0x28 + lVar7);
      puVar12 = puVar12 + 1;
      uVar8 = 0;
      while( true ) {
        *(undefined4 *)(lVar6 + (uVar8 & 0xffffffff) * 4) = uVar3;
        geometry::instanced_wall::InstancedWall::upload_to_storage_buffer();
        if (uVar8 + 1 == uVar10) break;
        uVar8 = uVar8 + 1;
        if (lVar2 == 0) goto LAB_14082c38e;
      }
    }
  }
LAB_14082c38e:
  country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
  gpu_flush<country_core::geometry::instanced_wall::BrickTransformSsbo>();
  return;
}

