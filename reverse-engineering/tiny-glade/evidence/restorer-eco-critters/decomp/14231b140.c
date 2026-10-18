// 0x14231b140 ducks_react_to_being_inside_buildings
// SIG: void __cdecl ducks_react_to_being_inside_buildings(void)

void __cdecl
system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings(void)

{
  uint *puVar1;
  longlong lVar2;
  float fVar3;
  float fVar4;
  char cVar5;
  uint uVar6;
  ulonglong *puVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  char extraout_AL;
  longlong extraout_RAX;
  longlong lVar12;
  ulonglong extraout_RAX_00;
  ulonglong uVar13;
  undefined8 *in_RCX;
  longlong *in_RDX;
  ulonglong uVar14;
  uint *puVar15;
  longlong *in_R9;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  longlong lVar19;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar20;
  longlong local_f8;
  longlong local_e0;
  longlong local_d8;
  longlong local_a8;
  
  puVar7 = (ulonglong *)*in_RCX;
  lVar8 = in_RCX[1];
  uVar17 = *puVar7;
  uVar14 = 0;
  uVar18 = uVar17 - *(ulonglong *)(lVar8 + 0x18);
  if (uVar17 < *(ulonglong *)(lVar8 + 0x18)) {
    uVar18 = uVar14;
  }
  uVar13 = uVar17 - *(ulonglong *)(lVar8 + 0x38);
  if (uVar17 < *(ulonglong *)(lVar8 + 0x38)) {
    uVar13 = uVar14;
  }
  local_e0 = uVar18 * 0x10 + *(longlong *)(lVar8 + 8);
  uVar17 = *(ulonglong *)(lVar8 + 0x10) - uVar18;
  if (*(ulonglong *)(lVar8 + 0x10) < uVar18) {
    local_e0 = 8;
    uVar17 = uVar14;
  }
  local_d8 = uVar13 * 0x10 + *(longlong *)(lVar8 + 0x28);
  uVar18 = *(ulonglong *)(lVar8 + 0x30) - uVar13;
  if (*(ulonglong *)(lVar8 + 0x30) < uVar13) {
    local_d8 = 8;
    uVar18 = uVar14;
  }
  *puVar7 = (*(longlong *)(lVar8 + 0x40) - uVar18) - uVar17;
  lVar19 = uVar17 * 0x10 + local_e0;
  lVar16 = uVar18 * 0x10 + local_d8;
  lVar8 = *in_RDX;
  lVar9 = in_RDX[1];
  lVar10 = *in_R9;
  lVar11 = in_R9[1];
LAB_14231b25c:
  do {
    if ((local_e0 == 0) || (local_e0 == lVar19)) {
      if ((local_d8 == 0) || (local_d8 == lVar16)) {
        return;
      }
      local_d8 = local_d8 + 0x10;
      local_e0 = 0;
    }
    else {
      local_e0 = local_e0 + 0x10;
    }
    *puVar7 = *puVar7 + 1;
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  } while ((1 < *(uint *)(extraout_RAX + 600)) || (1.5 < *(float *)(extraout_RAX + 0x280)));
  country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
  puVar15 = *(uint **)(lVar8 + 0xf8);
  puVar1 = puVar15 + *(longlong *)(lVar8 + 0x100);
  cVar5 = *(char *)(lVar8 + 0x108);
  local_a8 = 8;
  local_f8 = 0;
  uVar17 = 0;
  uVar18 = 0;
  do {
    if (cVar5 == '\0') {
      if (uVar18 == uVar17) {
        do {
          if (puVar15 == puVar1) goto LAB_14231b25c;
          uVar6 = *puVar15;
          puVar15 = puVar15 + 1;
          uVar17 = *(ulonglong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar6 * 0xa0);
        } while (uVar17 == 0);
        lVar12 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar6 * 0xa0;
        uVar18 = (ulonglong)*(uint *)(lVar12 + 0x94);
        lVar2 = *(longlong *)(lVar9 + 0x1a8) + uVar18 * 0x48;
        uVar18 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x40 + uVar18 * 0x48);
        if ((*(ulonglong *)(lVar8 + 0x110) < uVar18) &&
           (*(longlong *)(*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0)) {
          uVar14 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar18 <= uVar14) goto LAB_14231b557;
LAB_14231b51e:
          uVar18 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar14 * 8);
          if (uVar18 == 0) goto LAB_14231b557;
          local_f8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
          uVar14 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar14 < uVar18) goto LAB_14231b51e;
LAB_14231b557:
          local_f8 = 0;
        }
        local_a8 = *(longlong *)(lVar12 + 0x50);
        uVar18 = 0;
      }
      uVar14 = (ulonglong)*(uint *)(local_a8 + 8 + uVar18 * 0x10);
    }
    else {
      if (uVar18 == uVar17) {
        do {
          if (puVar15 == puVar1) goto LAB_14231b25c;
          uVar6 = *puVar15;
          puVar15 = puVar15 + 1;
          uVar17 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar6 * 0x48);
        } while (uVar17 == 0);
        lVar2 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar6 * 0x48;
        uVar18 = *(ulonglong *)(lVar2 + 0x40);
        if ((*(ulonglong *)(lVar8 + 0x110) < uVar18) &&
           (*(longlong *)(*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0)) {
          uVar14 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar18 <= uVar14) goto LAB_14231b546;
LAB_14231b4dc:
          uVar18 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar14 * 8);
          if (uVar18 == 0) goto LAB_14231b546;
          local_f8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar18 * 0x30);
        }
        else {
          uVar14 = *(ulonglong *)(lVar8 + 0x118);
          if (uVar14 < uVar18) goto LAB_14231b4dc;
LAB_14231b546:
          local_f8 = 0;
        }
        uVar18 = 0;
      }
      uVar14 = uVar18 & 0xffffffff;
    }
    fVar3 = *(float *)(extraout_RAX + 0x280);
    country_core::resources::walls::public_wall_state::PublicWallState::max_y();
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    fVar20 = 0.0;
    if (((extraout_RAX_00 >> 0x20 != 0) &&
        (uVar13 = extraout_RAX_00 & 0xffffffff, uVar13 < *(ulonglong *)(lVar11 + 0x10))) &&
       (lVar2 = *(longlong *)(lVar11 + 8),
       *(int *)(lVar2 + uVar13 * 0x14) == (int)(extraout_RAX_00 >> 0x20))) {
      uVar6 = *(uint *)(lVar2 + uVar13 * 0x14 + 4);
      uVar13 = (ulonglong)uVar6;
      if (((uVar13 != 0xffffffff) && (uVar13 < *(ulonglong *)(lVar10 + 0x38))) &&
         ((*(ulonglong *)(*(longlong *)(lVar10 + 0x28) + (ulonglong)(uVar6 >> 6) * 8) >>
           (uVar13 & 0x3f) & 1) != 0)) {
        country_core::resources::roofs::roof_shape::Roof::height();
        fVar20 = extraout_XMM0_Da_00;
      }
    }
    uVar18 = uVar18 + 1;
    country_core::resources::glade_border::GladeBorder::is_pos_inside();
    if (((extraout_AL != '\0') &&
        (fVar4 = *(float *)(uVar14 * 0x30 + local_f8 + 0x14), fVar3 <= fVar4)) &&
       (fVar4 <= extraout_XMM0_Da + fVar20)) {
      country_core::resources::audio_resources::AudioInstance::start_if_not_playing();
    }
  } while( true );
}

