// 0x1408dda10 signifier_edit_dims size=2768 sig=void __cdecl signifier_edit_dims(void)
void __cdecl
country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims(void)

{
  size_t *psVar1;
  size_t sVar2;
  uint *puVar3;
  ulonglong *puVar4;
  undefined4 *puVar5;
  longlong *plVar6;
  undefined1 *extraout_RAX;
  undefined1 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  void *extraout_RAX_02;
  longlong *in_RCX;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  longlong *in_R8;
  longlong in_R9;
  undefined1 *_Dst;
  undefined8 *puVar13;
  uint *puVar14;
  void *pvVar15;
  longlong lVar16;
  float fVar17;
  float fVar18;
  longlong local_208;
  void *local_200;
  size_t local_1f8;
  ulonglong *local_1d8;
  code *pcStack_1d0;
  longlong *local_1c8;
  ulonglong uStack_1c0;
  undefined1 *local_1b8;
  ulonglong uStack_1b0;
  undefined8 local_1a8;
  longlong local_1a0;
  undefined4 local_198;
  undefined4 uStack_194;
  longlong *local_188;
  undefined1 *puStack_180;
  longlong *local_178;
  ulonglong uStack_170;
  undefined1 *local_168;
  ulonglong uStack_160;
  undefined4 local_158;
  undefined4 uStack_154;
  longlong local_150;
  undefined4 local_148;
  longlong local_140;
  longlong local_138;
  undefined4 local_130;
  undefined4 local_12c;
  longlong local_128;
  longlong local_120;
  undefined **local_118;
  undefined **ppuStack_110;
  longlong **local_108;
  undefined8 *puStack_100;
  undefined **local_f8;
  ulonglong uStack_f0;
  undefined1 *local_e8;
  ulonglong uStack_e0;
  undefined8 local_d8;
  longlong lStack_d0;
  undefined8 local_c8;
  longlong *local_c0;
  longlong *local_b8;
  longlong local_b0;
  undefined1 *local_a8;
  undefined8 *local_a0;
  undefined1 local_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  local_c0 = (longlong *)*in_RCX;
  if (!SBORROW8(0,*local_c0)) {
    local_148 = (undefined4)local_c0[9];
    local_150 = local_c0[8];
    puVar4 = (ulonglong *)local_c0[2];
    sVar2 = (longlong)puVar4 * 8;
    if (((ulonglong)puVar4 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar2)) {
LAB_1408de49e:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    pvVar15 = (void *)local_c0[1];
    local_b0 = in_R9;
    if (sVar2 == 0) {
      local_a8 = &DAT_00000004;
      local_b8 = (longlong *)0x0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_a8 = extraout_RAX;
      local_b8 = (longlong *)puVar4;
      if (extraout_RAX == (undefined1 *)0x0) goto LAB_1408de49e;
    }
    memcpy(local_a8,pvVar15,sVar2);
    uVar12 = local_c0[5];
    sVar2 = uVar12 * 4;
    if ((uVar12 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar2)) {
LAB_1408de4b0:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    pvVar15 = (void *)local_c0[4];
    if (sVar2 == 0) {
      _Dst = &DAT_00000004;
      uVar11 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      uVar11 = uVar12;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_1408de4b0;
    }
    memcpy(_Dst,pvVar15,sVar2);
    local_158 = (undefined4)local_c0[6];
    local_188 = local_b8;
    puStack_180 = local_a8;
    local_1d8 = (ulonglong *)(local_c0 + 7);
    pcStack_1d0 = systems::ui::resources::ui_state::impl$15::fmt;
    local_118 = &PTR_s_rectangle_edit_dims_line__142aa4ca0;
    ppuStack_110 = (undefined **)0x1;
    local_108 = (longlong **)&local_1d8;
    puStack_100 = (undefined8 *)0x1;
    local_f8 = (undefined **)0x0;
    local_91 = 1;
    local_178 = (longlong *)puVar4;
    uStack_170 = uVar11;
    local_168 = _Dst;
    uStack_160 = uVar12;
    alloc::fmt::format::format_inner();
    local_198 = local_148;
    local_1a0 = local_150;
    local_1a8 = CONCAT44(uStack_154,local_158);
    local_1d8 = (ulonglong *)local_188;
    pcStack_1d0 = (code *)puStack_180;
    local_1c8 = local_178;
    uStack_1c0 = uStack_170;
    local_1b8 = local_168;
    uStack_1b0 = uStack_160;
    local_118 = (undefined **)0x1;
    ppuStack_110 = (undefined **)0x1;
    local_108 = (longlong **)local_188;
    puStack_100 = (undefined8 *)puStack_180;
    local_f8 = (undefined **)local_178;
    uStack_f0 = uStack_170;
    local_e8 = local_168;
    uStack_e0 = uStack_160;
    lStack_d0 = local_150;
    local_c8 = CONCAT44(uStack_194,local_148);
    local_d8 = local_1a8;
    std::alloc::__default_lib_allocator::__rust_alloc();
    local_b8 = (longlong *)local_208;
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    extraout_RAX_01[10] = local_c8;
    extraout_RAX_01[8] = local_d8;
    extraout_RAX_01[9] = lStack_d0;
    extraout_RAX_01[6] = local_e8;
    extraout_RAX_01[7] = uStack_e0;
    extraout_RAX_01[4] = local_f8;
    extraout_RAX_01[5] = uStack_f0;
    extraout_RAX_01[2] = local_108;
    extraout_RAX_01[3] = puStack_100;
    *extraout_RAX_01 = local_118;
    extraout_RAX_01[1] = ppuStack_110;
    ppuStack_110 = &
                   PTR_drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>_>_142aa4cb0
    ;
    if ((longlong)local_1f8 < 0) {
LAB_1408de4df:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (local_1f8 == 0) {
      pvVar15 = (void *)0x1;
      puVar13 = extraout_RAX_01;
    }
    else {
      local_a0 = extraout_RAX_01;
      std::alloc::__default_lib_allocator::__rust_alloc();
      puVar13 = local_a0;
      pvVar15 = extraout_RAX_02;
      if (extraout_RAX_02 == (void *)0x0) goto LAB_1408de4df;
    }
    memcpy(pvVar15,local_200,local_1f8);
    local_118 = (undefined **)local_1f8;
    local_f8 = &
               PTR_drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>_>_142aa4cb0
    ;
    ppuStack_110 = pvVar15;
    puStack_100 = puVar13;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    in_R9 = local_b0;
    if (local_b8 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  plVar6 = local_c0;
  local_140 = *in_R8;
  local_120 = in_R8[1];
  local_12c = *(undefined4 *)((longlong)in_R8 + 0x14);
  puVar14 = *(uint **)(local_140 + 0xf8);
  puVar3 = puVar14 + *(longlong *)(local_140 + 0x100);
  local_b8 = (longlong *)CONCAT71(local_b8._1_7_,*(undefined1 *)(local_140 + 0x108));
  psVar1 = (size_t *)(local_c0 + 8);
  local_130 = *(undefined4 *)(in_R9 + 0x1c);
  puVar5 = *(undefined4 **)(in_R9 + 0x10);
  local_138 = 8;
  lVar16 = 0;
  local_b0 = 0;
  local_a0 = (undefined8 *)0x0;
  local_a8 = (undefined1 *)0x0;
  uVar12 = 0;
  uVar11 = 0;
  do {
    if ((char)local_b8 == '\0') {
      if (uVar11 == uVar12) {
        if (puVar14 == puVar3) {
          return;
        }
        while( true ) {
          uVar12 = *(ulonglong *)
                    (*(longlong *)(local_120 + 0x100) + 0x58 + (ulonglong)*puVar14 * 0xa0);
          if (uVar12 != 0) break;
          puVar14 = puVar14 + 1;
          if (puVar14 == puVar3) {
            return;
          }
        }
        lVar7 = *(longlong *)(local_120 + 0x100) + (ulonglong)*puVar14 * 0xa0;
        uVar11 = (ulonglong)*(uint *)(lVar7 + 0x94);
        lVar16 = *(longlong *)(local_120 + 0x1a8) + uVar11 * 0x48;
        uVar11 = *(ulonglong *)(*(longlong *)(local_120 + 0x1a8) + 0x40 + uVar11 * 0x48);
        if ((*(ulonglong *)(local_140 + 0x110) < uVar11) &&
           (uVar9 = *(ulonglong *)
                     (*(longlong *)(lVar16 + 0x38) + *(ulonglong *)(local_140 + 0x110) * 8),
           uVar9 != 0)) {
          local_a8 = *(undefined1 **)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar9 * 0x30);
          uVar9 = *(ulonglong *)(local_140 + 0x118);
          if (uVar11 <= uVar9) goto LAB_1408de16d;
LAB_1408de12f:
          uVar9 = *(ulonglong *)(*(longlong *)(lVar16 + 0x38) + uVar9 * 8);
          if (uVar9 == 0) goto LAB_1408de16d;
          local_a0 = *(undefined8 **)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar9 * 0x30);
          uVar9 = *(ulonglong *)(local_140 + 0x120);
          lVar10 = *(longlong *)(lVar16 + 0x38);
          if (uVar11 <= uVar9) goto LAB_1408de167;
LAB_1408de188:
          uVar11 = *(ulonglong *)(lVar10 + uVar9 * 8);
          lVar8 = *(longlong *)(lVar16 + 0x18);
          if (uVar11 == 0) goto LAB_1408de1ae;
          local_b0 = *(longlong *)(lVar8 + 0x10 + ~uVar11 * 0x30);
        }
        else {
          local_a8 = (undefined1 *)0x0;
          uVar9 = *(ulonglong *)(local_140 + 0x118);
          if (uVar9 < uVar11) goto LAB_1408de12f;
LAB_1408de16d:
          local_a0 = (undefined8 *)0x0;
          uVar9 = *(ulonglong *)(local_140 + 0x120);
          lVar10 = *(longlong *)(lVar16 + 0x38);
          if (uVar9 < uVar11) goto LAB_1408de188;
LAB_1408de167:
          lVar8 = *(longlong *)(lVar16 + 0x18);
LAB_1408de1ae:
          local_b0 = 0;
        }
        puVar14 = puVar14 + 1;
        lVar10 = ~*(ulonglong *)(lVar10 + *(longlong *)(local_140 + 0x128) * 8) * 0x30;
        lVar16 = *(longlong *)(lVar8 + 0x10 + lVar10);
        local_128 = *(longlong *)(lVar8 + 0x28 + lVar10);
        local_138 = *(longlong *)(lVar7 + 0x50);
        uVar11 = 0;
      }
      uVar9 = (ulonglong)*(uint *)(local_138 + 8 + uVar11 * 0x10);
    }
    else {
      if (uVar11 == uVar12) {
        if (puVar14 == puVar3) {
          return;
        }
        while( true ) {
          uVar12 = *(ulonglong *)
                    (*(longlong *)(local_120 + 0x1a8) + 0x10 + (ulonglong)*puVar14 * 0x48);
          if (uVar12 != 0) break;
          puVar14 = puVar14 + 1;
          if (puVar14 == puVar3) {
            return;
          }
        }
        lVar16 = *(longlong *)(local_120 + 0x1a8) + (ulonglong)*puVar14 * 0x48;
        uVar11 = *(ulonglong *)(lVar16 + 0x40);
        if ((*(ulonglong *)(local_140 + 0x110) < uVar11) &&
           (uVar9 = *(ulonglong *)
                     (*(longlong *)(lVar16 + 0x38) + *(ulonglong *)(local_140 + 0x110) * 8),
           uVar9 != 0)) {
          local_a8 = *(undefined1 **)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar9 * 0x30);
          uVar9 = *(ulonglong *)(local_140 + 0x118);
          if (uVar11 <= uVar9) goto LAB_1408de097;
LAB_1408de059:
          uVar9 = *(ulonglong *)(*(longlong *)(lVar16 + 0x38) + uVar9 * 8);
          if (uVar9 == 0) goto LAB_1408de097;
          local_a0 = *(undefined8 **)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar9 * 0x30);
          uVar9 = *(ulonglong *)(local_140 + 0x120);
          lVar7 = *(longlong *)(lVar16 + 0x38);
          if (uVar11 <= uVar9) goto LAB_1408de091;
LAB_1408de0b2:
          uVar11 = *(ulonglong *)(lVar7 + uVar9 * 8);
          lVar10 = *(longlong *)(lVar16 + 0x18);
          if (uVar11 == 0) goto LAB_1408de0d8;
          local_b0 = *(longlong *)(lVar10 + 0x10 + ~uVar11 * 0x30);
        }
        else {
          local_a8 = (undefined1 *)0x0;
          uVar9 = *(ulonglong *)(local_140 + 0x118);
          if (uVar9 < uVar11) goto LAB_1408de059;
LAB_1408de097:
          local_a0 = (undefined8 *)0x0;
          uVar9 = *(ulonglong *)(local_140 + 0x120);
          lVar7 = *(longlong *)(lVar16 + 0x38);
          if (uVar9 < uVar11) goto LAB_1408de0b2;
LAB_1408de091:
          lVar10 = *(longlong *)(lVar16 + 0x18);
LAB_1408de0d8:
          local_b0 = 0;
        }
        puVar14 = puVar14 + 1;
        lVar7 = ~*(ulonglong *)(lVar7 + *(longlong *)(local_140 + 0x128) * 8) * 0x30;
        lVar16 = *(longlong *)(lVar10 + 0x10 + lVar7);
        local_128 = *(longlong *)(lVar10 + 0x28 + lVar7);
        uVar11 = 0;
      }
      uVar9 = uVar11 & 0xffffffff;
    }
    lVar7 = uVar9 * 0x30 + lVar16;
    if (!SBORROW8(0,*local_c0)) {
      lVar10 = plVar6[9];
      ppuStack_110 = (undefined **)CONCAT44(ppuStack_110._4_4_,(int)lVar10);
      local_118 = (undefined **)*psVar1;
      *(undefined4 *)(local_128 + uVar9 * 4) = local_12c;
      *(int *)(lVar7 + 0x18) = (int)lVar10;
      *(undefined ***)(lVar7 + 0x10) = local_118;
    }
    fVar17 = *(float *)(uVar9 * 0x1c + local_b0 + 0x18);
    fVar18 = fVar17 * 0.5 + (1.0 - fVar17) * 0.01;
    fVar17 = 0.5 - fVar18;
    local_188 = (longlong *)CONCAT44(local_188._4_4_,fVar17);
    if ((fVar17 < 0.0) || (1.0 < fVar17)) {
      local_1d8 = (ulonglong *)&local_188;
      pcStack_1d0 = core::fmt::float::impl$5::fmt;
      local_118 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
      ppuStack_110 = (undefined **)0x1;
      local_f8 = (undefined **)0x0;
      local_108 = (longlong **)&local_1d8;
      puStack_100 = (undefined8 *)0x1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    if (NAN(fVar17)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar18 = fVar18 + 0.5;
    local_188 = (longlong *)CONCAT44(local_188._4_4_,fVar18);
    if ((fVar18 < 0.0) || (1.0 < fVar18)) {
      local_1d8 = (ulonglong *)&local_188;
      pcStack_1d0 = core::fmt::float::impl$5::fmt;
      local_118 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
      ppuStack_110 = (undefined **)0x1;
      local_f8 = (undefined **)0x0;
      local_108 = (longlong **)&local_1d8;
      puStack_100 = (undefined8 *)0x1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    if (NAN(fVar18)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    ::utils::curve::Curve::crashy_slice<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
    *puVar5 = local_130;
    country_core::utils::geo::
    update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env_1>_>
              ();
    if (local_118 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (puStack_100 != (undefined8 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uVar11 = uVar11 + 1;
  } while( true );
}
