
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
code_r0x0001403b5c83:
      in_RCX[1] = 0;
      goto code_r0x0001403b5cc5;
    }
    break;
  case 5:
    if (*(char *)((longlong)in_RDX + 4) == 'h' && (int)*in_RDX == 0x63746970) {
code_r0x0001403b5c28:
      in_RCX[1] = 1;
      goto code_r0x0001403b5cc5;
    }
    break;
  case 8:
    _Buf2 = 
    "positioncamera_rig_arm_lengthpitch_offsetyaw_offsetposition_cache/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs"
    ;
    goto code_r0x0001403b5c54;
  case 9:
    if ((char)in_RDX[1] == 'e' && *in_RDX == 0x686361635f776179) goto code_r0x0001403b5c83;
    break;
  case 10:
    iVar1 = memcmp(in_RDX,
                   "yaw_offsetposition_cache/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs"
                   ,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 5;
      goto code_r0x0001403b5cc5;
    }
    goto code_r0x0001403b5cc2;
  case 0xb:
    if (*(longlong *)((longlong)in_RDX + 3) == 0x65686361635f6863 && *in_RDX == 0x61635f6863746970)
    goto code_r0x0001403b5c28;
    break;
  case 0xc:
    iVar1 = memcmp(in_RDX,&anon_388062fd7c8f4a9c2426742add959578_30_llvm_823095259154403401,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 4;
      goto code_r0x0001403b5cc5;
    }
    break;
  case 0xe:
    _Buf2 = 
    "position_cache/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs"
    ;
code_r0x0001403b5c54:
    iVar1 = memcmp(in_RDX,_Buf2,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 2;
      goto code_r0x0001403b5cc5;
    }
    break;
  case 0x15:
    iVar1 = memcmp(in_RDX,&anon_388062fd7c8f4a9c2426742add959578_29_llvm_823095259154403401,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 3;
      goto code_r0x0001403b5cc5;
    }
  }
code_r0x0001403b5cc2:
  in_RCX[1] = 6;
code_r0x0001403b5cc5:
  *in_RCX = 0;
  return;
}

