/* ui_delete_tree @0x0x141bd2510 size 1667 */

void __cdecl system_tree::ui_delete_tree::ui_delete_tree(void)

{
  undefined4 uVar1;
  undefined1 auVar2 [16];
  undefined1 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  ulonglong *puVar7;
  undefined4 *puVar8;
  size_t sVar9;
  undefined4 *puVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  ushort uVar13;
  int iVar14;
  undefined8 uVar15;
  char extraout_AL;
  uint uVar16;
  longlong extraout_RAX;
  void *_Dst;
  undefined8 *extraout_RAX_00;
  void *_Src;
  void *extraout_RAX_01;
  undefined8 *in_RCX;
  longlong lVar17;
  size_t _Size;
  longlong *in_R8;
  longlong lVar18;
  ulonglong uVar19;
  undefined8 *in_R9;
  ulonglong uVar20;
  longlong lVar21;
  longlong lVar22;
  ulonglong uVar23;
  longlong lVar24;
  longlong lVar25;
  undefined8 *puVar26;
  ulonglong uVar27;
  longlong lVar28;
  uint uVar29;
  bool bVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float unaff_XMM7_Da;
  float unaff_XMM8_Da;
  size_t *in_stack_00000028;
  longlong in_stack_00000040;
  undefined8 *in_stack_00000050;
  longlong local_110;
  undefined1 (*local_108) [16];
  ushort local_f8;
  longlong local_f0;
  float local_e8;
  float local_e0;
  
  if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE != '\0') {
    return;
  }
  fVar33 = *(float *)*in_stack_00000050;
  bVar30 = false;
  if ((*(int *)*in_RCX == 9) && (bVar30 = false, (char)((int *)*in_RCX)[1] == '\x06')) {
    local_108 = (undefined1 (*) [16])&DAT_142ef5fe9;
    country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer();
    country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
    bVar30 = extraout_AL != '\x02';
    local_110 = extraout_RAX;
    if (bVar30) {
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      unaff_XMM7_Da = local_e8;
      unaff_XMM8_Da = local_e0;
    }
  }
  fVar33 = fVar33 * 3.0;
  puVar7 = (ulonglong *)*in_R9;
  lVar22 = in_R9[1];
  uVar23 = *puVar7;
  uVar20 = 0;
  uVar19 = uVar23 - *(ulonglong *)(lVar22 + 0x18);
  if (uVar23 < *(ulonglong *)(lVar22 + 0x18)) {
    uVar19 = uVar20;
  }
  uVar27 = uVar23 - *(ulonglong *)(lVar22 + 0x38);
  if (uVar23 < *(ulonglong *)(lVar22 + 0x38)) {
    uVar27 = uVar20;
  }
  lVar21 = uVar19 * 0x20 + *(longlong *)(lVar22 + 8);
  uVar23 = *(ulonglong *)(lVar22 + 0x10) - uVar19;
  if (*(ulonglong *)(lVar22 + 0x10) < uVar19) {
    lVar21 = 8;
    uVar23 = uVar20;
  }
  lVar24 = uVar27 * 0x20 + *(longlong *)(lVar22 + 0x28);
  uVar19 = *(ulonglong *)(lVar22 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar22 + 0x30) < uVar27) {
    lVar24 = 8;
    uVar19 = uVar20;
  }
  uVar20 = (*(longlong *)(lVar22 + 0x40) - uVar19) - uVar23;
  *puVar7 = uVar20;
  lVar18 = uVar19 * 0x20 + lVar24;
  lVar22 = lVar21;
  do {
    if ((lVar22 == 0) || (lVar22 == uVar23 * 0x20 + lVar21)) {
      if ((lVar24 == 0) || (lVar24 == lVar18)) break;
      lVar17 = 0;
      lVar25 = lVar24 + 0x20;
      lVar28 = lVar24;
    }
    else {
      lVar17 = lVar22 + 0x20;
      lVar25 = lVar24;
      lVar28 = lVar22;
    }
    lVar24 = lVar25;
    uVar20 = uVar20 + 1;
    *puVar7 = uVar20;
    lVar22 = lVar17;
    if (*(char *)(lVar28 + 0x18) == '\0') {
      unaff_XMM7_Da = *(float *)(lVar28 + 8);
      unaff_XMM8_Da = *(float *)(lVar28 + 0x10);
      fVar33 = *(float *)(lVar28 + 0x14) + *(float *)(lVar28 + 0x14);
      bVar30 = true;
    }
  } while( true );
  if ((bVar30) &&
     (country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids(),
     local_f0 != 0)) {
    lVar22 = *in_R8;
    lVar21 = in_R8[1];
    uVar4 = *(undefined4 *)(in_stack_00000040 + 0x1c);
    puVar8 = *(undefined4 **)(in_stack_00000040 + 0x10);
    uVar5 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    sVar9 = *in_stack_00000028;
    puVar10 = (undefined4 *)in_stack_00000028[2];
    uVar16 = (uint)local_f8;
    do {
      if ((short)uVar16 == 0) {
        do {
          auVar2 = *local_108;
          uVar13 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar2[0xf] >> 7) << 0xf;
          local_110 = local_110 + -0x100;
          local_108 = local_108 + 1;
        } while (uVar13 == 0xffff);
        uVar16 = ~(uint)uVar13;
        uVar29 = uVar16 & -(uint)uVar13 - 2;
      }
      else {
        uVar29 = uVar16 - 1 & uVar16;
        if (local_110 == 0) {
          return;
        }
      }
      local_f0 = local_f0 + -1;
      iVar14 = 0;
      for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
        iVar14 = iVar14 + 1;
      }
      lVar24 = local_110 - (ulonglong)(uint)(iVar14 << 4);
      uVar23 = *(ulonglong *)(lVar24 + -8);
      uVar19 = uVar23 & 0xffffffff;
      if ((*(ulonglong *)(lVar21 + 0x10) <= uVar19) ||
         (*(int *)(*(longlong *)(lVar21 + 8) + uVar19 * 0x14) != (int)(uVar23 >> 0x20))) {
LAB_141bd2b61:
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      lVar18 = *(longlong *)(lVar21 + 8) + uVar19 * 0x14;
      uVar23 = (ulonglong)*(uint *)(lVar18 + 4);
      if ((uVar23 == 0xffffffff) ||
         ((*(ulonglong *)(lVar22 + 0x38) <= uVar23 ||
          ((*(ulonglong *)
             (*(longlong *)(lVar22 + 0x28) + (ulonglong)(*(uint *)(lVar18 + 4) >> 6) * 8) >>
            (uVar23 & 0x3f) & 1) == 0)))) goto LAB_141bd2b61;
      lVar17 = *(longlong *)(lVar21 + 0x1a8) + (ulonglong)*(uint *)(lVar18 + 0xc) * 0x48;
      uVar23 = *(ulonglong *)
                (*(longlong *)(lVar21 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar18 + 0xc) * 0x48);
      if ((*(ulonglong *)(lVar22 + 0x110) < uVar23) &&
         (uVar19 = *(ulonglong *)(*(longlong *)(lVar17 + 0x38) + *(ulonglong *)(lVar22 + 0x110) * 8)
         , uVar19 != 0)) {
        lVar25 = *(longlong *)(*(longlong *)(lVar17 + 0x18) + 0x10 + ~uVar19 * 0x30);
        uVar19 = *(ulonglong *)(lVar22 + 0x118);
        if (uVar23 <= uVar19) goto LAB_141bd29a0;
LAB_141bd296f:
        if (*(longlong *)(*(longlong *)(lVar17 + 0x38) + uVar19 * 8) == 0) goto LAB_141bd29a0;
        uVar19 = *(ulonglong *)(lVar22 + 0x120);
        if (uVar19 < uVar23) goto LAB_141bd29af;
LAB_141bd29e0:
        lVar17 = 0;
      }
      else {
        lVar25 = 0;
        uVar19 = *(ulonglong *)(lVar22 + 0x118);
        if (uVar19 < uVar23) goto LAB_141bd296f;
LAB_141bd29a0:
        uVar19 = *(ulonglong *)(lVar22 + 0x120);
        if (uVar23 <= uVar19) goto LAB_141bd29e0;
LAB_141bd29af:
        uVar23 = *(ulonglong *)(*(longlong *)(lVar17 + 0x38) + uVar19 * 8);
        if (uVar23 == 0) goto LAB_141bd29e0;
        lVar17 = *(longlong *)(*(longlong *)(lVar17 + 0x18) + 0x10 + ~uVar23 * 0x30);
      }
      uVar16 = *(uint *)(lVar18 + 0x10);
      lVar18 = (ulonglong)uVar16 * 0x30;
      fVar31 = unaff_XMM7_Da - *(float *)(lVar25 + lVar18);
      fVar32 = unaff_XMM8_Da - *(float *)(lVar25 + 8 + lVar18);
      if (SQRT(fVar32 * fVar32 + fVar31 * fVar31) < fVar33) {
        uVar11 = *(undefined8 *)(lVar24 + -0x10);
        *puVar10 = uVar5;
        lVar24 = *(longlong *)(sVar9 + 0x30);
        uVar12 = *(undefined8 *)(sVar9 + 0x40);
        if (lVar24 == *(longlong *)(sVar9 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        puVar26 = (undefined8 *)(lVar25 + lVar18);
        lVar18 = *(longlong *)(sVar9 + 0x28);
        *(undefined8 *)(lVar18 + lVar24 * 0x10) = uVar12;
        *(undefined8 *)(lVar18 + 8 + lVar24 * 0x10) = uVar11;
        *(longlong *)(sVar9 + 0x30) = lVar24 + 1;
        *(longlong *)(sVar9 + 0x40) = *(longlong *)(sVar9 + 0x40) + 1;
        *puVar8 = uVar4;
        uVar12 = *puVar26;
        uVar15 = puVar26[1];
        uVar1 = *(undefined4 *)(puVar26 + 2);
        uVar6 = *(undefined4 *)((longlong)puVar26 + 0x14);
        _Size = sVar9;
        asset::prefab::PrefabName::as_name_ref();
        if ((longlong)_Size < 0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        if (_Size == 0) {
          _Dst = (void *)0x1;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          _Dst = extraout_RAX_01;
          if (extraout_RAX_01 == (void *)0x0) {
                    // WARNING: Subroutine does not return
            alloc::raw_vec::handle_error();
          }
        }
        memcpy(_Dst,_Src,_Size);
        uVar3 = *(undefined1 *)(lVar17 + 8 + (ulonglong)uVar16 * 0x10);
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_00 = 1;
        extraout_RAX_00[1] = 1;
        *(undefined4 *)(extraout_RAX_00 + 2) = 2;
        extraout_RAX_00[3] = uVar11;
        extraout_RAX_00[4] = _Size;
        extraout_RAX_00[5] = _Dst;
        extraout_RAX_00[6] = _Size;
        extraout_RAX_00[7] = uVar12;
        extraout_RAX_00[8] = uVar15;
        *(undefined4 *)(extraout_RAX_00 + 9) = uVar1;
        *(undefined4 *)((longlong)extraout_RAX_00 + 0x4c) = uVar6;
        *(undefined1 *)(extraout_RAX_00 + 10) = uVar3;
        country_core::resources::history::History::add();
      }
      uVar16 = uVar29;
    } while (local_f0 != 0);
  }
  return;
}

