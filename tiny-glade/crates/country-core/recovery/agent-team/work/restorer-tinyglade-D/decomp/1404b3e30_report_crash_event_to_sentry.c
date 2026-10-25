// 1404b3e30 void __cdecl report_crash_event_to_sentry(void)

void __cdecl tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry(void)

{
  longlong *plVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  ulonglong uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *plVar7;
  undefined2 *extraout_RAX;
  undefined8 *in_RCX;
  void *in_RDX;
  undefined8 *puVar8;
  longlong lVar9;
  undefined8 *puVar10;
  longlong *in_R8;
  longlong *in_R9;
  undefined8 *puVar11;
  longlong *plVar12;
  bool bVar13;
  undefined8 local_868;
  undefined8 *puStack_860;
  longlong lStack_858;
  longlong local_850;
  longlong lStack_848;
  longlong local_840;
  longlong lStack_838;
  undefined8 *local_830;
  undefined8 uStack_828;
  longlong local_820;
  longlong lStack_818;
  longlong local_810;
  longlong lStack_808;
  undefined *local_800;
  undefined8 local_7f8;
  longlong local_7f0;
  longlong local_7e8;
  undefined2 *local_7e0;
  longlong local_7d8;
  undefined *local_7d0;
  undefined8 local_7c8;
  longlong local_7c0;
  longlong lStack_7b8;
  longlong local_7b0;
  longlong lStack_7a8;
  undefined *local_7a0;
  undefined8 local_798;
  longlong local_790;
  longlong lStack_788;
  longlong local_780;
  longlong lStack_778;
  undefined *local_770;
  undefined8 local_768;
  longlong local_760;
  longlong lStack_758;
  longlong local_750;
  longlong lStack_748;
  undefined *local_740;
  undefined8 local_738;
  longlong local_730;
  longlong lStack_728;
  longlong local_720;
  longlong lStack_718;
  undefined *local_710;
  undefined8 local_708;
  longlong local_700;
  longlong lStack_6f8;
  longlong local_6f0;
  longlong lStack_6e8;
  undefined *local_6e0;
  undefined8 local_6d8;
  longlong local_6d0;
  longlong lStack_6c8;
  longlong local_6c0;
  longlong lStack_6b8;
  undefined8 local_6b0;
  undefined8 local_6a8;
  undefined8 *local_3a0;
  undefined8 uStack_398;
  longlong local_390;
  undefined1 *local_388;
  undefined1 *local_380;
  undefined8 *local_378;
  undefined8 *puStack_370;
  longlong **local_368;
  undefined8 *puStack_360;
  undefined8 *puStack_358;
  longlong lStack_350;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined *local_188;
  undefined8 uStack_180;
  longlong local_178;
  longlong lStack_170;
  longlong local_168;
  longlong lStack_160;
  undefined8 *local_158;
  undefined8 uStack_150;
  longlong local_148;
  longlong lStack_140;
  longlong local_138;
  longlong lStack_130;
  longlong *local_128;
  code *local_120;
  undefined *local_118;
  undefined8 uStack_110;
  longlong local_108;
  longlong lStack_100;
  longlong local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  longlong local_d8;
  undefined1 local_c9 [17];
  void *local_b8;
  undefined8 *local_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong *local_88;
  longlong local_80;
  undefined8 *local_78;
  longlong *local_70;
  undefined8 *local_68;
  undefined1 local_59;
  undefined8 *local_58;
  undefined1 local_4e;
  undefined1 local_4c;
  undefined1 local_4b;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_368 = (longlong **)*in_R8;
  puVar2 = (undefined8 *)in_R8[1];
  puVar10 = puVar2 + in_R8[2] * 0xe;
  puVar8 = puVar2;
  puVar3 = puVar2;
  local_378 = puVar2;
  puStack_370 = puVar2;
  puStack_360 = puVar10;
  local_b8 = in_RDX;
  local_88 = in_R9;
  local_80 = (longlong)local_368;
  local_70 = in_R8;
  if (in_R8[2] != 0) {
    do {
      local_78 = puVar3 + 0xe;
      puVar8 = local_78;
      if (SBORROW8(0,(longlong)*puVar3)) break;
      local_800 = (undefined *)puVar3[0xd];
      local_810 = puVar3[0xb];
      lStack_808 = puVar3[0xc];
      local_820 = puVar3[9];
      lStack_818 = puVar3[10];
      puStack_860 = (undefined8 *)puVar3[1];
      lStack_858 = puVar3[2];
      local_850 = puVar3[3];
      lStack_848 = puVar3[4];
      local_840 = puVar3[5];
      lStack_838 = puVar3[6];
      local_830 = (undefined8 *)puVar3[7];
      uStack_828 = puVar3[8];
      local_868 = (undefined **)*puVar3;
      std::thread::local::LocalKey::
      with<tuple_<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global>_>,core::cell::Cell<bool>_>,sentry_core::hub_impl::impl_3::with::closure_env_0<sentry_core::hub::impl_0::with_active::closure_env_0<sentry_core::api::add_breadcrumb::closure_env_0<sentry_types::protocol::v7::Breadcrumb>,tuple_<>_>,tuple_<>_>,tuple_<>_>
                ();
      puVar11 = puVar3 + 0xe;
      puVar8 = puVar10;
      puVar3 = local_78;
    } while (puVar11 != puVar10);
  }
  local_68 = (undefined8 *)
             (((ulonglong)((longlong)puVar10 - (longlong)puVar8) >> 4) * 0x6db6db6db6db6db7);
  local_58 = (undefined8 *)0x0;
  puStack_370 = puVar8;
  local_b0 = puVar2;
  local_78 = puVar8;
  while (local_68 != local_58) {
    local_58 = (undefined8 *)((longlong)local_58 + 1);
    core::ptr::drop_in_place<sentry_types::protocol::v7::Breadcrumb>();
  }
  if (local_80 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar7 = local_70;
  plVar1 = local_70 + 0x2d;
  if (SBORROW8(0,local_70[0x2d])) {
    local_c9[0] = 0;
    plVar12 = local_70;
  }
  else {
    local_49 = 1;
    local_4a = 1;
    local_4b = 1;
    local_4c = 1;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    local_49 = 1;
    local_4a = 1;
    local_4b = 1;
    local_4c = 1;
    std::fs::read_to_string::inner();
    if (SBORROW8(0,(longlong)local_868)) {
      if (((uint)puStack_860 & 3) == 1) {
        local_78 = (undefined8 *)((longlong)puStack_860 + -1);
        local_58 = *(undefined8 **)((longlong)puStack_860 + -1);
        local_68 = *(undefined8 **)((longlong)puStack_860 + 7);
        if ((code *)*local_68 != (code *)0x0) {
          (*(code *)*local_68)(local_58);
        }
        if (local_68[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_c9[0] = 0;
      plVar12 = local_70;
    }
    else {
      local_58 = local_868;
      local_108 = 0;
      local_118 = (undefined *)0x1;
      uStack_110 = 0x80;
      local_68 = puStack_860;
      ron::options::Options::
      from_str_seed<core::marker::PhantomData<country_core::game_profile::GameProfile>,country_core::game_profile::GameProfile>
                ();
      bVar13 = (int)local_868 != 0xc4;
      if (bVar13) {
        memcpy(&local_378,(void *)((longlong)&local_868 + 4),0x1d4);
        plVar12 = local_70;
      }
      else {
        core::ptr::drop_in_place<ron::error::SpannedError>();
        plVar12 = local_70;
      }
      local_c9[0] = bVar13;
      local_70 = plVar12;
      if (local_58 != (undefined8 *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  local_108 = plVar12[0x29];
  lStack_100 = plVar12[0x2a];
  local_f8 = plVar12[0x2b];
  lStack_f0 = plVar12[0x2c];
  local_118 = &DAT_1429e8f00;
  uStack_110 = 3;
  lVar9 = plVar12[0xb];
  lStack_858 = plVar12[0xe];
  local_868 = (undefined **)plVar12[0xc];
  puStack_860 = (undefined8 *)plVar12[0xd];
  if (SBORROW8(0,lVar9)) {
    local_d8 = plVar12[0xe];
    local_e8 = plVar12[0xc];
    lStack_e0 = plVar12[0xd];
    local_128 = &local_e8;
    local_120 = crash_pickup::crash_payload::impl$5::fmt;
    local_868 = (undefined **)0x2;
    lStack_858 = 2;
    lStack_848 = 0;
    local_840 = 0x400000020;
    lStack_838 = CONCAT71(lStack_838._1_7_,3);
    local_378 = (undefined8 *)&DAT_1429e8f48;
    puStack_370 = (undefined8 *)0x1;
    local_368 = &local_128;
    puStack_360 = (undefined8 *)0x1;
    puStack_358 = &local_868;
    lStack_350 = 1;
    alloc::fmt::format::format_inner();
    local_868 = (undefined **)local_a8;
    puStack_860 = (undefined8 *)lStack_a0;
    lStack_858 = local_98;
    plVar12 = local_70;
    if ((!SBORROW8(0,local_e8)) && (local_e8 != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      plVar12 = local_70;
    }
  }
  else if (!SBORROW8(0,plVar12[0xf])) {
    uVar4 = plVar12[0x12];
    lVar6 = plVar12[0x1d];
    lVar5 = plVar12[0x21];
    local_68 = (undefined8 *)plVar12[0x22];
    local_58 = in_RCX;
    if (plVar12[0xf] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((uVar4 & 0x7fffffffffffffff) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (lVar6 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    in_RCX = local_58;
    plVar12 = local_70;
    if (lVar5 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  local_378 = (undefined8 *)&DAT_1429e8f03;
  puStack_370 = (undefined8 *)0x4;
  puStack_360 = local_868;
  puStack_358 = puStack_860;
  lStack_350 = lStack_858;
  lVar6 = *local_88;
  local_368 = (longlong **)lVar9;
  local_a8 = (longlong)local_868;
  lStack_a0 = (longlong)puStack_860;
  local_98 = lStack_858;
  if (SBORROW8(0,lVar6)) {
    lVar9 = 3;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined2 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined1 *)(extraout_RAX + 1) = 0x41;
    *extraout_RAX = 0x2f4e;
    local_7d8 = 3;
    local_7e0 = extraout_RAX;
  }
  else {
    lVar9 = local_88[1];
    local_7e0 = (undefined2 *)local_88[2];
    local_7d8 = local_88[3];
  }
  puStack_860 = (undefined8 *)local_118;
  local_7c0 = plVar12[0x35];
  lStack_7b8 = plVar12[0x36];
  local_7b0 = plVar12[0x37];
  lStack_7a8 = plVar12[0x38];
  local_790 = plVar12[0x39];
  lStack_788 = plVar12[0x3a];
  local_780 = plVar12[0x3b];
  lStack_778 = plVar12[0x3c];
  local_760 = plVar12[0x25];
  lStack_758 = plVar12[0x26];
  local_750 = plVar12[0x27];
  lStack_748 = plVar12[0x28];
  local_730 = plVar12[0x31];
  lStack_728 = plVar12[0x32];
  local_720 = plVar12[0x33];
  lStack_718 = plVar12[0x34];
  local_700 = *plVar1;
  lStack_6f8 = plVar7[0x2e];
  local_6f0 = plVar7[0x2f];
  lStack_6e8 = plVar7[0x30];
  local_6d0 = plVar12[7];
  lStack_6c8 = plVar12[8];
  local_6c0 = plVar12[9];
  lStack_6b8 = plVar12[10];
  local_188 = local_118;
  uStack_180 = uStack_110;
  local_178 = local_108;
  lStack_170 = lStack_100;
  local_168 = local_f8;
  lStack_160 = lStack_f0;
  local_158 = local_378;
  uStack_150 = puStack_370;
  local_148 = (longlong)local_368;
  lStack_140 = (longlong)puStack_360;
  local_138 = (longlong)puStack_358;
  lStack_130 = lStack_350;
  local_118 = (undefined *)0x0;
  local_868 = &local_118;
  local_810 = (longlong)puStack_358;
  lStack_808 = lStack_350;
  local_820 = (longlong)local_368;
  lStack_818 = (longlong)puStack_360;
  local_830 = local_378;
  uStack_828 = puStack_370;
  local_840 = local_f8;
  lStack_838 = lStack_f0;
  local_850 = local_108;
  lStack_848 = lStack_100;
  lStack_858 = uStack_110;
  local_800 = &DAT_1429e8f07;
  local_7f8 = 8;
  local_7d0 = &DAT_1429e8f0f;
  local_7c8 = 0xe;
  local_7a0 = &DAT_1429e8f1d;
  local_798 = 0xc;
  local_770 = &DAT_1429e8f29;
  local_768 = 6;
  local_740 = &DAT_1429e8f2f;
  local_738 = 8;
  local_710 = &DAT_1429e8f37;
  local_708 = 7;
  local_6e0 = &DAT_1429e8f3e;
  local_6d8 = 10;
  local_6b0 = 0;
  local_6a8 = 9;
  local_49 = 0;
  local_4a = 0;
  local_4b = 0;
  local_4c = 1;
  local_7f0 = lVar6;
  local_7e8 = lVar9;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<tuple_<ref_<str_>,enum2_<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error>_>_>,core::iter::adapters::map::Map<core::array::iter::IntoIter<tuple_<ref_<str_>,enum2_<core::result::Result<std::path::PathBuf,alloc::string::String>_>_>,9>,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env_4>_>
            ();
  local_4e = 1;
  alloc::vec::Vec::
  retain<tuple_<ref_<str_>,enum2_<core::result::Result<sentry_types::protocol::attachment::Attachment,anyhow::Error>_>_>,alloc::alloc::Global,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env_5>
            ();
  local_390 = (longlong)local_368;
  local_3a0 = local_378;
  uStack_398 = puStack_370;
  memcpy(&local_868,local_b8,0x4c8);
  local_388 = &local_59;
  local_380 = local_c9;
  local_4e = 0;
  std::thread::local::LocalKey::
  with<tuple_<core::cell::UnsafeCell<alloc::sync::Arc<sentry_core::hub::Hub,alloc::alloc::Global>_>,core::cell::Cell<bool>_>,sentry_core::hub_impl::impl_3::with::closure_env_0<sentry_core::api::with_scope::closure_env_0<tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env_6,tiny_glade::panic_reporter::sentry_reporter::report_crash_event_to_sentry::closure_env_7,uuid::Uuid>,uuid::Uuid>,uuid::Uuid>
            ();
  *(undefined8 *)((longlong)in_RCX + 1) = local_198;
  *(undefined8 *)((longlong)in_RCX + 9) = uStack_190;
  *(undefined1 *)in_RCX = 0;
  if ((local_70[3] != 0) && ((!SBORROW8(0,local_70[3]) || (local_70[4] != 0)))) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

