// default  size=485
// void __cdecl default(void)

void __cdecl country_core::resources::walls::decorator_archivist::impl$1::default(void)

{
  longlong lVar1;
  longlong lVar2;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar3;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar4;
  byte *extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong lVar5;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  undefined8 uVar6;
  undefined8 uVar7;
  
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
    lVar3 = extraout_RAX_00;
  }
  else {
    lVar3 = *(longlong *)(extraout_RAX + 8);
    in_RDX = *(undefined8 *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar3 + 1;
  uVar6 = in_RDX;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_01 != (longlong *)0x0) {
    lVar1 = *extraout_RAX_01;
    lVar2 = extraout_RAX_01[1];
    *extraout_RAX_01 = *extraout_RAX_01 + 1;
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
      *(undefined8 *)(extraout_RAX_02 + 0x10) = uVar6;
      lVar4 = extraout_RAX_03;
    }
    else {
      lVar4 = *(longlong *)(extraout_RAX_02 + 8);
      uVar6 = *(undefined8 *)(extraout_RAX_02 + 0x10);
    }
    *(longlong *)(extraout_RAX_02 + 8) = lVar4 + 1;
    uVar7 = uVar6;
    std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
    if ((*extraout_RAX_04 & 1) == 0) {
      std::sys::random::hashmap_random_keys();
      extraout_RAX_04[0] = 1;
      extraout_RAX_04[1] = 0;
      extraout_RAX_04[2] = 0;
      extraout_RAX_04[3] = 0;
      extraout_RAX_04[4] = 0;
      extraout_RAX_04[5] = 0;
      extraout_RAX_04[6] = 0;
      extraout_RAX_04[7] = 0;
      *(longlong *)(extraout_RAX_04 + 8) = extraout_RAX_05;
      *(undefined8 *)(extraout_RAX_04 + 0x10) = uVar7;
      lVar5 = extraout_RAX_05;
    }
    else {
      lVar5 = *(longlong *)(extraout_RAX_04 + 8);
      uVar7 = *(undefined8 *)(extraout_RAX_04 + 0x10);
    }
    *(longlong *)(extraout_RAX_04 + 8) = lVar5 + 1;
    in_RCX[4] = lVar3;
    in_RCX[5] = in_RDX;
    in_RCX[2] = 0;
    in_RCX[3] = 0;
    *in_RCX = &DAT_1429c2e40;
    in_RCX[1] = 0;
    in_RCX[0x18] = 0;
    in_RCX[0x2a] = 0;
    in_RCX[6] = &anon_a85850e2b41faa58b5d70f5d3927ab67_1_llvm_4557221853777664021;
    in_RCX[7] = 0;
    in_RCX[8] = 0;
    in_RCX[9] = 0;
    in_RCX[10] = lVar1;
    in_RCX[0xb] = lVar2;
    *(undefined1 *)(in_RCX + 0x3c) = 0;
    in_RCX[0xc] = &DAT_1429c2e40;
    in_RCX[0xd] = 0;
    in_RCX[0xe] = 0;
    in_RCX[0xf] = 0;
    in_RCX[0x10] = lVar4;
    in_RCX[0x11] = uVar6;
    in_RCX[0x12] = &DAT_1429c2e40;
    in_RCX[0x13] = 0;
    in_RCX[0x14] = 0;
    in_RCX[0x15] = 0;
    in_RCX[0x16] = lVar5;
    in_RCX[0x17] = uVar7;
    return;
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}



// CALLEES: [{'name': 'VAL', 'address': '140d4d740'}, {'name': 'call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>', 'address': '1400d9b20'}, {'name': 'hashmap_random_keys', 'address': '140d64e00'}, {'name': 'panic_access_error', 'address': '1428d80d0'}]