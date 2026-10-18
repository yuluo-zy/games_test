// project_onto_ray 0x140ac84a0

void __cdecl
country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray(void)

{
  float fVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined8 *in_RCX;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar4;
  float fVar6;
  float fVar7;
  undefined1 auVar5 [16];
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar15;
  float fVar16;
  undefined1 auVar14 [16];
  float *in_stack_00000028;
  undefined8 *in_stack_00000030;
  float *in_stack_00000038;
  
  if (*in_stack_00000038 <= in_stack_00000038[1]) {
    fVar1 = *(float *)(in_stack_00000030 + 1);
    fVar10 = *(float *)((longlong)in_stack_00000030 + 0x14) + fVar1;
    fVar8 = (float)*in_stack_00000030;
    fVar9 = (float)((ulonglong)*in_stack_00000030 >> 0x20);
    fVar4 = fVar9 * in_stack_00000028[0x24] + fVar8 * in_stack_00000028[0x20] +
            fVar1 * in_stack_00000028[0x28] + in_stack_00000028[0x2c];
    fVar6 = fVar9 * in_stack_00000028[0x25] + fVar8 * in_stack_00000028[0x21] +
            fVar1 * in_stack_00000028[0x29] + in_stack_00000028[0x2d];
    fVar7 = fVar9 * in_stack_00000028[0x26] + fVar8 * in_stack_00000028[0x22] +
            fVar1 * in_stack_00000028[0x2a] + in_stack_00000028[0x2e];
    fVar11 = (float)*(undefined8 *)((longlong)in_stack_00000030 + 0xc) + fVar8;
    fVar12 = (float)((ulonglong)*(undefined8 *)((longlong)in_stack_00000030 + 0xc) >> 0x20) + fVar9;
    fVar13 = fVar12 * in_stack_00000028[0x24] + fVar11 * in_stack_00000028[0x20] +
             fVar10 * in_stack_00000028[0x28] + in_stack_00000028[0x2c];
    fVar15 = fVar12 * in_stack_00000028[0x25] + fVar11 * in_stack_00000028[0x21] +
             fVar10 * in_stack_00000028[0x29] + in_stack_00000028[0x2d];
    fVar16 = fVar12 * in_stack_00000028[0x26] + fVar11 * in_stack_00000028[0x22] +
             fVar10 * in_stack_00000028[0x2a] + in_stack_00000028[0x2e];
    auVar5._0_4_ = fVar6 * in_stack_00000028[4] + fVar4 * *in_stack_00000028 +
                   fVar7 * in_stack_00000028[8] + in_stack_00000028[0xc];
    auVar5._4_4_ = fVar6 * in_stack_00000028[5] + fVar4 * in_stack_00000028[1] +
                   fVar7 * in_stack_00000028[9] + in_stack_00000028[0xd];
    auVar5._8_4_ = fVar6 * in_stack_00000028[6] + fVar4 * in_stack_00000028[2] +
                   fVar7 * in_stack_00000028[10] + in_stack_00000028[0xe];
    auVar5._12_4_ =
         fVar6 * in_stack_00000028[7] + fVar4 * in_stack_00000028[3] +
         fVar7 * in_stack_00000028[0xb] + in_stack_00000028[0xf];
    auVar14._0_4_ =
         fVar15 * in_stack_00000028[4] + fVar13 * *in_stack_00000028 + fVar16 * in_stack_00000028[8]
         + in_stack_00000028[0xc];
    auVar14._4_4_ =
         fVar15 * in_stack_00000028[5] + fVar13 * in_stack_00000028[1] +
         fVar16 * in_stack_00000028[9] + in_stack_00000028[0xd];
    auVar14._8_4_ =
         fVar15 * in_stack_00000028[6] + fVar13 * in_stack_00000028[2] +
         fVar16 * in_stack_00000028[10] + in_stack_00000028[0xe];
    auVar14._12_4_ =
         fVar15 * in_stack_00000028[7] + fVar13 * in_stack_00000028[3] +
         fVar16 * in_stack_00000028[0xb] + in_stack_00000028[0xf];
    auVar3._4_4_ = auVar5._12_4_;
    auVar3._0_4_ = auVar5._12_4_;
    auVar3._8_4_ = auVar5._12_4_;
    auVar3._12_4_ = auVar5._12_4_;
    divps(auVar5,auVar3);
    auVar2._4_4_ = auVar14._12_4_;
    auVar2._0_4_ = auVar14._12_4_;
    auVar2._8_4_ = auVar14._12_4_;
    auVar2._12_4_ = auVar14._12_4_;
    divps(auVar14,auVar2);
    ::utils::lerp();
    ::utils::lerp();
    fVar4 = extraout_XMM0_Da_00;
    if (extraout_XMM0_Da_00 <= 1e-10) {
      fVar4 = 1e-10;
    }
    fVar4 = extraout_XMM0_Da / fVar4;
    fVar6 = 1.0 - fVar4;
    *in_RCX = CONCAT44(fVar4 * fVar12 + fVar6 * fVar9,fVar4 * fVar11 + fVar6 * fVar8);
    *(float *)(in_RCX + 1) = fVar4 * fVar10 + fVar6 * fVar1;
    return;
  }
                    // WARNING: Subroutine does not return
  core::f32::impl$0::clamp::do_panic::runtime();
}

