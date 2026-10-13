// ADDRESS: 0x14093d310
// SIZE: 681
// SIGNATURE: void __cdecl handle_to_bindless_index(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::handle_to_bindless_index(void)

{
  uint *puVar1;
  uint uVar2;
  char cVar3;
  uint uVar4;
  longlong lVar5;
  undefined *puVar6;
  byte extraout_AL;
  char extraout_AL_00;
  longlong in_RCX;
  ulonglong in_RDX;
  byte bVar7;
  bool bVar8;
  
  puVar1 = (uint *)(in_RCX + 0x80);
  uVar4 = *(uint *)(in_RCX + 0x80);
  if (uVar4 < 0x3ffffffe) {
    LOCK();
    uVar2 = *puVar1;
    if (uVar4 == uVar2) {
      *puVar1 = uVar4 + 1;
    }
    UNLOCK();
    if (uVar4 == uVar2) {
      cVar3 = *(char *)(in_RCX + 0x88);
      goto joined_r0x00014093d4cf;
    }
  }
  std::sys::sync::rwlock::futex::RwLock::read_contended();
  cVar3 = *(char *)(in_RCX + 0x88);
joined_r0x00014093d4cf:
  if (cVar3 != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  if ((in_RDX < *(ulonglong *)(in_RCX + 0xb8)) &&
     (*(int *)(*(longlong *)(in_RCX + 0xb0) + in_RDX * 8) != 0)) {
    LOCK();
    uVar4 = *puVar1;
    *puVar1 = *puVar1 - 1;
    UNLOCK();
    if (!SBORROW4(0,uVar4 - 1 & 0xbfffffff)) {
      return;
    }
  }
  else {
    LOCK();
    uVar4 = *puVar1;
    *puVar1 = *puVar1 - 1;
    UNLOCK();
    if (SBORROW4(0,uVar4 - 1 & 0xbfffffff)) {
      std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
      LOCK();
      bVar8 = *puVar1 == 0;
      if (bVar8) {
        *puVar1 = 0x3fffffff;
      }
      UNLOCK();
    }
    else {
      LOCK();
      bVar8 = *puVar1 == 0;
      if (bVar8) {
        *puVar1 = 0x3fffffff;
      }
      UNLOCK();
    }
    if (!bVar8) {
      std::sys::sync::rwlock::futex::RwLock::write_contended();
    }
    puVar6 = __imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E;
    if ((*(ulonglong *)__imp__ZN3std9panicking11panic_count18GLOBAL_PANIC_COUNT17h923f4fc2d5dc75a1E
        & 0x7fffffffffffffff) == 0) {
      bVar7 = 0;
      cVar3 = *(char *)(in_RCX + 0x88);
    }
    else {
      std::panicking::panic_count::is_zero_slow_path();
      bVar7 = extraout_AL ^ 1;
      cVar3 = *(char *)(in_RCX + 0x88);
    }
    if (cVar3 != '\0') {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    uVar4 = *(uint *)(in_RCX + 0xc0);
    *(uint *)(in_RCX + 0xc0) = uVar4 + 1;
    if (0x1fdf < uVar4) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    lVar5 = *(longlong *)(in_RCX + 0xa0);
    if (lVar5 == *(longlong *)(in_RCX + 0x90)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(ulonglong *)(*(longlong *)(in_RCX + 0x98) + lVar5 * 8) = in_RDX;
    *(longlong *)(in_RCX + 0xa0) = lVar5 + 1;
    ::utils::impl$4::
    assign_auto_resize<enum2_<core::option::Option<country_core::asset_libraries::texture_library::DescriptorIndex>_>_>
              ();
    if (((bVar7 == 0) && ((*(ulonglong *)puVar6 & 0x7fffffffffffffff) != 0)) &&
       (std::panicking::panic_count::is_zero_slow_path(), extraout_AL_00 == '\0')) {
      *(undefined1 *)(in_RCX + 0x88) = 1;
    }
    LOCK();
    uVar4 = *puVar1;
    *puVar1 = *puVar1 + 0xc0000001;
    UNLOCK();
    if (uVar4 + 0xc0000001 < 0x40000000) {
      return;
    }
  }
  std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
  return;
}

