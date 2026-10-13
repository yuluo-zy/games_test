
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_clutter::autoclutter::add_lantern_water_setdresssing::add_lantern_water_setdresssing(void)

{
  longlong *plVar1;
  undefined1 auVar2 [16];
  undefined4 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong lVar6;
  ushort uVar7;
  float _Y;
  float fVar8;
  float fVar9;
  float fVar10;
  char extraout_AL;
  char extraout_AL_00;
  longlong extraout_RAX;
  undefined1 (*pauVar11) [16];
  ulonglong extraout_RAX_00;
  undefined8 *puVar12;
  uint uVar13;
  longlong *in_RCX;
  ulonglong uVar14;
  longlong lVar15;
  uint uVar16;
  longlong *in_RDX;
  undefined1 (*pauVar17) [16];
  undefined1 (*pauVar18) [16];
  undefined1 (*pauVar19) [16];
  undefined8 *puVar20;
  short sVar21;
  undefined1 (*pauVar22) [16];
  longlong lVar23;
  float extraout_XMM0_Da;
  float fVar24;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar25;
  float fVar26;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined2 local_29b;
  undefined1 local_299;
  undefined8 local_288;
  undefined4 local_280;
  undefined4 local_27c;
  undefined4 local_278;
  undefined4 local_274;
  float fStack_270;
  undefined4 local_26c;
  longlong local_268;
  undefined8 local_260;
  undefined4 local_258;
  undefined **local_250;
  longlong lStack_248;
  undefined8 **local_240;
  undefined8 local_238;
  undefined8 local_230;
  ulonglong local_220;
  undefined8 local_218;
  ulonglong local_210;
  longlong local_208;
  ulonglong local_200;
  longlong local_1f8;
  longlong local_1f0;
  undefined8 *local_1e8;
  longlong local_1e0;
  undefined8 *local_1d8;
  undefined8 *local_1d0;
  undefined4 *local_1c8;
  longlong local_1c0;
  undefined8 *local_1b8;
  code *pcStack_1b0;
  undefined8 local_1a4;
  undefined4 local_19c;
  undefined8 local_198;
  undefined8 local_190;
  ulonglong local_178;
  longlong local_170;
  undefined1 (*local_168) [16];
  undefined1 (*local_160) [16];
  ushort local_150;
  longlong local_148;
  undefined1 (*local_140) [16];
  undefined1 (*local_138) [16];
  ushort local_128;
  longlong local_120;
  
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  puVar12 = (undefined8 *)(extraout_RAX + 0x20);
  puVar20 = (undefined8 *)(extraout_RAX + 0x28);
  if (*(char *)(extraout_RAX + 0x298) == '\0') {
    puVar20 = (undefined8 *)(extraout_RAX + 0x10);
    puVar12 = (undefined8 *)(extraout_RAX + 8);
  }
  local_190 = *puVar20;
  local_198 = *puVar12;
  local_1c0 = *in_stack_00000030 + 0x58;
  country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
  if (extraout_AL != '\0') {
    return;
  }
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter_terrain_decorators();
  fVar10 = __real_3f4ccccd;
  fVar9 = __real_40a00000;
  fVar8 = __real_3dcccccd;
  _Y = __real_3f800000;
  local_200 = (ulonglong)local_150;
  local_218 = *in_stack_00000028;
  lVar23 = *in_RDX;
  local_1d8 = (undefined8 *)(lVar23 + 0x100);
  local_1e8 = (undefined8 *)(lVar23 + 0x108);
  local_1d0 = (undefined8 *)(lVar23 + 0x110);
  local_27c = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_268 = *in_RCX;
  local_1c8 = (undefined4 *)in_RCX[2];
  local_210 = (ulonglong)local_128;
  local_1e0 = local_268 + 0x48;
  local_220 = local_178;
  pauVar17 = local_140;
  pauVar18 = local_160;
  pauVar19 = local_138;
  pauVar22 = local_168;
  do {
    if ((local_178 & 1) == 0) {
      if (pauVar22 != (undefined1 (*) [16])0x0) {
        if (local_148 != 0) goto LAB_1422d326a;
        local_148 = 0;
      }
LAB_1422d328f:
      if (pauVar17 == (undefined1 (*) [16])0x0) {
        return;
      }
      if (local_120 == 0) {
        return;
      }
      if ((short)local_210 == 0) {
        do {
          auVar2 = *pauVar19;
          uVar7 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
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
          pauVar17 = pauVar17 + -0x50;
          pauVar19 = pauVar19 + 1;
        } while (uVar7 == 0xffff);
        local_210 = (ulonglong)~(uint)uVar7;
        uVar16 = ~(uint)uVar7 & -(uint)uVar7 - 2;
        pauVar11 = pauVar17;
      }
      else {
        uVar16 = (uint)local_210 - 1 & (uint)local_210;
        pauVar11 = pauVar17;
      }
      local_120 = local_120 + -1;
      uVar13 = (uint)local_210;
      local_210 = (ulonglong)uVar16;
      pauVar17 = pauVar11;
      pauVar22 = (undefined1 (*) [16])0x0;
    }
    else {
      if (pauVar22 == (undefined1 (*) [16])0x0) {
LAB_1422d3243:
        if (local_170 != 0) {
          pauVar11 = *(undefined1 (**) [16])(local_170 + 0x30);
          local_148 = *(longlong *)(local_170 + 0x48);
          auVar2 = *pauVar11;
          uVar16 = ~(uint)(ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
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
                                  (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
          local_200 = (ulonglong)uVar16;
          pauVar18 = pauVar11 + 1;
          if (local_148 != 0) {
            local_170 = 0;
            sVar21 = (short)uVar16;
            goto joined_r0x0001422d3317;
          }
          local_148 = 0;
        }
        local_170 = 0;
        goto LAB_1422d328f;
      }
      if (local_148 == 0) {
        local_148 = 0;
        goto LAB_1422d3243;
      }
LAB_1422d326a:
      sVar21 = (short)local_200;
      pauVar11 = pauVar22;
joined_r0x0001422d3317:
      if (sVar21 == 0) {
        do {
          auVar2 = *pauVar18;
          uVar7 = (ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
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
          pauVar11 = pauVar11 + -0x50;
          pauVar18 = pauVar18 + 1;
        } while (uVar7 == 0xffff);
        uVar13 = ~(uint)uVar7;
        local_200 = (ulonglong)(uVar13 & -(uint)uVar7 - 2);
        local_148 = local_148 + -1;
        pauVar22 = pauVar11;
      }
      else {
        uVar13 = (uint)local_200;
        local_148 = local_148 + -1;
        local_200 = (ulonglong)(uVar13 - 1 & uVar13);
        pauVar22 = pauVar11;
      }
    }
    uVar16 = 0;
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar16 = uVar16 + 1;
    }
    uVar14 = (ulonglong)uVar16;
    if (((pauVar11[uVar14 * -5 + -1][0xe] & 0x3e) == 0x3c) &&
       (pauVar11[uVar14 * -5 + -1][0xf] == '\0')) {
      uVar3 = *(undefined4 *)pauVar11[uVar14 * -5 + -5];
      local_208 = local_120;
      local_1f8 = local_170;
      local_1f0 = local_148;
      country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
      transform();
      ::utils::geometry::circle::Circle2d::new();
      local_278 = uVar3;
      ::utils::rng::init_rng();
      lVar23 = 0;
      do {
        fVar25 = (float)lVar23 / fVar9;
        fastrand::Rng::f32();
        fVar24 = extraout_XMM0_Da * fVar8 + fVar25;
        if (NAN(fVar24)) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        fVar24 = fmodf(fVar24,_Y);
        fVar24 = (float)(~-(uint)(fVar24 < 0.0) & (uint)fVar24 |
                        (uint)(fVar24 + _Y) & -(uint)(fVar24 < 0.0));
        local_288 = CONCAT44(local_288._4_4_,fVar24);
        if ((fVar24 < 0.0) || (_Y < fVar24)) {
          local_1b8 = &local_288;
          pcStack_1b0 = core::fmt::float::impl$5::fmt;
          local_250 = &anon_f6c43554dafde66f7c733bfe9702e96f_3_llvm_13471928359151186412;
          lStack_248 = 1;
          local_230 = 0;
          local_240 = &local_1b8;
          local_238 = 1;
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_fmt();
        }
        if (NAN(fVar24)) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic();
        }
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        fastrand::Rng::f32();
        fastrand::Rng::f32();
        ::utils::float_grid::impl$0::
        bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
        if (0.0 < extraout_XMM0_Da_00) {
          fastrand::Rng::f32();
          fastrand::Rng::f32();
          fVar24 = _Y / SQRT(extraout_XMM0_Da_02 * extraout_XMM0_Da_02 +
                             extraout_XMM0_Da_01 * extraout_XMM0_Da_01);
          local_250 = (undefined **)
                      CONCAT44(fVar24 * extraout_XMM0_Da_02,fVar24 * extraout_XMM0_Da_01);
          ::utils::swizzlers::impl$0::x0y();
          ::utils::swizzlers::impl$0::x0y();
          lStack_248 = CONCAT44(lStack_248._4_4_,local_278);
          local_240 = (undefined8 **)0x143042d28;
          local_238 = 7;
          local_250 = (undefined **)lVar23;
          ::utils::calculate_hash<tuple_<u32,usize,ref_<str_>_>_>();
          country_core::systems::ui::resources::ui_state::DragState::is_started();
          fVar24 = fVar10;
          if (extraout_AL_00 == '\0') {
            puVar12 = local_1d8;
            fVar26 = __real_bf4ccccd;
            if ((extraout_RAX_00 & 1) == 0) {
              fVar26 = fVar25 * __real_bdcccccd + fStack_270;
              puVar12 = local_1e8;
              fVar24 = __real_3fa66666;
            }
          }
          else {
            puVar12 = local_1d0;
            fVar26 = fVar25 * __real_bd4ccccd + fStack_270;
          }
          uVar5 = *puVar12;
          *local_1c8 = local_27c;
          fStack_270 = fVar26;
          fastrand::Rng::f32();
          fVar25 = extraout_XMM0_Da_03 * __real_3f99999a;
          local_288 = CONCAT44(fStack_270,local_274);
          local_280 = local_26c;
          local_258 = local_19c;
          local_260 = local_1a4;
          lVar6 = *(longlong *)(local_268 + 0x58);
          if (lVar6 == *(longlong *)(local_268 + 0x48)) {
            alloc::raw_vec::RawVec::
            grow_one<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>
                      ();
          }
          lVar4 = *(longlong *)(local_268 + 0x50);
          lVar15 = lVar6 * 0x68;
          *(undefined8 *)(lVar4 + lVar15) = 5;
          puVar12 = (undefined8 *)(lVar4 + 8 + lVar15);
          *puVar12 = local_1b8;
          puVar12[1] = pcStack_1b0;
          *(undefined4 *)(lVar4 + 0x18 + lVar15) = 2;
          plVar1 = (longlong *)(lVar4 + 0x1c + lVar15);
          *plVar1 = (longlong)local_250;
          plVar1[1] = lStack_248;
          *(undefined4 *)(lVar4 + 0x2c + lVar15) = local_240._0_4_;
          *(undefined1 *)(lVar4 + 0x30 + lVar15) = 5;
          *(undefined2 *)(lVar4 + 0x31 + lVar15) = local_29b;
          *(undefined1 *)(lVar4 + 0x33 + lVar15) = local_299;
          *(undefined4 *)(lVar4 + 0x34 + lVar15) = 0;
          *(undefined8 *)(lVar4 + 0x38 + lVar15) = uVar5;
          *(ulonglong *)(lVar4 + 0x40 + lVar15) = extraout_RAX_00;
          *(undefined8 *)(lVar4 + 0x48 + lVar15) = local_288;
          *(undefined4 *)(lVar4 + 0x50 + lVar15) = local_280;
          *(undefined8 *)(lVar4 + 0x54 + lVar15) = local_260;
          *(undefined4 *)(lVar4 + 0x5c + lVar15) = local_258;
          *(float *)(lVar4 + 0x60 + lVar15) =
               fVar24 * (fVar25 + (_Y - extraout_XMM0_Da_03) * fVar10);
          *(longlong *)(local_268 + 0x58) = lVar6 + 1;
        }
        lVar23 = lVar23 + 1;
        local_120 = local_208;
        local_178 = local_220;
        local_148 = local_1f0;
        local_170 = local_1f8;
      } while (lVar23 != 5);
    }
  } while( true );
}

