// address: 0x140867850
// pdb name: new
// signature: void __cdecl new(void)

void __cdecl country_core::render::pass_builder::draw_stream::DrawPacket::new(void)

{
  undefined4 extraout_EAX;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar1;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  undefined8 uVar2;
  undefined8 in_R8;
  undefined8 in_R9;
  
  uVar2 = in_RDX;
  rhapsody::raw::shader::impl$7::default();
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
  *(undefined4 *)(in_RCX + 0x15) = extraout_EAX;
  *(undefined2 *)(in_RCX + 0x16) = 0;
  *(undefined4 *)((longlong)in_RCX + 0xac) = 0x3f800000;
  *in_RCX = in_R8;
  in_RCX[1] = in_R9;
  in_RCX[0x14] = in_RDX;
  in_RCX[2] = 0;
  in_RCX[3] = 4;
  in_RCX[4] = 0;
  in_RCX[5] = 0;
  in_RCX[0xe] = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
  in_RCX[0xf] = 0;
  in_RCX[0x10] = 0;
  in_RCX[0x11] = 0;
  in_RCX[0x12] = lVar1;
  in_RCX[0x13] = uVar2;
  in_RCX[8] = 0x100000003;
  in_RCX[6] = 1;
  in_RCX[7] = 0;
  return;
}


