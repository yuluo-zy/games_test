// ADDRESS: 0x14092c920
// SIZE: 579
// SIGNATURE: void __cdecl new(void)

void __cdecl country_core::asset_libraries::mesh_library::AssetMeshLibrary::new(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong *extraout_RAX;
  byte *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar3;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar4;
  undefined8 *extraout_RAX_04;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  undefined8 uVar5;
  uint7 local_78;
  
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar1 = *extraout_RAX;
  lVar2 = extraout_RAX[1];
  *extraout_RAX = *extraout_RAX + 1;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_00 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_00[0] = 1;
    extraout_RAX_00[1] = 0;
    extraout_RAX_00[2] = 0;
    extraout_RAX_00[3] = 0;
    extraout_RAX_00[4] = 0;
    extraout_RAX_00[5] = 0;
    extraout_RAX_00[6] = 0;
    extraout_RAX_00[7] = 0;
    *(longlong *)(extraout_RAX_00 + 8) = extraout_RAX_01;
    *(undefined8 *)(extraout_RAX_00 + 0x10) = in_RDX;
    lVar3 = extraout_RAX_01;
  }
  else {
    lVar3 = *(longlong *)(extraout_RAX_00 + 8);
    in_RDX = *(undefined8 *)(extraout_RAX_00 + 0x10);
  }
  *(longlong *)(extraout_RAX_00 + 8) = lVar3 + 1;
  uVar5 = in_RDX;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_02 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_02[0] = 1;
    extraout_RAX_02[1] = 0;
    extraout_RAX_02[2] = 0;
    extraout_RAX_02[3] = 0;
    extraout_RAX_02[4] = 0;
    extraout_RAX_02[5] = 0;
    extraout_RAX_02[6] = 0;
    extraout_RAX_02[7] = 0;
    *(longlong *)(extraout_RAX_02 + 8) = extraout_RAX_03;
    *(undefined8 *)(extraout_RAX_02 + 0x10) = uVar5;
    lVar4 = extraout_RAX_03;
  }
  else {
    lVar4 = *(longlong *)(extraout_RAX_02 + 8);
    uVar5 = *(undefined8 *)(extraout_RAX_02 + 0x10);
  }
  *(longlong *)(extraout_RAX_02 + 8) = lVar4 + 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_04 != (undefined8 *)0x0) {
    *extraout_RAX_04 = 1;
    extraout_RAX_04[1] = 1;
    *(undefined1 *)(extraout_RAX_04 + 2) = 0;
    *(undefined1 *)((longlong)extraout_RAX_04 + 0x11) = 0;
    extraout_RAX_04[3] = 0;
    extraout_RAX_04[4] = 8;
    extraout_RAX_04[5] = 0;
    in_RCX[10] = lVar1;
    in_RCX[0xb] = lVar2;
    in_RCX[8] = 0;
    in_RCX[9] = 0;
    in_RCX[6] = &anon_a3e9af60763f14b22f2dc37588e49665_32_llvm_1322674437799810495;
    in_RCX[7] = 0;
    in_RCX[4] = 0;
    in_RCX[5] = (ulonglong)local_78 << 8;
    in_RCX[0xc] = 0;
    in_RCX[0xd] = &anon_03061c64e98a3a4ea7442d952ee9ce0d_23_llvm_2292516692454265093;
    in_RCX[0xe] = 0;
    in_RCX[0xf] = 0;
    in_RCX[0x10] = 0;
    in_RCX[0x11] = lVar3;
    in_RCX[0x12] = in_RDX;
    *in_RCX = 0;
    in_RCX[1] = 8;
    in_RCX[2] = 0;
    in_RCX[0x13] = 0;
    in_RCX[0x14] = &anon_03061c64e98a3a4ea7442d952ee9ce0d_23_llvm_2292516692454265093;
    in_RCX[0x15] = 0;
    in_RCX[0x16] = 0;
    in_RCX[0x17] = 0;
    in_RCX[0x18] = lVar4;
    in_RCX[0x19] = uVar5;
    in_RCX[3] = extraout_RAX_04;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

