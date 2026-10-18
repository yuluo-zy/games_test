// name=delete_color_icon_entities
// sig=void __cdecl delete_color_icon_entities(void)
// size=611
// address=0x142297ff0

void __cdecl system_color::color_menu::delete_color_icon_entities(void)

{
  uint *puVar1;
  uint uVar2;
  longlong lVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  longlong *in_RCX;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  uint *puVar13;
  
  lVar3 = *in_RCX;
  lVar4 = in_RCX[1];
  puVar13 = *(uint **)(lVar3 + 0xf8);
  puVar1 = puVar13 + *(longlong *)(lVar3 + 0x100);
  if (*(char *)(lVar3 + 0x108) == '\0') {
    lVar10 = 8;
    lVar9 = 0;
    lVar8 = 0;
    lVar11 = 0;
    do {
      if (lVar11 == lVar8) {
        do {
          if (puVar13 == puVar1) {
            return;
          }
          uVar2 = *puVar13;
          puVar13 = puVar13 + 1;
          lVar8 = *(longlong *)(*(longlong *)(lVar4 + 0x100) + 0x58 + (ulonglong)uVar2 * 0xa0);
        } while (lVar8 == 0);
        lVar10 = *(longlong *)(lVar4 + 0x100) + (ulonglong)uVar2 * 0xa0;
        uVar5 = (ulonglong)*(uint *)(lVar10 + 0x94);
        lVar9 = *(longlong *)(lVar4 + 0x1a8);
        if ((*(ulonglong *)(lVar3 + 0x110) < *(ulonglong *)(lVar9 + 0x40 + uVar5 * 0x48)) &&
           (uVar12 = *(ulonglong *)
                      (*(longlong *)(lVar9 + 0x38 + uVar5 * 0x48) +
                      *(ulonglong *)(lVar3 + 0x110) * 8), uVar12 != 0)) {
          lVar9 = *(longlong *)(*(longlong *)(lVar9 + uVar5 * 0x48 + 0x18) + 0x10 + ~uVar12 * 0x30);
        }
        else {
          lVar9 = 0;
        }
        lVar10 = *(longlong *)(lVar10 + 0x50);
        lVar11 = 0;
      }
      lVar6 = lVar11 * 0x10;
      lVar11 = lVar11 + 1;
      if (*(float *)(lVar9 + 0x18 + (ulonglong)*(uint *)(lVar10 + 8 + lVar6) * 0x24) <= 0.0) {
        bevy_ecs::system::commands::EntityCommands::despawn();
      }
    } while( true );
  }
  lVar8 = 0;
  uVar12 = 0;
  uVar5 = 0;
  do {
    if (uVar5 == uVar12) {
      do {
        if (puVar13 == puVar1) {
          return;
        }
        uVar2 = *puVar13;
        puVar13 = puVar13 + 1;
        uVar12 = *(ulonglong *)(*(longlong *)(lVar4 + 0x1a8) + 0x10 + (ulonglong)uVar2 * 0x48);
      } while (uVar12 == 0);
      lVar8 = *(longlong *)(lVar4 + 0x1a8) + (ulonglong)uVar2 * 0x48;
      if ((*(ulonglong *)(lVar3 + 0x110) < *(ulonglong *)(lVar8 + 0x40)) &&
         (uVar5 = *(ulonglong *)(*(longlong *)(lVar8 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
         uVar5 != 0)) {
        lVar8 = *(longlong *)(*(longlong *)(lVar8 + 0x18) + 0x10 + ~uVar5 * 0x30);
      }
      else {
        lVar8 = 0;
      }
      uVar5 = 0;
    }
    uVar7 = uVar5 & 0xffffffff;
    uVar5 = uVar5 + 1;
    if (*(float *)(lVar8 + 0x18 + uVar7 * 0x24) <= 0.0) {
      bevy_ecs::system::commands::EntityCommands::despawn();
    }
  } while( true );
}


