// name: start_input_recording size: 446
// void __cdecl start_input_recording(void)

void __cdecl country_core::resources::history::History::start_input_recording(void)

{
  longlong lVar1;
  longlong lVar2;
  char extraout_AL;
  undefined8 *extraout_RAX;
  longlong *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong in_RCX;
  
  core::cmp::PartialEq::
  ne<enum2_<country_core::systems::mode_manager::AppMode>,enum2_<country_core::systems::mode_manager::AppMode>_>
            ();
  if (extraout_AL == '\0') {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar1 = *extraout_RAX_00;
  lVar2 = extraout_RAX_00[1];
  *extraout_RAX_00 = *extraout_RAX_00 + 1;
  *extraout_RAX = 1;
  extraout_RAX[1] = 1;
  extraout_RAX[2] = &anon_869b53123b80c82212150de940b3e437_13_llvm_8061242415410628904;
  extraout_RAX[3] = 0;
  extraout_RAX[4] = 0;
  extraout_RAX[5] = 0;
  extraout_RAX[6] = lVar1;
  extraout_RAX[7] = lVar2;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 != (undefined8 *)0x0) {
    *extraout_RAX_01 = 1;
    extraout_RAX_01[1] = 1;
    extraout_RAX_01[2] = 0;
    extraout_RAX_01[4] = 0;
    extraout_RAX_01[6] = 0;
    extraout_RAX_01[7] = 4;
    extraout_RAX_01[8] = 0;
    extraout_RAX_01[9] = 0;
    extraout_RAX_01[10] = 4;
    extraout_RAX_01[0xb] = 0;
    if (!SBORROW8(0,*(longlong *)(in_RCX + 0x30))) {
      core::ptr::drop_in_place<country_core::resources::history::MultiEdit>();
    }
    *(undefined8 *)(in_RCX + 0x40) = 0;
    *(undefined8 *)(in_RCX + 0x30) = 0;
    *(undefined8 *)(in_RCX + 0x38) = 8;
    *(undefined8 **)(in_RCX + 0x48) = extraout_RAX;
    *(undefined8 **)(in_RCX + 0x50) = extraout_RAX_01;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}


