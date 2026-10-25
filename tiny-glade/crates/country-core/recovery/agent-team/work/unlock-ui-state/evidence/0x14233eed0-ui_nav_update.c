
// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x00014304deda) overlaps instruction at (ram,0x00014304ded9)
// 
// WARNING: Removing unreachable block (ram,0x00014304df14)
// WARNING: Removing unreachable block (ram,0x00014304df51)
// WARNING: Removing unreachable block (ram,0x00014304df17)
// WARNING: Removing unreachable block (ram,0x00014304df1c)
// WARNING: Removing unreachable block (ram,0x00014304e0c5)
// WARNING: Removing unreachable block (ram,0x00014304e0d1)
// WARNING: Removing unreachable block (ram,0x00014304e09c)
// WARNING: Type propagation algorithm not settling
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl toolbar::ui_interaction::ui_nav_update(void)

{
  byte *pbVar1;
  undefined1 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  longlong *plVar6;
  undefined *puVar7;
  undefined *puVar8;
  undefined *puVar9;
  undefined **ppuVar10;
  undefined8 uVar11;
  undefined **ppuVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  char extraout_AL_08;
  char cVar13;
  char cVar14;
  uint extraout_EAX;
  uint extraout_EAX_00;
  uint extraout_EAX_01;
  uint extraout_EAX_02;
  uint uVar15;
  undefined8 *puVar16;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong extraout_RAX_02;
  byte *pbVar17;
  undefined ***pppuVar18;
  int *piVar19;
  uint *puVar20;
  undefined **in_RCX;
  longlong lVar22;
  undefined ***pppuVar23;
  code *UNRECOVERED_JUMPTABLE;
  undefined ***pppuVar24;
  undefined2 uVar25;
  undefined8 *in_RDX;
  uint *puVar26;
  undefined8 uVar27;
  undefined **ppuVar28;
  byte bVar29;
  undefined **ppuVar30;
  ulonglong *puVar31;
  ulonglong *puVar32;
  char *pcVar33;
  char *pcVar34;
  undefined1 *puVar35;
  undefined1 *puVar36;
  byte *pbVar37;
  longlong lVar38;
  undefined **ppuVar39;
  undefined **in_R9;
  longlong lVar40;
  ulonglong uVar41;
  uint uVar42;
  byte bVar43;
  longlong lVar44;
  undefined **ppuVar45;
  ulonglong uVar46;
  longlong lVar47;
  undefined8 *puVar48;
  longlong in_FS_OFFSET;
  longlong unaff_GS_OFFSET;
  bool bVar49;
  byte in_AF;
  bool bVar50;
  bool bVar51;
  byte in_TF;
  byte in_IF;
  byte in_NT;
  byte in_AC;
  byte in_VIF;
  byte in_VIP;
  byte in_ID;
  int iVar53;
  undefined1 auVar52 [16];
  undefined **in_stack_00000028;
  ulonglong auStack_2c0 [5];
  undefined **local_298;
  undefined *local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined *local_258;
  undefined8 uStack_250;
  undefined1 local_248 [16];
  undefined8 local_238;
  longlong lStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  longlong lStack_200;
  undefined *local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined **local_1c8;
  undefined **ppuStack_1c0;
  undefined **local_1b8;
  undefined **local_1b0;
  undefined4 *local_1a8;
  undefined8 local_198;
  code *local_190;
  int local_188;
  int iStack_184;
  undefined **ppuStack_180;
  longlong local_178;
  longlong lStack_170;
  undefined **local_168;
  byte abStack_160 [4];
  undefined4 local_15c;
  undefined4 local_158;
  float local_154;
  undefined **local_150;
  undefined **local_148;
  undefined **ppuStack_140;
  undefined **local_138;
  undefined4 *puStack_130;
  undefined4 *local_128;
  int *local_120;
  undefined **local_118;
  undefined **ppuStack_110;
  undefined **local_108;
  undefined **local_e8;
  undefined **ppuStack_e0;
  undefined **local_d8;
  undefined **local_d0;
  ulonglong local_c8;
  undefined **local_c0;
  undefined **local_b8;
  undefined **local_b0;
  undefined8 local_a8;
  uint local_a0;
  uint local_9c;
  undefined **local_98;
  byte local_8e;
  byte local_8d;
  uint local_8c;
  uint local_88;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  undefined7 uVar21;
  
  bVar49 = false;
  puVar32 = auStack_2c0 + 1;
  local_80 = 0xfffffffffffffffe;
  puVar16 = (undefined8 *)*in_stack_00000028;
  local_b8 = (undefined **)in_RDX;
  country_core::systems::ui::resources::widget_params::UiFocus::is_mei();
  if (extraout_AL != '\0') {
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    *puVar16 = 0;
    return;
  }
  country_core::input::action_set::DigitalActionState::just_pressed();
  if (extraout_AL_00 == '\0') {
    local_8c = 0;
  }
  else {
    country_core::input::action_set::DigitalActionState::just_pressed();
    local_8c = extraout_EAX ^ 1;
  }
  country_core::input::action_set::DigitalActionState::just_pressed();
  if (extraout_AL_01 == '\0') {
    local_a0 = 0;
  }
  else {
    country_core::input::action_set::DigitalActionState::just_pressed();
    local_a0 = extraout_EAX_00 ^ 1;
  }
  country_core::input::action_set::DigitalActionState::just_pressed();
  if (extraout_AL_02 == '\0') {
    local_9c = 0;
  }
  else {
    country_core::input::action_set::DigitalActionState::just_pressed();
    local_9c = extraout_EAX_01 ^ 1;
  }
  country_core::input::action_set::DigitalActionState::just_pressed();
  local_d0 = in_R9;
  if (extraout_AL_03 == '\0') {
    local_88 = 0;
  }
  else {
    country_core::input::action_set::DigitalActionState::just_pressed();
    local_88 = extraout_EAX_02 ^ 1;
  }
  bVar43 = (byte)local_8c | (byte)local_a0;
  bVar29 = (byte)local_9c | (byte)local_88 | bVar43;
  ppuVar45 = in_stack_00000028;
  if (*(int *)(*in_stack_00000028 + 0x10) == 1) {
    uVar41 = *(ulonglong *)(*in_stack_00000028 + 0x18);
    country_core::input::action_set::DigitalActionState::just_released();
    if (extraout_AL_04 == '\0') {
      puVar7 = in_RCX[1];
      uVar46 = uVar41 & 0xffffffff;
      if (*(ulonglong *)(puVar7 + 0x10) <= uVar46) goto LAB_14233f184;
      iVar53 = (int)(uVar41 >> 0x20);
      if (*(int *)(*(longlong *)(puVar7 + 8) + uVar46 * 0x14) != iVar53) goto LAB_14233f184;
      lVar22 = *(longlong *)(puVar7 + 8) + uVar46 * 0x14;
      uVar46 = (ulonglong)*(uint *)(lVar22 + 4);
      if (((uVar46 == 0xffffffff) || (puVar8 = *in_RCX, *(ulonglong *)(puVar8 + 0x38) <= uVar46)) ||
         ((*(ulonglong *)
            (*(longlong *)(puVar8 + 0x28) + (ulonglong)(*(uint *)(lVar22 + 4) >> 6) * 8) >>
           (uVar46 & 0x3f) & 1) == 0)) goto LAB_14233f184;
      if (uVar46 < *(ulonglong *)(puVar7 + 0x108)) {
        lVar40 = uVar46 * 0xa0 + *(longlong *)(puVar7 + 0x100);
      }
      else {
        lVar40 = 0;
      }
      lVar38 = (ulonglong)*(uint *)(lVar22 + 0xc) * 0x48 + *(longlong *)(puVar7 + 0x1a8);
      uVar46 = *(ulonglong *)(puVar8 + 0x110);
      if ((uVar46 < *(ulonglong *)(lVar40 + 0x88)) &&
         (*(longlong *)(*(longlong *)(lVar40 + 0x80) + uVar46 * 8) != 0)) {
        bVar51 = true;
        bVar50 = true;
        if ((*(ulonglong *)(lVar38 + 0x40) <= uVar46) ||
           (uVar46 = *(ulonglong *)(*(longlong *)(lVar38 + 0x38) + uVar46 * 8), uVar46 == 0))
        goto LAB_14233f496;
        lVar40 = *(longlong *)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar46 * 0x30);
      }
      else {
        bVar50 = false;
LAB_14233f496:
        bVar51 = bVar50;
        lVar40 = 0;
      }
      uVar42 = *(uint *)(lVar22 + 0x10);
      if ((*(ulonglong *)(puVar8 + 0x120) < *(ulonglong *)(lVar38 + 0x40)) &&
         (uVar46 = *(ulonglong *)(*(longlong *)(lVar38 + 0x38) + *(ulonglong *)(puVar8 + 0x120) * 8)
         , uVar46 != 0)) {
        lVar22 = *(longlong *)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar46 * 0x30);
      }
      else {
        lVar22 = 0;
      }
      local_150 = (undefined **)0x0;
      if (bVar51) {
        local_150 = (undefined **)(lVar40 + (ulonglong)uVar42 * 0x10);
      }
      local_c8 = uVar41;
      local_8e = bVar43;
      country_core::input::action_set::DigitalActionState::just_pressed();
      if (extraout_AL_07 == '\0') {
        puVar48 = (undefined8 *)(lVar22 + (ulonglong)uVar42 * 0x40);
        lVar22 = 8;
        puVar16 = puVar48;
        if ((char)local_8c == '\0') {
          if ((byte)local_a0 == '\0') {
            if ((char)local_9c == '\0') {
              if ((char)local_88 == '\0') {
                return;
              }
              puVar16 = puVar48 + 6;
              lVar22 = 0x38;
            }
            else {
              puVar16 = puVar48 + 4;
              lVar22 = 0x28;
            }
          }
          else {
            puVar16 = puVar48 + 2;
            lVar22 = 0x18;
          }
        }
        uVar27 = *(undefined8 *)((longlong)puVar48 + lVar22);
        local_190 = (code *)*puVar16;
        if ((bVar29 & local_190 == (code *)0x0) == 0) {
                    // WARNING: Could not recover jumptable at 0x00014233f68c. Too many branches
                    // WARNING: Treating indirect jump as call
          (*(code *)(&DAT_14304ded8 + *(int *)(&DAT_14304ded8 + (longlong)local_190 * 4)))
                    (&DAT_14304ded8 + *(int *)(&DAT_14304ded8 + (longlong)local_190 * 4));
          return;
        }
        if (local_150 == (undefined **)0x0) {
          *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
          puVar7 = *in_stack_00000028;
          *(undefined8 *)(puVar7 + 0x10) = 1;
          *(int *)(puVar7 + 0x18) = (int)local_c8;
          *(int *)(puVar7 + 0x1c) = iVar53;
          return;
        }
        local_198 = uVar27;
        local_168 = in_RCX;
        std::thread::local::LocalKey::
        with<core::cell::Cell<tuple_<u64,u64>_>,std::hash::random::impl_0::new::closure_env_0,std::hash::random::RandomState>
                  ();
        local_1f8 = &DAT_14304d700;
        uStack_1f0 = 0;
        local_1e8 = 0;
        uStack_1e0 = 0;
        local_118 = (undefined **)0x0;
        ppuStack_110 = (undefined **)0x4;
        local_108 = (undefined **)0x0;
        lVar40 = *(longlong *)(*local_d0 + 0xa0);
        lVar22 = lVar40 + *(longlong *)(*local_d0 + 0xa8) * 0x28;
        local_1d8 = extraout_RAX;
        uStack_1d0 = uVar27;
        for (; ppuVar30 = local_108, lVar40 != lVar22; lVar40 = lVar40 + 0x28) {
          uVar27 = *(undefined8 *)(lVar40 + 0x10);
          if (local_108 == local_118) {
            alloc::raw_vec::RawVec::grow_one<tuple_<u32,u32>,alloc::alloc::Global>();
          }
          *(undefined8 *)((longlong)ppuStack_110 + (longlong)ppuVar30 * 8) = uVar27;
          local_108 = (undefined **)((longlong)ppuVar30 + 1);
        }
        while (local_108 != (undefined **)0x0) {
          while( true ) {
            uVar3 = *(undefined4 *)((longlong)ppuStack_110 + -4 + (longlong)local_108 * 8);
            local_108 = (undefined **)((longlong)local_108 + -1);
            taffy::node::Taffy::children();
            if ((int)local_148 != 4) break;
            local_98 = ppuStack_140;
            local_b0 = local_138;
            ppuVar30 = local_138;
            for (lVar22 = (longlong)puStack_130 << 3; ppuVar28 = local_108, lVar22 != 0;
                lVar22 = lVar22 + -8) {
              uVar4 = *(undefined4 *)ppuVar30;
              uVar5 = *(undefined4 *)((longlong)ppuVar30 + 4);
              if (local_108 == local_118) {
                alloc::raw_vec::RawVec::grow_one<tuple_<u32,u32>,alloc::alloc::Global>();
              }
              ppuVar30 = ppuVar30 + 1;
              *(undefined4 *)((longlong)ppuStack_110 + (longlong)ppuVar28 * 8) = uVar4;
              *(undefined4 *)((longlong)ppuStack_110 + 4 + (longlong)ppuVar28 * 8) = uVar5;
              local_108 = (undefined **)((longlong)ppuVar28 + 1);
              local_298 = (undefined **)CONCAT44(local_298._4_4_,uVar3);
              hashbrown::map::HashMap::
              insert<slotmap::DefaultKey,slotmap::DefaultKey,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
            }
            if (local_98 != (undefined **)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_108 == (undefined **)0x0) goto LAB_14233f7d0;
          }
        }
LAB_14233f7d0:
        uVar42 = local_88;
        lVar22 = 0;
        if (local_118 != (undefined **)0x0) {
          lVar22 = (longlong)local_118 << 3;
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_268 = local_1d8;
        uStack_260 = uStack_1d0;
        local_278 = local_1e8;
        uStack_270 = uStack_1e0;
        local_288 = local_1f8;
        uStack_280 = uStack_1f0;
        local_158 = (float)(local_a0 & 0xff) + (0.0 - (float)(local_8c & 0xff));
        local_154 = (float)(local_9c & 0xff) - (float)(uVar42 & 0xff);
        std::thread::local::LocalKey::
        with<core::cell::Cell<tuple_<u64,u64>_>,std::hash::random::impl_0::new::closure_env_0,std::hash::random::RandomState>
                  ();
        local_218 = 0;
        uStack_210 = 0;
        local_228 = &DAT_14304d700;
        uStack_220 = 0;
        local_98 = (undefined **)*local_b8;
        lVar40 = (longlong)local_b8[1];
        puVar26 = (uint *)local_98[0x1f];
        puVar20 = puVar26 + (longlong)local_98[0x20];
        local_b8 = (undefined **)CONCAT71(local_b8._1_7_,*(char *)(local_98 + 0x21));
        local_c0 = (undefined **)&DAT_00000008;
        local_b0 = (undefined **)&DAT_00000008;
        lVar38 = 0;
        lVar44 = 0;
        local_208 = extraout_RAX_00;
        lStack_200 = lVar22;
        do {
          ppuVar30 = local_d0;
          if ((char)local_b8 == '\0') {
            if (lVar44 == lVar38) {
              do {
                if (puVar26 == puVar20) goto LAB_14233fa2f;
                uVar42 = *puVar26;
                puVar26 = puVar26 + 1;
                lVar22 = *(longlong *)(lVar40 + 0x100);
                lVar38 = *(longlong *)(lVar22 + 0x58 + (ulonglong)uVar42 * 0xa0);
              } while (lVar38 == 0);
              local_c0 = *(undefined ***)(lVar22 + (ulonglong)uVar42 * 0xa0 + 0x50);
              lVar44 = 0;
            }
          }
          else if (lVar44 == lVar38) {
            do {
              if (puVar26 == puVar20) goto LAB_14233fa2f;
              uVar42 = *puVar26;
              puVar26 = puVar26 + 1;
              lVar22 = *(longlong *)(lVar40 + 0x1a8);
              lVar38 = *(longlong *)(lVar22 + 0x10 + (ulonglong)uVar42 * 0x48);
            } while (lVar38 == 0);
            local_b0 = *(undefined ***)(lVar22 + (ulonglong)uVar42 * 0x48 + 8);
            lVar44 = 0;
          }
          lVar44 = lVar44 + 1;
          local_82 = 1;
          hashbrown::map::HashMap::
          insert<slotmap::DefaultKey,bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
        } while( true );
      }
      bevy_ecs::event::writer::EventWriter::write<toolbar::ui_interaction::UiClickEvent>();
    }
    puVar16 = (undefined8 *)*in_stack_00000028;
    uVar27 = puVar16[2];
    uVar11 = puVar16[3];
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    *puVar16 = uVar27;
    puVar16[1] = uVar11;
LAB_14233f184:
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    *(undefined8 *)(*in_stack_00000028 + 0x10) = 0;
    return;
  }
  if ((bVar29 == 0) &&
     (country_core::input::action_set::DigitalActionState::just_pressed(), extraout_AL_05 == '\0'))
  {
    return;
  }
  country_core::systems::ui::resources::widget_params::UiFocus::is_toolbar();
  if (extraout_AL_06 == '\0') {
    puVar7 = *in_RCX;
    puVar8 = in_RCX[1];
    puVar26 = *(uint **)(puVar7 + 0xf8);
    puVar20 = puVar26 + *(longlong *)(puVar7 + 0x100);
    if (puVar7[0x108] == '\0') {
      lVar38 = 8;
      lVar40 = 0;
      bVar49 = false;
      lVar44 = 0;
      lVar22 = 0;
      while( true ) {
        if (lVar44 == lVar22) {
          do {
            if (puVar26 == puVar20) goto LAB_14233f3ae;
            uVar42 = *puVar26;
            puVar26 = puVar26 + 1;
            lVar22 = *(longlong *)(*(longlong *)(puVar8 + 0x100) + 0x58 + (ulonglong)uVar42 * 0xa0);
          } while (lVar22 == 0);
          lVar38 = *(longlong *)(puVar8 + 0x100) + (ulonglong)uVar42 * 0xa0;
          uVar41 = *(ulonglong *)(puVar7 + 0x118);
          if ((uVar41 < *(ulonglong *)(lVar38 + 0x88)) &&
             (*(longlong *)(*(longlong *)(lVar38 + 0x80) + uVar41 * 8) != 0)) {
            lVar40 = (ulonglong)*(uint *)(lVar38 + 0x94) * 0x48 + *(longlong *)(puVar8 + 0x1a8);
            bVar49 = true;
            if ((uVar41 < *(ulonglong *)(lVar40 + 0x40)) &&
               (uVar41 = *(ulonglong *)(*(longlong *)(lVar40 + 0x38) + uVar41 * 8), uVar41 != 0)) {
              lVar40 = *(longlong *)(*(longlong *)(lVar40 + 0x18) + 0x10 + ~uVar41 * 0x30);
            }
            else {
              lVar40 = 0;
            }
          }
          else {
            bVar49 = false;
          }
          lVar38 = *(longlong *)(lVar38 + 0x50);
          lVar44 = 0;
        }
        lVar47 = lVar44 * 0x10;
        ppuStack_110 = *(undefined ***)(lVar38 + lVar47);
        if (!bVar49) break;
        if ((ulonglong)ppuStack_110 >> 0x20 == 0) goto LAB_14233f3ae;
        if ((lVar40 == 0) ||
           (lVar44 = lVar44 + 1,
           *(char *)((ulonglong)*(uint *)(lVar47 + lVar38 + 8) * 3 + lVar40) != '\0'))
        goto LAB_14233f414;
      }
      if ((ulonglong)ppuStack_110 >> 0x20 == 0) goto LAB_14233f3ae;
    }
    else {
      lVar40 = 8;
      lVar22 = 0;
      bVar49 = false;
      uVar46 = 0;
      uVar41 = 0;
      do {
        if (uVar46 == uVar41) {
          do {
            if (puVar26 == puVar20) goto LAB_14233f3ae;
            uVar42 = *puVar26;
            puVar26 = puVar26 + 1;
            uVar41 = *(ulonglong *)(*(longlong *)(puVar8 + 0x1a8) + 0x10 + (ulonglong)uVar42 * 0x48)
            ;
          } while (uVar41 == 0);
          lVar40 = *(longlong *)(puVar8 + 0x1a8) + (ulonglong)uVar42 * 0x48;
          if ((*(ulonglong *)(puVar7 + 0x118) < *(ulonglong *)(lVar40 + 0x40)) &&
             (uVar46 = *(ulonglong *)
                        (*(longlong *)(lVar40 + 0x38) + *(ulonglong *)(puVar7 + 0x118) * 8),
             uVar46 != 0)) {
            lVar22 = *(longlong *)(*(longlong *)(lVar40 + 0x18) + 0x10 + ~uVar46 * 0x30);
            bVar49 = true;
          }
          else {
            bVar49 = false;
          }
          lVar40 = *(longlong *)(lVar40 + 8);
          uVar46 = 0;
        }
        ppuStack_110 = *(undefined ***)(lVar40 + uVar46 * 8);
        pcVar33 = (char *)((uVar46 & 0xffffffff) * 3 + lVar22);
        if (!bVar49) {
          pcVar33 = (char *)0x0;
        }
        if ((ulonglong)ppuStack_110 >> 0x20 == 0) goto LAB_14233f3ae;
      } while ((pcVar33 != (char *)0x0) && (uVar46 = uVar46 + 1, *pcVar33 == '\0'));
    }
LAB_14233f414:
    local_118 = (undefined **)0x1;
    pppuVar18 = &local_118;
    goto LAB_14233f3c0;
  }
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  plVar6 = (longlong *)*in_stack_00000028;
  local_1c8 = (undefined **)*plVar6;
  ppuStack_1c0 = (undefined **)plVar6[1];
  *plVar6 = 0;
LAB_14233f3d2:
  pbVar17 = (byte *)(ulonglong)*(uint *)((longlong)ppuVar45 + 0x1c);
  UNRECOVERED_JUMPTABLE = (code *)ppuVar45[2];
code_r0x00014233f3da:
  *(int *)UNRECOVERED_JUMPTABLE = (int)pbVar17;
  puVar7 = *ppuVar45;
  *(undefined ***)(puVar7 + 0x10) = local_1c8;
  *(undefined ***)(puVar7 + 0x18) = ppuStack_1c0;
                    // WARNING: Read-only address (ram,0x00014304dec8) is written
                    // WARNING: Read-only address (ram,0x00014304e008) is written
  return;
LAB_14233f3ae:
  local_118 = (undefined **)0x3;
  pppuVar18 = &local_148;
LAB_14233f3c0:
  local_148 = (undefined **)0x0;
  local_1c8 = *pppuVar18;
  ppuStack_1c0 = pppuVar18[1];
  goto LAB_14233f3d2;
LAB_14233fa2f:
  local_248._8_8_ = uStack_210;
  local_248._0_8_ = local_218;
  local_238 = local_208;
  lStack_230 = lStack_200;
  local_258 = local_228;
  uStack_250 = uStack_220;
  local_1c8 = local_d0;
  ppuStack_1c0 = &local_258;
  local_1b8 = local_168;
  local_1b0 = local_150;
  local_1a8 = &local_158;
  bevy_ecs::system::query::Query::
  get<tuple_<bevy_ecs::entity::Entity,ref_<toolbar::resouces_and_components::TaffyNode>,ref_<country_core::components::UiNav>_>,tuple_<>_>
            ();
  if (local_148 == (undefined **)0x0) {
    local_118 = ppuStack_140;
    ppuStack_110 = local_138;
    local_298 = &PTR_DAT_14304de08;
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_a8 = *local_148;
  hashbrown::map::impl$7::
  default<slotmap::DefaultKey,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>();
  local_8d = (byte)local_8c | (byte)local_88;
  pppuVar23 = &local_118;
LAB_14233faff:
  hashbrown::map::HashMap::
  insert<slotmap::DefaultKey,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>();
  if (extraout_AL_08 == '\0') {
    std::collections::hash::map::HashMap::
    get<slotmap::DefaultKey,slotmap::DefaultKey,std::hash::random::RandomState,slotmap::DefaultKey>
              ();
    if (extraout_RAX_01 == (longlong *)0x0) {
      lVar22 = *(longlong *)(*ppuVar30 + 0xa8);
      if (lVar22 != 0) {
        lVar40 = *(longlong *)(*ppuVar30 + 0xa0);
        ppuVar28 = (undefined **)(ulonglong)local_a8._4_4_;
        ppuVar39 = (undefined **)0x0;
        goto LAB_14233fc35;
      }
      goto LAB_142340005;
    }
    uVar42 = *(uint *)((longlong)extraout_RAX_01 + 4);
    puVar8 = (undefined *)*extraout_RAX_01;
    puVar7 = (undefined *)*extraout_RAX_01;
    resouces_and_components::taffy_layout::TaffyLayout::style();
    if ((*(char *)(extraout_RAX_02 + 0x14b) != '\0') == (bool)local_8e) {
      local_e8 = (undefined **)0x0;
      ppuStack_e0 = (undefined **)&DAT_00000004;
      local_d8 = (undefined **)0x0;
    }
    else {
      uVar41 = (ulonglong)uVar42;
      taffy::node::Taffy::children();
      if (local_188 != 4) {
        local_148 = (undefined **)CONCAT44(iStack_184,local_188);
        local_138 = (undefined **)local_178;
        puStack_130 = (undefined4 *)lStack_170;
        ppuStack_140 = ppuStack_180;
        local_298 = &PTR_DAT_14304de20;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_b8 = ppuStack_180;
      local_98 = (undefined **)local_178;
      if (lStack_170 == 0) {
LAB_142340109:
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      lVar22 = -8;
      lVar40 = 0;
      while (uVar27 = *(undefined8 *)(local_178 + lVar40 * 8),
            auVar52._0_4_ = -(uint)((int)uVar27 == (int)local_a8),
            iVar53 = -(uint)((int)((ulonglong)uVar27 >> 0x20) == (int)((ulonglong)local_a8 >> 0x20))
            , auVar52._4_4_ = auVar52._0_4_, auVar52._8_4_ = iVar53, auVar52._12_4_ = iVar53,
            iVar53 = movmskpd((int)uVar41,auVar52), iVar53 != 3) {
        lVar40 = lVar40 + 1;
        uVar41 = (lStack_170 * 8 + lVar22) - 8;
        lVar22 = lVar22 + -8;
        if (uVar41 == 0xfffffffffffffff8) goto LAB_142340109;
      }
      if (local_8d == 0) {
        alloc::slice::impl$0::to_vec<slotmap::DefaultKey>();
      }
      else {
        alloc::slice::impl$0::to_vec<slotmap::DefaultKey>();
        if ((undefined **)0x1 < local_138) {
          if ((ulonglong)local_138 >> 1 == 1) {
            uVar41 = 0;
          }
          else {
            lVar22 = (longlong)local_138 + -1;
            uVar41 = 0;
            do {
              puVar9 = ppuStack_140[uVar41];
              ppuStack_140[uVar41] = ppuStack_140[lVar22];
              ppuStack_140[lVar22] = puVar9;
              ppuVar45 = (undefined **)ppuStack_140[uVar41 + 1];
              ppuStack_140[uVar41 + 1] =
                   ppuStack_140[(longlong)local_138 + (uVar41 ^ 0x1ffffffffffffffe)];
              ppuStack_140[(longlong)local_138 + (uVar41 ^ 0x1ffffffffffffffe)] =
                   (undefined *)ppuVar45;
              uVar41 = uVar41 + 2;
              lVar22 = lVar22 + -2;
            } while (((ulonglong)local_138 >> 1 & 0xfffffffffffffffe) != uVar41);
          }
          ppuVar30 = local_d0;
          if (((ulonglong)local_138 & 2) != 0) {
            puVar9 = ppuStack_140[uVar41];
            ppuStack_140[uVar41] = ppuStack_140[(longlong)local_138 + ~uVar41];
            ppuStack_140[(longlong)local_138 + ~uVar41] = puVar9;
          }
        }
        local_d8 = local_138;
        local_e8 = local_148;
        ppuStack_e0 = ppuStack_140;
      }
      if (local_b8 != (undefined **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    ppuVar10 = *(undefined ***)(*ppuVar30 + 0xa0);
    local_98 = ppuVar10 + *(longlong *)(*ppuVar30 + 0xa8) * 5;
    ppuVar28 = local_98;
    while (ppuVar39 = ppuVar10, ppuVar12 = local_d8, ppuVar39 != ppuVar28) {
      ppuVar30 = ppuVar39 + 5;
      bVar29 = *(byte *)((longlong)ppuVar39 + 0x24);
      ppuVar10 = ppuVar30;
      if ((bVar29 != 4) && (*(int *)((longlong)ppuVar39 + 0x1c) == local_a8._4_4_)) {
        pppuVar24 = (undefined ***)((ulonglong)local_a8 & 0xffffffff);
        if (*(int *)(ppuVar39 + 3) == (int)local_a8) {
          uVar41 = (ulonglong)*(int *)(&DAT_14304deb8 + (ulonglong)(uint)bVar29 * 4);
          pppuVar18 = (undefined ***)(&DAT_14304deb8 + uVar41);
          bVar50 = pppuVar18 == (undefined ***)0x0;
          cVar14 = (char)((ulonglong)ppuVar28 >> 8);
          cVar13 = (char)pppuVar18;
          bVar43 = (byte)local_a8;
          puVar9 = local_a8;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
          switch(bVar29) {
          case 0:
            cVar14 = (byte)local_88;
            break;
          case 1:
            cVar14 = (char)local_9c;
            break;
          case 2:
            cVar14 = (char)local_8c;
            break;
          case 3:
            cVar14 = (byte)local_a0;
            break;
          case 4:
            goto code_r0x00014233fc5c;
          case 5:
            cVar14 = local_228._6_1_ + cVar14;
            local_228._0_7_ = CONCAT16(cVar14,(undefined6)local_228);
            local_a8 = puVar8;
            goto LAB_14233faff;
          case 6:
            (&UNK_14304de43)[uVar41] = (&UNK_14304de43)[uVar41] + bVar43;
            in_NT = (auStack_2c0[1] & 0x4000) != 0;
            bVar49 = (auStack_2c0[1] & 0x400) != 0;
            in_IF = (auStack_2c0[1] & 0x200) != 0;
            in_TF = (auStack_2c0[1] & 0x100) != 0;
            in_AF = (auStack_2c0[1] & 0x10) != 0;
            in_ID = (auStack_2c0[1] & 0x200000) != 0;
            in_AC = (auStack_2c0[1] & 0x40000) != 0;
            in_VIP = 0;
            in_VIF = 0;
            auStack_2c0[1] = 0x48000001;
            pppuVar24 = (undefined ***)((ulonglong)pppuVar23 & 0xffffffff);
          case 7:
            goto code_r0x00014233ffb1;
          case 8:
          case 10:
            *(byte *)pppuVar18 = *(byte *)pppuVar18 << 1 | (char)*(byte *)pppuVar18 < '\0';
            goto code_r0x000142340031;
          case 9:
            bVar49 = CARRY1((byte)local_158,bVar43);
            local_158 = (float)CONCAT31(local_158._1_3_,(byte)local_158 + bVar43);
            *(byte *)pppuVar18 = *(byte *)pppuVar18 + cVar13 + bVar49;
            goto LAB_14233f3c0;
          case 0xb:
            goto code_r0x00014234002f;
          default:
            if (0xfffffffebcfb2147 >= uVar41 && !bVar50) {
              local_158 = (float)((uint)local_158 & (uint)&local_238);
              (*(code *)*ppuVar28)();
                    // WARNING: Bad instruction - Truncating control flow here
              halt_baddata();
            }
                    // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
          case 0xe:
            if (bVar50 || (longlong)uVar41 < -0x14304deb8) {
              *(byte *)pppuVar18 = *(byte *)pppuVar18 + cVar13;
              *(byte *)pppuVar18 = *(byte *)pppuVar18 + cVar13;
              *(byte *)pppuVar18 = *(byte *)pppuVar18 + cVar14;
                    // WARNING: Bad instruction - Truncating control flow here
              halt_baddata();
            }
                    // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
          case 0x10:
                    // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
          }
          if (cVar14 != '\0') {
            local_b8 = ppuVar39;
            puVar9 = ppuVar39[2];
            if (local_d8 == local_e8) {
              alloc::raw_vec::RawVec::grow_one<tuple_<u32,u32>,alloc::alloc::Global>();
            }
            ppuStack_e0[(longlong)ppuVar12] = puVar9;
            ppuVar45 = (undefined **)((longlong)ppuVar12 + 1);
            local_d8 = ppuVar45;
            ppuVar28 = local_98;
          }
        }
      }
    }
    local_c0 = local_e8;
    local_b0 = ppuStack_e0;
    local_b8 = ppuVar39;
    if (local_d8 != (undefined **)0x0) {
      ppuVar30 = ppuStack_e0 + (longlong)local_d8;
      ppuVar45 = ppuStack_e0;
      do {
        iStack_184 = 0;
        local_15c = 0x7f7fffff;
        local_148 = &local_258;
        ppuStack_140 = local_168;
        local_138 = local_150;
        local_128 = &local_15c;
        local_120 = &local_188;
        puStack_130 = &local_158;
        iter_subtree<tuple_<>,toolbar::ui_interaction::ui_nav_update::closure_0::closure_env_0>();
        if (iStack_184 != 0) {
          *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
          puVar7 = *in_stack_00000028;
          *(undefined8 *)(puVar7 + 0x10) = 1;
          *(ulonglong *)(puVar7 + 0x18) = CONCAT44(iStack_184,local_188);
          if (local_c0 != (undefined **)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          bevy_ecs::component::ComponentDescriptor::
          drop_ptr<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
                    ();
          local_82 = 0;
          bevy_ecs::component::ComponentDescriptor::
          drop_ptr<country_core::resources::tree_storage::TreeStorage>();
          local_81 = 0;
          bevy_ecs::component::ComponentDescriptor::
          drop_ptr<country_core::resources::tree_storage::TreeStorage>();
          return;
        }
        ppuVar45 = ppuVar45 + 1;
      } while (ppuVar45 != ppuVar30);
    }
    pppuVar24 = pppuVar23;
    ppuVar30 = local_d0;
    ppuVar45 = in_stack_00000028;
    puVar9 = puVar7;
    if (local_c0 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      ppuVar30 = local_d0;
    }
code_r0x00014233ffb1:
    local_a8 = puVar9;
    pppuVar23 = pppuVar24;
    goto LAB_14233faff;
  }
  goto LAB_142340005;
  while (ppuVar39 = ppuVar39 + 5, (undefined **)(lVar22 * 0x28) != ppuVar39) {
LAB_14233fc35:
    if (((*(uint *)(lVar40 + 0x14 + (longlong)ppuVar39) == local_a8._4_4_) &&
        (*(int *)(lVar40 + 0x10 + (longlong)ppuVar39) == (int)local_a8)) &&
       (uVar42 = (uint)*(byte *)(lVar40 + 0x24 + (longlong)ppuVar39), uVar42 != 4)) {
      pppuVar18 = (undefined ***)(ulonglong)uVar42;
      goto code_r0x00014233fc5c;
    }
  }
LAB_142340005:
  bevy_ecs::component::ComponentDescriptor::
  drop_ptr<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>();
  local_82 = 0;
  bevy_ecs::component::ComponentDescriptor::
  drop_ptr<country_core::resources::tree_storage::TreeStorage>();
  local_81 = 0;
  bevy_ecs::component::ComponentDescriptor::
  drop_ptr<country_core::resources::tree_storage::TreeStorage>();
code_r0x000142340031:
  pbVar17 = &DAT_14304ded8;
  UNRECOVERED_JUMPTABLE = local_190;
code_r0x00014234003f:
  UNRECOVERED_JUMPTABLE =
       (code *)(pbVar17 + *(int *)(pbVar17 + (longlong)UNRECOVERED_JUMPTABLE * 4));
  goto code_r0x000142340046;
code_r0x00014234002f:
  while (pppuVar24 = (undefined ***)((longlong)pppuVar24 + -1),
        pppuVar24 != (undefined ***)0x0 && !bVar50) {
    puVar31 = puVar32 + 0xffffffffffffffff;
    puVar32 = puVar32 + 0xffffffffffffffff;
    *puVar31 = (ulonglong)(in_NT & 1) * 0x4000 | (ulonglong)SCARRY8(uVar41,0x14304deb8) * 0x800 |
               (ulonglong)bVar49 * 0x400 | (ulonglong)(in_IF & 1) * 0x200 |
               (ulonglong)(in_TF & 1) * 0x100 | (ulonglong)((longlong)pppuVar18 < 0) * 0x80 |
               (ulonglong)bVar50 * 0x40 | (ulonglong)(in_AF & 1) * 0x10 |
               (ulonglong)((POPCOUNT((ulonglong)pppuVar18 & 0xff) & 1U) == 0) * 4 |
               (ulonglong)(0xfffffffebcfb2147 < uVar41) | (ulonglong)(in_ID & 1) * 0x200000 |
               (ulonglong)(in_VIP & 1) * 0x100000 | (ulonglong)(in_VIF & 1) * 0x80000 |
               (ulonglong)(in_AC & 1) * 0x40000;
  }
  goto code_r0x000142340031;
code_r0x00014233fc5c:
  UNRECOVERED_JUMPTABLE = (code *)&DAT_14304dec8;
  pbVar17 = &DAT_14304dec8 + *(int *)(&DAT_14304dec8 + (longlong)pppuVar18 * 4);
  bVar29 = (byte)pbVar17;
  uVar25 = SUB82(ppuVar28,0);
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
  switch(pbVar17) {
  case (byte *)0x14233f3da:
    goto code_r0x00014233f3da;
  case (byte *)0x14234003f:
    goto code_r0x00014234003f;
  case (byte *)0x142340046:
code_r0x000142340046:
                    // WARNING: Could not recover jumptable at 0x00014234004d. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE)(UNRECOVERED_JUMPTABLE,local_198);
    return;
  case (byte *)0x14304dec8:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case (byte *)0x14304ded0:
    out(uVar25,bVar29);
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 & bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 | bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  *pbVar17 = *pbVar17 + bVar29;
  uVar21 = (undefined7)((ulonglong)pbVar17 >> 8);
  piVar19 = (int *)((ulonglong)pbVar17 ^ 0x42);
  *piVar19 = *piVar19 + (int)piVar19;
  cVar14 = (char)piVar19;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  pbVar37 = (byte *)((longlong)&local_158 + (ulonglong)bVar49 * -2 + 1);
  uVar2 = in(uVar25);
  local_158 = (float)CONCAT31((int3)((uint)local_158 >> 8),uVar2);
  pbVar1 = (byte *)((longlong)piVar19 + in_FS_OFFSET + 0x52);
  *pbVar1 = *pbVar1 & (byte)((ulonglong)pbVar17 >> 8);
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + -0x38;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + -0x38;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  *(char *)piVar19 = (char)*piVar19 + cVar14;
  bVar29 = cVar14 * '\x02';
  puVar20 = (uint *)CONCAT71(uVar21,bVar29);
  uVar15 = (uint)puVar20;
  *puVar20 = *puVar20 + uVar15;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 | bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 | bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
                    // WARNING: Read-only address (ram,0x00014304e008) is written
  UNK_14304e008 = 8;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 | bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 | bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  _DAT_fffffffffffffff8 = puVar20;
  *puVar20 = *puVar20 + uVar15;
  bVar50 = CARRY1((byte)*puVar20,bVar29);
  uVar42 = *puVar20;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  bVar51 = (byte)*puVar20 == 0;
  _DAT_fffffffffffffff0 = 0x49646c69;
  pcVar33 = (char *)((longlong)pppuVar23 + (ulonglong)bVar49 * -2 + 1);
  out(*(undefined1 *)pppuVar23,uVar25);
  if ((char)(byte)*puVar20 < '\0') {
    if (!bVar50 && !bVar51) {
      puVar35 = (undefined1 *)
                ((longlong)&local_158 + (ulonglong)bVar49 * -2 + (ulonglong)bVar49 * -2 + 2);
      bVar43 = in(uVar25);
      *pbVar37 = bVar43;
      lVar22 = (longlong)*(int *)((longlong)puVar20 + (longlong)ppuVar28 * 2 + 0x61) * 0x746e6572;
      pcVar34 = pcVar33 + (ulonglong)bVar49 * -8 + 4;
      out(*(undefined4 *)pcVar33,uVar25);
      pcVar33 = pcVar34 + (ulonglong)bVar49 * -2 + 1;
      out(*pcVar34,uVar25);
      if ((int)lVar22 != lVar22 || bVar51) goto code_r0x00014304e096;
      puVar36 = puVar35 + (ulonglong)bVar49 * -2 + 1;
      uVar2 = in(uVar25);
      *puVar35 = uVar2;
      lVar22 = (longlong)*(int *)((longlong)ppuVar30 + (longlong)puVar20 * 2 + 0x68) * 0x4e646c69;
      out(*(undefined4 *)pcVar33,uVar25);
      pcVar34 = pcVar33 + (ulonglong)bVar49 * -8 + 4 + (ulonglong)bVar49 * -2 + 1;
      out(pcVar33[(ulonglong)bVar49 * -8 + 4],uVar25);
      if ((int)lVar22 != lVar22 || bVar51) {
                    // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
      }
      pbVar37 = puVar36 + (ulonglong)bVar49 * -2 + 1;
      uVar2 = in(uVar25);
      *puVar36 = uVar2;
      bVar50 = (longlong)(int)((longlong)iRam00000003c90e9cc6 * 0x4e747570) !=
               (longlong)iRam00000003c90e9cc6 * 0x4e747570;
      goto code_r0x00014304e04e;
    }
    *(byte *)puVar20 = (byte)*puVar20 + bVar29;
code_r0x00014304e087:
    *(char *)ppuVar28 = *(char *)ppuVar28 + -0x22;
    *(byte *)puVar20 = (byte)*puVar20 + bVar29;
    *(byte *)puVar20 = (byte)*puVar20 + bVar29;
    *(byte *)puVar20 = (byte)*puVar20 + bVar29;
  }
  else {
    if (bVar51) {
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    pcVar34 = pcVar33 + (ulonglong)bVar49 * -2 + 1;
    out(*pcVar33,uVar25);
    if (SCARRY1((byte)uVar42,bVar29)) goto code_r0x00014304e0c1;
code_r0x00014304e04e:
    out(*(undefined4 *)pcVar34,uVar25);
    pcVar34 = (char *)(ulonglong)*(uint *)((longlong)ppuVar28 + unaff_GS_OFFSET + 0x61);
    if (bVar51) {
      *(byte *)puVar20 = (byte)*puVar20 + bVar29;
      *(byte *)puVar20 = (byte)*puVar20 + bVar29;
      bVar50 = 0x21 < *pbVar37;
      *pbVar37 = *pbVar37 - 0x22;
code_r0x00014304e0c1:
      if (!bVar50) {
                    // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
      }
      *(byte *)puVar20 = (byte)*puVar20 + bVar29;
      *puVar20 = *puVar20 & uVar15;
      *(byte *)puVar20 = (byte)*puVar20 + bVar29;
      bVar43 = cVar14 * ' ';
      pbVar17 = (byte *)CONCAT71(uVar21,bVar43);
      *(int *)ppuVar39 = *(int *)ppuVar39 + (int)pbVar17;
      *pbVar17 = *pbVar17 + bVar43;
      *(char *)ppuVar39 = *(char *)ppuVar39 + (char)ppuVar39;
      *pbVar17 = *pbVar17 + bVar43;
      *pbVar17 = *pbVar17 + bVar43;
      pcVar34[(longlong)pbVar17] = pcVar34[(longlong)pbVar17] + -0x22;
      bVar29 = *pbVar17;
      *pbVar17 = *pbVar17 + bVar43;
      cVar14 = bVar43 + CARRY1(bVar29,bVar43);
      pcVar33 = (char *)CONCAT71(uVar21,cVar14);
      *pcVar33 = *pcVar33 + cVar14;
      cVar14 = CARRY1(bVar29,bVar43) * '\x10';
      puVar20 = (uint *)CONCAT71(uVar21,cVar14);
      *(uint *)ppuVar39 = *(int *)ppuVar39 + (uint)puVar20;
      *(char *)puVar20 = (char)*puVar20 + cVar14;
      *(char *)ppuVar39 = *(char *)ppuVar39 + (char)ppuVar39;
      *(char *)puVar20 = (char)*puVar20 + cVar14;
      *(char *)puVar20 = (char)*puVar20 + cVar14;
      pcVar34[(longlong)puVar20] = pcVar34[(longlong)puVar20] + -0x22;
      *(char *)puVar20 = (char)*puVar20 + cVar14;
      *puVar20 = *puVar20 & (uint)puVar20;
      *(char *)puVar20 = (char)*puVar20 + cVar14;
      if ((char)*puVar20 == '\0') {
                    // WARNING: Bad instruction - Truncating control flow here
        halt_baddata();
      }
      out(*(undefined4 *)pcVar34,uVar25);
      bVar29 = in(uVar25);
      *pbVar37 = bVar29;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    pcVar33 = pcVar34;
    if (!bVar50) goto code_r0x00014304e087;
    if (!bVar50) {
                    // WARNING: Read-only address (ram,0x00014304dec8) is written
      DAT_14304dec8 = (byte)((ulonglong)ppuVar30 >> 8) ^ 0xa4;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    if (!bVar50) {
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    uVar3 = in(uVar25);
    *(undefined4 *)pbVar37 = uVar3;
    if (bVar50) {
      out(*(undefined4 *)pcVar34,uVar25);
      bVar29 = in(uVar25);
      pbVar37[(ulonglong)bVar49 * -8 + 4] = bVar29;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
  }
  *pcVar33 = *pcVar33 + -0x38;
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
code_r0x00014304e096:
  *(byte *)puVar20 = (byte)*puVar20 + bVar29;
                    // WARNING: Bad instruction - Truncating control flow here
  halt_baddata();
}

