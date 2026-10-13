// addr=0x140a596b0 name=start_if_not_playing
// signature=void __cdecl start_if_not_playing(void)


void __cdecl country_core::utils::audio_utils::AudioLoop::start_if_not_playing(void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  longlong in_RCX;
  undefined8 local_a0;
  
  *(undefined4 *)(in_RCX + 0x20) = 1000000000;
  if (*(char *)(in_RCX + 0x34) == '\0') {
    uVar1 = *(ulonglong *)(in_RCX + 0x10);
    uVar2 = *(ulonglong *)(in_RCX + 0x28);
    if (*(char *)(in_RCX + 0x35) != '\0') {
      if (uVar1 <= uVar2) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      resources::audio_resources::impl$10::set_parameter();
    }
    if (uVar1 <= uVar2) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (*(int *)(*(longlong *)(in_RCX + 8) + uVar2 * 0x30) == 1) {
      libfmod::EventInstance::start();
      if (local_a0 != -0x7ffffffffffffffc) {
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
    }
    *(undefined1 *)(in_RCX + 0x34) = 1;
  }
  return;
}


