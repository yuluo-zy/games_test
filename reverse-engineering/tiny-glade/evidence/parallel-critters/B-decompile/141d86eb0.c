
void __cdecl system_critters::butterfly::update_butterfly_audio(void)

{
  uint *puVar1;
  uint uVar2;
  longlong lVar3;
  longlong *in_RCX;
  longlong lVar4;
  longlong lVar5;
  uint *puVar6;
  
  lVar5 = *in_RCX;
  lVar3 = in_RCX[1];
  puVar6 = *(uint **)(lVar5 + 0xf8);
  puVar1 = puVar6 + *(longlong *)(lVar5 + 0x100);
  if (*(char *)(lVar5 + 0x108) == '\0') {
    lVar5 = 0;
    lVar4 = 0;
    do {
      if (lVar4 == lVar5) {
        do {
          if (puVar6 == puVar1) {
            return;
          }
          uVar2 = *puVar6;
          puVar6 = puVar6 + 1;
          lVar5 = *(longlong *)(*(longlong *)(lVar3 + 0x100) + 0x58 + (ulonglong)uVar2 * 0xa0);
        } while (lVar5 == 0);
        lVar4 = 0;
      }
      lVar4 = lVar4 + 1;
      country_core::resources::critters::ButterflyAudio::set_position();
    } while( true );
  }
  lVar5 = 0;
  lVar4 = 0;
  do {
    if (lVar4 == lVar5) {
      do {
        if (puVar6 == puVar1) {
          return;
        }
        uVar2 = *puVar6;
        puVar6 = puVar6 + 1;
        lVar5 = *(longlong *)(*(longlong *)(lVar3 + 0x1a8) + 0x10 + (ulonglong)uVar2 * 0x48);
      } while (lVar5 == 0);
      lVar4 = 0;
    }
    lVar4 = lVar4 + 1;
    country_core::resources::critters::ButterflyAudio::set_position();
  } while( true );
}

