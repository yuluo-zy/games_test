
void __cdecl system_critters::duck::set_duck_velocities_towards_goals(void)

{
  byte *pbVar1;
  undefined4 *puVar2;
  longlong lVar3;
  longlong lVar4;
  bool bVar5;
  float fVar6;
  char extraout_AL;
  uint uVar7;
  ulonglong *puVar8;
  longlong *in_RCX;
  longlong lVar9;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  longlong lVar14;
  float fVar15;
  undefined8 uVar16;
  float fVar17;
  float fVar18;
  float fVar19;
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
  fVar6 = __real_3f800000;
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
          lVar10 = *(longlong *)(lVar4 + 0x100) + (ulonglong)*local_120 * 0xa0;
          uVar11 = (ulonglong)*(uint *)(lVar10 + 0x94);
          lVar12 = *(longlong *)(lVar4 + 0x1a8) + uVar11 * 0x48;
          uVar11 = *(ulonglong *)(*(longlong *)(lVar4 + 0x1a8) + 0x40 + uVar11 * 0x48);
          if ((*(ulonglong *)(lVar3 + 0x110) < uVar11) &&
             (uVar13 = *(ulonglong *)
                        (*(longlong *)(lVar12 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
             uVar13 != 0)) {
            local_1b8 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar11 <= uVar13) goto LAB_1423148c6;
LAB_142314897:
            uVar13 = *(ulonglong *)(*(longlong *)(lVar12 + 0x38) + uVar13 * 8);
            if (uVar13 == 0) goto LAB_1423148c6;
            local_c8 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar13 < uVar11) goto LAB_1423148d5;
LAB_14231490c:
            local_c0 = 0;
            uVar13 = *(ulonglong *)(lVar3 + 0x128);
            lVar9 = *(longlong *)(lVar12 + 0x38);
            if (uVar13 < uVar11) goto LAB_142314922;
LAB_14231475e:
            lVar12 = *(longlong *)(lVar12 + 0x18);
LAB_142314762:
            local_b8 = 0;
          }
          else {
            local_1b8 = 0;
            uVar13 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar13 < uVar11) goto LAB_142314897;
LAB_1423148c6:
            local_c8 = 0;
            uVar13 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar11 <= uVar13) goto LAB_14231490c;
LAB_1423148d5:
            uVar13 = *(ulonglong *)(*(longlong *)(lVar12 + 0x38) + uVar13 * 8);
            if (uVar13 == 0) goto LAB_14231490c;
            local_c0 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(lVar3 + 0x128);
            lVar9 = *(longlong *)(lVar12 + 0x38);
            if (uVar11 <= uVar13) goto LAB_14231475e;
LAB_142314922:
            uVar11 = *(ulonglong *)(lVar9 + uVar13 * 8);
            lVar12 = *(longlong *)(lVar12 + 0x18);
            if (uVar11 == 0) goto LAB_142314762;
            local_b8 = *(longlong *)(lVar12 + 0x10 + ~uVar11 * 0x30);
          }
          local_120 = local_120 + 1;
          lVar14 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar3 + 0x130) * 8) * 0x30;
          local_188 = *(longlong *)(lVar12 + 0x10 + lVar14);
          local_190 = *(longlong *)(lVar12 + 0x28 + lVar14);
          lVar9 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar3 + 0x138) * 8) * 0x30;
          local_b0 = *(longlong *)(lVar12 + 0x10 + lVar9);
          local_198 = *(longlong *)(lVar12 + 0x28 + lVar9);
          local_100 = *(longlong *)(lVar10 + 0x50);
          local_88 = 0;
        }
        uVar11 = (ulonglong)*(uint *)(local_100 + 8 + local_88 * 0x10);
        local_88 = local_88 + 1;
      } while (*(char *)(local_c0 + uVar11) != '\x02');
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
          lVar12 = *(longlong *)(lVar4 + 0x1a8) + (ulonglong)*local_120 * 0x48;
          uVar11 = *(ulonglong *)(lVar12 + 0x40);
          if ((*(ulonglong *)(lVar3 + 0x110) < uVar11) &&
             (uVar13 = *(ulonglong *)
                        (*(longlong *)(lVar12 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
             uVar13 != 0)) {
            local_1b8 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar11 <= uVar13) goto LAB_1423146dc;
LAB_1423146ad:
            uVar13 = *(ulonglong *)(*(longlong *)(lVar12 + 0x38) + uVar13 * 8);
            if (uVar13 == 0) goto LAB_1423146dc;
            local_c8 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar13 < uVar11) goto LAB_1423146eb;
LAB_142314722:
            local_c0 = 0;
            uVar13 = *(ulonglong *)(lVar3 + 0x128);
            lVar10 = *(longlong *)(lVar12 + 0x38);
            if (uVar13 < uVar11) goto LAB_142314738;
LAB_1423145a0:
            lVar12 = *(longlong *)(lVar12 + 0x18);
LAB_1423145a4:
            local_b8 = 0;
          }
          else {
            local_1b8 = 0;
            uVar13 = *(ulonglong *)(lVar3 + 0x118);
            if (uVar13 < uVar11) goto LAB_1423146ad;
LAB_1423146dc:
            local_c8 = 0;
            uVar13 = *(ulonglong *)(lVar3 + 0x120);
            if (uVar11 <= uVar13) goto LAB_142314722;
LAB_1423146eb:
            uVar13 = *(ulonglong *)(*(longlong *)(lVar12 + 0x38) + uVar13 * 8);
            if (uVar13 == 0) goto LAB_142314722;
            local_c0 = *(longlong *)(*(longlong *)(lVar12 + 0x18) + 0x10 + ~uVar13 * 0x30);
            uVar13 = *(ulonglong *)(lVar3 + 0x128);
            lVar10 = *(longlong *)(lVar12 + 0x38);
            if (uVar11 <= uVar13) goto LAB_1423145a0;
LAB_142314738:
            uVar11 = *(ulonglong *)(lVar10 + uVar13 * 8);
            lVar12 = *(longlong *)(lVar12 + 0x18);
            if (uVar11 == 0) goto LAB_1423145a4;
            local_b8 = *(longlong *)(lVar12 + 0x10 + ~uVar11 * 0x30);
          }
          local_120 = local_120 + 1;
          lVar9 = ~*(ulonglong *)(lVar10 + *(longlong *)(lVar3 + 0x130) * 8) * 0x30;
          local_188 = *(longlong *)(lVar12 + 0x10 + lVar9);
          local_190 = *(longlong *)(lVar12 + 0x28 + lVar9);
          lVar10 = ~*(ulonglong *)(lVar10 + *(longlong *)(lVar3 + 0x138) * 8) * 0x30;
          local_b0 = *(longlong *)(lVar12 + 0x10 + lVar10);
          local_198 = *(longlong *)(lVar12 + 0x28 + lVar10);
          local_88 = 0;
        }
        uVar11 = local_88 & 0xffffffff;
        local_88 = local_88 + 1;
      } while (*(char *)(local_c0 + uVar11) != '\x02');
    }
    lVar10 = uVar11 * 0x30 + local_1b8;
    lVar12 = local_c8 + uVar11 * 0x100;
    pbVar1 = (byte *)(local_b8 + uVar11 * 0x10);
    puVar8 = (ulonglong *)(local_188 + uVar11 * 0x10);
    puVar2 = (undefined4 *)(local_190 + uVar11 * 4);
    if (((*pbVar1 & 1) == 0) || (*(char *)(lVar12 + 0xe0) != '\x01')) {
LAB_142314a16:
      uVar16 = *(undefined8 *)(lVar10 + 0x10);
      fVar17 = (float)uVar16;
      fVar18 = (float)((ulonglong)uVar16 >> 0x20);
      fVar19 = *(float *)(lVar10 + 0x18);
      fVar15 = fVar19;
    }
    else {
      fVar17 = (float)*(undefined8 *)(pbVar1 + 4);
      fVar18 = (float)((ulonglong)*(undefined8 *)(pbVar1 + 4) >> 0x20);
      fVar19 = *(float *)(pbVar1 + 0xc);
      _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
                ();
      if (extraout_AL != '\0') goto LAB_142314a16;
      uVar16 = *(undefined8 *)(lVar10 + 0x10);
      fVar15 = *(float *)(lVar10 + 0x18);
    }
    fVar19 = fVar19 - fVar15;
    fVar17 = fVar17 - (float)uVar16;
    fVar18 = fVar18 - (float)((ulonglong)uVar16 >> 0x20);
    fVar15 = fVar6 / SQRT(fVar19 * fVar19 + fVar18 * fVar18 + fVar17 * fVar17);
    bVar5 = (int)fVar15 - 1U < 0x7fffff ||
            (int)ABS(fVar15) - 0x800000U < 0x7f000000 && -1 < (int)fVar15;
    if (bVar5) {
      fVar19 = fVar19 * fVar15;
    }
    else {
      fVar19 = 0.0;
    }
    *puVar2 = local_d4;
    uVar7 = (uint)bVar5;
    *puVar8 = CONCAT44((int)(uVar7 << 0x1f) >> 0x1f,(int)(uVar7 << 0x1f) >> 0x1f) &
              CONCAT44(fVar18 * fVar15,fVar17 * fVar15);
    *(float *)(puVar8 + 1) = fVar19;
    *(undefined4 *)(local_198 + uVar11 * 4) = local_94;
    *(undefined4 *)(local_b0 + uVar11 * 4) = 0x3f800000;
    if (*(char *)(lVar12 + 0xfc) == '\x01') {
      *puVar2 = local_d4;
      *(undefined4 *)(puVar8 + 1) = 0;
      *puVar8 = 0;
    }
  } while( true );
}

