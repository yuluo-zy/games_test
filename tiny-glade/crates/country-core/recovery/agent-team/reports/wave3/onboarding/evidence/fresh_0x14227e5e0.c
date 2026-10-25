
void __cdecl system_onboarding::hint_orchestrator::BindingHintDisplay::resolve_steam_impl(void)

{
  int iVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  longlong *plVar10;
  uint uVar11;
  int *in_RCX;
  int *in_RDX;
  longlong *plVar12;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong lStack_98;
  undefined8 uStack_78;
  int *piStack_70;
  longlong lStack_68;
  
  iVar1 = *in_RDX;
  uVar4 = iVar1 - 0xc4U;
  if (8 < iVar1 - 0xc4U) {
    uVar4 = 4;
  }
  switch(iVar1) {
  default:
    uVar5 = *(undefined8 *)in_RDX;
    uVar6 = *(undefined8 *)(in_RDX + 2);
    uVar7 = *(undefined8 *)(in_RDX + 4);
    uVar8 = *(undefined8 *)(in_RDX + 6);
    uVar9 = *(undefined8 *)(in_RDX + 10);
    *(undefined8 *)(in_RCX + 8) = *(undefined8 *)(in_RDX + 8);
    *(undefined8 *)(in_RCX + 10) = uVar9;
    *(undefined8 *)(in_RCX + 4) = uVar7;
    *(undefined8 *)(in_RCX + 6) = uVar8;
    *(undefined8 *)in_RCX = uVar5;
    *(undefined8 *)(in_RCX + 2) = uVar6;
    return;
  case 0xc9:
    country_core::resources::steam::SteamInput::get_action_origin_glyphs();
    if (SBORROW8(0,lStack_c0)) {
      *in_RCX = 0xc4;
      return;
    }
    alloc::vec::in_place_collect::impl$1::
    from_iter<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<std::path::PathBuf,alloc::alloc::Global>,system_onboarding::hint_display_system::impl_0::resolve_steam_impl::closure_env_1>_>
              ();
    if (lStack_68 == 0) {
      *in_RCX = 0xc4;
      core::ptr::
      drop_in_place<alloc::vec::Vec<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>,alloc::alloc::Global>_>
                ();
    }
    else if (lStack_68 == 1) {
      if (*piStack_70 == 0xcd) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      *in_RCX = *piStack_70;
      uVar5 = *(undefined8 *)(piStack_70 + 3);
      uVar6 = *(undefined8 *)(piStack_70 + 5);
      uVar7 = *(undefined8 *)(piStack_70 + 7);
      uVar8 = *(undefined8 *)(piStack_70 + 8);
      uVar9 = *(undefined8 *)(piStack_70 + 10);
      *(undefined8 *)(in_RCX + 1) = *(undefined8 *)(piStack_70 + 1);
      *(undefined8 *)(in_RCX + 3) = uVar5;
      *(undefined8 *)(in_RCX + 5) = uVar6;
      *(undefined8 *)(in_RCX + 7) = uVar7;
      *(undefined8 *)(in_RCX + 8) = uVar8;
      *(undefined8 *)(in_RCX + 10) = uVar9;
      core::ptr::
      drop_in_place<alloc::vec::into_iter::IntoIter<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>,alloc::alloc::Global>_>
                ();
    }
    else {
      *(longlong *)(in_RCX + 6) = lStack_68;
      *(undefined8 *)(in_RCX + 2) = uStack_78;
      *(int **)(in_RCX + 4) = piStack_70;
      *in_RCX = 0xca;
    }
    break;
  case 0xca:
    uVar5 = *(undefined8 *)(in_RDX + 2);
    plVar10 = *(longlong **)(in_RDX + 4);
    lVar2 = *(longlong *)(in_RDX + 6);
    for (plVar12 = plVar10; plVar12 != plVar10 + lVar2 * 6; plVar12 = plVar12 + 6) {
      *(int *)plVar12 = 0xc4;
      resolve_steam_impl();
      uVar11 = (int)*plVar12 - 0xc4;
      if (8 < uVar11) {
        uVar11 = 4;
      }
      switch(uVar11) {
      case 0:
      case 2:
      case 3:
      case 4:
      case 5:
        goto code_r0x00014227e735;
      case 1:
        lVar3 = plVar12[1];
        if (!SBORROW8(0,lVar3)) goto joined_r0x00014227e846;
        if ((SBORROW8(0,plVar12[2])) || (plVar12[2] == 0)) goto code_r0x00014227e735;
        goto code_r0x00014227e730;
      case 6:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
        break;
      case 7:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
        break;
      default:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
      }
      lVar3 = plVar12[1];
joined_r0x00014227e846:
      if (lVar3 != 0) {
code_r0x00014227e730:
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
code_r0x00014227e735:
      plVar12[4] = lStack_a0;
      plVar12[5] = lStack_98;
      plVar12[2] = lStack_b0;
      plVar12[3] = lStack_a8;
      *plVar12 = lStack_c0;
      plVar12[1] = lStack_b8;
    }
    *(longlong *)(in_RCX + 6) = lVar2;
    *(undefined8 *)(in_RCX + 2) = uVar5;
    *(longlong **)(in_RCX + 4) = plVar10;
    *in_RCX = 0xca;
    break;
  case 0xcb:
    uVar5 = *(undefined8 *)(in_RDX + 2);
    plVar10 = *(longlong **)(in_RDX + 4);
    lVar2 = *(longlong *)(in_RDX + 6);
    for (plVar12 = plVar10; plVar12 != plVar10 + lVar2 * 6; plVar12 = plVar12 + 6) {
      *(int *)plVar12 = 0xc4;
      resolve_steam_impl();
      uVar11 = (int)*plVar12 - 0xc4;
      if (8 < uVar11) {
        uVar11 = 4;
      }
      switch(uVar11) {
      case 0:
      case 2:
      case 3:
      case 4:
      case 5:
        goto code_r0x00014227ea65;
      case 1:
        if (SBORROW8(0,plVar12[1])) {
          lVar3 = plVar12[2];
          if (SBORROW8(0,lVar3)) goto code_r0x00014227ea65;
          goto joined_r0x00014227eb6c;
        }
        if (plVar12[1] == 0) goto code_r0x00014227ea65;
        goto code_r0x00014227ea60;
      case 6:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
        break;
      case 7:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
        break;
      default:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
      }
      lVar3 = plVar12[1];
joined_r0x00014227eb6c:
      if (lVar3 != 0) {
code_r0x00014227ea60:
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
code_r0x00014227ea65:
      plVar12[4] = lStack_a0;
      plVar12[5] = lStack_98;
      plVar12[2] = lStack_b0;
      plVar12[3] = lStack_a8;
      *plVar12 = lStack_c0;
      plVar12[1] = lStack_b8;
    }
    *(longlong *)(in_RCX + 6) = lVar2;
    *(undefined8 *)(in_RCX + 2) = uVar5;
    *(longlong **)(in_RCX + 4) = plVar10;
    *in_RCX = 0xcb;
    break;
  case 0xcc:
    uVar5 = *(undefined8 *)(in_RDX + 2);
    plVar10 = *(longlong **)(in_RDX + 4);
    lVar2 = *(longlong *)(in_RDX + 6);
    for (plVar12 = plVar10; plVar12 != plVar10 + lVar2 * 6; plVar12 = plVar12 + 6) {
      *(int *)plVar12 = 0xc4;
      resolve_steam_impl();
      uVar11 = (int)*plVar12 - 0xc4;
      if (8 < uVar11) {
        uVar11 = 4;
      }
      switch(uVar11) {
      case 0:
      case 2:
      case 3:
      case 4:
      case 5:
        goto code_r0x00014227e8ca;
      case 1:
        lVar3 = plVar12[1];
        if (!SBORROW8(0,lVar3)) goto joined_r0x00014227e9db;
        if ((SBORROW8(0,plVar12[2])) || (plVar12[2] == 0)) goto code_r0x00014227e8ca;
        goto code_r0x00014227e8c5;
      case 6:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
        break;
      case 7:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
        break;
      default:
        core::ptr::
        drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>()
        ;
      }
      lVar3 = plVar12[1];
joined_r0x00014227e9db:
      if (lVar3 != 0) {
code_r0x00014227e8c5:
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
code_r0x00014227e8ca:
      plVar12[4] = lStack_a0;
      plVar12[5] = lStack_98;
      plVar12[2] = lStack_b0;
      plVar12[3] = lStack_a8;
      *plVar12 = lStack_c0;
      plVar12[1] = lStack_b8;
    }
    *(longlong *)(in_RCX + 6) = lVar2;
    *(undefined8 *)(in_RCX + 2) = uVar5;
    *(longlong **)(in_RCX + 4) = plVar10;
    *in_RCX = 0xcc;
  }
  if ((byte)((char)iVar1 + 0x36U) < 3) {
    return;
  }
  switch(uVar4) {
  case 0:
  case 2:
  case 3:
  case 4:
  case 5:
    goto code_r0x00014227ed6c;
  case 1:
    lVar2 = *(longlong *)(in_RDX + 2);
    if ((SBORROW8(0,lVar2)) && (lVar2 = *(longlong *)(in_RDX + 4), SBORROW8(0,lVar2))) {
      return;
    }
    if (lVar2 == 0) {
      return;
    }
    goto code_r0x00014227ed67;
  case 6:
    core::ptr::
    drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>();
    break;
  case 7:
    core::ptr::
    drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>();
    break;
  default:
    core::ptr::
    drop_in_place<slice2_<enum2_<system_onboarding::hint_orchestrator::BindingHintDisplay>_>_>();
  }
  if (*(longlong *)(in_RDX + 2) != 0) {
code_r0x00014227ed67:
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
code_r0x00014227ed6c:
  return;
}

