
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::main_camera::main_camera_update(void)

{
  undefined1 uVar1;
  uint uVar2;
  uint uVar3;
  undefined4 uVar4;
  float *pfVar5;
  uint *puVar6;
  undefined4 *puVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  undefined8 uVar12;
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  longlong *in_RCX;
  undefined8 *in_RDX;
  float fVar15;
  float fVar16;
  float fVar17;
  undefined1 auVar18 [16];
  float fVar19;
  float fVar20;
  undefined1 auVar21 [16];
  longlong *in_stack_00000028;
  longlong in_stack_00000030;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  
  pfVar5 = *(float **)(in_stack_00000030 + 0x18);
  puVar6 = *(uint **)(in_stack_00000030 + 0xd8);
  uVar2 = *puVar6;
  uVar3 = puVar6[1];
  uVar4 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  puVar7 = (undefined4 *)in_stack_00000028[2];
  *puVar7 = uVar4;
  lVar8 = *in_stack_00000028;
  *(float *)(lVar8 + 0x4d8) = (float)uVar2 / (float)uVar3;
  lVar9 = *in_stack_00000040;
  lVar10 = *in_RCX;
  if (*(int *)(lVar9 + 0x18) != 0) {
    if ((2 < *(uint *)(lVar10 + 0x18)) &&
       (fVar15 = (float)*(undefined8 *)(lVar10 + 0x160),
       fVar19 = (float)((ulonglong)*(undefined8 *)(lVar10 + 0x160) >> 0x20),
       fVar19 * fVar19 + fVar15 * fVar15 <= 0.0)) {
      return;
    }
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    *(undefined4 *)(lVar9 + 0x18) = 0;
  }
  if ((*(uint *)(lVar10 + 0x10) < 3) || (*(uint *)(lVar10 + 0x18) < 3)) {
    *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
    lVar9 = *in_stack_00000048;
    lVar11 = *(longlong *)(lVar9 + 0x30);
    uVar12 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar11 == *(longlong *)(lVar9 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar9 + 0x28) + lVar11 * 8) = uVar12;
    *(longlong *)(lVar9 + 0x30) = lVar11 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
  }
  uVar1 = *(undefined1 *)*in_RDX;
  *puVar7 = uVar4;
  *(undefined1 *)(lVar8 + 0x344) = uVar1;
  if (*(float *)(lVar10 + 0x168) != 0.0) {
    ::utils::geometry::ray::Ray::from_mouse_ss();
    fVar19 = *pfVar5 / (float)*puVar6;
    fVar15 = (__real_3f800000 - pfVar5[1]) / (float)puVar6[1];
    fVar20 = fVar19 + fVar19 + __real_bf800000;
    fVar15 = fVar15 + fVar15 + __real_3f800000;
    fVar19 = fVar15 * *(float *)(lVar8 + 0x50) + fVar20 * *(float *)(lVar8 + 0x40);
    fVar16 = fVar15 * *(float *)(lVar8 + 0x54) + fVar20 * *(float *)(lVar8 + 0x44);
    fVar17 = fVar15 * *(float *)(lVar8 + 0x58) + fVar20 * *(float *)(lVar8 + 0x48);
    fVar15 = fVar15 * *(float *)(lVar8 + 0x5c) + fVar20 * *(float *)(lVar8 + 0x4c);
    auVar21._0_4_ = *(float *)(lVar8 + 0x60) + fVar19 + *(float *)(lVar8 + 0x70);
    auVar21._4_4_ = *(float *)(lVar8 + 100) + fVar16 + *(float *)(lVar8 + 0x74);
    auVar21._8_4_ = *(float *)(lVar8 + 0x68) + fVar17 + *(float *)(lVar8 + 0x78);
    auVar21._12_4_ = *(float *)(lVar8 + 0x6c) + fVar15 + *(float *)(lVar8 + 0x7c);
    auVar14._4_4_ = auVar21._12_4_;
    auVar14._0_4_ = auVar21._12_4_;
    auVar14._8_4_ = auVar21._12_4_;
    auVar14._12_4_ = auVar21._12_4_;
    divps(auVar21,auVar14);
    auVar18._0_4_ =
         *(float *)(lVar8 + 0x60) * ___xmm_3c23d70a3c23d70a3c23d70a3c23d70a + fVar19 +
         *(float *)(lVar8 + 0x70);
    auVar18._4_4_ = *(float *)(lVar8 + 100) * _UNK_142aab4e4 + fVar16 + *(float *)(lVar8 + 0x74);
    auVar18._8_4_ = *(float *)(lVar8 + 0x68) * _UNK_142aab4e8 + fVar17 + *(float *)(lVar8 + 0x78);
    auVar18._12_4_ = *(float *)(lVar8 + 0x6c) * _UNK_142aab4ec + fVar15 + *(float *)(lVar8 + 0x7c);
    auVar13._4_4_ = auVar18._12_4_;
    auVar13._0_4_ = auVar18._12_4_;
    auVar13._8_4_ = auVar18._12_4_;
    auVar13._12_4_ = auVar18._12_4_;
    divps(auVar18,auVar13);
                    /* WARNING: Could not recover jumptable at 0x000140913048. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&DAT_142aabd30 + DAT_142aabd44))();
    return;
  }
                    /* WARNING: Could not recover jumptable at 0x000140913152. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&DAT_142aabd04 + *(int *)(&DAT_142aabd04 + (ulonglong)*(uint *)(lVar10 + 0x10) * 4)))
            (&DAT_142aabd04 + *(int *)(&DAT_142aabd04 + (ulonglong)*(uint *)(lVar10 + 0x10) * 4));
  return;
}

