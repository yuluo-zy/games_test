
/* WARNING: Removing unreachable block (ram,0x000140a4b342) */
/* WARNING: Removing unreachable block (ram,0x000140a4b344) */

void __cdecl
country_core::resources::walls::decorator::_::impl$0::serialize<serde_json::value::ser::Serializer>
          (void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar1;
  undefined1 *in_RCX;
  
  serde_json::value::ser::impl$8::serialize_field<utils::wall_space::WallSpaceCoord>();
  lVar1 = extraout_RAX;
  if ((extraout_RAX == 0) &&
     (serde_json::value::ser::impl$8::
      serialize_field<country_core::resources::walls::wall_attachment::WallAttachmentAnchor>(),
     lVar1 = extraout_RAX_00, extraout_RAX_00 == 0)) {
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

