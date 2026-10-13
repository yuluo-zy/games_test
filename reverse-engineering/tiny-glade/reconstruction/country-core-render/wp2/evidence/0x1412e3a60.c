// address 0x1412e3a60  name allocation_to_draw_multi_instanced<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// signature: void __cdecl allocation_to_draw_multi_instanced<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

void __cdecl
country_core::render::sparse_instance_buffer::SparseInstanceBuffer::
allocation_to_draw_multi_instanced<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

{
  byte bVar1;
  uint uVar2;
  uint *puVar3;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong in_R8;
  uint uVar4;
  uint *puVar5;
  uint uVar6;
  uint uVar7;
  longlong lVar8;
  longlong local_50;
  
  lVar8 = *(longlong *)(in_R8 + 8);
  if (lVar8 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  puVar5 = *(uint **)(lVar8 + 8);
  puVar3 = puVar5 + *(longlong *)(lVar8 + 0x10);
  local_50 = 0;
  lVar8 = 0;
  uVar4 = 0;
  uVar6 = 0;
  while( true ) {
    if (puVar5 == puVar3) {
      if (uVar4 <= uVar6 && uVar6 - uVar4 != 0) {
        if (lVar8 == 0) {
          alloc::raw_vec::RawVec::
          grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                    ();
        }
        *(uint *)(lVar8 * 8 + 4) = uVar4;
        *(uint *)(lVar8 * 8 + 8) = uVar6 - uVar4;
        local_50 = lVar8 + 1;
      }
      in_RCX[2] = local_50;
      *in_RCX = 0;
      in_RCX[1] = 4;
      return;
    }
    uVar2 = *puVar5;
    if (*(ulonglong *)(in_RDX + 0x28) <= (ulonglong)uVar2) break;
    uVar7 = uVar2 * 0x20;
    bVar1 = *(byte *)(*(longlong *)(in_RDX + 0x20) + (ulonglong)uVar2);
    uVar2 = uVar4;
    if ((uVar6 != uVar7) && (uVar2 = uVar7, uVar4 <= uVar6 && uVar6 - uVar4 != 0)) {
      if (lVar8 == 0) {
        alloc::raw_vec::RawVec::
        grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                  ();
      }
      *(uint *)(lVar8 * 8 + 4) = uVar4;
      *(uint *)(lVar8 * 8 + 8) = uVar6 - uVar4;
      lVar8 = lVar8 + 1;
      local_50 = lVar8;
    }
    uVar4 = uVar2;
    puVar5 = puVar5 + 1;
    uVar6 = uVar7 + bVar1;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}


