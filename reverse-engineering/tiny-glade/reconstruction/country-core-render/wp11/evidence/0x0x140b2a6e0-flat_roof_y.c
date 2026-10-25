// 0x140b2a6e0 flat_roof_y
// sig: void __cdecl flat_roof_y(void)

void __cdecl country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y(void)

{
  longlong in_RCX;
  
  if ((*(byte *)(in_RCX + 0x108) & 1) != 0) {
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

