// update_sheep_audio 0x142316490 sig=void __cdecl update_sheep_audio(void)

void __cdecl system_critters::sheep::sheep_behavior::update_sheep_audio(void)

{
  uint *puVar1;
  float fVar2;
  undefined4 uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong *in_RCX;
  ulonglong uVar9;
  longlong *in_RDX;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  uint *puVar15;
  longlong local_78;
  longlong local_48;
  
  lVar4 = *in_RCX;
  lVar5 = in_RCX[1];
  uVar3 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar15 = *(uint **)(lVar4 + 0xf8);
  puVar1 = puVar15 + *(longlong *)(lVar4 + 0x100);
  lVar6 = *in_RDX;
  if (*(char *)(lVar4 + 0x108) == '\0') {
    local_78 = 8;
    lVar11 = 0;
    lVar14 = 0;
    lVar7 = 0;
    do {
      if (lVar7 == lVar14) {
        if (puVar15 == puVar1) {
          return;
        }
        while( true ) {
          lVar14 = *(longlong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*puVar15 * 0xa0);
          if (lVar14 != 0) break;
          puVar15 = puVar15 + 1;
          if (puVar15 == puVar1) {
            return;
          }
        }
        lVar8 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*puVar15 * 0xa0;
        uVar9 = (ulonglong)*(uint *)(lVar8 + 0x94);
        lVar7 = *(longlong *)(lVar5 + 0x1a8);
        lVar11 = lVar7 + uVar9 * 0x48;
        if (*(ulonglong *)(lVar4 + 0x110) < *(ulonglong *)(lVar7 + 0x40 + uVar9 * 0x48)) {
          lVar10 = *(longlong *)(lVar11 + 0x18);
        }
        else {
          lVar10 = *(longlong *)(lVar11 + 0x18);
        }
        puVar15 = puVar15 + 1;
        lVar7 = ~*(ulonglong *)
                  (*(longlong *)(lVar7 + 0x38 + uVar9 * 0x48) + *(longlong *)(lVar4 + 0x118) * 8) *
                0x30;
        lVar11 = *(longlong *)(lVar10 + 0x10 + lVar7);
        local_48 = *(longlong *)(lVar10 + 0x28 + lVar7);
        local_78 = *(longlong *)(lVar8 + 0x50);
        lVar7 = 0;
      }
      uVar9 = (ulonglong)*(uint *)(local_78 + 8 + lVar7 * 0x10);
      lVar8 = uVar9 * 0x38;
      lVar7 = lVar7 + 1;
      country_core::resources::audio_resources::impl$10::set_position();
      fVar2 = *(float *)(lVar6 + 0x28);
      *(undefined4 *)(local_48 + uVar9 * 4) = uVar3;
      *(float *)(lVar11 + 0x34 + lVar8) = fVar2 + *(float *)(lVar11 + 0x34 + lVar8);
    } while( true );
  }
  lVar11 = 0;
  uVar9 = 0;
  uVar13 = 0;
  do {
    if (uVar13 == uVar9) {
      if (puVar15 == puVar1) {
        return;
      }
      while( true ) {
        uVar9 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)*puVar15 * 0x48);
        if (uVar9 != 0) break;
        puVar15 = puVar15 + 1;
        if (puVar15 == puVar1) {
          return;
        }
      }
      lVar11 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*puVar15 * 0x48;
      if (*(ulonglong *)(lVar4 + 0x110) < *(ulonglong *)(lVar11 + 0x40)) {
        lVar14 = *(longlong *)(lVar11 + 0x18);
      }
      else {
        lVar14 = *(longlong *)(lVar11 + 0x18);
      }
      puVar15 = puVar15 + 1;
      lVar7 = ~*(ulonglong *)(*(longlong *)(lVar11 + 0x38) + *(longlong *)(lVar4 + 0x118) * 8) *
              0x30;
      lVar11 = *(longlong *)(lVar14 + 0x10 + lVar7);
      local_78 = *(longlong *)(lVar14 + 0x28 + lVar7);
      uVar13 = 0;
    }
    uVar12 = uVar13 & 0xffffffff;
    lVar14 = uVar12 * 0x38;
    uVar13 = uVar13 + 1;
    country_core::resources::audio_resources::impl$10::set_position();
    fVar2 = *(float *)(lVar6 + 0x28);
    *(undefined4 *)(local_78 + uVar12 * 4) = uVar3;
    *(float *)(lVar11 + 0x34 + lVar14) = fVar2 + *(float *)(lVar11 + 0x34 + lVar14);
  } while( true );
}

