
void __cdecl
country_core::resources::history::auto_clutter_edit::_::impl$0::
deserialize<enum2_<serde_json::value::Value>_>(void)

{
  undefined8 *puVar1;
  char cVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  undefined4 uVar8;
  char *in_RCX;
  longlong lVar9;
  longlong local_150;
  longlong local_140;
  byte bStack_100;
  byte local_d8;
  undefined4 local_78;
  undefined4 uStack_74;
  undefined1 local_68;
  undefined7 uStack_67;
  
  cVar2 = *in_RCX;
  if (cVar2 == '\x03') {
    lVar9 = *(longlong *)(in_RCX + 8);
LAB_140a18638:
    serde_json::value::de::impl$3::
    variant_seed<core::marker::PhantomData<enum2_<country_core::resources::history::auto_clutter_edit::_::impl_0::deserialize::__Field>_>_>
              ();
    if ((byte)local_78 != '\a') {
      serde_json::value::de::impl$5::
      struct_variant<country_core::resources::history::auto_clutter_edit::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
                ();
    }
    if (cVar2 == '\x03') {
      return;
    }
    if (cVar2 == '\x05') {
      return;
    }
    core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
    if (lVar9 == 0) {
      return;
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  }
  if (cVar2 != '\x05') {
    uVar4 = *(undefined8 *)in_RCX;
    lVar9 = *(longlong *)(in_RCX + 8);
    serde_json::value::Value::unexpected();
    serde_json::error::impl$8::invalid_type();
    local_d8 = (byte)uVar4;
    if (local_d8 < 3) {
      return;
    }
    if (local_d8 == 3) {
      if (lVar9 == 0) {
        return;
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
      return;
    }
    if (local_d8 != 4) {
      core::ptr::
      drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                ();
      return;
    }
    core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
    if (lVar9 == 0) {
      return;
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  }
  lVar9 = *(longlong *)(in_RCX + 8);
  alloc::collections::btree::map::IntoIter::
  dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
  lVar5 = CONCAT44(uStack_74,local_78);
  if (lVar5 != 0) {
    lVar3 = *(longlong *)(lVar5 + 0x168 + CONCAT71(uStack_67,local_68) * 0x18);
    puVar1 = (undefined8 *)(lVar5 + CONCAT71(uStack_67,local_68) * 0x20);
    uVar4 = *puVar1;
    lVar5 = puVar1[1];
    if (!SBORROW8(0,lVar3)) {
      local_78 = (undefined4)uVar4;
      uVar8 = local_78;
      alloc::collections::btree::map::IntoIter::
      dying_next<alloc::string::String,enum2_<serde_json::value::Value>,alloc::alloc::Global>();
      local_78._0_1_ = (byte)uVar4;
      if (local_150 != 0) {
        lVar6 = *(longlong *)(local_150 + 0x168 + local_140 * 0x18);
        puVar1 = (undefined8 *)(local_150 + local_140 * 0x20);
        uVar4 = *puVar1;
        lVar7 = puVar1[1];
        if (!SBORROW8(0,lVar6)) {
          if (lVar6 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          bStack_100 = (byte)uVar4;
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
          if (2 < (byte)local_78) {
            if ((byte)local_78 == 3) {
              if (lVar5 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
            else if ((byte)local_78 == 4) {
              core::ptr::drop_in_place<serde_json::value::ser::SerializeVec>();
            }
            else {
              core::ptr::
              drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                        ();
            }
          }
          if (lVar3 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          goto LAB_140a1860c;
        }
      }
      core::ptr::drop_in_place<serde_json::map::IntoIter>();
      local_78 = uVar8;
      goto LAB_140a18638;
    }
  }
  serde::de::Error::invalid_value<serde_json::error::Error>();
LAB_140a1860c:
  core::ptr::drop_in_place<serde_json::map::IntoIter>();
  return;
}

