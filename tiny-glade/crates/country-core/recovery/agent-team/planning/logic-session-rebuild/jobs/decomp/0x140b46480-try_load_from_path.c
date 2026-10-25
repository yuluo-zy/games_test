// 0x140b46480 try_load_from_path
// signature: void __cdecl try_load_from_path(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "from<serde_json::error::Error>", "address": "1428a9850"}, {"name": "from_str<enum2$<serde_json::value::Value>_>", "address": "141405930"}, {"name": "index_into", "address": "1414744d0"}, {"name": "inner", "address": "140d59140"}, {"name": "with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::resources::session::impl$3::try_load_from_path::closure_env$0>", "address": "1415a5730"}]


// WARNING: Control flow encountered bad instruction data

void __cdecl country_core::resources::session::SessionMeta::try_load_from_path(void)

{
  char cVar1;
  uint uVar2;
  longlong extraout_RAX;
  undefined1 *extraout_RAX_00;
  undefined1 *puVar3;
  longlong extraout_RAX_01;
  longlong *in_RCX;
  undefined4 local_e8;
  undefined4 uStack_e4;
  longlong local_e0;
  longlong local_d8;
  longlong lStack_d0;
  longlong lStack_c8;
  longlong lStack_c0;
  longlong lStack_b8;
  longlong lStack_b0;
  longlong lStack_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  longlong local_78;
  undefined1 local_70;
  undefined1 uStack_6f;
  undefined6 uStack_6e;
  longlong local_68;
  undefined8 local_60;
  undefined8 uStack_58;
  longlong local_50;
  byte local_48;
  undefined4 uStack_47;
  undefined3 uStack_43;
  longlong local_40;
  longlong local_38;
  longlong lStack_28;
  undefined8 local_20;
  char *pcVar4;
  
  local_20 = 0xfffffffffffffffe;
  std::fs::read_to_string::inner();
  anyhow::context::impl$0::
  with_context<alloc::string::String,std::io::error::Error,alloc::string::String,country_core::resources::session::impl_3::try_load_from_path::closure_env_0>
            ();
  local_50 = CONCAT62(uStack_6e,CONCAT11(uStack_6f,local_70));
  if (SBORROW8(0,local_50)) {
    in_RCX[1] = local_68;
    *in_RCX = -0x8000000000000000;
    return;
  }
  local_78 = local_68;
  serde_json::de::from_str<enum2_<serde_json::value::Value>_>();
  if ((byte)local_e8 == 6) {
    anyhow::error::impl$1::from<serde_json::error::Error>();
    in_RCX[1] = extraout_RAX;
    *in_RCX = -0x8000000000000000;
  }
  else {
    uStack_47 = CONCAT13((undefined1)uStack_e4,local_e8._1_3_);
    uStack_43 = (undefined3)((uint)uStack_e4 >> 8);
    local_38 = local_d8;
    local_48 = (byte)local_e8;
    local_40 = local_e0;
    serde_json::value::index::impl$1::index_into();
    puVar3 = extraout_RAX_00;
    if (extraout_RAX_00 == (undefined1 *)0x0) {
      puVar3 = 
      __imp__ZN10serde_json5value5index87__LT_impl_u20_core__ops__index__Index_LT_I_GT__u20_for_u20_serde_json__value__Value_GT_5index4NULL17hd2864d0cd0b4dde5E
      ;
    }
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*puVar3) {
    case 0:
      local_70 = 0;
      break;
    case 1:
      uStack_6f = puVar3[1];
      local_70 = 1;
      break;
    case 2:
      local_68 = *(longlong *)(puVar3 + 8);
      local_60 = *(undefined8 *)(puVar3 + 0x10);
      local_70 = 2;
      break;
    case 3:
      alloc::string::impl$6::clone();
      local_70 = 3;
      break;
    case 4:
      alloc::vec::impl$11::clone<enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      local_70 = 4;
      break;
    case 5:
      if (*(longlong *)(puVar3 + 0x18) == 0) {
        local_68 = 0;
        uStack_58 = 0;
      }
      else {
        if (*(longlong *)(puVar3 + 8) == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        alloc::collections::btree::map::impl$2::clone::
        clone_subtree<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>()
        ;
      }
      local_70 = 5;
      break;
    default:
      out((char)*in_RCX,0xf72c);
      pcVar4 = (char *)((longlong)in_RCX + (longlong)&local_98 * 2 + 0x6e71fe06);
      *pcVar4 = *pcVar4 + '\x01';
      uVar2 = (int)puVar3 + 0xadd4efc;
      pcVar4 = (char *)(ulonglong)uVar2;
      *pcVar4 = *pcVar4 + '\x01';
      cVar1 = (char)uVar2;
      *pcVar4 = *pcVar4 + cVar1;
      *pcVar4 = *pcVar4 + cVar1;
      *pcVar4 = *pcVar4 + cVar1;
      *pcVar4 = *pcVar4 + cVar1;
      *pcVar4 = *pcVar4 + cVar1;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    serde_json::value::de::impl$2::
    deserialize_struct<country_core::resources::session::_::impl_0::deserialize::__Visitor>();
    if (SBORROW8(0,CONCAT44(uStack_e4,local_e8))) {
      anyhow::error::impl$1::from<serde_json::error::Error>();
      in_RCX[1] = extraout_RAX_01;
      *in_RCX = -0x8000000000000000;
      if (local_48 < 3) goto code_r0x000140b4651b;
      if (local_48 != 3) {
        if (local_48 != 4) {
          core::ptr::
          drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                    ();
          goto code_r0x000140b4651b;
        }
        lStack_28 = local_38;
        core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
      }
    }
    else {
      in_RCX[10] = local_98;
      in_RCX[0xb] = lStack_90;
      in_RCX[8] = lStack_a8;
      in_RCX[9] = lStack_a0;
      in_RCX[6] = lStack_b8;
      in_RCX[7] = lStack_b0;
      in_RCX[4] = lStack_c8;
      in_RCX[5] = lStack_c0;
      in_RCX[2] = local_d8;
      in_RCX[3] = lStack_d0;
      *in_RCX = CONCAT44(uStack_e4,local_e8);
      in_RCX[1] = local_e0;
      if (local_48 < 3) goto code_r0x000140b4651b;
      if (local_48 != 3) {
        if (local_48 != 4) {
          core::ptr::
          drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                    ();
          goto code_r0x000140b4651b;
        }
        lStack_28 = local_38;
        core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
      }
    }
    if (local_40 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
code_r0x000140b4651b:
  if (local_50 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

