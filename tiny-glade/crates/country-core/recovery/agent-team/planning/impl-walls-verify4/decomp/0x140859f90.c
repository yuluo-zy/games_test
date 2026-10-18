
void __cdecl country_core::resources::walls::inner_walls::InnerWalls::new(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar7;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(undefined8 *)(extraout_RAX + 0x10) = in_RDX;
    lVar7 = extraout_RAX_00;
  }
  else {
    lVar7 = *(longlong *)(extraout_RAX + 8);
    in_RDX = *(undefined8 *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar7 + 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar1 = *extraout_RAX_01;
  lVar2 = extraout_RAX_01[1];
  *extraout_RAX_01 = *extraout_RAX_01 + 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar3 = *extraout_RAX_02;
  lVar4 = extraout_RAX_02[1];
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_03 != (longlong *)0x0) {
    lVar5 = *extraout_RAX_03;
    lVar6 = extraout_RAX_03[1];
    *extraout_RAX_03 = *extraout_RAX_03 + 1;
    in_RCX[4] = lVar7;
    in_RCX[5] = in_RDX;
    in_RCX[2] = 0;
    in_RCX[3] = 0;
    *in_RCX = &anon_6a7339a970014eecaf19421d4b3f97a6_0_llvm_4089674097218605020;
    in_RCX[1] = 0;
    in_RCX[0x18] = 0;
    in_RCX[10] = lVar1;
    in_RCX[0xb] = lVar2;
    in_RCX[8] = 0;
    in_RCX[9] = 0;
    in_RCX[6] = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    in_RCX[7] = 0;
    in_RCX[0xc] = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    in_RCX[0xd] = 0;
    in_RCX[0xe] = 0;
    in_RCX[0xf] = 0;
    in_RCX[0x10] = lVar3;
    in_RCX[0x11] = lVar4;
    in_RCX[0x12] = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    in_RCX[0x13] = 0;
    in_RCX[0x14] = 0;
    in_RCX[0x15] = 0;
    in_RCX[0x16] = lVar5;
    in_RCX[0x17] = lVar6;
    return;
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}

