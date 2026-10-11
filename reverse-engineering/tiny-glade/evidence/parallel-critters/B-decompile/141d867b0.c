
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_critters::butterfly::set_butterfly_velocities_towards_goals(void)

{
  uint *puVar1;
  undefined4 *puVar2;
  uint *puVar3;
  undefined4 uVar4;
  float fVar5;
  longlong *in_RCX;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong *puVar8;
  uint uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  bool bVar15;
  bool bVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar21;
  undefined1 auVar20 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
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
  
  lVar12 = *in_RCX;
  lVar6 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::sheep::Sheep>,ref_<country_core::resources::critters::CritterGoal>,ref_mut_<country_core::resources::critters::CritterVelocityTarget>,ref_mut_<country_core::resources::critters::CritterVelocityMultiplier>_>,tuple_<>_>
            ();
  fVar5 = __real_3f800000;
  uVar4 = __real_3f000000;
  local_168 = local_118;
  local_180 = local_108;
  local_188 = local_f8;
  local_198 = local_e8;
  local_1a0 = local_e0;
  local_1a8 = local_d8;
  if ((local_a0 & 1) == 0) {
    if (local_a8 != local_b0) {
LAB_141d86c6d:
      uVar7 = (ulonglong)*(uint *)(local_168 + 8 + local_a8 * 0x10);
      goto LAB_141d86c7e;
    }
    if (local_138 != local_130) {
      do {
        if (*(longlong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)*local_138 * 0xa0) != 0)
        {
          lVar11 = *(longlong *)(lVar6 + 0x100) + (ulonglong)*local_138 * 0xa0;
          uVar7 = (ulonglong)*(uint *)(lVar11 + 0x94);
          lVar13 = *(longlong *)(lVar6 + 0x1a8) + uVar7 * 0x48;
          uVar7 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x40 + uVar7 * 0x48);
          if ((*(ulonglong *)(lVar12 + 0x110) < uVar7) &&
             (uVar10 = *(ulonglong *)
                        (*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
             uVar10 != 0)) {
            local_198 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar7 <= uVar10) goto LAB_141d86bcd;
LAB_141d86b91:
            uVar10 = *(ulonglong *)(*(longlong *)(lVar13 + 0x38) + uVar10 * 8);
            if (uVar10 == 0) goto LAB_141d86bcd;
            local_1a0 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar6 = *(longlong *)(lVar13 + 0x38);
            if (uVar7 <= uVar10) goto LAB_141d86bc7;
LAB_141d86be6:
            uVar7 = *(ulonglong *)(lVar6 + uVar10 * 8);
            lVar13 = *(longlong *)(lVar13 + 0x18);
            if (uVar7 == 0) goto LAB_141d86c0a;
            local_1a8 = *(longlong *)(lVar13 + 0x10 + ~uVar7 * 0x30);
          }
          else {
            local_198 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar10 < uVar7) goto LAB_141d86b91;
LAB_141d86bcd:
            local_1a0 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar6 = *(longlong *)(lVar13 + 0x38);
            if (uVar10 < uVar7) goto LAB_141d86be6;
LAB_141d86bc7:
            lVar13 = *(longlong *)(lVar13 + 0x18);
LAB_141d86c0a:
            local_1a8 = 0;
          }
          lVar14 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar12 + 0x128) * 8) * 0x30;
          local_180 = *(longlong *)(lVar13 + 0x10 + lVar14);
          local_188 = *(longlong *)(lVar13 + 0x28 + lVar14);
          lVar12 = ~*(ulonglong *)(lVar6 + *(longlong *)(lVar12 + 0x130) * 8) * 0x30;
          local_d0 = *(longlong *)(lVar13 + 0x10 + lVar12);
          local_c0 = *(longlong *)(lVar13 + 0x28 + lVar12);
          local_168 = *(longlong *)(lVar11 + 0x50);
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
      uVar7 = local_a8 & 0xffffffff;
LAB_141d86c7e:
      puVar1 = (uint *)(local_1a0 + uVar7 * 0x18);
      local_198 = uVar7 * 0x30 + local_198;
      puVar2 = (undefined4 *)(local_c0 + uVar7 * 4);
      puVar3 = (uint *)(local_1a8 + uVar7 * 0x10);
      puVar8 = (ulonglong *)(uVar7 * 0x10 + local_180);
      uVar9 = *puVar3;
      fVar18 = *(float *)(local_198 + 0x18);
      fVar17 = fVar18;
      if ((uVar9 & 1) != 0) {
        fVar17 = (float)puVar3[3];
      }
      fVar17 = fVar17 - fVar18;
      uVar10 = *(ulonglong *)(local_198 + 0x10);
      auVar23._8_8_ = 0;
      auVar23._0_8_ = uVar10;
      auVar20._0_8_ = CONCAT44((int)(uVar9 << 0x1f) >> 0x1f,(int)(uVar9 << 0x1f) >> 0x1f);
      auVar20._8_8_ = 0;
      auVar22._8_8_ = 0;
      auVar22._0_8_ = *(ulonglong *)(puVar3 + 1) & auVar20._0_8_;
      auVar22 = ~auVar20 & auVar23 | auVar22;
      fVar19 = auVar22._0_4_ - (float)uVar10;
      fVar21 = auVar22._4_4_ - (float)(uVar10 >> 0x20);
      fVar18 = __real_3f800000 / SQRT(fVar21 * fVar21 + fVar19 * fVar19 + fVar17 * fVar17);
      bVar15 = (int)ABS(fVar18) - 0x800000U < 0x7f000000;
      bVar16 = (int)fVar18 - 1U < 0x7fffff;
      fVar17 = fVar18 * fVar17 * __real_3fe147ae;
      fVar19 = fVar18 * fVar19 * ___xmm_00000000000000003fe147ae3fe147ae;
      fVar21 = fVar18 * fVar21 * _UNK_142f67d44;
      if (!bVar16 && (!bVar15 || -1 >= (int)fVar18)) {
        fVar17 = 0.0;
      }
      uVar9 = (uint)(bVar16 || bVar15 && -1 < (int)fVar18);
      *(undefined4 *)(local_188 + uVar7 * 4) = local_ec;
      *puVar8 = CONCAT44((int)(uVar9 << 0x1f) >> 0x1f,(int)(uVar9 << 0x1f) >> 0x1f) &
                CONCAT44(fVar21,fVar19);
      *(float *)(puVar8 + 1) = fVar17;
      fVar18 = (float)puVar1[4];
      *puVar2 = local_b4;
      *(float *)(local_d0 + uVar7 * 4) = fVar18 + fVar5;
                    /* WARNING: Could not recover jumptable at 0x000141d86de3. Too many branches */
                    /* WARNING: Treating indirect jump as call */
      (*(code *)(&DAT_142f68018 + *(int *)(&DAT_142f68018 + (ulonglong)*puVar1 * 4)))
                (local_130,puVar2,local_a8 + 1,uVar4);
      return;
    }
    if (local_138 != local_130) {
      do {
        if (*(longlong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)*local_138 * 0x48) != 0)
        {
          lVar6 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)*local_138 * 0x48;
          uVar7 = *(ulonglong *)(lVar6 + 0x40);
          if ((*(ulonglong *)(lVar12 + 0x110) < uVar7) &&
             (uVar10 = *(ulonglong *)
                        (*(longlong *)(lVar6 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
             uVar10 != 0)) {
            local_198 = *(longlong *)(*(longlong *)(lVar6 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar7 <= uVar10) goto LAB_141d86add;
LAB_141d86aa1:
            uVar10 = *(ulonglong *)(*(longlong *)(lVar6 + 0x38) + uVar10 * 8);
            if (uVar10 == 0) goto LAB_141d86add;
            local_1a0 = *(longlong *)(*(longlong *)(lVar6 + 0x18) + 0x10 + ~uVar10 * 0x30);
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar13 = *(longlong *)(lVar6 + 0x38);
            if (uVar7 <= uVar10) goto LAB_141d86ad7;
LAB_141d86af6:
            uVar7 = *(ulonglong *)(lVar13 + uVar10 * 8);
            lVar6 = *(longlong *)(lVar6 + 0x18);
            if (uVar7 == 0) goto LAB_141d86b1a;
            local_1a8 = *(longlong *)(lVar6 + 0x10 + ~uVar7 * 0x30);
          }
          else {
            local_198 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x118);
            if (uVar10 < uVar7) goto LAB_141d86aa1;
LAB_141d86add:
            local_1a0 = 0;
            uVar10 = *(ulonglong *)(lVar12 + 0x120);
            lVar13 = *(longlong *)(lVar6 + 0x38);
            if (uVar10 < uVar7) goto LAB_141d86af6;
LAB_141d86ad7:
            lVar6 = *(longlong *)(lVar6 + 0x18);
LAB_141d86b1a:
            local_1a8 = 0;
          }
          lVar11 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar12 + 0x128) * 8) * 0x30;
          local_180 = *(longlong *)(lVar6 + 0x10 + lVar11);
          local_188 = *(longlong *)(lVar6 + 0x28 + lVar11);
          lVar12 = ~*(ulonglong *)(lVar13 + *(longlong *)(lVar12 + 0x130) * 8) * 0x30;
          local_d0 = *(longlong *)(lVar6 + 0x10 + lVar12);
          local_c0 = *(longlong *)(lVar6 + 0x28 + lVar12);
          local_a8 = 0;
          goto LAB_141d86b74;
        }
        local_138 = local_138 + 1;
      } while (local_138 != local_130);
    }
  }
  return;
}

