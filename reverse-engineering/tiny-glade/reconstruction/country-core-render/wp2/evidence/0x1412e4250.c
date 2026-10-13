// address 0x1412e4250  name to_draw_multi_instanced<country_core::resources::roof_tile_ssbo::RoofTileSsbo>
// signature: void __cdecl to_draw_multi_instanced<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

void __cdecl
country_core::render::sparse_instance_buffer::impl$3::
to_draw_multi_instanced<country_core::resources::roof_tile_ssbo::RoofTileSsbo>(void)

{
  ulonglong *puVar1;
  byte bVar2;
  int iVar3;
  ulonglong uVar4;
  bool bVar5;
  undefined8 *in_RCX;
  longlong in_RDX;
  int iVar6;
  ulonglong *puVar7;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  int unaff_R14D;
  ulonglong unaff_R15;
  longlong local_60;
  longlong local_58;
  uint local_4c;
  
  puVar7 = *(ulonglong **)(in_RDX + 0x30);
  puVar1 = puVar7 + *(longlong *)(in_RDX + 0x38);
  local_60 = 0;
  local_58 = 0;
  iVar6 = 0;
  bVar5 = false;
  local_4c = 0;
  uVar8 = 0;
  while( true ) {
    if (!bVar5) goto LAB_1412e4360;
    while (unaff_R15 == 0) {
LAB_1412e4360:
      if ((puVar7 == (ulonglong *)0x0) || (puVar7 == puVar1)) {
        if (local_4c <= uVar8 && uVar8 - local_4c != 0) {
          if (local_58 == 0) {
            alloc::raw_vec::RawVec::
            grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                      ();
          }
          *(uint *)(local_58 * 8 + 4) = local_4c;
          *(uint *)(local_58 * 8 + 8) = uVar8 - local_4c;
          local_60 = local_58 + 1;
        }
        in_RCX[2] = local_60;
        *in_RCX = 0;
        in_RCX[1] = 4;
        return;
      }
      unaff_R14D = iVar6 << 6;
      unaff_R15 = *puVar7;
      puVar7 = puVar7 + 1;
      iVar6 = iVar6 + 1;
    }
    iVar3 = 0;
    for (uVar4 = unaff_R15; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x8000000000000000) {
      iVar3 = iVar3 + 1;
    }
    uVar9 = iVar3 + unaff_R14D;
    if (*(ulonglong *)(in_RDX + 0x28) <= (ulonglong)uVar9) break;
    uVar10 = uVar9 * 0x20;
    bVar2 = *(byte *)(*(longlong *)(in_RDX + 0x20) + (ulonglong)uVar9);
    uVar9 = local_4c;
    if ((uVar8 != uVar10) && (uVar9 = uVar10, local_4c <= uVar8 && uVar8 - local_4c != 0)) {
      if (local_58 == 0) {
        alloc::raw_vec::RawVec::
        grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                  ();
      }
      *(uint *)(local_58 * 8 + 4) = local_4c;
      *(uint *)(local_58 * 8 + 8) = uVar8 - local_4c;
      local_60 = local_58 + 1;
      local_58 = local_60;
    }
    local_4c = uVar9;
    unaff_R15 = unaff_R15 & unaff_R15 - 1;
    uVar8 = uVar10 + bVar2;
    bVar5 = true;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}


