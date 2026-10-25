// 0x141d867b0 set_butterfly_velocities_towards_goals
// SIG: void __cdecl set_butterfly_velocities_towards_goals(void)

void __cdecl system_critters::butterfly::set_butterfly_velocities_towards_goals(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  uint *puVar3;
  longlong *in_RCX;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong *puVar6;
  uint uVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  bool bVar13;
  bool bVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar19;
  undefined1 auVar18 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  longlong local_1a8;
  longlong local_1a0;
  longlong local_198;
  longlong local_188;
  longlong local_180;
  longlong local_168;
  uint *local_138;
  uint *local_130;
  longlong local_118;
  longlong local_108;
  longlong local_f8;
  undefined4 local_ec;
  longlong local_e8;
  longlong local_e0;
  longlong local_d8;
  longlong local_d0;
  longlong local_c0;
  undefined4 local_b4;
  ulonglong local_b0;
  ulonglong local_a8;
  byte local_a0;
  
  lVar10 = *in_RCX;
  lVar4 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::sheep::Sheep>,ref_<country_core::resources::critters::CritterGoal>,ref_mut_<country_core::resources::critters::CritterVelocityTarget>,ref_mut_<country_core::resources::critters::CritterVelocityMultiplier>_>,tuple_<>_>
            ();
  local_168 = local_118;
  local_180 = local_108;
  local_188 = local_f8;
  local_198 = local_e8;
  local_1a0 = local_e0;
  local_1a8 = local_d8;
  if ((local_a0 & 1) == 0) {
    if (local_a8 != local_b0) {
LAB_141d86c6d:
      uVar5 = (ulonglong)*(uint *)(local_168 + 8 + local_a8 * 0x10);
      goto LAB_141d86c7e;
    }
    if (local_138 != local_130) {
      do {
        if (*(longlong *)(*(longlong *)(lVar4 + 0x100) + 0x58 + (ulonglong)*local_138 * 0xa0) != 0)
        {
          lVar9 = *(longlong *)(lVar4 + 0x100) + (ulonglong)*local_138 * 0xa0;
          uVar5 = (ulonglong)*(uint *)(lVar9 + 0x94);
          lVar11 = *(longlong *)(lVar4 + 0x1a8) + uVar5 * 0x48;
          uVar5 = *(ulonglong *)(*(longlong *)(lVar4 + 0x1a8) + 0x40 + uVar5 * 0x48);
          if ((*(ulonglong *)(lVar10 + 0x110) < uVar5) &&
             (uVar8 = *(ulonglong *)
                       (*(longlong *)(lVar11 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
             uVar8 != 0)) {
            local_198 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar8 * 0x30);
            uVar8 = *(ulonglong *)(lVar10 + 0x118);
            if (uVar5 <= uVar8) goto LAB_141d86bcd;
LAB_141d86b91:
            uVar8 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + uVar8 * 8);
            if (uVar8 == 0) goto LAB_141d86bcd;
            local_1a0 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar8 * 0x30);
            uVar8 = *(ulonglong *)(lVar10 + 0x120);
            lVar4 = *(longlong *)(lVar11 + 0x38);
            if (uVar5 <= uVar8) goto LAB_141d86bc7;
LAB_141d86be6:
            uVar5 = *(ulonglong *)(lVar4 + uVar8 * 8);
            lVar11 = *(longlong *)(lVar11 + 0x18);
            if (uVar5 == 0) goto LAB_141d86c0a;
            local_1a8 = *(longlong *)(lVar11 + 0x10 + ~uVar5 * 0x30);
          }
          else {
            local_198 = 0;
            uVar8 = *(ulonglong *)(lVar10 + 0x118);
            if (uVar8 < uVar5) goto LAB_141d86b91;
LAB_141d86bcd:
            local_1a0 = 0;
            uVar8 = *(ulonglong *)(lVar10 + 0x120);
            lVar4 = *(longlong *)(lVar11 + 0x38);
            if (uVar8 < uVar5) goto LAB_141d86be6;
LAB_141d86bc7:
            lVar11 = *(longlong *)(lVar11 + 0x18);
LAB_141d86c0a:
            local_1a8 = 0;
          }
          lVar12 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar10 + 0x128) * 8) * 0x30;
          local_180 = *(longlong *)(lVar11 + 0x10 + lVar12);
          local_188 = *(longlong *)(lVar11 + 0x28 + lVar12);
          lVar10 = ~*(ulonglong *)(lVar4 + *(longlong *)(lVar10 + 0x130) * 8) * 0x30;
          local_d0 = *(longlong *)(lVar11 + 0x10 + lVar10);
          local_c0 = *(longlong *)(lVar11 + 0x28 + lVar10);
          local_168 = *(longlong *)(lVar9 + 0x50);
          local_a8 = 0;
          goto LAB_141d86c6d;
        }
        local_138 = local_138 + 1;
      } while (local_138 != local_130);
    }
  }
  else {
    if (local_a8 != local_b0) {
LAB_141d86b74:
      uVar5 = local_a8 & 0xffffffff;
LAB_141d86c7e:
      puVar1 = (uint *)(local_1a0 + uVar5 * 0x18);
      local_198 = uVar5 * 0x30 + local_198;
      puVar2 = (undefined4 *)(local_c0 + uVar5 * 4);
      puVar3 = (uint *)(local_1a8 + uVar5 * 0x10);
      puVar6 = (ulonglong *)(uVar5 * 0x10 + local_180);
      uVar7 = *puVar3;
      fVar16 = *(float *)(local_198 + 0x18);
      fVar15 = fVar16;
      if ((uVar7 & 1) != 0) {
        fVar15 = (float)puVar3[3];
      }
      fVar15 = fVar15 - fVar16;
      uVar8 = *(ulonglong *)(local_198 + 0x10);
      auVar21._8_8_ = 0;
      auVar21._0_8_ = uVar8;
      auVar18._0_8_ = CONCAT44((int)(uVar7 << 0x1f) >> 0x1f,(int)(uVar7 << 0x1f) >> 0x1f);
      auVar18._8_8_ = 0;
      auVar20._8_8_ = 0;
      auVar20._0_8_ = *(ulonglong *)(puVar3 + 1) & auVar18._0_8_;
      auVar20 = ~auVar18 & auVar21 | auVar20;
      fVar17 = auVar20._0_4_ - (float)uVar8;
      fVar19 = auVar20._4_4_ - (float)(uVar8 >> 0x20);
      fVar16 = 1.0 / SQRT(fVar19 * fVar19 + fVar17 * fVar17 + fVar15 * fVar15);
      bVar13 = (int)ABS(fVar16) - 0x800000U < 0x7f000000;
      bVar14 = (int)fVar16 - 1U < 0x7fffff;
      fVar15 = fVar16 * fVar15 * 1.76;
      if (!bVar14 && (!bVar13 || -1 >= (int)fVar16)) {
        fVar15 = 0.0;
      }
      uVar7 = (uint)(bVar14 || bVar13 && -1 < (int)fVar16);
      *(undefined4 *)(local_188 + uVar5 * 4) = local_ec;
      *puVar6 = CONCAT44((int)(uVar7 << 0x1f) >> 0x1f,(int)(uVar7 << 0x1f) >> 0x1f) &
                CONCAT44(fVar16 * fVar19 * 1.76,fVar16 * fVar17 * 1.76);
      *(float *)(puVar6 + 1) = fVar15;
      fVar16 = (float)puVar1[4];
      *puVar2 = local_b4;
      *(float *)(local_d0 + uVar5 * 4) = fVar16 + 1.0;
                    // WARNING: Could not recover jumptable at 0x000141d86de3. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_142f68018 + *(int *)(&DAT_142f68018 + (ulonglong)*puVar1 * 4)))
                (local_130,puVar2,local_a8 + 1,0x3f000000);
      return;
    }
    if (local_138 != local_130) {
      do {
        if (*(longlong *)(*(longlong *)(lVar4 + 0x1a8) + 0x10 + (ulonglong)*local_138 * 0x48) != 0)
        {
          lVar4 = *(longlong *)(lVar4 + 0x1a8) + (ulonglong)*local_138 * 0x48;
          uVar5 = *(ulonglong *)(lVar4 + 0x40);
          if ((*(ulonglong *)(lVar10 + 0x110) < uVar5) &&
             (uVar8 = *(ulonglong *)
                       (*(longlong *)(lVar4 + 0x38) + *(ulonglong *)(lVar10 + 0x110) * 8),
             uVar8 != 0)) {
            local_198 = *(longlong *)(*(longlong *)(lVar4 + 0x18) + 0x10 + ~uVar8 * 0x30);
            uVar8 = *(ulonglong *)(lVar10 + 0x118);
            if (uVar5 <= uVar8) goto LAB_141d86add;
LAB_141d86aa1:
            uVar8 = *(ulonglong *)(*(longlong *)(lVar4 + 0x38) + uVar8 * 8);
            if (uVar8 == 0) goto LAB_141d86add;
            local_1a0 = *(longlong *)(*(longlong *)(lVar4 + 0x18) + 0x10 + ~uVar8 * 0x30);
            uVar8 = *(ulonglong *)(lVar10 + 0x120);
            lVar11 = *(longlong *)(lVar4 + 0x38);
            if (uVar5 <= uVar8) goto LAB_141d86ad7;
LAB_141d86af6:
            uVar5 = *(ulonglong *)(lVar11 + uVar8 * 8);
            lVar4 = *(longlong *)(lVar4 + 0x18);
            if (uVar5 == 0) goto LAB_141d86b1a;
            local_1a8 = *(longlong *)(lVar4 + 0x10 + ~uVar5 * 0x30);
          }
          else {
            local_198 = 0;
            uVar8 = *(ulonglong *)(lVar10 + 0x118);
            if (uVar8 < uVar5) goto LAB_141d86aa1;
LAB_141d86add:
            local_1a0 = 0;
            uVar8 = *(ulonglong *)(lVar10 + 0x120);
            lVar11 = *(longlong *)(lVar4 + 0x38);
            if (uVar8 < uVar5) goto LAB_141d86af6;
LAB_141d86ad7:
            lVar4 = *(longlong *)(lVar4 + 0x18);
LAB_141d86b1a:
            local_1a8 = 0;
          }
          lVar9 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar10 + 0x128) * 8) * 0x30;
          local_180 = *(longlong *)(lVar4 + 0x10 + lVar9);
          local_188 = *(longlong *)(lVar4 + 0x28 + lVar9);
          lVar10 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar10 + 0x130) * 8) * 0x30;
          local_d0 = *(longlong *)(lVar4 + 0x10 + lVar10);
          local_c0 = *(longlong *)(lVar4 + 0x28 + lVar10);
          local_a8 = 0;
          goto LAB_141d86b74;
        }
        local_138 = local_138 + 1;
      } while (local_138 != local_130);
    }
  }
  return;
}

