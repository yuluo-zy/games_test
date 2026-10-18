// ui_adjust_wall_height_mode  size=1680
// void __cdecl ui_adjust_wall_height_mode(void)

void __cdecl
country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::
ui_adjust_wall_height_mode(void)

{
  char *pcVar1;
  undefined4 uVar2;
  float fVar3;
  float fVar4;
  undefined8 uVar5;
  undefined4 uVar6;
  ulonglong *puVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  undefined4 *puVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  uint uVar14;
  uint uVar15;
  uint extraout_EAX;
  ulonglong extraout_RAX;
  undefined8 *puVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong lVar20;
  ushort uVar21;
  ulonglong uVar22;
  longlong lVar23;
  longlong *in_R9;
  ulonglong uVar24;
  uint uVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  ulonglong uVar30;
  byte bVar31;
  char cVar32;
  char cVar36;
  float extraout_XMM0_Da;
  char cVar34;
  char cVar35;
  undefined1 auVar33 [16];
  float fVar37;
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  float fVar40;
  float fVar41;
  undefined8 *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong in_stack_00000060;
  undefined4 local_f8;
  float local_90;
  
  puVar7 = (ulonglong *)*in_stack_00000040;
  lVar29 = in_stack_00000040[1];
  uVar30 = *puVar7;
  uVar19 = 0;
  uVar18 = uVar30 - *(ulonglong *)(lVar29 + 0x18);
  if (uVar30 < *(ulonglong *)(lVar29 + 0x18)) {
    uVar18 = uVar19;
  }
  uVar17 = uVar30 - *(ulonglong *)(lVar29 + 0x38);
  if (uVar30 < *(ulonglong *)(lVar29 + 0x38)) {
    uVar17 = uVar19;
  }
  uVar30 = *(ulonglong *)(lVar29 + 0x10);
  uVar24 = *(ulonglong *)(lVar29 + 0x30);
  uVar26 = uVar24 - uVar17;
  if (uVar24 < uVar17) {
    uVar26 = uVar19;
  }
  uVar22 = uVar30 - uVar18;
  if (uVar30 < uVar18) {
    uVar22 = uVar19;
  }
  uVar19 = *(ulonglong *)(lVar29 + 0x40);
  lVar20 = *(longlong *)(lVar29 + 8);
  lVar29 = *(longlong *)(lVar29 + 0x28);
  *puVar7 = (uVar19 - uVar26) - uVar22;
  if (uVar30 >= uVar18 && uVar30 - uVar18 != 0 || uVar24 >= uVar17 && uVar24 - uVar17 != 0) {
    lVar29 = lVar29 + uVar17 * 0x10;
    lVar20 = lVar20 + uVar18 * 0x10;
    uVar27 = 0;
    if (uVar30 < uVar18) {
      lVar20 = 8;
    }
    uVar22 = lVar20 + -0x10 + uVar22 * 0x10;
    if (uVar30 <= uVar18) {
      uVar22 = uVar27;
    }
    if (uVar24 < uVar17) {
      lVar29 = 8;
    }
    uVar30 = lVar29 + -0x10 + uVar26 * 0x10;
    if (uVar24 <= uVar17) {
      uVar30 = uVar22;
    }
    *puVar7 = uVar19;
    puVar7 = (ulonglong *)*in_stack_00000038;
    lVar29 = in_stack_00000038[1];
    uVar18 = *puVar7;
    uVar19 = uVar18 - *(ulonglong *)(lVar29 + 0x18);
    if (uVar18 < *(ulonglong *)(lVar29 + 0x18)) {
      uVar19 = uVar27;
    }
    uVar17 = uVar18 - *(ulonglong *)(lVar29 + 0x38);
    if (uVar18 < *(ulonglong *)(lVar29 + 0x38)) {
      uVar17 = uVar27;
    }
    uVar18 = *(ulonglong *)(lVar29 + 0x30);
    uVar24 = uVar18 - uVar17;
    if (uVar18 < uVar17) {
      uVar24 = uVar27;
    }
    uVar26 = *(ulonglong *)(lVar29 + 0x10);
    uVar22 = uVar26 - uVar19;
    if (uVar26 < uVar19) {
      uVar22 = uVar27;
    }
    uVar27 = *(ulonglong *)(lVar29 + 0x40);
    *puVar7 = uVar27 - (uVar22 + uVar24);
    if (uVar26 >= uVar19 && uVar26 - uVar19 != 0 || uVar18 >= uVar17 && uVar18 - uVar17 != 0) {
      *puVar7 = uVar27;
      plVar8 = (longlong *)*in_R9;
      lVar29 = *(longlong *)(uVar30 + 8);
      if (plVar8[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar20 = *plVar8;
        bVar31 = (byte)(extraout_RAX >> 0x39);
        auVar33 = ZEXT216(CONCAT11(bVar31,bVar31));
        auVar33 = pshuflw(auVar33,auVar33,0);
        lVar23 = 0;
        uVar18 = extraout_RAX;
        while( true ) {
          uVar18 = plVar8[1] & uVar18;
          pcVar1 = (char *)(lVar20 + uVar18);
          cVar32 = auVar33[0];
          auVar39[0] = -(*pcVar1 == cVar32);
          cVar34 = auVar33[1];
          auVar39[1] = -(pcVar1[1] == cVar34);
          cVar35 = auVar33[2];
          auVar39[2] = -(pcVar1[2] == cVar35);
          cVar36 = auVar33[3];
          auVar39[3] = -(pcVar1[3] == cVar36);
          auVar39[4] = -(pcVar1[4] == cVar32);
          auVar39[5] = -(pcVar1[5] == cVar34);
          auVar39[6] = -(pcVar1[6] == cVar35);
          auVar39[7] = -(pcVar1[7] == cVar36);
          auVar39[8] = -(pcVar1[8] == cVar32);
          auVar39[9] = -(pcVar1[9] == cVar34);
          auVar39[10] = -(pcVar1[10] == cVar35);
          auVar39[0xb] = -(pcVar1[0xb] == cVar36);
          auVar39[0xc] = -(pcVar1[0xc] == cVar32);
          auVar39[0xd] = -(pcVar1[0xd] == cVar34);
          auVar39[0xe] = -(pcVar1[0xe] == cVar35);
          auVar39[0xf] = -(pcVar1[0xf] == cVar36);
          uVar21 = (ushort)(SUB161(auVar39 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar39[0xf] >> 7) << 0xf;
          uVar25 = (uint)uVar21;
          while (uVar21 != 0) {
            uVar14 = 0;
            for (uVar15 = uVar25; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
              uVar14 = uVar14 + 1;
            }
            lVar28 = (uVar14 + uVar18 & plVar8[1]) * -0x2a8;
            if (*(longlong *)(lVar20 + -0x2a8 + lVar28) == lVar29) {
              if ((*(ulonglong *)(lVar20 + -0x290 + lVar28) < 2) ||
                 (*(float *)(lVar20 + -0x270 + lVar28) <= 0.0)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              lVar20 = lVar20 + lVar28;
              ::utils::mouse_ss_to_cs();
              puVar9 = (undefined8 *)*in_stack_00000030;
              if ((*(byte *)(puVar9 + 2) & 1) == 0) {
                uVar6 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
                puVar10 = (undefined4 *)in_stack_00000030[2];
                *puVar10 = uVar6;
                get_closest_point_u_to_cursor();
                local_f8 = (undefined4)lVar29;
                *(undefined4 *)(puVar9 + 2) = 1;
                *(undefined4 *)((longlong)puVar9 + 0x14) = local_f8;
                uVar2 = *(undefined4 *)(lVar20 + -0x1c);
                *(undefined4 *)(puVar9 + 4) = 1;
                *(undefined4 *)((longlong)puVar9 + 0x24) = uVar2;
                *(undefined4 *)(puVar9 + 5) = 1;
                *(undefined4 *)((longlong)puVar9 + 0x2c) = uVar2;
                uVar11 = *(undefined8 *)(uVar30 + 8);
                *puVar10 = uVar6;
                *puVar9 = 1;
                puVar9[1] = uVar11;
                uVar2 = *(undefined4 *)(in_stack_00000060 + 0x1c);
                puVar10 = *(undefined4 **)(in_stack_00000060 + 0x10);
                *puVar10 = uVar2;
                resources::walls::decorator_archivist::DecoratorArchivist::
                archive_current_decorators_in_storages<ref_<str_>_>();
                *puVar10 = uVar2;
                resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall
                          ();
                *puVar10 = uVar2;
                _ZN102__LT_core__iter__adapters__map__Map_LT_I_C_F_GT__u20_as_u20_core__iter__traits__iterator__Iterator_GT_4fold17h1014fed281d97499E
                          ();
                if ((*(byte *)(puVar9 + 2) & 1) == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
              }
              ::utils::curve::Curve::
              get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                        ();
              uVar18 = (ulonglong)extraout_EAX;
              if (uVar18 < *(ulonglong *)(lVar20 + -600)) {
                uVar19 = uVar18 + 1;
                if (uVar19 < *(ulonglong *)(lVar20 + -600)) {
                  lVar29 = *(longlong *)(lVar20 + -0x260);
                  fVar37 = 1.0 - extraout_XMM0_Da;
                  fVar3 = *(float *)(lVar29 + 8 + uVar18 * 0xc);
                  fVar4 = *(float *)(lVar29 + 8 + uVar19 * 0xc);
                  uVar11 = *(undefined8 *)(lVar29 + uVar18 * 0xc);
                  uVar5 = *(undefined8 *)(lVar29 + uVar19 * 0xc);
                  fVar41 = (float)((ulonglong)uVar5 >> 0x20) * extraout_XMM0_Da +
                           (float)((ulonglong)uVar11 >> 0x20) * fVar37;
                  country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::
                  project_onto_ray();
                  if (*(int *)(puVar9 + 3) == 1) {
                    fVar40 = (local_90 + *(float *)((longlong)puVar9 + 0x1c)) - fVar41;
                    uVar12 = *(undefined8 *)(uVar30 + 8);
                    *(undefined4 *)in_stack_00000048[2] =
                         *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
                    lVar29 = *in_stack_00000048;
                    lVar20 = *(longlong *)(lVar29 + 0x30);
                    uVar13 = *(undefined8 *)(lVar29 + 0x40);
                    if (lVar20 == *(longlong *)(lVar29 + 0x20)) {
                      alloc::raw_vec::RawVec::
                      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>
                                ();
                    }
                    lVar23 = *(longlong *)(lVar29 + 0x28);
                    lVar28 = lVar20 * 0x30;
                    *(undefined4 *)(lVar23 + lVar28) = 1;
                    *(ulonglong *)(lVar23 + 4 + lVar28) =
                         CONCAT44(fVar41,(float)uVar5 * extraout_XMM0_Da + (float)uVar11 * fVar37);
                    *(float *)(lVar23 + 0xc + lVar28) = fVar4 * extraout_XMM0_Da + fVar37 * fVar3;
                    *(undefined4 *)(lVar23 + 0x10 + lVar28) = 1;
                    *(float *)(lVar23 + 0x14 + lVar28) = fVar40;
                    *(undefined8 *)(lVar23 + 0x18 + lVar28) = uVar12;
                    *(undefined1 *)(lVar23 + 0x20 + lVar28) = 0;
                    *(undefined8 *)(lVar23 + 0x28 + lVar28) = uVar13;
                    *(longlong *)(lVar29 + 0x30) = lVar20 + 1;
                    *(longlong *)(lVar29 + 0x40) = *(longlong *)(lVar29 + 0x40) + 1;
                    *(undefined4 *)in_stack_00000030[2] =
                         *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
                    puVar16 = puVar9 + 5;
                    lVar29 = 0x2c;
                  }
                  else {
                    puVar16 = puVar9 + 3;
                    fVar40 = (fVar41 + *(float *)(lVar20 + -0x1c)) - local_90;
                    *(undefined4 *)in_stack_00000030[2] =
                         *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
                    lVar29 = 0x1c;
                  }
                  *(undefined4 *)puVar16 = 1;
                  *(float *)((longlong)puVar9 + lVar29) = fVar40;
                  return;
                }
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            uVar21 = (ushort)(uVar25 - 1) & (ushort)uVar25;
            uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar21);
          }
          auVar38[0] = -(*pcVar1 == -1);
          auVar38[1] = -(pcVar1[1] == -1);
          auVar38[2] = -(pcVar1[2] == -1);
          auVar38[3] = -(pcVar1[3] == -1);
          auVar38[4] = -(pcVar1[4] == -1);
          auVar38[5] = -(pcVar1[5] == -1);
          auVar38[6] = -(pcVar1[6] == -1);
          auVar38[7] = -(pcVar1[7] == -1);
          auVar38[8] = -(pcVar1[8] == -1);
          auVar38[9] = -(pcVar1[9] == -1);
          auVar38[10] = -(pcVar1[10] == -1);
          auVar38[0xb] = -(pcVar1[0xb] == -1);
          auVar38[0xc] = -(pcVar1[0xc] == -1);
          auVar38[0xd] = -(pcVar1[0xd] == -1);
          auVar38[0xe] = -(pcVar1[0xe] == -1);
          auVar38[0xf] = -(pcVar1[0xf] == -1);
          if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar38 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar38 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar38 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar38 >> 0x2f,0) & 1) != 0) || (SUB161(auVar38 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') break;
          uVar18 = uVar18 + lVar23 + 0x10;
          lVar23 = lVar23 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  return;
}



// CALLEES: [{'name': '_ZN102_$LT$core..iter..adapters..map..Map$LT$I$C$F$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4fold17h1014fed281d97499E', 'address': '141293280'}, {'name': 'archive_current_decorators_in_storages<ref$<str$>_>', 'address': '1413e77f0'}, {'name': 'decorator_cull_allow_wall', 'address': '140b2bc00'}, {'name': 'get_closest_point_u_to_cursor', 'address': '140ac7400'}, {'name': 'get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>', 'address': '141467ce0'}, {'name': 'grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>', 'address': '1405629f0'}, {'name': 'hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId>_>', 'address': '1405b0d70'}, {'name': 'mouse_ss_to_cs', 'address': '140c906d0'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'panic_bounds_check', 'address': '1428d9518'}, {'name': 'project_onto_ray', 'address': '140ac84a0'}, {'name': 'unwrap_failed', 'address': '1428d9310'}]