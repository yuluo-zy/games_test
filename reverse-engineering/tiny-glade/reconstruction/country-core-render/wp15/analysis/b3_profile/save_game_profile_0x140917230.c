// addr=0x140917230 name=save_game_profile
// signature=void __cdecl save_game_profile(void)


void __cdecl country_core::game_profile::save_game_profile(void)

{
  longlong extraout_RAX;
  
  write_profile_via_tmp();
  if (extraout_RAX != 0) {
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
    anyhow::error::impl$6::drop();
  }
  return;
}


