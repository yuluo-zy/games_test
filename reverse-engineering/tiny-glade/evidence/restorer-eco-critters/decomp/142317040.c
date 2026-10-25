// 0x142317040 set_firefly_velocities_towards_goals
// SIG: void __cdecl set_firefly_velocities_towards_goals(void)

void __cdecl system_critters::firefly::set_firefly_velocities_towards_goals(void)

{
  undefined4 *puVar1;
  uint *puVar2;
  ulonglong uVar3;
  longlong lVar4;
  longlong lVar5;
  bool bVar6;
  longlong *in_RCX;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong *puVar9;
  uint uVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  float fVar14;
  float fVar15;
  float fVar17;
  undefined1 auVar16 [16];
  float fVar18;
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  longlong local_160;
  longlong local_158;
  longlong local_140;
  longlong local_138;
  longlong local_120;
  uint *local_f8;
  uint *local_f0;
  longlong local_d8;
  longlong local_c8;
  longlong local_b8;
  undefined4 local_ac;
  longlong local_a8;
  longlong local_98;
  longlong local_90;
  longlong local_80;
  undefined4 local_74;
  ulonglong local_70;
  ulonglong local_68;
  byte local_60;
  
  lVar4 = *in_RCX;
  lVar5 = in_RCX[1];
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<ref_<utils::components::transform::Transform>,ref_<country_core::resources::critters::sheep::Sheep>,ref_<country_core::resources::critters::CritterGoal>,ref_mut_<country_core::resources::critters::CritterVelocityTarget>,ref_mut_<country_core::resources::critters::CritterVelocityMultiplier>_>,tuple_<>_>
            ();
  local_120 = local_d8;
  local_138 = local_c8;
  local_140 = local_b8;
  local_158 = local_a8;
  local_160 = local_98;
  do {
    if ((local_60 & 1) == 0) {
      if (local_68 == local_70) {
        if (local_f8 == local_f0) {
          return;
        }
        while( true ) {
          local_70 = *(ulonglong *)
                      (*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)*local_f8 * 0xa0);
          if (local_70 != 0) break;
          local_f8 = local_f8 + 1;
          if (local_f8 == local_f0) {
            return;
          }
        }
        lVar7 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*local_f8 * 0xa0;
        uVar8 = (ulonglong)*(uint *)(lVar7 + 0x94);
        lVar11 = *(longlong *)(lVar5 + 0x1a8) + uVar8 * 0x48;
        uVar8 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x40 + uVar8 * 0x48);
        if ((*(ulonglong *)(lVar4 + 0x110) < uVar8) &&
           (uVar3 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8)
           , uVar3 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar3 * 0x30);
        }
        else {
          local_158 = 0;
        }
        lVar12 = *(longlong *)(lVar11 + 0x38);
        if (*(ulonglong *)(lVar4 + 0x120) < uVar8) {
          uVar8 = *(ulonglong *)(lVar12 + *(ulonglong *)(lVar4 + 0x120) * 8);
          lVar11 = *(longlong *)(lVar11 + 0x18);
          if (uVar8 == 0) goto LAB_14231737a;
          local_160 = *(longlong *)(lVar11 + 0x10 + ~uVar8 * 0x30);
        }
        else {
          lVar11 = *(longlong *)(lVar11 + 0x18);
LAB_14231737a:
          local_160 = 0;
        }
        local_f8 = local_f8 + 1;
        lVar13 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar4 + 0x128) * 8) * 0x30;
        local_138 = *(longlong *)(lVar11 + 0x10 + lVar13);
        local_140 = *(longlong *)(lVar11 + 0x28 + lVar13);
        lVar12 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar4 + 0x130) * 8) * 0x30;
        local_90 = *(longlong *)(lVar11 + 0x10 + lVar12);
        local_80 = *(longlong *)(lVar11 + 0x28 + lVar12);
        local_120 = *(longlong *)(lVar7 + 0x50);
        local_68 = 0;
      }
      uVar8 = (ulonglong)*(uint *)(local_120 + 8 + local_68 * 0x10);
    }
    else {
      if (local_68 == local_70) {
        if (local_f8 == local_f0) {
          return;
        }
        while( true ) {
          local_70 = *(ulonglong *)
                      (*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)*local_f8 * 0x48);
          if (local_70 != 0) break;
          local_f8 = local_f8 + 1;
          if (local_f8 == local_f0) {
            return;
          }
        }
        lVar11 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*local_f8 * 0x48;
        if ((*(ulonglong *)(lVar4 + 0x110) < *(ulonglong *)(lVar11 + 0x40)) &&
           (uVar8 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8)
           , uVar8 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar8 * 0x30);
        }
        else {
          local_158 = 0;
        }
        lVar7 = *(longlong *)(lVar11 + 0x38);
        if (*(ulonglong *)(lVar4 + 0x120) < *(ulonglong *)(lVar11 + 0x40)) {
          uVar8 = *(ulonglong *)(lVar7 + *(ulonglong *)(lVar4 + 0x120) * 8);
          lVar11 = *(longlong *)(lVar11 + 0x18);
          if (uVar8 == 0) goto LAB_142317314;
          local_160 = *(longlong *)(lVar11 + 0x10 + ~uVar8 * 0x30);
        }
        else {
          lVar11 = *(longlong *)(lVar11 + 0x18);
LAB_142317314:
          local_160 = 0;
        }
        local_f8 = local_f8 + 1;
        lVar12 = ~*(ulonglong *)(lVar7 + *(longlong *)(lVar4 + 0x128) * 8) * 0x30;
        local_138 = *(longlong *)(lVar11 + 0x10 + lVar12);
        local_140 = *(longlong *)(lVar11 + 0x28 + lVar12);
        lVar7 = ~*(ulonglong *)(lVar7 + *(longlong *)(lVar4 + 0x130) * 8) * 0x30;
        local_90 = *(longlong *)(lVar11 + 0x10 + lVar7);
        local_80 = *(longlong *)(lVar11 + 0x28 + lVar7);
        local_68 = 0;
      }
      uVar8 = local_68 & 0xffffffff;
    }
    lVar11 = uVar8 * 0x30 + local_158;
    puVar1 = (undefined4 *)(local_140 + uVar8 * 4);
    puVar2 = (uint *)(local_160 + uVar8 * 0x10);
    puVar9 = (ulonglong *)(uVar8 * 0x10 + local_138);
    uVar10 = *puVar2;
    fVar14 = *(float *)(lVar11 + 0x18);
    if ((uVar10 & 1) != 0) {
      fVar14 = (float)puVar2[3];
    }
    fVar14 = fVar14 - *(float *)(lVar11 + 0x18);
    uVar3 = *(ulonglong *)(lVar11 + 0x10);
    auVar20._8_8_ = 0;
    auVar20._0_8_ = uVar3;
    auVar16._0_8_ = CONCAT44((int)(uVar10 << 0x1f) >> 0x1f,(int)(uVar10 << 0x1f) >> 0x1f);
    auVar16._8_8_ = 0;
    auVar19._8_8_ = 0;
    auVar19._0_8_ = *(ulonglong *)(puVar2 + 1) & auVar16._0_8_;
    auVar19 = ~auVar16 & auVar20 | auVar19;
    fVar15 = auVar19._0_4_ - (float)uVar3;
    fVar17 = auVar19._4_4_ - (float)(uVar3 >> 0x20);
    fVar18 = 1.0 / SQRT(fVar17 * fVar17 + fVar15 * fVar15 + fVar14 * fVar14);
    bVar6 = (int)fVar18 - 1U < 0x7fffff ||
            (int)ABS(fVar18) - 0x800000U < 0x7f000000 && -1 < (int)fVar18;
    if (bVar6) {
      fVar14 = fVar14 * fVar18;
    }
    else {
      fVar14 = 0.0;
    }
    local_68 = local_68 + 1;
    uVar10 = (uint)bVar6;
    *puVar1 = local_ac;
    *puVar9 = CONCAT44((int)(uVar10 << 0x1f) >> 0x1f,(int)(uVar10 << 0x1f) >> 0x1f) &
              CONCAT44(fVar17 * fVar18,fVar15 * fVar18);
    *(float *)(puVar9 + 1) = fVar14;
    *(undefined4 *)(uVar8 * 4 + local_80) = local_74;
    *(undefined4 *)(uVar8 * 4 + local_90) = 0x3f800000;
    if (*(float *)(lVar11 + 0x14) <= 1.0 && *(float *)(lVar11 + 0x14) != 1.0) {
      fVar14 = *(float *)((longlong)puVar9 + 4);
      if (fVar14 <= 0.25) {
        fVar14 = 0.25;
      }
      *puVar1 = local_ac;
      *(float *)((longlong)puVar9 + 4) = fVar14;
    }
  } while( true );
}

