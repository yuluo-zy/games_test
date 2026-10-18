// 0x140ac79e0 history_record_adjust_wall_height size=1348 sig=void __cdecl history_record_adjust_wall_height(void)
void __cdecl
country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height_mode::
history_record_adjust_wall_height(void)

{
  char *pcVar1;
  ulonglong *puVar2;
  longlong lVar3;
  ulonglong uVar4;
  byte *pbVar5;
  longlong *plVar6;
  longlong lVar7;
  undefined8 uVar8;
  uint uVar9;
  uint uVar10;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  uint uVar15;
  longlong in_R9;
  ulonglong uVar16;
  longlong lVar17;
  ushort uVar18;
  ulonglong uVar19;
  longlong lVar20;
  byte bVar21;
  char cVar22;
  char cVar27;
  float fVar23;
  char cVar25;
  char cVar26;
  undefined1 auVar24 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  float fVar34;
  float fVar35;
  longlong *in_stack_00000028;
  longlong *in_stack_00000038;
  
  puVar2 = (ulonglong *)*in_RDX;
  lVar3 = in_RDX[1];
  uVar14 = *puVar2;
  uVar13 = 0;
  uVar16 = uVar14 - *(ulonglong *)(lVar3 + 0x18);
  if (uVar14 < *(ulonglong *)(lVar3 + 0x18)) {
    uVar16 = uVar13;
  }
  uVar11 = uVar14 - *(ulonglong *)(lVar3 + 0x38);
  if (uVar14 < *(ulonglong *)(lVar3 + 0x38)) {
    uVar11 = uVar13;
  }
  uVar14 = *(ulonglong *)(lVar3 + 0x30);
  uVar19 = uVar14 - uVar11;
  if (uVar14 < uVar11) {
    uVar19 = uVar13;
  }
  uVar4 = *(ulonglong *)(lVar3 + 0x10);
  uVar12 = uVar4 - uVar16;
  if (uVar4 < uVar16) {
    uVar12 = uVar13;
  }
  uVar13 = *(ulonglong *)(lVar3 + 0x40);
  *puVar2 = uVar13 - (uVar12 + uVar19);
  if (uVar4 >= uVar16 && uVar4 - uVar16 != 0 || uVar14 >= uVar11 && uVar14 - uVar11 != 0) {
    *puVar2 = uVar13;
    pbVar5 = (byte *)*in_RCX;
    if (((*(int *)(pbVar5 + 0x20) == 1) && ((pbVar5[0x28] & 1) != 0)) && ((*pbVar5 & 1) != 0)) {
      fVar35 = *(float *)(pbVar5 + 0x24);
      fVar34 = *(float *)(pbVar5 + 0x2c);
      lVar3 = *(longlong *)(pbVar5 + 8);
      plVar6 = (longlong *)*in_stack_00000028;
      if (plVar6[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar7 = *plVar6;
        bVar21 = (byte)(extraout_RAX >> 0x39);
        auVar24 = ZEXT216(CONCAT11(bVar21,bVar21));
        auVar24 = pshuflw(auVar24,auVar24,0);
        lVar17 = 0;
        uVar14 = extraout_RAX;
        while( true ) {
          uVar14 = plVar6[1] & uVar14;
          pcVar1 = (char *)(lVar7 + uVar14);
          cVar22 = auVar24[0];
          auVar31[0] = -(*pcVar1 == cVar22);
          cVar25 = auVar24[1];
          auVar31[1] = -(pcVar1[1] == cVar25);
          cVar26 = auVar24[2];
          auVar31[2] = -(pcVar1[2] == cVar26);
          cVar27 = auVar24[3];
          auVar31[3] = -(pcVar1[3] == cVar27);
          auVar31[4] = -(pcVar1[4] == cVar22);
          auVar31[5] = -(pcVar1[5] == cVar25);
          auVar31[6] = -(pcVar1[6] == cVar26);
          auVar31[7] = -(pcVar1[7] == cVar27);
          auVar31[8] = -(pcVar1[8] == cVar22);
          auVar31[9] = -(pcVar1[9] == cVar25);
          auVar31[10] = -(pcVar1[10] == cVar26);
          auVar31[0xb] = -(pcVar1[0xb] == cVar27);
          auVar31[0xc] = -(pcVar1[0xc] == cVar22);
          auVar31[0xd] = -(pcVar1[0xd] == cVar25);
          auVar31[0xe] = -(pcVar1[0xe] == cVar26);
          auVar31[0xf] = -(pcVar1[0xf] == cVar27);
          uVar18 = (ushort)(SUB161(auVar31 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar31[0xf] >> 7) << 0xf;
          uVar15 = (uint)uVar18;
          while (uVar18 != 0) {
            uVar9 = 0;
            for (uVar10 = uVar15; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
              uVar9 = uVar9 + 1;
            }
            lVar20 = (uVar9 + uVar14 & plVar6[1]) * -0x2a8;
            if (*(longlong *)(lVar7 + -0x2a8 + lVar20) == lVar3) {
              if ((*(ulonglong *)(lVar7 + -0x290 + lVar20) < 2) ||
                 (*(float *)(lVar7 + -0x270 + lVar20) <= 0.0)) goto LAB_140ac7ef4;
              cVar22 = *(char *)(lVar7 + lVar20 + -10);
              resources::walls::public_walls::PublicWalls::roof_component();
              if (extraout_RAX_00 == 0) {
                if (cVar22 == '\0') {
                  fVar23 = 0.5;
                }
                else {
                  fVar23 = 0.3;
                }
              }
              else {
                fVar23 = *(float *)(&__real_3fcccccd3f333333 +
                                   (ulonglong)(*(float *)(extraout_RAX_00 + 0x30) <= 0.1) * 4);
              }
              if (fVar23 <= fVar35) {
                fVar23 = fVar35;
              }
              if (cVar22 == '\0') {
                fVar35 = 14.0;
              }
              else {
                fVar35 = 2.5;
              }
              if (fVar23 <= fVar35) {
                fVar35 = fVar23;
              }
              goto LAB_140ac7bf2;
            }
            uVar18 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar18);
          }
          auVar28[0] = -(*pcVar1 == -1);
          auVar28[1] = -(pcVar1[1] == -1);
          auVar28[2] = -(pcVar1[2] == -1);
          auVar28[3] = -(pcVar1[3] == -1);
          auVar28[4] = -(pcVar1[4] == -1);
          auVar28[5] = -(pcVar1[5] == -1);
          auVar28[6] = -(pcVar1[6] == -1);
          auVar28[7] = -(pcVar1[7] == -1);
          auVar28[8] = -(pcVar1[8] == -1);
          auVar28[9] = -(pcVar1[9] == -1);
          auVar28[10] = -(pcVar1[10] == -1);
          auVar28[0xb] = -(pcVar1[0xb] == -1);
          auVar28[0xc] = -(pcVar1[0xc] == -1);
          auVar28[0xd] = -(pcVar1[0xd] == -1);
          auVar28[0xe] = -(pcVar1[0xe] == -1);
          auVar28[0xf] = -(pcVar1[0xf] == -1);
          if ((((((((((((((((SUB161(auVar28 >> 7,0) & 1) != 0 || (SUB161(auVar28 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar28 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar28 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar28 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar28 >> 0x2f,0) & 1) != 0) || (SUB161(auVar28 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar28 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar28 >> 0x47,0) & 1) != 0) || (SUB161(auVar28 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar28 >> 0x57,0) & 1) != 0) || (SUB161(auVar28 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar28 >> 0x67,0) & 1) != 0) || (SUB161(auVar28 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar28 >> 0x77,0) & 1) != 0) || auVar28[0xf] < '\0') break;
          uVar14 = uVar14 + lVar17 + 0x10;
          lVar17 = lVar17 + 0x10;
        }
      }
LAB_140ac7bf2:
      if (plVar6[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar7 = *plVar6;
        bVar21 = (byte)(extraout_RAX_01 >> 0x39);
        auVar24 = ZEXT216(CONCAT11(bVar21,bVar21));
        auVar24 = pshuflw(auVar24,auVar24,0);
        lVar17 = 0;
        uVar14 = extraout_RAX_01;
        do {
          uVar14 = plVar6[1] & uVar14;
          pcVar1 = (char *)(lVar7 + uVar14);
          cVar22 = auVar24[0];
          auVar32[0] = -(*pcVar1 == cVar22);
          cVar25 = auVar24[1];
          auVar32[1] = -(pcVar1[1] == cVar25);
          cVar26 = auVar24[2];
          auVar32[2] = -(pcVar1[2] == cVar26);
          cVar27 = auVar24[3];
          auVar32[3] = -(pcVar1[3] == cVar27);
          auVar32[4] = -(pcVar1[4] == cVar22);
          auVar32[5] = -(pcVar1[5] == cVar25);
          auVar32[6] = -(pcVar1[6] == cVar26);
          auVar32[7] = -(pcVar1[7] == cVar27);
          auVar32[8] = -(pcVar1[8] == cVar22);
          auVar32[9] = -(pcVar1[9] == cVar25);
          auVar32[10] = -(pcVar1[10] == cVar26);
          auVar32[0xb] = -(pcVar1[0xb] == cVar27);
          auVar32[0xc] = -(pcVar1[0xc] == cVar22);
          auVar32[0xd] = -(pcVar1[0xd] == cVar25);
          auVar32[0xe] = -(pcVar1[0xe] == cVar26);
          auVar32[0xf] = -(pcVar1[0xf] == cVar27);
          uVar18 = (ushort)(SUB161(auVar32 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar32[0xf] >> 7) << 0xf;
          uVar15 = (uint)uVar18;
          while (uVar18 != 0) {
            uVar9 = 0;
            for (uVar10 = uVar15; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
              uVar9 = uVar9 + 1;
            }
            lVar20 = (uVar9 + uVar14 & plVar6[1]) * -0x2a8;
            if (*(longlong *)(lVar7 + -0x2a8 + lVar20) == lVar3) {
              if ((*(ulonglong *)(lVar7 + -0x290 + lVar20) < 2) ||
                 (*(float *)(lVar7 + -0x270 + lVar20) <= 0.0)) {
LAB_140ac7ef4:
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              cVar22 = *(char *)(lVar7 + lVar20 + -10);
              resources::walls::public_walls::PublicWalls::roof_component();
              if (extraout_RAX_02 == 0) {
                if (cVar22 == '\0') {
                  fVar23 = 0.5;
                }
                else {
                  fVar23 = 0.3;
                }
              }
              else {
                fVar23 = *(float *)(&__real_3fcccccd3f333333 +
                                   (ulonglong)(*(float *)(extraout_RAX_02 + 0x30) <= 0.1) * 4);
              }
              if (fVar23 <= fVar34) {
                fVar23 = fVar34;
              }
              if (cVar22 == '\0') {
                fVar34 = 14.0;
              }
              else {
                fVar34 = 2.5;
              }
              if (fVar23 <= fVar34) {
                fVar34 = fVar23;
              }
              goto LAB_140ac7d44;
            }
            uVar18 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar18);
          }
          auVar29[0] = -(*pcVar1 == -1);
          auVar29[1] = -(pcVar1[1] == -1);
          auVar29[2] = -(pcVar1[2] == -1);
          auVar29[3] = -(pcVar1[3] == -1);
          auVar29[4] = -(pcVar1[4] == -1);
          auVar29[5] = -(pcVar1[5] == -1);
          auVar29[6] = -(pcVar1[6] == -1);
          auVar29[7] = -(pcVar1[7] == -1);
          auVar29[8] = -(pcVar1[8] == -1);
          auVar29[9] = -(pcVar1[9] == -1);
          auVar29[10] = -(pcVar1[10] == -1);
          auVar29[0xb] = -(pcVar1[0xb] == -1);
          auVar29[0xc] = -(pcVar1[0xc] == -1);
          auVar29[0xd] = -(pcVar1[0xd] == -1);
          auVar29[0xe] = -(pcVar1[0xe] == -1);
          auVar29[0xf] = -(pcVar1[0xf] == -1);
          if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar29 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar29 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar29 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar29 >> 0x2f,0) & 1) != 0) || (SUB161(auVar29 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar29 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x47,0) & 1) != 0) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') break;
          uVar14 = uVar14 + lVar17 + 0x10;
          lVar17 = lVar17 + 0x10;
        } while( true );
      }
LAB_140ac7d44:
      if ((fVar35 != fVar34) || (NAN(fVar35) || NAN(fVar34))) {
        **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
        resources::history::EditType::new_height_changed();
        resources::history::History::add();
        if (plVar6[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar7 = *plVar6;
          bVar21 = (byte)(extraout_RAX_03 >> 0x39);
          auVar24 = ZEXT216(CONCAT11(bVar21,bVar21));
          auVar24 = pshuflw(auVar24,auVar24,0);
          lVar17 = 0;
          uVar14 = extraout_RAX_03;
          while( true ) {
            uVar14 = plVar6[1] & uVar14;
            pcVar1 = (char *)(lVar7 + uVar14);
            cVar22 = auVar24[0];
            auVar33[0] = -(*pcVar1 == cVar22);
            cVar25 = auVar24[1];
            auVar33[1] = -(pcVar1[1] == cVar25);
            cVar26 = auVar24[2];
            auVar33[2] = -(pcVar1[2] == cVar26);
            cVar27 = auVar24[3];
            auVar33[3] = -(pcVar1[3] == cVar27);
            auVar33[4] = -(pcVar1[4] == cVar22);
            auVar33[5] = -(pcVar1[5] == cVar25);
            auVar33[6] = -(pcVar1[6] == cVar26);
            auVar33[7] = -(pcVar1[7] == cVar27);
            auVar33[8] = -(pcVar1[8] == cVar22);
            auVar33[9] = -(pcVar1[9] == cVar25);
            auVar33[10] = -(pcVar1[10] == cVar26);
            auVar33[0xb] = -(pcVar1[0xb] == cVar27);
            auVar33[0xc] = -(pcVar1[0xc] == cVar22);
            auVar33[0xd] = -(pcVar1[0xd] == cVar25);
            auVar33[0xe] = -(pcVar1[0xe] == cVar26);
            auVar33[0xf] = -(pcVar1[0xf] == cVar27);
            uVar18 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar33[0xf] >> 7) << 0xf;
            uVar15 = (uint)uVar18;
            while (uVar18 != 0) {
              uVar9 = 0;
              for (uVar10 = uVar15; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
                uVar9 = uVar9 + 1;
              }
              lVar20 = (uVar9 + uVar14 & plVar6[1]) * -0x2a8;
              if (*(longlong *)(lVar7 + -0x2a8 + lVar20) == lVar3) {
                if ((*(ulonglong *)(lVar7 + -0x290 + lVar20) < 2) ||
                   (*(float *)(lVar7 + -0x270 + lVar20) <= 0.0)) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic();
                }
                if (*(uint *)(lVar7 + lVar20 + -0x48) < 2) {
                  *(undefined4 *)in_stack_00000038[2] =
                       *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
                  lVar3 = *in_stack_00000038;
                  lVar7 = *(longlong *)(lVar3 + 0x30);
                  uVar8 = *(undefined8 *)(lVar3 + 0x40);
                  if (lVar7 == *(longlong *)(lVar3 + 0x20)) {
                    alloc::raw_vec::RawVec::
                    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                              ();
                  }
                  *(undefined8 *)(*(longlong *)(lVar3 + 0x28) + lVar7 * 8) = uVar8;
                  *(longlong *)(lVar3 + 0x30) = lVar7 + 1;
                  *(longlong *)(lVar3 + 0x40) = *(longlong *)(lVar3 + 0x40) + 1;
                }
                goto LAB_140ac7ea6;
              }
              uVar18 = (ushort)(uVar15 - 1) & (ushort)uVar15;
              uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar18);
            }
            auVar30[0] = -(*pcVar1 == -1);
            auVar30[1] = -(pcVar1[1] == -1);
            auVar30[2] = -(pcVar1[2] == -1);
            auVar30[3] = -(pcVar1[3] == -1);
            auVar30[4] = -(pcVar1[4] == -1);
            auVar30[5] = -(pcVar1[5] == -1);
            auVar30[6] = -(pcVar1[6] == -1);
            auVar30[7] = -(pcVar1[7] == -1);
            auVar30[8] = -(pcVar1[8] == -1);
            auVar30[9] = -(pcVar1[9] == -1);
            auVar30[10] = -(pcVar1[10] == -1);
            auVar30[0xb] = -(pcVar1[0xb] == -1);
            auVar30[0xc] = -(pcVar1[0xc] == -1);
            auVar30[0xd] = -(pcVar1[0xd] == -1);
            auVar30[0xe] = -(pcVar1[0xe] == -1);
            auVar30[0xf] = -(pcVar1[0xf] == -1);
            if ((((((((((((((((SUB161(auVar30 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar30 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar30 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar30 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar30 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar30 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar30 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar30 >> 0x3f,0) & 1) != 0) || (SUB161(auVar30 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar30 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar30 >> 0x57,0) & 1) != 0) || (SUB161(auVar30 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar30 >> 0x67,0) & 1) != 0) || (SUB161(auVar30 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar30 >> 0x77,0) & 1) != 0) || auVar30[0xf] < '\0') break;
            uVar14 = uVar14 + lVar17 + 0x10;
            lVar17 = lVar17 + 0x10;
          }
        }
      }
    }
LAB_140ac7ea6:
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    pbVar5[0] = 0;
    pbVar5[1] = 0;
    pbVar5[2] = 0;
    pbVar5[3] = 0;
    pbVar5[4] = 0;
    pbVar5[5] = 0;
    pbVar5[6] = 0;
    pbVar5[7] = 0;
    pbVar5[0x10] = 0;
    pbVar5[0x11] = 0;
    pbVar5[0x12] = 0;
    pbVar5[0x13] = 0;
    pbVar5[0x18] = 0;
    pbVar5[0x19] = 0;
    pbVar5[0x1a] = 0;
    pbVar5[0x1b] = 0;
    pbVar5[0x20] = 0;
    pbVar5[0x21] = 0;
    pbVar5[0x22] = 0;
    pbVar5[0x23] = 0;
    pbVar5[0x28] = 0;
    pbVar5[0x29] = 0;
    pbVar5[0x2a] = 0;
    pbVar5[0x2b] = 0;
  }
  return;
}
