// 0x140b2a5d0 get_wall_shape
// sig: void __cdecl get_wall_shape(void)

void __cdecl
country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape(void)

{
  longlong in_RCX;
  
  if (*(uint *)(in_RCX + 600) < 2) {
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

