// 0x140158620 format<enum2$<tiny_glade::gui::settings::FpsCap>_>
// signature: void __cdecl format<enum2$<tiny_glade::gui::settings::FpsCap>_>(void)

void __cdecl
toolbar::mei::widgets::tweaker::impl$8::format<enum2_<tiny_glade::gui::settings::FpsCap>_>(void)

{
  char extraout_AL;
  char extraout_AL_00;
  char cVar1;
  undefined8 *in_RCX;
  longlong *in_RDX;
  
  if ((ulonglong)in_RDX[1] <= *(ulonglong *)in_RDX[2]) {
                    // WARNING: Subroutine does not return
    ::core::panicking::panic_bounds_check();
  }
  if ((*(byte *)(*in_RDX + *(ulonglong *)in_RDX[2] * 8) & 1) == 0) {
    ::core::fmt::num::imp::impl$6::fmt();
    cVar1 = extraout_AL_00;
  }
  else {
    lang_lib::lookup();
    ::core::fmt::Formatter::write_str();
    cVar1 = extraout_AL;
  }
  if (cVar1 == '\0') {
    in_RCX[2] = 0;
    *in_RCX = 0;
    in_RCX[1] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  ::core::result::unwrap_failed();
}

