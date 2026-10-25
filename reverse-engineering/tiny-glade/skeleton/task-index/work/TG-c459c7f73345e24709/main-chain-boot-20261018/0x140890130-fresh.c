
void __cdecl core::ptr::drop_in_place<country_core::systems::save::load_game::LoadGameManager>(void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  longlong in_RCX;
  
  uVar1 = *(ulonglong *)(in_RCX + 0x18);
  uVar2 = 2;
  if ((uVar1 ^ 0x8000000000000000) < 2) {
    uVar2 = uVar1 ^ 0x8000000000000000;
  }
  if (uVar2 != 0) {
    if (uVar2 == 1) {
      uVar1 = *(ulonglong *)(in_RCX + 0x20);
    }
    else {
      drop_in_place<country_core::systems::save::save_game::GameSaveData>();
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (uVar1 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  drop_in_place<alloc::vec::Vec<enum2_<country_core::systems::save::load_game::PostGameLoadCommand>,alloc::alloc::Global>_>
            ();
  return;
}

