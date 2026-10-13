// name: close_file size: 254
// void __cdecl close_file(void)

void __cdecl country_core::systems::save::save_game::AutoSaveState::close_file(void)

{
  longlong in_RCX;
  undefined4 local_a8;
  
  std::sync::mpmc::Sender::send<enum2_<country_core::systems::save::save_game::AsyncSaveCmd>_>();
  if (local_a8 == 0xb) {
    *(undefined8 *)(in_RCX + 0x120) = 0;
    if (!SBORROW8(0,*(longlong *)(in_RCX + 0x28))) {
      if ((!SBORROW8(0,*(longlong *)(in_RCX + 0x40))) && (*(longlong *)(in_RCX + 0x40) != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      core::ptr::drop_in_place<country_core::resources::history::HistoryChainSnapshots>();
    }
    *(undefined8 *)(in_RCX + 0x28) = 0x8000000000000000;
    *(undefined4 *)(in_RCX + 0x140) = 0;
    *(undefined8 *)(in_RCX + 0x144) = 0x501502f900000000;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


