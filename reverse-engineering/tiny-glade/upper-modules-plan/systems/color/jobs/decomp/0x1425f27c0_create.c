// name=create
// sig=void __cdecl create(void)
// size=1947

// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000143136f28) overlaps instruction at (ram,0x000143136f26)
// 

void __cdecl system_color::radial_menu::ColorRadialMenu::create(void)

{
  byte bVar1;
  uint uVar2;
  undefined8 *puVar3;
  ulonglong *puVar4;
  longlong lVar5;
  byte bVar6;
  char cVar7;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  char *pcVar9;
  int *piVar10;
  ulonglong *in_RCX;
  code *pcVar11;
  longlong lVar12;
  undefined **ppuVar13;
  ulonglong uVar14;
  longlong *plVar15;
  undefined1 *puVar16;
  undefined **ppuVar18;
  longlong lVar19;
  ulonglong *puVar20;
  undefined8 *in_R11;
  ulonglong uVar21;
  longlong *plVar22;
  undefined1 uVar23;
  float fVar24;
  float extraout_XMM0_Da;
  float fVar25;
  float in_XMM1_Da;
  float fVar26;
  float in_XMM2_Da;
  float in_XMM3_Da;
  float fVar27;
  float _X;
  float in_stack_00000028;
  float in_stack_00000030;
  float in_stack_00000038;
  float in_stack_00000040;
  longlong *in_stack_00000048;
  undefined8 *in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 *in_stack_00000060;
  undefined **in_stack_00000068;
  undefined8 auStack_3d8 [19];
  longlong lStack_340;
  undefined1 auStack_338 [32];
  ulonglong auStack_318 [4];
  ulonglong local_2f8;
  uint uStack_2f0;
  undefined8 uStack_2e0;
  float fStack_2d8;
  float fStack_2d4;
  undefined8 uStack_2d0;
  float afStack_2c0 [2];
  undefined1 local_2b8;
  undefined1 local_2b7;
  undefined8 local_2b0;
  undefined8 local_268 [2];
  longlong local_258;
  longlong local_250;
  longlong local_248;
  undefined4 local_240;
  undefined8 local_238;
  undefined8 uStack_230;
  ulonglong local_228;
  ulonglong local_220;
  float local_20c;
  longlong *local_208;
  undefined8 local_200;
  undefined8 uStack_1f8;
  undefined8 local_1f0;
  undefined8 uStack_1e8;
  undefined4 local_1e0;
  undefined8 local_1d4;
  undefined4 local_1cc;
  ulonglong *local_1c8;
  ulonglong *local_1c0;
  longlong local_1b0;
  ulonglong local_1a8;
  longlong local_1a0;
  longlong local_198;
  ulonglong *local_190;
  ulonglong *local_188;
  ulonglong local_180;
  float local_178;
  float local_174;
  float local_170;
  float local_16c;
  undefined4 local_168;
  float local_164;
  float local_160;
  uint local_15c;
  ulonglong local_158;
  ulonglong uStack_150;
  ulonglong local_148;
  ulonglong local_140;
  uint local_134;
  longlong *plStack_128;
  longlong *plStack_120;
  ulonglong uStack_110;
  undefined2 local_102;
  uint local_100;
  char local_fc;
  byte local_fb;
  byte local_fa;
  char local_f9;
  float local_f8;
  undefined1 local_f3;
  char local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  short sVar8;
  undefined8 *puVar17;
  
  puVar16 = auStack_338;
  puVar3 = (undefined8 *)&local_2b8;
  local_f0 = 0xfffffffffffffffe;
  uVar14 = in_stack_00000048[2];
  if (uVar14 == 0) {
    uVar14 = 0;
  }
  else {
    lVar19 = uVar14 * 0x28;
    lVar12 = 0;
    do {
      uVar14 = uVar14 - (*(char *)(in_stack_00000048[1] + 0x18 + lVar12) == '\x05');
      lVar12 = lVar12 + 0x28;
    } while (lVar19 != lVar12);
  }
  if ((uVar14 >> 0x3d == 0) && (uVar14 * 8 < 0x7ffffffffffffff9)) {
    if (uVar14 * 8 == 0) {
      uStack_150 = 8;
      local_158 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      uStack_150 = extraout_RAX;
      local_158 = uVar14;
      if (extraout_RAX == 0) goto LAB_1425f3755;
    }
    local_148 = 0;
    fVar26 = ((in_stack_00000030 + in_stack_00000038) * ((float)(longlong)uVar14 + -1.0)) /
             in_XMM3_Da;
    fVar27 = fVar26 * -0.5;
    ::utils::angle_range::AngleRange::from_min_and_len();
    alloc::vec::impl$11::clone<system_color::radial_menu::IconInfo,alloc::alloc::Global>();
    uVar14 = local_2f8;
    local_228 = auStack_318[3];
    if (local_2f8 < auStack_318[2]) {
      if (local_2f8 == 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        local_228 = 8;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_realloc();
        local_228 = extraout_RAX_00;
        if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
      }
    }
    local_258 = 0;
    local_250 = 8;
    local_248 = 0;
    local_238 = *in_stack_00000050;
    uStack_230 = in_stack_00000050[1];
    local_200 = *in_stack_00000058;
    uStack_1f8 = in_stack_00000058[1];
    local_1f0 = in_stack_00000058[2];
    uStack_1e8 = in_stack_00000058[3];
    local_1e0 = *(undefined4 *)(in_stack_00000058 + 4);
    local_240 = 0;
    local_1d4 = *in_stack_00000060;
    local_1cc = *(undefined4 *)(in_stack_00000060 + 1);
    local_268[0] = 0;
    local_208 = (longlong *)0x0;
    local_220 = uVar14;
    puVar20 = (ulonglong *)in_stack_00000048[1];
    local_1b0 = in_stack_00000048[2];
    in_stack_00000048[2] = 0;
    local_1c0 = puVar20 + local_1b0 * 5;
    local_1a8 = 0;
    local_20c = fVar26;
    local_190 = in_RCX;
    local_188 = local_1c0;
    if (local_1b0 != 0) {
      fVar26 = (fVar27 + 0.7853982 + in_stack_00000028) * in_XMM3_Da;
      local_fc = *(char *)((longlong)in_stack_00000058 + 0x1c);
      local_170 = *(float *)(in_stack_00000058 + 1);
      local_fb = *(byte *)(in_stack_00000058 + 2);
      local_160 = *(float *)((longlong)in_stack_00000058 + 0x14);
      local_f9 = *(char *)((longlong)in_stack_00000058 + 0x11);
      local_15c = *(uint *)(in_stack_00000058 + 3);
      local_164 = *(float *)((longlong)in_stack_00000058 + 4);
      local_16c = *(float *)((longlong)in_stack_00000058 + 0xc);
      local_fa = *(byte *)((longlong)in_stack_00000058 + 0x1d);
      ppuVar13 = (undefined **)(ulonglong)local_fa;
      local_168 = *(undefined4 *)in_stack_00000058;
      local_178 = in_stack_00000030 * 0.75;
      local_174 = in_stack_00000030 * 0.25;
      lVar19 = local_1b0 * 0x28;
      lVar12 = 1;
      local_100 = 0;
      puVar16 = auStack_338;
      puVar4 = puVar20;
      do {
        puVar20 = puVar4 + 5;
        local_140 = *puVar4;
        local_1c8 = puVar20;
        if (SBORROW8(0,local_140)) break;
        plVar22 = (longlong *)(lVar12 - 1);
        local_180 = puVar4[1];
        uVar14 = puVar4[2];
        bVar1 = (byte)puVar4[3];
        plVar15 = (longlong *)(ulonglong)bVar1;
        local_f2 = *(char *)((longlong)puVar4 + 0x19);
        local_102 = *(undefined2 *)((longlong)puVar4 + 0x1a);
        local_f8 = *(float *)((longlong)puVar4 + 0x1c);
        local_134 = (uint)puVar4[4];
        uVar2 = *(uint *)((longlong)puVar4 + 0x24);
        uVar21 = (ulonglong)uVar2;
        bVar6 = bVar1 - 3;
        if (0xb < bVar6) {
          bVar6 = 8;
        }
        ppuVar18 = (undefined **)(ulonglong)bVar6;
        local_1a0 = lVar12;
        local_198 = lVar19;
        if (bVar6 == 2) {
          fVar26 = fVar26 - local_174;
          *(undefined8 *)(puVar16 + -8) = 0x1425f2cd1;
          fVar24 = cosf(fVar26 / in_XMM3_Da);
          *(undefined8 *)(puVar16 + -8) = 0x1425f2cde;
          fVar27 = sinf(fVar26 / in_XMM3_Da);
          pcVar9 = (char *)0x0;
          _X = local_178;
        }
        else {
          _X = fVar26 / in_XMM3_Da;
          *(undefined8 *)(puVar16 + -8) = 0x1425f2d12;
          fVar24 = cosf(_X);
          *(undefined8 *)(puVar16 + -8) = 0x1425f2d1f;
          fVar25 = sinf(_X);
          uVar23 = SUB81(ppuVar13,0);
          pcVar11 = (code *)&DAT_143136f20;
          pcVar9 = &DAT_143136f20 + *(int *)(&DAT_143136f20 + (longlong)ppuVar18 * 4);
          sVar8 = (short)pcVar9;
          cVar7 = (char)pcVar9;
          fVar27 = fVar25;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
          switch(ppuVar18) {
          case (undefined **)0x6:
            local_f1 = 1;
            pcVar11 = (code *)0x0;
            *(undefined **)(puVar16 + -8) = &UNK_1425f2dc1;
            country_core::resources::color_icon::ColorMenuBrightnessValue::to_f32();
            if (ABS(local_164 - extraout_XMM0_Da) < 0.001) {
LAB_1425f2e2d:
              if ((local_100 & 1) == 0) {
                local_208 = plVar22;
              }
              pcVar9 = (char *)0x1;
              local_100 = (uint)CONCAT71((int7)((ulonglong)pcVar11 >> 8),1);
              _X = in_stack_00000030;
              goto LAB_1425f2e60;
            }
            break;
          case (undefined **)0x8:
            if (local_fb != bVar1) break;
            if ((local_fb != 2) ||
               (((local_160 == local_f8 && (local_f9 == local_f2)) &&
                (pcVar11 = (code *)(ulonglong)local_134, local_15c == local_134))))
            goto LAB_1425f2e2d;
            goto LAB_1425f2d35;
          case (undefined **)0x9:
            if ((local_170 == local_f8) && (!NAN(local_170) && !NAN(local_f8))) goto LAB_1425f2e2d;
            break;
          case (undefined **)0xa:
            if (local_fc == local_f2) goto LAB_1425f2e2d;
            break;
          case (undefined **)0xb:
            if (local_16c == local_f8) goto LAB_1425f2e2d;
            break;
          case (undefined **)0xc:
            *pcVar9 = *pcVar9 + cVar7;
            goto LAB_1425f2eb9;
          case (undefined **)0xd:
                    // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
          case (undefined **)0xe:
            goto code_r0x0001425f3050;
          case (undefined **)0xf:
            *(undefined4 *)
             (&UNK_143136f28 +
             (longlong)ppuVar18 + (longlong)*(int *)(&DAT_143136f20 + (longlong)ppuVar18 * 4)) =
                 0x43136f20;
            goto code_r0x0001425f30ae;
          case (undefined **)0x10:
                    // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
          case (undefined **)0x11:
            piVar10 = (int *)(ulonglong)(uint)(int)sVar8;
            *piVar10 = *piVar10 + (int)sVar8;
            *(char *)((longlong)plVar15 + 0x50c4dbc25) =
                 *(char *)((longlong)plVar15 + 0x50c4dbc25) + ' ';
            *(char *)piVar10 = (char)*piVar10 + cVar7;
            goto code_r0x0001425f3188;
          case (undefined **)0x12:
            goto code_r0x0001425f31f1;
          case (undefined **)0x13:
            puVar16 = puVar16 + 8;
            piVar10 = (int *)(ulonglong)(uint)(int)sVar8;
            *piVar10 = *piVar10 + (int)sVar8;
            *(char *)((longlong)plVar15 + 0x50c4dbc25) =
                 *(char *)((longlong)plVar15 + 0x50c4dbc25) + ' ';
            *(char *)piVar10 = (char)*piVar10 + cVar7;
            goto code_r0x0001425f310f;
          case (undefined **)0x14:
            pcVar11 = (code *)swi(3);
            (*pcVar11)();
            return;
          case (undefined **)0x15:
            puVar16 = puVar16 + 8;
            piVar10 = (int *)(ulonglong)(uint)(int)sVar8;
            *piVar10 = *piVar10 + (int)sVar8;
            *(char *)((longlong)plVar15 + 0x50c4dbc25) =
                 *(char *)((longlong)plVar15 + 0x50c4dbc25) + ' ';
            *(char *)piVar10 = (char)*piVar10 + cVar7;
            goto code_r0x0001425f2ffa;
          case (undefined **)0x16:
            goto code_r0x0001425f322d;
          case (undefined **)0x17:
            puVar16 = puVar16 + 8;
            piVar10 = (int *)(ulonglong)(uint)(int)sVar8;
            *piVar10 = *piVar10 + (int)sVar8;
            *(char *)((longlong)plVar15 + 0x50c4dbc25) =
                 *(char *)((longlong)plVar15 + 0x50c4dbc25) + ' ';
            *(char *)piVar10 = (char)*piVar10 + cVar7;
            goto code_r0x0001425f2f81;
          case (undefined **)0x18:
          case (undefined **)0x19:
          case (undefined **)0x1b:
          case (undefined **)0x1d:
            *(longlong *)(lVar12 + -0x38) = *(longlong *)(lVar12 + -0x38) + 1;
            pcVar11 = (code *)*in_R11;
            *(undefined **)(puVar16 + -8) = &UNK_143136f2d;
            (*pcVar11)();
            goto code_r0x000143136f2d;
          case (undefined **)0x1a:
          case (undefined **)0x1c:
            puVar17 = (undefined8 *)(puVar16 + -8);
            *(undefined8 **)(puVar16 + -8) = puVar3;
            cVar7 = '\x12';
            do {
              puVar3 = puVar3 + -1;
              puVar17 = puVar17 + -1;
              *puVar17 = *puVar3;
              cVar7 = cVar7 + -1;
            } while ('\0' < cVar7);
            *(undefined1 **)(puVar16 + -0xa0) = puVar16 + -8;
            puVar16 = puVar16 + -0xffeb;
code_r0x000143136f2d:
            pcVar11 = (code *)*in_R11;
            *(undefined **)(puVar16 + -8) = &UNK_143136f35;
            (*pcVar11)();
            pcVar11 = (code *)*in_R11;
            *(undefined **)(puVar16 + -8) = &UNK_143136f3d;
            (*pcVar11)();
                    // WARNING: Bad instruction - Truncating control flow here
            halt_baddata();
          }
LAB_1425f2d35:
          pcVar9 = (char *)0x0;
          _X = in_stack_00000030;
        }
LAB_1425f2e60:
        auStack_318[2] = local_140;
        auStack_318[3] = local_180;
        pcVar11 = (code *)0x0;
        uVar23 = bVar1 == 5;
        local_2f8 = uVar14;
        if ((longlong)plVar22 < 0) {
          fVar25 = (float)plVar22;
          plVar22 = (longlong *)((ulonglong)((uint)plVar22 & 1) | (ulonglong)plVar22 >> 1);
        }
        else {
          fVar25 = (float)(longlong)plVar22;
        }
LAB_1425f2eb9:
        fStack_2d8 = *(float *)(&__real_3e99999a3f800000 +
                               CONCAT71((int7)((ulonglong)pcVar11 >> 8),uVar23) * 4) * _X;
        afStack_2c0[0] =
             (fVar25 * -0.1 + (float)(-(uint)(0.0 < in_stack_00000028) & 0xbf400000)) *
             (1.0 / in_stack_00000040);
        uStack_2e0 = 0;
        uStack_2d0 = CONCAT44(fVar27 * in_XMM3_Da + in_XMM2_Da,fVar24 * in_XMM3_Da + in_XMM1_Da);
        afStack_2c0[1] = 0.0;
        local_2b8 = 0;
        local_2b7 = SUB81(pcVar9,0);
        local_2b0 = 1;
        local_f1 = 0;
        uStack_2f0 = uVar2;
        fStack_2d4 = fStack_2d8;
        *(undefined8 *)(puVar16 + -8) = 0x1425f2f57;
        ppuVar13 = in_stack_00000068;
        bevy_ecs::system::commands::Commands::
        spawn<tuple_<country_core::components::Ui2dExtent,country_core::components::Ui2dImage,country_core::resources::color_icon::CustomizationIcon,country_core::components::UiDrawOrder>_>
                  ();
        uVar21 = uStack_110;
        plVar15 = plStack_128;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
        switch(bVar6) {
        case 0:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
code_r0x0001425f2f81:
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f34ef;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pcVar9 = (char *)plVar15[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::ColorIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          goto code_r0x0001425f3057;
        case 1:
          plVar22 = plStack_128;
          if (plStack_128 == (longlong *)0x0) {
            plVar22 = plStack_120;
          }
code_r0x0001425f3188:
          ppuVar18 = (undefined **)plVar22[2];
          if ((ulonglong)(*plVar22 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f351b;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar19 = plVar22[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::WallStyleIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          goto code_r0x0001425f3297;
        case 2:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f346b;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
code_r0x0001425f30ae:
          lVar19 = plVar15[1];
          *(code **)(lVar19 + (longlong)ppuVar18) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::ColorWheelSeparator>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar19 + 8 + (longlong)ppuVar18) = bevy_ecs::error::handler::panic;
          *(ulonglong *)(lVar19 + 0x10 + (longlong)ppuVar18) = uVar21;
          *(undefined1 *)(lVar19 + 0x18 + (longlong)ppuVar18) = 0;
          plVar15[2] = (longlong)(ppuVar18 + 4);
          lVar19 = local_198;
          if (local_248 == local_258) goto code_r0x0001425f33a4;
          goto LAB_1425f2c00;
        case 3:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
code_r0x0001425f310f:
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f3497;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pcVar9 = (char *)plVar15[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::FlagConnectionIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          break;
        case 4:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
code_r0x0001425f2ffa:
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f3413;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pcVar9 = (char *)plVar15[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::FreehandHeightTypeToggleIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          break;
        case 5:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f3547;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pcVar9 = (char *)plVar15[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::FlatRoofRimTypeToggle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          break;
        case 6:
          plVar22 = plStack_128;
          if (plStack_128 == (longlong *)0x0) {
code_r0x0001425f322d:
            plVar22 = plStack_120;
          }
          uVar21 = uStack_110;
          ppuVar18 = (undefined **)plVar22[2];
          if ((ulonglong)(*plVar22 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f3573;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar19 = plVar22[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::ColorMenuBrightnessIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          goto code_r0x0001425f3297;
        case 7:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f34c3;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pcVar9 = (char *)plVar15[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::ChimneySmokeToggle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          break;
        case 8:
          plVar22 = plStack_128;
          if (plStack_128 == (longlong *)0x0) {
            plVar22 = plStack_120;
          }
          ppuVar18 = (undefined **)plVar22[2];
          if ((ulonglong)(*plVar22 - (longlong)ppuVar18) < 0x28) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined8 *)(puVar16 + -8) = 0x1425f35cb;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar19 = plVar22[1];
          *(code **)(lVar19 + (longlong)ppuVar18) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::FlatRoofStyleIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)(lVar19 + 8 + (longlong)ppuVar18) = bevy_ecs::error::handler::panic;
          *(ulonglong *)(lVar19 + 0x10 + (longlong)ppuVar18) = uVar21;
          *(byte *)(lVar19 + 0x18 + (longlong)ppuVar18) = bVar1;
          *(char *)(lVar19 + 0x19 + (longlong)ppuVar18) = local_f2;
          *(undefined2 *)(lVar19 + 0x1a + (longlong)ppuVar18) = local_102;
          *(float *)(lVar19 + 0x1c + (longlong)ppuVar18) = local_f8;
          *(uint *)(lVar19 + 0x20 + (longlong)ppuVar18) = local_134;
          *(undefined1 *)(lVar19 + 0x24 + (longlong)ppuVar18) = 0;
          ppuVar18 = ppuVar18 + 5;
          goto code_r0x0001425f334c;
        case 9:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f343f;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pcVar9 = (char *)plVar15[1];
code_r0x0001425f3050:
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::PlasterPeelIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          goto code_r0x0001425f3057;
        case 10:
          plVar22 = plStack_128;
          if (plStack_128 == (longlong *)0x0) {
            plVar22 = plStack_120;
          }
          ppuVar18 = (undefined **)plVar22[2];
          if ((ulonglong)(*plVar22 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f359f;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          lVar19 = plVar22[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::StairRailingIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
code_r0x0001425f3297:
          *(code **)(lVar19 + (longlong)ppuVar18) = pcVar11;
          *(code **)(lVar19 + 8 + (longlong)ppuVar18) = bevy_ecs::error::handler::panic;
          *(ulonglong *)(lVar19 + 0x10 + (longlong)ppuVar18) = uVar21;
          *(undefined1 *)(lVar19 + 0x18 + (longlong)ppuVar18) = 0;
          *(char *)(lVar19 + 0x19 + (longlong)ppuVar18) = local_f2;
          ppuVar18 = ppuVar18 + 4;
code_r0x0001425f334c:
          plVar22[2] = (longlong)ppuVar18;
          if (local_148 == local_158) goto code_r0x0001425f3360;
          goto LAB_1425f337a;
        case 0xb:
          if (plStack_128 == (longlong *)0x0) {
            plVar15 = plStack_120;
          }
          ppuVar18 = (undefined **)plVar15[2];
          if ((ulonglong)(*plVar15 - (longlong)ppuVar18) < 0x20) {
            local_f1 = 0;
            *(undefined8 *)(puVar16 + 0x20) = 1;
            *(undefined **)(puVar16 + -8) = &UNK_1425f33e7;
            ppuVar13 = ppuVar18;
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pcVar9 = (char *)plVar15[1];
          pcVar11 = core::ops::function::FnOnce::
                    call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::color_icon::MagicColorIcon>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
code_r0x0001425f3057:
          *(code **)(pcVar9 + (longlong)ppuVar18) = pcVar11;
          *(code **)(pcVar9 + (longlong)(ppuVar18 + 1)) = bevy_ecs::error::handler::panic;
          *(ulonglong *)(pcVar9 + (longlong)(ppuVar18 + 2)) = uVar21;
          *(float *)(pcVar9 + (longlong)(ppuVar18 + 3)) = local_f8;
          pcVar9[(longlong)ppuVar18 + 0x1c] = '\0';
          goto code_r0x0001425f3200;
        default:
                    // WARNING: Bad instruction - Truncating control flow here
          halt_baddata();
        case 0xe:
        case 0x10:
                    // WARNING: Could not recover jumptable at 0x000143136f5a. Too many branches
                    // WARNING: Treating indirect jump as call
          (**(code **)((longlong)plVar22 - 0x6f00b43f))
                    (CONCAT71(0x143136f,
                              (char)*(undefined4 *)(&DAT_143136f50 + (longlong)ppuVar18 * 4) + 0x50U
                              ^ 0x50));
          return;
        }
        *(code **)(pcVar9 + (longlong)ppuVar18) = pcVar11;
        pcVar11 = bevy_ecs::error::handler::panic;
code_r0x0001425f31f1:
        *(code **)(pcVar9 + (longlong)(ppuVar18 + 1)) = pcVar11;
        *(ulonglong *)(pcVar9 + (longlong)(ppuVar18 + 2)) = uVar21;
        pcVar9[(longlong)(ppuVar18 + 3)] = '\0';
code_r0x0001425f3200:
        plVar15[2] = (longlong)(ppuVar18 + 4);
        if (local_148 == local_158) {
code_r0x0001425f3360:
          uVar14 = local_148;
          local_f1 = 0;
          ppuVar13 = &PTR_DAT_143136ed8;
          *(undefined1 **)(puVar16 + -8) = &LAB_1425f337a;
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          local_148 = uVar14;
        }
LAB_1425f337a:
        *(ulonglong *)(uStack_150 + local_148 * 8) = uVar21;
        local_148 = local_148 + 1;
        lVar19 = local_198;
        if (local_248 == local_258) {
code_r0x0001425f33a4:
          lVar19 = local_198;
          lVar5 = local_248;
          local_f1 = 0;
          ppuVar13 = &PTR_DAT_143136ef0;
          *(undefined8 *)(puVar16 + -8) = 0x1425f33bb;
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
          local_248 = lVar5;
        }
LAB_1425f2c00:
        *(ulonglong *)(local_250 + local_248 * 8) = uVar21;
        local_248 = local_248 + 1;
        fVar26 = fVar26 + _X + in_stack_00000038;
        lVar12 = lVar12 + 1;
        lVar19 = lVar19 + -0x28;
        puVar4 = puVar20;
      } while (lVar19 != 0);
    }
    if (local_188 != puVar20) {
      uVar14 = (ulonglong)((longlong)local_188 - (longlong)puVar20) / 0x28;
      puVar20 = puVar20 + 1;
      do {
        if (puVar20[-1] != 0) {
          *(undefined8 *)(puVar16 + -8) = 0x1425f3630;
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        puVar20 = puVar20 + 5;
        uVar14 = uVar14 - 1;
      } while (uVar14 != 0);
    }
    *(undefined8 *)(puVar16 + -8) = 0x1425f3648;
    memcpy(auStack_318 + 2,local_268,0xa0);
    local_f3 = 0;
    *(undefined8 *)(puVar16 + -8) = 0x1425f366c;
    bevy_ecs::system::commands::Commands::spawn<system_color::radial_menu::ColorRadialMenu>();
    local_190[2] = local_148;
    *local_190 = local_158;
    local_190[1] = uStack_150;
    local_190[3] = local_1a8;
    lVar19 = in_stack_00000048[2];
    if (lVar19 != 0) {
      lVar12 = in_stack_00000048[1] + 8;
      do {
        if (*(longlong *)(lVar12 + -8) != 0) {
          *(undefined8 *)(puVar16 + -8) = 0x1425f36d0;
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        lVar12 = lVar12 + 0x28;
        lVar19 = lVar19 + -1;
      } while (lVar19 != 0);
    }
    if (*in_stack_00000048 != 0) {
      *(undefined8 *)(puVar16 + -8) = 0x1425f36f0;
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    return;
  }
LAB_1425f3755:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

