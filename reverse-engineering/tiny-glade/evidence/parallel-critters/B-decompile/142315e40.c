
void __cdecl system_critters::sheep::sheep_behavior::set_sheep_velocities_towards_goals(void)

{
  longlong lVar1;
  longlong lVar2;
  float fVar3;
  char extraout_AL;
  byte *pbVar4;
  longlong *in_RCX;
  ulonglong uVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  ulonglong *puVar11;
  bool bVar12;
  bool bVar13;
  bool bVar14;
  float fVar15;
  undefined8 uVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  longlong local_1a0;
  longlong local_198;
  longlong local_190;
  longlong local_188;
  longlong local_180;
  longlong local_178;
  ulonglong local_170;
  uint *local_118;
  uint *local_110;
  longlong local_f8;
  longlong local_e8;
  longlong local_d8;
  undefined4 local_cc;
  longlong local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a0;
  undefined4 local_94;
  ulonglong local_90;
  ulonglong local_88;
  byte local_80;
  
  lVar1 = *in_RCX;
  lVar2 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::sheep::Sheep>,ref_<country_core::resources::critters::CritterGoal>,ref_mut_<country_core::resources::critters::CritterVelocityTarget>,ref_mut_<country_core::resources::critters::CritterVelocityMultiplier>_>,tuple_<>_>
            ();
  fVar3 = __real_3f800000;
  local_178 = local_e8;
  local_180 = local_d8;
  local_198 = local_c0;
  local_1a0 = local_b8;
  local_188 = local_b0;
  local_190 = local_a0;
  do {
    if ((local_80 & 1) == 0) {
      if (local_88 == local_90) {
        if (local_118 == local_110) {
          return;
        }
        while( true ) {
          local_90 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x100) + 0x58 + (ulonglong)*local_118 * 0xa0);
          if (local_90 != 0) break;
          local_118 = local_118 + 1;
          if (local_118 == local_110) {
            return;
          }
        }
        lVar7 = *(longlong *)(lVar2 + 0x100) + (ulonglong)*local_118 * 0xa0;
        uVar5 = (ulonglong)*(uint *)(lVar7 + 0x94);
        lVar10 = *(longlong *)(lVar2 + 0x1a8) + uVar5 * 0x48;
        uVar5 = *(ulonglong *)(*(longlong *)(lVar2 + 0x1a8) + 0x40 + uVar5 * 0x48);
        if ((*(ulonglong *)(lVar1 + 0x110) < uVar5) &&
           (uVar8 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + *(ulonglong *)(lVar1 + 0x110) * 8)
           , uVar8 != 0)) {
          local_c8 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar8 * 0x30);
          uVar8 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar5 <= uVar8) goto LAB_14231624e;
LAB_142316212:
          uVar8 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + uVar8 * 8);
          if (uVar8 == 0) goto LAB_14231624e;
          local_198 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar8 * 0x30);
          uVar8 = *(ulonglong *)(lVar1 + 0x120);
          lVar6 = *(longlong *)(lVar10 + 0x38);
          if (uVar5 <= uVar8) goto LAB_142316248;
LAB_142316267:
          uVar5 = *(ulonglong *)(lVar6 + uVar8 * 8);
          lVar10 = *(longlong *)(lVar10 + 0x18);
          if (uVar5 == 0) goto LAB_14231628b;
          local_1a0 = *(longlong *)(lVar10 + 0x10 + ~uVar5 * 0x30);
        }
        else {
          local_c8 = 0;
          uVar8 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar8 < uVar5) goto LAB_142316212;
LAB_14231624e:
          local_198 = 0;
          uVar8 = *(ulonglong *)(lVar1 + 0x120);
          lVar6 = *(longlong *)(lVar10 + 0x38);
          if (uVar8 < uVar5) goto LAB_142316267;
LAB_142316248:
          lVar10 = *(longlong *)(lVar10 + 0x18);
LAB_14231628b:
          local_1a0 = 0;
        }
        local_118 = local_118 + 1;
        lVar9 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar1 + 0x128) * 8) * 0x30;
        local_178 = *(longlong *)(lVar10 + 0x10 + lVar9);
        local_180 = *(longlong *)(lVar10 + 0x28 + lVar9);
        lVar6 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar1 + 0x130) * 8) * 0x30;
        local_188 = *(longlong *)(lVar10 + 0x10 + lVar6);
        local_190 = *(longlong *)(lVar10 + 0x28 + lVar6);
        local_f8 = *(longlong *)(lVar7 + 0x50);
        local_88 = 0;
      }
      uVar5 = (ulonglong)*(uint *)(local_f8 + 8 + local_88 * 0x10);
      local_170 = local_88;
    }
    else {
      if (local_88 == local_90) {
        if (local_118 == local_110) {
          return;
        }
        while( true ) {
          local_90 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x1a8) + 0x10 + (ulonglong)*local_118 * 0x48);
          if (local_90 != 0) break;
          local_118 = local_118 + 1;
          if (local_118 == local_110) {
            return;
          }
        }
        lVar10 = *(longlong *)(lVar2 + 0x1a8) + (ulonglong)*local_118 * 0x48;
        uVar5 = *(ulonglong *)(lVar10 + 0x40);
        if ((*(ulonglong *)(lVar1 + 0x110) < uVar5) &&
           (uVar8 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + *(ulonglong *)(lVar1 + 0x110) * 8)
           , uVar8 != 0)) {
          local_c8 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar8 * 0x30);
          uVar8 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar5 <= uVar8) goto LAB_142316155;
LAB_142316119:
          uVar8 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + uVar8 * 8);
          if (uVar8 == 0) goto LAB_142316155;
          local_198 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar8 * 0x30);
          uVar8 = *(ulonglong *)(lVar1 + 0x120);
          lVar7 = *(longlong *)(lVar10 + 0x38);
          if (uVar5 <= uVar8) goto LAB_14231614f;
LAB_14231616e:
          uVar5 = *(ulonglong *)(lVar7 + uVar8 * 8);
          lVar10 = *(longlong *)(lVar10 + 0x18);
          if (uVar5 == 0) goto LAB_142316192;
          local_1a0 = *(longlong *)(lVar10 + 0x10 + ~uVar5 * 0x30);
        }
        else {
          local_c8 = 0;
          uVar8 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar8 < uVar5) goto LAB_142316119;
LAB_142316155:
          local_198 = 0;
          uVar8 = *(ulonglong *)(lVar1 + 0x120);
          lVar7 = *(longlong *)(lVar10 + 0x38);
          if (uVar8 < uVar5) goto LAB_14231616e;
LAB_14231614f:
          lVar10 = *(longlong *)(lVar10 + 0x18);
LAB_142316192:
          local_1a0 = 0;
        }
        local_118 = local_118 + 1;
        lVar6 = ~*(ulonglong *)(lVar7 + *(longlong *)(lVar1 + 0x128) * 8) * 0x30;
        local_178 = *(longlong *)(lVar10 + 0x10 + lVar6);
        local_180 = *(longlong *)(lVar10 + 0x28 + lVar6);
        lVar7 = ~*(ulonglong *)(lVar7 + *(longlong *)(lVar1 + 0x130) * 8) * 0x30;
        local_188 = *(longlong *)(lVar10 + 0x10 + lVar7);
        local_190 = *(longlong *)(lVar10 + 0x28 + lVar7);
        local_88 = 0;
      }
      uVar5 = local_88 & 0xffffffff;
      local_170 = local_88;
    }
    lVar10 = uVar5 * 0x30 + local_c8;
    pbVar4 = (byte *)(local_1a0 + uVar5 * 0x10);
    puVar11 = (ulonglong *)(uVar5 * 0x10 + local_178);
    if (((*pbVar4 & 1) == 0) || (*(char *)(uVar5 * 0x68 + local_198 + 0x30) != '\x01')) {
LAB_1423163d0:
      uVar16 = *(undefined8 *)(lVar10 + 0x10);
      fVar17 = (float)uVar16;
      fVar18 = (float)((ulonglong)uVar16 >> 0x20);
      fVar19 = *(float *)(lVar10 + 0x18);
      fVar15 = fVar19;
    }
    else {
      fVar17 = (float)*(undefined8 *)(pbVar4 + 4);
      fVar18 = (float)((ulonglong)*(undefined8 *)(pbVar4 + 4) >> 0x20);
      fVar19 = *(float *)(pbVar4 + 0xc);
      country_core::resources::critters::sheep::Sheep::being_petted();
      if (extraout_AL != '\0') goto LAB_1423163d0;
      uVar16 = *(undefined8 *)(lVar10 + 0x10);
      fVar15 = *(float *)(lVar10 + 0x18);
    }
    fVar19 = fVar19 - fVar15;
    fVar17 = fVar17 - (float)uVar16;
    fVar18 = fVar18 - (float)((ulonglong)uVar16 >> 0x20);
    fVar15 = fVar3 / SQRT(fVar19 * fVar19 + fVar18 * fVar18 + fVar17 * fVar17);
    bVar14 = -1 < (int)fVar15;
    bVar12 = (int)ABS(fVar15) - 0x800000U < 0x7f000000;
    bVar13 = (int)fVar15 - 1U < 0x7fffff;
    if (bVar13 || bVar12 && bVar14) {
      fVar19 = fVar19 * fVar15;
    }
    else {
      fVar19 = 0.0;
    }
    local_88 = local_170 + 1;
    *(undefined4 *)(local_180 + uVar5 * 4) = local_cc;
    *puVar11 = CONCAT44((int)((uint)(bVar13 || bVar12 && bVar14) << 0x1f) >> 0x1f,
                        (int)((uint)(bVar13 || bVar12 && bVar14) << 0x1f) >> 0x1f) &
               CONCAT44(fVar18 * fVar15,fVar17 * fVar15);
    *(float *)(puVar11 + 1) = fVar19;
    *(undefined4 *)(local_190 + uVar5 * 4) = local_94;
    *(undefined4 *)(local_188 + uVar5 * 4) = 0x3f800000;
  } while( true );
}

