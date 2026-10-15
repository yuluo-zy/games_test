// 0x1401588c0 format<ref$<alloc::string::String>_>
// signature: void __cdecl format<ref$<alloc::string::String>_>(void)

void __cdecl toolbar::mei::widgets::tweaker::impl$8::format<ref_<alloc::string::String>_>(void)

{
  char extraout_AL;
  undefined8 *in_RCX;
  longlong in_RDX;
  
  if (*(ulonglong *)(in_RDX + 8) <= **(ulonglong **)(in_RDX + 0x10)) {
                    // WARNING: Subroutine does not return
    ::core::panicking::panic_bounds_check();
  }
  ::core::fmt::impl$20::fmt();
  if (extraout_AL == '\0') {
    in_RCX[2] = 0;
    *in_RCX = 0;
    in_RCX[1] = 1;
    return;
  }
                    // WARNING: Subroutine does not return
  ::core::result::unwrap_failed();
}

