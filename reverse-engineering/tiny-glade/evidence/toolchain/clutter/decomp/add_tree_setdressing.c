
void __cdecl
system_clutter::autoclutter::add_tree_vegetation_setdressing::add_tree_setdressing(void)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  bool bVar7;
  float _Y;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char cVar12;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  longlong extraout_RAX;
  undefined8 *puVar13;
  longlong lVar14;
  ulonglong uVar15;
  ulonglong extraout_RAX_00;
  longlong *in_RCX;
  ulonglong uVar16;
  longlong lVar17;
  undefined8 *puVar18;
  uint *puVar19;
  longlong *in_R8;
  longlong lVar20;
  float extraout_XMM0_Da;
  float fVar21;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar22;
  float fVar23;
  longlong *in_stack_00000028;
  longlong local_218;
  longlong local_210;
  undefined4 local_208;
  float fStack_204;
  undefined4 local_200;
  undefined2 local_1fb;
  undefined1 local_1f9;
  undefined8 local_1e0;
  undefined4 local_1d8;
  undefined4 local_1d4;
  undefined **local_1d0;
  longlong lStack_1c8;
  undefined8 **local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  longlong local_1a0;
  ulonglong local_198;
  undefined8 *local_190;
  longlong local_188;
  undefined8 local_180;
  undefined4 local_178;
  undefined8 *local_170;
  uint *local_168;
  longlong local_160;
  ulonglong local_158;
  undefined8 *local_150;
  undefined8 *local_148;
  longlong local_140;
  undefined4 *local_138;
  undefined8 *local_128;
  code *pcStack_120;
  undefined8 local_114;
  undefined4 local_10c;
  undefined8 local_108;
  undefined8 local_100;
  
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  fVar11 = __real_3f4ccccd;
  fVar10 = __real_41400000;
  fVar9 = __real_3f99999a;
  fVar8 = __real_3dcccccd;
  _Y = __real_3f800000;
  puVar18 = (undefined8 *)(extraout_RAX + 0x20);
  puVar13 = (undefined8 *)(extraout_RAX + 0x28);
  if (*(char *)(extraout_RAX + 0x298) == '\0') {
    puVar13 = (undefined8 *)(extraout_RAX + 0x10);
    puVar18 = (undefined8 *)(extraout_RAX + 8);
  }
  local_100 = *puVar13;
  local_108 = *puVar18;
  lVar4 = *in_stack_00000028;
  local_160 = in_stack_00000028[1];
  puVar19 = *(uint **)(lVar4 + 0xf8);
  local_168 = puVar19 + *(longlong *)(lVar4 + 0x100);
  cVar1 = *(char *)(lVar4 + 0x108);
  lVar20 = *in_R8;
  local_148 = (undefined8 *)(lVar20 + 0x100);
  local_170 = (undefined8 *)(lVar20 + 0x108);
  local_150 = (undefined8 *)(lVar20 + 0x118);
  local_190 = (undefined8 *)(lVar20 + 0x110);
  local_1d4 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_188 = *in_RCX;
  local_138 = (undefined4 *)in_RCX[2];
  local_140 = local_188 + 0x48;
  local_1a0 = 8;
  local_210 = 0;
  local_218 = 0;
  local_198 = 0;
  uVar16 = 0;
  do {
    if (cVar1 == '\0') {
      if (uVar16 == local_198) {
        do {
          if (puVar19 == local_168) {
            return;
          }
          uVar2 = *puVar19;
          puVar19 = puVar19 + 1;
          local_198 = *(ulonglong *)
                       (*(longlong *)(local_160 + 0x100) + 0x58 + (ulonglong)uVar2 * 0xa0);
        } while (local_198 == 0);
        lVar14 = *(longlong *)(local_160 + 0x100) + (ulonglong)uVar2 * 0xa0;
        uVar16 = (ulonglong)*(uint *)(lVar14 + 0x94);
        lVar20 = *(longlong *)(local_160 + 0x1a8) + uVar16 * 0x48;
        uVar16 = *(ulonglong *)(*(longlong *)(local_160 + 0x1a8) + 0x40 + uVar16 * 0x48);
        if ((*(ulonglong *)(lVar4 + 0x110) < uVar16) &&
           (uVar15 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8),
           uVar15 != 0)) {
          local_210 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar15 * 0x30);
          uVar15 = *(ulonglong *)(lVar4 + 0x118);
          if (uVar16 <= uVar15) goto LAB_1422d3bf0;
LAB_1422d3baf:
          uVar16 = *(ulonglong *)(*(longlong *)(lVar20 + 0x38) + uVar15 * 8);
          if (uVar16 == 0) goto LAB_1422d3bf0;
          local_218 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar16 * 0x30);
        }
        else {
          local_210 = 0;
          uVar15 = *(ulonglong *)(lVar4 + 0x118);
          if (uVar15 < uVar16) goto LAB_1422d3baf;
LAB_1422d3bf0:
          local_218 = 0;
        }
        local_1a0 = *(longlong *)(lVar14 + 0x50);
        uVar16 = 0;
      }
      uVar15 = (ulonglong)*(uint *)(local_1a0 + 8 + uVar16 * 0x10);
    }
    else {
      if (uVar16 == local_198) {
        do {
          if (puVar19 == local_168) {
            return;
          }
          uVar2 = *puVar19;
          puVar19 = puVar19 + 1;
          local_198 = *(ulonglong *)
                       (*(longlong *)(local_160 + 0x1a8) + 0x10 + (ulonglong)uVar2 * 0x48);
        } while (local_198 == 0);
        lVar20 = *(longlong *)(local_160 + 0x1a8) + (ulonglong)uVar2 * 0x48;
        uVar16 = *(ulonglong *)(lVar20 + 0x40);
        if ((*(ulonglong *)(lVar4 + 0x110) < uVar16) &&
           (uVar15 = *(ulonglong *)
                      (*(longlong *)(lVar20 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8),
           uVar15 != 0)) {
          local_210 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar15 * 0x30);
          uVar15 = *(ulonglong *)(lVar4 + 0x118);
          if (uVar16 <= uVar15) goto LAB_1422d3bd7;
LAB_1422d3b6d:
          uVar16 = *(ulonglong *)(*(longlong *)(lVar20 + 0x38) + uVar15 * 8);
          if (uVar16 == 0) goto LAB_1422d3bd7;
          local_218 = *(longlong *)(*(longlong *)(lVar20 + 0x18) + 0x10 + ~uVar16 * 0x30);
        }
        else {
          local_210 = 0;
          uVar15 = *(ulonglong *)(lVar4 + 0x118);
          if (uVar15 < uVar16) goto LAB_1422d3b6d;
LAB_1422d3bd7:
          local_218 = 0;
        }
        uVar16 = 0;
      }
      uVar15 = uVar16 & 0xffffffff;
    }
    country_core::systems::ui::resources::ui_state::ActiveWidget::is_dragging();
    if (extraout_AL == '\0') {
      country_core::resources::placement_data::PlacementDataSsbo::position();
      country_core::systems::ui::resources::ui_state::DragState::is_started();
      cVar12 = extraout_AL_01;
joined_r0x0001422d3cd7:
      if ((cVar12 == '\0') &&
         (country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted(),
         extraout_AL_02 == '\0')) {
        bVar7 = false;
      }
      else {
        bVar7 = false;
      }
    }
    else {
      iVar3 = *(int *)(uVar15 * 0x30 + local_218 + 0x2c);
      country_core::resources::placement_data::PlacementDataSsbo::position();
      bVar7 = true;
      if (iVar3 != 2) {
        country_core::systems::ui::resources::ui_state::DragState::is_started();
        cVar12 = extraout_AL_00;
        goto joined_r0x0001422d3cd7;
      }
    }
    local_158 = uVar16 + 1;
    ::utils::geometry::circle::Circle2d::new();
    ::utils::rng::init_rng();
    lVar20 = 0;
    do {
      fVar22 = (float)lVar20 / fVar10;
      fastrand::Rng::f32();
      fVar21 = extraout_XMM0_Da * fVar8 + fVar22;
      if (NAN(fVar21)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      fVar21 = fmodf(fVar21,_Y);
      fVar21 = (float)(~-(uint)(fVar21 < 0.0) & (uint)fVar21 |
                      (uint)(fVar21 + _Y) & -(uint)(fVar21 < 0.0));
      local_1e0 = CONCAT44(local_1e0._4_4_,fVar21);
      if ((fVar21 < 0.0) || (_Y < fVar21)) {
        local_128 = &local_1e0;
        pcStack_120 = core::fmt::float::impl$5::fmt;
        local_1d0 = &anon_f6c43554dafde66f7c733bfe9702e96f_3_llvm_13471928359151186412;
        lStack_1c8 = 1;
        local_1b0 = 0;
        local_1c0 = &local_128;
        local_1b8 = 1;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      if (NAN(fVar21)) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic();
      }
      ::utils::geometry::circle::Circle2d::get_pos_at_u();
      fastrand::Rng::f32();
      fastrand::Rng::f32();
      if ((!bVar7) &&
         (country_core::systems::ui::resources::ui_state::DragState::is_started(),
         extraout_AL_03 == '\0')) {
        country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
      }
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      if (0.0 < extraout_XMM0_Da_00) {
        fastrand::Rng::f32();
        fastrand::Rng::f32();
        fVar21 = _Y / SQRT(extraout_XMM0_Da_02 * extraout_XMM0_Da_02 +
                           extraout_XMM0_Da_01 * extraout_XMM0_Da_01);
        local_1d0 = (undefined **)
                    CONCAT44(fVar21 * extraout_XMM0_Da_02,fVar21 * extraout_XMM0_Da_01);
        ::utils::swizzlers::impl$0::x0y();
        ::utils::swizzlers::impl$0::x0y();
        local_1d0 = *(undefined ***)(uVar15 * 0x10 + local_210);
        local_1c0 = (undefined8 **)&DAT_143042d91;
        local_1b8 = 4;
        lStack_1c8 = lVar20;
        ::utils::
        calculate_hash<tuple_<country_core::components::TreeStorageIndex,usize,ref_<str_>_>_>();
        country_core::systems::ui::resources::ui_state::DragState::is_started();
        fVar21 = fVar11;
        if ((extraout_AL_04 == '\0') &&
           (country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted(),
           extraout_AL_05 == '\0')) {
          if (bVar7) {
            puVar18 = local_150;
            fVar23 = fVar22 * __real_bd4ccccd + fStack_204;
          }
          else {
            puVar18 = local_148;
            fVar23 = __real_bf4ccccd;
            if ((extraout_RAX_00 & 1) == 0) {
              puVar18 = local_170;
              fVar21 = __real_3fa66666;
              fVar23 = fVar22 * __real_bd4ccccd + fStack_204;
            }
          }
        }
        else {
          fVar23 = fVar22 * __real_bd4ccccd + fStack_204;
          country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
          puVar18 = local_190;
          if (extraout_AL_06 != '\0') {
            fVar23 = fVar23 + __real_bdcccccd;
          }
        }
        fStack_204 = fVar23;
        uVar6 = *puVar18;
        *local_138 = local_1d4;
        fastrand::Rng::f32();
        local_1e0 = CONCAT44(fStack_204,local_208);
        local_1d8 = local_200;
        local_178 = local_10c;
        local_180 = local_114;
        lVar14 = *(longlong *)(local_188 + 0x58);
        if (lVar14 == *(longlong *)(local_188 + 0x48)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>
                    ();
        }
        lVar5 = *(longlong *)(local_188 + 0x50);
        lVar17 = lVar14 * 0x68;
        *(undefined8 *)(lVar5 + lVar17) = 5;
        puVar18 = (undefined8 *)(lVar5 + 8 + lVar17);
        *puVar18 = local_128;
        puVar18[1] = pcStack_120;
        *(undefined4 *)(lVar5 + 0x18 + lVar17) = 2;
        puVar18 = (undefined8 *)(lVar5 + 0x1c + lVar17);
        *puVar18 = local_1d0;
        puVar18[1] = lStack_1c8;
        *(undefined4 *)(lVar5 + 0x2c + lVar17) = local_1c0._0_4_;
        *(undefined1 *)(lVar5 + 0x30 + lVar17) = 5;
        *(undefined2 *)(lVar5 + 0x31 + lVar17) = local_1fb;
        *(undefined1 *)(lVar5 + 0x33 + lVar17) = local_1f9;
        *(undefined4 *)(lVar5 + 0x34 + lVar17) = 0;
        *(undefined8 *)(lVar5 + 0x38 + lVar17) = uVar6;
        *(ulonglong *)(lVar5 + 0x40 + lVar17) = extraout_RAX_00;
        *(undefined8 *)(lVar5 + 0x48 + lVar17) = local_1e0;
        *(undefined4 *)(lVar5 + 0x50 + lVar17) = local_1d8;
        *(undefined8 *)(lVar5 + 0x54 + lVar17) = local_180;
        *(undefined4 *)(lVar5 + 0x5c + lVar17) = local_178;
        *(float *)(lVar5 + 0x60 + lVar17) =
             fVar21 * (extraout_XMM0_Da_03 * fVar9 + (_Y - extraout_XMM0_Da_03) * fVar11);
        *(longlong *)(local_188 + 0x58) = lVar14 + 1;
      }
      lVar20 = lVar20 + 1;
      uVar16 = local_158;
    } while (lVar20 != 0xc);
  } while( true );
}

