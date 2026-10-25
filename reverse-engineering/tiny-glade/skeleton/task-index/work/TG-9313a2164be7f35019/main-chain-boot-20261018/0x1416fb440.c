
void __cdecl
country_core::systems::camera_rig::_::impl$0::deserialize::impl$0::
visit_str<serde_json::error::Error>(void)

{
  int iVar1;
  char *_Buf2;
  undefined1 *in_RCX;
  longlong *in_RDX;
  size_t in_R8;
  
  switch(in_R8) {
  case 3:
    if (*(char *)((longlong)in_RDX + 2) == 'w' && (short)*in_RDX == 0x6179) {
code_r0x0001416fb523:
      in_RCX[1] = 0;
      goto code_r0x0001416fb565;
    }
    break;
  case 5:
    if (*(char *)((longlong)in_RDX + 4) == 'h' && (int)*in_RDX == 0x63746970) {
code_r0x0001416fb4c8:
      in_RCX[1] = 1;
      goto code_r0x0001416fb565;
    }
    break;
  case 8:
    _Buf2 = "positiongoalmove_state";
    goto code_r0x0001416fb4f4;
  case 9:
    if ((char)in_RDX[1] == 'e' && *in_RDX == 0x686361635f776179) goto code_r0x0001416fb523;
    break;
  case 10:
    iVar1 = memcmp(in_RDX,"yaw_offsetyaw_cachepitch_cacheposition_cache",in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 5;
      goto code_r0x0001416fb565;
    }
    goto code_r0x0001416fb562;
  case 0xb:
    if (*(longlong *)((longlong)in_RDX + 3) == 0x65686361635f6863 && *in_RDX == 0x61635f6863746970)
    goto code_r0x0001416fb4c8;
    break;
  case 0xc:
    iVar1 = memcmp(in_RDX,&anon_0555c73667d6be1dc866b3f0ae0b3f3f_102_llvm_14020587298740829885,in_R8
                  );
    if (iVar1 == 0) {
      in_RCX[1] = 4;
      goto code_r0x0001416fb565;
    }
    break;
  case 0xe:
    _Buf2 = "position_cache";
code_r0x0001416fb4f4:
    iVar1 = memcmp(in_RDX,_Buf2,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 2;
      goto code_r0x0001416fb565;
    }
    break;
  case 0x15:
    iVar1 = memcmp(in_RDX,&anon_0555c73667d6be1dc866b3f0ae0b3f3f_101_llvm_14020587298740829885,in_R8
                  );
    if (iVar1 == 0) {
      in_RCX[1] = 3;
      goto code_r0x0001416fb565;
    }
  }
code_r0x0001416fb562:
  in_RCX[1] = 6;
code_r0x0001416fb565:
  *in_RCX = 0;
  return;
}

