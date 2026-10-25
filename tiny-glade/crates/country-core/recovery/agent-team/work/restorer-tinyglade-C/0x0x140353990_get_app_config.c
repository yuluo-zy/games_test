// get_app_config @ 0x0x140353990
// sig: void __cdecl get_app_config(void)
// size: 1537


void __cdecl tiny_glade::app_config::get_app_config(void)

{
  undefined *puVar1;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined8 uVar2;
  longlong *local_300;
  code *local_2f8;
  longlong *local_2f0;
  code *local_2e8;
  undefined *local_1e0;
  undefined8 *local_1d8;
  longlong **local_1d0;
  undefined8 local_1c8;
  undefined8 local_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined8 local_1a0;
  undefined4 local_198;
  undefined4 local_194;
  undefined **local_190;
  undefined8 local_188;
  longlong **local_180;
  undefined8 local_178;
  undefined8 local_170;
  undefined2 local_fc;
  longlong local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  longlong local_98 [4];
  longlong local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  longlong local_50;
  undefined8 *local_48;
  undefined *local_40;
  undefined1 local_31;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  paths::config();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::sys::pal::windows::fs::stat();
  if ((int)local_1e0 == 2) {
    if (((uint)local_1d8 & 3) == 1) {
      local_50 = (longlong)local_1d8 + -1;
      local_40 = *(undefined **)((longlong)local_1d8 + -1);
      local_48 = *(undefined8 **)((longlong)local_1d8 + 7);
      if ((code *)*local_48 != (code *)0x0) {
        (*(code *)*local_48)(local_40);
      }
      if (local_48[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    paths::installed();
    std::path::Path::_join();
    local_31 = 1;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    local_31 = 1;
    uVar2 = extraout_RAX;
    std::sys::pal::windows::fs::stat();
    if ((int)local_1e0 == 2) {
      if (((uint)local_1d8 & 3) == 1) {
        local_50 = (longlong)local_1d8 + -1;
        local_40 = *(undefined **)((longlong)local_1d8 + -1);
        local_48 = *(undefined8 **)((longlong)local_1d8 + 7);
        if ((code *)*local_48 != (code *)0x0) {
          (*(code *)*local_48)(local_40);
        }
        if (local_48[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_2f8 = std::path::impl$44::fmt;
      local_1e0 = &DAT_1429c38b0;
      local_1d8 = (undefined8 *)0x2;
      local_1c0 = 0;
      local_1d0 = &local_300;
      local_1c8 = 1;
      local_31 = 1;
      local_300 = &local_b8;
      anyhow::__private::format_err();
      in_RCX[1] = extraout_RAX_00;
      *in_RCX = 0x8000000000000000;
    }
    else {
      local_78 = local_b8;
      uStack_70 = uStack_b0;
      local_68 = local_a8;
      uStack_60 = uStack_a0;
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      std::sys::os_str::wtf8::Buf::as_mut_slice();
      std::sys::pal::windows::fs::copy();
      anyhow::context::impl$0::
      with_context<u64,std::io::error::Error,alloc::string::String,tiny_glade::app_config::get_app_config::closure_env_0>
                ();
      if (extraout_RAX_01 == 0) {
        if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
          local_2f8 = std::path::impl$44::fmt;
          local_2e8 = std::path::impl$44::fmt;
          local_300 = &local_78;
          local_2f0 = local_98;
          toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
          local_1c0 = *extraout_RAX_02;
          uStack_1b8 = extraout_RAX_02[1];
          local_194 = *(undefined4 *)(extraout_RAX_02 + 2);
          local_1b0 = 3;
          local_1a8 = 0x1429c3950;
          local_1a0 = 0x16;
          local_190 = &PTR_s_Initialized_settings__>_1429c38f0;
          local_188 = 2;
          local_180 = &local_300;
          local_178 = 2;
          local_170 = 0;
          local_1e0 = (undefined *)0x0;
          local_1d8 = (undefined8 *)((longlong)&DAT_1429c394f + 1);
          local_1d0 = (longlong **)0x16;
          local_1c8 = 0;
          local_198 = 1;
          ::log::__private_api::impl$2::log();
        }
        if (local_78 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        goto LAB_140353d96;
      }
      in_RCX[1] = uVar2;
      *in_RCX = 0x8000000000000000;
      local_b8 = local_78;
    }
    if (local_b8 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    goto LAB_140353f62;
  }
LAB_140353d96:
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  if (SBORROW8(0,(longlong)local_1e0)) {
    if (((uint)local_1d8 & 3) == 1) {
      local_50 = (longlong)local_1d8 + -1;
      local_40 = *(undefined **)((longlong)local_1d8 + -1);
      local_48 = *(undefined8 **)((longlong)local_1d8 + 7);
      if ((code *)*local_48 != (code *)0x0) {
        (*(code *)*local_48)(local_40);
      }
      if (local_48[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
LAB_140353ed2:
    country_core::game_config::impl$13::default();
    if (*(char *)(in_RDX + 0x148) != '\0') goto LAB_140353f38;
LAB_140353f2c:
    if (!SBORROW8(0,*(longlong *)(in_RDX + 0xa8))) goto LAB_140353f38;
  }
  else {
    local_40 = local_1e0;
    local_2f0 = (longlong *)0x0;
    local_300 = (longlong *)0x1;
    local_2f8 = (code *)0x80;
    local_48 = local_1d8;
    ron::options::Options::
    from_str_seed<core::marker::PhantomData<country_core::game_config::GameConfig>,country_core::game_config::GameConfig>
              ();
    puVar1 = local_1e0;
    if (SBORROW8(0,(longlong)local_1e0)) {
      core::ptr::drop_in_place<ron::error::SpannedError>();
    }
    else {
      memcpy(&local_300,&local_1d8,0x120);
    }
    if (local_40 != (undefined *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (SBORROW8(0,(longlong)puVar1)) goto LAB_140353ed2;
    local_1e0 = puVar1;
    memcpy(&local_1d8,&local_300,0x120);
    if (*(char *)(in_RDX + 0x148) == '\0') goto LAB_140353f2c;
LAB_140353f38:
    local_fc = 0x101;
  }
  country_core::game_config::GameConfig::upgrade();
  memcpy(in_RCX,&local_1e0,0x128);
LAB_140353f62:
  if (local_98[0] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

