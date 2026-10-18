// wishlist_button @ 0x0x1403d07f0
// sig: void __cdecl wishlist_button(void)
// size: 388


void __cdecl tiny_glade::gui::gui_escape_menu::wishlist_button(void)

{
  char extraout_AL;
  undefined8 local_80;
  
  toolbar::mei::widgets::impl$26::image<ref_<str_>_>();
  lang_lib::lookup();
  alloc::str::impl$5::to_uppercase();
  ::utils::linearstep();
  ::utils::lerp();
  toolbar::mei::widgets::impl$26::menu_button();
  toolbar::mei::widgets::MenuButtonView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  toolbar::mei::widgets::MenuButtonView::max_width();
  toolbar::mei::widgets::MenuButtonView::align();
  toolbar::mei::widgets::impl$37::deref();
  toolbar::mei::core::Node::used();
  if (extraout_AL != '\0') {
    country_core::systems::steam::SteamOverlayOpenResources::
    open_overlay_or_browser_to_webpage<ref_<str_>_>();
  }
  if (local_80 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

