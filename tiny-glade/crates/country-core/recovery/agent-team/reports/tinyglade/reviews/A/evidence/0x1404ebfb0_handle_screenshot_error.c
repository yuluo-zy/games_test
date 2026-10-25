// handle_screenshot_error @ 0x1404ebfb0 size=303
// sig: void __cdecl handle_screenshot_error(void)


void __cdecl tiny_glade::render_loop::handle_screenshot_error(void)

{
  longlong in_RCX;
  
  if (in_RCX != 0) {
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
    anyhow::error::impl$6::drop();
  }
  return;
}


