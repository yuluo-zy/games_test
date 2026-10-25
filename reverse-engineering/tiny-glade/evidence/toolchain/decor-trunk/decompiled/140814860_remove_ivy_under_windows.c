// ADDR 0x140814860
// SIG void __cdecl remove_ivy_under_windows(void)
// ORIG system::remove_ivy_under_windows


void __cdecl system_decorator::remove_ivy_under_windows::remove_ivy_under_windows(void)

{
  char *pcVar1;
  float fVar2;
  undefined8 uVar3;
  int iVar4;
  ulonglong *puVar5;
  ulonglong uVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  longlong *plVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  undefined8 *in_RCX;
  longlong *plVar14;
  longlong *plVar15;
  ulonglong uVar16;
  undefined8 *in_R8;
  ulonglong uVar17;
  ulonglong uVar18;
  uint uVar19;
  longlong *plVar20;
  longlong lVar21;
  longlong *plVar22;
  ushort uVar23;
  longlong *plVar24;
  char cVar25;
  byte bVar26;
  char cVar28;
  char cVar29;
  char cVar30;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined1 auVar27 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  char cVar35;
  undefined1 unaff_XMM8_Bc;
  char cVar36;
  undefined1 unaff_XMM8_Bd;
  char cVar37;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  char cVar38;
  undefined1 unaff_XMM8_Bg;
  char cVar39;
  undefined1 unaff_XMM8_Bh;
  char cVar40;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  char cVar41;
  undefined1 unaff_XMM8_Bk;
  char cVar42;
  undefined1 unaff_XMM8_Bl;
  char cVar43;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  char cVar44;
  undefined1 unaff_XMM8_Bo;
  char cVar45;
  undefined1 unaff_XMM8_Bp;
  char cVar46;
  longlong *in_stack_00000028;
  undefined1 local_12a [2];
  undefined8 local_128;
  float local_120;
  int local_11c;
  undefined **local_118;
  longlong local_110;
  undefined1 **local_108;
  undefined8 local_100;
  undefined8 local_f8;
  longlong local_e0;
  undefined8 local_c8;
  undefined1 *local_c0;
  code *local_b8;
  longlong *local_b0;
  longlong local_a8;
  longlong local_a0;
  
  country_core::resources::walls::decorator_subtype::impl$8::from();
  puVar5 = (ulonglong *)*in_R8;
  lVar21 = in_R8[1];
  uVar6 = *puVar5;
  uVar16 = 0;
  uVar18 = uVar6 - *(ulonglong *)(lVar21 + 0x18);
  if (uVar6 < *(ulonglong *)(lVar21 + 0x18)) {
    uVar18 = uVar16;
  }
  uVar17 = uVar6 - *(ulonglong *)(lVar21 + 0x38);
  if (uVar6 < *(ulonglong *)(lVar21 + 0x38)) {
    uVar17 = uVar16;
  }
  plVar20 = (longlong *)(uVar18 * 0x90 + *(longlong *)(lVar21 + 8));
  uVar6 = *(ulonglong *)(lVar21 + 0x10) - uVar18;
  if (*(ulonglong *)(lVar21 + 0x10) < uVar18) {
    plVar20 = (longlong *)&DAT_00000008;
    uVar6 = uVar16;
  }
  plVar14 = (longlong *)(uVar17 * 0x90 + *(longlong *)(lVar21 + 0x28));
  uVar18 = *(ulonglong *)(lVar21 + 0x30) - uVar17;
  if (*(ulonglong *)(lVar21 + 0x30) < uVar17) {
    plVar14 = (longlong *)&DAT_00000008;
    uVar18 = uVar16;
  }
  *puVar5 = (*(longlong *)(lVar21 + 0x40) - uVar18) - uVar6;
  local_b0 = plVar14 + uVar18 * 0x12;
  local_c8 = *in_RCX;
  local_e0 = *in_stack_00000028;
  iVar9 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                 CONCAT13(unaff_XMM8_Bd,
                          CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  cVar35 = (char)((uint)iVar9 >> 8);
  cVar36 = (char)((uint)iVar9 >> 0x10);
  cVar37 = (char)((uint)iVar9 >> 0x18);
  iVar10 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                            CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                  CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  cVar38 = (char)((uint)iVar10 >> 8);
  cVar39 = (char)((uint)iVar10 >> 0x10);
  cVar40 = (char)((uint)iVar10 >> 0x18);
  iVar11 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  cVar41 = (char)((uint)iVar11 >> 8);
  cVar42 = (char)((uint)iVar11 >> 0x10);
  cVar43 = (char)((uint)iVar11 >> 0x18);
  iVar12 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
  cVar44 = (char)((uint)iVar12 >> 8);
  cVar45 = (char)((uint)iVar12 >> 0x10);
  cVar46 = (char)((uint)iVar12 >> 0x18);
  plVar22 = plVar20;
LAB_140814ac0:
  do {
    do {
      if ((plVar22 == (longlong *)0x0) || (plVar22 == plVar20 + uVar6 * 0x12)) {
        if ((plVar14 == (longlong *)0x0) || (plVar14 == local_b0)) {
          return;
        }
        plVar15 = (longlong *)0x0;
        plVar13 = plVar14 + 0x12;
        plVar24 = plVar14;
      }
      else {
        plVar15 = plVar22 + 0x12;
        plVar13 = plVar14;
        plVar24 = plVar22;
      }
      plVar14 = plVar13;
      *puVar5 = *puVar5 + 1;
      lVar21 = *plVar24 + -2;
      if (1 < *plVar24 - 3U) {
        lVar21 = 0;
      }
      plVar22 = plVar15;
    } while (lVar21 == 2);
    if (lVar21 == 1) {
      iVar4 = (int)plVar24[8];
      uVar3 = *(undefined8 *)((longlong)plVar24 + 0x2c);
      fVar2 = *(float *)((longlong)plVar24 + 0x34);
      local_a8 = plVar24[1];
      local_a0 = plVar24[2];
      local_11c = iVar4;
      if ((((char)plVar24[0x10] == '\0') && (2 < (byte)(*(char *)((longlong)plVar24 + 0x3a) - 2U)))
         && ((country_core::resources::walls::decorator_storage::DecoratorStorage::get(),
             extraout_RAX_00 != 0 && (*(longlong *)(extraout_RAX_00 + 0x48) != 0)))) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        bVar26 = (byte)(extraout_RAX_01 >> 0x39);
        auVar27 = ZEXT216(CONCAT11(bVar26,bVar26));
        auVar27 = pshuflw(auVar27,auVar27,0);
        lVar21 = 0;
        uVar18 = extraout_RAX_01;
        while( true ) {
          uVar18 = *(ulonglong *)(extraout_RAX_00 + 0x38) & uVar18;
          pcVar1 = (char *)(*(longlong *)(extraout_RAX_00 + 0x30) + uVar18);
          cVar25 = auVar27[0];
          auVar33[0] = -(*pcVar1 == cVar25);
          cVar28 = auVar27[1];
          auVar33[1] = -(pcVar1[1] == cVar28);
          cVar29 = auVar27[2];
          auVar33[2] = -(pcVar1[2] == cVar29);
          cVar30 = auVar27[3];
          auVar33[3] = -(pcVar1[3] == cVar30);
          auVar33[4] = -(pcVar1[4] == cVar25);
          auVar33[5] = -(pcVar1[5] == cVar28);
          auVar33[6] = -(pcVar1[6] == cVar29);
          auVar33[7] = -(pcVar1[7] == cVar30);
          auVar33[8] = -(pcVar1[8] == cVar25);
          auVar33[9] = -(pcVar1[9] == cVar28);
          auVar33[10] = -(pcVar1[10] == cVar29);
          auVar33[0xb] = -(pcVar1[0xb] == cVar30);
          auVar33[0xc] = -(pcVar1[0xc] == cVar25);
          auVar33[0xd] = -(pcVar1[0xd] == cVar28);
          auVar33[0xe] = -(pcVar1[0xe] == cVar29);
          auVar33[0xf] = -(pcVar1[0xf] == cVar30);
          uVar23 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
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
          uVar19 = (uint)uVar23;
          if (uVar23 != 0) break;
LAB_1408149e0:
          auVar31[0] = -(*pcVar1 == (char)iVar9);
          auVar31[1] = -(pcVar1[1] == cVar35);
          auVar31[2] = -(pcVar1[2] == cVar36);
          auVar31[3] = -(pcVar1[3] == cVar37);
          auVar31[4] = -(pcVar1[4] == (char)iVar10);
          auVar31[5] = -(pcVar1[5] == cVar38);
          auVar31[6] = -(pcVar1[6] == cVar39);
          auVar31[7] = -(pcVar1[7] == cVar40);
          auVar31[8] = -(pcVar1[8] == (char)iVar11);
          auVar31[9] = -(pcVar1[9] == cVar41);
          auVar31[10] = -(pcVar1[10] == cVar42);
          auVar31[0xb] = -(pcVar1[0xb] == cVar43);
          auVar31[0xc] = -(pcVar1[0xc] == (char)iVar12);
          auVar31[0xd] = -(pcVar1[0xd] == cVar44);
          auVar31[0xe] = -(pcVar1[0xe] == cVar45);
          auVar31[0xf] = -(pcVar1[0xf] == cVar46);
          if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar31 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar31 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar31 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar31 >> 0x2f,0) & 1) != 0) || (SUB161(auVar31 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar31 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar31 >> 0x47,0) & 1) != 0) || (SUB161(auVar31 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar31 >> 0x57,0) & 1) != 0) || (SUB161(auVar31 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar31 >> 0x67,0) & 1) != 0) || (SUB161(auVar31 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar31 >> 0x77,0) & 1) != 0) || auVar31[0xf] < '\0') goto LAB_140814ac0;
          uVar18 = uVar18 + lVar21 + 0x10;
          lVar21 = lVar21 + 0x10;
        }
        while( true ) {
          uVar7 = 0;
          for (uVar8 = uVar19; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
            uVar7 = uVar7 + 1;
          }
          if (iVar4 == *(int *)(*(longlong *)(extraout_RAX_00 + 0x30) + -0x50 +
                               (uVar7 + uVar18 & *(ulonglong *)(extraout_RAX_00 + 0x38)) * -0x50))
          break;
          uVar23 = (ushort)(uVar19 - 1) & (ushort)uVar19;
          uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar23);
          if (uVar23 == 0) goto LAB_1408149e0;
        }
        local_118 = *(undefined ***)(local_e0 + 8);
        local_110 = *(longlong *)(local_e0 + 0x10) * 2;
        local_108 = *(undefined1 ***)(local_e0 + 0x50);
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                  ();
        local_120 = fVar2 - extraout_XMM0_Da * 0.0;
        local_128 = CONCAT44((float)((ulonglong)uVar3 >> 0x20) - extraout_XMM0_Da,
                             (float)uVar3 - extraout_XMM0_Da * 0.0);
        country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype();
        country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
        std::collections::hash::map::HashMap::
        get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                  ();
        if (extraout_RAX == 0) {
LAB_140814e75:
          local_c0 = local_12a;
          local_b8 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
          local_118 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
          local_110 = 1;
          local_f8 = 0;
          local_108 = &local_c0;
          local_100 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
LAB_140814df3:
        country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall();
        country_core::systems::ivy::ivy_pruner::remove_ivy_at_location();
      }
      goto LAB_140814ac0;
    }
    if (((plVar24[2] != 3) &&
        (country_core::resources::walls::decorator_storage::DecoratorStorage::get(),
        extraout_RAX_02 != 0)) && (*(longlong *)(extraout_RAX_02 + 0x48) != 0)) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      bVar26 = (byte)(extraout_RAX_03 >> 0x39);
      auVar27 = ZEXT216(CONCAT11(bVar26,bVar26));
      auVar27 = pshuflw(auVar27,auVar27,0);
      lVar21 = 0;
      uVar18 = extraout_RAX_03;
      while( true ) {
        uVar18 = *(ulonglong *)(extraout_RAX_02 + 0x38) & uVar18;
        pcVar1 = (char *)(*(longlong *)(extraout_RAX_02 + 0x30) + uVar18);
        cVar25 = auVar27[0];
        auVar34[0] = -(*pcVar1 == cVar25);
        cVar28 = auVar27[1];
        auVar34[1] = -(pcVar1[1] == cVar28);
        cVar29 = auVar27[2];
        auVar34[2] = -(pcVar1[2] == cVar29);
        cVar30 = auVar27[3];
        auVar34[3] = -(pcVar1[3] == cVar30);
        auVar34[4] = -(pcVar1[4] == cVar25);
        auVar34[5] = -(pcVar1[5] == cVar28);
        auVar34[6] = -(pcVar1[6] == cVar29);
        auVar34[7] = -(pcVar1[7] == cVar30);
        auVar34[8] = -(pcVar1[8] == cVar25);
        auVar34[9] = -(pcVar1[9] == cVar28);
        auVar34[10] = -(pcVar1[10] == cVar29);
        auVar34[0xb] = -(pcVar1[0xb] == cVar30);
        auVar34[0xc] = -(pcVar1[0xc] == cVar25);
        auVar34[0xd] = -(pcVar1[0xd] == cVar28);
        auVar34[0xe] = -(pcVar1[0xe] == cVar29);
        auVar34[0xf] = -(pcVar1[0xf] == cVar30);
        uVar23 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf
        ;
        uVar19 = (uint)uVar23;
        if (uVar23 != 0) {
          do {
            uVar7 = 0;
            for (uVar8 = uVar19; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar7 = uVar7 + 1;
            }
            if ((int)plVar24[0xd] ==
                *(int *)(*(longlong *)(extraout_RAX_02 + 0x30) + -0x50 +
                        (uVar7 + uVar18 & *(ulonglong *)(extraout_RAX_02 + 0x38)) * -0x50)) {
              if (1 < *(byte *)((longlong)plVar24 + 0x42)) goto LAB_140814ac0;
              fVar2 = *(float *)((longlong)plVar24 + 0x3c);
              local_118 = *(undefined ***)(local_e0 + 8);
              local_110 = *(longlong *)(local_e0 + 0x10) * 2;
              local_108 = *(undefined1 ***)(local_e0 + 0x50);
              uVar3 = *(undefined8 *)((longlong)plVar24 + 0x34);
              ::utils::float_grid::impl$0::
              bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
                        ();
              local_120 = fVar2 - extraout_XMM0_Da_00 * 0.0;
              local_128 = CONCAT44((float)((ulonglong)uVar3 >> 0x20) - extraout_XMM0_Da_00,
                                   (float)uVar3 - extraout_XMM0_Da_00 * 0.0);
              country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype();
              country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
              std::collections::hash::map::HashMap::
              get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                        ();
              if (extraout_RAX_04 != 0) goto LAB_140814df3;
              goto LAB_140814e75;
            }
            uVar23 = (ushort)(uVar19 - 1) & (ushort)uVar19;
            uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar23);
          } while (uVar23 != 0);
        }
        auVar32[0] = -(*pcVar1 == (char)iVar9);
        auVar32[1] = -(pcVar1[1] == cVar35);
        auVar32[2] = -(pcVar1[2] == cVar36);
        auVar32[3] = -(pcVar1[3] == cVar37);
        auVar32[4] = -(pcVar1[4] == (char)iVar10);
        auVar32[5] = -(pcVar1[5] == cVar38);
        auVar32[6] = -(pcVar1[6] == cVar39);
        auVar32[7] = -(pcVar1[7] == cVar40);
        auVar32[8] = -(pcVar1[8] == (char)iVar11);
        auVar32[9] = -(pcVar1[9] == cVar41);
        auVar32[10] = -(pcVar1[10] == cVar42);
        auVar32[0xb] = -(pcVar1[0xb] == cVar43);
        auVar32[0xc] = -(pcVar1[0xc] == (char)iVar12);
        auVar32[0xd] = -(pcVar1[0xd] == cVar44);
        auVar32[0xe] = -(pcVar1[0xe] == cVar45);
        auVar32[0xf] = -(pcVar1[0xf] == cVar46);
        if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 || (SUB161(auVar32 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar32 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar32 >> 0x1f,0) & 1) != 0) || (SUB161(auVar32 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar32 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar32 >> 0x37,0) & 1) != 0) || (SUB161(auVar32 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar32 >> 0x47,0) & 1) != 0) || (SUB161(auVar32 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar32 >> 0x57,0) & 1) != 0) || (SUB161(auVar32 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar32 >> 0x67,0) & 1) != 0) || (SUB161(auVar32 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar32 >> 0x77,0) & 1) != 0) || auVar32[0xf] < '\0') break;
        uVar18 = uVar18 + lVar21 + 0x10;
        lVar21 = lVar21 + 0x10;
      }
    }
  } while( true );
}


