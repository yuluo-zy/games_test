// 0x142315e40 set_sheep_velocities_towards_goals
// SIG: void __cdecl set_sheep_velocities_towards_goals(void)

void __cdecl system_critters::sheep::sheep_behavior::set_sheep_velocities_towards_goals(void)

{
  longlong lVar1;
  longlong lVar2;
  char extraout_AL;
  byte *pbVar3;
  longlong *in_RCX;
  ulonglong uVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  ulonglong *puVar10;
  bool bVar11;
  bool bVar12;
  bool bVar13;
  float fVar14;
  undefined8 uVar15;
  float fVar16;
  float fVar17;
  float fVar18;
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
        lVar6 = *(longlong *)(lVar2 + 0x100) + (ulonglong)*local_118 * 0xa0;
        uVar4 = (ulonglong)*(uint *)(lVar6 + 0x94);
        lVar9 = *(longlong *)(lVar2 + 0x1a8) + uVar4 * 0x48;
        uVar4 = *(ulonglong *)(*(longlong *)(lVar2 + 0x1a8) + 0x40 + uVar4 * 0x48);
        if ((*(ulonglong *)(lVar1 + 0x110) < uVar4) &&
           (uVar7 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + *(ulonglong *)(lVar1 + 0x110) * 8),
           uVar7 != 0)) {
          local_c8 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar7 * 0x30);
          uVar7 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar4 <= uVar7) goto LAB_14231624e;
LAB_142316212:
          uVar7 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + uVar7 * 8);
          if (uVar7 == 0) goto LAB_14231624e;
          local_198 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar7 * 0x30);
          uVar7 = *(ulonglong *)(lVar1 + 0x120);
          lVar5 = *(longlong *)(lVar9 + 0x38);
          if (uVar4 <= uVar7) goto LAB_142316248;
LAB_142316267:
          uVar4 = *(ulonglong *)(lVar5 + uVar7 * 8);
          lVar9 = *(longlong *)(lVar9 + 0x18);
          if (uVar4 == 0) goto LAB_14231628b;
          local_1a0 = *(longlong *)(lVar9 + 0x10 + ~uVar4 * 0x30);
        }
        else {
          local_c8 = 0;
          uVar7 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar7 < uVar4) goto LAB_142316212;
LAB_14231624e:
          local_198 = 0;
          uVar7 = *(ulonglong *)(lVar1 + 0x120);
          lVar5 = *(longlong *)(lVar9 + 0x38);
          if (uVar7 < uVar4) goto LAB_142316267;
LAB_142316248:
          lVar9 = *(longlong *)(lVar9 + 0x18);
LAB_14231628b:
          local_1a0 = 0;
        }
        local_118 = local_118 + 1;
        lVar8 = ~*(ulonglong *)(lVar5 + *(longlong *)(lVar1 + 0x128) * 8) * 0x30;
        local_178 = *(longlong *)(lVar9 + 0x10 + lVar8);
        local_180 = *(longlong *)(lVar9 + 0x28 + lVar8);
        lVar5 = ~*(ulonglong *)(lVar5 + *(longlong *)(lVar1 + 0x130) * 8) * 0x30;
        local_188 = *(longlong *)(lVar9 + 0x10 + lVar5);
        local_190 = *(longlong *)(lVar9 + 0x28 + lVar5);
        local_f8 = *(longlong *)(lVar6 + 0x50);
        local_88 = 0;
      }
      uVar4 = (ulonglong)*(uint *)(local_f8 + 8 + local_88 * 0x10);
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
        lVar9 = *(longlong *)(lVar2 + 0x1a8) + (ulonglong)*local_118 * 0x48;
        uVar4 = *(ulonglong *)(lVar9 + 0x40);
        if ((*(ulonglong *)(lVar1 + 0x110) < uVar4) &&
           (uVar7 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + *(ulonglong *)(lVar1 + 0x110) * 8),
           uVar7 != 0)) {
          local_c8 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar7 * 0x30);
          uVar7 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar4 <= uVar7) goto LAB_142316155;
LAB_142316119:
          uVar7 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + uVar7 * 8);
          if (uVar7 == 0) goto LAB_142316155;
          local_198 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar7 * 0x30);
          uVar7 = *(ulonglong *)(lVar1 + 0x120);
          lVar6 = *(longlong *)(lVar9 + 0x38);
          if (uVar4 <= uVar7) goto LAB_14231614f;
LAB_14231616e:
          uVar4 = *(ulonglong *)(lVar6 + uVar7 * 8);
          lVar9 = *(longlong *)(lVar9 + 0x18);
          if (uVar4 == 0) goto LAB_142316192;
          local_1a0 = *(longlong *)(lVar9 + 0x10 + ~uVar4 * 0x30);
        }
        else {
          local_c8 = 0;
          uVar7 = *(ulonglong *)(lVar1 + 0x118);
          if (uVar7 < uVar4) goto LAB_142316119;
LAB_142316155:
          local_198 = 0;
          uVar7 = *(ulonglong *)(lVar1 + 0x120);
          lVar6 = *(longlong *)(lVar9 + 0x38);
          if (uVar7 < uVar4) goto LAB_14231616e;
LAB_14231614f:
          lVar9 = *(longlong *)(lVar9 + 0x18);
LAB_142316192:
          local_1a0 = 0;
        }
        local_118 = local_118 + 1;
        lVar5 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar1 + 0x128) * 8) * 0x30;
        local_178 = *(longlong *)(lVar9 + 0x10 + lVar5);
        local_180 = *(longlong *)(lVar9 + 0x28 + lVar5);
        lVar6 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar1 + 0x130) * 8) * 0x30;
        local_188 = *(longlong *)(lVar9 + 0x10 + lVar6);
        local_190 = *(longlong *)(lVar9 + 0x28 + lVar6);
        local_88 = 0;
      }
      uVar4 = local_88 & 0xffffffff;
      local_170 = local_88;
    }
    lVar9 = uVar4 * 0x30 + local_c8;
    pbVar3 = (byte *)(local_1a0 + uVar4 * 0x10);
    puVar10 = (ulonglong *)(uVar4 * 0x10 + local_178);
    if (((*pbVar3 & 1) == 0) || (*(char *)(uVar4 * 0x68 + local_198 + 0x30) != '\x01')) {
LAB_1423163d0:
      uVar15 = *(undefined8 *)(lVar9 + 0x10);
      fVar16 = (float)uVar15;
      fVar17 = (float)((ulonglong)uVar15 >> 0x20);
      fVar18 = *(float *)(lVar9 + 0x18);
      fVar14 = fVar18;
    }
    else {
      fVar16 = (float)*(undefined8 *)(pbVar3 + 4);
      fVar17 = (float)((ulonglong)*(undefined8 *)(pbVar3 + 4) >> 0x20);
      fVar18 = *(float *)(pbVar3 + 0xc);
      country_core::resources::critters::sheep::Sheep::being_petted();
      if (extraout_AL != '\0') goto LAB_1423163d0;
      uVar15 = *(undefined8 *)(lVar9 + 0x10);
      fVar14 = *(float *)(lVar9 + 0x18);
    }
    fVar18 = fVar18 - fVar14;
    fVar16 = fVar16 - (float)uVar15;
    fVar17 = fVar17 - (float)((ulonglong)uVar15 >> 0x20);
    fVar14 = 1.0 / SQRT(fVar18 * fVar18 + fVar17 * fVar17 + fVar16 * fVar16);
    bVar13 = -1 < (int)fVar14;
    bVar11 = (int)ABS(fVar14) - 0x800000U < 0x7f000000;
    bVar12 = (int)fVar14 - 1U < 0x7fffff;
    if (bVar12 || bVar11 && bVar13) {
      fVar18 = fVar18 * fVar14;
    }
    else {
      fVar18 = 0.0;
    }
    local_88 = local_170 + 1;
    *(undefined4 *)(local_180 + uVar4 * 4) = local_cc;
    *puVar10 = CONCAT44((int)((uint)(bVar12 || bVar11 && bVar13) << 0x1f) >> 0x1f,
                        (int)((uint)(bVar12 || bVar11 && bVar13) << 0x1f) >> 0x1f) &
               CONCAT44(fVar17 * fVar14,fVar16 * fVar14);
    *(float *)(puVar10 + 1) = fVar18;
    *(undefined4 *)(local_190 + uVar4 * 4) = local_94;
    *(undefined4 *)(local_188 + uVar4 * 4) = 0x3f800000;
  } while( true );
}

