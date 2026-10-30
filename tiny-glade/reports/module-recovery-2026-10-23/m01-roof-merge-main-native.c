
void __cdecl
system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position(void)

{
  char *pcVar1;
  int iVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  undefined8 uVar10;
  longlong *plVar11;
  longlong *plVar12;
  longlong *plVar13;
  longlong *plVar14;
  longlong *plVar15;
  longlong *plVar16;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong extraout_RAX_04;
  float *extraout_RAX_05;
  longlong *plVar17;
  longlong **pplVar18;
  int *in_RCX;
  longlong *in_RDX;
  ulonglong uVar19;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  byte bVar23;
  ushort uVar24;
  int iVar25;
  uint uVar26;
  byte bVar27;
  char cVar28;
  char cVar32;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  char cVar30;
  char cVar31;
  undefined1 auVar29 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  float fVar35;
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  float fVar36;
  undefined8 *in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  longlong in_stack_00000048;
  undefined8 in_stack_00000050;
  undefined1 local_41d [2];
  undefined1 local_41b;
  undefined2 local_41a;
  undefined1 *local_418;
  code *pcStack_410;
  undefined8 local_408;
  longlong *local_3f8;
  int local_3ec;
  undefined8 *local_3e8;
  longlong local_3e0;
  longlong local_3d8;
  ulonglong local_3d0;
  int *local_3c8;
  undefined8 local_3c0;
  ulonglong local_3b8;
  undefined8 local_3b0;
  longlong *local_3a8;
  longlong lStack_3a0;
  longlong local_398;
  longlong lStack_390;
  longlong local_388;
  longlong lStack_380;
  longlong local_378;
  longlong lStack_370;
  longlong local_368;
  longlong lStack_360;
  longlong lStack_358;
  undefined4 uStack_350;
  undefined4 local_34c;
  undefined4 uStack_348;
  undefined8 uStack_344;
  longlong *local_338;
  longlong lStack_330;
  longlong local_328;
  longlong lStack_320;
  longlong local_318;
  longlong lStack_310;
  longlong local_308;
  longlong lStack_300;
  longlong local_2f8;
  longlong lStack_2f0;
  longlong local_2e8;
  undefined4 uStack_2e0;
  undefined4 uStack_2dc;
  undefined4 uStack_2d8;
  undefined8 uStack_2d4;
  undefined8 *local_2c0;
  longlong local_2b8;
  longlong *local_2b0;
  longlong local_2a8;
  longlong local_2a0;
  longlong local_298;
  undefined8 uStack_290;
  longlong *local_288;
  longlong local_280;
  undefined8 *local_278;
  undefined8 *local_270;
  longlong local_268;
  longlong lStack_260;
  undefined4 local_258;
  int local_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  undefined8 uStack_238;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_228;
  undefined4 local_224;
  undefined4 uStack_220;
  longlong lStack_21c;
  longlong local_214;
  longlong lStack_20c;
  longlong local_204;
  longlong lStack_1fc;
  longlong local_1f4;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined8 uStack_1e0;
  undefined4 local_1d4;
  float fStack_1d0;
  float fStack_1cc;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  longlong *plStack_1b8;
  longlong lStack_1b0;
  longlong local_1a8;
  longlong lStack_1a0;
  longlong local_198;
  longlong lStack_190;
  longlong lStack_188;
  longlong lStack_180;
  undefined4 local_178;
  undefined1 uStack_174;
  undefined2 uStack_173;
  undefined1 uStack_171;
  int iStack_170;
  float fStack_16c;
  longlong *local_168 [5];
  longlong lStack_140;
  longlong local_138;
  longlong lStack_130;
  longlong local_128;
  longlong lStack_120;
  longlong local_118;
  undefined8 uStack_104;
  undefined8 local_f0;
  undefined8 uStack_e8;
  longlong *local_e0;
  longlong lStack_d8;
  longlong local_d0;
  longlong lStack_c8;
  longlong local_c0;
  
  lVar22 = in_stack_00000038 + in_stack_00000040 * 4;
  local_338 = &local_298;
  local_298 = in_stack_00000048;
  uStack_290 = in_stack_00000050;
  local_280 = in_stack_00000030;
  local_278 = in_stack_00000028;
  local_270 = &local_3b0;
  local_2a0 = lVar22;
  local_288 = in_RDX;
  if (in_stack_00000040 != 0) {
    do {
      in_stack_00000038 = in_stack_00000038 + 4;
      core::ops::function::impls::impl$3::
      call_mut<tuple_<ref_<system_decorator::collision_detection::DecoratorIntersection>_>,system_decorator::merge_proposal_roof_decorators::find_best_roof_decorator_merge_position::closure_env_0>
                ();
      if (local_248 != 4) {
        local_2a8 = in_stack_00000038;
        local_3a8 = (longlong *)CONCAT44(uStack_240,uStack_244);
        lStack_3a0 = CONCAT44((undefined4)uStack_238,uStack_23c);
        local_398 = CONCAT44(uStack_230,uStack_238._4_4_);
        lStack_390 = CONCAT44(uStack_228,uStack_22c);
        local_388 = CONCAT44(uStack_220,local_224);
        lStack_140 = lStack_21c;
        local_138 = local_214;
        lStack_130 = lStack_20c;
        local_128 = local_204;
        lStack_120 = lStack_1fc;
        local_118 = local_1f4;
        uStack_104 = uStack_1e0;
        local_3c8 = in_RCX;
        if (in_stack_00000038 != local_2a0) {
          local_3c0 = uStack_290;
          local_2b8 = local_280;
          local_3e8 = local_278;
          local_2c0 = local_270;
          uStack_348 = uStack_1e4;
          uStack_344 = uStack_1e0;
          lStack_358 = local_1f4;
          uStack_350 = uStack_1ec;
          local_34c = uStack_1e8;
          local_368 = local_204;
          lStack_360 = lStack_1fc;
          local_378 = local_214;
          lStack_370 = lStack_20c;
          lStack_380 = lStack_21c;
          local_3d0 = (ulonglong)(local_2a0 - in_stack_00000038) >> 2;
          local_3e0 = local_298 + 0x50;
          local_3f8 = local_288;
          local_2b0 = local_288 + 6;
          iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                                   CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                         CONCAT13(unaff_XMM6_Bd,
                                  CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
          iVar7 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                                   CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                         CONCAT13(unaff_XMM6_Bh,
                                  CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
          iVar8 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                                   CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                         CONCAT13(unaff_XMM6_Bl,
                                  CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
          iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                                   CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                         CONCAT13(unaff_XMM6_Bp,
                                  CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
          local_3b8 = 0;
          local_3d8 = local_298;
          iVar25 = local_248;
LAB_142001187:
          lVar22 = local_3d8;
          iVar2 = *(int *)(in_stack_00000038 + local_3b8 * 4);
          local_3ec = iVar2;
          if (*(longlong *)(local_3d8 + 0x48) != 0) {
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                      ();
            lVar3 = *(longlong *)(lVar22 + 0x30);
            uVar20 = *(ulonglong *)(lVar22 + 0x38);
            bVar27 = (byte)(extraout_RAX >> 0x39);
            auVar29 = ZEXT216(CONCAT11(bVar27,bVar27));
            auVar29 = pshuflw(auVar29,auVar29,0);
            lVar22 = 0;
            uVar19 = extraout_RAX;
            do {
              uVar19 = uVar20 & uVar19;
              pcVar1 = (char *)(lVar3 + uVar19);
              cVar28 = auVar29[0];
              auVar34[0] = -(*pcVar1 == cVar28);
              cVar30 = auVar29[1];
              auVar34[1] = -(pcVar1[1] == cVar30);
              cVar31 = auVar29[2];
              auVar34[2] = -(pcVar1[2] == cVar31);
              cVar32 = auVar29[3];
              auVar34[3] = -(pcVar1[3] == cVar32);
              auVar34[4] = -(pcVar1[4] == cVar28);
              auVar34[5] = -(pcVar1[5] == cVar30);
              auVar34[6] = -(pcVar1[6] == cVar31);
              auVar34[7] = -(pcVar1[7] == cVar32);
              auVar34[8] = -(pcVar1[8] == cVar28);
              auVar34[9] = -(pcVar1[9] == cVar30);
              auVar34[10] = -(pcVar1[10] == cVar31);
              auVar34[0xb] = -(pcVar1[0xb] == cVar32);
              auVar34[0xc] = -(pcVar1[0xc] == cVar28);
              auVar34[0xd] = -(pcVar1[0xd] == cVar30);
              auVar34[0xe] = -(pcVar1[0xe] == cVar31);
              auVar34[0xf] = -(pcVar1[0xf] == cVar32);
              uVar24 = (ushort)(SUB161(auVar34 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
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
                       (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar34[0xf] >> 7) << 0xf;
              uVar26 = (uint)uVar24;
              while (uVar24 != 0) {
                uVar4 = 0;
                for (uVar5 = uVar26; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
                  uVar4 = uVar4 + 1;
                }
                lVar21 = (uVar4 + uVar19 & uVar20) * -0x50;
                if (iVar2 == *(int *)(lVar3 + -0x50 + lVar21)) {
                  country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype()
                  ;
                  country_core::resources::walls::decorator::DecoratorRank::new();
                  country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
                  std::collections::hash::map::HashMap::
                  get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                            ();
                  if (extraout_RAX_00 == 0) {
LAB_142001809:
                    local_418 = local_41d;
                    pcStack_410 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
                    local_248 = 0x42c21c80;
                    uStack_244 = 1;
                    uStack_240 = 1;
                    uStack_23c = 0;
                    uStack_228 = 0;
                    local_224 = 0;
                    uStack_238 = &local_418;
                    uStack_230 = 1;
                    uStack_22c = 0;
                    // WARNING: Subroutine does not return
                    core::panicking::panic_fmt();
                  }
                  lVar3 = lVar3 + lVar21;
                  bVar27 = *(char *)((longlong)local_3f8 + 0x46) - 0x38;
                  if (7 < bVar27) {
                    bVar27 = 6;
                  }
                  cVar28 = *(char *)(lVar3 + -2);
                  bVar23 = cVar28 - 0x38;
                  if (7 < bVar23) {
                    bVar23 = 6;
                  }
                  local_248 = iVar25;
                  if ((bVar27 == 6 && cVar28 != *(char *)((longlong)local_3f8 + 0x46)) ||
                     (bVar23 != bVar27)) {
LAB_142001300:
                    local_338 = local_3a8;
                    lStack_330 = lStack_3a0;
                    local_328 = local_398;
                    lStack_320 = lStack_390;
                    local_318 = local_388;
                    lStack_310 = lStack_380;
                    local_308 = local_378;
                    lStack_300 = lStack_370;
                    local_2f8 = local_368;
                    lStack_2f0 = lStack_360;
                    local_2e8 = lStack_358;
                    uStack_2e0 = uStack_350;
                    uStack_2dc = local_34c;
                    uStack_2d8 = uStack_348;
                    uStack_2d4 = uStack_344;
                  }
                  else {
                    fVar36 = *(float *)(extraout_RAX_00 + 0x30);
                    uVar20 = (ulonglong)*(byte *)(lVar3 + -4) +
                             (ulonglong)*(byte *)((longlong)local_3f8 + 0x44);
                    country_core::resources::walls::decorator::DecoratorType::max_rank();
                    if (extraout_RAX_01 < uVar20) goto LAB_142001300;
                    local_318 = local_3f8[4];
                    local_338 = (longlong *)*local_3f8;
                    lStack_330 = local_3f8[1];
                    local_328 = local_3f8[2];
                    lStack_320 = local_3f8[3];
                    local_41a = *(undefined2 *)((longlong)local_3f8 + 0x45);
                    local_41b = *(undefined1 *)((longlong)local_3f8 + 0x47);
                    lVar22 = local_3f8[5];
                    local_258 = (undefined4)local_2b0[2];
                    local_268 = *local_2b0;
                    lStack_260 = local_2b0[1];
                    country_core::resources::walls::decorator::DecoratorRank::new();
                    country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
                    std::collections::hash::map::HashMap::
                    get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                              ();
                    if (extraout_RAX_02 == 0) goto LAB_142001809;
                    fVar36 = (*(float *)(extraout_RAX_02 + 0x30) - fVar36) * 0.5;
                    if ((*(byte *)(local_2b8 + 8) & 1) == 0) {
                      pcStack_410 = (code *)CONCAT44(pcStack_410._4_4_,
                                                     *(undefined4 *)(local_3e8 + 1));
                      local_418 = (undefined1 *)*local_3e8;
                      country_core::resources::roofs::roof_shape::Roof::
                      get_roof_shape_from_pos_ws_clamped();
                      country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
                      fVar36 = fVar36 / extraout_XMM0_Da_00;
                    }
                    else {
                      pcStack_410 = (code *)CONCAT44(pcStack_410._4_4_,
                                                     *(undefined4 *)(local_3e8 + 1));
                      local_418 = (undefined1 *)*local_3e8;
                      country_core::resources::roofs::roof_shape::Roof::
                      get_roof_shape_from_pos_ws_clamped();
                      country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
                      local_418 = (undefined1 *)*extraout_RAX_03;
                      pcStack_410 = (code *)extraout_RAX_03[1];
                      local_408 = extraout_RAX_03[2];
                      ::utils::geometry::rectangle::Rectangle2d::get_closest_edge_id();
                      ::utils::geometry::rectangle::Rectangle2d::edge_length();
                      fVar36 = (fVar36 / extraout_XMM0_Da) * 0.25;
                    }
                    country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof();
                    country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof();
                    fVar35 = (float)*local_2c0;
                    uVar26 = -(uint)(*extraout_RAX_05 < fVar35);
                    fStack_1d0 = (float)(~uVar26 & (uint)-fVar36 | uVar26 & (uint)fVar36) +
                                 *extraout_RAX_05;
                    local_178 = local_258;
                    lStack_188 = local_268;
                    lStack_180 = lStack_260;
                    uStack_1e4 = uStack_348;
                    uStack_1e0 = uStack_344;
                    uVar10 = uStack_1e0;
                    local_1f4 = lStack_358;
                    uStack_1ec = uStack_350;
                    uStack_1e8 = local_34c;
                    local_204 = local_368;
                    lStack_1fc = lStack_360;
                    local_214 = local_378;
                    lStack_20c = lStack_370;
                    local_224 = (undefined4)local_388;
                    uStack_220 = (undefined4)((ulonglong)local_388 >> 0x20);
                    lStack_21c = lStack_380;
                    uStack_238._4_4_ = (undefined4)local_398;
                    uStack_230 = (undefined4)((ulonglong)local_398 >> 0x20);
                    uStack_22c = (undefined4)lStack_390;
                    uStack_228 = (undefined4)((ulonglong)lStack_390 >> 0x20);
                    uStack_244 = SUB84(local_3a8,0);
                    uStack_240 = (undefined4)((ulonglong)local_3a8 >> 0x20);
                    uStack_23c = (undefined4)lStack_3a0;
                    uStack_238._0_4_ = (undefined4)((ulonglong)lStack_3a0 >> 0x20);
                    local_1d4 = *(undefined4 *)(extraout_RAX_04 + 8);
                    fVar35 = fStack_1d0 - fVar35;
                    fVar36 = extraout_RAX_05[1] - (float)((ulonglong)*local_2c0 >> 0x20);
                    fStack_16c = SQRT(fVar36 * fVar36 + fVar35 * fVar35);
                    local_c0 = local_318;
                    local_d0 = local_328;
                    lStack_c8 = lStack_320;
                    local_e0 = local_338;
                    lStack_d8 = lStack_330;
                    fStack_1cc = extraout_RAX_05[1];
                    local_198 = local_318;
                    local_1a8 = local_328;
                    lStack_1a0 = lStack_320;
                    plStack_1b8 = local_338;
                    lStack_1b0 = lStack_330;
                    uStack_1c8 = local_f0;
                    uStack_1c0 = uStack_e8;
                    lStack_190 = lVar22;
                    uStack_174 = (char)uVar20;
                    uStack_173 = local_41a;
                    uStack_171 = local_41b;
                    iStack_170 = iVar2;
                    uStack_1e0._4_4_ = (float)((ulonglong)uStack_344 >> 0x20);
                    uStack_1e0 = uVar10;
                    if (NAN(fStack_16c) || NAN(uStack_1e0._4_4_)) {
                    // WARNING: Subroutine does not return
                      core::option::unwrap_failed();
                    }
                    plVar17 = (longlong *)&uStack_244;
                    if (fStack_16c < uStack_1e0._4_4_) {
                      plVar17 = (longlong *)&local_1d4;
                      local_248 = 0;
                    }
                    uStack_2dc = (undefined4)*(undefined8 *)((longlong)plVar17 + 0x5c);
                    uStack_2d8 = (undefined4)
                                 ((ulonglong)*(undefined8 *)((longlong)plVar17 + 0x5c) >> 0x20);
                    uStack_2d4 = *(undefined8 *)((longlong)plVar17 + 100);
                    local_2e8 = plVar17[10];
                    uStack_2e0 = (undefined4)plVar17[0xb];
                    uStack_2dc = (undefined4)((ulonglong)plVar17[0xb] >> 0x20);
                    local_2f8 = plVar17[8];
                    lStack_2f0 = plVar17[9];
                    local_308 = plVar17[6];
                    lStack_300 = plVar17[7];
                    local_318 = plVar17[4];
                    lStack_310 = plVar17[5];
                    local_328 = plVar17[2];
                    lStack_320 = plVar17[3];
                    local_338 = (longlong *)*plVar17;
                    lStack_330 = plVar17[1];
                  }
                  local_3a8 = local_338;
                  lStack_3a0 = lStack_330;
                  local_398 = local_328;
                  lStack_390 = lStack_320;
                  local_388 = local_318;
                  lStack_380 = lStack_310;
                  local_378 = local_308;
                  lStack_370 = lStack_300;
                  local_368 = local_2f8;
                  lStack_360 = lStack_2f0;
                  lStack_358 = local_2e8;
                  uStack_350 = uStack_2e0;
                  local_34c = uStack_2dc;
                  uStack_348 = uStack_2d8;
                  uStack_344 = uStack_2d4;
                  local_3b8 = local_3b8 + 1;
                  iVar25 = local_248;
                  if (local_3b8 != local_3d0) goto LAB_142001187;
                  pplVar18 = &local_3a8;
                  goto LAB_1420017bf;
                }
                uVar24 = (ushort)(uVar26 - 1) & (ushort)uVar26;
                uVar26 = CONCAT22((short)(uVar26 - 1 >> 0x10),uVar24);
              }
              auVar33[0] = -(*pcVar1 == (char)iVar6);
              auVar33[1] = -(pcVar1[1] == (char)((uint)iVar6 >> 8));
              auVar33[2] = -(pcVar1[2] == (char)((uint)iVar6 >> 0x10));
              auVar33[3] = -(pcVar1[3] == (char)((uint)iVar6 >> 0x18));
              auVar33[4] = -(pcVar1[4] == (char)iVar7);
              auVar33[5] = -(pcVar1[5] == (char)((uint)iVar7 >> 8));
              auVar33[6] = -(pcVar1[6] == (char)((uint)iVar7 >> 0x10));
              auVar33[7] = -(pcVar1[7] == (char)((uint)iVar7 >> 0x18));
              auVar33[8] = -(pcVar1[8] == (char)iVar8);
              auVar33[9] = -(pcVar1[9] == (char)((uint)iVar8 >> 8));
              auVar33[10] = -(pcVar1[10] == (char)((uint)iVar8 >> 0x10));
              auVar33[0xb] = -(pcVar1[0xb] == (char)((uint)iVar8 >> 0x18));
              auVar33[0xc] = -(pcVar1[0xc] == (char)iVar9);
              auVar33[0xd] = -(pcVar1[0xd] == (char)((uint)iVar9 >> 8));
              auVar33[0xe] = -(pcVar1[0xe] == (char)((uint)iVar9 >> 0x10));
              auVar33[0xf] = -(pcVar1[0xf] == (char)((uint)iVar9 >> 0x18));
              if ((((((((((((((((SUB161(auVar33 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar33 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar33 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar33 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar33 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar33 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar33 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar33 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar33 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar33 >> 0x4f,0) & 1) != 0) || (SUB161(auVar33 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar33 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar33 >> 0x67,0) & 1) != 0) || (SUB161(auVar33 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar33 >> 0x77,0) & 1) != 0) || auVar33[0xf] < '\0') break;
              uVar19 = uVar19 + lVar22 + 0x10;
              lVar22 = lVar22 + 0x10;
            } while( true );
          }
                    // WARNING: Subroutine does not return
          core::option::expect_failed();
        }
        pplVar18 = local_168;
LAB_1420017bf:
        uVar10 = *(undefined8 *)((longlong)pplVar18 + 100);
        *(undefined8 *)(local_3c8 + 0x18) = *(undefined8 *)((longlong)pplVar18 + 0x5c);
        *(undefined8 *)(local_3c8 + 0x1a) = uVar10;
        plVar17 = pplVar18[0xb];
        *(longlong **)(local_3c8 + 0x15) = pplVar18[10];
        *(longlong **)(local_3c8 + 0x17) = plVar17;
        plVar17 = pplVar18[9];
        *(longlong **)(local_3c8 + 0x11) = pplVar18[8];
        *(longlong **)(local_3c8 + 0x13) = plVar17;
        plVar17 = *pplVar18;
        plVar11 = pplVar18[1];
        plVar12 = pplVar18[2];
        plVar13 = pplVar18[3];
        plVar14 = pplVar18[4];
        plVar15 = pplVar18[5];
        plVar16 = pplVar18[7];
        *(longlong **)(local_3c8 + 0xd) = pplVar18[6];
        *(longlong **)(local_3c8 + 0xf) = plVar16;
        *(longlong **)(local_3c8 + 9) = plVar14;
        *(longlong **)(local_3c8 + 0xb) = plVar15;
        *(longlong **)(local_3c8 + 5) = plVar12;
        *(longlong **)(local_3c8 + 7) = plVar13;
        *(longlong **)(local_3c8 + 1) = plVar17;
        *(longlong **)(local_3c8 + 3) = plVar11;
        in_RCX = local_3c8;
        goto LAB_142000f8b;
      }
    } while (in_stack_00000038 != lVar22);
  }
  local_248 = 4;
LAB_142000f8b:
  *in_RCX = local_248;
  return;
}

