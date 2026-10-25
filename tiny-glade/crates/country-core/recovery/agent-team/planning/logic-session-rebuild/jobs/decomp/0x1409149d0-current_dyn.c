// 0x1409149d0 current_dyn
// signature: void __cdecl current_dyn(void)


void __cdecl country_core::systems::camera_rig::CameraModeStates::current_dyn(void)

{
  longlong *in_RCX;
  undefined **ppuVar1;
  longlong in_RDX;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  
  ppuVar1 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_19_llvm_14020587298740829885;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch(*(undefined1 *)(in_RDX + 0x3d0)) {
  case 0:
    lVar3 = 0x70;
    lVar4 = 0x8c;
    lVar2 = in_RDX;
    break;
  case 1:
    lVar3 = 0x128;
    lVar4 = 0x144;
    lVar2 = in_RDX + 0xb8;
    break;
  case 2:
    ppuVar1 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_17_llvm_14020587298740829885;
    lVar3 = 0x1a8;
    lVar4 = 0x1c4;
    lVar2 = in_RDX + 0x170;
    break;
  case 3:
    ppuVar1 = (undefined **)&anon_0555c73667d6be1dc866b3f0ae0b3f3f_16_llvm_14020587298740829885;
    lVar3 = 0x248;
    lVar4 = 0x264;
    lVar2 = in_RDX + 0x1f0;
    break;
  case 4:
    lVar3 = 0x300;
    lVar4 = 0x31c;
    lVar2 = in_RDX + 0x290;
    break;
  case 5:
    ppuVar1 = &anon_0555c73667d6be1dc866b3f0ae0b3f3f_18_llvm_14020587298740829885;
    lVar3 = 0x388;
    lVar4 = 0x3a4;
    lVar2 = in_RDX + 0x348;
  }
  *in_RCX = lVar2;
  in_RCX[1] = (longlong)ppuVar1;
  in_RCX[2] = lVar4 + in_RDX;
  in_RCX[3] = lVar3 + in_RDX;
  return;
}

