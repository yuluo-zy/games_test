
// WARNING: Type propagation algorithm not settling

void __cdecl
country_core::systems::wall::shapes::rectangle::ui_modes::edit_dims::UiRectangleDimsEdit::signifier
          (void)

{
  char cVar1;
  size_t sVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  float *pfVar5;
  float *pfVar6;
  float fVar7;
  undefined4 uVar8;
  undefined8 *extraout_RAX;
  float *extraout_RAX_00;
  longlong lVar9;
  undefined8 *extraout_RAX_01;
  void *extraout_RAX_02;
  float *extraout_RAX_03;
  byte *in_RCX;
  longlong *in_RDX;
  size_t *in_R9;
  float *pfVar10;
  void *_Dst;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float in_XMM1_Da;
  float fVar11;
  float fVar12;
  float fVar13;
  float *local_1b8;
  undefined8 uStack_1b0;
  longlong local_1a0;
  void *local_198;
  float *local_190;
  undefined *local_188;
  void *local_180;
  float **local_178;
  undefined8 local_170;
  undefined8 local_168;
  float *local_158;
  float *pfStack_150;
  float *local_148;
  float *local_138 [3];
  code *pcStack_120;
  char *local_118;
  size_t sStack_110;
  size_t local_108;
  undefined8 local_100;
  float local_f8;
  undefined4 uStack_f4;
  float local_f0;
  undefined4 local_ec;
  float *local_e8;
  float *pfStack_e0;
  float **local_d8;
  code *pcStack_d0;
  float **local_c8;
  code *pcStack_c0;
  undefined4 *local_b8;
  code *local_b0;
  longlong local_a8;
  size_t *local_a0;
  float *local_98;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  if (in_RCX[0x58] == 0) {
    lVar9 = *in_RDX;
    if (*(char *)(lVar9 + 0x18) == '\0') {
      return;
    }
    sVar2 = in_RDX[1];
    if (*(size_t *)(lVar9 + 0x38) != sVar2) {
      return;
    }
    if (*(int *)(lVar9 + 0x28) == 2) {
      return;
    }
    local_a0 = in_R9;
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    fVar11 = in_XMM1_Da;
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    local_e8 = (float *)CONCAT44(fVar11 * 0.5 + in_XMM1_Da * 0.5,
                                 extraout_XMM0_Da_00 * 0.5 + extraout_XMM0_Da * 0.5);
    ::utils::swizzlers::impl$1::xvy();
    ::utils::geometry::rectangle::Rectangle2d::get_edge_normal();
    local_138[0] = (float *)(in_RCX + 0x30);
    local_138[1] = (float *)resources::walls::impl$7::fmt;
    pcStack_120 = glam::f32::vec3::impl$93::fmt;
    local_e8 = (float *)&DAT_142c6e3e0;
    pfStack_e0 = (float *)0x2;
    local_d8 = local_138;
    pcStack_d0 = (code *)0x2;
    local_c8 = (float **)0x0;
    fVar12 = fVar11;
    local_138[2] = &local_f8;
    alloc::fmt::format::format_inner();
    pfStack_e0 = (float *)CONCAT44(pfStack_e0._4_4_,local_f0);
    local_e8 = (float *)CONCAT44(uStack_f4,local_f8);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    *(undefined4 *)(extraout_RAX + 2) = extraout_XMM0_Da_01;
    *(float *)((longlong)extraout_RAX + 0x14) = fVar11;
    extraout_RAX[3] = local_e8;
    *(undefined4 *)(extraout_RAX + 4) = pfStack_e0._0_4_;
    pfStack_e0 = (float *)&DAT_142c6e400;
    if (-1 < (longlong)local_178) {
      local_98 = (float *)extraout_RAX;
      if (local_178 == (float **)0x0) {
        pfVar10 = (float *)0x1;
      }
      else {
        local_a8 = 1;
        std::alloc::__default_lib_allocator::__rust_alloc();
        pfVar10 = extraout_RAX_00;
        if (extraout_RAX_00 == (float *)0x0) goto LAB_14155f340;
      }
      memcpy(pfVar10,local_180,(size_t)local_178);
      local_e8 = (float *)local_178;
      local_d8 = local_178;
      pcStack_d0 = (code *)local_98;
      local_c8 = (float **)&DAT_142c6e400;
      pfStack_e0 = pfVar10;
      append_only_vec::AppendOnlyVec::
      push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
      if (local_188 != (undefined *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      fVar7 = local_f0;
      fVar11 = local_f8;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_03 != (float *)0x0) {
        local_98 = extraout_RAX_03;
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        fVar13 = fVar12;
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        *local_98 = extraout_XMM0_Da_02 - fVar11;
        local_98[1] = fVar12 - fVar7;
        local_98[2] = extraout_XMM0_Da_03 - fVar11;
        local_98[3] = fVar13 - fVar7;
        local_138[0] = (float *)0x2;
        local_138[1] = local_98;
        local_138[2] = (float *)0x2;
        ::utils::curve::Curve::
        try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
        if (!SBORROW8(0,(longlong)local_e8)) {
          local_108 = (size_t)local_b8;
          local_118 = (char *)local_c8;
          sStack_110 = (size_t)pcStack_c0;
          local_138[2] = (float *)local_d8;
          pcStack_120 = pcStack_d0;
          local_138[0] = local_e8;
          local_138[1] = pfStack_e0;
          pfStack_e0 = (float *)CONCAT44(pfStack_e0._4_4_,local_f0);
          local_e8 = (float *)CONCAT44(uStack_f4,local_f8);
          if (!SBORROW8(0,*local_a0)) {
            if (*local_a0 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_a0[3] != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
          local_a0[6] = local_108;
          local_a0[4] = (size_t)local_118;
          local_a0[5] = sStack_110;
          local_a0[2] = (size_t)local_138[2];
          local_a0[3] = (size_t)pcStack_120;
          *local_a0 = (size_t)local_138[0];
          local_a0[1] = (size_t)local_138[1];
          local_a0[7] = sVar2;
          local_a0[8] = (size_t)local_e8;
          *(undefined4 *)(local_a0 + 9) = pfStack_e0._0_4_;
          return;
        }
        local_138[0] = pfStack_e0;
        local_138[1] = (float *)local_d8;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    local_a8 = 0;
LAB_14155f340:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if ((*in_RCX & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar3 = *(undefined8 *)(in_RCX + 0x10);
  uVar4 = *(undefined8 *)(in_RCX + 0x18);
  local_1b8 = *(float **)(in_RCX + 0x20);
  uStack_1b0 = *(undefined8 *)(in_RCX + 0x28);
  lVar9 = *in_RDX;
  if (*(char *)(lVar9 + 0x18) == '\x01') {
    local_ec = 0x3f800000;
    if ((*(longlong *)(lVar9 + 0x38) != in_RDX[1]) || (*(char *)(lVar9 + 0x19) == '\0'))
    goto LAB_14155ee47;
  }
  else {
    local_ec = 0x3f800000;
    if (((*(char *)(lVar9 + 0x18) == '\0') || (*(longlong *)(lVar9 + 0x38) != in_RDX[1])) ||
       (*(char *)(lVar9 + 0x19) == '\x02')) goto LAB_14155ee47;
  }
  local_ec = 0x3f99999a;
LAB_14155ee47:
  local_138[0] = (float *)((ulonglong)local_138[0] & 0xffffffffffffff00);
  local_138[1] = (float *)&DAT_142c6e4b0;
  local_138[2] = (float *)0xa;
  pcStack_120 = (code *)CONCAT71(pcStack_120._1_7_,1);
  local_118 = "flat_arrow_outline";
  sStack_110 = 0x12;
  local_100 = 2;
  pfVar10 = (float *)(in_RCX + 0x30);
  lVar9 = 0;
  local_148 = pfVar10;
  while( true ) {
    if (lVar9 == 2) {
      return;
    }
    local_108 = lVar9 + 1;
    cVar1 = *(char *)(local_138 + lVar9 * 3);
    if (cVar1 == '\x02') {
      return;
    }
    local_a8 = lVar9 + 1;
    local_158 = local_138[lVar9 * 3 + 1];
    pfStack_150 = local_138[lVar9 * 3 + 2];
    pfStack_e0 = (float *)resources::walls::impl$7::fmt;
    local_d8 = &local_1b8;
    pcStack_d0 = glam::f32::vec3::impl$93::fmt;
    local_c8 = &local_158;
    pcStack_c0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
    local_b8 = &local_ec;
    local_b0 = core::fmt::float::impl$5::fmt;
    local_188 = &DAT_142c6e4d0;
    local_180 = (void *)0x4;
    local_170 = 4;
    local_168 = 0;
    local_178 = &local_e8;
    local_e8 = pfVar10;
    alloc::fmt::format::format_inner();
    uVar8 = local_ec;
    pfVar6 = pfStack_150;
    pfVar5 = local_158;
    pfVar10 = local_190;
    local_98 = (float *)local_1a0;
    local_a0 = local_198;
    local_e8 = local_1b8;
    pfStack_e0 = (float *)CONCAT44(pfStack_e0._4_4_,(undefined4)uStack_1b0);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = 1;
    extraout_RAX_01[1] = 1;
    extraout_RAX_01[2] = pfVar5;
    extraout_RAX_01[3] = pfVar6;
    extraout_RAX_01[4] = uVar3;
    extraout_RAX_01[5] = uVar4;
    extraout_RAX_01[6] = local_e8;
    *(undefined4 *)(extraout_RAX_01 + 7) = pfStack_e0._0_4_;
    *(undefined4 *)((longlong)extraout_RAX_01 + 0x3c) = uVar8;
    *(char *)(extraout_RAX_01 + 8) = cVar1;
    pfStack_e0 = (float *)&DAT_142c6e510;
    if ((longlong)pfVar10 < 0) break;
    if (pfVar10 == (float *)0x0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_02;
      if (extraout_RAX_02 == (void *)0x0) break;
    }
    memcpy(_Dst,local_a0,(size_t)pfVar10);
    local_e8 = pfVar10;
    local_d8 = (float **)pfVar10;
    local_c8 = (float **)&DAT_142c6e510;
    pfStack_e0 = _Dst;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    pfVar10 = local_148;
    lVar9 = local_a8;
    if (local_98 != (float *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      lVar9 = local_a8;
    }
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

