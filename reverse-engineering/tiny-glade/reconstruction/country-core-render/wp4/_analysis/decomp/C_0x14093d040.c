// ADDRESS: 0x14093d040
// SIZE: 516
// SIGNATURE: void __cdecl bindless_textures_dset(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset(void)

{
  int *piVar1;
  int iVar2;
  char cVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  undefined *puVar7;
  byte extraout_AL;
  char extraout_AL_00;
  longlong in_RCX;
  byte bVar8;
  bool bVar9;
  undefined1 local_c0 [16];
  undefined *local_b0;
  int *local_a8;
  longlong local_a0;
  longlong local_98;
  longlong local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined1 *local_60;
  int *local_58;
  byte local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  piVar1 = (int *)(in_RCX + 0x80);
  LOCK();
  bVar9 = *(int *)(in_RCX + 0x80) == 0;
  if (bVar9) {
    *(int *)(in_RCX + 0x80) = 0x3fffffff;
  }
  UNLOCK();
  if (!bVar9) {
    std::sys::sync::rwlock::futex::RwLock::write_contended();
  }
  puVar7 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E &
      0x7fffffffffffffff) == 0) {
    bVar8 = 0;
    cVar3 = *(char *)(in_RCX + 0x88);
  }
  else {
    std::panicking::panic_count::is_zero_slow_path();
    bVar8 = extraout_AL ^ 1;
    cVar3 = *(char *)(in_RCX + 0x88);
  }
  if (cVar3 != '\0') {
    uVar6 = (ulonglong)local_a0 >> 8;
    local_a0 = CONCAT71((int7)uVar6,bVar8);
    local_a8 = piVar1;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_58 = piVar1;
  local_50 = bVar8;
  if (*(longlong *)(in_RCX + 0xa0) == 0) {
    if (bVar8 != 0) goto LAB_14093d19a;
    uVar6 = *(ulonglong *)puVar7;
  }
  else {
    uVar4 = *(undefined8 *)(in_RCX + 0xb0);
    uVar5 = *(undefined8 *)(in_RCX + 0xb8);
    bumpalo::Bump::new();
    local_98 = in_RCX + 0x90;
    local_a8 = *(int **)(in_RCX + 0x98);
    local_90 = *(longlong *)(in_RCX + 0xa0);
    *(undefined8 *)(in_RCX + 0xa0) = 0;
    local_a0 = (longlong)local_a8 + local_90 * 8;
    local_88 = 0;
    local_80 = uVar4;
    local_78 = uVar5;
    local_60 = local_c0;
    rhapsody::raw::device::GraphicsDevice::
    update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl_1::update_bindless_texture_table::closure_env_0>_>
              ();
    puVar7 = __imp__ZN7bumpalo11EMPTY_CHUNK17h9156b7dbfe37bdb3E;
    while (local_b0 != puVar7) {
      local_b0 = *(undefined **)(local_b0 + 0x18);
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (bVar8 != 0) goto LAB_14093d19a;
    uVar6 = *(ulonglong *)
             __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
  }
  if (((uVar6 & 0x7fffffffffffffff) != 0) &&
     (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
    *(undefined1 *)(in_RCX + 0x88) = 1;
  }
LAB_14093d19a:
  LOCK();
  iVar2 = *piVar1;
  *piVar1 = *piVar1 + -0x3fffffff;
  UNLOCK();
  if (0x3fffffff < iVar2 + 0xc0000001U) {
    std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
  }
  return;
}

