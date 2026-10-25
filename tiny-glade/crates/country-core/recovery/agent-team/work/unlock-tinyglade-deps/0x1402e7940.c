
// WARNING: Removing unreachable block (ram,0x0001402e7c66)
// WARNING: Removing unreachable block (ram,0x0001402e7c99)
// WARNING: Removing unreachable block (ram,0x0001402e7ca7)
// WARNING: Removing unreachable block (ram,0x0001402e7cab)

void __cdecl
country_core::game_config::_::impl$0::
serialize<ref_mut_<ron::ser::Serializer<ref_mut_<alloc::string::String>_>_>_>(void)

{
  char cVar1;
  char cVar2;
  char *in_RCX;
  longlong in_RDX;
  char local_88;
  undefined7 uStack_87;
  undefined1 uStack_80;
  undefined7 uStack_7f;
  undefined1 uStack_78;
  undefined7 uStack_77;
  undefined1 uStack_70;
  undefined7 uStack_6f;
  undefined1 uStack_68;
  undefined7 local_67;
  undefined1 uStack_60;
  undefined7 uStack_5f;
  undefined1 uStack_58;
  undefined7 local_57;
  undefined1 uStack_50;
  undefined7 uStack_4f;
  undefined1 uStack_48;
  undefined7 uStack_47;
  
  cVar1 = *(char *)(in_RDX + 0x14);
  cVar2 = *(char *)(in_RDX + 0x15);
  ron::ser::impl$3::serialize_struct<ref_mut_<alloc::string::String>_>();
  if (local_88 == '2') {
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,f32>();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,f32>();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,bool>();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,bool>();
    if (cVar1 != '\x06') {
      ron::ser::impl$11::
      serialize_field<ref_mut_<alloc::string::String>,country_core::game_config::_::impl_0::serialize::__SerializeWith>
                ();
    }
    if (cVar2 != '\x06') {
      ron::ser::impl$11::
      serialize_field<ref_mut_<alloc::string::String>,country_core::game_config::_::impl_0::serialize::__SerializeWith>
                ();
    }
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,f32>();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,f32>();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,f32>();
    ron::ser::impl$11::
    serialize_field<ref_mut_<alloc::string::String>,country_core::game_config::ScreenshotFormat>();
    ron::ser::impl$11::serialize_field<ref_mut_<alloc::string::String>,bool>();
    ron::ser::impl$11::end<ref_mut_<alloc::string::String>_>();
  }
  else {
    *(ulonglong *)(in_RCX + 0x40) = CONCAT71(uStack_47,uStack_48);
    *(ulonglong *)(in_RCX + 0x30) = CONCAT71(local_57,uStack_58);
    *(ulonglong *)(in_RCX + 0x38) = CONCAT71(uStack_4f,uStack_50);
    *(ulonglong *)(in_RCX + 0x20) = CONCAT71(local_67,uStack_68);
    *(ulonglong *)(in_RCX + 0x28) = CONCAT71(uStack_5f,uStack_60);
    *(ulonglong *)(in_RCX + 0x10) = CONCAT71(uStack_77,uStack_78);
    *(ulonglong *)(in_RCX + 0x18) = CONCAT71(uStack_6f,uStack_70);
    *(ulonglong *)(in_RCX + 1) = CONCAT17(uStack_80,uStack_87);
    *(ulonglong *)(in_RCX + 9) = CONCAT17(uStack_78,uStack_7f);
    *in_RCX = local_88;
  }
  return;
}

