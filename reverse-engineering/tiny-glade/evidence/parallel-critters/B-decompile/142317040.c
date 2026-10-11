
void __cdecl system_critters::firefly::set_firefly_velocities_towards_goals(void)

{
  undefined4 *puVar1;
  uint *puVar2;
  ulonglong uVar3;
  longlong lVar4;
  longlong lVar5;
  bool bVar6;
  float fVar7;
  float fVar8;
  longlong *in_RCX;
  longlong lVar9;
  ulonglong uVar10;
  ulonglong *puVar11;
  uint uVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  float fVar16;
  float fVar17;
  float fVar19;
  undefined1 auVar18 [16];
  float fVar20;
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
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
  fVar8 = __real_3e800000;
  fVar7 = __real_3f800000;
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
        lVar9 = *(longlong *)(lVar5 + 0x100) + (ulonglong)*local_f8 * 0xa0;
        uVar10 = (ulonglong)*(uint *)(lVar9 + 0x94);
        lVar13 = *(longlong *)(lVar5 + 0x1a8) + uVar10 * 0x48;
        uVar10 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x40 + uVar10 * 0x48);
        if ((*(ulonglong *)(lVar4 + 0x110) < uVar10) &&
           (uVar3 = *(ulonglong *)(*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8)
           , uVar3 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar3 * 0x30);
        }
        else {
          local_158 = 0;
        }
        lVar14 = *(longlong *)(lVar13 + 0x38);
        if (*(ulonglong *)(lVar4 + 0x120) < uVar10) {
          uVar10 = *(ulonglong *)(lVar14 + *(ulonglong *)(lVar4 + 0x120) * 8);
          lVar13 = *(longlong *)(lVar13 + 0x18);
          if (uVar10 == 0) goto LAB_14231737a;
          local_160 = *(longlong *)(lVar13 + 0x10 + ~uVar10 * 0x30);
        }
        else {
          lVar13 = *(longlong *)(lVar13 + 0x18);
LAB_14231737a:
          local_160 = 0;
        }
        local_f8 = local_f8 + 1;
        lVar15 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar4 + 0x128) * 8) * 0x30;
        local_138 = *(longlong *)(lVar13 + 0x10 + lVar15);
        local_140 = *(longlong *)(lVar13 + 0x28 + lVar15);
        lVar14 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar4 + 0x130) * 8) * 0x30;
        local_90 = *(longlong *)(lVar13 + 0x10 + lVar14);
        local_80 = *(longlong *)(lVar13 + 0x28 + lVar14);
        local_120 = *(longlong *)(lVar9 + 0x50);
        local_68 = 0;
      }
      uVar10 = (ulonglong)*(uint *)(local_120 + 8 + local_68 * 0x10);
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
        lVar13 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)*local_f8 * 0x48;
        if ((*(ulonglong *)(lVar4 + 0x110) < *(ulonglong *)(lVar13 + 0x40)) &&
           (uVar10 = *(ulonglong *)
                      (*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8),
           uVar10 != 0)) {
          local_158 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar10 * 0x30);
        }
        else {
          local_158 = 0;
        }
        lVar9 = *(longlong *)(lVar13 + 0x38);
        if (*(ulonglong *)(lVar4 + 0x120) < *(ulonglong *)(lVar13 + 0x40)) {
          uVar10 = *(ulonglong *)(lVar9 + *(ulonglong *)(lVar4 + 0x120) * 8);
          lVar13 = *(longlong *)(lVar13 + 0x18);
          if (uVar10 == 0) goto LAB_142317314;
          local_160 = *(longlong *)(lVar13 + 0x10 + ~uVar10 * 0x30);
        }
        else {
          lVar13 = *(longlong *)(lVar13 + 0x18);
LAB_142317314:
          local_160 = 0;
        }
        local_f8 = local_f8 + 1;
        lVar14 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar4 + 0x128) * 8) * 0x30;
        local_138 = *(longlong *)(lVar13 + 0x10 + lVar14);
        local_140 = *(longlong *)(lVar13 + 0x28 + lVar14);
        lVar9 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar4 + 0x130) * 8) * 0x30;
        local_90 = *(longlong *)(lVar13 + 0x10 + lVar9);
        local_80 = *(longlong *)(lVar13 + 0x28 + lVar9);
        local_68 = 0;
      }
      uVar10 = local_68 & 0xffffffff;
    }
    lVar13 = uVar10 * 0x30 + local_158;
    puVar1 = (undefined4 *)(local_140 + uVar10 * 4);
    puVar2 = (uint *)(local_160 + uVar10 * 0x10);
    puVar11 = (ulonglong *)(uVar10 * 0x10 + local_138);
    uVar12 = *puVar2;
    fVar16 = *(float *)(lVar13 + 0x18);
    if ((uVar12 & 1) != 0) {
      fVar16 = (float)puVar2[3];
    }
    fVar16 = fVar16 - *(float *)(lVar13 + 0x18);
    uVar3 = *(ulonglong *)(lVar13 + 0x10);
    auVar22._8_8_ = 0;
    auVar22._0_8_ = uVar3;
    auVar18._0_8_ = CONCAT44((int)(uVar12 << 0x1f) >> 0x1f,(int)(uVar12 << 0x1f) >> 0x1f);
    auVar18._8_8_ = 0;
    auVar21._8_8_ = 0;
    auVar21._0_8_ = *(ulonglong *)(puVar2 + 1) & auVar18._0_8_;
    auVar21 = ~auVar18 & auVar22 | auVar21;
    fVar17 = auVar21._0_4_ - (float)uVar3;
    fVar19 = auVar21._4_4_ - (float)(uVar3 >> 0x20);
    fVar20 = fVar7 / SQRT(fVar19 * fVar19 + fVar17 * fVar17 + fVar16 * fVar16);
    bVar6 = (int)fVar20 - 1U < 0x7fffff ||
            (int)ABS(fVar20) - 0x800000U < 0x7f000000 && -1 < (int)fVar20;
    if (bVar6) {
      fVar16 = fVar16 * fVar20;
    }
    else {
      fVar16 = 0.0;
    }
    local_68 = local_68 + 1;
    uVar12 = (uint)bVar6;
    *puVar1 = local_ac;
    *puVar11 = CONCAT44((int)(uVar12 << 0x1f) >> 0x1f,(int)(uVar12 << 0x1f) >> 0x1f) &
               CONCAT44(fVar19 * fVar20,fVar17 * fVar20);
    *(float *)(puVar11 + 1) = fVar16;
    *(undefined4 *)(uVar10 * 4 + local_80) = local_74;
    *(undefined4 *)(uVar10 * 4 + local_90) = 0x3f800000;
    if (*(float *)(lVar13 + 0x14) <= fVar7 && fVar7 != *(float *)(lVar13 + 0x14)) {
      fVar16 = *(float *)((longlong)puVar11 + 4);
      if (*(float *)((longlong)puVar11 + 4) <= fVar8) {
        fVar16 = fVar8;
      }
      *puVar1 = local_ac;
      *(float *)((longlong)puVar11 + 4) = fVar16;
    }
  } while( true );
}

