
void __cdecl
serde::de::impl$6::
next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl_0::deserialize::impl_2::visit_seq::__DeserializeWith>
          (void)

{
  char *pcVar1;
  undefined1 uVar2;
  undefined1 *in_RCX;
  longlong in_RDX;
  char local_38;
  undefined1 local_37;
  undefined8 local_30;
  
  pcVar1 = *(char **)(in_RDX + 8);
  if (pcVar1 == *(char **)(in_RDX + 0x18)) {
LAB_1415d0f5f:
    in_RCX[1] = 3;
  }
  else {
    *(char **)(in_RDX + 8) = pcVar1 + 0x20;
    if (*pcVar1 == '\x06') goto LAB_1415d0f5f;
    country_core::resources::walls::inner_wall_state::
    deserialize_wall_style<enum2_<serde_json::value::Value>_>();
    if (local_38 != '\0') {
      *(undefined8 *)(in_RCX + 8) = local_30;
      uVar2 = 1;
      goto LAB_1415d0f65;
    }
    in_RCX[1] = local_37;
  }
  uVar2 = 0;
LAB_1415d0f65:
  *in_RCX = uVar2;
  return;
}

