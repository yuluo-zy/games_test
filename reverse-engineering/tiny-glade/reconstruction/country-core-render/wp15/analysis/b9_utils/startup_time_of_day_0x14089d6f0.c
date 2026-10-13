// addr=0x14089d6f0 name=startup_time_of_day
// signature=void __cdecl startup_time_of_day(void)


void __cdecl country_core::startup::startup_time_of_day::startup_time_of_day(void)

{
  longlong *plVar1;
  longlong *in_RCX;
  longlong local_50;
  longlong lStack_48;
  longlong local_40;
  
  std::fs::read_to_string::inner();
  if (SBORROW8(0,local_50)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  serde_json::de::from_str<country_core::resources::time_of_day::TodSequence>();
  if (!SBORROW8(0,local_50)) {
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    plVar1 = (longlong *)*in_RCX;
    if (*plVar1 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar1[2] = local_40;
    *plVar1 = local_50;
    plVar1[1] = lStack_48;
    if (local_50 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      return;
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


