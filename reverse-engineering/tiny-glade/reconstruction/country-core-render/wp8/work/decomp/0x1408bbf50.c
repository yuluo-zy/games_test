
void __cdecl country_core::resources::rocky_terrain::RockyTerrainMesh::load(void)

{
  undefined8 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined1 auVar5 [16];
  ulonglong uVar6;
  longlong extraout_RAX;
  undefined4 *puVar7;
  longlong extraout_RAX_00;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong local_e0;
  undefined8 *local_d8;
  ulonglong local_d0;
  longlong lStack_c8;
  longlong local_c0;
  ulonglong uStack_b8;
  ulonglong local_60;
  ulonglong local_58;
  
  std::fs::read_to_string::inner();
  if (SBORROW8(0,local_e0)) {
    anyhow::error::impl$1::from<std::io::error::Error>();
  }
  else {
    serde_json::de::from_str<country_core::resources::rocky_terrain::RockyTerrainMesh>();
    if (SBORROW8(0,local_e0)) {
      anyhow::error::impl$1::from<serde_json::error::Error>();
      if (local_e0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      auVar5._8_8_ = 0;
      auVar5._0_8_ = local_d0;
      uVar6 = SUB168(auVar5 * ZEXT816(0x30),0);
      if ((SUB168(auVar5 * ZEXT816(0x30),8) != 0) || (0x7ffffffffffffffc < uVar6)) {
LAB_1408bc2fb:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (uVar6 == 0) {
        lVar9 = 4;
        local_60 = 0;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        lVar9 = extraout_RAX;
        local_60 = local_d0;
        if (extraout_RAX == 0) goto LAB_1408bc2fb;
      }
      if (local_d0 != 0) {
        puVar7 = (undefined4 *)(lVar9 + 0x2c);
        do {
          uVar4 = *(undefined4 *)((longlong)local_d8 + 0xc);
          uVar2 = *(undefined4 *)(local_d8 + 2);
          uVar3 = *(undefined4 *)(local_d8 + 4);
          puVar7[-9] = *(undefined4 *)(local_d8 + 1);
          *(undefined8 *)(puVar7 + -0xb) = *local_d8;
          puVar7[-8] = uVar4;
          *(undefined8 *)(puVar7 + -7) = *local_d8;
          puVar7[-5] = *(undefined4 *)(local_d8 + 1);
          puVar7[-4] = uVar2;
          *(undefined8 *)(puVar7 + -3) = *(undefined8 *)((longlong)local_d8 + 0x14);
          puVar7[-1] = *(undefined4 *)((longlong)local_d8 + 0x1c);
          *puVar7 = uVar3;
          puVar7 = puVar7 + 0xc;
          local_d8 = (undefined8 *)((longlong)local_d8 + 0x24);
          local_d0 = local_d0 - 1;
        } while (local_d0 != 0);
      }
      asset_libraries::ssbo_library::AssetSsboLibrary::
      add_new_with_data<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO>();
      if (0x7ffffffffffffffc < uStack_b8 << 5) {
LAB_1408bc317:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (uStack_b8 << 5 == 0) {
        lVar9 = 4;
        local_58 = 0;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == 0) goto LAB_1408bc317;
        local_58 = uStack_b8;
        lVar9 = extraout_RAX_00;
      }
      if (uStack_b8 != 0) {
        if (uStack_b8 == 1) {
          uVar6 = 0;
        }
        else {
          lVar8 = 0;
          uVar6 = 0;
          do {
            uVar4 = *(undefined4 *)(local_c0 + 0xc + lVar8);
            *(undefined4 *)(lVar9 + 8 + lVar8 * 2) = *(undefined4 *)(local_c0 + 8 + lVar8);
            *(undefined8 *)(lVar9 + lVar8 * 2) = *(undefined8 *)(local_c0 + lVar8);
            *(undefined4 *)(lVar9 + 0xc + lVar8 * 2) = uVar4;
            puVar1 = (undefined8 *)(lVar9 + 0x10 + lVar8 * 2);
            *puVar1 = 0;
            puVar1[1] = 0;
            uVar4 = *(undefined4 *)(local_c0 + 0x1c + lVar8);
            *(undefined4 *)(lVar9 + 0x28 + lVar8 * 2) = *(undefined4 *)(local_c0 + 0x18 + lVar8);
            *(undefined8 *)(lVar9 + 0x20 + lVar8 * 2) = *(undefined8 *)(local_c0 + 0x10 + lVar8);
            *(undefined4 *)(lVar9 + 0x2c + lVar8 * 2) = uVar4;
            uVar6 = uVar6 + 2;
            puVar1 = (undefined8 *)(lVar9 + 0x30 + lVar8 * 2);
            *puVar1 = 0;
            puVar1[1] = 0;
            lVar8 = lVar8 + 0x20;
          } while ((uStack_b8 & 0xfffffffffffffffe) != uVar6);
        }
        if ((uStack_b8 & 1) != 0) {
          lVar10 = uVar6 * 0x10;
          uVar4 = *(undefined4 *)(local_c0 + 0xc + lVar10);
          lVar8 = uVar6 * 0x20;
          *(undefined4 *)(lVar9 + 8 + lVar8) = *(undefined4 *)(local_c0 + 8 + lVar10);
          *(undefined8 *)(lVar9 + lVar8) = *(undefined8 *)(local_c0 + lVar10);
          *(undefined4 *)(lVar9 + 0xc + lVar8) = uVar4;
          puVar1 = (undefined8 *)(lVar9 + 0x10 + lVar8);
          *puVar1 = 0;
          puVar1[1] = 0;
        }
      }
      asset_libraries::ssbo_library::AssetSsboLibrary::
      add_new_with_data<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO>();
      if (local_58 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_60 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_e0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (lStack_c8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_e0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  return;
}


