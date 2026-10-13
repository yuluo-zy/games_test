// ADDRESS: 0x140a07390
// SIZE: 166
// SIGNATURE: void __cdecl new(void)

void __cdecl country_core::render::mesh::Mesh::new(void)

{
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar1;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  undefined8 uVar2;
  undefined8 in_R8;
  
  uVar2 = in_RDX;
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
    *(undefined8 *)(extraout_RAX + 0x10) = uVar2;
    lVar1 = extraout_RAX_00;
  }
  else {
    lVar1 = *(longlong *)(extraout_RAX + 8);
    uVar2 = *(undefined8 *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar1 + 1;
  in_RCX[9] = 0;
  in_RCX[10] = 0;
  in_RCX[7] = &anon_cc634b371bd8d6b6c1e8ab9dcbca5d5b_40_llvm_9402988993337875171;
  in_RCX[8] = 0;
  in_RCX[0xb] = lVar1;
  in_RCX[0xc] = uVar2;
  *in_RCX = 0;
  in_RCX[1] = 4;
  in_RCX[2] = 0;
  in_RCX[3] = 0x8000000000000000;
  in_RCX[0xd] = in_RDX;
  in_RCX[0xe] = in_R8;
  return;
}

