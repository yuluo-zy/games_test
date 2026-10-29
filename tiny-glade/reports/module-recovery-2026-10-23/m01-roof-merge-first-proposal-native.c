
void __cdecl
core::ops::function::impls::impl$3::
call_mut<tuple_<ref_<system_decorator::collision_detection::DecoratorIntersection>_>,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env_0>
          (void)

{
  char *pcVar1;
  float fVar2;
  float fVar3;
  undefined4 uVar4;
  undefined2 uVar5;
  int iVar6;
  longlong *plVar7;
  longlong lVar8;
  longlong lVar9;
  longlong *plVar10;
  uint uVar11;
  uint uVar12;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong extraout_RAX_04;
  float *extraout_RAX_05;
  undefined4 *in_RCX;
  undefined1 uVar13;
  longlong *in_RDX;
  ulonglong uVar14;
  longlong lVar15;
  int *in_R8;
  longlong lVar16;
  byte bVar17;
  ushort uVar18;
  uint uVar19;
  byte bVar20;
  char cVar21;
  char cVar25;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  char cVar23;
  char cVar24;
  undefined1 auVar22 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  float fVar28;
  float fVar29;
  undefined1 local_11a [2];
  undefined8 local_118;
  code *pcStack_110;
  undefined8 local_108;
  int local_100;
  int local_fc;
  undefined **local_f8;
  undefined8 local_f0;
  undefined8 **local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 *local_c8;
  code *pcStack_c0;
  longlong local_b8;
  longlong lStack_b0;
  longlong local_a8;
  longlong local_98;
  longlong lStack_90;
  undefined4 local_88;
  
  plVar7 = (longlong *)*in_RDX;
  iVar6 = *in_R8;
  lVar8 = *plVar7;
  local_fc = iVar6;
  if (*(longlong *)(lVar8 + 0x48) == 0) {
LAB_141fe6bf9:
                    // WARNING: Subroutine does not return
    option::expect_failed();
  }
  hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
  lVar9 = *(longlong *)(lVar8 + 0x30);
  bVar20 = (byte)(extraout_RAX >> 0x39);
  auVar22 = ZEXT216(CONCAT11(bVar20,bVar20));
  auVar22 = pshuflw(auVar22,auVar22,0);
  lVar16 = 0;
  uVar14 = extraout_RAX;
  do {
    uVar14 = *(ulonglong *)(lVar8 + 0x38) & uVar14;
    pcVar1 = (char *)(lVar9 + uVar14);
    cVar21 = auVar22[0];
    auVar27[0] = -(*pcVar1 == cVar21);
    cVar23 = auVar22[1];
    auVar27[1] = -(pcVar1[1] == cVar23);
    cVar24 = auVar22[2];
    auVar27[2] = -(pcVar1[2] == cVar24);
    cVar25 = auVar22[3];
    auVar27[3] = -(pcVar1[3] == cVar25);
    auVar27[4] = -(pcVar1[4] == cVar21);
    auVar27[5] = -(pcVar1[5] == cVar23);
    auVar27[6] = -(pcVar1[6] == cVar24);
    auVar27[7] = -(pcVar1[7] == cVar25);
    auVar27[8] = -(pcVar1[8] == cVar21);
    auVar27[9] = -(pcVar1[9] == cVar23);
    auVar27[10] = -(pcVar1[10] == cVar24);
    auVar27[0xb] = -(pcVar1[0xb] == cVar25);
    auVar27[0xc] = -(pcVar1[0xc] == cVar21);
    auVar27[0xd] = -(pcVar1[0xd] == cVar23);
    auVar27[0xe] = -(pcVar1[0xe] == cVar24);
    auVar27[0xf] = -(pcVar1[0xf] == cVar25);
    uVar18 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe | (ushort)(auVar27[0xf] >> 7) << 0xf;
    uVar19 = (uint)uVar18;
    while (uVar18 != 0) {
      uVar11 = 0;
      for (uVar12 = uVar19; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
        uVar11 = uVar11 + 1;
      }
      lVar15 = (uVar11 + uVar14 & *(ulonglong *)(lVar8 + 0x38)) * -0x50;
      if (iVar6 == *(int *)(lVar9 + -0x50 + lVar15)) {
        local_100 = iVar6;
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
        country_core::resources::walls::decorator::DecoratorRank::new();
        uVar13 = extraout_AL;
        country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
        local_118._0_2_ = CONCAT11(uVar13,extraout_AL_00);
        std::collections::hash::map::HashMap::
        get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                  ();
        if (extraout_RAX_00 == 0) {
          local_c8 = &local_118;
          pcStack_c0 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
          local_e8 = &local_c8;
LAB_141fe6eca:
          local_d8 = 0;
          local_f0 = 1;
          local_f8 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
          local_e0 = 1;
                    // WARNING: Subroutine does not return
          panicking::panic_fmt();
        }
        lVar9 = lVar9 + lVar15;
        plVar10 = (longlong *)plVar7[2];
        bVar20 = *(char *)((longlong)plVar10 + 0x46) - 0x38;
        if (7 < bVar20) {
          bVar20 = 6;
        }
        cVar21 = *(char *)(lVar9 + -2);
        bVar17 = cVar21 - 0x38;
        if (7 < bVar17) {
          bVar17 = 6;
        }
        if ((bVar20 != 6 || cVar21 == *(char *)((longlong)plVar10 + 0x46)) && (bVar17 == bVar20)) {
          fVar29 = *(float *)(extraout_RAX_00 + 0x30);
          uVar14 = (ulonglong)*(byte *)(lVar9 + -4) + (ulonglong)*(byte *)((longlong)plVar10 + 0x44)
          ;
          country_core::resources::walls::decorator::DecoratorType::max_rank();
          if (uVar14 <= extraout_RAX_01) {
            local_a8 = plVar10[4];
            local_c8 = (undefined8 *)*plVar10;
            pcStack_c0 = (code *)plVar10[1];
            local_b8 = plVar10[2];
            lStack_b0 = plVar10[3];
            uVar5 = *(undefined2 *)((longlong)plVar10 + 0x45);
            uVar13 = *(undefined1 *)((longlong)plVar10 + 0x47);
            lVar8 = plVar10[5];
            local_88 = (undefined4)plVar10[8];
            local_98 = plVar10[6];
            lStack_90 = plVar10[7];
            country_core::resources::walls::decorator::DecoratorRank::new();
            country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
            std::collections::hash::map::HashMap::
            get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                      ();
            if (extraout_RAX_02 != 0) {
              fVar29 = (*(float *)(extraout_RAX_02 + 0x30) - fVar29) * 0.5;
              if ((*(byte *)(plVar7[3] + 8) & 1) == 0) {
                pcStack_110 = (code *)CONCAT44(pcStack_110._4_4_,
                                               *(undefined4 *)((undefined8 *)plVar7[4] + 1));
                local_118 = *(undefined8 **)plVar7[4];
                country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped
                          ();
                country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
                fVar29 = fVar29 / extraout_XMM0_Da_00;
              }
              else {
                pcStack_110 = (code *)CONCAT44(pcStack_110._4_4_,
                                               *(undefined4 *)((undefined8 *)plVar7[4] + 1));
                local_118 = *(undefined8 **)plVar7[4];
                country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped
                          ();
                country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
                local_118 = (undefined8 *)*extraout_RAX_03;
                pcStack_110 = (code *)extraout_RAX_03[1];
                local_108 = extraout_RAX_03[2];
                ::utils::geometry::rectangle::Rectangle2d::get_closest_edge_id();
                ::utils::geometry::rectangle::Rectangle2d::edge_length();
                fVar29 = (fVar29 / extraout_XMM0_Da) * 0.25;
              }
              country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof();
              country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof();
              fVar2 = extraout_RAX_05[1];
              fVar3 = *(float *)plVar7[5];
              uVar19 = -(uint)(*extraout_RAX_05 < fVar3);
              fVar28 = (float)(~uVar19 & (uint)-fVar29 | (uint)fVar29 & uVar19) + *extraout_RAX_05;
              uVar4 = *(undefined4 *)(extraout_RAX_04 + 8);
              fVar29 = fVar2 - ((float *)plVar7[5])[1];
              *in_RCX = 0;
              in_RCX[1] = uVar4;
              in_RCX[2] = fVar28;
              fVar28 = fVar28 - fVar3;
              in_RCX[3] = fVar2;
              *(longlong *)(in_RCX + 0xc) = local_b8;
              *(longlong *)(in_RCX + 0xe) = lStack_b0;
              *(undefined8 **)(in_RCX + 8) = local_c8;
              *(code **)(in_RCX + 10) = pcStack_c0;
              *(longlong *)(in_RCX + 0x10) = local_a8;
              *(longlong *)(in_RCX + 0x12) = lVar8;
              *(longlong *)(in_RCX + 0x14) = local_98;
              *(longlong *)(in_RCX + 0x16) = lStack_90;
              in_RCX[0x18] = local_88;
              *(char *)(in_RCX + 0x19) = (char)uVar14;
              *(undefined2 *)((longlong)in_RCX + 0x65) = uVar5;
              *(undefined1 *)((longlong)in_RCX + 0x67) = uVar13;
              in_RCX[0x1a] = local_100;
              in_RCX[0x1b] = SQRT(fVar29 * fVar29 + fVar28 * fVar28);
              return;
            }
            pcStack_110 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
            local_e8 = (undefined8 **)&local_118;
            local_118 = (undefined8 *)local_11a;
            goto LAB_141fe6eca;
          }
        }
        *in_RCX = 4;
        return;
      }
      uVar18 = (ushort)(uVar19 - 1) & (ushort)uVar19;
      uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar18);
    }
    auVar26[0] = -(*pcVar1 == -1);
    auVar26[1] = -(pcVar1[1] == -1);
    auVar26[2] = -(pcVar1[2] == -1);
    auVar26[3] = -(pcVar1[3] == -1);
    auVar26[4] = -(pcVar1[4] == -1);
    auVar26[5] = -(pcVar1[5] == -1);
    auVar26[6] = -(pcVar1[6] == -1);
    auVar26[7] = -(pcVar1[7] == -1);
    auVar26[8] = -(pcVar1[8] == -1);
    auVar26[9] = -(pcVar1[9] == -1);
    auVar26[10] = -(pcVar1[10] == -1);
    auVar26[0xb] = -(pcVar1[0xb] == -1);
    auVar26[0xc] = -(pcVar1[0xc] == -1);
    auVar26[0xd] = -(pcVar1[0xd] == -1);
    auVar26[0xe] = -(pcVar1[0xe] == -1);
    auVar26[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar26 >> 0x37,0) & 1) != 0) || (SUB161(auVar26 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar26 >> 0x47,0) & 1) != 0) || (SUB161(auVar26 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar26 >> 0x57,0) & 1) != 0) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') goto LAB_141fe6bf9;
    uVar14 = uVar14 + lVar16 + 0x10;
    lVar16 = lVar16 + 0x10;
  } while( true );
}

