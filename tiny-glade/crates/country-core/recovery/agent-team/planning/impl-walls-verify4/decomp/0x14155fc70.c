
void __cdecl
country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::widget
          (void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  char *pcVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  byte bVar7;
  uint extraout_EAX;
  uint extraout_EAX_00;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong uVar8;
  undefined8 *in_RCX;
  undefined4 uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong *in_R8;
  longlong lVar12;
  ulonglong uVar13;
  uint uVar14;
  ulonglong *in_R9;
  ushort uVar15;
  longlong lVar16;
  longlong lVar17;
  char cVar18;
  char cVar22;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  char cVar20;
  char cVar21;
  undefined1 auVar19 [16];
  float fVar23;
  float fVar24;
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  float fVar27;
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  float fVar30;
  float fVar31;
  float fVar32;
  longlong *in_stack_00000028;
  byte in_stack_00000030;
  undefined8 local_178;
  undefined8 *local_170;
  code *local_168;
  undefined4 local_15c;
  undefined1 local_158;
  undefined8 local_157;
  undefined4 local_14f;
  undefined2 local_14b;
  undefined1 local_149;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 **local_128;
  float local_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  int local_108;
  int local_f0;
  int local_d8;
  undefined1 local_c0;
  undefined1 local_b0;
  undefined8 local_af;
  undefined4 local_a7;
  undefined2 local_a3;
  undefined1 local_a1;
  undefined8 local_a0;
  undefined8 uStack_98;
  byte local_90;
  undefined1 local_8f;
  
  if (*(char *)(in_RCX + 0xb) == '\0') {
    if (in_R9[3] == 0) {
LAB_141560459:
                    // WARNING: Subroutine does not return
      core::option::expect_failed();
    }
    lVar4 = *in_stack_00000028;
    lVar17 = in_stack_00000028[1];
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    uVar10 = *in_R9;
    bVar7 = (byte)(extraout_RAX_00 >> 0x39);
    auVar19 = ZEXT216(CONCAT11(bVar7,bVar7));
    auVar19 = pshuflw(auVar19,auVar19,0);
    lVar12 = 0;
    uVar13 = extraout_RAX_00;
    do {
      uVar13 = in_R9[1] & uVar13;
      pcVar3 = (char *)(uVar10 + uVar13);
      cVar18 = auVar19[0];
      auVar29[0] = -(*pcVar3 == cVar18);
      cVar20 = auVar19[1];
      auVar29[1] = -(pcVar3[1] == cVar20);
      cVar21 = auVar19[2];
      auVar29[2] = -(pcVar3[2] == cVar21);
      cVar22 = auVar19[3];
      auVar29[3] = -(pcVar3[3] == cVar22);
      auVar29[4] = -(pcVar3[4] == cVar18);
      auVar29[5] = -(pcVar3[5] == cVar20);
      auVar29[6] = -(pcVar3[6] == cVar21);
      auVar29[7] = -(pcVar3[7] == cVar22);
      auVar29[8] = -(pcVar3[8] == cVar18);
      auVar29[9] = -(pcVar3[9] == cVar20);
      auVar29[10] = -(pcVar3[10] == cVar21);
      auVar29[0xb] = -(pcVar3[0xb] == cVar22);
      auVar29[0xc] = -(pcVar3[0xc] == cVar18);
      auVar29[0xd] = -(pcVar3[0xd] == cVar20);
      auVar29[0xe] = -(pcVar3[0xe] == cVar21);
      auVar29[0xf] = -(pcVar3[0xf] == cVar22);
      uVar15 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe | (ushort)(auVar29[0xf] >> 7) << 0xf;
      uVar14 = (uint)uVar15;
      while (uVar15 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar14; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar11 = uVar5 + uVar13 & in_R9[1];
        if (in_RCX[6] == *(longlong *)((uVar10 - 0x10) + uVar11 * -0x10)) {
          uStack_130 = *(ulonglong *)((uVar10 - 8) + uVar11 * -0x10);
          uVar13 = uStack_130 & 0xffffffff;
          if ((uVar13 < *(ulonglong *)(lVar17 + 0x10)) &&
             (uVar10 = *(ulonglong *)(lVar17 + 8),
             *(int *)(uVar10 + uVar13 * 0x14) == (int)(uStack_130 >> 0x20))) {
            lVar12 = uVar10 + uVar13 * 0x14;
            uVar14 = *(uint *)(lVar12 + 4);
            uVar10 = (ulonglong)uVar14;
            if (uVar10 != 0xffffffff) {
              uVar9 = 0;
              if ((*(ulonglong *)(lVar4 + 0x38) <= uVar10) ||
                 ((*(ulonglong *)(*(longlong *)(lVar4 + 0x28) + (ulonglong)(uVar14 >> 6) * 8) >>
                   (uVar10 & 0x3f) & 1) == 0)) goto LAB_14156059b;
              if (*(longlong *)
                   (*(longlong *)
                     (*(longlong *)
                       (*(longlong *)(lVar17 + 0x1a8) + 0x18 +
                       (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48) + 0x10 +
                     ~*(ulonglong *)
                       (*(longlong *)
                         (*(longlong *)(lVar17 + 0x1a8) + 0x38 +
                         (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48) +
                       *(longlong *)(lVar4 + 0x110) * 8) * 0x30) + 0x10 +
                   (ulonglong)*(uint *)(lVar12 + 0x10) * 0x68) == 0) {
                local_158 = 7;
                goto LAB_141560341;
              }
              ::utils::geometry::rectangle::Rectangle2d::get_point_u();
              ::utils::geometry::rectangle::Rectangle2d::get_point_u();
              local_178 = CONCAT44(local_178._4_4_,extraout_XMM0_Da_03);
              if ((extraout_XMM0_Da_03 < 0.0) || (1.0 < extraout_XMM0_Da_03)) {
                local_170 = &local_178;
                local_168 = core::fmt::float::impl$5::fmt;
                local_138 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
                uStack_130 = 1;
                uStack_118 = 0;
                uStack_114 = 0;
                local_128 = &local_170;
                local_120 = 1.4013e-45;
                uStack_11c = 0;
                    // WARNING: Subroutine does not return
                core::panicking::panic_fmt();
              }
              if (NAN(extraout_XMM0_Da_03)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              local_178 = CONCAT44(local_178._4_4_,extraout_XMM0_Da_04);
              if ((extraout_XMM0_Da_04 < 0.0) || (1.0 < extraout_XMM0_Da_04)) {
                local_170 = &local_178;
                local_168 = core::fmt::float::impl$5::fmt;
                local_138 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
                uStack_130 = 1;
                uStack_118 = 0;
                uStack_114 = 0;
                local_128 = &local_170;
                local_120 = 1.4013e-45;
                uStack_11c = 0;
                    // WARNING: Subroutine does not return
                core::panicking::panic_fmt();
              }
              if (NAN(extraout_XMM0_Da_04)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              local_138 = (undefined **)CONCAT44(extraout_XMM0_Da_03,1);
              uStack_130 = CONCAT44(1,extraout_XMM0_Da_04);
              local_128 = (undefined8 **)0x3e933333;
              local_120 = -0.5;
              uStack_11c = CONCAT31(uStack_11c._1_3_,1);
              resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris();
              goto LAB_14156033c;
            }
          }
          uVar14 = (uint)uVar10;
          uVar9 = 1;
LAB_14156059b:
          local_138 = (undefined **)CONCAT44(uVar14,uVar9);
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
        uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
      }
      auVar26[0] = -(*pcVar3 == -1);
      auVar26[1] = -(pcVar3[1] == -1);
      auVar26[2] = -(pcVar3[2] == -1);
      auVar26[3] = -(pcVar3[3] == -1);
      auVar26[4] = -(pcVar3[4] == -1);
      auVar26[5] = -(pcVar3[5] == -1);
      auVar26[6] = -(pcVar3[6] == -1);
      auVar26[7] = -(pcVar3[7] == -1);
      auVar26[8] = -(pcVar3[8] == -1);
      auVar26[9] = -(pcVar3[9] == -1);
      auVar26[10] = -(pcVar3[10] == -1);
      auVar26[0xb] = -(pcVar3[0xb] == -1);
      auVar26[0xc] = -(pcVar3[0xc] == -1);
      auVar26[0xd] = -(pcVar3[0xd] == -1);
      auVar26[0xe] = -(pcVar3[0xe] == -1);
      auVar26[0xf] = -(pcVar3[0xf] == -1);
      if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar26 >> 0x3f,0) & 1) != 0) || (SUB161(auVar26 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar26 >> 0x4f,0) & 1) != 0) || (SUB161(auVar26 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar26 >> 0x5f,0) & 1) != 0) || (SUB161(auVar26 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar26 >> 0x6f,0) & 1) != 0) || (SUB161(auVar26 >> 0x77,0) & 1) != 0) ||
          auVar26[0xf] < '\0') goto LAB_141560459;
      uVar13 = uVar13 + lVar12 + 0x10;
      lVar12 = lVar12 + 0x10;
    } while( true );
  }
  lVar4 = in_RCX[6];
  local_138 = (undefined **)lVar4;
  if (in_R8[3] == 0) {
LAB_14156044d:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar17 = *in_R8;
  bVar7 = (byte)(extraout_RAX >> 0x39);
  auVar19 = ZEXT216(CONCAT11(bVar7,bVar7));
  auVar19 = pshuflw(auVar19,auVar19,0);
  lVar12 = 0;
  uVar10 = extraout_RAX;
  do {
    uVar10 = in_R8[1] & uVar10;
    pcVar3 = (char *)(lVar17 + uVar10);
    cVar18 = auVar19[0];
    auVar28[0] = -(*pcVar3 == cVar18);
    cVar20 = auVar19[1];
    auVar28[1] = -(pcVar3[1] == cVar20);
    cVar21 = auVar19[2];
    auVar28[2] = -(pcVar3[2] == cVar21);
    cVar22 = auVar19[3];
    auVar28[3] = -(pcVar3[3] == cVar22);
    auVar28[4] = -(pcVar3[4] == cVar18);
    auVar28[5] = -(pcVar3[5] == cVar20);
    auVar28[6] = -(pcVar3[6] == cVar21);
    auVar28[7] = -(pcVar3[7] == cVar22);
    auVar28[8] = -(pcVar3[8] == cVar18);
    auVar28[9] = -(pcVar3[9] == cVar20);
    auVar28[10] = -(pcVar3[10] == cVar21);
    auVar28[0xb] = -(pcVar3[0xb] == cVar22);
    auVar28[0xc] = -(pcVar3[0xc] == cVar18);
    auVar28[0xd] = -(pcVar3[0xd] == cVar20);
    auVar28[0xe] = -(pcVar3[0xe] == cVar21);
    auVar28[0xf] = -(pcVar3[0xf] == cVar22);
    uVar15 = (ushort)(SUB161(auVar28 >> 7,0) & 1) | (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe | (ushort)(auVar28[0xf] >> 7) << 0xf;
    uVar14 = (uint)uVar15;
    while (uVar15 != 0) {
      uVar5 = 0;
      for (uVar6 = uVar14; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar5 = uVar5 + 1;
      }
      lVar16 = (uVar5 + uVar10 & in_R8[1]) * -0x2a8;
      if (*(longlong *)(lVar17 + -0x2a8 + lVar16) == lVar4) {
        if ((*(ulonglong *)(lVar17 + -0x290 + lVar16) < 2) ||
           (uVar9 = 0, *(float *)(lVar17 + -0x270 + lVar16) <= 0.0)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        lVar17 = lVar17 + lVar16;
        ::utils::geometry::rectangle::Rectangle2d::get_edge_normal();
        uVar10 = *(ulonglong *)(lVar17 + -0x1b8);
        local_15c = uVar9;
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        ::utils::curve::Curve::
        get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>
                  ();
        uVar11 = (ulonglong)extraout_EAX;
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        ::utils::curve::Curve::
        get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>
                  ();
        uVar13 = *(ulonglong *)(lVar17 + -0x208);
        if (uVar13 <= uVar11) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        uVar1 = uVar11 + 1;
        if (uVar13 <= uVar1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        uVar8 = (ulonglong)extraout_EAX_00;
        if (uVar13 <= uVar8) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        uVar2 = uVar8 + 1;
        if (uVar13 <= uVar2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (uVar10 <= uVar11) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (uVar10 <= uVar1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (uVar10 <= uVar8) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (uVar10 <= uVar2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        lVar4 = *(longlong *)(lVar17 + -0x210);
        fVar27 = *(float *)(lVar4 + 4 + uVar1 * 0xc) * extraout_XMM0_Da +
                 (1.0 - extraout_XMM0_Da) * *(float *)(lVar4 + 4 + uVar11 * 0xc);
        fVar24 = *(float *)(lVar4 + 4 + uVar2 * 0xc) * extraout_XMM0_Da_00 +
                 (1.0 - extraout_XMM0_Da_00) * *(float *)(lVar4 + 4 + uVar8 * 0xc);
        fVar23 = fVar24;
        if (fVar24 <= fVar27) {
          fVar23 = fVar27;
        }
        fVar23 = (float)(~-(uint)NAN(fVar27) & (uint)fVar23 | -(uint)NAN(fVar27) & (uint)fVar24);
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        fVar24 = fVar23;
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        local_178 = CONCAT44(fVar24 * 0.5 + fVar23 * 0.5,
                             extraout_XMM0_Da_02 * 0.5 + extraout_XMM0_Da_01 * 0.5);
        ::utils::swizzlers::impl$1::xvy();
        ::utils::swizzlers::impl$0::x0y();
        fVar31 = SUB84(local_170,0) + SUB84(local_138,0) * 0.63 * 0.5;
        fVar32 = (float)((ulonglong)local_170 >> 0x20) +
                 (float)((ulonglong)local_138 >> 0x20) * 0.63 * 0.5;
        fVar30 = (float)uStack_130 * 0.63 * 0.5 + local_168._0_4_;
        ::utils::swizzlers::impl$0::x0y();
        ::utils::rotation_from_normal();
        in_RCX[1] = 0;
        *in_RCX = 1;
        *(float *)(in_RCX + 2) = (float)local_138;
        *(float *)((longlong)in_RCX + 0x14) = local_138._4_4_;
        *(float *)(in_RCX + 3) = (float)uStack_130;
        *(float *)((longlong)in_RCX + 0x1c) = uStack_130._4_4_;
        in_RCX[4] = CONCAT44(fVar32,fVar31);
        *(float *)(in_RCX + 5) = fVar30;
        fVar24 = uStack_130._4_4_ * uStack_130._4_4_;
        fVar27 = (float)uStack_130 * (float)uStack_130 +
                 local_138._4_4_ * local_138._4_4_ + (float)local_138 * (float)local_138;
        fVar23 = (float)uStack_130 * 1.0 + local_138._4_4_ * 0.0 + (float)local_138 * 0.0;
        fVar23 = fVar23 + fVar23;
        uStack_130._4_4_ = uStack_130._4_4_ + uStack_130._4_4_;
        local_120 = uStack_130._4_4_ * ((float)local_138 * 0.0 - local_138._4_4_ * 0.0) +
                    fVar23 * (float)uStack_130 + (fVar24 - fVar27) * 1.0 + fVar30;
        local_128 = (undefined8 **)
                    CONCAT44(fVar32 + uStack_130._4_4_ *
                                      ((float)uStack_130 * 0.0 - (float)local_138 * 1.0) +
                                      fVar23 * local_138._4_4_ + (fVar24 - fVar27) * 0.0,
                             fVar31 + uStack_130._4_4_ *
                                      (local_138._4_4_ * 1.0 - (float)uStack_130 * 0.0) +
                                      fVar23 * (float)local_138 + (fVar24 - fVar27) * 0.0);
        uStack_11c = 0x3f800000;
        uStack_118 = 0x3f800000;
        uStack_114 = 0x3f800000;
        ui_edit_rectangle_orientation::box_trigger_volume();
LAB_14156033c:
        local_158 = 2;
LAB_141560341:
        bVar7 = in_stack_00000030 ^ 1;
        local_f0 = (uint)bVar7 + (uint)bVar7 * 2;
        local_90 = *(byte *)(in_RCX + 0xb) ^ 1;
        local_138 = (undefined **)0x0;
        uStack_130 = 8;
        local_128 = (undefined8 **)0x0;
        local_120 = (float)(uint)bVar7;
        local_108 = (int)local_120 + (uint)bVar7;
        uStack_118 = 0;
        uStack_114 = 0;
        local_c0 = 0;
        local_b0 = local_158;
        local_af = local_157;
        local_a7 = local_14f;
        local_a3 = local_14b;
        local_a1 = local_149;
        local_a0 = local_148;
        uStack_98 = uStack_140;
        local_8f = 1;
        local_d8 = local_f0;
        systems::ui::resources::ui_state::UiState::
        add<ref_<country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit>_>
                  ();
        return;
      }
      uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
      uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
    }
    auVar25[0] = -(*pcVar3 == -1);
    auVar25[1] = -(pcVar3[1] == -1);
    auVar25[2] = -(pcVar3[2] == -1);
    auVar25[3] = -(pcVar3[3] == -1);
    auVar25[4] = -(pcVar3[4] == -1);
    auVar25[5] = -(pcVar3[5] == -1);
    auVar25[6] = -(pcVar3[6] == -1);
    auVar25[7] = -(pcVar3[7] == -1);
    auVar25[8] = -(pcVar3[8] == -1);
    auVar25[9] = -(pcVar3[9] == -1);
    auVar25[10] = -(pcVar3[10] == -1);
    auVar25[0xb] = -(pcVar3[0xb] == -1);
    auVar25[0xc] = -(pcVar3[0xc] == -1);
    auVar25[0xd] = -(pcVar3[0xd] == -1);
    auVar25[0xe] = -(pcVar3[0xe] == -1);
    auVar25[0xf] = -(pcVar3[0xf] == -1);
    if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar25 >> 0x37,0) & 1) != 0) || (SUB161(auVar25 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar25 >> 0x47,0) & 1) != 0) || (SUB161(auVar25 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar25 >> 0x57,0) & 1) != 0) || (SUB161(auVar25 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar25 >> 0x67,0) & 1) != 0) || (SUB161(auVar25 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar25 >> 0x77,0) & 1) != 0) || auVar25[0xf] < '\0') goto LAB_14156044d;
    uVar10 = uVar10 + lVar12 + 0x10;
    lVar12 = lVar12 + 0x10;
  } while( true );
}

