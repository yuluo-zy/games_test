// boxes 0x14157ce40 size=67
// signature: void __cdecl boxes(void)


void __cdecl
country_core::asset_libraries::audio_trigger_library::AudioTriggerCollection::boxes(void)

{
  uint uVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *in_RCX;
  longlong in_RDX;
  longlong lVar4;
  
  lVar2 = *(longlong *)(in_RDX + 8);
  lVar3 = *(longlong *)(in_RDX + 0x10);
  lVar4 = 0x1c;
  if (lVar3 == 0) {
    lVar4 = 0;
  }
  uVar1 = *(uint *)(in_RDX + 0x1c);
  *in_RCX = lVar4 + lVar2;
  in_RCX[1] = lVar3 * 0x1c + lVar2;
  in_RCX[2] = (ulonglong)(lVar3 != 0);
  in_RCX[3] = (ulonglong)uVar1;
  return;
}

