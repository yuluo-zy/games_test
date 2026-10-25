
void __cdecl
country_core::systems::wall::shapes::rectangle::ui_modes::edit_orientation::
UiRectangleOrientationEdit::signifier(void)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  uint uVar4;
  size_t _Size;
  uint uVar5;
  longlong lVar6;
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  longlong in_RCX;
  longlong *in_RDX;
  void *_Dst;
  uint extraout_XMM0_Da;
  uint extraout_XMM0_Da_00;
  undefined4 in_XMM1_Da;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  longlong local_188;
  code *local_180;
  undefined8 *local_178;
  code *local_170;
  undefined4 *local_168;
  code *local_160;
  longlong local_148;
  void *local_140;
  size_t local_138;
  uint local_128 [2];
  undefined8 uStack_120;
  undefined8 local_118;
  undefined1 local_110;
  char *local_108;
  undefined8 local_100;
  longlong local_f8;
  undefined8 local_f0;
  undefined8 local_e4;
  undefined4 local_dc;
  undefined8 *local_d8;
  longlong local_d0;
  undefined8 local_c8;
  undefined4 local_c0;
  undefined *local_b8;
  undefined *local_b0;
  longlong *local_a8;
  undefined8 *local_a0;
  undefined *local_98;
  void *local_88;
  longlong local_80;
  undefined4 local_74;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  local_d0 = in_RCX;
  if (*(char *)(in_RCX + 0x2c) == '\0') {
    lVar6 = *in_RDX;
    if (*(char *)(lVar6 + 0x18) == '\0') {
      return;
    }
    if (*(longlong *)(lVar6 + 0x38) != in_RDX[1]) {
      return;
    }
    if (*(int *)(lVar6 + 0x28) == 2) {
      return;
    }
    ::utils::geometry::rectangle::Rectangle2d::get_point();
    ::utils::swizzlers::impl$1::xvy();
    ::utils::geometry::rectangle::Rectangle2d::get_corner_normal();
    local_128[0] = extraout_XMM0_Da_00;
    local_128[1] = in_XMM1_Da;
    ::utils::swizzlers::impl$0::x0y();
    ::utils::rotation_from_normal();
    uVar9 = 0x3f19999a;
    uVar7 = (undefined4)uStack_120;
    uVar8 = uStack_120._4_4_;
    goto LAB_14083db58;
  }
  ::utils::geometry::rectangle::Rectangle2d::get_point();
  ::utils::swizzlers::impl$1::xvy();
  ::utils::geometry::rectangle::Rectangle2d::get_corner_normal();
  local_128[0] = extraout_XMM0_Da;
  local_128[1] = in_XMM1_Da;
  ::utils::swizzlers::impl$0::x0y();
  lVar6 = *in_RDX;
  if (*(char *)(lVar6 + 0x18) == '\x01') {
    uVar9 = 0x3ef5c290;
    if ((*(longlong *)(lVar6 + 0x38) == in_RDX[1]) && (*(char *)(lVar6 + 0x19) != '\0')) {
LAB_14083db3b:
      uVar9 = 0x3f1374bd;
    }
  }
  else {
    uVar9 = 0x3ef5c290;
    if (((*(char *)(lVar6 + 0x18) != '\0') && (*(longlong *)(lVar6 + 0x38) == in_RDX[1])) &&
       (*(char *)(lVar6 + 0x19) != '\x02')) goto LAB_14083db3b;
  }
  ::utils::rotation_from_normal();
  uVar7 = (undefined4)uStack_120;
  uVar8 = uStack_120._4_4_;
LAB_14083db58:
  uVar5 = local_128[0];
  local_c0 = local_dc;
  local_c8 = local_e4;
  local_128[0] = local_128[0] & 0xffffff00;
  uStack_120 = &DAT_142a93950;
  local_118 = 0xc;
  local_110 = 1;
  local_108 = "corner_arrow_outlinerectangle__rotate_";
  local_100 = 0x14;
  local_f0 = 2;
  lVar6 = 0;
  local_74 = uVar9;
  while( true ) {
    if (lVar6 == 2) {
      return;
    }
    lVar1 = lVar6 + 1;
    uVar4 = local_128[lVar6 * 6];
    if ((char)uVar4 == '\x02') {
      return;
    }
    uVar2 = (&uStack_120)[lVar6 * 3];
    uVar3 = (&local_118)[lVar6 * 3];
    local_188 = local_d0;
    local_180 = resources::walls::impl$7::fmt;
    local_178 = &local_c8;
    local_170 = glam::f32::vec3::impl$93::fmt;
    local_168 = &local_74;
    local_160 = core::fmt::float::impl$5::fmt;
    local_b8 = &DAT_142a93988;
    local_b0 = (undefined *)0x3;
    local_a8 = &local_188;
    local_a0 = (undefined8 *)0x3;
    local_98 = (undefined *)0x0;
    local_f8 = lVar1;
    alloc::fmt::format::format_inner();
    uVar9 = local_74;
    _Size = local_138;
    local_80 = local_148;
    local_88 = local_140;
    local_b8 = (undefined *)local_c8;
    local_b0 = (undefined *)CONCAT44(local_b0._4_4_,local_c0);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    extraout_RAX[2] = uVar2;
    extraout_RAX[3] = uVar3;
    *(uint *)(extraout_RAX + 4) = uVar5;
    *(uint *)((longlong)extraout_RAX + 0x24) = local_128[1];
    *(undefined4 *)(extraout_RAX + 5) = uVar7;
    *(undefined4 *)((longlong)extraout_RAX + 0x2c) = uVar8;
    *(undefined4 *)(extraout_RAX + 6) = uVar9;
    *(undefined **)((longlong)extraout_RAX + 0x34) = local_b8;
    *(undefined4 *)((longlong)extraout_RAX + 0x3c) = local_b0._0_4_;
    *(char *)(extraout_RAX + 8) = (char)uVar4;
    local_b0 = &DAT_142a939b8;
    if ((longlong)_Size < 0) break;
    local_d8 = extraout_RAX;
    if (_Size == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (void *)0x0) break;
    }
    memcpy(_Dst,local_88,_Size);
    local_b8 = (undefined *)_Size;
    local_a8 = (longlong *)_Size;
    local_a0 = local_d8;
    local_98 = &DAT_142a939b8;
    local_b0 = _Dst;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    lVar6 = lVar1;
    if (local_80 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

