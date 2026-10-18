// validate_param  size=199
// void __cdecl validate_param(void)

void __cdecl country_core::resources::walls::decorator_archivist::_::impl$0::validate_param(void)

{
  ulonglong uVar1;
  undefined8 uVar2;
  undefined8 *in_RCX;
  ulonglong in_RDX;
  undefined8 uVar3;
  ulonglong in_R8;
  longlong in_R9;
  undefined *puVar4;
  
  uVar2 = 6;
  puVar4 = &DAT_1429dc225;
  if (in_RDX < *(ulonglong *)(in_R9 + 0x218)) {
    uVar1 = *(ulonglong *)(*(longlong *)(in_R9 + 0x210) + in_RDX * 8);
    if ((uVar1 != 0) && (*(longlong *)(*(longlong *)(in_R9 + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)) {
      uVar2 = 0x15;
      puVar4 = &DAT_1429dc210;
      if ((in_R8 < *(ulonglong *)(in_R9 + 0x218)) &&
         ((uVar1 = *(ulonglong *)(*(longlong *)(in_R9 + 0x210) + in_R8 * 8), uVar1 != 0 &&
          (*(longlong *)(*(longlong *)(in_R9 + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)))) {
        uVar3 = 0x8000000000000001;
        goto LAB_1403f7bb1;
      }
    }
  }
  in_RCX[1] = &DAT_1429d62a0;
  in_RCX[2] = 0x17;
  uVar3 = 0x8000000000000000;
  in_RCX[3] = 0x8000000000000000;
  in_RCX[4] = &anon_4a564d73eabec35e9939b64ff0b23625_460_llvm_3143597165263885678;
  in_RCX[5] = 0x4f;
  in_RCX[6] = 0x8000000000000000;
  in_RCX[7] = puVar4;
  in_RCX[8] = uVar2;
  *(undefined1 *)(in_RCX + 9) = 0;
LAB_1403f7bb1:
  *in_RCX = uVar3;
  return;
}



// CALLEES: []