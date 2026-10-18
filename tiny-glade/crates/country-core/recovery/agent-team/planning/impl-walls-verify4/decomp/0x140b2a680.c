
void __cdecl country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable(void)

{
  longlong in_RCX;
  
  if ((*(byte *)(in_RCX + 0x110) & 1) != 0) {
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic_fmt();
}

