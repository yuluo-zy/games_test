
void __cdecl country_core::resources::gui::impl$9::default(void)

{
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  undefined8 in_RDX;
  longlong lVar1;
  undefined8 local_d8 [4];
  undefined8 local_b8;
  undefined8 local_98;
  undefined8 local_78;
  undefined8 local_58;
  undefined8 local_38;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  local_d8[0] = 0x8000000000000000;
  local_b8 = 0x8000000000000000;
  local_98 = 0x8000000000000000;
  local_78 = 0x8000000000000000;
  local_58 = 0x8000000000000000;
  local_38 = 0;
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
    lVar1 = extraout_RAX_00;
  }
  else {
    lVar1 = *(longlong *)(extraout_RAX + 8);
    in_RDX = *(undefined8 *)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar1 + 1;
  *(undefined1 *)((longlong)in_RCX + 0x102) = 0;
  *(undefined4 *)(in_RCX + 0x1e) = 0;
  *(undefined1 *)((longlong)in_RCX + 0xf4) = 0;
  *(undefined2 *)((longlong)in_RCX + 0xf5) = 0;
  in_RCX[0x1d] = 0x3fc000003f800000;
  *in_RCX = 0;
  *(undefined1 *)((longlong)in_RCX + 0xf7) = 1;
  memcpy(in_RCX + 2,local_d8,0xa8);
  *(undefined1 *)(in_RCX + 0x1f) = 0;
  *(undefined2 *)((longlong)in_RCX + 0x103) = 0;
  in_RCX[0x17] = &DAT_142ab24e0;
  in_RCX[0x18] = 0;
  in_RCX[0x19] = 0;
  in_RCX[0x1a] = 0;
  in_RCX[0x1b] = lVar1;
  in_RCX[0x1c] = in_RDX;
  *(undefined8 *)((longlong)in_RCX + 0xf9) = 0;
  *(undefined1 *)((longlong)in_RCX + 0x101) = 0;
  return;
}


