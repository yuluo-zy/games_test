// 0x140b34f30 to_rgb :: void __cdecl to_rgb(void)

// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::resources::walls::HighlightColor::to_rgb(void)

{
  uint uVar1;
  ulonglong *in_RCX;
  undefined1 in_DL;
  undefined7 in_register_00000011;
  bool bVar2;
  undefined4 uVar3;
  ulonglong uVar4;
  
  bVar2 = *(char *)CONCAT71(in_register_00000011,in_DL) == '\x01';
  uVar3 = 0;
  if (!bVar2) {
    uVar3 = 0x3d8f5c29;
  }
  uVar1 = (uint)bVar2;
  uVar4 = CONCAT44((int)(uVar1 << 0x1f) >> 0x1f,(int)(uVar1 << 0x1f) >> 0x1f);
  *in_RCX = uVar4 & __xmm_00000000000000003cf5c2903dcccccd |
            ~uVar4 & (ulonglong)__xmm_00000000000000003da3d70b3d8f5c29;
  *(undefined4 *)(in_RCX + 1) = uVar3;
  return;
}


