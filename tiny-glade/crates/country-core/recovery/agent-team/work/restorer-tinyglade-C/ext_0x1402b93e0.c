
void __cdecl
alloc::vec::spec_extend::impl$0::
spec_extend<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env_0>,alloc::vec::into_iter::IntoIter<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env_1>,alloc::alloc::Global>
          (void)

{
  uint uVar1;
  longlong lVar2;
  uint *puVar3;
  longlong lVar4;
  longlong *plVar5;
  ulonglong uVar6;
  uint uVar7;
  longlong *in_RCX;
  longlong lVar8;
  longlong *in_RDX;
  longlong *plVar9;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong lVar14;
  longlong local_80;
  longlong local_78;
  longlong *local_70;
  uint *local_68;
  longlong local_60;
  longlong local_58;
  longlong *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar11 = *in_RDX;
  local_50 = in_RDX;
  local_70 = in_RCX;
joined_r0x0001402b9418:
  lVar13 = lVar11;
  local_58 = lVar11;
  if (lVar11 != 0) goto LAB_1402b948b;
LAB_1402b9550:
  lVar13 = in_RDX[8];
  local_60 = lVar11;
joined_r0x0001402b9559:
  if (lVar13 != 0) {
    plVar5 = (longlong *)in_RDX[9];
    plVar9 = (longlong *)in_RDX[0xb];
    if (plVar5 != plVar9) {
      puVar3 = (uint *)in_RDX[0xc];
      local_68 = (uint *)in_RDX[0xd];
LAB_1402b9595:
      in_RCX = local_70;
      lVar13 = *plVar5;
      lVar11 = plVar5[1];
      lVar14 = plVar5[2];
      uVar1 = *(uint *)(plVar5 + 3);
      plVar5 = plVar5 + 4;
      in_RDX[9] = (longlong)plVar5;
      uVar7 = *puVar3;
      if (uVar7 < uVar1) {
        uVar7 = uVar1;
      }
      *puVar3 = uVar7;
      if (*local_68 < uVar1) {
        if (!SBORROW8(0,lVar13)) goto LAB_1402b96ee;
      }
      else {
        if (lVar14 != 0) {
          lVar11 = lVar11 + 0x10;
          do {
            if (*(longlong *)(lVar11 + -8) != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            lVar11 = lVar11 + 0x20;
            lVar14 = lVar14 + -1;
          } while (lVar14 != 0);
        }
        in_RDX = local_50;
        if (lVar13 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      if (plVar5 == plVar9) goto LAB_1402b9630;
      goto LAB_1402b9595;
    }
  }
LAB_1402b9630:
  if (in_RDX[4] == 0) goto LAB_1402b97ed;
  plVar5 = (longlong *)in_RDX[5];
  plVar9 = plVar5;
  if (plVar5 == (longlong *)in_RDX[7]) {
LAB_1402b978f:
    uVar6 = in_RDX[7] - (longlong)plVar9;
    if (uVar6 != 0) {
      uVar6 = uVar6 >> 5;
      plVar9 = plVar9 + 2;
      do {
        if (plVar9[-1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        plVar9 = plVar9 + 4;
        uVar6 = uVar6 - 1;
      } while (uVar6 != 0);
    }
    if (local_50[6] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_50[4] = 0;
LAB_1402b97ed:
    core::ptr::
    drop_in_place<core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env_0>,alloc::vec::into_iter::IntoIter<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env_1>_>
              ();
    return;
  }
  in_RDX[5] = (longlong)(plVar5 + 4);
  lVar14 = *plVar5;
  plVar9 = plVar5 + 4;
  if (lVar14 == 2) goto LAB_1402b978f;
  lVar13 = 0;
  lVar11 = local_60;
  in_RCX = local_70;
  do {
    lVar10 = plVar5[3];
    local_68 = (uint *)plVar5[1];
    local_60 = plVar5[2];
    lVar4 = in_RCX[2];
    if (lVar4 == *in_RCX) {
      uVar6 = (ulonglong)(local_50[3] - local_50[1]) >> 5;
      if (lVar11 == 0) {
        uVar6 = 0;
      }
      uVar12 = (ulonglong)(local_50[7] - local_50[5]) >> 5;
      if (local_50[4] == 0) {
        uVar12 = 0;
      }
      lVar13 = uVar12 + uVar6;
      if ((local_50[8] == 0) || (local_50[0xb] == local_50[9])) {
        plVar5 = &local_78;
      }
      else {
        plVar5 = &local_80;
        lVar13 = 0;
      }
      *plVar5 = lVar13;
      raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      lVar13 = lVar11;
      local_58 = lVar11;
    }
    lVar2 = in_RCX[1];
    lVar8 = lVar4 * 0x20;
    *(longlong *)(lVar2 + lVar8) = lVar14;
    *(uint **)(lVar2 + 8 + lVar8) = local_68;
    *(longlong *)(lVar2 + 0x10 + lVar8) = local_60;
    *(longlong *)(lVar2 + 0x18 + lVar8) = lVar10;
    in_RCX[2] = lVar4 + 1;
    in_RDX = local_50;
    if (lVar13 == 0) goto LAB_1402b9550;
LAB_1402b948b:
    plVar5 = (longlong *)in_RDX[1];
    plVar9 = plVar5;
    if (plVar5 == (longlong *)in_RDX[3]) break;
    plVar9 = plVar5 + 4;
    in_RDX[1] = (longlong)plVar9;
    lVar14 = *plVar5;
  } while (lVar14 != 2);
  uVar6 = in_RDX[3] - (longlong)plVar9;
  if (uVar6 != 0) {
    uVar6 = uVar6 >> 5;
    plVar9 = plVar9 + 2;
    do {
      if (plVar9[-1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar9 = plVar9 + 4;
      uVar6 = uVar6 - 1;
    } while (uVar6 != 0);
  }
  if (in_RDX[2] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *in_RDX = 0;
  local_60 = 0;
  local_58 = 0;
  lVar13 = in_RDX[8];
  goto joined_r0x0001402b9559;
LAB_1402b96ee:
  if (local_58 != 0) {
    uVar6 = in_RDX[3] - in_RDX[1];
    if (uVar6 != 0) {
      uVar6 = uVar6 >> 5;
      lVar10 = in_RDX[1] + 0x10;
      do {
        if (*(longlong *)(lVar10 + -8) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar10 = lVar10 + 0x20;
        uVar6 = uVar6 - 1;
      } while (uVar6 != 0);
    }
    if (in_RDX[2] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  *in_RDX = lVar11;
  in_RDX[1] = lVar11;
  in_RDX[2] = lVar13;
  in_RDX[3] = lVar14 * 0x20 + lVar11;
  goto joined_r0x0001402b9418;
}

