
void __cdecl rhapsody::raw::swapchain::Swapchain::present(void)

{
  byte extraout_AL;
  longlong in_RCX;
  longlong in_RDX;
  
  if (*(short *)(in_RCX + 0x14) != *(short *)(*(longlong *)(in_RDX + 0x648) + 0xa50)) {
                    // WARNING: Subroutine does not return
    core::panicking::
    assert_failed<rhapsody::raw::managed::handle::RuntimeDeviceId,rhapsody::raw::managed::handle::RuntimeDeviceId>
              ();
  }
  if (*(char *)(in_RCX + 0x16) == '\x02') {
    if (*(int *)(*(longlong *)(in_RCX + 8) + 0xc) == *(int *)(in_RCX + 0x10)) {
      managed::swapchain::ManagedSwapchain::present();
      *(byte *)(in_RCX + 0x1c) = *(byte *)(in_RCX + 0x1c) | extraout_AL;
      return;
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::panicking::
  assert_failed<rhapsody::raw::managed::handle::ManagedResourceTypeId,rhapsody::raw::managed::handle::ManagedResourceTypeId>
            ();
}

