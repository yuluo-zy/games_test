
void __cdecl
system_decorator::ui_systems::handle_splitting_wall_decorators::
display_decorator_wall_parts_and_get_hovered_part_id(void)

{
  float fVar1;
  float fVar2;
  byte bVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  undefined1 extraout_AL;
  undefined1 extraout_AL_00;
  undefined1 extraout_AL_01;
  undefined1 extraout_AL_02;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  float *in_RCX;
  ulonglong uVar9;
  undefined1 uVar10;
  longlong in_RDX;
  longlong in_R8;
  float *in_R9;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
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
  float fVar32;
  float fVar33;
  float fVar34;
  longlong local_1e8;
  longlong local_1d0;
  ulonglong local_198;
  undefined1 local_190;
  undefined1 uStack_18f;
  undefined6 uStack_18e;
  undefined8 local_188;
  undefined8 *local_178;
  float local_170;
  ulonglong *local_168;
  code *pcStack_160;
  undefined8 **local_158;
  code *pcStack_150;
  undefined8 local_148;
  longlong local_138;
  undefined **local_130;
  undefined8 uStack_128;
  undefined8 **local_120;
  undefined8 uStack_118;
  undefined8 local_110;
  longlong local_100;
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  fVar1 = *in_RCX;
  fVar2 = in_RCX[1];
  fVar4 = in_RCX[2];
  fVar20 = in_RCX[3];
  fVar11 = fVar20 * fVar20;
  fVar21 = fVar4 * fVar4 + fVar2 * fVar2 + fVar1 * fVar1;
  fVar12 = fVar4 * 1.0 + fVar2 * 0.0 + fVar1 * 0.0;
  fVar12 = fVar12 + fVar12;
  fVar20 = fVar20 + fVar20;
  fVar15 = fVar20 * (fVar2 * 1.0 - fVar4 * 0.0) + fVar12 * fVar1 + (fVar11 - fVar21) * 0.0;
  fVar18 = fVar20 * (fVar4 * 0.0 - fVar1 * 1.0) + fVar12 * fVar2 + (fVar11 - fVar21) * 0.0;
  fVar19 = fVar20 * (fVar1 * 0.0 - fVar2 * 0.0) + fVar12 * fVar4 + (fVar11 - fVar21) * 1.0;
  fVar1 = *(float *)(in_RDX + 0x18);
  local_170 = in_RCX[6];
  local_178 = *(undefined8 **)(in_RCX + 4);
  fVar33 = (float)*(undefined8 *)(in_RDX + 0x1c);
  fVar34 = (float)((ulonglong)*(undefined8 *)(in_RDX + 0x1c) >> 0x20);
  fVar16 = fVar19 * (fVar34 - local_170) +
           (fVar1 - SUB84(local_178,0)) * fVar15 +
           fVar18 * (fVar33 - (float)((ulonglong)local_178 >> 0x20));
  fVar4 = *in_R9;
  fVar20 = in_R9[1];
  fVar11 = in_R9[3];
  fVar2 = in_R9[2];
  fVar22 = fVar2 + fVar2;
  fVar31 = fVar20 + fVar20;
  fVar29 = fVar4 * (fVar4 + fVar4);
  fVar30 = (fVar4 + fVar4) * fVar11;
  bVar3 = *(byte *)(in_R8 + 0x44);
  fVar12 = in_R9[7];
  fVar21 = in_R9[8];
  fVar5 = in_R9[9];
  local_f1 = *(char *)(in_R8 + 0x46);
  uVar9 = (ulonglong)bVar3 - 1;
  if ((longlong)uVar9 < 0) {
    fVar32 = (float)uVar9;
  }
  else {
    fVar32 = (float)(int)uVar9;
  }
  fVar6 = in_R9[4];
  fVar7 = in_R9[5];
  fVar8 = in_R9[6];
  fVar17 = (float)(~((int)((uint)(local_f1 == ':') << 0x1f) >> 0x1f) & 0x3f800000);
  fVar13 = (float)((int)((uint)(local_f1 == ':') << 0x1f) >> 0x1f & 0x3f800000);
  fVar23 = 100.0;
  uVar9 = 0;
  do {
    if (bVar3 <= uVar9) {
      local_168 = (ulonglong *)&local_178;
      pcStack_160 = glam::f32::vec3::impl$92::fmt;
      local_158 = (undefined8 **)&stack0x00000030;
      pcStack_150 = country_core::resources::walls::decorator::impl$2::fmt;
      local_130 = (undefined **)&DAT_142c23a18;
      uStack_128 = 2;
      uStack_118 = 2;
      local_110 = 0;
      local_120 = &local_168;
      alloc::fmt::format::format_inner();
      uStack_128 = CONCAT44(uStack_128._4_4_,local_170);
      local_130 = (undefined **)local_178;
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_100 = local_188;
      if (extraout_RAX_03 != (undefined8 *)0x0) {
        *extraout_RAX_03 = 1;
        extraout_RAX_03[1] = 1;
        extraout_RAX_03[2] = local_130;
        *(undefined4 *)(extraout_RAX_03 + 3) = (undefined4)uStack_128;
        country_core::systems::ui::signifiers::SignifierFactory::new();
        local_148 = local_110;
        local_158 = local_120;
        pcStack_150 = (code *)uStack_118;
        local_168 = (ulonglong *)local_130;
        pcStack_160 = (code *)uStack_128;
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
        if (CONCAT62(uStack_18e,CONCAT11(uStack_18f,local_190)) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        return;
      }
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    local_198 = uVar9;
    if (fVar17 == 1.0 && fVar13 == 0.0) {
      country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype();
      uVar10 = extraout_AL;
      country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
      local_190 = extraout_AL_00;
      uStack_18f = uVar10;
      std::collections::hash::map::HashMap::
      get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                ();
      if (extraout_RAX == 0) {
LAB_141278b9e:
        local_168 = (ulonglong *)&local_190;
        pcStack_160 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
        local_130 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
        uStack_128 = 1;
        local_110 = 0;
        uStack_118 = 1;
        local_120 = &local_168;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      fVar25 = *(float *)(extraout_RAX + 0x30);
      if (-1 < (longlong)local_198) goto LAB_141278644;
LAB_1412786a3:
      fVar24 = (float)local_198;
    }
    else {
      if (local_f1 != ':') {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype();
      uVar10 = extraout_AL_01;
      country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
      local_190 = extraout_AL_02;
      uStack_18f = uVar10;
      std::collections::hash::map::HashMap::
      get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                ();
      if (extraout_RAX_00 == 0) goto LAB_141278b9e;
      fVar25 = *(float *)(extraout_RAX_00 + 0x3c) - *(float *)(extraout_RAX_00 + 0x38);
      if ((longlong)local_198 < 0) goto LAB_1412786a3;
LAB_141278644:
      fVar24 = (float)(longlong)local_198;
    }
    uVar9 = uVar9 + 1;
    fVar25 = (fVar24 / fVar32 + -0.5) * fVar25 * (1.0 - 1.0 / (float)bVar3);
    fVar24 = fVar25 * fVar17;
    fVar27 = fVar25 * fVar13;
    fVar25 = fVar25 * 0.0;
    fVar26 = fVar25 * fVar5 * (fVar31 * fVar11 + fVar22 * fVar4) +
             fVar27 * fVar21 * (fVar31 * fVar4 - fVar11 * fVar22) +
             fVar24 * fVar12 * (1.0 - (fVar20 * fVar31 + fVar2 * fVar22)) + fVar6;
    fVar28 = fVar25 * fVar5 * (fVar22 * fVar20 - fVar30) +
             fVar27 * fVar21 * (1.0 - (fVar2 * fVar22 + fVar29)) +
             fVar24 * fVar12 * (fVar31 * fVar4 + fVar11 * fVar22) + fVar7;
    fVar25 = fVar25 * fVar5 * (1.0 - (fVar29 + fVar20 * fVar31)) +
             fVar27 * fVar21 * (fVar30 + fVar22 * fVar20) +
             fVar24 * fVar12 * (fVar22 * fVar4 - fVar31 * fVar11) + fVar8;
    if ((bVar3 == 3) && (local_198 == 1)) {
      local_168 = &local_198;
      pcStack_160 = core::fmt::num::imp::impl$9::fmt;
      local_158 = (undefined8 **)&stack0x00000030;
      pcStack_150 = country_core::resources::walls::decorator::impl$2::fmt;
      local_130 = (undefined **)&DAT_142c23a18;
      uStack_128 = 2;
      uStack_118 = 2;
      local_110 = 0;
      local_120 = &local_168;
      alloc::fmt::format::format_inner();
      local_100 = local_1d0;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_01 = 1;
      extraout_RAX_01[1] = 1;
      extraout_RAX_01[2] = CONCAT44(fVar28,fVar26);
      *(float *)(extraout_RAX_01 + 3) = fVar25;
      country_core::systems::ui::signifiers::SignifierFactory::new();
      local_148 = local_110;
      local_158 = local_120;
      pcStack_150 = (code *)uStack_118;
      local_168 = (ulonglong *)local_130;
      pcStack_160 = (code *)uStack_128;
      country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
      if (local_100 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    else {
      fVar24 = fVar28 - (fVar33 - fVar18 * fVar16);
      fVar27 = fVar25 - (fVar34 - fVar16 * fVar19);
      fVar14 = fVar26 - (fVar1 - fVar16 * fVar15);
      fVar24 = SQRT(fVar14 * fVar14 + fVar24 * fVar24 + fVar27 * fVar27);
      if (fVar24 < fVar23) {
        local_178 = (undefined8 *)CONCAT44(fVar28,fVar26);
        local_170 = fVar25;
        fVar23 = fVar24;
      }
      local_100 = CONCAT44(local_100._4_4_,fVar23);
      local_168 = &local_198;
      pcStack_160 = core::fmt::num::imp::impl$9::fmt;
      local_158 = (undefined8 **)&stack0x00000030;
      pcStack_150 = country_core::resources::walls::decorator::impl$2::fmt;
      local_130 = (undefined **)&DAT_142c23a18;
      uStack_128 = 2;
      uStack_118 = 2;
      local_110 = 0;
      local_120 = &local_168;
      alloc::fmt::format::format_inner();
      local_138 = local_1e8;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_02 = 1;
      extraout_RAX_02[1] = 1;
      extraout_RAX_02[2] = CONCAT44(fVar28,fVar26);
      *(float *)(extraout_RAX_02 + 3) = fVar25;
      country_core::systems::ui::signifiers::SignifierFactory::new();
      local_148 = local_110;
      local_158 = local_120;
      pcStack_150 = (code *)uStack_118;
      local_168 = (ulonglong *)local_130;
      pcStack_160 = (code *)uStack_128;
      country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
      fVar23 = (float)local_100;
      if (local_138 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  } while( true );
}


