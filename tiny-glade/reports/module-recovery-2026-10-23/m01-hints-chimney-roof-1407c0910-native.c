
void __cdecl system_decorator::decorator_visual::chimney_roof::generate_roof_chimneys(void)

{
  byte *pbVar1;
  char *pcVar2;
  undefined8 *puVar3;
  ulonglong *puVar4;
  byte bVar5;
  byte bVar6;
  undefined4 *puVar7;
  code *pcVar8;
  int iVar9;
  uint uVar10;
  uint3 uVar11;
  ulonglong uVar12;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong extraout_RAX_06;
  float *extraout_RAX_07;
  longlong *_Dst;
  undefined4 *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  uint uVar14;
  longlong in_RCX;
  longlong *plVar15;
  ulonglong in_RDX;
  ulonglong uVar16;
  longlong lVar17;
  longlong lVar18;
  longlong lVar19;
  undefined8 *in_R8;
  undefined *puVar20;
  longlong lVar21;
  ushort uVar22;
  undefined1 (*pauVar23) [16];
  uint uVar24;
  byte bVar25;
  char cVar26;
  char cVar30;
  float extraout_XMM0_Da;
  char cVar28;
  char cVar29;
  undefined1 auVar27 [16];
  float fVar31;
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  float fVar34;
  float fVar35;
  longlong in_stack_00000028;
  ulonglong local_398;
  ulonglong uStack_390;
  undefined8 local_388;
  undefined8 uStack_380;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_304;
  undefined4 local_2fc;
  undefined8 local_2f8;
  undefined4 local_2f0;
  undefined8 local_2ec;
  undefined4 local_2e4;
  undefined8 local_2e0;
  undefined8 uStack_2d8;
  undefined8 local_2d0;
  float local_278;
  float fStack_274;
  float fStack_270;
  float fStack_26c;
  undefined4 local_268;
  float fStack_264;
  undefined4 uStack_260;
  undefined4 uStack_25c;
  longlong local_258;
  longlong lStack_250;
  undefined7 local_238;
  undefined1 uStack_231;
  undefined7 uStack_230;
  byte local_218;
  undefined2 uStack_217;
  undefined4 uStack_215;
  undefined4 uStack_211;
  uint uStack_20d;
  undefined1 uStack_209;
  undefined8 local_208;
  undefined1 auStack_200 [16];
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 local_1e8;
  undefined4 uStack_1e4;
  ushort uStack_1e0;
  undefined2 uStack_1de;
  undefined4 uStack_1dc;
  longlong local_1d8;
  undefined8 local_188;
  longlong lStack_180;
  longlong local_178;
  longlong local_168;
  undefined8 local_160;
  longlong local_158;
  longlong local_150;
  undefined4 local_140;
  undefined4 local_13c;
  longlong local_138;
  ulonglong local_130;
  undefined2 local_123;
  undefined1 local_121;
  longlong *local_120;
  longlong local_118;
  longlong local_110;
  undefined1 (*local_108) [16];
  longlong *local_100;
  uint local_f8;
  byte local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_150 = 0;
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE != '\0') {
      return;
    }
LAB_1407c0cb5:
    country_core::resources::walls::decorator_subtype::impl$8::from();
    country_core::resources::walls::decorator_storage::DecoratorStorage::iter();
    local_168 = CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
    local_108 = (undefined1 (*) [16])CONCAT44(uStack_1ec,uStack_1f0);
    local_160 = *in_R8;
    local_140 = *(undefined4 *)(in_stack_00000028 + 0x1c);
    puVar7 = *(undefined4 **)(in_stack_00000028 + 0x10);
    uVar13 = (uint)uStack_1e0;
    local_f3 = local_218;
    lVar19 = auStack_200._0_8_;
    local_110 = auStack_200._8_8_;
    local_138 = local_1d8;
    uVar12 = local_208;
    uVar16 = local_130;
joined_r0x0001407c0d94:
    local_130 = uVar12;
    local_118 = lVar19;
    if (local_130 == 2) {
LAB_1407c0f47:
      local_130 = uVar16;
      if ((local_f3 & local_168 != 0) == 0) goto LAB_1407c1839;
      local_130 = 2;
      local_168 = 0;
      lVar18 = local_158;
      local_118 = lVar19;
      local_f8 = uVar13;
    }
    else {
      if (local_110 == 0) {
joined_r0x0001407c0e80:
        lVar18 = local_158;
        local_158 = lVar18;
        uVar16 = local_130;
        if ((local_130 & 1) == 0) {
          local_110 = 0;
          lVar19 = local_118;
        }
        else {
          local_110 = 0;
          lVar19 = 0;
          if (local_118 != 0) {
            local_110 = 0;
            local_118 = 0;
            local_f8 = uVar13;
            goto LAB_1407c0f93;
          }
        }
        goto LAB_1407c0f47;
      }
      if (local_138 == 0) {
        local_138 = 0;
        goto joined_r0x0001407c0e80;
      }
      if ((short)uVar13 == 0) {
        do {
          auVar27 = *local_108;
          uVar22 = (ushort)(SUB161(auVar27 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
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
                   (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar27[0xf] >> 7) << 0xf;
          local_110 = local_110 + -0x980;
          local_108 = local_108 + 1;
        } while (uVar22 == 0xffff);
        uVar13 = ~(uint)uVar22;
        local_f8 = uVar13 & -(uint)uVar22 - 2;
        lVar18 = local_110;
      }
      else {
        local_f8 = uVar13 - 1 & uVar13;
        lVar18 = local_110;
      }
      for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      }
      local_110 = lVar18;
      country_core::resources::walls::decorator_storage::impl$16::from();
      local_138 = local_138 + -1;
      uVar13 = local_f8;
      if (extraout_RAX_03 == 3) goto joined_r0x0001407c0e80;
    }
LAB_1407c0f93:
    local_158 = lVar18;
    country_core::resources::walls::decorator_storage::Decorators::iter_modified();
    lVar18 = CONCAT17((undefined1)uStack_211,CONCAT43(uStack_215,CONCAT21(uStack_217,local_218)));
    pauVar23 = (undefined1 (*) [16])CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
    uVar24 = (uint)(ushort)auStack_200._0_2_;
    local_100 = (longlong *)auStack_200._8_8_;
    local_120 = (longlong *)CONCAT44(uStack_1ec,uStack_1f0);
LAB_1407c0fe9:
    uVar13 = local_f8;
    lVar19 = local_118;
    uVar12 = local_130;
    uVar16 = local_130;
    if (local_100 != (longlong *)0x0) {
      if ((short)uVar24 == 0) {
        do {
          auVar27 = *pauVar23;
          uVar22 = (ushort)(SUB161(auVar27 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
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
                   (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar27[0xf] >> 7) << 0xf;
          lVar18 = lVar18 + -0x40;
          pauVar23 = pauVar23 + 1;
        } while (uVar22 == 0xffff);
        uVar14 = ~(uint)uVar22;
        uVar24 = uVar14 & -(uint)uVar22 - 2;
      }
      else {
        uVar14 = uVar24;
        uVar24 = uVar24 - 1 & uVar24;
        if (lVar18 == 0) goto joined_r0x0001407c0d94;
      }
      iVar9 = 0;
      for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
        iVar9 = iVar9 + 1;
      }
      iVar9 = *(int *)((lVar18 - (ulonglong)(uint)(iVar9 << 2)) + -4);
      local_218 = (byte)iVar9;
      uStack_217 = (undefined2)((uint)iVar9 >> 8);
      uStack_215._0_1_ = (undefined1)((uint)iVar9 >> 0x18);
      if (local_120[9] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        local_100 = (longlong *)((longlong)local_100 + -1);
        lVar19 = local_120[6];
        bVar25 = (byte)(extraout_RAX_04 >> 0x39);
        auVar27 = ZEXT216(CONCAT11(bVar25,bVar25));
        auVar27 = pshuflw(auVar27,auVar27,0);
        lVar21 = 0;
        uVar16 = extraout_RAX_04;
        while( true ) {
          uVar16 = local_120[7] & uVar16;
          pcVar2 = (char *)(lVar19 + uVar16);
          cVar26 = auVar27[0];
          auVar33[0] = -(*pcVar2 == cVar26);
          cVar28 = auVar27[1];
          auVar33[1] = -(pcVar2[1] == cVar28);
          cVar29 = auVar27[2];
          auVar33[2] = -(pcVar2[2] == cVar29);
          cVar30 = auVar27[3];
          auVar33[3] = -(pcVar2[3] == cVar30);
          auVar33[4] = -(pcVar2[4] == cVar26);
          auVar33[5] = -(pcVar2[5] == cVar28);
          auVar33[6] = -(pcVar2[6] == cVar29);
          auVar33[7] = -(pcVar2[7] == cVar30);
          auVar33[8] = -(pcVar2[8] == cVar26);
          auVar33[9] = -(pcVar2[9] == cVar28);
          auVar33[10] = -(pcVar2[10] == cVar29);
          auVar33[0xb] = -(pcVar2[0xb] == cVar30);
          auVar33[0xc] = -(pcVar2[0xc] == cVar26);
          auVar33[0xd] = -(pcVar2[0xd] == cVar28);
          auVar33[0xe] = -(pcVar2[0xe] == cVar29);
          auVar33[0xf] = -(pcVar2[0xf] == cVar30);
          uVar22 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
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
          uVar13 = (uint)uVar22;
          while (uVar22 != 0) {
            uVar14 = 0;
            for (uVar10 = uVar13; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
              uVar14 = uVar14 + 1;
            }
            lVar17 = (uVar14 + uVar16 & local_120[7]) * -0x50;
            if (iVar9 == *(int *)(lVar19 + -0x50 + lVar17)) {
              country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
              if (extraout_AL_00 == '\x15') {
                lVar19 = lVar19 + lVar17;
                local_f1 = *(undefined1 *)(lVar19 + -1);
                country_core::resources::walls::cached_decorator_transforms::
                CachedDecoratorTransforms::transform();
                local_13c = *(undefined4 *)(lVar19 + -0x20);
                *puVar7 = local_140;
                blueprint::DecoratorBlueprints::insert();
                if (*extraout_RAX_05 != 0 || extraout_RAX_05[1] != 0) {
                  local_218 = 0x28;
                  uStack_217 = 0xa917;
                  uStack_215 = 0x142;
                  uStack_211 = 0x100;
                  uStack_20d = 0;
                  uStack_209 = 0;
                  local_208 = 8;
                  auStack_200 = (undefined1  [16])0x0;
                    // WARNING: Subroutine does not return
                  core::panicking::panic_fmt();
                }
                extraout_RAX_05[1] = 0;
                *extraout_RAX_05 = 1;
                extraout_RAX_05[2] = CONCAT44(fStack_274,local_278);
                extraout_RAX_05[3] = CONCAT44(fStack_26c,fStack_270);
                extraout_RAX_05[4] = CONCAT44(fStack_264,local_268);
                extraout_RAX_05[5] = CONCAT44(uStack_25c,uStack_260);
                extraout_RAX_05[6] = local_258;
                extraout_RAX_05[7] = lStack_250;
                country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
                country_core::resources::walls::public_walls::PublicWalls::get();
                if (extraout_RAX_06 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                country_core::resources::walls::decorator::DecoratorExtraInfo::as_chimney();
                fVar34 = *extraout_RAX_07 + 3.0;
                local_178 = lStack_250;
                local_188 = CONCAT44(uStack_25c,uStack_260);
                lStack_180 = local_258;
                uStack_20d = 0;
                uStack_215 = 0;
                uStack_211 = 0;
                local_f2 = *(undefined1 *)(extraout_RAX_07 + 1);
                uVar11 = CONCAT21(uStack_217,local_218);
                lVar19 = extraout_RAX_05[0x18];
                if (lVar19 == extraout_RAX_05[0x16]) {
                  alloc::raw_vec::RawVec::
                  grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                            ();
                }
                lVar21 = extraout_RAX_05[0x17];
                lVar17 = lVar19 * 0x160;
                *(undefined8 *)(lVar21 + lVar17) = 0;
                *(undefined8 *)(lVar21 + 0x10 + lVar17) = 0;
                *(undefined8 *)(lVar21 + 0x20 + lVar17) = 0;
                *(undefined4 *)(lVar21 + 0x30 + lVar17) = 0;
                *(undefined8 *)(lVar21 + 0x34 + lVar17) = local_304;
                *(undefined4 *)(lVar21 + 0x3c + lVar17) = local_2fc;
                *(undefined4 *)(lVar21 + 0x40 + lVar17) = 0;
                puVar3 = (undefined8 *)(lVar21 + 0x44 + lVar17);
                *puVar3 = CONCAT17((undefined1)uStack_211,
                                   CONCAT43(uStack_215,CONCAT21(uStack_217,local_218)));
                puVar3[1] = CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
                puVar4 = (ulonglong *)(lVar21 + 0x54 + lVar17);
                *puVar4 = local_208;
                puVar4[1] = auStack_200._0_8_;
                puVar3 = (undefined8 *)(lVar21 + 100 + lVar17);
                *puVar3 = auStack_200._8_8_;
                puVar3[1] = CONCAT44(uStack_1ec,uStack_1f0);
                puVar3 = (undefined8 *)(lVar21 + 0x70 + lVar17);
                *puVar3 = CONCAT44(local_1e8,uStack_1ec);
                puVar3[1] = CONCAT26(uStack_1de,CONCAT24(uStack_1e0,uStack_1e4));
                *(undefined4 *)(lVar21 + 0x80 + lVar17) = 0;
                *(undefined8 *)(lVar21 + 0x84 + lVar17) = local_2f8;
                *(undefined4 *)(lVar21 + 0x8c + lVar17) = local_2f0;
                *(undefined4 *)(lVar21 + 0x90 + lVar17) = 0;
                *(undefined8 *)(lVar21 + 0x94 + lVar17) = local_2ec;
                *(undefined4 *)(lVar21 + 0x9c + lVar17) = local_2e4;
                *(undefined8 *)(lVar21 + 0xa0 + lVar17) = 1;
                *(undefined8 *)(lVar21 + 0xa8 + lVar17) = 0;
                *(undefined **)(lVar21 + 0xb0 + lVar17) =
                     &anon_b17bdb6b936b48c513e8fa54c354b9a3_2_llvm_3328826292648170336;
                *(undefined8 *)(lVar21 + 0xb8 + lVar17) = 10;
                *(undefined4 *)(lVar21 + 0xd0 + lVar17) = local_268;
                puVar3 = (undefined8 *)(lVar21 + 0xc0 + lVar17);
                *puVar3 = CONCAT44(fStack_274,local_278);
                puVar3[1] = CONCAT44(fStack_26c,fStack_270);
                *(float *)(lVar21 + 0xd4 + lVar17) = fVar34 + fStack_264;
                *(longlong *)(lVar21 + 0xe8 + lVar17) = local_178;
                puVar3 = (undefined8 *)(lVar21 + 0xd8 + lVar17);
                *puVar3 = local_188;
                puVar3[1] = lStack_180;
                *(undefined4 *)(lVar21 + 0xf0 + lVar17) = 0;
                *(undefined4 *)(lVar21 + 0xf8 + lVar17) = 0;
                *(undefined8 *)(lVar21 + 0x10c + lVar17) = local_2d0;
                puVar3 = (undefined8 *)(lVar21 + 0xfc + lVar17);
                *puVar3 = local_2e0;
                puVar3[1] = uStack_2d8;
                *(undefined1 *)(lVar21 + 0x114 + lVar17) = 2;
                *(undefined1 *)(lVar21 + 0x117 + lVar17) = local_121;
                *(undefined2 *)(lVar21 + 0x115 + lVar17) = local_123;
                *(undefined4 *)(lVar21 + 0x118 + lVar17) = 0;
                puVar3 = (undefined8 *)(lVar21 + 300 + lVar17);
                *puVar3 = local_388;
                puVar3[1] = uStack_380;
                puVar4 = (ulonglong *)(lVar21 + 0x11c + lVar17);
                *puVar4 = local_398;
                puVar4[1] = uStack_390;
                *(undefined1 *)(lVar21 + 0x13c + lVar17) = 2;
                *(undefined8 *)(lVar21 + 0x144 + lVar17) = 0;
                *(ulonglong *)(lVar21 + 0x13d + lVar17) = (ulonglong)uVar11;
                *(undefined1 *)(lVar21 + 0x14c + lVar17) = 0;
                *(undefined1 *)(lVar21 + 0x14d + lVar17) = local_f2;
                *(undefined1 *)(lVar21 + 0x14e + lVar17) = local_f1;
                *(undefined2 *)(lVar21 + 0x14f + lVar17) = 0x900;
                *(ulonglong *)(lVar21 + 0x151 + lVar17) = CONCAT17(uStack_231,local_238);
                *(ulonglong *)(lVar21 + 0x158 + lVar17) = CONCAT71(uStack_230,uStack_231);
                extraout_RAX_05[0x18] = lVar19 + 1;
                fVar35 = fVar34 * 0.5;
                fStack_264 = fStack_264 + fVar35;
                country_core::resources::walls::cached_decorator_transforms::
                CachedDecoratorTransforms::transform();
                fVar31 = local_208._4_4_;
                country_core::resources::walls::public_wall_state::PublicWallState::max_y();
                local_188 = CONCAT44(fVar34,0x3f59999a);
                lStack_180 = CONCAT44(lStack_180._4_4_,0x3f59999a);
                uStack_20d = uStack_20d & 0xffffff00;
                local_218 = 0;
                uStack_217 = 0;
                uStack_215 = 0;
                uStack_211 = 0;
                country_core::systems::collision::world::RaycastMeshData::new_cuboid();
                local_218 = 1;
                uStack_217 = 0;
                uStack_215 = 0;
                uStack_211 = 0x100;
                uStack_20d = 0;
                uStack_209 = 0;
                local_1e8 = (undefined4)local_378;
                uStack_1e4 = (undefined4)((ulonglong)local_378 >> 0x20);
                uStack_1e0 = (ushort)uStack_370;
                uStack_1de = (undefined2)((ulonglong)uStack_370 >> 0x10);
                uStack_1dc = (undefined4)((ulonglong)uStack_370 >> 0x20);
                uStack_1f0 = (undefined4)uStack_380;
                uStack_1ec = (undefined4)((ulonglong)uStack_380 >> 0x20);
                local_208 = local_398;
                auStack_200._8_8_ = local_388;
                auStack_200._0_8_ = uStack_390;
                std::alloc::__default_lib_allocator::__rust_alloc();
                if (_Dst == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
                  alloc::alloc::handle_alloc_error();
                }
                memcpy(_Dst,&local_218,0x90);
                LOCK();
                lVar19 = *_Dst;
                *_Dst = *_Dst + 1;
                UNLOCK();
                if (*_Dst == 0 || SCARRY8(lVar19,1) != *_Dst < 0) {
                    // WARNING: Does not return
                  pcVar8 = (code *)invalidInstructionException();
                  (*pcVar8)();
                }
                local_218 = (byte)local_268;
                uStack_217 = (undefined2)((uint)local_268 >> 8);
                uStack_215._1_3_ = SUB43(fStack_264,0);
                uStack_215 = CONCAT31(uStack_215._1_3_,(char)((uint)local_268 >> 0x18));
                uStack_211._1_3_ = (undefined3)uStack_260;
                uStack_211 = CONCAT31(uStack_211._1_3_,(char)((uint)fStack_264 >> 0x18));
                uStack_20d._0_1_ = (undefined1)((uint)uStack_260 >> 0x18);
                blueprint::DecoratorBlueprint::set_colliders();
                std::alloc::__default_lib_allocator::__rust_alloc();
                if (extraout_RAX_08 == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
                  alloc::alloc::handle_alloc_error();
                }
                *extraout_RAX_08 = 2;
                extraout_RAX_08[1] = fVar31 - extraout_XMM0_Da;
                extraout_RAX_08[2] = local_268;
                extraout_RAX_08[3] = uStack_260;
                extraout_RAX_08[4] = fStack_264 - fVar35;
                fVar34 = fStack_270 * fStack_270 + fStack_274 * fStack_274 + local_278 * local_278;
                fVar31 = fStack_270 * 0.0 + fStack_274 * 0.0 + local_278 * 1.0;
                fVar31 = fVar31 + fVar31;
                extraout_RAX_08[5] = fVar35 + fStack_264;
                *(ulonglong *)(extraout_RAX_08 + 6) =
                     CONCAT44(fVar31 * fStack_270 + (fStack_26c * fStack_26c - fVar34) * 0.0 +
                              (local_278 * 0.0 - fStack_274 * 1.0) * (fStack_26c + fStack_26c),
                              fVar31 * local_278 + (fStack_26c * fStack_26c - fVar34) * 1.0 +
                              (fStack_274 * 0.0 - fStack_270 * 0.0) * (fStack_26c + fStack_26c));
                extraout_RAX_08[8] = local_13c;
                extraout_RAX_08[9] = (fStack_264 - fVar35) + 3.0;
                if (extraout_RAX_05[0x23] != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                extraout_RAX_05[0x23] = (longlong)extraout_RAX_08;
              }
              goto LAB_1407c0fe9;
            }
            uVar22 = (ushort)(uVar13 - 1) & (ushort)uVar13;
            uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar22);
          }
          auVar32[0] = -(*pcVar2 == -1);
          auVar32[1] = -(pcVar2[1] == -1);
          auVar32[2] = -(pcVar2[2] == -1);
          auVar32[3] = -(pcVar2[3] == -1);
          auVar32[4] = -(pcVar2[4] == -1);
          auVar32[5] = -(pcVar2[5] == -1);
          auVar32[6] = -(pcVar2[6] == -1);
          auVar32[7] = -(pcVar2[7] == -1);
          auVar32[8] = -(pcVar2[8] == -1);
          auVar32[9] = -(pcVar2[9] == -1);
          auVar32[10] = -(pcVar2[10] == -1);
          auVar32[0xb] = -(pcVar2[0xb] == -1);
          auVar32[0xc] = -(pcVar2[0xc] == -1);
          auVar32[0xd] = -(pcVar2[0xd] == -1);
          auVar32[0xe] = -(pcVar2[0xe] == -1);
          auVar32[0xf] = -(pcVar2[0xf] == -1);
          if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 || (SUB161(auVar32 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar32 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar32 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar32 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar32 >> 0x2f,0) & 1) != 0) || (SUB161(auVar32 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar32 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar32 >> 0x47,0) & 1) != 0) || (SUB161(auVar32 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar32 >> 0x57,0) & 1) != 0) || (SUB161(auVar32 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar32 >> 0x67,0) & 1) != 0) || (SUB161(auVar32 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar32 >> 0x77,0) & 1) != 0) || auVar32[0xf] < '\0') break;
          uVar16 = uVar16 + lVar21 + 0x10;
          lVar21 = lVar21 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    goto joined_r0x0001407c0d94;
  }
  core::str::impl$0::rfind<ref_<str_>_>();
  if (extraout_RAX == 1) {
    if (in_RDX != 0) {
      if (in_RDX < 0x48) {
        if ((char)(&DAT_142a860d0)[in_RDX] < -0x40) {
LAB_1407c09d6:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      else if (in_RDX != 0x48) goto LAB_1407c09d6;
    }
    core::str::impl$0::rfind<ref_<str_>_>();
    if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
      if (in_RDX + 2 < 0x48) {
        if ("stem_decorator::decorator_visual::chimney_roof::generate_roof_chimneys::f"[in_RDX] <
            -0x40) {
LAB_1407c0a43:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      else if (in_RDX + 2 != 0x48) goto LAB_1407c0a43;
    }
  }
  puVar20 = &DAT_142a86225;
  do {
    bVar25 = puVar20[-1];
    uVar13 = (uint)(char)bVar25;
    if ((char)bVar25 < '\0') {
      bVar5 = puVar20[-2];
      if ((char)bVar5 < -0x40) {
        bVar6 = puVar20[-3];
        if ((char)bVar6 < -0x40) {
          pbVar1 = puVar20 + -4;
          puVar20 = puVar20 + -4;
          uVar13 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          puVar20 = puVar20 + -3;
          uVar13 = bVar6 & 0xf;
        }
        uVar13 = bVar5 & 0x3f | uVar13 << 6;
      }
      else {
        puVar20 = puVar20 + -2;
        uVar13 = bVar5 & 0x1f;
      }
      uVar13 = (uint)(bVar25 & 0x3f) | uVar13 << 6;
      if (uVar13 != 0x5c) goto LAB_1407c0b04;
LAB_1407c0b1e:
      if (puVar20 == &UNK_142a861e7) break;
      if (puVar20 + -0x142a861e7 < (undefined *)0x3d) {
        if (-0x41 < (char)puVar20[1]) break;
      }
      else if (puVar20 + -0x142a861e7 == (undefined *)0x3d) break;
                    // WARNING: Subroutine does not return
      core::str::slice_error_fail();
    }
    puVar20 = puVar20 + -1;
    if (uVar13 == 0x5c) goto LAB_1407c0b1e;
LAB_1407c0b04:
    if (uVar13 == 0x2f) goto LAB_1407c0b1e;
  } while (puVar20 != &DAT_142a861e8);
  local_100 = (longlong *)in_RCX;
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_01 == 1) {
    plVar15 = extraout_RAX_01 + 1;
  }
  else if ((*extraout_RAX_01 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar15 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0))
  goto LAB_1407c1899;
  if (*plVar15 == 0) {
    *plVar15 = -1;
    local_120 = plVar15;
    puffin::ThreadProfiler::begin_scope();
    *local_120 = *local_120 + 1;
    local_150 = 1;
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0')
    goto LAB_1407c0cb5;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 1) {
      plVar15 = extraout_RAX_02 + 1;
    }
    else if ((*extraout_RAX_02 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_10, extraout_RAX_10 == (longlong *)0x0))
    goto LAB_1407c1899;
    if (*plVar15 == 0) {
      *plVar15 = -1;
      local_100 = plVar15;
      puffin::ThreadProfiler::end_scope();
      goto LAB_1407c0c49;
    }
  }
  goto LAB_1407c18a5;
LAB_1407c1839:
  if (local_150 == 0) {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_11 == 1) {
    plVar15 = extraout_RAX_11 + 1;
  }
  else if ((*extraout_RAX_11 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar15 = extraout_RAX_12, extraout_RAX_12 == (longlong *)0x0)) {
LAB_1407c1899:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*plVar15 == 0) {
    *plVar15 = -1;
    local_100 = plVar15;
    puffin::ThreadProfiler::end_scope();
LAB_1407c0c49:
    *local_100 = *local_100 + 1;
    return;
  }
LAB_1407c18a5:
                    // WARNING: Subroutine does not return
  core::cell::panic_already_borrowed();
}

