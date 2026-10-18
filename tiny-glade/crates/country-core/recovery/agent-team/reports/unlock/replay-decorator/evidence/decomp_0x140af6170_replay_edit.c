
void __cdecl country_core::systems::history::history_process::replay_edit(void)

{
  size_t _Size;
  undefined1 auVar1 [16];
  byte bVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  ulonglong uVar5;
  code *pcVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  ulonglong *extraout_RAX;
  ulonglong uVar11;
  undefined1 *extraout_RAX_00;
  undefined1 *_Dst;
  longlong *extraout_RAX_01;
  longlong *plVar12;
  longlong in_RCX;
  byte *pbVar13;
  longlong lVar14;
  byte *pbVar15;
  char in_DL;
  byte *pbVar16;
  ulonglong *puVar17;
  ulonglong uVar18;
  undefined1 (*pauVar19) [16];
  undefined8 *puVar20;
  undefined8 *puVar21;
  char cVar22;
  undefined8 *puVar23;
  longlong lVar24;
  longlong *in_R9;
  longlong lVar25;
  char cVar26;
  longlong lVar27;
  bool bVar28;
  undefined1 local_220 [24];
  longlong local_208;
  longlong local_200;
  byte *pbStack_1f8;
  ulonglong local_1f0;
  byte *local_1e8;
  byte *pbStack_1e0;
  longlong *local_1d8;
  char *pcStack_1d0;
  longlong **pplStack_1c8;
  longlong *plStack_1c0;
  longlong *plStack_1b8;
  undefined8 uStack_1b0;
  undefined1 (*local_188) [16];
  undefined1 (*local_180) [16];
  undefined1 *local_178;
  ushort local_170;
  longlong local_168;
  char local_160;
  char *local_158;
  longlong **pplStack_150;
  longlong local_148;
  longlong *local_140;
  undefined1 (*local_138) [16];
  longlong *local_130;
  longlong *local_128;
  longlong *local_120;
  longlong *plStack_118;
  undefined1 *local_110;
  undefined8 *local_108;
  byte *local_100;
  longlong local_f8;
  undefined8 *local_f0;
  longlong local_e8;
  longlong local_e0;
  longlong *local_d8;
  longlong local_d0;
  undefined8 *local_c8;
  longlong local_c0;
  longlong local_b8;
  byte *pbStack_b0;
  ulonglong local_a8;
  ulonglong local_98;
  ulonglong *local_90;
  ulonglong local_88;
  longlong *local_80;
  longlong *local_78;
  longlong *local_70;
  undefined1 *local_68;
  ulonglong local_60;
  undefined1 local_55;
  undefined1 local_54;
  undefined1 local_53;
  undefined1 local_52;
  undefined1 local_51;
  char local_50;
  char local_4f;
  char local_4e;
  char local_4d;
  char local_4c;
  undefined1 local_4b;
  char local_4a;
  char local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_200 = 0;
  pbStack_1f8 = &DAT_00000008;
  local_1f0 = 0;
  lVar27 = *(longlong *)(in_RCX + 8);
  local_148 = lVar27 + *(longlong *)(in_RCX + 0x10) * 0x18;
  local_78 = in_R9;
  for (; local_208 = lVar27, lVar27 != local_148; lVar27 = lVar27 + 0x18) {
    generate_replay_cmds_from_edit<country_core::systems::history::history_process::replay_edit::closure_env_0>
              ();
  }
  local_a8 = local_1f0;
  local_b8 = local_200;
  pbStack_b0 = pbStack_1f8;
  local_80 = *(longlong **)(in_RCX + 0x18);
  LOCK();
  lVar27 = *local_80;
  *local_80 = *local_80 + 1;
  UNLOCK();
  if (*local_80 == 0 || SCARRY8(lVar27,1) != *local_80 < 0) {
LAB_140af6ea5:
                    // WARNING: Does not return
    pcVar6 = (code *)invalidInstructionException();
    (*pcVar6)();
  }
  local_70 = *(longlong **)(in_RCX + 0x20);
  LOCK();
  lVar27 = *local_70;
  *local_70 = *local_70 + 1;
  UNLOCK();
  if (*local_70 == 0 || SCARRY8(lVar27,1) != *local_70 < 0) goto LAB_140af6ea5;
  if ((in_DL != '\0') && (1 < local_1f0)) {
    uVar10 = local_1f0 >> 1;
    pbVar16 = pbStack_1f8 + local_1f0 * 0x30 + -0x10;
    pbVar13 = pbStack_1f8 + 0x20;
    do {
      uVar7 = *(undefined8 *)(pbVar13 + -0x20);
      uVar8 = *(undefined8 *)(pbVar13 + -0x18);
      uVar9 = *(undefined8 *)(pbVar16 + -0x18);
      *(undefined8 *)(pbVar13 + -0x20) = *(undefined8 *)(pbVar16 + -0x20);
      *(undefined8 *)(pbVar13 + -0x18) = uVar9;
      *(undefined8 *)(pbVar16 + -0x20) = uVar7;
      *(undefined8 *)(pbVar16 + -0x18) = uVar8;
      uVar7 = *(undefined8 *)(pbVar13 + -0x10);
      uVar8 = *(undefined8 *)(pbVar13 + -8);
      uVar9 = *(undefined8 *)(pbVar16 + -8);
      *(undefined8 *)(pbVar13 + -0x10) = *(undefined8 *)(pbVar16 + -0x10);
      *(undefined8 *)(pbVar13 + -8) = uVar9;
      *(undefined8 *)(pbVar16 + -0x10) = uVar7;
      *(undefined8 *)(pbVar16 + -8) = uVar8;
      uVar7 = *(undefined8 *)pbVar13;
      uVar8 = *(undefined8 *)(pbVar13 + 8);
      uVar9 = *(undefined8 *)(pbVar16 + 8);
      *(undefined8 *)pbVar13 = *(undefined8 *)pbVar16;
      *(undefined8 *)(pbVar13 + 8) = uVar9;
      *(undefined8 *)pbVar16 = uVar7;
      *(undefined8 *)(pbVar16 + 8) = uVar8;
      pbVar13 = pbVar13 + 0x30;
      pbVar16 = pbVar16 + -0x30;
      uVar10 = uVar10 - 1;
    } while (uVar10 != 0);
  }
  local_e0 = 0;
  local_d8 = (longlong *)&DAT_00000008;
  local_d0 = 0;
  local_4d = in_DL;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == (ulonglong *)0x0) {
    local_51 = 1;
    local_52 = 1;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  local_98 = 8;
  local_88 = 0;
  uVar10 = 0;
  local_90 = extraout_RAX;
  while (pcStack_1d0 = (char *)local_a8, uVar10 < local_a8) {
    local_88 = 0;
    if (local_98 == 0) {
      local_49 = 1;
      local_4a = 1;
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    *local_90 = uVar10;
    local_88 = 1;
    pbVar16 = pbStack_b0 + local_a8 * 0x30;
    uVar10 = uVar10 + 1;
    lVar27 = 8;
    lVar25 = 8;
    uVar18 = 0;
    pbVar13 = pbStack_b0;
    while (uVar5 = local_88, puVar17 = local_90, bVar28 = lVar25 != 0, lVar25 = lVar25 + -1, bVar28)
    {
      if (uVar10 == 0) {
        if (pbVar13 == pbVar16) break;
      }
      else {
        uVar11 = ((ulonglong)((longlong)pbVar16 - (longlong)pbVar13) >> 4) * -0x5555555555555555;
        if (uVar11 < uVar10 || uVar11 - uVar10 == 0) break;
        pbVar13 = pbVar13 + uVar10 * 0x30;
        uVar18 = uVar18 + uVar10;
      }
      local_60 = uVar18;
      if (local_88 != 0) {
        lVar24 = 0;
        do {
          if (local_a8 <= *(ulonglong *)((longlong)local_90 + lVar24)) {
            local_49 = 1;
            local_4a = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          lVar14 = *(ulonglong *)((longlong)local_90 + lVar24) * 0x30;
          if (10 < pbStack_b0[lVar14]) goto LAB_140af6660;
          pbVar15 = pbStack_b0 + lVar14;
          switch(pbStack_b0[lVar14]) {
          case 0:
            bVar2 = *pbVar13;
            if (bVar2 == 0) {
              if ((pbVar15[1] == pbVar13[1]) && ((pbVar15[1] == 0 || (pbVar15[2] == pbVar13[2]))))
              goto LAB_140af6660;
            }
            else if ((bVar2 != 4) && (bVar2 != 8)) goto LAB_140af6660;
            break;
          default:
            goto LAB_140af6660;
          case 4:
            if (*pbVar13 != 0) {
              if (*pbVar13 == 5) goto code_r0x000140af6525;
              goto LAB_140af6660;
            }
            break;
          case 5:
            pbVar15 = pbVar13;
            if (*pbVar13 != 4) goto LAB_140af6660;
code_r0x000140af6525:
            if (*(longlong *)(pbVar15 + 0x20) != 1) goto LAB_140af6660;
            uVar10 = 0;
            if (*(ulonglong *)(pbVar15 + 8) <= *(ulonglong *)(pbVar15 + 0x18)) {
              uVar10 = *(ulonglong *)(pbVar15 + 8);
            }
            lVar14 = *(ulonglong *)(pbVar15 + 0x18) - uVar10;
            if ((*(longlong *)(*(longlong *)(pbVar15 + 0x10) + 0x10 + lVar14 * 0x18) != 1) ||
               (*(undefined1 **)
                 (*(longlong *)
                   (*(longlong *)(*(longlong *)(pbVar15 + 0x10) + lVar14 * 0x18 + 8) + 8) + 0x10) !=
                &DAT_8000000000000002)) goto LAB_140af6660;
            break;
          case 6:
            if (*pbVar13 != 6) goto LAB_140af6660;
            break;
          case 7:
            if ((*pbVar13 != 7) && (*pbVar13 != 10)) goto LAB_140af6660;
            break;
          case 8:
            if ((*pbVar13 & 7) != 0) goto LAB_140af6660;
            break;
          case 10:
            if (*pbVar13 != 7) goto LAB_140af6660;
          }
          lVar24 = lVar24 + 8;
        } while (lVar27 != lVar24);
      }
      if (local_88 == local_98) {
        local_49 = 1;
        local_4a = 1;
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      }
      pbVar13 = pbVar13 + 0x30;
      local_90[uVar5] = local_60;
      uVar18 = local_60 + 1;
      local_88 = uVar5 + 1;
      lVar27 = lVar27 + 8;
      uVar10 = 0;
    }
LAB_140af6660:
    _Size = local_88 * 8;
    if ((local_88 >> 0x3d != 0) || (0x7ffffffffffffff8 < _Size)) {
LAB_140af6e5f:
      local_49 = 1;
      local_4a = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (_Size == 0) {
      _Dst = &DAT_00000008;
      local_60 = 0;
      uVar10 = local_60;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      uVar10 = uVar5;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_140af6e5f;
    }
    local_60 = uVar10;
    local_68 = _Dst;
    memcpy(_Dst,puVar17,_Size);
    lVar27 = local_d0;
    if (local_d0 == local_e0) {
      alloc::raw_vec::RawVec::
      grow_one<enum2_<slab::Entry<enum2_<futures_util::lock::mutex::Waiter>_>_>,alloc::alloc::Global>
                ();
    }
    local_d8[lVar27 * 3] = local_60;
    local_d8[lVar27 * 3 + 1] = (longlong)local_68;
    local_d8[lVar27 * 3 + 2] = uVar5;
    local_d0 = lVar27 + 1;
    puVar17 = local_90 + (local_88 - 1);
    if (local_88 == 0) {
      puVar17 = (ulonglong *)0x0;
    }
    uVar10 = *puVar17 + 1;
  }
  local_a8 = 0;
  pbStack_1e0 = pbStack_b0 + (longlong)pcStack_1d0 * 0x30;
  local_1e8 = pbStack_b0;
  local_1d8 = &local_b8;
  pplStack_1c8 = (longlong **)0x0;
  local_49 = 1;
  local_4a = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<enum2_<core::option::Option<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>_>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>,alloc::alloc::Global>,enum2_<core::option::Option<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>_>_(*)(enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)>_>
            ();
  local_188 = (undefined1 (*) [16])local_80[2];
  local_178 = *local_188 + local_80[3] + 1;
  auVar1 = *local_188;
  local_170 = ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
  pauVar19 = local_188 + 1;
  local_168 = local_80[5];
  local_1e8 = (byte *)0x0;
  plStack_1b8 = (longlong *)0x0;
  local_160 = local_4d;
  local_4e = '\x01';
  local_4f = '\x01';
  local_180 = pauVar19;
  alloc::sync::impl$74::
  to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>_>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_env_0>_>
            ();
  pbStack_1e0 = (byte *)(local_d8 + local_d0 * 3);
  local_1e8 = (byte *)local_d8;
  pcStack_1d0 = &local_4d;
  pplStack_1c8 = &local_140;
  local_53 = 1;
  local_54 = 1;
  local_1d8 = (longlong *)local_220;
  local_140 = extraout_RAX_01;
  local_138 = pauVar19;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::history::replay_cmd_buffer::ReplayBucket,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::vec::Vec<usize,alloc::alloc::Global>_>,country_core::systems::history::history_process::schedule_replay_commands::closure_env_2>_>
            ();
  if (local_80[5] == 0) {
    lVar27 = local_70[8];
    if ((lVar27 == 0) && (local_70[0xb] == 0)) {
      cVar26 = '\x01';
      cVar22 = '\x01';
      goto LAB_140af6aac;
    }
    puVar21 = local_f0 + local_e8 * 8;
    cVar22 = '\x01';
    if (puVar21 != (undefined8 *)&DAT_00000040 && local_e8 != 0) goto LAB_140af6a6c;
    local_120 = (longlong *)0x0;
    plStack_118 = (longlong *)&DAT_00000008;
    local_110 = (undefined1 *)0x0;
    local_158 = (char *)0x0;
    local_130 = local_80;
    LOCK();
    *local_80 = *local_80 + -1;
    UNLOCK();
    if (*local_80 == 0) {
      local_55 = 1;
      alloc::sync::Arc::
      drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                ();
    }
    local_128 = (longlong *)0x0;
    lVar27 = local_70[8];
    plVar12 = local_70;
  }
  else {
    puVar21 = local_f0 + local_e8 * 8;
    if (puVar21 != (undefined8 *)&DAT_00000040 && local_e8 != 0) {
      local_1e8 = (byte *)puVar21[-3];
      puVar21[-3] = local_80;
      if (local_1e8 != (byte *)0x0) {
        LOCK();
        *(longlong *)local_1e8 = *(longlong *)local_1e8 + -1;
        UNLOCK();
        if (*(longlong *)local_1e8 == 0) {
          local_4b = 1;
          local_4c = '\0';
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                    ();
        }
      }
      lVar27 = local_70[8];
      cVar22 = '\0';
LAB_140af6a6c:
      if ((lVar27 == 0) && (local_70[0xb] == 0)) {
        cVar26 = '\x01';
      }
      else {
        local_1e8 = (byte *)puVar21[-2];
        puVar21[-2] = local_70;
        if (local_1e8 != (byte *)0x0) {
          LOCK();
          *(longlong *)local_1e8 = *(longlong *)local_1e8 + -1;
          UNLOCK();
          if (*(longlong *)local_1e8 == 0) {
            local_4b = 0;
            local_4c = cVar22;
            alloc::sync::Arc::
            drop_slow<country_core::resources::stairs::stairs_state::StairsHistoryDiff,alloc::alloc::Global>
                      ();
          }
        }
        cVar26 = '\0';
      }
      goto LAB_140af6aac;
    }
    local_120 = (longlong *)0x0;
    plStack_118 = (longlong *)&DAT_00000008;
    local_110 = (undefined1 *)0x0;
    local_158 = (char *)0x0;
    lVar27 = local_70[8];
    local_128 = local_80;
    plVar12 = local_70;
  }
  local_70 = plVar12;
  if ((lVar27 == 0) && (local_130 = plVar12, plVar12[0xb] == 0)) {
    LOCK();
    *plVar12 = *plVar12 + -1;
    UNLOCK();
    if (*plVar12 == 0) {
      alloc::sync::Arc::
      drop_slow<country_core::resources::stairs::stairs_state::StairsHistoryDiff,alloc::alloc::Global>
                ();
    }
    plVar12 = (longlong *)0x0;
  }
  local_1e8 = (byte *)local_120;
  pbStack_1e0 = (byte *)plStack_118;
  local_1d8 = (longlong *)local_110;
  uStack_1b0 = CONCAT71(uStack_1b0._1_7_,local_4d);
  pcStack_1d0 = local_158;
  pplStack_1c8 = pplStack_150;
  plStack_1c0 = local_128;
  plStack_1b8 = plVar12;
  if (local_e8 == local_f8) {
    alloc::raw_vec::RawVec::
    grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
  }
  local_f0[local_e8 * 8 + 6] = plStack_1b8;
  (local_f0 + local_e8 * 8 + 6)[1] = uStack_1b0;
  local_f0[local_e8 * 8 + 4] = pplStack_1c8;
  (local_f0 + local_e8 * 8 + 4)[1] = plStack_1c0;
  local_f0[local_e8 * 8 + 2] = local_1d8;
  (local_f0 + local_e8 * 8 + 2)[1] = pcStack_1d0;
  local_f0[local_e8 * 8] = local_1e8;
  (local_f0 + local_e8 * 8)[1] = pbStack_1e0;
  local_e8 = local_e8 + 1;
  cVar26 = '\0';
  cVar22 = '\0';
LAB_140af6aac:
  lVar27 = local_e8;
  local_c0 = local_f8;
  local_c8 = local_f0;
  LOCK();
  *local_140 = *local_140 + -1;
  UNLOCK();
  if (*local_140 == 0) {
    local_4f = cVar22;
    local_4e = cVar26;
    alloc::sync::Arc::
    drop_slow<slice2_<country_core::resources::walls::decorator::DecoratorId>,alloc::alloc::Global>
              ();
  }
  local_4a = cVar22;
  local_49 = cVar26;
  core::ptr::
  drop_in_place<alloc::vec::Vec<enum2_<core::option::Option<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>_>,alloc::alloc::Global>_>
            ();
  if (local_98 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_d0 != 0) {
    plVar12 = local_d8 + 1;
    lVar25 = local_d0;
    do {
      if (plVar12[-1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar12 = plVar12 + 3;
      lVar25 = lVar25 + -1;
    } while (lVar25 != 0);
  }
  if (local_e0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (cVar26 != '\0') {
    LOCK();
    *local_70 = *local_70 + -1;
    UNLOCK();
    if (*local_70 == 0) {
      local_50 = cVar22;
      alloc::sync::Arc::
      drop_slow<country_core::resources::stairs::stairs_state::StairsHistoryDiff,alloc::alloc::Global>
                ();
    }
  }
  if (cVar22 != '\0') {
    LOCK();
    *local_80 = *local_80 + -1;
    UNLOCK();
    if (*local_80 == 0) {
      alloc::sync::Arc::
      drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                ();
    }
  }
  local_68 = (undefined1 *)local_a8;
  local_60 = 0;
  local_100 = pbStack_b0;
  while (local_68 != (undefined1 *)local_60) {
    local_60 = local_60 + 1;
    core::ptr::
    drop_in_place<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>();
  }
  if (local_b8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  puVar23 = local_c8 + lVar27 * 8;
  local_120 = local_c8;
  plStack_118 = local_c8;
  local_110 = (undefined1 *)local_c0;
  puVar21 = local_c8;
  local_108 = puVar23;
  if (lVar27 != 0) {
    uVar3 = *(undefined4 *)((longlong)local_78 + 0x1c);
    puVar17 = (ulonglong *)*local_78;
    puVar4 = (undefined4 *)local_78[2];
    local_78 = local_c8;
    do {
      puVar20 = local_78 + 8;
      plVar12 = (longlong *)*local_78;
      puVar21 = puVar20;
      if (SBORROW8(0,(longlong)plVar12)) break;
      uStack_1b0 = local_78[7];
      pbStack_1e0 = (byte *)local_78[1];
      local_1d8 = (longlong *)local_78[2];
      pcStack_1d0 = (char *)local_78[3];
      pplStack_1c8 = (longlong **)local_78[4];
      plStack_1c0 = (longlong *)local_78[5];
      plStack_1b8 = (longlong *)local_78[6];
      *puVar4 = uVar3;
      puVar17[4] = puVar17[4] + 1;
      uVar10 = *puVar17;
      uVar18 = puVar17[3];
      local_1e8 = (byte *)plVar12;
      local_78 = puVar20;
      if (uVar18 == uVar10) {
        alloc::collections::vec_deque::VecDeque::
        grow<country_core::resources::history::replay_cmd_buffer::ReplayBucket,alloc::alloc::Global>
                  ();
        uVar10 = *puVar17;
        uVar18 = puVar17[3];
      }
      if (uVar18 + puVar17[2] < uVar10) {
        uVar10 = 0;
      }
      uVar5 = puVar17[1];
      lVar27 = ((uVar18 + puVar17[2]) - uVar10) * 0x40;
      puVar21 = (undefined8 *)(uVar5 + 0x30 + lVar27);
      *puVar21 = plStack_1b8;
      puVar21[1] = uStack_1b0;
      plVar12 = (longlong *)(uVar5 + 0x20 + lVar27);
      *plVar12 = (longlong)pplStack_1c8;
      plVar12[1] = (longlong)plStack_1c0;
      plVar12 = (longlong *)(uVar5 + 0x10 + lVar27);
      *plVar12 = (longlong)local_1d8;
      plVar12[1] = (longlong)pcStack_1d0;
      *(undefined8 *)(uVar5 + lVar27) = local_1e8;
      ((undefined8 *)(uVar5 + lVar27))[1] = pbStack_1e0;
      puVar17[3] = puVar17[3] + 1;
      puVar21 = puVar23;
    } while (puVar20 != puVar23);
  }
  local_68 = (undefined1 *)((ulonglong)((longlong)puVar23 - (longlong)puVar21) >> 6);
  local_60 = 0;
  plStack_118 = puVar21;
  local_78 = puVar21;
  while (local_68 != (undefined1 *)local_60) {
    local_60 = local_60 + 1;
    core::ptr::drop_in_place<country_core::resources::history::replay_cmd_buffer::ReplayBucket>();
  }
  if (local_c0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

