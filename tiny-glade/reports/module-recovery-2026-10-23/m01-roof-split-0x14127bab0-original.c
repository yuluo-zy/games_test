
void __cdecl
system_decorator::ui_systems::handle_splitting_roof_decorators::
handle_moving_out_roof_decorator_part(void)

{
  longlong lVar1;
  float fVar2;
  float fVar3;
  undefined1 uVar4;
  int iVar5;
  undefined **ppuVar6;
  ulonglong uVar7;
  char extraout_AL;
  byte bVar8;
  undefined1 extraout_AL_00;
  undefined1 extraout_AL_01;
  uint uVar9;
  undefined4 extraout_EAX;
  undefined4 extraout_EAX_00;
  longlong *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar10;
  int ****ppppiVar11;
  int *piVar12;
  longlong *in_RCX;
  ulonglong *puVar13;
  longlong *plVar14;
  undefined1 uVar15;
  undefined **in_RDX;
  ulonglong *puVar16;
  ulonglong uVar17;
  ulonglong *in_R9;
  ulonglong uVar18;
  bool bVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  undefined4 in_stack_00000028;
  longlong in_stack_00000030;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  longlong local_2f8;
  undefined **local_2e8;
  undefined8 uStack_2e0;
  longlong local_2d8;
  undefined8 uStack_2d0;
  longlong local_2c8;
  longlong lStack_2c0;
  longlong local_2b8;
  longlong lStack_2b0;
  undefined8 *local_2a0;
  undefined8 *local_298;
  ulonglong *local_290;
  longlong local_288;
  longlong lStack_280;
  longlong local_278;
  longlong lStack_270;
  longlong local_268;
  undefined4 local_258;
  undefined4 uStack_254;
  undefined4 uStack_250;
  undefined4 uStack_24c;
  undefined4 local_248;
  undefined4 uStack_244;
  undefined4 uStack_240;
  undefined4 uStack_23c;
  longlong local_238;
  undefined8 local_228;
  undefined4 local_220;
  undefined8 local_218;
  undefined4 local_210;
  ulonglong *local_208;
  ulonglong local_200;
  longlong local_1f8;
  longlong lStack_1f0;
  undefined4 local_1e8;
  undefined8 local_1d8;
  ulonglong local_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  ulonglong local_1b8;
  ulonglong local_1b0;
  ulonglong local_1a8;
  longlong *local_1a0;
  int ***local_198;
  ulonglong local_190;
  undefined8 local_188;
  undefined8 uStack_180;
  ulonglong local_178 [2];
  undefined **local_168;
  undefined **local_160;
  undefined **local_158;
  undefined **local_150;
  undefined **local_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  longlong local_120;
  longlong lStack_118;
  longlong local_110;
  longlong lStack_108;
  undefined4 local_100;
  undefined4 local_f8;
  undefined1 local_f4;
  char local_f3;
  undefined1 local_f2;
  undefined1 local_f0;
  undefined1 local_ef;
  undefined1 local_ee;
  undefined2 local_ec;
  undefined1 local_e4;
  longlong local_e0;
  undefined4 *puStack_d8;
  longlong local_d0;
  undefined4 local_c4;
  longlong *local_c0;
  undefined1 local_b8 [5];
  char local_b3;
  byte local_b2;
  undefined1 local_b1;
  undefined8 local_b0;
  
  local_b0 = 0xfffffffffffffffe;
  local_210 = *(undefined4 *)(in_stack_00000030 + 0x18);
  local_218 = *(undefined8 *)(in_stack_00000030 + 0x10);
  local_168 = (undefined **)*in_R9;
  local_160 = (undefined **)in_R9[1];
  local_208 = in_R9;
  local_1a0 = in_RCX;
  local_158 = in_RDX;
  country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info_mut();
  if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_b3 = *(char *)((longlong)extraout_RAX + 0x46);
  local_2f8 = extraout_RAX[4];
  local_318 = *extraout_RAX;
  uStack_310 = extraout_RAX[1];
  local_308 = extraout_RAX[2];
  uStack_300 = extraout_RAX[3];
  local_b2 = *(byte *)((longlong)extraout_RAX + 0x44);
  local_c4 = (undefined4)extraout_RAX[5];
  local_1e8 = (undefined4)extraout_RAX[8];
  local_1f8 = extraout_RAX[6];
  lStack_1f0 = extraout_RAX[7];
  local_c0 = extraout_RAX;
  country_core::resources::walls::decorator::DecoratorInfo::dst_as_roof();
  local_228 = *extraout_RAX_00;
  local_220 = *(undefined4 *)(extraout_RAX_00 + 1);
  local_178[0] = 0;
  local_b1 = 1;
  country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
  if (extraout_AL == '\b') {
    local_b1 = 1;
    country_core::resources::walls::decorator::DecoratorRank::new();
    local_b1 = 1;
    uVar15 = extraout_AL_00;
    country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
    local_1d8._0_2_ = CONCAT11(uVar15,extraout_AL_01);
    local_b1 = 1;
    std::collections::hash::map::HashMap::
    get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
              ();
    if (extraout_RAX_01 == 0) {
      uStack_2e0 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
      local_150 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
      local_148 = (undefined **)0x1;
      local_130 = 0;
      local_140 = &local_2e8;
      uStack_138 = 1;
      local_b1 = 1;
      local_2e8 = (undefined **)&local_1d8;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    fVar22 = *(float *)(extraout_RAX_01 + 0x30);
    fVar21 = *(float *)(extraout_RAX_01 + 0x3c) - *(float *)(extraout_RAX_01 + 0x38);
  }
  else {
    if (extraout_AL != '\x02') {
      local_150 = &PTR_s_internal_error__entered_unreacha_142c23f78;
      local_148 = (undefined **)0x1;
      local_140 = (undefined ***)local_b8;
      uStack_138 = 0;
      local_130 = 0;
      local_b1 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    bVar8 = *(char *)((longlong)local_c0 + 0x44) - 1;
    if (2 < bVar8) {
      local_b1 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar22 = *(float *)(&DAT_142c23fa0 + (ulonglong)bVar8 * 4);
    fVar21 = 1.0;
  }
  local_150 = local_158;
  local_148 = (undefined **)CONCAT44(local_148._4_4_,0xbf800000);
  local_140 = (undefined ***)(~(ulonglong)local_158 + (ulonglong)local_b2);
  uStack_138 = CONCAT44(uStack_138._4_4_,0x3f800000);
  fVar20 = 0.0;
  local_200 = (ulonglong)local_b2;
  if (local_b3 == ':') {
    fVar23 = 0.0;
    fVar20 = 1.0;
  }
  else {
    fVar23 = 1.0;
  }
  lVar1 = 0;
  bVar8 = local_b2;
  while (lVar10 = lVar1, uVar7 = local_178[0], lVar10 != 2) {
    lVar1 = lVar10 + 1;
    ppuVar6 = (&local_150)[lVar10 * 2];
    if (ppuVar6 != (undefined **)0x0) {
      local_130 = lVar1;
      uVar9 = (int)ppuVar6 - 1;
      if (2 < (byte)uVar9) {
        local_b1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      fVar2 = *(float *)(&DAT_142c23fa0 + (ulonglong)(uVar9 & 0xff) * 4);
      bVar19 = 2 < local_178[0];
      ppppiVar11 = (int ****)local_198;
      if (!bVar19) {
        ppppiVar11 = &local_198;
      }
      puVar13 = local_178;
      if (bVar19) {
        puVar13 = &local_190;
      }
      fVar3 = *(float *)(&local_148 + lVar10 * 2);
      puVar16 = &local_190;
      if (!bVar19) {
        puVar16 = local_178;
        uVar7 = 2;
      }
      uVar17 = *puVar16;
      if (*puVar16 == uVar7) {
        local_b1 = 1;
        smallvec::SmallVec::
        reserve_one_unchecked<array_<system_decorator::ui_systems::handle_splitting_roof_decorators::handle_moving_out_roof_decorator_part::FracturedPart,2>_>
                  ();
        puVar13 = &local_190;
        ppppiVar11 = (int ****)local_198;
        uVar17 = local_190;
        bVar8 = local_b2;
      }
      *(int *)(ppppiVar11 + uVar17 * 2) = 1;
      *(ulonglong *)((longlong)ppppiVar11 + (uVar17 * 4 + 1) * 4) =
           CONCAT44(fVar3 * (fVar21 + -1.0) * 0.5 * fVar20,fVar3 * (fVar22 - fVar2) * 0.5 * fVar23);
      *(char *)((longlong)ppppiVar11 + (uVar17 * 4 + 3) * 4) = (char)ppuVar6;
      *puVar13 = *puVar13 + 1;
    }
  }
  if (bVar8 == 0) {
    local_b1 = 1;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar18 = 0;
  uVar17 = local_178[0];
  if (2 < local_178[0]) {
    uVar17 = local_190;
  }
  local_1b8 = 0;
  if (2 < local_178[0]) {
    local_190 = uVar18;
    local_1b8 = local_178[0];
  }
  local_1c8 = local_188;
  uStack_1c0 = uStack_180;
  local_1d8 = (int ****)local_198;
  local_1d0 = local_190;
  local_1b0 = 0;
  local_1a8 = uVar17;
  local_e0 = 0;
  puStack_d8 = (undefined4 *)&DAT_00000004;
  local_d0 = 0;
  local_2a0 = &local_218;
  local_298 = &local_228;
  local_290 = local_208;
  if (uVar17 != 0) {
    local_1b0 = 1;
    ppppiVar11 = (int ****)&local_1d8;
    if (2 < local_178[0]) {
      ppppiVar11 = (int ****)local_198;
    }
    if (*(int *)ppppiVar11 != 2) {
      uVar15 = *(undefined1 *)((longlong)ppppiVar11 + 0xc);
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
      *puStack_d8 = in_stack_00000028;
      local_d0 = 1;
      uVar4 = *(undefined1 *)((longlong)local_c0 + 0x46);
      handle_moving_out_roof_decorator_part::closure_1();
      local_110 = local_2b8;
      lStack_108 = lStack_2b0;
      local_120 = local_2c8;
      lStack_118 = lStack_2c0;
      local_130 = local_2d8;
      uStack_128 = uStack_2d0;
      local_140 = (undefined ***)local_2e8;
      uStack_138 = (longlong)uStack_2e0;
      local_150 = local_168;
      local_148 = local_160;
      local_100 = 0;
      local_f8 = 0;
      local_f0 = 1;
      local_ee = 0;
      local_ec = 0x4002;
      uStack_2e0._0_5_ = CONCAT14(2,(undefined4)uStack_2e0);
      local_ef = uVar15;
      local_e4 = uVar4;
      apply_decorator_op::apply_decorator_op();
    }
    uVar18 = 1;
    if (uVar17 != 1) {
      local_1b0 = 2;
      ppppiVar11 = (int ****)&local_1d8;
      if (2 < uVar7) {
        ppppiVar11 = (int ****)local_198;
      }
      if (*(int *)(ppppiVar11 + 2) != 2) {
        local_c0 = (longlong *)CONCAT71(local_c0._1_7_,*(undefined1 *)((longlong)ppppiVar11 + 0x1c))
        ;
        country_core::resources::walls::decorator::TransientDecoratorIdGen::generate();
        lVar1 = local_d0;
        if (local_d0 == local_e0) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
        }
        puStack_d8[lVar1] = extraout_EAX;
        local_d0 = lVar1 + 1;
        handle_moving_out_roof_decorator_part::closure_1();
        if ((int)local_150 == 3) {
          local_238 = local_2f8;
          local_258 = (undefined4)local_318;
          uStack_254 = local_318._4_4_;
          uStack_250 = (undefined4)uStack_310;
          uStack_24c = uStack_310._4_4_;
          local_248 = (undefined4)local_308;
          uStack_244 = local_308._4_4_;
          uStack_240 = (undefined4)uStack_300;
          uStack_23c = uStack_300._4_4_;
        }
        else {
          local_238 = local_120;
          local_258 = (undefined4)local_140;
          uStack_254 = local_140._4_4_;
          uStack_250 = (undefined4)uStack_138;
          uStack_24c = uStack_138._4_4_;
          local_248 = (undefined4)local_130;
          uStack_244 = local_130._4_4_;
          uStack_240 = (undefined4)uStack_128;
          uStack_23c = uStack_128._4_4_;
        }
        local_110 = local_1f8;
        lStack_108 = lStack_1f0;
        local_100 = local_1e8;
        local_148 = local_168;
        local_140 = (undefined ***)local_160;
        uStack_138 = CONCAT44(uStack_254,local_258);
        local_130 = CONCAT44(uStack_24c,uStack_250);
        uStack_128 = CONCAT44(uStack_244,local_248);
        local_120 = CONCAT44(uStack_23c,uStack_240);
        lStack_118 = local_238;
        local_f8 = local_c4;
        local_f4 = 1;
        local_f3 = local_b3;
        local_f2 = local_c0._0_1_;
        local_150 = (undefined **)0x3;
        uStack_2e0._0_5_ = CONCAT14(2,(undefined4)uStack_2e0);
        apply_decorator_op::apply_decorator_op();
      }
      uVar18 = 2;
      if (uVar17 != 2) {
        ppppiVar11 = (int ****)&local_1d8;
        if (2 < uVar7) {
          ppppiVar11 = (int ****)local_198;
        }
        local_1b0 = 3;
        uVar18 = 3;
        if (*(int *)(ppppiVar11 + 4) != 2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
    }
  }
  country_core::resources::walls::decorator::TransientDecoratorIdGen::generate();
  handle_moving_out_roof_decorator_part::closure_1();
  plVar14 = &local_318;
  if ((int)local_150 != 3) {
    plVar14 = &local_140;
  }
  local_288 = *plVar14;
  lStack_280 = plVar14[1];
  local_278 = plVar14[2];
  lStack_270 = plVar14[3];
  local_268 = plVar14[4];
  bevy_app::plugin::Plugin::ready<country_core::systems::main_camera::MainCameraPlugin>();
  lStack_118 = local_268;
  uStack_128 = local_278;
  local_120 = lStack_270;
  uStack_138 = local_288;
  local_130 = lStack_280;
  local_110 = local_1f8;
  lStack_108 = lStack_1f0;
  local_100 = local_1e8;
  local_148 = local_168;
  local_140 = (undefined ***)local_160;
  local_f8 = local_c4;
  local_f4 = 1;
  local_f3 = local_b3;
  local_150 = (undefined **)0x3;
  uStack_2e0._0_5_ = CONCAT14(2,(undefined4)uStack_2e0);
  apply_decorator_op::apply_decorator_op();
  local_1a0[2] = local_d0;
  *local_1a0 = local_e0;
  local_1a0[1] = (longlong)puStack_d8;
  *(undefined4 *)(local_1a0 + 3) = extraout_EAX_00;
  local_1a0[4] = local_288;
  local_1a0[5] = lStack_280;
  local_1a0[6] = local_278;
  local_1a0[7] = lStack_270;
  local_1a0[8] = local_268;
  ppppiVar11 = (int ****)&local_1d8;
  if (2 < uVar7) {
    ppppiVar11 = (int ****)local_198;
  }
  piVar12 = (int *)((ulonglong)(uint)((int)uVar18 << 4) + (longlong)ppppiVar11);
  do {
    uVar18 = uVar18 + 1;
    if (uVar18 - uVar17 == 1) break;
    local_1b0 = uVar18;
    iVar5 = *piVar12;
    piVar12 = piVar12 + 4;
  } while (iVar5 != 2);
  if (2 < uVar7) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


