// address 0x1409917d0
// name new
// signature void __cdecl new(void)

// WARNING: Function: __chkstk replaced with injection: alloca_probe

void __cdecl country_core::render::nani::gpu_world::GpuWorld::new(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong extraout_RAX;
  undefined8 *extraout_RAX_00;
  void *in_RCX;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  undefined1 local_15a8 [5184];
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  longlong local_148;
  ulonglong uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 uStack_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  longlong local_a8;
  ulonglong uStack_a0;
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  ulonglong local_58;
  longlong local_50;
  longlong local_48;
  undefined1 local_3a;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  buffers::nani_calculate_draw_list_params();
  uVar7 = uStack_a0;
  lVar6 = local_a8;
  local_50 = uStack_a0 * 4;
  local_58 = uStack_a0;
  if (uStack_a0 == 0) {
    local_48 = 4;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == 0) {
      local_39 = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_48 = extraout_RAX;
    if (uVar7 < 8 || (ulonglong)(extraout_RAX - lVar6) < 0x20) {
      uVar10 = 0;
    }
    else {
      uVar10 = uVar7 & 0xfffffffffffffff8;
      uVar9 = 0;
      do {
        puVar1 = (undefined8 *)(lVar6 + uVar9 * 4);
        uVar3 = puVar1[1];
        puVar2 = (undefined8 *)(lVar6 + 0x10 + uVar9 * 4);
        uVar4 = *puVar2;
        uVar5 = puVar2[1];
        puVar2 = (undefined8 *)(extraout_RAX + uVar9 * 4);
        *puVar2 = *puVar1;
        puVar2[1] = uVar3;
        puVar1 = (undefined8 *)(extraout_RAX + 0x10 + uVar9 * 4);
        *puVar1 = uVar4;
        puVar1[1] = uVar5;
        uVar9 = uVar9 + 8;
      } while (uVar10 != uVar9);
      if (uVar7 == uVar10) goto LAB_140991907;
    }
    uVar8 = uVar10;
    for (uVar9 = uVar7 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
      *(undefined4 *)(extraout_RAX + uVar8 * 4) = *(undefined4 *)(lVar6 + uVar8 * 4);
      uVar8 = uVar8 + 1;
    }
    if (uVar10 - uVar7 < 0xfffffffffffffffd) {
      do {
        *(undefined4 *)(extraout_RAX + uVar8 * 4) = *(undefined4 *)(lVar6 + uVar8 * 4);
        *(undefined4 *)(extraout_RAX + 4 + uVar8 * 4) = *(undefined4 *)(lVar6 + 4 + uVar8 * 4);
        *(undefined4 *)(extraout_RAX + 8 + uVar8 * 4) = *(undefined4 *)(lVar6 + 8 + uVar8 * 4);
        *(undefined4 *)(extraout_RAX + 0xc + uVar8 * 4) = *(undefined4 *)(lVar6 + 0xc + uVar8 * 4);
        uVar8 = uVar8 + 4;
      } while (uVar7 != uVar8);
    }
  }
LAB_140991907:
  local_3a = 1;
  core::array::impl$34::
  default<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,enum2_<core::option::Option<enum2_<country_core::render::nani::gpu_world::GpuWorldSubset>_>_>_>_>
            ();
  local_108 = local_68;
  uStack_100 = uStack_60;
  local_118 = local_78;
  uStack_110 = uStack_70;
  local_128 = local_88;
  uStack_120 = uStack_80;
  local_138 = local_98;
  uStack_130 = uStack_90;
  local_148 = local_a8;
  uStack_140 = uStack_a0;
  local_158 = local_b8;
  uStack_150 = uStack_b0;
  local_168 = local_c8;
  uStack_160 = uStack_c0;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    extraout_RAX_00[0xc] = local_68;
    extraout_RAX_00[0xd] = uStack_60;
    extraout_RAX_00[10] = local_78;
    extraout_RAX_00[0xb] = uStack_70;
    extraout_RAX_00[8] = local_88;
    extraout_RAX_00[9] = uStack_80;
    extraout_RAX_00[6] = local_98;
    extraout_RAX_00[7] = uStack_90;
    extraout_RAX_00[4] = local_a8;
    extraout_RAX_00[5] = uStack_a0;
    extraout_RAX_00[2] = local_b8;
    extraout_RAX_00[3] = uStack_b0;
    *extraout_RAX_00 = local_c8;
    extraout_RAX_00[1] = uStack_c0;
    ssbo::TypedStorageBuffer::new<country_core::render::nani::mesh_atlas::NaniMeshAtlasPartInfo>();
    uVar7 = local_58;
    ssbo::TypedStorageBuffer::new<country_core::render::nani::gpu_world::NaniDrawListInfo>();
    memcpy(in_RCX,local_15a8,0x1440);
    *(undefined8 *)((longlong)in_RCX + 0x1458) = local_f0;
    *(undefined8 *)((longlong)in_RCX + 0x1460) = uStack_e8;
    *(undefined8 *)((longlong)in_RCX + 0x1468) = local_e0;
    *(undefined8 *)((longlong)in_RCX + 0x1470) = uStack_d8;
    *(undefined8 *)((longlong)in_RCX + 0x1478) = local_d0;
    *(longlong *)((longlong)in_RCX + 0x14a0) = local_148;
    *(undefined8 *)((longlong)in_RCX + 0x1480) = local_168;
    *(undefined8 *)((longlong)in_RCX + 0x1488) = uStack_160;
    *(undefined8 *)((longlong)in_RCX + 0x1490) = local_158;
    *(undefined8 *)((longlong)in_RCX + 0x1498) = uStack_150;
    *(undefined8 **)((longlong)in_RCX + 0x1450) = extraout_RAX_00;
    *(undefined8 *)((longlong)in_RCX + 0x1440) = 0;
    *(undefined8 *)((longlong)in_RCX + 0x1448) = 0;
    if (uVar7 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

