// 0x1401589f0 format<tiny_glade::gui::settings::RefreshRateMillihertz>
// signature: void __cdecl format<tiny_glade::gui::settings::RefreshRateMillihertz>(void)

void __cdecl
toolbar::mei::widgets::tweaker::impl$8::format<tiny_glade::gui::settings::RefreshRateMillihertz>
          (void)

{
  char extraout_AL;
  undefined8 *in_RCX;
  longlong in_RDX;
  
  if (*(ulonglong *)(in_RDX + 8) <= **(ulonglong **)(in_RDX + 0x10)) {
                    // WARNING: Subroutine does not return
    ::core::panicking::panic_bounds_check();
  }
  ::core::fmt::write();
  if (extraout_AL == '\0') {
    in_RCX[2] = 0;
    *in_RCX = 0;
    in_RCX[1] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  ::core::result::unwrap_failed();
}

