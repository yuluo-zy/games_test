
void __cdecl
country_core::resources::history::wall_compound_edit::_::impl$0::
deserialize<enum2_<serde_json::value::Value>_>(void)

{
  longlong *plVar1;
  undefined8 *puVar2;
  char cVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  longlong lVar7;
  undefined4 uVar8;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  longlong extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined4 *in_RCX;
  char *in_RDX;
  longlong lVar9;
  longlong local_150;
  longlong local_140;
  longlong local_118;
  longlong lStack_110;
  byte bStack_100;
  byte local_d8;
  byte local_78;
  undefined7 uStack_77;
  undefined1 local_68;
  undefined7 uStack_67;
  
  cVar3 = *in_RDX;
  if (cVar3 != '\x03') {
    if (cVar3 != '\x05') {
      uVar6 = *(undefined8 *)in_RDX;
      lVar9 = *(longlong *)(in_RDX + 8);
      serde_json::value::Value::unexpected();
      serde_json::error::impl$8::invalid_type();
      *(undefined8 *)(in_RCX + 2) = extraout_RAX_00;
      *in_RCX = 1;
      local_d8 = (byte)uVar6;
      if (local_d8 < 3) {
        return;
      }
      if (local_d8 != 3) {
        if (local_d8 != 4) {
          core::ptr::
          drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                    ();
          return;
        }
        core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
      }
      goto joined_r0x000141402e11;
    }
    lVar9 = *(longlong *)(in_RDX + 8);
    alloc::collections::btree::map::IntoIter::
    dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
    lVar5 = CONCAT71(uStack_77,local_78);
    if (lVar5 != 0) {
      lVar4 = *(longlong *)(lVar5 + 0x168 + CONCAT71(uStack_67,local_68) * 0x18);
      plVar1 = (longlong *)(lVar5 + CONCAT71(uStack_67,local_68) * 0x20);
      local_118 = *plVar1;
      lVar5 = plVar1[1];
      if (!SBORROW8(0,lVar4)) {
        local_78 = (byte)local_118;
        alloc::collections::btree::map::IntoIter::
        dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
        lStack_110 = lVar5;
        if (local_150 != 0) {
          plVar1 = (longlong *)(local_150 + 0x168 + local_140 * 0x18);
          local_118 = *plVar1;
          lStack_110 = plVar1[1];
          puVar2 = (undefined8 *)(local_150 + local_140 * 0x20);
          uVar6 = *puVar2;
          lVar7 = puVar2[1];
          if (!SBORROW8(0,local_118)) {
            if (local_118 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            bStack_100 = (byte)uVar6;
            if (2 < bStack_100) {
              if (bStack_100 == 3) {
                if (lVar7 != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
              }
              else if (bStack_100 == 4) {
                core::ptr::drop_in_place<serde_json::value::ser::SerializeVec>();
              }
              else {
                core::ptr::
                drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                          ();
              }
            }
            serde::de::Error::invalid_value<serde_json::error::Error>();
            *(undefined8 *)(in_RCX + 2) = extraout_RAX_02;
            *in_RCX = 1;
            if (2 < local_78) {
              if (local_78 == 3) {
                if (lVar5 != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
              }
              else if (local_78 == 4) {
                core::ptr::drop_in_place<serde_json::value::ser::SerializeVec>();
              }
              else {
                core::ptr::
                drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                          ();
              }
            }
            if (lVar4 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            goto LAB_141402d58;
          }
        }
        core::ptr::drop_in_place<serde_json::map::IntoIter>();
        goto LAB_141402d85;
      }
    }
    serde::de::Error::invalid_value<serde_json::error::Error>();
    *(undefined8 *)(in_RCX + 2) = extraout_RAX;
    *in_RCX = 1;
LAB_141402d58:
    core::ptr::drop_in_place<serde_json::map::IntoIter>();
    return;
  }
  lVar9 = *(longlong *)(in_RDX + 8);
LAB_141402d85:
  serde_json::value::de::impl$3::
  variant_seed<core::marker::PhantomData<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::__Field>_>
            ();
  if ((char)local_118 == '\x03') {
LAB_141402dca:
    *(longlong *)(in_RCX + 2) = lStack_110;
    uVar8 = 1;
LAB_141402dd3:
    *in_RCX = uVar8;
  }
  else if ((char)local_118 == '\0') {
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
  }
  else {
    if ((char)local_118 != '\x01') {
      serde_json::value::de::impl$5::unit_variant();
      lStack_110 = extraout_RAX_01;
      if (extraout_RAX_01 != 0) goto LAB_141402dca;
      in_RCX[1] = 2;
      uVar8 = 0;
      goto LAB_141402dd3;
    }
    serde_json::value::de::impl$5::
    struct_variant<country_core::resources::history::wall_compound_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
              ();
  }
  if (cVar3 == '\x03') {
    return;
  }
  if (cVar3 == '\x05') {
    return;
  }
  core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
joined_r0x000141402e11:
  if (lVar9 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

