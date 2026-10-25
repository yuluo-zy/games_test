
void __cdecl country_core::systems::save::snapshot::init_game_state_from_snapshot(void)

{
  undefined1 auVar1 [16];
  undefined4 uVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  ushort uVar7;
  uint uVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  undefined8 *puVar11;
  undefined8 *puVar12;
  uint uVar13;
  uint uVar14;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  longlong lVar15;
  undefined8 *puVar16;
  longlong lVar17;
  longlong lVar18;
  ulonglong uVar19;
  undefined1 (*pauVar20) [16];
  longlong lVar21;
  void *in_R8;
  undefined8 *puVar22;
  undefined1 (*pauVar23) [16];
  longlong lVar24;
  bool bVar25;
  undefined8 local_2c8;
  undefined *local_2c0;
  undefined8 local_2b8;
  undefined8 local_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined8 local_298;
  undefined *local_290;
  undefined8 local_288;
  undefined4 local_280;
  undefined4 local_27c;
  undefined **local_278;
  undefined8 local_270;
  undefined8 local_268;
  undefined8 local_260;
  undefined8 uStack_258;
  undefined4 *local_208;
  undefined4 *local_200;
  undefined4 *local_1f8;
  longlong local_1f0;
  undefined8 local_1e8;
  undefined8 local_1e0;
  undefined4 *local_1d8;
  undefined4 *local_1d0;
  undefined4 *local_1c8;
  undefined8 *local_1c0;
  undefined4 *local_1b8;
  undefined4 *local_1b0;
  longlong local_1a8;
  undefined4 *local_1a0;
  undefined8 local_198;
  undefined4 *local_190;
  undefined8 *local_188;
  undefined4 *local_180;
  undefined8 *local_178;
  undefined8 *local_170;
  longlong local_168;
  undefined8 *local_160;
  undefined4 local_154;
  undefined4 local_150;
  undefined4 local_14c;
  undefined4 local_148;
  undefined4 local_144;
  undefined4 local_140;
  undefined4 local_13c;
  undefined4 local_138;
  undefined4 local_134;
  undefined4 local_130;
  undefined4 local_12c;
  longlong local_128;
  longlong local_120;
  undefined4 *local_118;
  longlong *local_110;
  longlong local_108;
  longlong local_100;
  undefined4 *local_f8;
  longlong local_f0;
  undefined8 local_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 local_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 *local_98;
  longlong local_90;
  undefined8 *local_88;
  undefined8 *local_80;
  longlong local_78;
  undefined8 *local_70;
  undefined8 *local_68;
  undefined8 *local_60;
  undefined1 local_51;
  undefined1 local_50;
  undefined1 local_4f;
  undefined1 local_4e;
  undefined1 local_4d;
  undefined1 local_4c;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_60 = in_RCX;
  if (3 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_49 = 1;
    local_4a = 1;
    local_4b = 1;
    local_4c = 1;
    local_4d = 1;
    local_4e = 1;
    local_4f = 1;
    local_50 = 1;
    local_51 = 1;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_2a8 = *extraout_RAX;
    uStack_2a0 = extraout_RAX[1];
    local_27c = *(undefined4 *)(extraout_RAX + 2);
    local_298 = 4;
    local_290 = &DAT_142cdc6c0;
    local_288 = 0x25;
    local_278 = &PTR_s_Loading_game_state_from_a_snapsh_142cdc888;
    local_270 = 1;
    local_268 = 8;
    local_2c8 = 0;
    local_260 = 0;
    uStack_258 = 0;
    local_2c0 = &DAT_142cdc6c0;
    local_2b8 = 0x25;
    local_2b0 = 0;
    local_280 = 1;
    local_49 = 1;
    local_4a = 1;
    local_4b = 1;
    local_4c = 1;
    local_4d = 1;
    local_4e = 1;
    local_4f = 1;
    local_50 = 1;
    local_51 = 1;
    ::log::__private_api::impl$2::log();
  }
  puVar16 = local_60;
  memcpy(&local_2c8,in_R8,0xc0);
  local_1e8 = *(undefined8 *)((longlong)in_R8 + 0xc0);
  local_1d8 = *(undefined4 **)((longlong)in_R8 + 0xd0);
  local_148 = *(undefined4 *)((longlong)in_R8 + 0xdc);
  local_1e0 = *(undefined8 *)((longlong)in_R8 + 0xe0);
  local_1d0 = *(undefined4 **)((longlong)in_R8 + 0xf0);
  local_144 = *(undefined4 *)((longlong)in_R8 + 0xfc);
  local_88 = *(undefined8 **)((longlong)in_R8 + 0x100);
  local_1c8 = *(undefined4 **)((longlong)in_R8 + 0x110);
  local_140 = *(undefined4 *)((longlong)in_R8 + 0x11c);
  local_1a8 = *(longlong *)((longlong)in_R8 + 0x120);
  local_1a0 = *(undefined4 **)((longlong)in_R8 + 0x130);
  local_d8 = (undefined4)puVar16[7];
  uStack_d4 = (undefined4)((ulonglong)puVar16[7] >> 0x20);
  local_134 = *(undefined4 *)((longlong)in_R8 + 0x13c);
  local_198 = *(undefined8 *)((longlong)in_R8 + 0x240);
  local_190 = *(undefined4 **)((longlong)in_R8 + 0x250);
  local_130 = *(undefined4 *)((longlong)in_R8 + 0x25c);
  lVar24 = *(longlong *)((longlong)in_R8 + 0x140);
  local_118 = *(undefined4 **)((longlong)in_R8 + 0x150);
  local_98 = (undefined8 *)CONCAT44(local_98._4_4_,*(undefined4 *)((longlong)in_R8 + 0x15c));
  local_108 = *(longlong *)((longlong)in_R8 + 0x160);
  local_1b0 = *(undefined4 **)((longlong)in_R8 + 0x170);
  local_138 = *(undefined4 *)((longlong)in_R8 + 0x17c);
  lVar17 = *(longlong *)((longlong)in_R8 + 0x180);
  local_70 = *(undefined8 **)((longlong)in_R8 + 400);
  local_68 = (undefined8 *)CONCAT44(local_68._4_4_,*(undefined4 *)((longlong)in_R8 + 0x19c));
  lVar21 = *(longlong *)((longlong)in_R8 + 0x1a0);
  local_208 = *(undefined4 **)((longlong)in_R8 + 0x1b0);
  local_154 = *(undefined4 *)((longlong)in_R8 + 0x1bc);
  local_f0 = *(longlong *)((longlong)in_R8 + 0x1c0);
  local_200 = *(undefined4 **)((longlong)in_R8 + 0x1d0);
  local_150 = *(undefined4 *)((longlong)in_R8 + 0x1dc);
  local_78 = *(longlong *)((longlong)in_R8 + 0x1e0);
  local_f8 = *(undefined4 **)((longlong)in_R8 + 0x1f0);
  local_128 = CONCAT44(local_128._4_4_,*(undefined4 *)((longlong)in_R8 + 0x1fc));
  local_120 = *(longlong *)((longlong)in_R8 + 0x200);
  local_1f8 = *(undefined4 **)((longlong)in_R8 + 0x210);
  local_14c = *(undefined4 *)((longlong)in_R8 + 0x21c);
  local_188 = *(undefined8 **)((longlong)in_R8 + 0x220);
  local_180 = *(undefined4 **)((longlong)in_R8 + 0x230);
  local_12c = *(undefined4 *)((longlong)in_R8 + 0x23c);
  local_1c0 = *(undefined8 **)((longlong)in_R8 + 0x260);
  local_1b8 = *(undefined4 **)((longlong)in_R8 + 0x270);
  local_13c = *(undefined4 *)((longlong)in_R8 + 0x27c);
  local_e8 = puVar16[5];
  uStack_e0 = (undefined4)puVar16[6];
  uStack_dc = (undefined4)((ulonglong)puVar16[6] >> 0x20);
  local_49 = 1;
  local_4a = 1;
  local_4b = 1;
  local_4c = 1;
  local_4d = 1;
  local_4e = 1;
  local_4f = 1;
  local_50 = 1;
  local_51 = 0;
  resources::walls::inner_walls::InnerWalls::load_state();
  local_100 = local_60[8];
  if (local_60[10] != 0) {
    local_1f0 = lVar24 + 0x20;
    local_110 = (longlong *)(local_60[10] * 0x58);
    lVar18 = 0;
    local_90 = local_60[9];
    do {
      if (*(int *)(local_90 + 8 + lVar18) == 2) break;
      uVar4 = *(undefined8 *)(local_90 + lVar18);
      local_80 = (undefined8 *)CONCAT44(local_80._4_4_,*(undefined4 *)(local_90 + 0x50 + lVar18));
      puVar16 = (undefined8 *)(local_90 + 0x24 + lVar18);
      local_e8 = *puVar16;
      uVar6 = puVar16[1];
      puVar16 = (undefined8 *)(local_90 + 0x30 + lVar18);
      uVar9 = puVar16[1];
      local_d8 = (undefined4)((ulonglong)*puVar16 >> 0x20);
      uStack_d4 = (undefined4)uVar9;
      uStack_d0 = (undefined4)((ulonglong)uVar9 >> 0x20);
      uStack_e0 = (undefined4)uVar6;
      uStack_dc = (undefined4)((ulonglong)uVar6 >> 0x20);
      local_178 = *(undefined8 **)(local_90 + 0x40 + lVar18);
      local_170 = (undefined8 *)CONCAT44(local_170._4_4_,*(undefined4 *)(local_90 + 0x48 + lVar18));
      *local_118 = local_98._0_4_;
      lVar5 = *(longlong *)(lVar24 + 0x30);
      uVar6 = *(undefined8 *)(lVar24 + 0x40);
      if (lVar5 == *(longlong *)(lVar24 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
      }
      lVar3 = *(longlong *)(lVar24 + 0x28);
      lVar15 = lVar5 * 0x40;
      *(undefined8 *)(lVar3 + lVar15) = uVar6;
      *(undefined8 *)(lVar3 + 8 + lVar15) = uVar4;
      *(undefined4 *)(lVar3 + 0x10 + lVar15) = local_80._0_4_;
      *(undefined8 **)(lVar3 + 0x14 + lVar15) = local_178;
      *(undefined4 *)(lVar3 + 0x1c + lVar15) = local_170._0_4_;
      puVar16 = (undefined8 *)(lVar3 + 0x20 + lVar15);
      *puVar16 = local_e8;
      puVar16[1] = CONCAT44(uStack_dc,uStack_e0);
      puVar16 = (undefined8 *)(lVar3 + 0x2c + lVar15);
      *puVar16 = CONCAT44(local_d8,uStack_dc);
      puVar16[1] = CONCAT44(uStack_d0,uStack_d4);
      *(undefined1 *)(lVar3 + 0x3c + lVar15) = 1;
      *(longlong *)(lVar24 + 0x30) = lVar5 + 1;
      *(longlong *)(lVar24 + 0x40) = *(longlong *)(lVar24 + 0x40) + 1;
      lVar18 = lVar18 + 0x58;
    } while (local_110 != (longlong *)lVar18);
  }
  if (local_100 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *local_1d8 = local_148;
  local_110 = local_60 + 0xb;
  *local_1d0 = local_144;
  local_49 = 0;
  local_4a = 1;
  local_4b = 1;
  local_4c = 1;
  local_4d = 1;
  local_4e = 1;
  local_4f = 1;
  local_50 = 1;
  local_51 = 0;
  resources::walls::decorator_storage::DecoratorStorage::restore_snapshot();
  local_90 = local_60[0xe];
  puVar16 = (undefined8 *)local_60[0xf];
  local_98 = puVar16;
  if (local_60[0x10] != 0) {
    local_80 = puVar16 + local_60[0x10] * 2;
    local_118 = (undefined4 *)(lVar17 + 0x20);
    do {
      uVar4 = *puVar16;
      uVar2 = *(undefined4 *)(puVar16 + 1);
      *(undefined4 *)local_70 = local_68._0_4_;
      lVar24 = *(longlong *)(lVar17 + 0x30);
      uVar6 = *(undefined8 *)(lVar17 + 0x40);
      if (lVar24 == *(longlong *)(lVar17 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar18 = *(longlong *)(lVar17 + 0x28);
      *(undefined8 *)(lVar18 + lVar24 * 0x18) = uVar6;
      *(undefined8 *)(lVar18 + 8 + lVar24 * 0x18) = uVar4;
      *(undefined4 *)(lVar18 + 0x10 + lVar24 * 0x18) = uVar2;
      *(undefined1 *)(lVar18 + 0x14 + lVar24 * 0x18) = 0;
      *(longlong *)(lVar17 + 0x30) = lVar24 + 1;
      *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
      puVar16 = puVar16 + 2;
    } while (puVar16 != local_80);
  }
  if (local_90 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_80 = (undefined8 *)local_60[0x11];
  puVar16 = (undefined8 *)local_60[0x12];
  local_68 = puVar16;
  if (local_60[0x13] != 0) {
    puVar22 = puVar16 + local_60[0x13] * 2;
    local_70 = (undefined8 *)(lVar21 + 0x20);
    do {
      uVar4 = *puVar16;
      uVar2 = *(undefined4 *)(puVar16 + 1);
      *local_208 = local_154;
      lVar24 = *(longlong *)(lVar21 + 0x30);
      uVar6 = *(undefined8 *)(lVar21 + 0x40);
      if (lVar24 == *(longlong *)(lVar21 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar17 = *(longlong *)(lVar21 + 0x28);
      *(undefined8 *)(lVar17 + lVar24 * 0x18) = uVar6;
      *(undefined8 *)(lVar17 + 8 + lVar24 * 0x18) = uVar4;
      *(undefined4 *)(lVar17 + 0x10 + lVar24 * 0x18) = uVar2;
      *(undefined1 *)(lVar17 + 0x14 + lVar24 * 0x18) = 0;
      *(longlong *)(lVar21 + 0x30) = lVar24 + 1;
      *(longlong *)(lVar21 + 0x40) = *(longlong *)(lVar21 + 0x40) + 1;
      puVar16 = puVar16 + 2;
    } while (puVar16 != puVar22);
  }
  if (local_80 != (undefined8 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_68 = (undefined8 *)local_60[0x14];
  puVar16 = (undefined8 *)local_60[0x15];
  local_70 = puVar16;
  if (local_60[0x16] != 0) {
    puVar22 = puVar16 + local_60[0x16] * 2;
    do {
      uVar4 = *puVar16;
      uVar2 = *(undefined4 *)(puVar16 + 1);
      *local_200 = local_150;
      lVar24 = *(longlong *)(local_f0 + 0x30);
      uVar6 = *(undefined8 *)(local_f0 + 0x40);
      if (lVar24 == *(longlong *)(local_f0 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar17 = *(longlong *)(local_f0 + 0x28);
      *(undefined8 *)(lVar17 + lVar24 * 0x18) = uVar6;
      *(undefined8 *)(lVar17 + 8 + lVar24 * 0x18) = uVar4;
      *(undefined4 *)(lVar17 + 0x10 + lVar24 * 0x18) = uVar2;
      *(undefined1 *)(lVar17 + 0x14 + lVar24 * 0x18) = 0;
      *(longlong *)(local_f0 + 0x30) = lVar24 + 1;
      *(longlong *)(local_f0 + 0x40) = *(longlong *)(local_f0 + 0x40) + 1;
      puVar16 = puVar16 + 2;
    } while (puVar16 != puVar22);
  }
  if (local_68 != (undefined8 *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_70 = local_60 + 0x20;
  pauVar20 = (undefined1 (*) [16])local_60[0x21];
  lVar24 = local_60[0x24];
  auVar1 = *pauVar20;
  uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
  pauVar23 = pauVar20 + 1;
  local_f0 = local_78 + 0x20;
  while (bVar25 = lVar24 != 0, lVar24 = lVar24 + -1, bVar25) {
    if ((short)uVar13 == 0) {
      do {
        auVar1 = *pauVar23;
        uVar7 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
        pauVar20 = pauVar20 + -0x30;
        pauVar23 = pauVar23 + 1;
      } while (uVar7 == 0xffff);
      uVar14 = ~(uint)uVar7;
      uVar13 = uVar14 & -(uint)uVar7 - 2;
    }
    else {
      uVar14 = uVar13;
      uVar13 = uVar13 - 1 & uVar13;
    }
    uVar8 = 0;
    for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
      uVar8 = uVar8 + 1;
    }
    uVar2 = *(undefined4 *)pauVar20[(ulonglong)uVar8 * -3 + -3];
    *local_f8 = (undefined4)local_128;
    lVar17 = *(longlong *)(local_78 + 0x30);
    uVar4 = *(undefined8 *)(local_78 + 0x40);
    if (lVar17 == *(longlong *)(local_78 + 0x20)) {
      local_49 = 0;
      local_4a = 0;
      local_4b = 0;
      local_4c = 0;
      local_4d = 1;
      local_4e = 1;
      local_4f = 1;
      local_50 = 1;
      local_51 = 0;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar21 = *(longlong *)(local_78 + 0x28);
    *(undefined8 *)(lVar21 + lVar17 * 0x10) = uVar4;
    *(undefined4 *)(lVar21 + 8 + lVar17 * 0x10) = uVar2;
    *(longlong *)(local_78 + 0x30) = lVar17 + 1;
    *(longlong *)(local_78 + 0x40) = *(longlong *)(local_78 + 0x40) + 1;
  }
  local_a8 = local_70[8];
  local_e8 = *local_70;
  local_c8 = local_70[4];
  uStack_c0 = local_70[5];
  local_b8 = local_70[6];
  uStack_b0 = local_70[7];
  local_d8 = (undefined4)local_70[2];
  uStack_d4 = (undefined4)((ulonglong)local_70[2] >> 0x20);
  uStack_d0 = (undefined4)local_70[3];
  uStack_cc = (undefined4)((ulonglong)local_70[3] >> 0x20);
  uStack_e0 = (undefined4)local_70[1];
  uStack_dc = (undefined4)((ulonglong)local_70[1] >> 0x20);
  *local_1c8 = local_140;
  hashbrown::raw::RawTableInner::
  drop_inner_table<tuple_<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke>,alloc::alloc::Global>
            ();
  puVar16 = local_1c0;
  local_88[8] = local_a8;
  local_88[6] = local_b8;
  local_88[7] = uStack_b0;
  local_88[4] = local_c8;
  local_88[5] = uStack_c0;
  local_88[2] = CONCAT44(uStack_d4,local_d8);
  local_88[3] = CONCAT44(uStack_cc,uStack_d0);
  *local_88 = local_e8;
  local_88[1] = CONCAT44(uStack_dc,uStack_e0);
  local_e8 = local_60[0x2f];
  local_c8 = local_60[0x33];
  uStack_c0 = local_60[0x34];
  local_d8 = (undefined4)local_60[0x31];
  uStack_d4 = (undefined4)((ulonglong)local_60[0x31] >> 0x20);
  uStack_d0 = (undefined4)local_60[0x32];
  uStack_cc = (undefined4)((ulonglong)local_60[0x32] >> 0x20);
  uStack_e0 = (undefined4)local_60[0x30];
  uStack_dc = (undefined4)((ulonglong)local_60[0x30] >> 0x20);
  *local_1b8 = local_13c;
  lVar24 = local_1c0[1];
  if ((lVar24 != 0) && (lVar24 + (lVar24 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  puVar16[4] = local_c8;
  puVar16[5] = uStack_c0;
  puVar16[2] = CONCAT44(uStack_d4,local_d8);
  puVar16[3] = CONCAT44(uStack_cc,uStack_d0);
  *puVar16 = local_e8;
  puVar16[1] = CONCAT44(uStack_dc,uStack_e0);
  local_49 = 0;
  local_4a = 0;
  local_4b = 0;
  local_4c = 0;
  local_4d = 0;
  local_4e = 0;
  local_4f = 1;
  local_50 = 1;
  local_51 = 0;
  resources::world_raster_defs::AllWorldRasterWrite::write();
  *local_1b0 = local_138;
  lVar24 = *(longlong *)(local_108 + 0x30);
  uVar4 = *(undefined8 *)(local_108 + 0x40);
  if (lVar24 == *(longlong *)(local_108 + 0x20)) {
    local_49 = 0;
    local_4a = 0;
    local_4b = 0;
    local_4c = 0;
    local_4d = 0;
    local_4e = 0;
    local_4f = 1;
    local_50 = 1;
    local_51 = 0;
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  puVar16 = local_60;
  *(undefined8 *)(*(longlong *)(local_108 + 0x28) + lVar24 * 8) = uVar4;
  *(longlong *)(local_108 + 0x30) = lVar24 + 1;
  *(longlong *)(local_108 + 0x40) = *(longlong *)(local_108 + 0x40) + 1;
  *local_1a0 = local_134;
  *(undefined8 *)(local_1a8 + 0x50) = local_60[0x35];
  if (*(longlong *)(local_1a8 + 0x10) != local_60[0x1c]) {
    local_49 = 0;
    local_4a = 0;
    local_4b = 0;
    local_4c = 0;
    local_4d = 0;
    local_4e = 0;
    local_4f = 1;
    local_50 = 1;
    local_51 = 0;
                    // WARNING: Subroutine does not return
    core::slice::impl$0::copy_from_slice::len_mismatch_fail();
  }
  memcpy(*(void **)(local_1a8 + 8),(void *)local_60[0x1b],*(longlong *)(local_1a8 + 0x10) << 2);
  local_168 = puVar16[0x1d];
  local_178 = (undefined8 *)puVar16[0x1e];
  lVar24 = puVar16[0x1f];
  lVar21 = lVar24 * 0x40;
  puVar22 = local_178 + lVar24 * 8;
  lVar17 = local_168;
  puVar12 = local_178;
  local_170 = local_178;
  puVar10 = local_178;
  local_160 = puVar22;
  puVar11 = local_88;
  lVar18 = local_78;
  while (local_88 = puVar12, lVar24 != 0) {
    local_88 = puVar10 + 8;
    local_78 = puVar10[1];
    puVar16 = local_60;
    local_128 = lVar17;
    if (SBORROW8(0,local_78)) break;
    uVar4 = *puVar10;
    local_f8 = (undefined4 *)puVar10[2];
    local_c8 = puVar10[7];
    local_d8 = (undefined4)puVar10[5];
    uStack_d4 = (undefined4)((ulonglong)puVar10[5] >> 0x20);
    uStack_d0 = (undefined4)puVar10[6];
    uStack_cc = (undefined4)((ulonglong)puVar10[6] >> 0x20);
    local_e8 = puVar10[3];
    uStack_e0 = (undefined4)puVar10[4];
    uStack_dc = (undefined4)((ulonglong)puVar10[4] >> 0x20);
    *local_1f8 = local_14c;
    lVar24 = *(longlong *)(local_120 + 0x30);
    uVar6 = *(undefined8 *)(local_120 + 0x40);
    if (lVar24 == *(longlong *)(local_120 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                ();
    }
    lVar17 = *(longlong *)(local_120 + 0x28);
    *(longlong *)(lVar17 + lVar24 * 0x48) = local_78;
    *(undefined4 **)(lVar17 + 8 + lVar24 * 0x48) = local_f8;
    puVar16 = (undefined8 *)(lVar17 + 0x10 + lVar24 * 0x48);
    *puVar16 = local_e8;
    puVar16[1] = CONCAT44(uStack_dc,uStack_e0);
    puVar16 = (undefined8 *)(lVar17 + 0x20 + lVar24 * 0x48);
    *puVar16 = CONCAT44(uStack_d4,local_d8);
    puVar16[1] = CONCAT44(uStack_cc,uStack_d0);
    *(undefined8 *)(lVar17 + 0x30 + lVar24 * 0x48) = local_c8;
    *(undefined8 *)(lVar17 + 0x38 + lVar24 * 0x48) = uVar4;
    *(undefined8 *)(lVar17 + 0x40 + lVar24 * 0x48) = uVar6;
    *(longlong *)(local_120 + 0x30) = lVar24 + 1;
    *(longlong *)(local_120 + 0x40) = *(longlong *)(local_120 + 0x40) + 1;
    lVar21 = lVar21 + -0x40;
    lVar17 = local_128;
    puVar12 = puVar22;
    puVar10 = local_88;
    puVar11 = local_88;
    lVar18 = local_78;
    puVar16 = local_60;
    lVar24 = lVar21;
  }
  local_170 = local_88;
  local_78 = lVar18;
  if ((longlong)puVar22 - (longlong)local_88 != 0) {
    uVar19 = (ulonglong)((longlong)puVar22 - (longlong)local_88) >> 6;
    puVar22 = local_88 + 2;
    local_88 = puVar11;
    do {
      if (puVar22[-1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      puVar22 = puVar22 + 8;
      uVar19 = uVar19 - 1;
      puVar11 = local_88;
    } while (uVar19 != 0);
  }
  local_88 = puVar11;
  if (lVar17 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  puVar22 = local_188;
  local_e8 = puVar16[0x29];
  local_c8 = puVar16[0x2d];
  uStack_c0 = puVar16[0x2e];
  local_d8 = (undefined4)puVar16[0x2b];
  uStack_d4 = (undefined4)((ulonglong)puVar16[0x2b] >> 0x20);
  uStack_d0 = (undefined4)puVar16[0x2c];
  uStack_cc = (undefined4)((ulonglong)puVar16[0x2c] >> 0x20);
  uStack_e0 = (undefined4)puVar16[0x2a];
  uStack_dc = (undefined4)((ulonglong)puVar16[0x2a] >> 0x20);
  *local_180 = local_12c;
  lVar24 = local_188[1];
  if ((lVar24 != 0) && (lVar24 + (lVar24 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  puVar22[4] = local_c8;
  puVar22[5] = uStack_c0;
  puVar22[2] = CONCAT44(uStack_d4,local_d8);
  puVar22[3] = CONCAT44(uStack_cc,uStack_d0);
  *puVar22 = local_e8;
  puVar22[1] = CONCAT44(uStack_dc,uStack_e0);
  *local_190 = local_130;
  local_c8 = puVar16[4];
  local_e8 = *puVar16;
  local_d8 = (undefined4)puVar16[2];
  uStack_d4 = (undefined4)((ulonglong)puVar16[2] >> 0x20);
  uStack_d0 = (undefined4)puVar16[3];
  uStack_cc = (undefined4)((ulonglong)puVar16[3] >> 0x20);
  uStack_e0 = (undefined4)puVar16[1];
  uStack_dc = (undefined4)((ulonglong)puVar16[1] >> 0x20);
  local_49 = 0;
  local_4a = 0;
  local_4b = 0;
  local_4c = 0;
  local_4d = 0;
  local_4e = 0;
  local_4f = 0;
  local_50 = 0;
  local_51 = 0;
  resources::stairs::stairs_state::StairsState::restore_snapshot();
  lVar24 = local_60[0xd];
  if (lVar24 != 0) {
    lVar17 = local_60[0xc] + 0x18;
    do {
      if (*(longlong *)(lVar17 + -8) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar17 = lVar17 + 0x28;
      lVar24 = lVar24 + -1;
    } while (lVar24 != 0);
  }
  if (*local_110 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  lVar24 = local_60[0x19];
  if (lVar24 != 0) {
    lVar17 = local_60[0x18] + 0x18;
    do {
      if ((!SBORROW8(0,*(longlong *)(lVar17 + -8))) && (*(longlong *)(lVar17 + -8) != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar17 = lVar17 + 0x28;
      lVar24 = lVar24 + -1;
    } while (lVar24 != 0);
  }
  if (local_60[0x17] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_60[0x1a] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

