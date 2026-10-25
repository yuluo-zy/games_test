
// WARNING: Removing unreachable block (ram,0x000140039d63)
// WARNING: Removing unreachable block (ram,0x000140039d96)
// WARNING: Removing unreachable block (ram,0x000140039da4)
// WARNING: Removing unreachable block (ram,0x000140039da8)

void __cdecl
country_core::render::render_config::_::impl$0::
serialize<ref_mut_<ron::ser::Serializer<ref_mut_<alloc::string::String>_>_>_>(void)

{
  char *in_RCX;
  char local_78;
  undefined7 uStack_77;
  undefined1 uStack_70;
  undefined7 uStack_6f;
  undefined1 uStack_68;
  undefined7 uStack_67;
  undefined1 uStack_60;
  undefined7 uStack_5f;
  undefined1 uStack_58;
  undefined7 local_57;
  undefined1 uStack_50;
  undefined7 uStack_4f;
  undefined1 uStack_48;
  undefined7 local_47;
  undefined1 uStack_40;
  undefined7 uStack_3f;
  undefined1 uStack_38;
  undefined7 uStack_37;
  
  ron::ser::impl$3::serialize_struct<ref_mut_<alloc::string::String>_>();
  if (local_78 == '2') {
    ron::ser::impl$11::
    serialize_field<ref_mut_<alloc::string::String>,enum2_<core::option::Option<alloc::string::String>_>_>
              ();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,array_<i32,2>_>();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,array_<u32,2>_>();
    ron::ser::impl$11::end<ref_mut_<alloc::string::String>_>();
  }
  else {
    *(ulonglong *)(in_RCX + 0x40) = CONCAT71(uStack_37,uStack_38);
    *(ulonglong *)(in_RCX + 0x30) = CONCAT71(local_47,uStack_48);
    *(ulonglong *)(in_RCX + 0x38) = CONCAT71(uStack_3f,uStack_40);
    *(ulonglong *)(in_RCX + 0x20) = CONCAT71(local_57,uStack_58);
    *(ulonglong *)(in_RCX + 0x28) = CONCAT71(uStack_4f,uStack_50);
    *(ulonglong *)(in_RCX + 0x10) = CONCAT71(uStack_67,uStack_68);
    *(ulonglong *)(in_RCX + 0x18) = CONCAT71(uStack_5f,uStack_60);
    *(ulonglong *)(in_RCX + 1) = CONCAT17(uStack_70,uStack_77);
    *(ulonglong *)(in_RCX + 9) = CONCAT17(uStack_68,uStack_6f);
    *in_RCX = local_78;
  }
  return;
}

