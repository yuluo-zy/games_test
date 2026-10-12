
/* WARNING: Removing unreachable block (ram,0x0001416eab9d) */
/* WARNING: Removing unreachable block (ram,0x0001416eab9f) */

void __cdecl
country_core::resources::roofs::roof_shape::_::impl$0::serialize<serde_json::value::ser::Serializer>
          (void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong lVar1;
  undefined1 *in_RCX;
  
  serde_json::value::ser::impl$8::serialize_field<f32>();
  lVar1 = extraout_RAX;
  if ((((extraout_RAX == 0) &&
       (serde_json::value::ser::impl$8::serialize_field<f32>(), lVar1 = extraout_RAX_00,
       extraout_RAX_00 == 0)) &&
      (serde_json::value::ser::impl$8::serialize_field<glam::f32::vec2::Vec2>(),
      lVar1 = extraout_RAX_01, extraout_RAX_01 == 0)) &&
     (((serde_json::value::ser::impl$8::serialize_field<f32>(), lVar1 = extraout_RAX_02,
       extraout_RAX_02 == 0 &&
       (serde_json::value::ser::impl$8::
        serialize_field<country_core::resources::roofs::roof_shape::RoofRidgeDirection>(),
       lVar1 = extraout_RAX_03, extraout_RAX_03 == 0)) &&
      ((serde_json::value::ser::impl$8::serialize_field<f32>(), lVar1 = extraout_RAX_04,
       extraout_RAX_04 == 0 &&
       (serde_json::value::ser::impl$8::
        serialize_field<country_core::resources::roofs::roof_shape::FlatRoofRimStyle>(),
       lVar1 = extraout_RAX_05, extraout_RAX_05 == 0)))))) {
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

