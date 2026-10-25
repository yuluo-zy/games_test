
void __cdecl
country_core::game_config::_::impl$0::deserialize::impl$0::visit_str<enum2_<ron::error::Error>_>
          (void)

{
  undefined1 *in_RCX;
  ulonglong uVar1;
  uint uVar2;
  ulonglong *in_RDX;
  undefined8 in_R8;
  ulonglong uVar3;
  
  switch(in_R8) {
  case 2:
    if ((short)*in_RDX == 0x6975) {
      in_RCX[1] = 2;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 3:
    if (*(char *)((longlong)in_RDX + 2) == 'v' && (short)*in_RDX == 0x6564) {
      in_RCX[1] = 3;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 4:
    if ((int)*in_RDX == 0x656d6167) {
      in_RCX[1] = 4;
      *in_RCX = 0x32;
      return;
    }
    if ((int)*in_RDX == 0x676e616c) {
      in_RCX[1] = 5;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 5:
    if (*(char *)((longlong)in_RDX + 4) == 'o' && (int)*in_RDX == 0x65646976) {
      in_RCX[1] = 0;
      *in_RCX = 0x32;
      return;
    }
    if (*(char *)((longlong)in_RDX + 4) == 'o' && (int)*in_RDX == 0x69647561) {
      in_RCX[1] = 1;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0xd:
    in_RCX[1] = '\x06' - (*(longlong *)((longlong)in_RDX + 5) == 0x656469727265766f &&
                         *in_RDX == 0x65766f5f676e616c);
    *in_RCX = 0x32;
    return;
  case 0xe:
    uVar1 = 0x6f76657277726974;
    uVar3 = *in_RDX;
    uVar3 = uVar3 >> 0x38 | (uVar3 & 0xff000000000000) >> 0x28 | (uVar3 & 0xff0000000000) >> 0x18 |
            (uVar3 & 0xff00000000) >> 8 | (uVar3 & 0xff000000) << 8 | (uVar3 & 0xff0000) << 0x18 |
            (uVar3 & 0xff00) << 0x28 | uVar3 << 0x38;
    if (uVar3 == 0x6f76657277726974) {
      uVar1 = 0x6974655f6c616e67;
      uVar3 = *(ulonglong *)((longlong)in_RDX + 6);
      uVar3 = uVar3 >> 0x38 | (uVar3 & 0xff000000000000) >> 0x28 | (uVar3 & 0xff0000000000) >> 0x18
              | (uVar3 & 0xff00000000) >> 8 | (uVar3 & 0xff000000) << 8 | (uVar3 & 0xff0000) << 0x18
              | (uVar3 & 0xff00) << 0x28 | uVar3 << 0x38;
      uVar2 = 0;
      if (uVar3 == 0x6974655f6c616e67) goto code_r0x0001402e7e66;
    }
    uVar2 = -(uint)(uVar3 < uVar1) | 1;
code_r0x0001402e7e66:
    in_RCX[1] = '\x06' - (uVar2 == 0);
    *in_RCX = 0x32;
    return;
  }
  in_RCX[1] = 6;
  *in_RCX = 0x32;
  return;
}

