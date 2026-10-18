
void __cdecl system_onboarding::hint_orchestrator::hint_orchestrator(void)

{
  undefined4 uVar1;
  byte bVar2;
  int iVar3;
  longlong lVar4;
  longlong lVar5;
  ulonglong *puVar6;
  ulonglong uVar7;
  undefined8 *puVar8;
  undefined8 ***pppuVar9;
  bool bVar10;
  longlong *plVar11;
  byte extraout_AL;
  undefined8 ****extraout_RAX;
  byte bVar12;
  undefined8 *in_RCX;
  ulonglong uVar13;
  undefined8 *in_RDX;
  longlong lVar14;
  ulonglong uVar15;
  undefined8 uVar16;
  code *pcVar17;
  undefined8 ****ppppuVar18;
  ulonglong uVar19;
  undefined8 ***pppuVar20;
  undefined8 *in_R8;
  ulonglong uVar21;
  longlong *in_R9;
  ulonglong uVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  longlong lVar25;
  byte *pbVar26;
  byte unaff_R13B;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000038;
  undefined8 ***local_1a0;
  undefined8 ***pppuStack_198;
  undefined8 ***local_190;
  undefined8 ***local_188;
  undefined8 ***pppuStack_180;
  undefined8 ***local_178;
  longlong local_170;
  undefined8 *local_158;
  undefined8 local_150;
  longlong local_148;
  longlong *local_140;
  ulonglong local_138;
  undefined8 ***local_130;
  code *local_128;
  undefined8 ***local_118;
  undefined8 ***pppuStack_110;
  undefined8 ***local_108;
  longlong local_100;
  longlong lStack_f8;
  longlong local_f0;
  undefined1 uStack_e8;
  undefined7 uStack_e7;
  undefined1 uStack_e0;
  undefined8 uStack_df;
  undefined8 local_d4;
  undefined4 local_cc;
  undefined1 local_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  longlong local_a0;
  undefined8 ***local_98;
  undefined8 ***pppuStack_90;
  undefined8 ***local_88;
  code *local_80;
  longlong *local_78;
  longlong *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  local_118 = (undefined8 ***)*in_RDX;
  lVar4 = in_RDX[1];
  uVar22 = (ulonglong)*local_118;
  uVar23 = 0;
  uVar13 = uVar22 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar22 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar13 = uVar23;
  }
  uVar21 = uVar22 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar22 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar21 = uVar23;
  }
  pppuStack_110 = (undefined8 ***)(uVar13 * 0x58 + *(longlong *)(lVar4 + 8));
  uVar22 = *(ulonglong *)(lVar4 + 0x10) - uVar13;
  if (*(ulonglong *)(lVar4 + 0x10) < uVar13) {
    pppuStack_110 = (undefined8 ***)&DAT_00000008;
    uVar22 = uVar23;
  }
  local_100 = uVar21 * 0x58 + *(longlong *)(lVar4 + 0x28);
  uVar13 = *(ulonglong *)(lVar4 + 0x30) - uVar21;
  if (*(ulonglong *)(lVar4 + 0x30) < uVar21) {
    local_100 = 8;
    uVar13 = uVar23;
  }
  local_f0 = uVar13 + uVar22;
  *local_118 = (undefined8 **)(*(longlong *)(lVar4 + 0x40) - local_f0);
  local_108 = pppuStack_110 + uVar22 * 0xb;
  lStack_f8 = uVar13 * 0x58 + local_100;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,bevy_ecs::event::iterators::EventIterator<system_onboarding::hint_orchestrator::DisplayHintRequest>_>
            ();
  local_68 = local_140;
  local_118 = &local_98;
  if (1 < local_138) {
    if (local_138 < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::slice::impl_0::sort_by::closure_env_0<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,system_onboarding::hint_orchestrator::hint_orchestrator::closure_env_0>_>
                ();
    }
    else {
      core::slice::sort::stable::
      driftsort_main<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::slice::impl_0::sort_by::closure_env_0<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,system_onboarding::hint_orchestrator::hint_orchestrator::closure_env_0>,alloc::vec::Vec<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::alloc::Global>_>
                ();
    }
  }
  lVar4 = *in_R9;
  lVar25 = in_R9[1];
  if (*(char *)(lVar4 + 0x108) == '\0') {
    lVar14 = 0;
    do {
      if (*(longlong *)(lVar4 + 0x100) << 2 == lVar14) goto LAB_14227fe6f;
      lVar5 = *(longlong *)(lVar25 + 0x100);
      in_R9 = (longlong *)((ulonglong)*(uint *)(*(longlong *)(lVar4 + 0xf8) + lVar14) * 0xa0);
      lVar14 = lVar14 + 4;
    } while (*(longlong *)(lVar5 + 0x58 + (longlong)in_R9) == 0);
    uVar22 = (ulonglong)*(uint *)((longlong)in_R9 + lVar5 + 0x94);
    lVar14 = *(longlong *)(lVar25 + 0x1a8) + uVar22 * 0x48;
    uVar22 = *(ulonglong *)(*(longlong *)(lVar25 + 0x1a8) + 0x40 + uVar22 * 0x48);
    if ((*(ulonglong *)(lVar4 + 0x110) < uVar22) &&
       (uVar13 = *(ulonglong *)(*(longlong *)(lVar14 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8),
       uVar13 != 0)) {
      lVar25 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar13 * 0x30);
      uVar13 = *(ulonglong *)(lVar4 + 0x118);
      uVar23 = *(ulonglong *)((longlong)in_R9 + lVar5 + 0x88);
      if (uVar23 <= uVar13) goto LAB_14227ff30;
LAB_14227fecf:
      if (*(longlong *)(*(longlong *)((longlong)in_R9 + lVar5 + 0x80) + uVar13 * 8) == 0)
      goto LAB_14227ff30;
      bVar10 = true;
      if ((uVar22 <= uVar13) ||
         (uVar22 = *(ulonglong *)(*(longlong *)(lVar14 + 0x38) + uVar13 * 8), uVar22 == 0))
      goto LAB_14227ff33;
      lVar14 = *(longlong *)(*(longlong *)(lVar14 + 0x18) + 0x10 + ~uVar22 * 0x30);
    }
    else {
      lVar25 = 0;
      uVar13 = *(ulonglong *)(lVar4 + 0x118);
      uVar23 = *(ulonglong *)((longlong)in_R9 + lVar5 + 0x88);
      if (uVar13 < uVar23) goto LAB_14227fecf;
LAB_14227ff30:
      bVar10 = false;
LAB_14227ff33:
      lVar14 = 0;
    }
    unaff_R13B = 1;
    if ((*(ulonglong *)(lVar4 + 0x120) < uVar23) &&
       (*(longlong *)
         (*(longlong *)((longlong)in_R9 + lVar5 + 0x80) + *(ulonglong *)(lVar4 + 0x120) * 8) != 0))
    {
      unaff_R13B = 0;
    }
    puVar8 = *(undefined8 **)((longlong)in_R9 + lVar5 + 0x50);
    in_R9 = (longlong *)*puVar8;
    uVar13 = (ulonglong)*(uint *)(puVar8 + 1);
    pbVar26 = (byte *)(lVar25 + uVar13);
    uVar22 = 0;
    if (bVar10) {
      uVar22 = lVar14 + uVar13 * 4;
    }
  }
  else {
    lVar14 = 0;
    do {
      if (*(longlong *)(lVar4 + 0x100) << 2 == lVar14) goto LAB_14227fe6f;
      uVar13 = (ulonglong)*(uint *)(*(longlong *)(lVar4 + 0xf8) + lVar14);
      in_R9 = (longlong *)(uVar13 * 9);
      lVar14 = lVar14 + 4;
    } while (*(longlong *)(*(longlong *)(lVar25 + 0x1a8) + 0x10 + uVar13 * 0x48) == 0);
    lVar25 = *(longlong *)(lVar25 + 0x1a8) + uVar13 * 0x48;
    uVar13 = *(ulonglong *)(lVar25 + 0x40);
    if ((*(ulonglong *)(lVar4 + 0x110) < uVar13) &&
       (uVar22 = *(ulonglong *)(*(longlong *)(lVar25 + 0x38) + *(ulonglong *)(lVar4 + 0x110) * 8),
       uVar22 != 0)) {
      pbVar26 = *(byte **)(*(longlong *)(lVar25 + 0x18) + 0x10 + ~uVar22 * 0x30);
      uVar22 = *(ulonglong *)(lVar4 + 0x118);
      if (uVar13 <= uVar22) goto LAB_14227ff08;
LAB_14227fe87:
      uVar22 = *(ulonglong *)(*(longlong *)(lVar25 + 0x38) + uVar22 * 8);
      if (uVar22 == 0) goto LAB_14227ff08;
      uVar22 = *(ulonglong *)(*(longlong *)(lVar25 + 0x18) + 0x10 + ~uVar22 * 0x30);
      uVar23 = *(ulonglong *)(lVar4 + 0x120);
    }
    else {
      pbVar26 = (byte *)0x0;
      uVar22 = *(ulonglong *)(lVar4 + 0x118);
      if (uVar22 < uVar13) goto LAB_14227fe87;
LAB_14227ff08:
      uVar22 = 0;
      uVar23 = *(ulonglong *)(lVar4 + 0x120);
    }
    unaff_R13B = true;
    if (uVar23 < uVar13) {
      unaff_R13B = *(longlong *)(*(longlong *)(lVar25 + 0x38) + uVar23 * 8) == 0;
    }
    in_R9 = (longlong *)**(undefined8 **)(lVar25 + 8);
  }
LAB_14227ff71:
  puVar6 = (ulonglong *)*in_stack_00000028;
  lVar4 = in_stack_00000028[1];
  uVar13 = *puVar6;
  uVar21 = 0;
  uVar23 = uVar13 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar13 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar23 = uVar21;
  }
  uVar15 = uVar13 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar13 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar15 = uVar21;
  }
  uVar13 = *(ulonglong *)(lVar4 + 0x30);
  uVar24 = uVar13 - uVar15;
  if (uVar15 > uVar13) {
    uVar24 = uVar21;
  }
  uVar7 = *(ulonglong *)(lVar4 + 0x10);
  uVar19 = uVar7 - uVar23;
  if (uVar23 > uVar7) {
    uVar19 = uVar21;
  }
  uVar21 = *(ulonglong *)(lVar4 + 0x40);
  *puVar6 = uVar21 - (uVar19 + uVar24);
  if (uVar23 <= uVar7 && uVar7 - uVar23 != 0 || uVar15 <= uVar13 && uVar13 - uVar15 != 0) {
    *puVar6 = uVar21;
    bVar12 = 1;
  }
  else {
    iVar3 = *(int *)*in_RCX;
    bVar12 = 1;
    if (iVar3 != 0xd) {
      country_core::resources::gui::EscMenuState::is_open();
      bVar12 = iVar3 == 0x13 | extraout_AL;
    }
  }
  puVar6 = (ulonglong *)*in_R8;
  lVar4 = in_R8[1];
  uVar13 = *puVar6;
  uVar21 = 0;
  uVar23 = uVar13 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar13 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar23 = uVar21;
  }
  uVar15 = uVar13 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar13 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar15 = uVar21;
  }
  uVar13 = *(ulonglong *)(lVar4 + 0x30);
  uVar24 = uVar13 - uVar15;
  if (uVar13 < uVar15) {
    uVar24 = uVar21;
  }
  uVar7 = *(ulonglong *)(lVar4 + 0x10);
  uVar19 = uVar7 - uVar23;
  if (uVar7 < uVar23) {
    uVar19 = uVar21;
  }
  uVar21 = *(ulonglong *)(lVar4 + 0x40);
  *puVar6 = uVar21 - (uVar19 + uVar24);
  if ((uVar7 >= uVar23 && uVar7 - uVar23 != 0 || uVar13 >= uVar15 && uVar13 - uVar15 != 0) &&
     (*puVar6 = uVar21, bVar12 == 0)) goto LAB_1422803ce;
  if (pbVar26 == (byte *)0x0) {
    if (local_138 == 0) goto LAB_1422803ce;
    bVar2 = *(byte *)(*local_68 + 0x49);
    if ((bVar12 & bVar2 < 0x12) != 0) goto LAB_1422803ce;
    if (bVar2 == 0x12) {
      uVar16 = 0x12;
      lang_lib::lookup();
      local_130 = &local_98;
      local_128 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_118 = (undefined8 ***)&DAT_1430388f0;
      pppuStack_110 = (undefined8 ***)0x2;
      local_108 = &local_130;
      local_100 = 1;
      lStack_f8 = 0;
      pppuStack_90 = (undefined8 ***)uVar16;
      alloc::fmt::format::format_inner();
      local_98 = local_1a0;
      pppuStack_90 = pppuStack_198;
      local_88 = local_190;
    }
    else if (bVar2 == 2) {
      uVar16 = 0x10;
      lang_lib::lookup();
      pcVar17 = (code *)0x10;
      local_150 = uVar16;
      lang_lib::lookup();
      local_98 = (undefined8 ***)&local_158;
      pppuStack_90 = (undefined8 ***)
                     anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>
      ;
      local_88 = &local_130;
      local_80 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_118 = (undefined8 ***)&DAT_1430388b8;
      pppuStack_110 = (undefined8 ***)0x2;
      local_108 = &local_98;
      local_100 = 2;
      lStack_f8 = 0;
      local_130 = extraout_RAX;
      local_128 = pcVar17;
      alloc::fmt::format::format_inner();
      local_98 = local_188;
      pppuStack_90 = pppuStack_180;
      local_88 = local_178;
    }
    else {
      local_98 = (undefined8 ***)(*local_68 + 0x49);
      pppuStack_90 = (undefined8 ***)impl$14::fmt;
      local_118 = (undefined8 ***)&DAT_143038918;
      pppuStack_110 = (undefined8 ****)0x1;
      local_108 = &local_98;
      local_100 = 1;
      lStack_f8 = 0;
      alloc::fmt::format::format_inner();
      local_a0 = local_170;
      lang_lib::lookup();
      alloc::str::impl$5::to_uppercase();
      local_88 = local_108;
      local_98 = local_118;
      pppuStack_90 = pppuStack_110;
      if (local_a0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    local_108 = local_88;
    local_118 = local_98;
    pppuStack_110 = pppuStack_90;
    puVar8 = (undefined8 *)*local_68;
    local_c8 = *(undefined1 *)((longlong)puVar8 + 0x49);
    local_b0 = puVar8[2];
    local_c0 = *puVar8;
    uStack_b8 = puVar8[1];
    local_d4 = 0;
    local_cc = 0;
    local_100 = 0;
    lStack_f8 = 0;
    local_f0 = 0;
    uStack_e8 = 0;
    uStack_e7 = 0;
    uStack_e0 = 0;
    uStack_df = 0;
    bevy_ecs::system::commands::Commands::
    spawn<tuple_<system_onboarding::hint_orchestrator::HintLayoutMemo,system_onboarding::hint_display_system::HintSpring,system_onboarding::hint_display_system::HintAnimation,system_onboarding::hint_display_system::HintText,system_onboarding::hint_orchestrator::HintType,enum2_<system_onboarding::hint_orchestrator::HintPlacement>_>_>
              ();
    plVar11 = local_78;
    if (*(int *)*local_68 == 10) {
      ppppuVar18 = (undefined8 ****)pppuStack_90;
      if ((undefined8 ****)pppuStack_90 == (undefined8 ****)0x0) {
        ppppuVar18 = (undefined8 ****)local_88;
      }
      pppuVar20 = ppppuVar18[2];
      if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      pppuVar9 = ppppuVar18[1];
      *(code **)((longlong)pppuVar9 + (longlong)pppuVar20) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<system_onboarding::hint_orchestrator::InteractionPrompt>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20) = bevy_ecs::error::handler::panic;
      *(longlong **)((longlong)(pppuVar9 + 2) + (longlong)pppuVar20) = plVar11;
      *(undefined1 *)((longlong)(pppuVar9 + 3) + (longlong)pppuVar20) = 0;
      ppppuVar18[2] = pppuVar20 + 4;
    }
    impl$30::clone();
    bevy_ecs::system::commands::EntityCommands::
    insert<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
    plVar11 = local_78;
    if (*(char *)(*local_68 + 0x48) == '\0') goto LAB_1422803ce;
    ppppuVar18 = (undefined8 ****)pppuStack_90;
    if ((undefined8 ****)pppuStack_90 == (undefined8 ****)0x0) {
      ppppuVar18 = (undefined8 ****)local_88;
    }
    pppuVar20 = ppppuVar18[2];
    if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x28) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    pppuVar9 = ppppuVar18[1];
    *(code **)((longlong)pppuVar9 + (longlong)pppuVar20) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<system_onboarding::hint_orchestrator::HintProgressBar>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(code **)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20) = bevy_ecs::error::handler::panic;
    *(longlong **)((longlong)(pppuVar9 + 2) + (longlong)pppuVar20) = plVar11;
    *(undefined8 *)((longlong)(pppuVar9 + 3) + (longlong)pppuVar20) = 0;
    *(undefined1 *)((longlong)(pppuVar9 + 4) + (longlong)pppuVar20) = 0;
    pppuVar20 = pppuVar20 + 5;
  }
  else {
    bVar2 = *pbVar26;
    if ((bVar12 & (ulonglong)bVar2 < 0x12) == 0) {
      if (local_138 != 0) {
        puVar8 = (undefined8 *)*local_68;
        if (bVar2 == *(byte *)((longlong)puVar8 + 0x49)) {
          local_98 = (undefined8 ***)*in_stack_00000038;
          pppuStack_90 = (undefined8 ***)in_stack_00000038[1];
          local_80 = (code *)in_stack_00000038[3];
          local_88 = (undefined8 ***)in_stack_00000038[2];
          local_108 = (undefined8 ***)puVar8[2];
          local_118 = (undefined8 ***)*puVar8;
          pppuStack_110 = (undefined8 ***)puVar8[1];
          ppppuVar18 = (undefined8 ****)pppuStack_90;
          if ((undefined8 ****)pppuStack_90 == (undefined8 ****)0x0) {
            ppppuVar18 = (undefined8 ****)local_88;
          }
          pppuVar20 = ppppuVar18[2];
          local_78 = in_R9;
          if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x38) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pppuVar9 = ppppuVar18[1];
          *(code **)((longlong)pppuVar9 + (longlong)pppuVar20) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<enum2_<system_onboarding::hint_orchestrator::HintPlacement>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(undefined8 *)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20) = local_118;
          ((undefined8 *)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20))[1] = pppuStack_110;
          *(undefined8 ****)((longlong)(pppuVar9 + 3) + (longlong)pppuVar20) = local_108;
          *(undefined1 *)((longlong)(pppuVar9 + 4) + (longlong)pppuVar20) = 0;
          *(longlong **)((longlong)(pppuVar9 + 5) + (longlong)pppuVar20) = in_R9;
          *(code **)((longlong)(pppuVar9 + 6) + (longlong)pppuVar20) =
               bevy_ecs::error::handler::panic;
          pppuVar20 = pppuVar20 + 7;
          ppppuVar18[2] = pppuVar20;
          if (*(int *)(*local_68 + 0x18) != 0xc4) {
            impl$30::clone();
            bevy_ecs::system::commands::EntityCommands::
            insert<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>();
            pppuVar20 = ppppuVar18[2];
          }
          if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x18) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          plVar11 = local_78;
          pppuVar9 = ppppuVar18[1];
          *(code **)((longlong)pppuVar9 + (longlong)pppuVar20) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<system_onboarding::hint_display_system::HintMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20) =
               bevy_ecs::error::handler::warn;
          *(longlong **)((longlong)(pppuVar9 + 2) + (longlong)pppuVar20) = in_R9;
          ppppuVar18[2] = pppuVar20 + 3;
          ppppuVar18 = (undefined8 ****)pppuStack_90;
          if ((undefined8 ****)pppuStack_90 == (undefined8 ****)0x0) {
            ppppuVar18 = (undefined8 ****)local_88;
          }
          pppuVar20 = ppppuVar18[2];
          if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x18) {
            alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          }
          pppuVar9 = ppppuVar18[1];
          *(code **)((longlong)pppuVar9 + (longlong)pppuVar20) =
               core::ops::function::FnOnce::
               call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::remove::closure_env_0<system_onboarding::hint_display_system::HintDeleteAfterCooldown>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          ;
          *(code **)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20) =
               bevy_ecs::error::handler::warn;
          *(longlong **)((longlong)(pppuVar9 + 2) + (longlong)pppuVar20) = plVar11;
          pppuVar20 = pppuVar20 + 3;
          goto LAB_1422803ca;
        }
        ppppuVar18 = (undefined8 ****)in_stack_00000038[1];
        if (ppppuVar18 == (undefined8 ****)0x0) {
          ppppuVar18 = (undefined8 ****)in_stack_00000038[2];
        }
        pppuVar20 = ppppuVar18[2];
        if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x20) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        }
        goto LAB_1422803a1;
      }
      if ((~unaff_R13B & 1) != 0 || uVar22 != 0) goto LAB_1422803ce;
      ppppuVar18 = (undefined8 ****)in_stack_00000038[1];
      if (ppppuVar18 == (undefined8 ****)0x0) {
        ppppuVar18 = (undefined8 ****)in_stack_00000038[2];
      }
      uVar1 = *(undefined4 *)(&DAT_143038ae8 + (ulonglong)bVar2 * 4);
      pppuVar20 = ppppuVar18[2];
      if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      pppuVar9 = ppppuVar18[1];
      *(code **)((longlong)pppuVar9 + (longlong)pppuVar20) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<system_onboarding::hint_display_system::HintDeleteAfterCooldown>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20) = bevy_ecs::error::handler::panic;
      *(longlong **)((longlong)(pppuVar9 + 2) + (longlong)pppuVar20) = in_R9;
      *(undefined4 *)((longlong)(pppuVar9 + 3) + (longlong)pppuVar20) = uVar1;
      *(undefined1 *)((longlong)pppuVar9 + 0x1c + (longlong)pppuVar20) = 0;
    }
    else {
      ppppuVar18 = (undefined8 ****)in_stack_00000038[1];
      if (ppppuVar18 == (undefined8 ****)0x0) {
        ppppuVar18 = (undefined8 ****)in_stack_00000038[2];
      }
      pppuVar20 = ppppuVar18[2];
      if ((ulonglong)((longlong)*ppppuVar18 - (longlong)pppuVar20) < 0x20) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
LAB_1422803a1:
      pppuVar9 = ppppuVar18[1];
      *(code **)((longlong)pppuVar9 + (longlong)pppuVar20) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<system_onboarding::hint_display_system::HintMarkedForDeletion>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(code **)((longlong)(pppuVar9 + 1) + (longlong)pppuVar20) = bevy_ecs::error::handler::panic;
      *(longlong **)((longlong)(pppuVar9 + 2) + (longlong)pppuVar20) = in_R9;
      *(undefined1 *)((longlong)(pppuVar9 + 3) + (longlong)pppuVar20) = 0;
    }
    pppuVar20 = pppuVar20 + 4;
  }
LAB_1422803ca:
  ppppuVar18[2] = pppuVar20;
LAB_1422803ce:
  if (local_148 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
LAB_14227fe6f:
  pbVar26 = (byte *)0x0;
  goto LAB_14227ff71;
}

