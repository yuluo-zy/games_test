// addr=0x140916480 name=load_game_profile
// signature=void __cdecl load_game_profile(void)


void __cdecl country_core::game_profile::load_game_profile(void)

{
  longlong lVar1;
  longlong lVar2;
  char extraout_AL;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong in_RCX;
  undefined8 unaff_RBX;
  ulonglong uVar3;
  longlong *plVar4;
  longlong local_508;
  undefined8 **ppuStack_500;
  code *local_4f8;
  undefined8 uStack_4f0;
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  char *local_4d0;
  undefined8 uStack_4c8;
  undefined4 local_4c0;
  undefined4 uStack_4bc;
  undefined **ppuStack_4b8;
  undefined8 local_4b0;
  undefined8 ***local_4a8;
  undefined8 local_4a0;
  undefined8 uStack_498;
  undefined8 **local_2f8;
  code *pcStack_2f0;
  undefined8 local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  char *local_2c8;
  undefined8 uStack_2c0;
  undefined **ppuStack_2b0;
  undefined8 local_2a8;
  ulonglong **local_298;
  code *local_290;
  longlong local_288;
  undefined8 **ppuStack_280;
  code *local_278;
  undefined8 uStack_270;
  ulonglong **local_268;
  undefined8 local_260;
  undefined8 local_258;
  undefined8 local_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 local_230;
  undefined8 local_228;
  undefined4 local_220;
  undefined4 local_21c;
  undefined8 local_218;
  undefined8 local_210;
  undefined8 ***local_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined4 local_1f0;
  undefined4 local_1e4;
  undefined4 local_1d8;
  undefined4 local_1cc;
  undefined4 local_1c0;
  undefined4 local_1b4;
  undefined4 local_1a8;
  undefined4 local_19c;
  undefined4 local_190;
  undefined4 local_184;
  undefined4 local_178;
  undefined4 local_16c;
  undefined4 local_160;
  undefined4 local_154;
  undefined4 local_148;
  undefined4 local_13c;
  undefined4 local_130;
  undefined4 local_124;
  undefined4 local_118;
  undefined4 local_10c;
  undefined4 local_100;
  undefined4 local_f4;
  undefined4 local_e8;
  undefined4 local_dc;
  undefined4 local_d0;
  undefined4 local_c4;
  undefined4 local_b8;
  undefined2 local_ac;
  undefined2 local_a8;
  undefined2 local_a4;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined7 uStack_70;
  undefined1 uStack_69;
  undefined7 uStack_68;
  undefined8 *local_60;
  undefined8 **local_58;
  longlong local_48;
  undefined8 **local_40;
  ulonglong local_38;
  char local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  paths::config();
  std::path::Path::_join();
  local_29 = 1;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  local_29 = 1;
  std::path::Path::is_file();
  if (extraout_AL == '\0') {
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_29 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_4e8 = *extraout_RAX_00;
      uStack_4e0 = extraout_RAX_00[1];
      uStack_4bc = *(undefined4 *)(extraout_RAX_00 + 2);
      local_4d8 = 3;
      local_4d0 = 
      "country_core::game_profileFailed deserializing profile.ron. Falling back to defaults. The error was: "
      ;
      uStack_4c8 = 0x1a;
      ppuStack_4b8 = (undefined **)&DAT_142aaca58;
      local_4b0 = 1;
      local_4a8 = (undefined8 ***)&DAT_00000008;
      local_508 = 0;
      local_4a0 = 0;
      uStack_498 = 0;
      ppuStack_500 = (undefined8 **)0x142aacaa8;
      local_4f8 = (code *)0x1a;
      uStack_4f0 = 0;
      local_4c0 = 1;
      local_29 = 1;
      ::log::__private_api::impl$2::log();
    }
    local_78 = 0;
    uStack_70 = 0;
    local_88 = 0;
    uStack_80 = 0;
    uStack_69 = 0;
    uStack_68 = 0;
    local_29 = 1;
    uuid::Uuid::new_v4();
    local_268 = (ulonglong **)CONCAT44(local_268._4_4_,0x2a);
    local_260 = (code *)CONCAT44(0x26,(undefined4)local_260);
    local_250 = CONCAT44(local_250._4_4_,0x14);
    local_248 = CONCAT44(0x17,(undefined4)local_248);
    local_238 = CONCAT44(local_238._4_4_,0x18);
    local_230 = (char *)CONCAT44(0x24,(undefined4)local_230);
    local_220 = 0xc3;
    local_218 = (undefined *)CONCAT44(0xc3,(undefined4)local_218);
    local_208 = (undefined8 ***)CONCAT44(local_208._4_4_,0xc3);
    local_200 = CONCAT44(0xc3,(undefined4)local_200);
    local_1f0 = 7;
    local_1e4 = 8;
    local_1d8 = 9;
    local_1cc = 10;
    local_1c0 = 0xb;
    local_1b4 = 0xc;
    local_1a8 = 0xd;
    local_19c = 0xe;
    local_190 = 0xf;
    local_184 = 0x23;
    local_178 = 0x27;
    local_16c = 0x2d;
    local_160 = 0x2b;
    local_154 = 0xc3;
    local_148 = 0x38;
    local_13c = 0xab;
    local_130 = 0xa5;
    local_124 = 0x1b;
    local_118 = 0x49;
    local_10c = 0x3a;
    local_100 = 0x51;
    local_f4 = 0x52;
    local_e8 = 0x50;
    local_dc = 0x53;
    local_d0 = 0x4f;
    local_c4 = 0x4e;
    local_ac = 1;
    local_a8 = 2;
    local_a4 = 1;
    local_b8 = 0xc3;
    local_90 = 0;
    uVar3 = CONCAT71((int7)((ulonglong)unaff_RBX >> 8),1);
    goto LAB_140916e76;
  }
  local_4f8 = local_278;
  uStack_4f0 = uStack_270;
  local_508 = local_288;
  ppuStack_500 = ppuStack_280;
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  if (local_508 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (SBORROW8(0,(longlong)local_60)) {
    local_2f8 = local_58;
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_268 = (ulonglong **)&local_2f8;
      local_260 = std::io::error::impl$0::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_4e8 = *extraout_RAX;
      uStack_4e0 = extraout_RAX[1];
      uStack_4bc = *(undefined4 *)(extraout_RAX + 2);
      local_4d8 = 1;
      local_4d0 = 
      "country_core::game_profileFailed deserializing profile.ron. Falling back to defaults. The error was: "
      ;
      uStack_4c8 = 0x1a;
      ppuStack_4b8 = &PTR_s_Failed_reading_profile_ron__Fall_142aacb80;
      local_4b0 = 1;
      local_4a8 = &local_268;
      local_4a0 = 1;
      uStack_498 = 0;
      local_508 = 0;
      ppuStack_500 = (undefined8 **)0x142aacaa8;
      local_4f8 = (code *)0x1a;
      uStack_4f0 = 0;
      local_4c0 = 1;
      ::log::__private_api::impl$2::log();
    }
    local_78 = 0;
    uStack_70 = 0;
    local_88 = 0;
    uStack_80 = 0;
    uStack_69 = 0;
    uStack_68 = 0;
    uuid::Uuid::new_v4();
    local_268 = (ulonglong **)CONCAT44(local_268._4_4_,0x2a);
    local_260 = (code *)CONCAT44(0x26,(undefined4)local_260);
    local_250 = CONCAT44(local_250._4_4_,0x14);
    local_248 = CONCAT44(0x17,(undefined4)local_248);
    local_238 = CONCAT44(local_238._4_4_,0x18);
    local_230 = (char *)CONCAT44(0x24,(undefined4)local_230);
    local_220 = 0xc3;
    local_218 = (undefined *)CONCAT44(0xc3,(undefined4)local_218);
    local_208 = (undefined8 ***)CONCAT44(local_208._4_4_,0xc3);
    local_200 = CONCAT44(0xc3,(undefined4)local_200);
    local_1f0 = 7;
    local_1e4 = 8;
    local_1d8 = 9;
    local_1cc = 10;
    local_1c0 = 0xb;
    local_1b4 = 0xc;
    local_1a8 = 0xd;
    local_19c = 0xe;
    local_190 = 0xf;
    local_184 = 0x23;
    local_178 = 0x27;
    local_16c = 0x2d;
    local_160 = 0x2b;
    local_154 = 0xc3;
    local_148 = 0x38;
    local_13c = 0xab;
    local_130 = 0xa5;
    local_124 = 0x1b;
    local_118 = 0x49;
    local_10c = 0x3a;
    local_100 = 0x51;
    local_f4 = 0x52;
    local_e8 = 0x50;
    local_dc = 0x53;
    local_d0 = 0x4f;
    local_c4 = 0x4e;
    local_ac = 1;
    local_a8 = 2;
    local_a4 = 1;
    local_b8 = 0xc3;
    local_90 = 0;
    if (((uint)local_2f8 & 3) == 1) {
      local_48 = (longlong)local_2f8 + -1;
      local_38 = *(ulonglong *)((longlong)local_2f8 + -1);
      local_40 = *(undefined8 ***)((longlong)local_2f8 + 7);
      if ((code *)*local_40 != (code *)0x0) {
        (*(code *)*local_40)(local_38);
      }
      if (local_40[1] != (ulonglong *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
LAB_140916e6f:
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    local_38 = (ulonglong)local_60;
    local_258 = 0;
    local_268 = (ulonglong **)0x1;
    local_260 = (code *)0x80;
    local_40 = local_58;
    ron::options::Options::
    from_str_seed<core::marker::PhantomData<country_core::game_profile::GameProfile>,country_core::game_profile::GameProfile>
              ();
    if ((int)local_508 == 0xc4) {
      local_2a8 = local_4b0;
      ppuStack_2b0 = ppuStack_4b8;
      local_2c8 = local_4d0;
      uStack_2c0 = uStack_4c8;
      local_2d8 = uStack_4e0;
      uStack_2d0 = local_4d8;
      local_2e8 = uStack_4f0;
      uStack_2e0 = local_4e8;
      local_2f8 = ppuStack_500;
      pcStack_2f0 = local_4f8;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_298 = (ulonglong **)&local_2f8;
        local_290 = ron::error::impl$15::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_248 = *extraout_RAX_01;
        uStack_240 = extraout_RAX_01[1];
        local_21c = *(undefined4 *)(extraout_RAX_01 + 2);
        local_238 = 1;
        local_230 = 
        "country_core::game_profileFailed deserializing profile.ron. Falling back to defaults. The error was: "
        ;
        local_228 = 0x1a;
        local_218 = &DAT_142aacb10;
        local_210 = 1;
        local_208 = &local_298;
        local_200 = 1;
        local_1f8 = 0;
        local_268 = (ulonglong **)0x0;
        local_260 = (code *)0x142aacaa8;
        local_258 = 0x1a;
        local_250 = 0;
        local_220 = 1;
        ::log::__private_api::impl$2::log();
      }
      local_78 = 0;
      uStack_70 = 0;
      local_88 = 0;
      uStack_80 = 0;
      uStack_69 = 0;
      uStack_68 = 0;
      uuid::Uuid::new_v4();
      local_268 = (ulonglong **)CONCAT44(local_268._4_4_,0x2a);
      local_260 = (code *)CONCAT44(0x26,(undefined4)local_260);
      local_250 = CONCAT44(local_250._4_4_,0x14);
      local_248 = CONCAT44(0x17,(undefined4)local_248);
      local_238 = CONCAT44(local_238._4_4_,0x18);
      local_230 = (char *)CONCAT44(0x24,(undefined4)local_230);
      local_220 = 0xc3;
      local_218 = (undefined *)CONCAT44(0xc3,(undefined4)local_218);
      local_208 = (undefined8 ***)CONCAT44(local_208._4_4_,0xc3);
      local_200 = CONCAT44(0xc3,(undefined4)local_200);
      local_1f0 = 7;
      local_1e4 = 8;
      local_1d8 = 9;
      local_1cc = 10;
      local_1c0 = 0xb;
      local_1b4 = 0xc;
      local_1a8 = 0xd;
      local_19c = 0xe;
      local_190 = 0xf;
      local_184 = 0x23;
      local_178 = 0x27;
      local_16c = 0x2d;
      local_160 = 0x2b;
      local_154 = 0xc3;
      local_148 = 0x38;
      local_13c = 0xab;
      local_130 = 0xa5;
      local_124 = 0x1b;
      local_118 = 0x49;
      local_10c = 0x3a;
      local_100 = 0x51;
      local_f4 = 0x52;
      local_e8 = 0x50;
      local_dc = 0x53;
      local_d0 = 0x4f;
      local_c4 = 0x4e;
      local_ac = 1;
      local_a8 = 2;
      local_a4 = 1;
      local_b8 = 0xc3;
      local_90 = 0;
      core::ptr::drop_in_place<ron::error::SpannedError>();
    }
    else {
      memcpy(&local_268,&local_508,0x208);
    }
    if ((undefined8 *)local_38 != (undefined8 *)0x0) goto LAB_140916e6f;
  }
  uVar3 = 0;
LAB_140916e76:
  local_29 = (char)uVar3;
  write_profile_via_tmp();
  if (extraout_RAX_02 != 0) {
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      local_38 = CONCAT44(local_38._4_4_,(int)uVar3);
      local_2f8 = &local_60;
      pcStack_2f0 = anyhow::error::impl$5::fmt;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_4e8 = *extraout_RAX_03;
      uStack_4e0 = extraout_RAX_03[1];
      uStack_4bc = *(undefined4 *)(extraout_RAX_03 + 2);
      local_4d8 = 1;
      local_4d0 = 
      "country_core::game_profileFailed deserializing profile.ron. Falling back to defaults. The error was: "
      ;
      uStack_4c8 = 0x1a;
      ppuStack_4b8 = &anon_0de6f48e523060bc80b429b03e3c24c8_157_llvm_11933763432560700012;
      local_4b0 = 1;
      local_4a8 = &local_2f8;
      local_4a0 = 1;
      uStack_498 = 0;
      local_508 = 0;
      ppuStack_500 = (undefined8 **)0x142aacaa8;
      local_4f8 = (code *)0x1a;
      uStack_4f0 = 0;
      local_4c0 = 1;
      ::log::__private_api::impl$2::log();
      uVar3 = local_38 & 0xffffffff;
    }
    local_29 = (char)uVar3;
    anyhow::error::impl$6::drop();
  }
  memcpy(&local_508,&local_268,0x208);
  plVar4 = *(longlong **)(in_RCX + 8);
  if (plVar4 == (longlong *)0x0) {
    plVar4 = *(longlong **)(in_RCX + 0x10);
  }
  lVar1 = plVar4[2];
  if ((ulonglong)(*plVar4 - lVar1) < 0x210) {
    local_29 = (char)uVar3;
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar2 = plVar4[1];
  *(code **)(lVar2 + lVar1) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<country_core::game_profile::GameProfile>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  memcpy((void *)(lVar2 + lVar1 + 8),&local_508,0x208);
  plVar4[2] = lVar1 + 0x210;
  if (((char)uVar3 != '\0') && (local_288 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


