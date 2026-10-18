
// WARNING: Removing unreachable block (ram,0x0001405c9d15)
// WARNING: Removing unreachable block (ram,0x0001405c9d17)

void __cdecl
country_core::resources::critters::sheep::control::_::impl$0::
serialize<serde_json::value::ser::Serializer>(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar1;
  undefined1 *in_RCX;
  
  serde_json::value::ser::impl$8::serialize_field<glam::f32::vec3::Vec3>();
  lVar1 = extraout_RAX;
  if (((extraout_RAX == 0) &&
      (serde_json::value::ser::impl$8::
       serialize_field<enum2_<core::option::Option<glam::f32::vec3::Vec3>_>_>(),
      lVar1 = extraout_RAX_00, extraout_RAX_00 == 0)) &&
     (serde_json::value::ser::impl$8::
      serialize_field<enum2_<country_core::resources::critters::sheep::SheepMoveState>_>(),
     lVar1 = extraout_RAX_01, extraout_RAX_01 == 0)) {
    serde_json::value::ser::impl$6::end();
  }
  else {
    *(longlong *)(in_RCX + 8) = lVar1;
    *in_RCX = 6;
    core::ptr::
    drop_in_place<serde_json::map::Map<alloc::string::String,enum2_<serde_json::value::Value>_>_>();
  }
  return;
}

