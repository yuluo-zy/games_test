// 0x1401584d0 format<tiny_glade::gui::settings::DisplayMode>
// signature: void __cdecl format<tiny_glade::gui::settings::DisplayMode>(void)

void __cdecl
toolbar::mei::widgets::tweaker::impl$7::format<tiny_glade::gui::settings::DisplayMode>(void)

{
  char extraout_AL;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  if (*(char *)*in_RDX == '\0') {
    lang_lib::lookup();
  }
  else if (*(char *)*in_RDX == '\x01') {
    lang_lib::lookup();
  }
  else {
    lang_lib::lookup();
  }
  ::core::fmt::Formatter::write_str();
  if (extraout_AL == '\0') {
    in_RCX[2] = 0;
    *in_RCX = 0;
    in_RCX[1] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  ::core::result::unwrap_failed();
}

