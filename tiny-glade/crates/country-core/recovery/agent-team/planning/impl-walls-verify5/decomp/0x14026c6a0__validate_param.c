// validate_param  size=194
// void __cdecl validate_param(void)

void __cdecl country_core::resources::walls::decorator_subtype::_::impl$0::validate_param(void)

{
  ulonglong uVar1;
  char *pcVar2;
  undefined8 *in_RCX;
  ulonglong in_RDX;
  ulonglong in_R8;
  longlong in_R9;
  
  pcVar2 = "::subtypes";
  if (in_RDX < *(ulonglong *)(in_R9 + 0x218)) {
    uVar1 = *(ulonglong *)(*(longlong *)(in_R9 + 0x210) + in_RDX * 8);
    if ((((uVar1 != 0) && (*(longlong *)(*(longlong *)(in_R9 + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0))
        && (pcVar2 = "::mesh_lib::subtypes", in_R8 < *(ulonglong *)(in_R9 + 0x218))) &&
       ((uVar1 = *(ulonglong *)(*(longlong *)(in_R9 + 0x210) + in_R8 * 8), uVar1 != 0 &&
        (*(longlong *)(*(longlong *)(in_R9 + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
      *in_RCX = 0x8000000000000001;
      return;
    }
  }
  in_RCX[1] = &DAT_1429a6c32;
  in_RCX[2] = 0x17;
  in_RCX[3] = 0x8000000000000000;
  in_RCX[4] = &anon_4a564d73eabec35e9939b64ff0b23625_373_llvm_3143597165263885678;
  in_RCX[5] = 0x50;
  in_RCX[6] = 0x8000000000000000;
  in_RCX[7] = pcVar2;
  in_RCX[8] = 10;
  *(undefined1 *)(in_RCX + 9) = 0;
  *in_RCX = 0x8000000000000000;
  return;
}



// CALLEES: []