// addr=0x14083e210 name=read_attribute_from_json
// signature=void __cdecl read_attribute_from_json(void)


void __cdecl country_core::utils::load_json::read_attribute_from_json(void)

{
  undefined8 **ppuVar1;
  undefined *puVar2;
  undefined8 **extraout_RAX;
  char *extraout_RAX_00;
  char *pcVar3;
  undefined8 extraout_RAX_01;
  char *extraout_RAX_02;
  undefined ***extraout_RAX_03;
  char *extraout_RAX_04;
  char *extraout_RAX_05;
  char *extraout_RAX_06;
  undefined ***pppuVar4;
  undefined8 *in_RCX;
  undefined8 local_130;
  undefined8 uStack_128;
  longlong local_120;
  undefined8 local_118;
  undefined8 uStack_110;
  longlong local_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  longlong local_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong local_d8;
  longlong local_c8;
  longlong *local_c0;
  longlong local_b8;
  undefined8 *local_b0 [2];
  undefined **local_a0;
  longlong **local_98;
  longlong **local_90;
  undefined8 **local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined **local_68;
  undefined8 **local_60;
  longlong *local_58;
  code *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  serde_json::value::index::impl$1::index_into();
  puVar2 = 
  __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
  ;
  serde_json::value::index::impl$1::index_into();
  local_68 = (undefined **)extraout_RAX;
  if (extraout_RAX == (undefined8 **)0x0) {
    local_68 = (undefined **)puVar2;
  }
  local_60 = (undefined8 **)serde_json::value::impl$1::fmt;
  local_98 = (longlong **)&DAT_142a937b8;
  local_90 = (longlong **)0x1;
  local_80 = 1;
  local_78 = 0;
  local_88 = &local_68;
  alloc::fmt::format::format_inner();
  if (local_b8 == 9) {
    if ((char)local_c0[1] == ']' && *local_c0 == 0x312c22746e69225b) {
      serde_json::value::index::impl$1::index_into();
      serde_json::value::index::impl$1::index_into();
      pcVar3 = extraout_RAX_04;
      if (extraout_RAX_04 == (char *)0x0) {
        pcVar3 = 
        __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
        ;
      }
      if (*pcVar3 != '\x04') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      alloc::vec::spec_from_iter::impl$0::
      from_iter<i32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2_<serde_json::value::Value>_>,country_core::utils::load_json::read_attribute_from_json::closure_env_3>_>
                ();
      if (local_108 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      in_RCX[3] = local_108;
      in_RCX[1] = local_118;
      in_RCX[2] = uStack_110;
      *in_RCX = 0;
      goto LAB_14083e484;
    }
  }
  else if (local_b8 == 0xb) {
    if (*(longlong *)((longlong)local_c0 + 3) == 0x5d342c2274616f6c &&
        *local_c0 == 0x2274616f6c66225b) {
      serde_json::value::index::impl$1::index_into();
      serde_json::value::index::impl$1::index_into();
      pcVar3 = extraout_RAX_02;
      if (extraout_RAX_02 == (char *)0x0) {
        pcVar3 = 
        __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
        ;
      }
      if (*pcVar3 != '\x04') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      ppuVar1 = *(undefined8 ***)(pcVar3 + 0x18);
      if (ppuVar1 == (undefined8 **)0x0) {
        pppuVar4 = (undefined ***)&DAT_00000004;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        pppuVar4 = extraout_RAX_03;
        if (extraout_RAX_03 == (undefined ***)0x0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
      }
      local_98 = &local_58;
      local_58 = (longlong *)0x0;
      local_90 = (longlong **)0x0;
      local_88 = pppuVar4;
      local_68 = (undefined **)ppuVar1;
      local_60 = pppuVar4;
      core::iter::adapters::map::impl$2::
      fold<array_<f32,4>,core::slice::iter::Iter<enum2_<serde_json::value::Value>_>,country_core::utils::load_json::read_attribute_from_json::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<array_<f32,4>,alloc::vec::impl_20::extend_trusted::closure_env_0<array_<f32,4>,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2_<serde_json::value::Value>_>,country_core::utils::load_json::read_attribute_from_json::closure_env_0>_>_>_>
                ();
      if (local_58 == (longlong *)0x0) {
        local_a0 = local_68;
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      *in_RCX = 4;
      in_RCX[1] = local_68;
      in_RCX[2] = local_60;
      in_RCX[3] = local_58;
      goto LAB_14083e484;
    }
    if (*(longlong *)((longlong)local_c0 + 3) == 0x5d332c2274616f6c &&
        *local_c0 == 0x2274616f6c66225b) {
      serde_json::value::index::impl$1::index_into();
      serde_json::value::index::impl$1::index_into();
      pcVar3 = extraout_RAX_05;
      if (extraout_RAX_05 == (char *)0x0) {
        pcVar3 = 
        __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
        ;
      }
      if (*pcVar3 != '\x04') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      local_98 = *(longlong ***)(pcVar3 + 0x10);
      local_90 = local_98 + *(longlong *)(pcVar3 + 0x18) * 4;
      local_88 = local_b0;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<array_<f32,3>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2_<serde_json::value::Value>_>,country_core::utils::load_json::read_attribute_from_json::closure_env_1>_>
                ();
      if (local_d8 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      in_RCX[3] = local_d8;
      in_RCX[1] = local_e8;
      in_RCX[2] = uStack_e0;
      *in_RCX = 3;
      goto LAB_14083e484;
    }
    if (*(longlong *)((longlong)local_c0 + 3) == 0x5d322c2274616f6c &&
        *local_c0 == 0x2274616f6c66225b) {
      serde_json::value::index::impl$1::index_into();
      serde_json::value::index::impl$1::index_into();
      pcVar3 = extraout_RAX_06;
      if (extraout_RAX_06 == (char *)0x0) {
        pcVar3 = 
        __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
        ;
      }
      if (*pcVar3 != '\x04') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      local_98 = *(longlong ***)(pcVar3 + 0x10);
      local_90 = local_98 + *(longlong *)(pcVar3 + 0x18) * 4;
      local_88 = local_b0;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<array_<f32,2>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2_<serde_json::value::Value>_>,country_core::utils::load_json::read_attribute_from_json::closure_env_2>_>
                ();
      if (local_f0 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      in_RCX[3] = local_f0;
      in_RCX[1] = local_100;
      in_RCX[2] = uStack_f8;
      *in_RCX = 2;
      goto LAB_14083e484;
    }
    if (*(longlong *)((longlong)local_c0 + 3) == 0x5d312c2274616f6c &&
        *local_c0 == 0x2274616f6c66225b) {
      serde_json::value::index::impl$1::index_into();
      serde_json::value::index::impl$1::index_into();
      pcVar3 = extraout_RAX_00;
      if (extraout_RAX_00 == (char *)0x0) {
        pcVar3 = puVar2;
      }
      if (*pcVar3 != '\x04') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      alloc::vec::spec_from_iter::impl$0::
      from_iter<f32,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2_<serde_json::value::Value>_>,country_core::utils::load_json::read_attribute_from_json::closure_env_4>_>
                ();
      if (local_120 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      in_RCX[3] = local_120;
      in_RCX[1] = local_130;
      in_RCX[2] = uStack_128;
      *in_RCX = 1;
      goto LAB_14083e484;
    }
  }
  local_68 = (undefined **)local_b0;
  local_60 = (undefined8 **)
             anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_50 = alloc::string::impl$23::fmt;
  local_98 = (longlong **)&DAT_142a93a50;
  local_90 = (longlong **)0x3;
  local_80 = 2;
  local_78 = 0;
  local_88 = &local_68;
  local_58 = &local_c8;
  alloc::fmt::format::format_inner();
  anyhow::Error::msg<alloc::string::String>();
  in_RCX[1] = extraout_RAX_01;
  *in_RCX = 5;
LAB_14083e484:
  if (local_c8 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


