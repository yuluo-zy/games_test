
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
  float fVar10;
  float fVar11;
  float fVar12;
  longlong *in_RCX;
  longlong lVar13;
  longlong lVar14;
  longlong *in_RDX;
  longlong lVar15;
  ulonglong uVar16;
  longlong lVar17;
  ulonglong uVar18;
  undefined4 *puVar19;
  ulonglong uVar20;
  uint *puVar21;
  undefined4 extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  float fVar22;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  undefined4 uVar23;
  float fVar24;
  longlong local_140;
  longlong local_138;
  longlong local_120;
  longlong local_118;
  longlong local_108;
  
  fVar12 = __real_3e4ccccd;
  fVar11 = __real_3ecccccd;
  fVar10 = __real_3f800000;
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar21 = *(uint **)(lVar7 + 0xf8);
  puVar1 = puVar21 + *(longlong *)(lVar7 + 0x100);
  cVar5 = *(char *)(lVar7 + 0x108);
  lVar9 = *in_RDX;
  local_138 = 0;
  local_140 = 0;
  local_108 = 8;
  uVar20 = 0;
  uVar16 = 0;
  do {
    if (cVar5 == '\0') {
      if (uVar16 == uVar20) {
        if (puVar21 == puVar1) {
          return;
        }
        while( true ) {
          uVar20 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar21 * 0xa0);
          if (uVar20 != 0) break;
          puVar21 = puVar21 + 1;
          if (puVar21 == puVar1) {
            return;
          }
        }
        lVar13 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar21 * 0xa0;
        uVar16 = (ulonglong)*(uint *)(lVar13 + 0x94);
        lVar17 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x18 + uVar16 * 0x48);
        lVar14 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x38 + uVar16 * 0x48);
        lVar15 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar17 + 0x10 + lVar15);
        local_118 = *(longlong *)(lVar17 + 0x28 + lVar15);
        lVar14 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar17 + 0x10 + lVar14);
        local_120 = *(longlong *)(lVar17 + 0x28 + lVar14);
        local_108 = *(longlong *)(lVar13 + 0x50);
        puVar21 = puVar21 + 1;
        uVar16 = 0;
      }
      uVar18 = (ulonglong)*(uint *)(local_108 + 8 + uVar16 * 0x10);
    }
    else {
      if (uVar16 == uVar20) {
        if (puVar21 == puVar1) {
          return;
        }
        while( true ) {
          uVar20 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar21 * 0x48);
          if (uVar20 != 0) break;
          puVar21 = puVar21 + 1;
          if (puVar21 == puVar1) {
            return;
          }
        }
        lVar17 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar21 * 0x48;
        lVar14 = *(longlong *)(lVar17 + 0x38);
        lVar17 = *(longlong *)(lVar17 + 0x18);
        lVar13 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
        local_140 = *(longlong *)(lVar17 + 0x10 + lVar13);
        local_118 = *(longlong *)(lVar17 + 0x28 + lVar13);
        lVar14 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
        local_138 = *(longlong *)(lVar17 + 0x10 + lVar14);
        local_120 = *(longlong *)(lVar17 + 0x28 + lVar14);
        puVar21 = puVar21 + 1;
        uVar16 = 0;
      }
      uVar18 = uVar16 & 0xffffffff;
    }
    lVar17 = uVar18 * 0x1c + local_140;
    puVar2 = (undefined4 *)(local_118 + uVar18 * 4);
    puVar3 = (undefined4 *)(local_120 + uVar18 * 4);
    puVar19 = (undefined4 *)(uVar18 * 0x10 + local_138);
    fVar22 = *(float *)(lVar9 + 0x28);
    *puVar2 = uVar6;
    fVar22 = fVar22 + *(float *)(lVar17 + 0xc);
    *(float *)(lVar17 + 0xc) = fVar22;
    if (*(char *)(lVar17 + 0x18) == '\x02') {
      fVar24 = 0.0;
      if (0.0 <= fVar22 / fVar11) {
        fVar24 = fVar22 / fVar11;
      }
      fVar22 = fVar10;
      if (fVar24 <= fVar10) {
        fVar22 = fVar24;
      }
      uVar4 = *(undefined8 *)(lVar17 + 0x10);
      ::utils::easing_functions::EasingFunction::apply();
      *puVar3 = uVar6;
      *(ulonglong *)(puVar19 + 2) =
           CONCAT44(extraout_XMM0_Da_01 * (float)((ulonglong)uVar4 >> 0x20),
                    extraout_XMM0_Da_01 * (float)uVar4);
      if ((fVar22 == fVar10) && (!NAN(fVar22) && !NAN(fVar10))) {
        bevy_ecs::system::commands::EntityCommands::despawn();
      }
    }
    else if (*(char *)(lVar17 + 0x18) == '\x01') {
      if (fVar10 <= fVar22) {
        *(undefined1 *)(lVar17 + 0x18) = 2;
        *puVar2 = uVar6;
        *(undefined4 *)(lVar17 + 0xc) = 0;
      }
    }
    else {
      fVar24 = 0.0;
      if (0.0 <= fVar22 / fVar12) {
        fVar24 = fVar22 / fVar12;
      }
      fVar22 = fVar10;
      if (fVar24 <= fVar10) {
        fVar22 = fVar24;
      }
      uVar4 = *(undefined8 *)(lVar17 + 0x10);
      ::utils::easing_functions::EasingFunction::apply();
      *puVar3 = uVar6;
      *(ulonglong *)(puVar19 + 2) =
           CONCAT44(extraout_XMM0_Da_02 * (float)((ulonglong)uVar4 >> 0x20),
                    extraout_XMM0_Da_02 * (float)uVar4);
      if ((fVar22 == fVar10) && (!NAN(fVar22) && !NAN(fVar10))) {
        *(undefined1 *)(lVar17 + 0x18) = 1;
        *puVar2 = uVar6;
        *(undefined4 *)(lVar17 + 0xc) = 0;
      }
    }
    uVar16 = uVar16 + 1;
    country_core::resources::window::WindowSize::as_vec2();
    uVar23 = extraout_XMM0_Da;
    country_core::utils::world_space_to_screen_space();
    *puVar3 = uVar6;
    *puVar19 = extraout_XMM0_Da_00;
    puVar19[1] = uVar23;
  } while( true );
}

