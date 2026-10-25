// 0x142314470 set_duck_velocities_towards_goals
// SIG: void __cdecl set_duck_velocities_towards_goals(void)

void __cdecl system_critters::duck::set_duck_velocities_towards_goals(void)

{
  byte *pbVar1;
  undefined4 *puVar2;
  longlong lVar3;
  longlong lVar4;
  bool bVar5;
  char extraout_AL;
  uint uVar6;
  ulonglong *puVar7;
  longlong *in_RCX;
  longlong lVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  float fVar14;
  undefined8 uVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  longlong local_1b8;
  longlong local_198;
  longlong local_190;
  longlong local_188;
  uint *local_120;
  uint *local_118;
  longlong local_100;
  longlong local_f0;
  longlong local_e0;
  undefined4 local_d4;
  longlong local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a0;
  undefined4 local_94;
  ulonglong local_90;
  ulonglong local_88;
  byte local_80;
  
  lVar3 = *in_RCX;
  lVar4 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::Duck>,ref_<enum2_<country_core::resources::critters::DuckState>_>,ref_<country_core::resources::critters::CritterGoal>,ref_mut_<country_core::resources::critters::CritterVelocityTarget>,ref_mut_<country_core::resources::critters::CritterVelocityMultiplier>_>,tuple_<>_>
            ();
  local_188 = local_f0;
  local_190 = local_e0;
  local_1b8 = local_d0;
  local_198 = local_a0;
  do {
    if ((local_80 & 1) == 0) {
      do {
        if (local_88 == local_90) {
          if (local_120 == local_118) {
            return;
          }
          while( true ) {
            local_90 = *(ulonglong *)
                        (*(longlong *)(lVar4 + 0x100) + 0x58 + (ulonglong)*local_120 * 0xa0);
            if (local_90 != 0) break;
            local_120 = local_120 + 1;
            if (local_120 == local_118) {
              return;
            }
          }
          lVar9 = *(longlong *)(lVar4 + 0x100) + (ulonglong)*local_120 * 0xa0;
          uVar10 = (ulonglong)*(uint *)(lVar9 + 0x94);
          lVar11 = *(longlong *)(lVar4 + 0x1a8) + uVar10 * 0x48;
          uVar10 = *(ulonglong *)(*(longlong *)(lVar4 + 0x1a8) + 0x40 + uVar10 * 0x48);
          if ((*(ulonglong *)(lVar3 + 0x110) < uVar10) &&
             (uVar12 = *(ulonglong *)
                        (*(longlong *)(lVar11 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
             uVar12 != 0)) {
            local_1b8 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar12 * 0x30);
            uVar12 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar10 <= uVar12) goto LAB_1423148c6;
LAB_142314897:
            uVar12 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + uVar12 * 8);
            if (uVar12 == 0) goto LAB_1423148c6;
            local_c8 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar12 * 0x30);
            uVar12 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar12 < uVar10) goto LAB_1423148d5;
LAB_14231490c:
            local_c0 = 0;
            uVar12 = *(ulonglong *)(lVar3 + 0x128);
            lVar8 = *(longlong *)(lVar11 + 0x38);
            if (uVar12 < uVar10) goto LAB_142314922;
LAB_14231475e:
            lVar11 = *(longlong *)(lVar11 + 0x18);
LAB_142314762:
            local_b8 = 0;
          }
          else {
            local_1b8 = 0;
            uVar12 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar12 < uVar10) goto LAB_142314897;
LAB_1423148c6:
            local_c8 = 0;
            uVar12 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar10 <= uVar12) goto LAB_14231490c;
LAB_1423148d5:
            uVar12 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + uVar12 * 8);
            if (uVar12 == 0) goto LAB_14231490c;
            local_c0 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar12 * 0x30);
            uVar12 = *(ulonglong *)(lVar3 + 0x128);
            lVar8 = *(longlong *)(lVar11 + 0x38);
            if (uVar10 <= uVar12) goto LAB_14231475e;
LAB_142314922:
            uVar10 = *(ulonglong *)(lVar8 + uVar12 * 8);
            lVar11 = *(longlong *)(lVar11 + 0x18);
            if (uVar10 == 0) goto LAB_142314762;
            local_b8 = *(longlong *)(lVar11 + 0x10 + ~uVar10 * 0x30);
          }
          local_120 = local_120 + 1;
          lVar13 = ~*(ulonglong *)(lVar8 + *(longlong *)(lVar3 + 0x130) * 8) * 0x30;
          local_188 = *(longlong *)(lVar11 + 0x10 + lVar13);
          local_190 = *(longlong *)(lVar11 + 0x28 + lVar13);
          lVar8 = ~*(ulonglong *)(lVar8 + *(longlong *)(lVar3 + 0x138) * 8) * 0x30;
          local_b0 = *(longlong *)(lVar11 + 0x10 + lVar8);
          local_198 = *(longlong *)(lVar11 + 0x28 + lVar8);
          local_100 = *(longlong *)(lVar9 + 0x50);
          local_88 = 0;
        }
        uVar10 = (ulonglong)*(uint *)(local_100 + 8 + local_88 * 0x10);
        local_88 = local_88 + 1;
      } while (*(char *)(local_c0 + uVar10) != '\x02');
    }
    else {
      do {
        if (local_88 == local_90) {
          if (local_120 == local_118) {
            return;
          }
          while( true ) {
            local_90 = *(ulonglong *)
                        (*(longlong *)(lVar4 + 0x1a8) + 0x10 + (ulonglong)*local_120 * 0x48);
            if (local_90 != 0) break;
            local_120 = local_120 + 1;
            if (local_120 == local_118) {
              return;
            }
          }
          lVar11 = *(longlong *)(lVar4 + 0x1a8) + (ulonglong)*local_120 * 0x48;
          uVar10 = *(ulonglong *)(lVar11 + 0x40);
          if ((*(ulonglong *)(lVar3 + 0x110) < uVar10) &&
             (uVar12 = *(ulonglong *)
                        (*(longlong *)(lVar11 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
             uVar12 != 0)) {
            local_1b8 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar12 * 0x30);
            uVar12 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar10 <= uVar12) goto LAB_1423146dc;
LAB_1423146ad:
            uVar12 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + uVar12 * 8);
            if (uVar12 == 0) goto LAB_1423146dc;
            local_c8 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar12 * 0x30);
            uVar12 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar12 < uVar10) goto LAB_1423146eb;
LAB_142314722:
            local_c0 = 0;
            uVar12 = *(ulonglong *)(lVar3 + 0x128);
            lVar9 = *(longlong *)(lVar11 + 0x38);
            if (uVar12 < uVar10) goto LAB_142314738;
LAB_1423145a0:
            lVar11 = *(longlong *)(lVar11 + 0x18);
LAB_1423145a4:
            local_b8 = 0;
          }
          else {
            local_1b8 = 0;
            uVar12 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar12 < uVar10) goto LAB_1423146ad;
LAB_1423146dc:
            local_c8 = 0;
            uVar12 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar10 <= uVar12) goto LAB_142314722;
LAB_1423146eb:
            uVar12 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + uVar12 * 8);
            if (uVar12 == 0) goto LAB_142314722;
            local_c0 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar12 * 0x30);
            uVar12 = *(ulonglong *)(lVar3 + 0x128);
            lVar9 = *(longlong *)(lVar11 + 0x38);
            if (uVar10 <= uVar12) goto LAB_1423145a0;
LAB_142314738:
            uVar10 = *(ulonglong *)(lVar9 + uVar12 * 8);
            lVar11 = *(longlong *)(lVar11 + 0x18);
            if (uVar10 == 0) goto LAB_1423145a4;
            local_b8 = *(longlong *)(lVar11 + 0x10 + ~uVar10 * 0x30);
          }
          local_120 = local_120 + 1;
          lVar8 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar3 + 0x130) * 8) * 0x30;
          local_188 = *(longlong *)(lVar11 + 0x10 + lVar8);
          local_190 = *(longlong *)(lVar11 + 0x28 + lVar8);
          lVar9 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar3 + 0x138) * 8) * 0x30;
          local_b0 = *(longlong *)(lVar11 + 0x10 + lVar9);
          local_198 = *(longlong *)(lVar11 + 0x28 + lVar9);
          local_88 = 0;
        }
        uVar10 = local_88 & 0xffffffff;
        local_88 = local_88 + 1;
      } while (*(char *)(local_c0 + uVar10) != '\x02');
    }
    lVar9 = uVar10 * 0x30 + local_1b8;
    lVar11 = local_c8 + uVar10 * 0x100;
    pbVar1 = (byte *)(local_b8 + uVar10 * 0x10);
    puVar7 = (ulonglong *)(local_188 + uVar10 * 0x10);
    puVar2 = (undefined4 *)(local_190 + uVar10 * 4);
    if (((*pbVar1 & 1) == 0) || (*(char *)(lVar11 + 0xe0) != '\x01')) {
LAB_142314a16:
      uVar15 = *(undefined8 *)(lVar9 + 0x10);
      fVar16 = (float)uVar15;
      fVar17 = (float)((ulonglong)uVar15 >> 0x20);
      fVar18 = *(float *)(lVar9 + 0x18);
      fVar14 = fVar18;
    }
    else {
      fVar16 = (float)*(undefined8 *)(pbVar1 + 4);
      fVar17 = (float)((ulonglong)*(undefined8 *)(pbVar1 + 4) >> 0x20);
      fVar18 = *(float *)(pbVar1 + 0xc);
      _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
                ();
      if (extraout_AL != '\0') goto LAB_142314a16;
      uVar15 = *(undefined8 *)(lVar9 + 0x10);
      fVar14 = *(float *)(lVar9 + 0x18);
    }
    fVar18 = fVar18 - fVar14;
    fVar16 = fVar16 - (float)uVar15;
    fVar17 = fVar17 - (float)((ulonglong)uVar15 >> 0x20);
    fVar14 = 1.0 / SQRT(fVar18 * fVar18 + fVar17 * fVar17 + fVar16 * fVar16);
    bVar5 = (int)fVar14 - 1U < 0x7fffff ||
            (int)ABS(fVar14) - 0x800000U < 0x7f000000 && -1 < (int)fVar14;
    if (bVar5) {
      fVar18 = fVar18 * fVar14;
    }
    else {
      fVar18 = 0.0;
    }
    *puVar2 = local_d4;
    uVar6 = (uint)bVar5;
    *puVar7 = CONCAT44((int)(uVar6 << 0x1f) >> 0x1f,(int)(uVar6 << 0x1f) >> 0x1f) &
              CONCAT44(fVar17 * fVar14,fVar16 * fVar14);
    *(float *)(puVar7 + 1) = fVar18;
    *(undefined4 *)(local_198 + uVar10 * 4) = local_94;
    *(undefined4 *)(local_b0 + uVar10 * 4) = 0x3f800000;
    if (*(char *)(lVar11 + 0xfc) == '\x01') {
      *puVar2 = local_d4;
      *(undefined4 *)(puVar7 + 1) = 0;
      *puVar7 = 0;
    }
  } while( true );
}

