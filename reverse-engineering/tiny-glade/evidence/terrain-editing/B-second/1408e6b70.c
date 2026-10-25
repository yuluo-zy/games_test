
/* WARNING: Removing unreachable block (ram,0x0001408e6c13) */
/* WARNING: Removing unreachable block (ram,0x0001408e6c15) */

void __cdecl
country_core::resources::terrain_editing::_::impl$0::serialize<serde_json::value::ser::Serializer>
          (void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar1;
  undefined1 *in_RCX;
  
  serde_json::value::ser::impl$8::
  serialize_field<std::collections::hash::map::HashMap<country_core::resources::terrain_editing::TerrainStrokeId,country_core::resources::terrain_stroke::TerrainStroke,std::hash::random::RandomState>_>
            ();
  lVar1 = extraout_RAX;
  if ((extraout_RAX == 0) &&
     (serde_json::value::ser::impl$8::serialize_field<usize>(), lVar1 = extraout_RAX_00,
     extraout_RAX_00 == 0)) {
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

