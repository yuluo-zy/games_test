// signifier_edit_circle_line  size=2088
// void __cdecl signifier_edit_circle_line(void)

void __cdecl
country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line(void)

{
  uint *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined4 *puVar5;
  undefined8 *extraout_RAX;
  code *extraout_RAX_00;
  longlong lVar6;
  ulonglong uVar7;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong lVar8;
  uint *puVar9;
  ulonglong uVar10;
  longlong in_R8;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  code *pcVar14;
  undefined8 *puVar15;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar16;
  float fVar17;
  longlong local_1c0;
  longlong *local_1b8;
  void *local_1b0;
  code *local_1a8;
  undefined8 local_1a0;
  undefined4 local_194;
  undefined8 local_190;
  undefined8 uStack_188;
  code *local_180;
  longlong *local_178;
  longlong local_170;
  longlong local_168;
  longlong local_160;
  undefined **local_158;
  code *pcStack_150;
  code **local_148;
  undefined8 *local_140;
  undefined *local_138;
  code *local_120;
  code *pcStack_118;
  longlong **local_110;
  longlong local_108;
  longlong local_100;
  undefined8 *local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  pcVar14 = (code *)*in_RDX;
  local_180 = pcVar14;
  if ((pcVar14[0x38] != (code)0x2) && (((byte)pcVar14[0x38] & 1) == 0)) {
    uVar4 = *(undefined8 *)(pcVar14 + 8);
    local_178 = in_RCX;
    ::utils::geometry::circle::Circle2d::new();
    fVar16 = 0.75;
    ::utils::geometry::circle::Circle2d::get_pos_at_u();
    local_190 = CONCAT44((float)((ulonglong)local_190 >> 0x20) - fVar16,
                         (float)local_190 - extraout_XMM0_Da);
    pcStack_118 = systems::ui::resources::ui_state::impl$15::fmt;
    local_158 = &PTR_s_edit_circle__line_142ad44f8;
    pcStack_150 = (code *)0x2;
    local_148 = &local_120;
    local_140 = (undefined8 *)0x1;
    local_138 = (undefined *)0x0;
    local_120 = pcVar14;
    alloc::fmt::format::format_inner();
    local_158 = (undefined **)local_190;
    pcStack_150 = (code *)uStack_188;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    extraout_RAX[2] = local_158;
    extraout_RAX[3] = pcStack_150;
    extraout_RAX[4] = uVar4;
    pcStack_150 = (code *)&DAT_142ad4518;
    if ((longlong)local_1a8 < 0) {
LAB_140a6525d:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (local_1a8 == (code *)0x0) {
      pcVar14 = (code *)0x1;
      puVar15 = extraout_RAX;
    }
    else {
      local_f8 = extraout_RAX;
      std::alloc::__default_lib_allocator::__rust_alloc();
      pcVar14 = extraout_RAX_00;
      puVar15 = local_f8;
      if (extraout_RAX_00 == (code *)0x0) goto LAB_140a6525d;
    }
    memcpy(pcVar14,local_1b0,(size_t)local_1a8);
    local_158 = (undefined **)local_1a8;
    local_148 = (code **)local_1a8;
    local_138 = &DAT_142ad4518;
    pcStack_150 = pcVar14;
    local_140 = puVar15;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    in_RCX = local_178;
    if (local_1b8 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      in_RCX = local_178;
    }
  }
  local_108 = *in_RCX;
  local_160 = in_RCX[1];
  lVar3 = in_RCX[2];
  puVar9 = *(uint **)(local_108 + 0xf8);
  puVar1 = puVar9 + *(longlong *)(local_108 + 0x100);
  local_178 = (longlong *)CONCAT71(local_178._1_7_,*(undefined1 *)(local_108 + 0x108));
  local_194 = *(undefined4 *)(in_R8 + 0x1c);
  puVar5 = *(undefined4 **)(in_R8 + 0x10);
  local_1c0 = 8;
  local_100 = 0;
  local_f8 = (undefined8 *)0x0;
  lVar13 = 0;
  uVar10 = 0;
  uVar12 = 0;
  do {
    if ((char)local_178 == '\0') {
      if (uVar12 == uVar10) {
        if (puVar9 == puVar1) {
          return;
        }
        while( true ) {
          uVar10 = *(ulonglong *)
                    (*(longlong *)(local_160 + 0x100) + 0x58 + (ulonglong)*puVar9 * 0xa0);
          if (uVar10 != 0) break;
          puVar9 = puVar9 + 1;
          if (puVar9 == puVar1) {
            return;
          }
        }
        lVar6 = *(longlong *)(local_160 + 0x100) + (ulonglong)*puVar9 * 0xa0;
        uVar7 = (ulonglong)*(uint *)(lVar6 + 0x94);
        lVar13 = *(longlong *)(local_160 + 0x1a8);
        uVar12 = *(ulonglong *)(lVar13 + 0x40 + uVar7 * 0x48);
        lVar2 = *(longlong *)(lVar13 + 0x18 + uVar7 * 0x48);
        lVar13 = *(longlong *)(lVar13 + 0x38 + uVar7 * 0x48);
        if ((*(ulonglong *)(local_108 + 0x118) < uVar12) &&
           (uVar7 = *(ulonglong *)(lVar13 + *(ulonglong *)(local_108 + 0x118) * 8), uVar7 != 0)) {
          local_f8 = *(undefined8 **)(lVar2 + 0x10 + ~uVar7 * 0x30);
          uVar7 = *(ulonglong *)(local_108 + 0x120);
          if (uVar12 <= uVar7) goto LAB_140a64f0f;
LAB_140a64ef0:
          uVar12 = *(ulonglong *)(lVar13 + uVar7 * 8);
          if (uVar12 == 0) goto LAB_140a64f0f;
          local_100 = *(longlong *)(lVar2 + 0x10 + ~uVar12 * 0x30);
        }
        else {
          local_f8 = (undefined8 *)0x0;
          uVar7 = *(ulonglong *)(local_108 + 0x120);
          if (uVar7 < uVar12) goto LAB_140a64ef0;
LAB_140a64f0f:
          local_100 = 0;
        }
        lVar8 = ~*(ulonglong *)(lVar13 + *(longlong *)(local_108 + 0x110) * 8) * 0x30;
        lVar13 = *(longlong *)(lVar2 + 0x10 + lVar8);
        local_170 = *(longlong *)(lVar2 + 0x20 + lVar8);
        local_168 = *(longlong *)(lVar2 + 0x28 + lVar8);
        puVar9 = puVar9 + 1;
        local_1c0 = *(longlong *)(lVar6 + 0x50);
        uVar12 = 0;
      }
      uVar7 = (ulonglong)*(uint *)(local_1c0 + 8 + uVar12 * 0x10);
    }
    else {
      if (uVar12 == uVar10) {
        if (puVar9 == puVar1) {
          return;
        }
        while( true ) {
          uVar10 = *(ulonglong *)
                    (*(longlong *)(local_160 + 0x1a8) + 0x10 + (ulonglong)*puVar9 * 0x48);
          if (uVar10 != 0) break;
          puVar9 = puVar9 + 1;
          if (puVar9 == puVar1) {
            return;
          }
        }
        lVar2 = *(longlong *)(local_160 + 0x1a8) + (ulonglong)*puVar9 * 0x48;
        lVar6 = *(longlong *)(lVar2 + 0x18);
        lVar8 = *(longlong *)(lVar2 + 0x38);
        if ((*(ulonglong *)(local_108 + 0x118) < *(ulonglong *)(lVar2 + 0x40)) &&
           (uVar12 = *(ulonglong *)(lVar8 + *(ulonglong *)(local_108 + 0x118) * 8), uVar12 != 0)) {
          local_f8 = *(undefined8 **)(lVar6 + 0x10 + ~uVar12 * 0x30);
        }
        else {
          local_f8 = (undefined8 *)0x0;
        }
        lVar11 = ~*(ulonglong *)(lVar8 + *(longlong *)(local_108 + 0x110) * 8) * 0x30;
        lVar13 = *(longlong *)(lVar6 + 0x10 + lVar11);
        local_170 = *(longlong *)(lVar6 + 0x20 + lVar11);
        local_168 = *(longlong *)(lVar6 + 0x28 + lVar11);
        puVar9 = puVar9 + 1;
        if ((*(ulonglong *)(local_108 + 0x120) < *(ulonglong *)(lVar2 + 0x40)) &&
           (uVar12 = *(ulonglong *)(lVar8 + *(ulonglong *)(local_108 + 0x120) * 8), uVar12 != 0)) {
          local_100 = *(longlong *)(lVar6 + 0x10 + ~uVar12 * 0x30);
        }
        else {
          local_100 = 0;
        }
        uVar12 = 0;
      }
      uVar7 = uVar12 & 0xffffffff;
    }
    local_1b8 = (longlong *)(uVar7 * 0x28 + lVar13);
    local_1b0 = (void *)(local_170 + uVar7 * 4);
    local_1a8 = (code *)(local_168 + uVar7 * 4);
    fVar16 = *(float *)(uVar7 * 0x1c + local_100 + 0x18);
    fVar17 = fVar16 * 0.1 + (1.0 - fVar16) * 0.01;
    fVar16 = 0.75 - fVar17;
    local_190 = CONCAT44(local_190._4_4_,fVar16);
    local_1a0 = lVar3;
    if ((fVar16 < 0.0) || (1.0 < fVar16)) {
      local_120 = (code *)&local_190;
      pcStack_118 = core::fmt::float::impl$5::fmt;
      local_158 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
      pcStack_150 = (code *)0x1;
      local_138 = (undefined *)0x0;
      local_148 = &local_120;
      local_140 = (undefined8 *)0x1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    if (NAN(fVar16)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    fVar17 = fVar17 + 0.75;
    local_190 = CONCAT44(local_190._4_4_,fVar17);
    if ((fVar17 < 0.0) || (1.0 < fVar17)) {
      local_120 = (code *)&local_190;
      pcStack_118 = core::fmt::float::impl$5::fmt;
      local_158 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
      pcStack_150 = (code *)0x1;
      local_138 = (undefined *)0x0;
      local_148 = &local_120;
      local_140 = (undefined8 *)0x1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    if (NAN(fVar17)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    if (local_180[0x38] != (code)0x2) {
      ::utils::geometry::circle::Circle2d::new();
      fVar16 = 0.75;
      ::utils::geometry::circle::Circle2d::get_pos_at_u();
      local_158 = (undefined **)
                  CONCAT44((float)((ulonglong)local_120 >> 0x20) - fVar16,
                           SUB84(local_120,0) - extraout_XMM0_Da_00);
      pcStack_150 = pcStack_118;
      *(undefined4 *)local_1a8 = local_1a0._4_4_;
      *local_1b8 = (longlong)local_158;
      local_1b8[1] = (longlong)pcStack_118;
      local_120 = (code *)local_158;
    }
    ::utils::geometry::circle::Circle2d::as_curve2_range();
    pcStack_118 = pcStack_150 + (longlong)local_148 * 8;
    local_120 = pcStack_150;
    local_110 = &local_1b8;
    *puVar5 = local_194;
    country_core::utils::geo::
    update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env_1>_>
              ();
    if (local_158 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_140 != (undefined8 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uVar12 = uVar12 + 1;
  } while( true );
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'as_curve2_range', 'address': '140c97f50'}, {'name': 'format_inner', 'address': '140d79f90'}, {'name': 'get_pos_at_u', 'address': '140c975a0'}, {'name': 'handle_alloc_error', 'address': '1428d9000'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'memcpy', 'address': '142923c00'}, {'name': 'new', 'address': '140c97540'}, {'name': 'panic', 'address': '1428d9430'}, {'name': 'panic_fmt', 'address': '1428d9390'}, {'name': 'push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>', 'address': '14146c880'}, {'name': 'update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::circle::signifiers::edit_line::signifier_edit_circle_line::closure_env$1>_>', 'address': '14157eda0'}]