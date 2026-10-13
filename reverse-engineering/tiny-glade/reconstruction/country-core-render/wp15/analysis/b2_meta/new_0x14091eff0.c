// addr=0x14091eff0 name=new
// signature=void __cdecl new(void)


void __cdecl country_core::meta_commander::MetaCommander::new(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar5;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  uVar1 = *in_RDX;
  uVar2 = in_RDX[1];
  uVar3 = in_RDX[2];
  uVar4 = in_RDX[3];
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
    *(undefined8 **)(extraout_RAX + 0x10) = in_RDX;
    lVar5 = extraout_RAX_00;
  }
  else {
    lVar5 = *(longlong *)(extraout_RAX + 8);
    in_RDX = *(undefined8 **)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = lVar5 + 1;
  in_RCX[2] = uVar3;
  in_RCX[3] = uVar4;
  *in_RCX = uVar1;
  in_RCX[1] = uVar2;
  in_RCX[4] = &DAT_142aadc30;
  in_RCX[5] = 0;
  in_RCX[6] = 0;
  in_RCX[7] = 0;
  in_RCX[8] = lVar5;
  in_RCX[9] = in_RDX;
  return;
}


