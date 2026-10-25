
void __cdecl
serde_json::value::de::impl$2::
deserialize_struct<country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::__Visitor>
          (void)

{
  byte bVar1;
  longlong lVar2;
  bool bVar3;
  bool bVar4;
  undefined8 extraout_RAX;
  undefined8 *in_RCX;
  byte *in_RDX;
  
  if (*in_RDX == 4) {
    visit_array<country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::__Visitor>
              ();
    bVar4 = false;
    bVar1 = *in_RDX;
  }
  else {
    if (*in_RDX == 5) {
      visit_object<country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::__Visitor>
                ();
      bVar4 = true;
      bVar3 = false;
      bVar1 = *in_RDX;
      goto joined_r0x00014159fdeb;
    }
    Value::invalid_type<serde_json::error::Error>();
    in_RCX[1] = extraout_RAX;
    *in_RCX = 0x8000000000000000;
    bVar4 = true;
    bVar1 = *in_RDX;
  }
  bVar3 = true;
joined_r0x00014159fdeb:
  if (bVar1 == 4) {
    if (!bVar4) {
      return;
    }
    core::ptr::drop_in_place<slice2_<enum2_<serde_json::value::Value>_>_>();
    lVar2 = *(longlong *)(in_RDX + 8);
  }
  else {
    if (bVar1 == 5) {
      if (bVar3) {
        core::ptr::
        drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>
                  ();
        return;
      }
      return;
    }
    if (bVar1 < 3) {
      return;
    }
    lVar2 = *(longlong *)(in_RDX + 8);
  }
  if (lVar2 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

