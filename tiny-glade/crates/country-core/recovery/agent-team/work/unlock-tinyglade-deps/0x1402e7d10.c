
// WARNING: Removing unreachable block (ram,0x0001402e7d60)

void __cdecl
country_core::game_config::_::impl$0::
serialize<ref_mut_<ron::ser::Serializer<ref_mut_<alloc::string::String>_>_>_>(void)

{
  undefined8 *in_RCX;
  char *in_RDX;
  char local_58;
  undefined7 uStack_57;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined8 local_28;
  undefined8 uStack_20;
  undefined8 local_18;
  
  if (*in_RDX == '\0') {
    ron::ser::Serializer::validate_identifier<ref_mut_<alloc::string::String>_>();
    if (local_58 == '2') {
      ron::ser::Serializer::write_identifier<ref_mut_<alloc::string::String>_>();
      goto LAB_1402e7da1;
    }
  }
  else {
    ron::ser::Serializer::validate_identifier<ref_mut_<alloc::string::String>_>();
    if (local_58 == '2') {
      ron::ser::Serializer::write_identifier<ref_mut_<alloc::string::String>_>();
LAB_1402e7da1:
      *(undefined1 *)in_RCX = 0x32;
      return;
    }
  }
  in_RCX[8] = local_18;
  in_RCX[6] = local_28;
  in_RCX[7] = uStack_20;
  in_RCX[4] = local_38;
  in_RCX[5] = uStack_30;
  in_RCX[2] = local_48;
  in_RCX[3] = uStack_40;
  *in_RCX = CONCAT71(uStack_57,local_58);
  in_RCX[1] = uStack_50;
  return;
}

