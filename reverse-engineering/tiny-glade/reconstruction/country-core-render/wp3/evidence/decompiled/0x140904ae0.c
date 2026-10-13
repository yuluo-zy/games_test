// address 0x140904ae0
// name nani_draw
// signature void __cdecl nani_draw(void)

void __cdecl country_core::render::nani::draw_ext::impl$0::nani_draw(void)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  byte bVar3;
  longlong lVar4;
  longlong extraout_RAX;
  longlong lVar5;
  longlong extraout_RAX_00;
  void *in_RCX;
  longlong lVar6;
  ulonglong *puVar7;
  byte in_DL;
  ulonglong uVar8;
  undefined1 local_388 [248];
  undefined1 local_290 [248];
  undefined1 local_198 [296];
  ulonglong local_70;
  ulonglong local_68;
  ulonglong *local_60;
  void *local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar4 = *(longlong *)(*(longlong *)((longlong)in_RCX + 0xe0) + 0x1f0);
  local_49 = 1;
  uVar8 = 0x662978458699a5ef;
  local_58 = in_RCX;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX == 0) || (*(ulonglong *)(lVar4 + 0x218) <= uVar8)) ||
     (uVar8 = *(ulonglong *)(*(longlong *)(lVar4 + 0x210) + uVar8 * 8), uVar8 == 0)) {
LAB_140904f39:
    local_49 = 1;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar6 = ~uVar8 * 0x60;
  if (*(longlong *)(*(longlong *)(lVar4 + 0x1e0) + 0x20 + lVar6) == 0) goto LAB_140904f39;
  bVar3 = *(byte *)((longlong)local_58 + 0xf0);
  lVar6 = *(longlong *)(*(longlong *)(lVar4 + 0x1e0) + lVar6 + 0x10);
  local_70 = (ulonglong)in_DL;
  puVar1 = (ulonglong *)((ulonglong)((uint)in_DL * 0xc0) + lVar6);
  uVar8 = *(ulonglong *)(lVar6 + (ulonglong)((uint)in_DL * 0xc0));
  if (uVar8 < 0xfffffffffffffff0 && (char)(((uint)uVar8 & 8) >> 3) == '\0') {
    LOCK();
    uVar2 = *puVar1;
    if (uVar8 == uVar2) {
      *puVar1 = uVar8 + 0x10;
    }
    UNLOCK();
    if (uVar8 == uVar2) goto LAB_140904c07;
  }
  local_49 = 1;
  parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
LAB_140904c07:
  local_49 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_49 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  uVar8 = puVar1[1];
  local_60 = puVar1;
  if (uVar8 == 0x8000000000000001) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  puVar7 = puVar1 + 0x15;
  if (SBORROW8(0,uVar8)) {
    puVar7 = puVar1 + 1;
  }
  local_68 = puVar7[1];
  if (SBORROW8(0,uVar8)) {
    lVar5 = 0;
  }
  else {
    local_4a = 1;
    lVar5 = (**(code **)(puVar1[0x14] + 0x38))(puVar1[0x13]);
  }
  if ((ulonglong)(byte)(in_DL * '\x04' | (byte)(0x302010000 >> ((bVar3 & 7) << 3))) <
      *(ulonglong *)(*(longlong *)(lVar6 + 0x1450) + 0x28)) {
    local_4a = 1;
    uVar8 = 0x8226cc786e679355;
    bevy_ecs::component::Components::get_resource_id();
    if (((extraout_RAX_00 != 0) && (uVar8 < *(ulonglong *)(lVar4 + 0x218))) &&
       ((uVar8 = *(ulonglong *)(*(longlong *)(lVar4 + 0x210) + uVar8 * 8), uVar8 != 0 &&
        (*(longlong *)(*(longlong *)(lVar4 + 0x1e0) + 0x20 + ~uVar8 * 0x60) != 0)))) {
      local_4a = 1;
      asset_libraries::ssbo_library::AssetSsboLibrary::
      get_rg_buffer<country_core::render::nani::buffers::NaniDrawListLengthSsbo>();
      local_4a = 1;
      asset_libraries::ssbo_library::AssetSsboLibrary::
      get_rg_buffer<country_core::render::nani::buffers::NaniDrawListSsbo>();
      memcpy(local_388,local_58,0xf8);
      local_4a = 0;
      pass_builder::draw_builder::impl$2::
      read_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo>();
      if (lVar5 == 0) {
        memcpy(local_198,local_290,0xf8);
      }
      else {
        local_4a = 0;
        pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer();
      }
      if ((NANI_FORCE_FINE_GRAINED_SORTING == '\0') &&
         ((1 < in_DL || (NANI_FORCE_NO_FINE_GRAINED_SORTING != '\0')))) {
        memcpy(local_388,local_198,0xf8);
        local_4a = 0;
        pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect_raw();
      }
      else {
        memcpy(local_388,local_198,0xf8);
        local_4a = 0;
        pass_builder::draw_builder::DrawBuilder::draw_indexed_indirect_count();
      }
      puVar1 = local_60;
      local_49 = 0;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      local_49 = 0;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      uVar8 = *puVar1;
      *puVar1 = *puVar1 - 0x10;
      UNLOCK();
      if ((uVar8 & 0xfffffffffffffff2) == 0x12) {
        local_49 = 0;
        parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
      }
      return;
    }
    local_4a = 1;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_4a = 1;
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}

