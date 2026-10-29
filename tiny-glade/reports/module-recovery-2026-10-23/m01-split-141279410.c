
void __cdecl
system_decorator::ui_systems::handle_splitting_wall_decorators::
handle_moving_out_wall_decorator_part(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  int iVar4;
  undefined8 uVar5;
  longlong lVar6;
  undefined1 auVar7 [16];
  float fVar8;
  float fVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  undefined8 *puVar13;
  int *piVar14;
  int iVar15;
  longlong *plVar16;
  byte bVar17;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  undefined1 extraout_AL_01;
  undefined1 extraout_AL_02;
  undefined1 extraout_AL_03;
  undefined1 extraout_AL_04;
  undefined1 extraout_AL_05;
  uint uVar18;
  undefined4 extraout_EAX;
  undefined4 extraout_EAX_00;
  ulonglong *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong *plVar19;
  longlong extraout_RAX_04;
  undefined4 *puVar20;
  longlong extraout_RAX_05;
  int *piVar21;
  undefined4 *in_RCX;
  undefined8 **ppuVar22;
  undefined1 uVar23;
  undefined8 *in_RDX;
  ulonglong *puVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  ulonglong *in_R9;
  ulonglong *puVar27;
  undefined8 *puVar28;
  ulonglong uVar29;
  bool bVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  uint uVar35;
  uint uVar36;
  uint uVar37;
  undefined1 unaff_XMM12 [16];
  undefined1 auVar38 [16];
  undefined4 in_stack_00000028;
  undefined8 *in_stack_00000040;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 *local_308;
  undefined8 *puStack_300;
  undefined8 *local_2f8;
  undefined8 *puStack_2f0;
  undefined8 *local_2e8;
  undefined8 *local_2d8;
  undefined8 *puStack_2d0;
  undefined8 *local_2c8;
  undefined8 *puStack_2c0;
  undefined8 *local_2b8;
  undefined4 *local_2b0;
  ulonglong local_2a8;
  undefined1 local_2a0;
  undefined1 uStack_29f;
  undefined6 uStack_29e;
  ulonglong *local_298;
  int *local_290;
  undefined8 *local_288;
  ulonglong local_278;
  ulonglong uStack_270;
  undefined4 local_268;
  int local_264;
  undefined8 local_260;
  undefined8 local_258;
  undefined4 local_250;
  undefined4 uStack_24c;
  undefined4 local_248;
  undefined4 uStack_244;
  undefined8 local_240;
  undefined8 uStack_238;
  ulonglong local_230;
  undefined8 local_228;
  code *local_220;
  undefined8 local_218;
  undefined8 uStack_210;
  ulonglong local_208;
  ulonglong local_200;
  ulonglong local_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined8 *local_1e0;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined8 *local_1d0;
  undefined8 *puStack_1c8;
  ulonglong local_1c0;
  ulonglong uStack_1b8;
  undefined8 *local_1a8;
  undefined8 *local_1a0;
  undefined8 local_198;
  longlong local_190;
  undefined4 *puStack_188;
  longlong local_180;
  undefined8 local_178;
  undefined8 *local_170;
  undefined8 *local_168;
  undefined8 *puStack_160;
  undefined8 *local_158;
  undefined8 *puStack_150;
  undefined8 *local_148;
  undefined8 *puStack_140;
  ulonglong local_138;
  ulonglong uStack_130;
  undefined4 local_128;
  undefined4 local_120;
  undefined1 local_11c;
  char local_11b;
  undefined1 local_11a;
  undefined1 local_118;
  undefined1 local_117;
  undefined1 local_116;
  undefined2 local_114;
  undefined1 local_10c;
  undefined4 local_104;
  undefined8 *local_100;
  ulonglong *local_f8;
  float local_ec;
  undefined4 local_e8;
  char local_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  local_1a8 = (undefined8 *)*in_R9;
  local_1a0 = (undefined8 *)in_R9[1];
  country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info_mut();
  if (extraout_RAX == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_e2 = *(char *)((longlong)extraout_RAX + 0x46);
  local_2e8 = (undefined8 *)extraout_RAX[4];
  local_308 = (undefined8 *)*extraout_RAX;
  puStack_300 = (undefined8 *)extraout_RAX[1];
  local_2f8 = (undefined8 *)extraout_RAX[2];
  puStack_2f0 = (undefined8 *)extraout_RAX[3];
  local_e8 = CONCAT31(local_e8._1_3_,*(undefined1 *)((longlong)extraout_RAX + 0x44));
  local_104 = (undefined4)extraout_RAX[5];
  local_268 = (undefined4)extraout_RAX[8];
  local_278 = extraout_RAX[6];
  uStack_270 = extraout_RAX[7];
  country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall();
  if (extraout_RAX_00 == (undefined8 *)0x0) {
    local_318._0_3_ = CONCAT12(2,(undefined2)local_318);
    local_264 = 0;
  }
  else {
    uVar5 = extraout_RAX_00[4];
    local_338 = *extraout_RAX_00;
    uStack_330 = extraout_RAX_00[1];
    local_328 = extraout_RAX_00[2];
    uStack_320 = extraout_RAX_00[3];
    local_264 = 0;
    local_318._2_1_ = (char)((ulonglong)uVar5 >> 0x10);
    bVar30 = local_318._2_1_ != '\x02';
    local_318 = uVar5;
    if (bVar30) {
      local_264 = 1;
      local_260 = local_338;
    }
  }
  local_230 = 0;
  local_e1 = 1;
  local_2b0 = in_RCX;
  local_100 = in_RDX;
  local_f8 = in_R9;
  country_core::resources::walls::decorator::DecoratorInfo::try_dst_as_wall();
  uVar26 = 0;
  if (extraout_RAX_01 != 0) {
    local_e1 = 1;
    country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached();
    uVar26 = extraout_RAX_02;
    if ((char)extraout_RAX_02 != '\0') {
      if ((byte)local_e8 == '\0') {
        local_e1 = 1;
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      bVar17 = (byte)local_e8 - 1;
      local_250 = 0;
      uStack_244 = CONCAT31(uStack_244._1_3_,bVar17);
      local_230 = 1;
      local_e8 = 0;
      local_100 = (undefined8 *)(ulonglong)bVar17;
      uVar25 = 1;
      goto LAB_141279a38;
    }
  }
  if (local_264 != 0) {
    local_e1 = 1;
    country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
    local_ec = (float)CONCAT31(local_ec._1_3_,extraout_AL);
    local_258 = *in_stack_00000040;
    local_e1 = 1;
    country_core::resources::walls::decorator::DecoratorRank::new();
    local_198 = in_stack_00000040[1];
    local_e1 = 1;
    uVar23 = extraout_AL_00;
    country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
    local_228._0_2_ = CONCAT11(uVar23,extraout_AL_01);
    local_e1 = 1;
    std::collections::hash::map::HashMap::
    get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
              ();
    if (extraout_RAX_03 == 0) {
      local_1f0 = &local_228;
      uStack_1e8 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
      local_178 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
      local_170 = (undefined8 *)0x1;
      local_158 = (undefined8 *)0x0;
      local_168 = &local_1f0;
      puStack_160 = (undefined8 *)0x1;
      local_e1 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    fVar33 = *(float *)(extraout_RAX_03 + 0x3c) - *(float *)(extraout_RAX_03 + 0x38);
    fVar1 = *(float *)(extraout_RAX_03 + 0x30);
    local_2a8 = (ulonglong)(byte)local_e8;
    local_1f0 = local_100;
    uStack_1e8 = (code *)CONCAT44(uStack_1e8._4_4_,0xbf800000);
    local_1e0 = (undefined8 *)(~(ulonglong)local_100 + (ulonglong)(byte)local_e8);
    uStack_1d8 = 0x3f800000;
    uVar18 = (uint)(local_e2 == ':');
    fVar31 = (float)(~((int)(uVar18 << 0x1f) >> 0x1f) & 0x3f800000);
    fVar34 = (float)((int)(uVar18 << 0x1f) >> 0x1f & 0x3f800000);
    uVar18 = (uint)(local_e2 != ':');
    puVar28 = (undefined8 *)0x0;
    uVar35 = (int)(uVar18 * -0x80000000) >> 0x1f;
    uVar36 = (int)(uVar18 * -0x80000000) >> 0x1f;
    uVar37 = (int)(uVar18 * -0x80000000) >> 0x1f;
    do {
      plVar16 = &local_1f0 + (longlong)puVar28 * 2;
      do {
        plVar19 = plVar16;
        if (puVar28 == (undefined8 *)0x2) {
          local_e1 = 1;
          bevy_app::plugin::Plugin::ready<country_core::systems::main_camera::MainCameraPlugin>();
          local_e1 = 1;
          country_core::resources::walls::decorator::DecoratorRank::new();
          local_e1 = 1;
          uVar23 = extraout_AL_04;
          country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
          local_228._0_2_ = CONCAT11(uVar23,extraout_AL_05);
          local_e1 = 1;
          std::collections::hash::map::HashMap::
          get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                    ();
          if (extraout_RAX_05 == 0) {
            uStack_1e8 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
            local_178 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
            local_170 = (undefined8 *)0x1;
            local_158 = (undefined8 *)0x0;
            local_168 = &local_1f0;
            puStack_160 = (undefined8 *)0x1;
            local_e1 = 1;
            local_1f0 = &local_228;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          if ((byte)local_e8 == '\0') {
            local_e1 = 1;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          uVar26 = local_2a8 - 1;
          if ((longlong)uVar26 < 0) {
            fVar32 = (float)uVar26;
          }
          else {
            fVar32 = (float)(int)uVar26;
          }
          fVar32 = (float)local_100 / fVar32 + (float)local_100 / fVar32 + -1.0;
          local_ec = (fVar1 - *(float *)(extraout_RAX_05 + 0x30)) * 0.5 * fVar31 * fVar32;
          local_100 = (undefined8 *)
                      (ulonglong)
                      (uint)(fVar32 * fVar34 * (fVar33 - (*(float *)(extraout_RAX_05 + 0x3c) -
                                                         *(float *)(extraout_RAX_05 + 0x38))) * 0.5)
          ;
          local_e8 = 1;
          uVar25 = local_230;
          goto LAB_141279a38;
        }
        puVar28 = (undefined8 *)((longlong)puVar28 + 1);
        lVar6 = *plVar19;
        plVar16 = plVar19 + 2;
      } while (lVar6 == 0);
      local_1d0 = puVar28;
      fVar32 = *(float *)(plVar19 + 1);
      local_e1 = 1;
      country_core::resources::walls::decorator::DecoratorRank::new();
      local_e1 = 1;
      uVar23 = extraout_AL_02;
      country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
      _local_2a0 = CONCAT11(uVar23,extraout_AL_03);
      local_e1 = 1;
      std::collections::hash::map::HashMap::
      get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                ();
      if (extraout_RAX_04 == 0) {
        local_228 = (int *)&local_2a0;
        local_220 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
        local_178 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
        local_170 = (undefined8 *)0x1;
        local_158 = (undefined8 *)0x0;
        local_168 = &local_228;
        puStack_160 = (undefined8 *)0x1;
        local_e1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      fVar8 = *(float *)(extraout_RAX_04 + 0x30);
      fVar9 = *(float *)(extraout_RAX_04 + 0x38);
      fVar2 = *(float *)(extraout_RAX_04 + 0x3c);
      fVar3 = *(float *)(extraout_RAX_04 + 0x38);
      bVar30 = 2 < local_230;
      puVar20 = (undefined4 *)CONCAT44(uStack_24c,local_250);
      if (!bVar30) {
        puVar20 = &local_250;
      }
      puVar27 = &local_230;
      if (bVar30) {
        puVar27 = (ulonglong *)&local_248;
      }
      puVar24 = (ulonglong *)&local_248;
      uVar26 = local_230;
      if (!bVar30) {
        puVar24 = &local_230;
        uVar26 = 2;
      }
      uVar25 = *puVar24;
      if (uVar25 == uVar26) {
        local_e1 = 1;
        smallvec::SmallVec::
        reserve_one_unchecked<array_<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>_>
                  ();
        puVar20 = (undefined4 *)CONCAT44(uStack_24c,local_250);
        uVar25 = CONCAT44(uStack_244,local_248);
        puVar27 = (ulonglong *)&local_248;
      }
      auVar7._4_4_ = uVar36;
      auVar7._0_4_ = uVar35;
      auVar7._8_4_ = uVar37;
      auVar7._12_4_ = 0;
      auVar38._0_4_ = (uint)(fVar32 * (fVar1 - fVar8) * 0.5 * fVar31) & uVar35;
      auVar38._4_4_ = (uint)(fVar32 * (fVar33 - (fVar2 - fVar3)) * 0.5 * fVar34) & uVar36;
      auVar38._8_4_ = (uint)((0.0 - fVar9) * 0.0 * 0.0 * 0.0) & uVar37;
      auVar38._12_4_ = 0;
      unaff_XMM12 = auVar38 | ~auVar7 & unaff_XMM12;
      puVar20[uVar25 * 4] = uVar18;
      *(longlong *)(puVar20 + uVar25 * 4 + 1) = unaff_XMM12._0_8_;
      *(char *)(puVar20 + uVar25 * 4 + 3) = (char)lVar6;
      *puVar27 = *puVar27 + 1;
    } while( true );
  }
  uVar25 = 0;
  local_100 = (undefined8 *)uVar26;
  local_e8 = 0;
LAB_141279a38:
  local_298 = local_f8;
  local_290 = &local_264;
  local_288 = &local_338;
  piVar14 = (int *)CONCAT44(uStack_24c,local_250);
  local_218 = local_240;
  uStack_210 = uStack_238;
  uVar29 = 0;
  local_220 = (code *)CONCAT44(uStack_244,local_248);
  uVar26 = uVar25;
  if (2 < uVar25) {
    local_220 = (code *)uVar29;
    uVar26 = CONCAT44(uStack_244,local_248);
  }
  local_208 = 0;
  if (2 < uVar25) {
    local_208 = uVar25;
  }
  local_228 = piVar14;
  local_200 = 0;
  local_1f8 = uVar26;
  local_190 = 0;
  puStack_188 = (undefined4 *)&DAT_00000004;
  local_180 = 0;
  if (uVar26 != 0) {
    local_200 = 1;
    piVar21 = (int *)&local_228;
    if (2 < uVar25) {
      piVar21 = piVar14;
    }
    iVar4 = *piVar21;
    if (iVar4 != 2) {
      uVar5 = *(undefined8 *)(piVar21 + 1);
      local_f8 = (ulonglong *)CONCAT71(local_f8._1_7_,(char)piVar21[3]);
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      *puStack_188 = in_stack_00000028;
      local_180 = 1;
      uVar23 = *(undefined1 *)((longlong)extraout_RAX + 0x46);
      local_178 = (undefined **)CONCAT44(iVar4,in_stack_00000028);
      local_170 = (undefined8 *)uVar5;
      handle_moving_out_wall_decorator_part::closure_3();
      puStack_150 = (undefined8 *)CONCAT44(uStack_1d4,uStack_1d8);
      local_168 = local_1f0;
      puStack_160 = (undefined8 *)uStack_1e8;
      local_158 = local_1e0;
      local_148 = local_1d0;
      puStack_140 = puStack_1c8;
      local_138 = local_1c0;
      uStack_130 = uStack_1b8;
      local_178 = (undefined **)local_1a8;
      local_170 = local_1a0;
      local_128 = 0;
      local_120 = 0;
      local_118 = 1;
      local_117 = local_f8._0_1_;
      local_116 = 0;
      local_114 = 0x4002;
      uStack_1e8._0_5_ = CONCAT14(2,(undefined4)uStack_1e8);
      local_10c = uVar23;
      apply_decorator_op::apply_decorator_op();
    }
    uVar29 = 1;
    if (uVar26 != 1) {
      local_200 = 2;
      piVar21 = (int *)&local_228;
      if (2 < uVar25) {
        piVar21 = piVar14;
      }
      iVar4 = piVar21[4];
      if (iVar4 != 2) {
        local_f8 = *(ulonglong **)(piVar21 + 5);
        iVar15 = piVar21[7];
        country_core::resources::walls::decorator::TransientDecoratorIdGen::generate();
        lVar6 = local_180;
        if (local_180 == local_190) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
        }
        puStack_188[lVar6] = extraout_EAX;
        local_180 = lVar6 + 1;
        local_1f0 = (undefined8 *)CONCAT44(iVar4,extraout_EAX);
        uStack_1e8 = (code *)local_f8;
        handle_moving_out_wall_decorator_part::closure_3();
        if ((int)local_178 == 3) {
          local_2b8 = local_2e8;
          local_2d8 = local_308;
          puStack_2d0 = puStack_300;
          local_2c8 = local_2f8;
          puStack_2c0 = puStack_2f0;
        }
        else {
          local_2b8 = local_148;
          local_2d8 = local_168;
          puStack_2d0 = puStack_160;
          local_2c8 = local_158;
          puStack_2c0 = puStack_150;
        }
        local_138 = local_278;
        uStack_130 = uStack_270;
        local_128 = local_268;
        local_170 = local_1a8;
        local_168 = local_1a0;
        puStack_140 = local_2b8;
        local_120 = local_104;
        local_11c = 1;
        local_11b = local_e2;
        local_178 = (undefined **)0x3;
        uStack_1e8._0_5_ = CONCAT14(2,(undefined4)uStack_1e8);
        puStack_160 = local_2d8;
        local_158 = puStack_2d0;
        puStack_150 = local_2c8;
        local_148 = puStack_2c0;
        local_11a = (char)iVar15;
        apply_decorator_op::apply_decorator_op();
      }
      uVar29 = 2;
      if (uVar26 != 2) {
        piVar21 = (int *)&local_228;
        if (2 < uVar25) {
          piVar21 = piVar14;
        }
        local_200 = 3;
        uVar29 = 3;
        if (piVar21[8] != 2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
    }
  }
  country_core::resources::walls::decorator::TransientDecoratorIdGen::generate();
  uStack_1e8 = (code *)((ulonglong)(uint)local_ec | (longlong)local_100 << 0x20);
  local_1f0 = (undefined8 *)CONCAT44(local_e8,extraout_EAX_00);
  handle_moving_out_wall_decorator_part::closure_3();
  ppuVar22 = &local_308;
  if ((int)local_178 != 3) {
    ppuVar22 = &local_168;
  }
  puVar10 = *ppuVar22;
  puVar11 = ppuVar22[1];
  puVar12 = ppuVar22[2];
  puVar13 = ppuVar22[3];
  puVar28 = ppuVar22[4];
  bevy_app::plugin::Plugin::ready<country_core::systems::main_camera::MainCameraPlugin>();
  local_138 = local_278;
  uStack_130 = uStack_270;
  local_128 = local_268;
  local_170 = local_1a8;
  local_168 = local_1a0;
  local_120 = local_104;
  local_11c = 1;
  local_11b = local_e2;
  local_178 = (undefined **)0x3;
  uStack_1e8._0_5_ = CONCAT14(2,(undefined4)uStack_1e8);
  puStack_160 = puVar10;
  local_158 = puVar11;
  puStack_150 = puVar12;
  local_148 = puVar13;
  puStack_140 = puVar28;
  apply_decorator_op::apply_decorator_op();
  *(longlong *)(local_2b0 + 6) = local_180;
  *(longlong *)(local_2b0 + 2) = local_190;
  *(undefined4 **)(local_2b0 + 4) = puStack_188;
  *local_2b0 = extraout_EAX_00;
  piVar21 = (int *)&local_228;
  if (2 < uVar25) {
    piVar21 = piVar14;
  }
  piVar21 = (int *)((ulonglong)(uint)((int)uVar29 << 4) + (longlong)piVar21);
  do {
    uVar29 = uVar29 + 1;
    if (uVar29 - uVar26 == 1) break;
    local_200 = uVar29;
    iVar4 = *piVar21;
    piVar21 = piVar21 + 4;
  } while (iVar4 != 2);
  if (2 < uVar25) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


