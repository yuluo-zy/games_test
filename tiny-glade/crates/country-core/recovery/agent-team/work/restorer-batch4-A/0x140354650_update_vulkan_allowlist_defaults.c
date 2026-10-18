// update_vulkan_allowlist_defaults
// sig: void __cdecl update_vulkan_allowlist_defaults(void)
// size: 1414

void __cdecl tiny_glade::app_config::update_vulkan_allowlist_defaults(void)

{
  longlong lVar1;
  ulonglong uVar2;
  longlong lVar3;
  uint uVar4;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong in_RCX;
  longlong lVar5;
  longlong local_1e0 [4];
  longlong *local_1c0;
  undefined **ppuStack_1b8;
  undefined8 **local_1b0;
  code *pcStack_1a8;
  undefined8 local_1a0;
  undefined8 uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 **local_168;
  code *local_160;
  longlong *local_158;
  undefined **ppuStack_150;
  undefined8 **local_148;
  code *pcStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  longlong local_f8;
  longlong *local_f0;
  longlong local_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 **local_98;
  undefined8 local_90;
  undefined **local_88;
  undefined8 **local_80;
  undefined8 **local_78;
  longlong local_70;
  undefined8 uStack_68;
  longlong *local_58;
  longlong local_50;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  paths::installed();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  if (SBORROW8(0,local_f8)) {
    local_1c0 = local_f0;
    if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
      ppuStack_150 = (undefined **)std::path::impl$44::fmt;
      local_148 = &local_1c0;
      pcStack_140 = std::io::error::impl$0::fmt;
      local_158 = local_1e0;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_b8 = *extraout_RAX;
      uStack_b0 = extraout_RAX[1];
      local_a8 = 1;
      local_a0 = 0x1429c3950;
      local_98 = (longlong **)0x16;
      local_88 = &PTR_s_Error_reading_1429c3ae0;
      local_80 = (longlong **)0x2;
      local_78 = &local_158;
      local_70 = 2;
      uStack_68 = 0;
      local_d8 = 0;
      local_d0 = 0x1429c3950;
      local_c8 = 0x16;
      local_c0 = 0;
      local_90 = (longlong **)CONCAT44(*(undefined4 *)(extraout_RAX + 2),1);
      ::log::__private_api::impl$2::log();
    }
    if (((uint)local_1c0 & 3) != 1) goto LAB_140354b66;
    local_e0 = (longlong)local_1c0 + -1;
    local_50 = *(longlong *)((longlong)local_1c0 + -1);
    local_58 = *(longlong **)((longlong)local_1c0 + 7);
    if ((code *)*local_58 != (code *)0x0) {
      (*(code *)*local_58)(local_50);
    }
    if ((undefined8 *)local_58[1] != (undefined8 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    local_50 = local_f8;
    local_c8 = 0;
    local_d8 = 1;
    local_d0 = 0x80;
    local_58 = local_f0;
    ron::options::Options::
    from_str_seed<core::marker::PhantomData<alloc::vec::Vec<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>_>,alloc::vec::Vec<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>_>
              ();
    if ((char)local_1c0 == '2') {
      local_158 = (longlong *)((ulonglong)local_158 & 0xffffffff00000000);
      lVar1 = *(longlong *)(in_RCX + 0x28);
      local_80 = local_1b0 + (longlong)pcStack_1a8 * 4;
      local_70 = in_RCX + 0x30;
      local_d8 = 0;
      local_b8 = 0;
      local_98 = local_1b0;
      local_90 = local_1b0;
      local_88 = ppuStack_1b8;
      local_78 = &local_158;
      alloc::vec::spec_extend::impl$0::
      spec_extend<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,core::iter::adapters::flatten::FlatMap<core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<tiny_glade::app_config::VulkanLayerAllowlistDefaults,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env_0>,alloc::vec::into_iter::IntoIter<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>,tiny_glade::app_config::update_vulkan_allowlist_defaults::closure_env_1>,alloc::alloc::Global>
                ();
      uVar4 = *(uint *)(in_RCX + 0x30);
      if (*(uint *)(in_RCX + 0x30) < (uint)local_158) {
        uVar4 = (uint)local_158;
      }
      *(uint *)(in_RCX + 0x30) = uVar4;
      uVar2 = *(ulonglong *)(in_RCX + 0x28);
      if (1 < uVar2) {
        lVar3 = *(longlong *)(in_RCX + 0x20);
        if (uVar2 < 0x15) {
          lVar5 = lVar3 + 0x20;
          do {
            core::slice::sort::shared::smallsort::
            insert_tail<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool_(*)(ref_<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>_>,ref_<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>_>)>
                      ();
            lVar5 = lVar5 + 0x20;
          } while (lVar5 != uVar2 * 0x20 + lVar3);
        }
        else {
          core::slice::sort::stable::
          driftsort_main<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,bool_(*)(ref_<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>_>,ref_<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>_>),alloc::vec::Vec<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>_>
                    ();
        }
      }
      alloc::vec::Vec::
      dedup_by<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global,alloc::vec::impl_5::dedup::closure_env_0<enum2_<rhapsody::raw::init::GraphicsInstanceLayerAllowlistItem>,alloc::alloc::Global>_>
                ();
      if ((lVar1 != *(longlong *)(in_RCX + 0x28)) &&
         (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_b8 = *extraout_RAX_00;
        uStack_b0 = extraout_RAX_00[1];
        local_a8 = 3;
        local_a0 = 0x1429c3950;
        local_98 = (longlong **)0x16;
        local_88 = &PTR_s_Vulkan_allowlist_defaults_have_b_1429c3a50;
        local_80 = (longlong **)0x1;
        local_78 = (undefined8 **)&DAT_00000008;
        local_d8 = 0;
        local_70 = 0;
        uStack_68 = 0;
        local_d0 = 0x1429c3950;
        local_c8 = 0x16;
        local_c0 = 0;
        local_90 = (longlong **)CONCAT44(*(undefined4 *)(extraout_RAX_00 + 2),1);
        ::log::__private_api::impl$2::log();
      }
      write_game_config_to_disk();
    }
    else {
      local_108 = local_170;
      local_118 = local_180;
      uStack_110 = uStack_178;
      local_128 = local_190;
      uStack_120 = uStack_188;
      local_138 = local_1a0;
      uStack_130 = uStack_198;
      local_148 = local_1b0;
      pcStack_140 = pcStack_1a8;
      local_158 = local_1c0;
      ppuStack_150 = ppuStack_1b8;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_168 = &local_158;
        local_160 = ron::error::impl$15::fmt;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_b8 = *extraout_RAX_01;
        uStack_b0 = extraout_RAX_01[1];
        local_a8 = 1;
        local_a0 = 0x1429c3950;
        local_98 = (longlong **)0x16;
        local_88 = &PTR_s_Error_deserializing_Vulkan_allow_1429c3aa8;
        local_80 = (longlong **)0x1;
        local_78 = &local_168;
        local_70 = 1;
        uStack_68 = 0;
        local_d8 = 0;
        local_d0 = 0x1429c3950;
        local_c8 = 0x16;
        local_c0 = 0;
        local_90 = (longlong **)CONCAT44(*(undefined4 *)(extraout_RAX_01 + 2),1);
        ::log::__private_api::impl$2::log();
      }
      core::ptr::drop_in_place<ron::error::SpannedError>();
    }
    if (local_50 == 0) goto LAB_140354b66;
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
LAB_140354b66:
  if (local_1e0[0] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

