// signifier_edit_circle_arrow  size=1139
// void __cdecl signifier_edit_circle_arrow(void)

void __cdecl
country_core::systems::wall::shapes::circle::signifiers::edit_arrow::signifier_edit_circle_arrow
          (void)

{
  uint *puVar1;
  char cVar2;
  void *_Src;
  size_t sVar3;
  undefined **ppuVar4;
  undefined *puVar5;
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong *in_RCX;
  ulonglong uVar9;
  longlong lVar10;
  longlong *in_RDX;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  uint *puVar14;
  longlong lVar15;
  longlong lVar16;
  void *_Dst;
  longlong local_110;
  code *local_108;
  longlong local_100;
  longlong local_f8;
  longlong local_f0;
  undefined8 local_e8;
  undefined4 local_e0;
  longlong local_d8;
  size_t *local_d0;
  longlong local_c8;
  longlong local_c0;
  undefined **local_b8;
  undefined *puStack_b0;
  longlong *local_a8;
  undefined8 *local_a0;
  undefined *local_98;
  longlong local_88;
  undefined4 local_80;
  undefined4 uStack_7c;
  size_t local_78;
  undefined8 *local_70;
  undefined8 *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  lVar15 = *in_RDX;
  if (*(char *)(lVar15 + 0x38) != '\x02') {
    local_80 = *(undefined4 *)(lVar15 + 0x34);
    local_88 = *(longlong *)(lVar15 + 0x2c);
    ::utils::rotation_from_normal();
    puVar5 = puStack_b0;
    ppuVar4 = local_b8;
    local_e0 = *(undefined4 *)(lVar15 + 0x28);
    local_e8 = *(undefined8 *)(lVar15 + 0x20);
    local_108 = systems::ui::resources::ui_state::impl$15::fmt;
    local_b8 = &PTR_s_edit_circle__arrow_142aed868;
    puStack_b0 = (undefined *)0x2;
    local_a8 = &local_110;
    local_a0 = (undefined8 *)0x1;
    local_98 = (undefined *)0x0;
    local_110 = lVar15;
    alloc::fmt::format::format_inner();
    _Src = (void *)CONCAT44(uStack_7c,local_80);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    extraout_RAX[2] = ppuVar4;
    extraout_RAX[3] = puVar5;
    extraout_RAX[4] = local_e8;
    *(undefined4 *)(extraout_RAX + 5) = local_e0;
    puStack_b0 = &DAT_142aed888;
    if ((longlong)local_78 < 0) {
      local_68 = (undefined8 *)0x0;
LAB_140b3558e:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_70 = extraout_RAX;
    if (local_78 == 0) {
      _Dst = (void *)0x1;
    }
    else {
      local_68 = (undefined8 *)0x1;
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (void *)0x0) goto LAB_140b3558e;
    }
    memcpy(_Dst,_Src,local_78);
    local_b8 = (undefined **)local_78;
    local_a8 = (longlong *)local_78;
    local_a0 = local_70;
    local_98 = &DAT_142aed888;
    puStack_b0 = _Dst;
    append_only_vec::AppendOnlyVec::
    push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
    if (local_88 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  local_c8 = *in_RCX;
  lVar13 = in_RCX[1];
  local_70 = (undefined8 *)CONCAT44(local_70._4_4_,*(undefined4 *)((longlong)in_RCX + 0x14));
  puVar14 = *(uint **)(local_c8 + 0xf8);
  puVar1 = puVar14 + *(longlong *)(local_c8 + 0x100);
  local_68 = (undefined8 *)(lVar15 + 0x2c);
  local_d0 = (size_t *)(lVar15 + 0x20);
  local_d8 = lVar15;
  if (*(char *)(local_c8 + 0x108) == '\0') {
    local_c0 = 8;
    lVar11 = 0;
    lVar16 = 0;
    local_100 = 0;
    do {
      if (local_100 == lVar16) {
        if (puVar14 == puVar1) {
          return;
        }
        while( true ) {
          lVar16 = *(longlong *)(*(longlong *)(lVar13 + 0x100) + 0x58 + (ulonglong)*puVar14 * 0xa0);
          if (lVar16 != 0) break;
          puVar14 = puVar14 + 1;
          if (puVar14 == puVar1) {
            return;
          }
        }
        lVar8 = *(longlong *)(lVar13 + 0x100) + (ulonglong)*puVar14 * 0xa0;
        uVar9 = (ulonglong)*(uint *)(lVar8 + 0x94);
        lVar6 = *(longlong *)(*(longlong *)(lVar13 + 0x1a8) + 0x18 + uVar9 * 0x48);
        lVar10 = ~*(ulonglong *)
                   (*(longlong *)(*(longlong *)(lVar13 + 0x1a8) + 0x38 + uVar9 * 0x48) +
                   *(longlong *)(local_c8 + 0x118) * 8) * 0x30;
        lVar11 = *(longlong *)(lVar6 + 0x10 + lVar10);
        local_f0 = *(longlong *)(lVar6 + 0x28 + lVar10);
        local_c0 = *(longlong *)(lVar8 + 0x50);
        puVar14 = puVar14 + 1;
        lVar6 = 0;
        cVar2 = *(char *)(lVar15 + 0x38);
        lVar8 = 1;
      }
      else {
        cVar2 = *(char *)(lVar15 + 0x38);
        lVar8 = local_100 + 1;
        lVar6 = local_100;
      }
      local_100 = lVar8;
      if (cVar2 != '\x02') {
        uVar9 = (ulonglong)*(uint *)(local_c0 + 8 + lVar6 * 0x10);
        lVar15 = uVar9 * 0x30;
        local_80 = *(undefined4 *)(local_68 + 1);
        local_88 = *local_68;
        ::utils::rotation_from_normal();
        *(undefined4 *)(local_f0 + uVar9 * 4) = local_70._0_4_;
        *(size_t *)(lVar11 + lVar15) = (size_t)local_b8;
        ((size_t *)(lVar11 + lVar15))[1] = (size_t)puStack_b0;
        sVar3 = local_d0[1];
        puStack_b0 = (undefined *)CONCAT44(puStack_b0._4_4_,(int)sVar3);
        local_b8 = (undefined **)*local_d0;
        *(undefined4 *)(local_f0 + uVar9 * 4) = local_70._0_4_;
        *(int *)(lVar11 + 0x18 + lVar15) = (int)sVar3;
        *(undefined ***)(lVar11 + 0x10 + lVar15) = local_b8;
        lVar15 = local_d8;
      }
    } while( true );
  }
  lVar16 = 0;
  uVar12 = 0;
  uVar9 = 0;
  local_f8 = lVar13;
LAB_140b35300:
  do {
    uVar7 = uVar9;
    if (uVar7 != uVar12) goto LAB_140b35340;
    if (puVar14 == puVar1) {
      return;
    }
    while( true ) {
      uVar12 = *(ulonglong *)(*(longlong *)(lVar13 + 0x1a8) + 0x10 + (ulonglong)*puVar14 * 0x48);
      if (uVar12 != 0) break;
      puVar14 = puVar14 + 1;
      if (puVar14 == puVar1) {
        return;
      }
    }
    lVar16 = *(longlong *)(lVar13 + 0x1a8) + (ulonglong)*puVar14 * 0x48;
    lVar11 = *(longlong *)(lVar16 + 0x18);
    lVar6 = ~*(ulonglong *)(*(longlong *)(lVar16 + 0x38) + *(longlong *)(local_c8 + 0x118) * 8) *
            0x30;
    lVar16 = *(longlong *)(lVar11 + 0x10 + lVar6);
    local_c0 = *(longlong *)(lVar11 + 0x28 + lVar6);
    puVar14 = puVar14 + 1;
    uVar7 = 0;
    uVar9 = 1;
  } while (*(char *)(lVar15 + 0x38) == '\x02');
  goto LAB_140b35399;
LAB_140b35340:
  uVar9 = uVar7 + 1;
  if (*(char *)(lVar15 + 0x38) != '\x02') {
LAB_140b35399:
    uVar7 = uVar7 & 0xffffffff;
    lVar15 = uVar7 * 0x30;
    local_80 = *(undefined4 *)(local_68 + 1);
    local_88 = *local_68;
    ::utils::rotation_from_normal();
    *(undefined4 *)(local_c0 + uVar7 * 4) = local_70._0_4_;
    *(size_t *)(lVar16 + lVar15) = (size_t)local_b8;
    ((size_t *)(lVar16 + lVar15))[1] = (size_t)puStack_b0;
    sVar3 = local_d0[1];
    puStack_b0 = (undefined *)CONCAT44(puStack_b0._4_4_,(int)sVar3);
    local_b8 = (undefined **)*local_d0;
    *(undefined4 *)(local_c0 + uVar7 * 4) = local_70._0_4_;
    *(int *)(lVar16 + 0x18 + lVar15) = (int)sVar3;
    *(undefined ***)(lVar16 + 0x10 + lVar15) = local_b8;
    lVar13 = local_f8;
    lVar15 = local_d8;
  }
  goto LAB_140b35300;
}



// CALLEES: [{'name': '__rust_alloc', 'address': '140613c10'}, {'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'format_inner', 'address': '140d79f90'}, {'name': 'handle_alloc_error', 'address': '1428d9000'}, {'name': 'handle_error', 'address': '1428d8fe3'}, {'name': 'memcpy', 'address': '142923c00'}, {'name': 'push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>', 'address': '14146c880'}, {'name': 'rotation_from_normal', 'address': '140c90a40'}]