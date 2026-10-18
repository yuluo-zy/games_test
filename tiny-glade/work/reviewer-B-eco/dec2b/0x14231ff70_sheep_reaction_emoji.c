
void __cdecl
system_critters::sheep::sheep_react_to_being_inside_buildings::sheep_reaction_emoji(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  undefined8 uVar4;
  char cVar5;
  undefined4 uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong *in_RCX;
  longlong lVar10;
  longlong lVar11;
  longlong *in_RDX;
  longlong lVar12;
  ulonglong uVar13;
  longlong lVar14;
  ulonglong uVar15;
  undefined4 *puVar16;
  ulonglong uVar17;
  uint *puVar18;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  float fVar19;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  undefined4 uVar20;
  float fVar21;
  longlong local_140;
  longlong local_138;
  longlong local_120;
  longlong local_118;
  longlong local_108;
  
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar18 = *(uint **)(lVar7 + 0xf8);
  puVar1 = puVar18 + *(longlong *)(lVar7 + 0x100);
  cVar5 = *(char *)(lVar7 + 0x108);
  lVar9 = *in_RDX;
  local_138 = 0;
  local_140 = 0;
  local_108 = 8;
  uVar17 = 0;
  uVar13 = 0;
  do {
    if (cVar5 == '\0') {
      if (uVar13 == uVar17) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar17 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
          if (uVar17 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar10 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar18 * 0xa0;
        uVar13 = (ulonglong)*(uint *)(lVar10 + 0x94);
        lVar14 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x18 + uVar13 * 0x48);
        lVar11 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x38 + uVar13 * 0x48);
        lVar12 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar14 + 0x10 + lVar12);
        local_118 = *(longlong *)(lVar14 + 0x28 + lVar12);
        lVar11 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar14 + 0x10 + lVar11);
        local_120 = *(longlong *)(lVar14 + 0x28 + lVar11);
        local_108 = *(longlong *)(lVar10 + 0x50);
        puVar18 = puVar18 + 1;
        uVar13 = 0;
      }
      uVar15 = (ulonglong)*(uint *)(local_108 + 8 + uVar13 * 0x10);
    }
    else {
      if (uVar13 == uVar17) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          uVar17 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar18 * 0x48);
          if (uVar17 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar14 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar18 * 0x48;
        lVar11 = *(longlong *)(lVar14 + 0x38);
        lVar14 = *(longlong *)(lVar14 + 0x18);
        lVar10 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar14 + 0x10 + lVar10);
        local_118 = *(longlong *)(lVar14 + 0x28 + lVar10);
        lVar11 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar14 + 0x10 + lVar11);
        local_120 = *(longlong *)(lVar14 + 0x28 + lVar11);
        puVar18 = puVar18 + 1;
        uVar13 = 0;
      }
      uVar15 = uVar13 & 0xffffffff;
    }
    lVar14 = uVar15 * 0x1c + local_140;
    puVar2 = (undefined4 *)(local_118 + uVar15 * 4);
    puVar3 = (undefined4 *)(local_120 + uVar15 * 4);
    puVar16 = (undefined4 *)(uVar15 * 0x10 + local_138);
    fVar19 = *(float *)(lVar9 + 0x28);
    *puVar2 = uVar6;
    fVar19 = fVar19 + *(float *)(lVar14 + 0xc);
    *(float *)(lVar14 + 0xc) = fVar19;
    if (*(char *)(lVar14 + 0x18) == '\x02') {
      fVar21 = 0.0;
      if (0.0 <= fVar19 / 0.4) {
        fVar21 = fVar19 / 0.4;
      }
      fVar19 = 1.0;
      if (fVar21 <= 1.0) {
        fVar19 = fVar21;
      }
      uVar4 = *(undefined8 *)(lVar14 + 0x10);
      ::utils::easing_functions::EasingFunction::apply();
      *puVar3 = uVar6;
      *(ulonglong *)(puVar16 + 2) =
           CONCAT44(extraout_XMM0_Da_01 * (float)((ulonglong)uVar4 >> 0x20),
                    extraout_XMM0_Da_01 * (float)uVar4);
      if ((fVar19 == 1.0) && (!NAN(fVar19))) {
        bevy_ecs::system::commands::EntityCommands::despawn();
      }
    }
    else if (*(char *)(lVar14 + 0x18) == '\x01') {
      if (1.0 <= fVar19) {
        *(undefined1 *)(lVar14 + 0x18) = 2;
        *puVar2 = uVar6;
        *(undefined4 *)(lVar14 + 0xc) = 0;
      }
    }
    else {
      fVar21 = 0.0;
      if (0.0 <= fVar19 / 0.2) {
        fVar21 = fVar19 / 0.2;
      }
      fVar19 = 1.0;
      if (fVar21 <= 1.0) {
        fVar19 = fVar21;
      }
      uVar4 = *(undefined8 *)(lVar14 + 0x10);
      ::utils::easing_functions::EasingFunction::apply();
      *puVar3 = uVar6;
      *(ulonglong *)(puVar16 + 2) =
           CONCAT44(extraout_XMM0_Da_02 * (float)((ulonglong)uVar4 >> 0x20),
                    extraout_XMM0_Da_02 * (float)uVar4);
      if ((fVar19 == 1.0) && (!NAN(fVar19))) {
        *(undefined1 *)(lVar14 + 0x18) = 1;
        *puVar2 = uVar6;
        *(undefined4 *)(lVar14 + 0xc) = 0;
      }
    }
    uVar13 = uVar13 + 1;
    country_core::resources::window::WindowSize::as_vec2();
    uVar20 = extraout_XMM0_Da;
    country_core::utils::world_space_to_screen_space();
    *puVar3 = uVar6;
    *puVar16 = extraout_XMM0_Da_00;
    puVar16[1] = uVar20;
  } while( true );
}

