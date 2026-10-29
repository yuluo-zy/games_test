
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent
          (void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined1 uVar3;
  byte bVar4;
  longlong lVar5;
  longlong *plVar6;
  longlong *plVar7;
  undefined1 auVar8 [16];
  undefined4 uVar9;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  undefined1 extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  char extraout_AL_08;
  char extraout_AL_09;
  char extraout_AL_10;
  char extraout_AL_11;
  char extraout_AL_12;
  char extraout_AL_13;
  undefined1 extraout_AL_14;
  byte extraout_AL_15;
  undefined4 extraout_EAX;
  int iVar10;
  float extraout_EAX_00;
  uint extraout_EAX_01;
  uint extraout_EAX_02;
  uint extraout_EAX_03;
  uint extraout_EAX_04;
  undefined4 extraout_EAX_05;
  undefined4 extraout_EAX_06;
  undefined4 extraout_EAX_07;
  uint extraout_EAX_08;
  int extraout_EAX_09;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  ulonglong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  longlong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  ulonglong extraout_RAX_11;
  ulonglong uVar13;
  undefined8 extraout_RAX_12;
  longlong extraout_RAX_13;
  float *extraout_RAX_14;
  undefined8 *in_RCX;
  undefined1 uVar14;
  byte bVar15;
  longlong lVar16;
  uint uVar17;
  longlong *in_R8;
  longlong *plVar18;
  longlong in_R9;
  byte bVar19;
  undefined8 uVar20;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar21;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  undefined4 extraout_XMM0_Da_08;
  undefined4 extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  uint extraout_XMM0_Da_11;
  undefined4 extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  uint extraout_XMM0_Da_14;
  undefined4 extraout_XMM0_Da_15;
  float extraout_XMM0_Da_16;
  uint extraout_XMM0_Da_17;
  undefined4 extraout_XMM0_Da_18;
  float extraout_XMM0_Da_19;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  float fVar34;
  float fVar35;
  undefined4 uVar36;
  undefined4 uVar37;
  float *in_stack_00000028;
  undefined4 in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 in_stack_00000060;
  undefined8 in_stack_00000068;
  byte in_stack_00000070;
  byte in_stack_00000080;
  undefined8 local_278;
  undefined8 uStack_270;
  float *apfStack_260 [2];
  float fStack_250;
  undefined1 auStack_248 [16];
  undefined1 auStack_238 [16];
  float fStack_228;
  float fStack_224;
  float *pfStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  code *pcStack_200;
  undefined1 *puStack_1f8;
  code *pcStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined4 uStack_1d0;
  undefined1 uStack_1cc;
  undefined1 uStack_1cb;
  byte bStack_1ca;
  undefined1 uStack_1c9;
  undefined1 uStack_1bc;
  char cStack_1b9;
  undefined4 uStack_1b8;
  undefined1 uStack_1b4;
  undefined4 uStack_1b0;
  float fStack_1a8;
  float fStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  ulonglong uStack_198;
  float fStack_190;
  float fStack_18c;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 uStack_178;
  undefined8 uStack_170;
  undefined1 *puStack_168;
  undefined8 local_138;
  longlong *plStack_130;
  undefined1 uStack_128;
  undefined1 uStack_127;
  undefined2 uStack_126;
  float fStack_124;
  float fStack_120;
  float fStack_11c;
  undefined8 uStack_118;
  undefined8 uStack_110;
  float fStack_104;
  float *local_100;
  undefined1 uStack_f2;
  undefined1 uStack_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  fVar21 = in_stack_00000028[8];
  auVar33 = ZEXT416((uint)fVar21);
  fVar22 = in_stack_00000028[9];
  fVar35 = in_stack_00000028[10];
  auVar32 = ZEXT416((uint)fVar35);
  if (((0x7f7fffff < (uint)ABS(fVar21)) || (0x7f7fffff < (uint)ABS(fVar22))) ||
     (0x7f7fffff < (uint)ABS(fVar35))) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  local_138 = CONCAT44(local_138._4_4_,in_stack_00000030);
  fVar24 = *in_stack_00000028;
  fVar34 = in_stack_00000028[1];
  local_218 = *(undefined ***)in_stack_00000028;
  fVar23 = in_stack_00000028[2];
  fVar29 = in_stack_00000028[3];
  uStack_210 = *(code **)(in_stack_00000028 + 2);
  plVar18 = in_R8;
  glam::f32::sse2::vec4::Vec4::is_finite_mask();
  auVar8._4_4_ = local_188._4_4_;
  auVar8._0_4_ = (float)local_188;
  auVar8._8_4_ = (float)uStack_180;
  auVar8._12_4_ = uStack_180._4_4_;
  iVar10 = movmskps(extraout_EAX,auVar8);
  if (iVar10 != 0xf) {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  local_100 = in_stack_00000028 + 8;
  plStack_130 = in_RCX;
                    /* WARNING: Switch is manually overridden */
  switch(*(undefined1 *)(in_stack_00000028 + 4)) {
  case 1:
    pfStack_220 = in_stack_00000028 + 6;
    country_core::resources::walls::decorator::DecoratorType::can_place_on_roof();
    if (extraout_AL_04 == '\0') {
      *(float *)(in_RCX + 5) = local_100[2];
      in_RCX[4] = *(ulonglong *)local_100;
      *(float *)(in_RCX + 2) = fVar24;
      *(float *)((longlong)in_RCX + 0x14) = fVar34;
      *(float *)(in_RCX + 3) = fVar23;
      *(float *)((longlong)in_RCX + 0x1c) = fVar29;
      *in_RCX = 4;
      return;
    }
    plVar6 = *(longlong **)(in_stack_00000028 + 6);
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    if (extraout_RAX_04 >> 0x20 == 0) {
      local_188 = &pfStack_220;
      uStack_180._0_4_ = 10.6978;
      uStack_180._4_4_ = 1.4013e-45;
      local_218 = (undefined **)&UNK_142c245a8;
      uStack_210 = (code *)0x1;
      puStack_1f8 = (undefined1 *)0x0;
      uStack_208 = &local_188;
      pcStack_200 = (code *)0x1;
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_fmt();
    }
    lVar5 = in_R8[1];
    uVar13 = extraout_RAX_04 & 0xffffffff;
    if ((uVar13 < *(ulonglong *)(lVar5 + 0x10)) &&
       (plVar7 = *(longlong **)(lVar5 + 8), plVar18 = plVar7,
       *(int *)((longlong)plVar7 + uVar13 * 0x14) == (int)(extraout_RAX_04 >> 0x20))) {
      uVar17 = *(uint *)((longlong)plVar7 + uVar13 * 0x14 + 4);
      plVar18 = (longlong *)(ulonglong)uVar17;
      if (plVar18 != (longlong *)0xffffffff) {
        lVar11 = *in_R8;
        uVar37 = 0;
        if ((plVar18 < *(longlong **)(lVar11 + 0x38)) &&
           ((*(ulonglong *)(*(longlong *)(lVar11 + 0x28) + (ulonglong)(uVar17 >> 6) * 8) >>
             ((ulonglong)plVar18 & 0x3f) & 1) != 0)) {
          uVar12 = (ulonglong)*(uint *)((longlong)plVar7 + uVar13 * 0x14 + 0xc);
          uVar17 = *(uint *)((longlong)plVar7 + uVar13 * 0x14 + 0x10);
          lVar5 = *(longlong *)
                   (*(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x18 + uVar12 * 0x48) + 0x10 +
                   ~*(ulonglong *)
                     (*(longlong *)(*(longlong *)(lVar5 + 0x1a8) + 0x38 + uVar12 * 0x48) +
                     *(longlong *)(lVar11 + 0x110) * 8) * 0x30);
          country_core::resources::roofs::roof_shape::Roof::is_flat();
          if (in_stack_00000080 == 1 && extraout_AL_05 == '\0') {
code_r0x00014127e121:
            *(float *)(in_RCX + 5) = local_100[2];
            in_RCX[4] = *(ulonglong *)local_100;
            *(float *)(in_RCX + 2) = fVar24;
            *(float *)((longlong)in_RCX + 0x14) = fVar34;
            *(float *)(in_RCX + 3) = fVar23;
            *(float *)((longlong)in_RCX + 0x1c) = fVar29;
            *in_RCX = 4;
            return;
          }
          bVar4 = *(byte *)((longlong)in_stack_00000038 + 0x46);
          fVar21 = 0.0;
          if ((~bVar4 & 0x3e) == 0 || (~bVar4 & 0x38) != 0) {
            country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat();
            fVar21 = extraout_XMM0_Da_05;
          }
          if (((uint)in_stack_00000028[0xb] & 1) == 0) {
                    /* WARNING: Subroutine does not return */
            core::option::unwrap_failed();
          }
          fVar22 = in_stack_00000028[0xc];
          fVar35 = in_stack_00000028[0xd];
          country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians();
          fVar21 = fVar21 - extraout_XMM0_Da_06;
          uStack_198 = *(ulonglong *)local_100;
          fStack_190 = local_100[2];
          fStack_1a8 = (float)CONCAT31(fStack_1a8._1_3_,
                                       *(undefined1 *)((longlong)in_stack_00000038 + 0x44));
          fStack_228 = fVar22;
          fStack_224 = fVar35;
          if (bVar4 == 0x3d) {
            country_core::resources::roofs::roof_shape::Roof::is_flat();
            if ((char)extraout_RAX_05 == '\0') {
              uStack_210 = (code *)CONCAT44(uStack_210._4_4_,local_100[2]);
              local_218 = *(undefined ***)local_100;
              country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge();
              iVar10 = CONCAT22(uStack_126,CONCAT11(uStack_127,uStack_128));
              uVar37 = (undefined4)local_138;
              if (iVar10 == 2) {
                alloc::alloc::exchange_malloc();
                fStack_11c = fStack_11c + _UNK_142a89ef4;
                uStack_118._0_4_ = (float)uStack_118 + _UNK_142a89ef8;
                uStack_118._4_4_ = uStack_118._4_4_ + _UNK_142a89efc;
                *extraout_RAX_14 = fStack_120 + ___xmm_00000000000000003e4ccccd00000000;
                extraout_RAX_14[1] = fStack_11c;
                extraout_RAX_14[2] = (float)uStack_118;
                extraout_RAX_14[3] = uStack_118._4_4_;
                *(ulonglong *)(extraout_RAX_14 + 4) =
                     CONCAT44((float)((ulonglong)uStack_110 >> 0x20) + 0.0,
                              __real_3e4ccccd + (float)uStack_110);
                local_188._0_4_ = 2.8026e-45;
                local_188._4_4_ = 0.0;
                uStack_180._0_4_ = SUB84(extraout_RAX_14,0);
                uStack_180._4_4_ = (float)((ulonglong)extraout_RAX_14 >> 0x20);
                uStack_178._0_4_ = 2.8026e-45;
                uStack_178._4_4_ = 0.0;
                country_core::utils::debug_viz::DebugLine::new();
                country_core::utils::debug_viz::DebugLine::line_width();
                country_core::utils::debug_viz::DebugLine::dashed();
                country_core::utils::debug_viz::DebugLine::show();
              }
              else if (iVar10 == 1) {
                fStack_250 = fStack_11c;
                country_core::utils::debug_viz::DebugSphere::new();
                country_core::utils::debug_viz::DebugSphere::radius();
                country_core::utils::debug_viz::DebugSphere::solid();
                country_core::utils::debug_viz::DebugSphere::show();
              }
              uStack_180._0_4_ = local_100[2];
              local_188._0_4_ = (float)*(ulonglong *)local_100;
              local_188._4_4_ = (float)(*(ulonglong *)local_100 >> 0x20);
              country_core::systems::wall::shapes::shape_placement::snap_to_roof_ridge();
              uVar13 = (ulonglong)local_218 & 0xffffffff;
              if ((int)local_218 != 0) {
                fVar22 = ___xmm_0000000000000000400333333d4ccccd;
                fVar35 = _UNK_142c24174;
                if ((int)local_218 != 1) {
                  fVar22 = local_218._4_4_;
                  fVar35 = _UNK_142c24164;
                }
                fVar21 = 0.0;
              }
              uVar36 = 0;
              plStack_130 = plVar6;
              iVar10 = 0;
            }
            else {
              uVar36 = 0;
              uVar13 = extraout_RAX_05;
              plStack_130 = plVar6;
              uVar37 = (undefined4)local_138;
              iVar10 = 0;
            }
          }
          else {
            if ((__real_3f800000 < fVar35) &&
               (country_core::resources::roofs::roof_shape::Roof::is_flat(), extraout_AL_08 == '\0')
               ) goto code_r0x00014127e121;
            local_218 = (undefined **)in_stack_00000038;
            uStack_210 = (code *)in_stack_00000060;
            uStack_208 = (ulonglong *)in_stack_00000068;
            plStack_130 = plVar6;
            derive_decorator_subtype::derive_decorator_subtype();
            country_core::resources::walls::decorator::DecoratorRank::new();
            country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
            std::collections::hash::map::HashMap::
            get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                      ();
            if (extraout_RAX_06 == 0) goto code_r0x00014127ecaf;
            iVar10 = 0;
            uVar37 = (undefined4)local_138;
            uVar13 = extraout_RAX_06;
            if (((in_stack_00000070 == 0) ||
                (country_core::resources::roofs::roof_shape::Roof::is_flat(),
                uVar13 = extraout_RAX_07, (char)extraout_RAX_07 != '\0')) ||
               (country_core::resources::walls::decorator::DecoratorType::max_rank(),
               uVar13 = extraout_RAX_08, extraout_RAX_08 < 2)) {
              uVar36 = 0;
            }
            else {
              uStack_128 = 0;
              uStack_127 = 0;
              uStack_126 = 0;
              fStack_124 = 0.0;
              fStack_120 = 5.60519e-45;
              fStack_11c = 0.0;
              uStack_118 = 0;
              country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators()
              ;
              if (extraout_RAX_09 != 0) {
                uStack_1c9 = *(undefined1 *)((longlong)in_stack_00000038 + 0x47);
                uStack_1cb = *(undefined1 *)((longlong)in_stack_00000038 + 0x45);
                uStack_1d0 = *(undefined4 *)(in_stack_00000038 + 8);
                uStack_1e0 = in_stack_00000038[6];
                uStack_1d8 = in_stack_00000038[7];
                local_218 = (undefined **)CONCAT44(local_218._4_4_,uVar37);
                uStack_210 = (code *)*in_stack_00000038;
                uStack_208 = (ulonglong *)in_stack_00000038[1];
                pcStack_200 = (code *)in_stack_00000038[2];
                puStack_1f8 = (undefined1 *)in_stack_00000038[3];
                pcStack_1f0 = (code *)in_stack_00000038[4];
                uStack_1e8 = in_stack_00000038[5];
                uStack_1cc = fStack_1a8._0_1_;
                uStack_180._0_4_ = local_100[2];
                local_188._0_4_ = (float)*(ulonglong *)local_100;
                local_188._4_4_ = (float)(*(ulonglong *)local_100 >> 0x20);
                bStack_1ca = bVar4;
                uStack_178._0_4_ = fVar24;
                uStack_178._4_4_ = fVar34;
                uStack_170._0_4_ = fVar23;
                uStack_170._4_4_ = fVar29;
                merge_proposal_roof_decorators::
                intersect_roof_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env_1>
                          ();
              }
              country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators()
              ;
              local_188._0_4_ = SUB84(in_stack_00000038,0);
              local_188._4_4_ = (float)((ulonglong)in_stack_00000038 >> 0x20);
              uStack_180 = &fStack_228;
              uStack_178._0_4_ = SUB84(local_100,0);
              uStack_178._4_4_ = (float)((ulonglong)local_100 >> 0x20);
              puStack_168 = &uStack_128;
              uStack_170 = (ulonglong)uVar17 * 0x58 + lVar5;
              core::option::Option::
              and_then<ref_<country_core::resources::walls::decorator_storage::Decorators>,system_decorator::merge_proposal_roof_decorators::RoofDecoratorMergeProposal,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_env_2>
                        ();
              uVar13 = extraout_RAX_10;
              if (CONCAT44(fStack_124,CONCAT22(uStack_126,CONCAT11(uStack_127,uStack_128))) != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                uVar13 = extraout_RAX_11;
              }
              uVar36 = 0;
              if ((int)local_218 != 4) {
                local_278 = uStack_208;
                uStack_270 = pcStack_200;
                uVar13 = (ulonglong)CONCAT14(uStack_1b4,uStack_1b0);
                uVar36 = 1;
                iVar10 = (int)local_218;
                fVar21 = local_218._4_4_;
                fVar22 = (float)uStack_210;
                fVar35 = uStack_210._4_4_;
              }
            }
          }
          *(float *)(in_RCX + 0xb) = fStack_190;
          in_RCX[10] = uStack_198;
          *in_RCX = in_stack_00000060;
          in_RCX[1] = in_stack_00000068;
          in_RCX[2] = 0;
          in_RCX[3] = plStack_130;
          *(int *)(in_RCX + 4) = iVar10;
          *(float *)((longlong)in_RCX + 0x24) = fVar21;
          *(float *)(in_RCX + 5) = fVar22;
          *(float *)((longlong)in_RCX + 0x2c) = fVar35;
          in_RCX[6] = local_278;
          in_RCX[7] = uStack_270;
          *(undefined4 *)(in_RCX + 8) = uVar36;
          *(ulonglong *)((longlong)in_RCX + 0x44) = uVar13;
          *(undefined4 *)((longlong)in_RCX + 0x4c) = uVar37;
          plStack_130 = in_RCX;
          goto code_r0x00014127ea54;
        }
        goto code_r0x00014127ec68;
      }
    }
    uVar17 = (uint)plVar18;
    uVar37 = 1;
code_r0x00014127ec68:
    local_218 = (undefined **)CONCAT44(uVar17,uVar37);
                    /* WARNING: Subroutine does not return */
    core::result::unwrap_failed();
  case 2:
    country_core::resources::walls::decorator::DecoratorType::can_place_on_terrain();
    if (extraout_AL_02 == '\0') {
      *(float *)(in_RCX + 2) = local_100[2];
      in_RCX[1] = *(ulonglong *)local_100;
      *in_RCX = 3;
      return;
    }
    country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat();
    in_RCX[10] = *(ulonglong *)local_100;
    *(float *)(in_RCX + 0xb) = local_100[2];
    fStack_1a8._0_1_ = *(undefined1 *)((longlong)in_stack_00000038 + 0x44);
    *in_RCX = in_stack_00000060;
    in_RCX[1] = in_stack_00000068;
    in_RCX[2] = 1;
    *(undefined4 *)(in_RCX + 4) = 2;
    *(undefined4 *)((longlong)in_RCX + 0x24) = extraout_XMM0_Da_01;
    *(float *)(in_RCX + 5) = fVar21;
    *(float *)((longlong)in_RCX + 0x2c) = fVar35;
    *(undefined4 *)(in_RCX + 8) = 0;
    *(undefined4 *)((longlong)in_RCX + 0x4c) = (undefined4)local_138;
code_r0x00014127ea54:
    *(undefined1 *)((longlong)plStack_130 + 0x5c) = fStack_1a8._0_1_;
    return;
  case 3:
    apfStack_260[0] = in_stack_00000028 + 5;
    country_core::resources::walls::decorator::DecoratorType::can_place_on_roof();
    if (extraout_AL_03 == '\0') goto code_r0x00014127e121;
    lVar11 = *(longlong *)(in_R9 + 0x10) * 0x150;
    fVar22 = in_stack_00000028[5];
    fVar24 = in_stack_00000028[6];
    lVar5 = *(longlong *)(in_R9 + 8) + -0x150;
    do {
      lVar16 = lVar5;
      if (lVar11 == 0) {
        country_core::resources::stairs::stairs_assembly_params::impl$2::from();
        itertools::Itertools::
        collect_vec<core::iter::adapters::map::Map<core::slice::iter::Iter<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_interaction_intent::closure_4::closure_env_0>_>
                  ();
        local_218 = (undefined **)apfStack_260;
        uStack_210 = core::fmt::impl$74::
                     fmt<country_core::resources::stairs::stairs_state::StairGraphEdge>;
        uStack_208 = &uStack_198;
        pcStack_200 = country_core::resources::stairs::stairs_assembly_params::impl$23::fmt;
        pcStack_1f0 = alloc::vec::impl$27::
                      fmt<country_core::resources::stairs::stairs_state::StairGraphEdge,alloc::alloc::Global>
        ;
        local_188._0_4_ = 97.13666;
        local_188._4_4_ = 1.4013e-45;
        uStack_180._0_4_ = 4.2039e-45;
        uStack_180._4_4_ = 0.0;
        puStack_168 = (undefined1 *)0x0;
        uStack_170._0_4_ = 4.2039e-45;
        uStack_170._4_4_ = 0.0;
        puStack_1f8 = &uStack_128;
        uStack_178 = &local_218;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      lVar11 = lVar11 + -0x150;
      lVar5 = lVar16 + 0x150;
    } while ((*(float *)(lVar16 + 0x278) != fVar22) || (*(float *)(lVar16 + 0x27c) != fVar24));
    lVar5 = *(longlong *)(lVar16 + 0x1c0);
    uVar13 = *(ulonglong *)(lVar16 + 0x1c8);
    local_188._4_4_ = 0.0;
    uStack_180._0_4_ = 0.0;
    uStack_180._4_4_ = 0.0;
    uStack_178._0_4_ = 0.0;
    uStack_178._4_4_ = 0.0;
    uStack_170._0_4_ = 0.0;
    ::utils::curve::Curve::
    get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
              ();
    fVar34 = __real_3f800000;
    uVar12 = (ulonglong)extraout_EAX_01;
    if (uVar13 <= uVar12) {
code_r0x00014127eb07:
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    if (uVar13 <= uVar12 + 1) {
code_r0x00014127eb16:
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    if (*(ulonglong *)(lVar16 + 0x160) <= uVar12) goto code_r0x00014127eaec;
    if (*(ulonglong *)(lVar16 + 0x160) <= uVar12 + 1) goto code_r0x00014127eaf8;
    uVar1 = *(undefined8 *)(lVar5 + uVar12 * 8);
    uVar20 = *(undefined8 *)(lVar5 + 8 + uVar12 * 8);
    fVar25 = (float)uVar1;
    fVar23 = (float)((ulonglong)uVar1 >> 0x20);
    fVar23 = ((float)((ulonglong)uVar20 >> 0x20) - fVar23) * extraout_XMM0_Da_02 + fVar23;
    fVar25 = ((float)uVar20 - fVar25) * extraout_XMM0_Da_02 + fVar25;
    uVar1 = *(undefined8 *)(*(longlong *)(lVar16 + 0x158) + uVar12 * 8);
    uVar20 = *(undefined8 *)(*(longlong *)(lVar16 + 0x158) + 8 + uVar12 * 8);
    fVar29 = ((float)uVar20 * extraout_XMM0_Da_02 +
             (__real_3f800000 - extraout_XMM0_Da_02) * (float)uVar1) - fVar21;
    fVar30 = ((float)((ulonglong)uVar20 >> 0x20) * extraout_XMM0_Da_02 +
             (__real_3f800000 - extraout_XMM0_Da_02) * (float)((ulonglong)uVar1 >> 0x20)) - fVar35;
    fVar27 = fVar25 * fVar30 - fVar23 * fVar29;
    fVar29 = fVar29 + fVar27 * fVar23;
    fVar30 = fVar30 - fVar27 * fVar25;
    local_188._0_4_ =
         (float)((uint)fVar27 & ___xmm_7fffffff7fffffff7fffffff7fffffff) +
         fVar30 * fVar30 + fVar29 * fVar29;
    ::utils::curve::Curve::
    get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
              ();
    uVar12 = (ulonglong)extraout_EAX_02;
    if (uVar13 <= uVar12) goto code_r0x00014127eb07;
    if (uVar13 <= uVar12 + 1) goto code_r0x00014127eb16;
    if (*(ulonglong *)(lVar16 + 0x160) <= uVar12) goto code_r0x00014127eaec;
    auStack_248 = auVar32;
    auStack_238 = auVar33;
    fStack_104 = fVar24;
    if (uVar12 + 1 < *(ulonglong *)(lVar16 + 0x160)) {
      uVar1 = *(undefined8 *)(lVar5 + uVar12 * 8);
      uVar20 = *(undefined8 *)(lVar5 + 8 + uVar12 * 8);
      fVar29 = (float)uVar1;
      fVar24 = (float)((ulonglong)uVar1 >> 0x20);
      fVar24 = ((float)((ulonglong)uVar20 >> 0x20) - fVar24) * extraout_XMM0_Da_03 + fVar24;
      fVar29 = ((float)uVar20 - fVar29) * extraout_XMM0_Da_03 + fVar29;
      uVar1 = *(undefined8 *)(*(longlong *)(lVar16 + 0x158) + uVar12 * 8);
      uVar20 = *(undefined8 *)(*(longlong *)(lVar16 + 0x158) + 8 + uVar12 * 8);
      fVar23 = ((float)uVar20 * extraout_XMM0_Da_03 + (fVar34 - extraout_XMM0_Da_03) * (float)uVar1)
               - fVar21;
      fVar27 = ((float)((ulonglong)uVar20 >> 0x20) * extraout_XMM0_Da_03 +
               (fVar34 - extraout_XMM0_Da_03) * (float)((ulonglong)uVar1 >> 0x20)) - fVar35;
      fVar25 = fVar29 * fVar27 - fVar24 * fVar23;
      fVar23 = fVar23 + fVar25 * fVar24;
      fVar27 = fVar27 - fVar25 * fVar29;
      uStack_170._4_4_ =
           (float)((uint)fVar25 & ___xmm_7fffffff7fffffff7fffffff7fffffff) +
           fVar27 * fVar27 + fVar23 * fVar23;
      iVar10 = 0;
      fVar24 = 0.0;
      uStack_1a0 = 0;
      uStack_19c = 0;
      fVar23 = fVar34;
      fStack_1a8 = fVar21;
      fStack_1a4 = fVar35;
      do {
        fVar29 = fStack_1a4;
        fVar35 = fStack_1a8;
        fVar21 = ___real_3e124925;
        uVar17 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
        fVar34 = fVar34 - fVar24;
        fVar25 = fVar34 * ___real_3e124925 + fVar24;
        uStack_198 = CONCAT44(uStack_198._4_4_,fVar25);
        if ((fVar25 < 0.0) || (fVar23 < fVar25)) {
code_r0x00014127ddd9:
          uStack_128 = SUB81(&uStack_198,0);
          uStack_127 = (undefined1)((ulonglong)&uStack_198 >> 8);
          uStack_126 = (undefined2)((ulonglong)&uStack_198 >> 0x10);
          fStack_124 = (float)((ulonglong)&uStack_198 >> 0x20);
          fStack_120 = 6.774544;
          fStack_11c = 1.4013e-45;
          local_218 = &anon_b35f6c505c1858eb9906e98b629952b4_3_llvm_7383313424677244125;
          uStack_210 = (code *)0x1;
          puStack_1f8 = (undefined1 *)0x0;
          uStack_208 = (ulonglong *)&uStack_128;
          pcStack_200 = (code *)0x1;
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        lVar11 = 2;
        while( true ) {
          if (NAN(fVar25)) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic();
          }
          ::utils::curve::Curve::
          get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                    ();
          fVar25 = __real_3f800000;
          uVar12 = (ulonglong)extraout_EAX_03;
          if (uVar13 <= uVar12) goto code_r0x00014127eb07;
          if (uVar13 <= uVar12 + 1) goto code_r0x00014127eb16;
          if (*(ulonglong *)(lVar16 + 0x160) <= uVar12) goto code_r0x00014127eaec;
          if (*(ulonglong *)(lVar16 + 0x160) <= uVar12 + 1) goto code_r0x00014127eaf8;
          uVar1 = *(undefined8 *)(lVar5 + uVar12 * 8);
          uVar20 = *(undefined8 *)(lVar5 + 8 + uVar12 * 8);
          fVar26 = (float)uVar1;
          fVar27 = (float)((ulonglong)uVar1 >> 0x20);
          fVar27 = ((float)((ulonglong)uVar20 >> 0x20) - fVar27) * extraout_XMM0_Da_04 + fVar27;
          fVar26 = ((float)uVar20 - fVar26) * extraout_XMM0_Da_04 + fVar26;
          uVar1 = *(undefined8 *)(*(longlong *)(lVar16 + 0x158) + uVar12 * 8);
          uVar20 = *(undefined8 *)(*(longlong *)(lVar16 + 0x158) + 8 + uVar12 * 8);
          fVar30 = ((float)uVar20 * extraout_XMM0_Da_04 +
                   (fVar23 - extraout_XMM0_Da_04) * (float)uVar1) - fVar35;
          fVar31 = ((float)((ulonglong)uVar20 >> 0x20) * extraout_XMM0_Da_04 +
                   (fVar23 - extraout_XMM0_Da_04) * (float)((ulonglong)uVar1 >> 0x20)) - fVar29;
          fVar28 = fVar26 * fVar31 - fVar27 * fVar30;
          fVar30 = fVar30 + fVar28 * fVar27;
          fVar31 = fVar31 - fVar28 * fVar26;
          (&fStack_18c)[lVar11] = (float)((uint)fVar28 & uVar17) + fVar31 * fVar31 + fVar30 * fVar30
          ;
          if (lVar11 == 7) break;
          fVar25 = (float)lVar11 * fVar21 * fVar34 + fVar24;
          uStack_198 = CONCAT44(uStack_198._4_4_,fVar25);
          if ((fVar25 < 0.0) || (lVar11 = lVar11 + 1, fVar23 < fVar25)) goto code_r0x00014127ddd9;
        }
        if (NAN(local_188._4_4_) || NAN((float)local_188)) {
code_r0x00014127eb9d:
                    /* WARNING: Subroutine does not return */
          core::option::unwrap_failed();
        }
        fVar21 = local_188._4_4_;
        if ((float)local_188 <= local_188._4_4_) {
          fVar21 = (float)local_188;
        }
        if (NAN(fVar21) || NAN((float)uStack_180)) goto code_r0x00014127eb9d;
        fVar35 = (float)uStack_180;
        if (fVar21 <= (float)uStack_180) {
          fVar35 = fVar21;
        }
        if (NAN(fVar35) || NAN(uStack_180._4_4_)) goto code_r0x00014127eb9d;
        fVar23 = uStack_180._4_4_;
        if (fVar35 <= uStack_180._4_4_) {
          fVar23 = fVar35;
        }
        if (NAN(fVar23) || NAN((float)uStack_178)) goto code_r0x00014127eb9d;
        fVar29 = (float)uStack_178;
        if (fVar23 <= (float)uStack_178) {
          fVar29 = fVar23;
        }
        if (NAN(fVar29) || NAN(uStack_178._4_4_)) goto code_r0x00014127eb9d;
        fVar27 = uStack_178._4_4_;
        if (fVar29 <= uStack_178._4_4_) {
          fVar27 = fVar29;
        }
        if (NAN(fVar27) || NAN((float)uStack_170)) goto code_r0x00014127eb9d;
        fVar30 = (float)uStack_170;
        if (fVar27 <= (float)uStack_170) {
          fVar30 = fVar27;
        }
        if (NAN(fVar30) || NAN(uStack_170._4_4_)) goto code_r0x00014127eb9d;
        uVar12 = (ulonglong)(local_188._4_4_ < (float)local_188);
        if ((float)uStack_180 < fVar21) {
          uVar12 = 2;
        }
        if (uStack_180._4_4_ < fVar35) {
          uVar12 = 3;
        }
        if ((float)uStack_178 < fVar23) {
          uVar12 = 4;
        }
        if (uStack_178._4_4_ < fVar29) {
          uVar12 = 5;
        }
        if ((float)uStack_170 < fVar27) {
          uVar12 = 6;
        }
        if (uStack_170._4_4_ < fVar30) {
          uVar12 = 7;
        }
        lVar11 = 0;
        if (uVar12 != 0) {
          lVar11 = uVar12 - 1;
        }
        if (5 < uVar12) {
          uVar12 = 6;
        }
        local_188._0_4_ = *(float *)((longlong)&local_188 + lVar11 * 4);
        uStack_170._4_4_ = *(float *)((longlong)&local_188 + (uVar12 + 1) * 4);
        fVar21 = (float)lVar11 * ___real_3e124925 * fVar34 + fVar24;
        fVar24 = (float)(uVar12 + 1) * ___real_3e124925 * fVar34 + fVar24;
        fVar34 = fVar24;
        if (fVar24 <= fVar21) {
          fVar34 = fVar21;
        }
        iVar10 = iVar10 + 1;
        fVar35 = fVar24;
        if (fVar21 <= fVar24) {
          fVar35 = fVar21;
        }
        fVar24 = (float)(~-(uint)NAN(fVar21) & (uint)fVar35 | -(uint)NAN(fVar21) & (uint)fVar24);
        fVar23 = __real_3f800000;
      } while (iVar10 != 4);
      fVar21 = (fVar34 + fVar24) * __real_3f000000;
      uStack_198 = CONCAT44(uStack_198._4_4_,fVar21);
      if ((fVar21 < 0.0) || (__real_3f800000 < fVar21)) {
        uStack_128 = SUB81(&uStack_198,0);
        uStack_127 = (undefined1)((ulonglong)&uStack_198 >> 8);
        uStack_126 = (undefined2)((ulonglong)&uStack_198 >> 0x10);
        fStack_124 = (float)((ulonglong)&uStack_198 >> 0x20);
        fStack_120 = 6.7744827;
        fStack_11c = 1.4013e-45;
        local_218 = (undefined **)&UNK_142c24648;
        uStack_210 = (code *)0x1;
        puStack_1f8 = (undefined1 *)0x0;
        uStack_208 = (ulonglong *)&uStack_128;
        pcStack_200 = (code *)0x1;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      uVar9 = (undefined4)local_138;
      uVar37 = auStack_248._0_4_;
      uVar36 = auStack_238._0_4_;
      if (NAN(fVar21)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                ();
      uVar13 = (ulonglong)extraout_EAX_04;
      if (*(ulonglong *)(lVar16 + 0x160) <= uVar13) goto code_r0x00014127eaec;
      uVar12 = uVar13 + 1;
      if (uVar12 < *(ulonglong *)(lVar16 + 0x160)) {
        if (*(ulonglong *)(lVar16 + 0x1c8) <= uVar13) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        if (*(ulonglong *)(lVar16 + 0x1c8) <= uVar12) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        lVar5 = *(longlong *)(lVar16 + 0x1c0);
        fVar35 = *(float *)(lVar5 + uVar13 * 8);
        fVar24 = *(float *)(lVar5 + 4 + uVar13 * 8);
        fVar35 = (*(float *)(lVar5 + 8 + uVar13 * 8) - fVar35) * extraout_XMM0_Da_07 + fVar35;
        fVar24 = (*(float *)(lVar5 + 0xc + uVar13 * 8) - fVar24) * extraout_XMM0_Da_07 + fVar24;
        fVar34 = fVar25 / SQRT(fVar24 * fVar24 + fVar35 * fVar35);
        if ((int)fVar34 - 1U < 0x7fffff ||
            (int)ABS(fVar34) - 0x800000U < 0x7f000000 && -1 < (int)fVar34) {
          fVar35 = fVar35 * fVar34;
          fVar24 = fVar24 * (float)((uint)fVar34 ^ ___xmm_80000000800000008000000080000000);
        }
        else {
          fVar35 = 0.0;
          fVar24 = __real_80000000;
        }
        if (*(ulonglong *)(lVar16 + 0x1b0) <= uVar13) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        if (*(ulonglong *)(lVar16 + 0x1b0) <= uVar12) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        lVar5 = *(longlong *)(lVar16 + 0x158);
        fVar25 = fVar25 - extraout_XMM0_Da_07;
        fVar34 = *(float *)(lVar5 + uVar13 * 8);
        fVar23 = *(float *)(lVar5 + 4 + uVar13 * 8);
        fVar29 = *(float *)(lVar5 + 8 + uVar13 * 8);
        fVar27 = *(float *)(lVar5 + 0xc + uVar13 * 8);
        fVar30 = *(float *)(*(longlong *)(lVar16 + 0x1a8) + uVar13 * 4);
        fVar30 = ((*(float *)(*(longlong *)(lVar16 + 0x1a8) + 4 + uVar13 * 4) - fVar30) *
                  extraout_XMM0_Da_07 + fVar30) * __real_3f000000;
        country_core::resources::walls::decorator::DecoratorRotationXZ::from_quat();
        country_core::resources::walls::decorator::DecoratorRotationXZ::from_xz_forward_vector();
        country_core::resources::walls::decorator::DecoratorRotationXZ::offset();
        plStack_130[10] = *(ulonglong *)local_100;
        *(float *)(plStack_130 + 0xb) = local_100[2];
        fStack_1a8._0_1_ = *(undefined1 *)((longlong)in_stack_00000038 + 0x44);
        *plStack_130 = in_stack_00000060;
        plStack_130[1] = in_stack_00000068;
        plStack_130[2] = 2;
        *(undefined4 *)(plStack_130 + 4) = 3;
        *(undefined4 *)((longlong)plStack_130 + 0x24) = extraout_XMM0_Da_09;
        *(float *)(plStack_130 + 5) = fVar22;
        *(float *)((longlong)plStack_130 + 0x2c) = fStack_104;
        *(float *)(plStack_130 + 6) =
             (((float)uVar37 - (fVar27 * extraout_XMM0_Da_07 + fVar25 * fVar23)) * fVar35 +
             ((float)uVar36 - (fVar29 * extraout_XMM0_Da_07 + fVar34 * fVar25)) * fVar24) / fVar30;
        *(float *)((longlong)plStack_130 + 0x34) = fVar21;
        *(undefined4 *)(plStack_130 + 8) = 0;
        *(undefined4 *)((longlong)plStack_130 + 0x4c) = uVar9;
        goto code_r0x00014127ea54;
      }
    }
    goto code_r0x00014127eaf8;
  }
  uVar1 = *(undefined8 *)(in_stack_00000028 + 6);
  pfStack_220 = (float *)CONCAT44(pfStack_220._4_4_,(undefined4)local_138);
  plStack_130 = in_R8;
  country_core::resources::walls::public_walls::PublicWalls::get();
  if (extraout_RAX == 0) {
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
  local_218 = (undefined **)in_stack_00000038;
  uStack_210 = (code *)in_stack_00000060;
  uStack_208 = (ulonglong *)in_stack_00000068;
  fStack_1a8 = extraout_EAX_00;
  derive_decorator_subtype::derive_decorator_subtype();
  uVar3 = *(undefined1 *)((longlong)in_stack_00000038 + 0x44);
  country_core::resources::walls::decorator::DecoratorRank::new();
  uVar14 = extraout_AL;
  uStack_f2 = uVar3;
  country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
  uStack_128 = extraout_AL_00;
  uStack_127 = uVar14;
  std::collections::hash::map::HashMap::
  get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
            ();
  if (extraout_RAX_00 == 0) {
code_r0x00014127ecaf:
    local_188 = (float **)&uStack_128;
    uStack_180._0_4_ = 10.363022;
    uStack_180._4_4_ = 1.4013e-45;
    local_218 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
    uStack_210 = (code *)0x1;
    puStack_1f8 = (undefined1 *)0x0;
    uStack_208 = &local_188;
    pcStack_200 = (code *)0x1;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  fVar21 = *(float *)(extraout_RAX_00 + 0x30);
  fVar35 = *(float *)(extraout_RAX_00 + 0x3c);
  fVar25 = *(float *)(extraout_RAX_00 + 0x38);
  auStack_248 = auVar32;
  auStack_238 = auVar33;
  ::utils::curve::Curve::
  get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  fVar27 = __real_3f800000;
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
            ();
  _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
            ();
  uVar17 = *(uint *)(extraout_RAX + 600);
  local_138 = extraout_RAX_01;
  if (in_stack_00000070 != 0) {
    local_188._0_4_ = 0.0;
    local_188._4_4_ = 0.0;
    uStack_180._0_4_ = 5.60519e-45;
    uStack_180._4_4_ = 0.0;
    uStack_178._0_4_ = 0.0;
    uStack_178._4_4_ = 0.0;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators();
    if (extraout_RAX_02 != 0) {
      uStack_128 = (undefined1)extraout_XMM0_Da_00;
      uStack_127 = (undefined1)((uint)extraout_XMM0_Da_00 >> 8);
      uStack_126 = (undefined2)((uint)extraout_XMM0_Da_00 >> 0x10);
      fStack_124 = fVar22;
      fStack_120 = fVar21;
      fStack_11c = fVar35 - fVar25;
      collision_detection::
      intersect_wall_decorators<system_decorator::ui_systems::decorator_interaction_intent::calculate_decorator_wall_interaction_intent::closure_env_0>
                ();
    }
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators();
    if (extraout_RAX_03 == 0) {
      if (CONCAT44(local_188._4_4_,(float)local_188) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      merge_proposal_wall_decorators::find_best_wall_decorator_merge_position();
      if (CONCAT44(local_188._4_4_,(float)local_188) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (cStack_1b9 != '\x02') {
        fStack_104 = (float)CONCAT31(fStack_104._1_3_,(undefined1)uStack_208);
        fStack_228 = uStack_208._3_4_;
        fStack_224 = (float)CONCAT31(fStack_224._1_3_,uStack_208._7_1_);
        uVar13 = (ulonglong)CONCAT14(uStack_1bc,uStack_1b8);
        uVar36 = 1;
        uVar20 = uStack_210;
        uVar37 = (int)local_218;
        bVar19 = uStack_208._2_1_;
        bVar15 = uStack_208._1_1_;
        fVar22 = local_218._4_4_;
        goto code_r0x00014127e7f7;
      }
    }
  }
  uVar37 = extraout_XMM0_Da_00;
  if (*(int *)(extraout_RAX + 600) == 1) {
    uStack_178._0_4_ = (float)*(undefined8 *)(extraout_RAX + 0x26c);
    uStack_178._4_4_ = (float)((ulonglong)*(undefined8 *)(extraout_RAX + 0x26c) >> 0x20);
    local_188._0_4_ = (float)*(undefined8 *)(extraout_RAX + 0x25c);
    local_188._4_4_ = (float)((ulonglong)*(undefined8 *)(extraout_RAX + 0x25c) >> 0x20);
    uStack_180._0_4_ = (float)*(undefined8 *)(extraout_RAX + 0x264);
    uStack_180._4_4_ = (float)((ulonglong)*(undefined8 *)(extraout_RAX + 0x264) >> 0x20);
    uVar3 = *(undefined1 *)((longlong)in_stack_00000038 + 0x46);
    ::utils::geometry::rectangle::Rectangle2d::get_point_u();
    uStack_128 = SUB41(fStack_1a8,0);
    uStack_127 = (undefined1)((uint)fStack_1a8 >> 8);
    uStack_126 = (undefined2)((uint)fStack_1a8 >> 0x10);
    ::utils::wall_space::impl$1::project_clamp();
    uStack_f1 = uVar3;
    country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner();
    if (extraout_AL_06 == '\0') {
code_r0x00014127e2f1:
      fStack_104 = (float)CONCAT31(fStack_104._1_3_,extraout_AL_01);
      ::utils::geometry::rectangle::Rectangle2d::get_point_u();
      uStack_128 = SUB41(fStack_1a8,0);
      uStack_127 = (undefined1)((uint)fStack_1a8 >> 8);
      uStack_126 = (undefined2)((uint)fStack_1a8 >> 0x10);
      ::utils::wall_space::impl$1::project_clamp();
      uStack_f1 = uVar3;
      country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner();
      if (extraout_AL_09 != '\0') {
        country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
        fVar35 = __real_3e4ccccd;
        if (extraout_AL_10 == '\0') {
          fVar35 = __real_3eeb851f;
        }
        country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
        uStack_128 = (undefined1)extraout_EAX_06;
        uStack_127 = (undefined1)((uint)extraout_EAX_06 >> 8);
        uStack_126 = (undefined2)((uint)extraout_EAX_06 >> 0x10);
        fStack_124 = extraout_XMM0_Da_13;
        ::utils::wall_space::impl$1::subtract();
        uVar36 = extraout_XMM0_Da_12;
        if ((float)(extraout_XMM0_Da_14 & ___xmm_7fffffff7fffffff7fffffff7fffffff) <
            __real_3f000000 * fVar21 + fVar35) goto code_r0x00014127e5c9;
      }
      ::utils::geometry::rectangle::Rectangle2d::get_point_u();
      uStack_128 = SUB41(fStack_1a8,0);
      uStack_127 = (undefined1)((uint)fStack_1a8 >> 8);
      uStack_126 = (undefined2)((uint)fStack_1a8 >> 0x10);
      ::utils::wall_space::impl$1::project_clamp();
      uStack_f1 = uVar3;
      country_core::resources::walls::decorator::DecoratorType::should_offer_snap_to_corner();
      if (extraout_AL_11 != '\0') {
        country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
        fVar35 = __real_3e4ccccd;
        if (extraout_AL_12 == '\0') {
          fVar35 = __real_3eeb851f;
        }
        country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
        uStack_128 = (undefined1)extraout_EAX_07;
        uStack_127 = (undefined1)((uint)extraout_EAX_07 >> 8);
        uStack_126 = (undefined2)((uint)extraout_EAX_07 >> 0x10);
        fStack_124 = extraout_XMM0_Da_16;
        ::utils::wall_space::impl$1::subtract();
        uVar36 = extraout_XMM0_Da_15;
        if ((float)(extraout_XMM0_Da_17 & ___xmm_7fffffff7fffffff7fffffff7fffffff) <
            __real_3f000000 * fVar21 + fVar35) goto code_r0x00014127e5c9;
      }
      ::utils::geometry::rectangle::Rectangle2d::get_point_u();
      uStack_128 = SUB41(fStack_1a8,0);
      uStack_127 = (undefined1)((uint)fStack_1a8 >> 8);
      uStack_126 = (undefined2)((uint)fStack_1a8 >> 0x10);
      fStack_124 = extraout_XMM0_Da;
      ::utils::wall_space::impl$1::project_clamp();
      cull_oob_decorators::should_snap_to_corner();
      uVar36 = extraout_XMM0_Da_18;
      if (extraout_AL_13 == '\0') goto code_r0x00014127e5e7;
    }
    else {
      country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
      fVar35 = __real_3e4ccccd;
      if (extraout_AL_07 == '\0') {
        fVar35 = __real_3eeb851f;
      }
      country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
      uStack_128 = (undefined1)extraout_EAX_05;
      uStack_127 = (undefined1)((uint)extraout_EAX_05 >> 8);
      uStack_126 = (undefined2)((uint)extraout_EAX_05 >> 0x10);
      fStack_124 = extraout_XMM0_Da_10;
      ::utils::wall_space::impl$1::subtract();
      uVar36 = extraout_XMM0_Da_08;
      if (__real_3f000000 * fVar21 + fVar35 <=
          (float)(extraout_XMM0_Da_11 & ___xmm_7fffffff7fffffff7fffffff7fffffff))
      goto code_r0x00014127e2f1;
    }
code_r0x00014127e5c9:
    country_core::resources::walls::wall_attachment::WallCornerAttachment::from_rectangle_corner_id
              ();
    fStack_104 = (float)CONCAT31(fStack_104._1_3_,extraout_AL_14);
    _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
              ();
    uVar37 = uVar36;
    local_138 = extraout_RAX_12;
  }
  else {
    fStack_104 = (float)CONCAT31(fStack_104._1_3_,extraout_AL_01);
  }
code_r0x00014127e5e7:
  ::utils::curve::Curve::
  get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  ::utils::curve::Curve::
  get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
            ();
  uVar13 = (ulonglong)extraout_EAX_08;
  if (*(ulonglong *)(extraout_RAX + 0x10) <= uVar13) {
code_r0x00014127eaec:
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_bounds_check();
  }
  if (uVar13 + 1 < *(ulonglong *)(extraout_RAX + 0x10)) {
    bVar19 = uVar17 < 2 | in_stack_00000080 ^ 1;
    uVar20 = *(undefined8 *)(*(longlong *)(extraout_RAX + 8) + uVar13 * 8);
    uVar2 = *(undefined8 *)(*(longlong *)(extraout_RAX + 8) + 8 + uVar13 * 8);
    local_188._0_4_ =
         extraout_XMM0_Da_19 * (float)uVar2 + (fVar27 - extraout_XMM0_Da_19) * (float)uVar20;
    local_188._4_4_ =
         extraout_XMM0_Da_19 * (float)((ulonglong)uVar2 >> 0x20) +
         (fVar27 - extraout_XMM0_Da_19) * (float)((ulonglong)uVar20 >> 0x20);
    ::utils::swizzlers::impl$1::xvy();
    uVar36 = 0;
    snap_balcony_door();
    fVar21 = (float)uStack_180;
    bVar4 = uStack_180._4_1_;
    uVar13 = (ulonglong)uStack_180._4_1_;
    if ((byte)(uStack_180._4_1_ - 3) < 2) {
      bVar15 = 0;
      uVar20 = local_138;
    }
    else {
      uVar20 = CONCAT44(local_188._4_4_,(float)local_188);
      country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall();
      if (extraout_RAX_13 == 0) {
        bVar15 = 0;
      }
      else {
        country_core::resources::walls::WallId::is_valid();
        bVar15 = extraout_AL_15 | *(byte *)(extraout_RAX_13 + 0x21);
      }
      bVar15 = (bVar15 | in_stack_00000070) & 1;
      if (bVar15 == 0) {
        fVar21 = fVar22;
        uVar20 = local_138;
      }
      fVar22 = fVar21;
      bVar15 = bVar15 & bVar4 == 1;
      uVar36 = 0;
    }
code_r0x00014127e7f7:
    local_188._0_4_ = fStack_1a8;
    ::utils::wall_space::impl$1::project();
    if (extraout_EAX_09 == 1) {
      *(undefined1 *)((longlong)in_RCX + 0x3f) = fStack_224._0_1_;
      *(float *)((longlong)in_RCX + 0x3b) = fStack_228;
      in_RCX[10] = *(ulonglong *)local_100;
      *(float *)(in_RCX + 0xb) = local_100[2];
      in_RCX[1] = in_stack_00000068;
      in_RCX[2] = 0;
      in_RCX[3] = uVar1;
      *(undefined4 *)(in_RCX + 4) = 1;
      *(undefined4 *)(in_RCX + 5) = uVar37;
      *(float *)((longlong)in_RCX + 0x2c) = fVar22;
      in_RCX[6] = uVar20;
      *(undefined1 *)(in_RCX + 7) = fStack_104._0_1_;
      *(byte *)((longlong)in_RCX + 0x39) = bVar15;
      *(byte *)((longlong)in_RCX + 0x3a) = bVar19;
      *(undefined4 *)(in_RCX + 8) = uVar36;
      *(ulonglong *)((longlong)in_RCX + 0x44) = uVar13;
      *(undefined4 *)((longlong)in_RCX + 0x4c) = pfStack_220._0_4_;
      *(undefined1 *)((longlong)in_RCX + 0x5c) = uStack_f2;
      *in_RCX = in_stack_00000060;
      return;
    }
    *(float *)(in_RCX + 5) = local_100[2];
    in_RCX[4] = *(ulonglong *)local_100;
    *(float *)(in_RCX + 2) = fVar24;
    *(float *)((longlong)in_RCX + 0x14) = fVar34;
    *(float *)(in_RCX + 3) = fVar23;
    *(float *)((longlong)in_RCX + 0x1c) = fVar29;
    *in_RCX = 4;
    return;
  }
code_r0x00014127eaf8:
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_bounds_check();
}

