// 0x140158b80 format<country_core::render::render_config::ImageQualityPreset>
// signature: void __cdecl format<country_core::render::render_config::ImageQualityPreset>(void)

void __cdecl
toolbar::mei::widgets::tweaker::impl$8::
format<country_core::render::render_config::ImageQualityPreset>(void)

{
  char extraout_AL;
  undefined8 *in_RCX;
  longlong in_RDX;
  
  if (*(ulonglong *)(in_RDX + 8) <= **(ulonglong **)(in_RDX + 0x10)) {
                    // WARNING: Subroutine does not return
    ::core::panicking::panic_bounds_check();
  }
  country_core::render::render_config::impl$2::fmt();
  if (extraout_AL == '\0') {
    in_RCX[2] = 0;
    *in_RCX = 0;
    in_RCX[1] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  ::core::result::unwrap_failed();
}

