
void __cdecl
country_core::resources::critters::sheep::_::impl$0::
deserialize<ref_mut_<serde_json::de::Deserializer<serde_json::read::StrRead>_>_>(void)

{
  char *pcVar1;
  byte bVar2;
  undefined4 uVar3;
  ulonglong uVar4;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined4 *in_RCX;
  longlong in_RDX;
  ulonglong uVar5;
  char local_50;
  char cStack_4f;
  undefined8 local_48;
  uint local_38;
  undefined4 uStack_34;
  undefined4 local_30;
  undefined4 uStack_2c;
  
  uVar4 = *(ulonglong *)(in_RDX + 0x28);
  if (uVar4 < *(ulonglong *)(in_RDX + 0x20)) {
    do {
      uVar4 = uVar4 + 1;
      bVar2 = *(byte *)(*(longlong *)(in_RDX + 0x18) + -1 + uVar4);
      uVar5 = (ulonglong)bVar2;
      if (0x22 < uVar5) {
LAB_1413ff939:
        if (bVar2 == 0x7b) {
          pcVar1 = (char *)(in_RDX + 0x30);
          *pcVar1 = *pcVar1 + -1;
          if (*pcVar1 != '\0') {
            *(ulonglong *)(in_RDX + 0x28) = uVar4;
            serde::de::impl$5::
            deserialize<country_core::resources::critters::sheep::_::impl_0::deserialize::__Field,ref_mut_<serde_json::de::Deserializer<serde_json::read::StrRead>_>_>
                      ();
            if (local_50 != '\0') goto LAB_1413ff969;
            uVar4 = *(ulonglong *)(in_RDX + 0x28);
            if (*(ulonglong *)(in_RDX + 0x20) <= uVar4) goto LAB_1413ff9d8;
            goto LAB_1413ff9a5;
          }
        }
        break;
      }
      if ((0x100002600U >> (uVar5 & 0x3f) & 1) == 0) {
        if (uVar5 != 0x22) goto LAB_1413ff939;
        serde::de::impl$5::
        deserialize<country_core::resources::critters::sheep::_::impl_0::deserialize::__Field,ref_mut_<serde_json::de::Deserializer<serde_json::read::StrRead>_>_>
                  ();
        if (local_50 == '\0') {
          serde_json::error::impl$8::invalid_type();
          local_48 = extraout_RAX_02;
        }
        goto LAB_1413ffae3;
      }
      *(ulonglong *)(in_RDX + 0x28) = uVar4;
    } while (uVar4 != *(ulonglong *)(in_RDX + 0x20));
  }
  serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>();
  local_48 = extraout_RAX;
  goto LAB_1413ffae3;
LAB_1413ffa51:
  uVar4 = uVar4 + 1;
  bVar2 = *(byte *)(*(longlong *)(in_RDX + 0x18) + -1 + uVar4);
  if ((0x20 < (ulonglong)bVar2) || ((0x100002600U >> ((ulonglong)bVar2 & 0x3f) & 1) == 0)) {
    if (bVar2 == 0x7d) {
      *(ulonglong *)(in_RDX + 0x28) = uVar4;
      in_RCX[3] = uStack_2c;
      *(ulonglong *)(in_RCX + 1) = CONCAT44(local_30,uStack_34);
      uVar3 = 0;
      goto LAB_1413ffab5;
    }
    goto LAB_1413ffa80;
  }
  *(ulonglong *)(in_RDX + 0x28) = uVar4;
  if (uVar4 == *(ulonglong *)(in_RDX + 0x20)) {
LAB_1413ffa80:
    serde_json::de::Deserializer::error<serde_json::read::SliceRead>();
    *(undefined8 *)(in_RCX + 2) = extraout_RAX_01;
    uVar3 = 1;
LAB_1413ffab5:
    *in_RCX = uVar3;
    return;
  }
  goto LAB_1413ffa51;
  while( true ) {
    if ((0x100002600U >> (uVar5 & 0x3f) & 1) == 0) {
      if (uVar5 == 0x3a) {
        *(ulonglong *)(in_RDX + 0x28) = uVar4;
        if (cStack_4f == '\0') {
          serde_json::de::impl$14::
          struct_variant<serde_json::read::StrRead,country_core::resources::critters::sheep::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
                    ();
        }
        else {
          serde_json::de::impl$14::
          struct_variant<serde_json::read::StrRead,country_core::resources::critters::sheep::_::impl_0::deserialize::impl_2::visit_enum::__Visitor>
                    ();
        }
        if ((local_38 & 1) != 0) goto LAB_1413ffade;
        *(char *)(in_RDX + 0x30) = *(char *)(in_RDX + 0x30) + '\x01';
        uVar4 = *(ulonglong *)(in_RDX + 0x28);
        if (*(ulonglong *)(in_RDX + 0x20) <= uVar4) goto LAB_1413ffa80;
        goto LAB_1413ffa51;
      }
      break;
    }
    *(ulonglong *)(in_RDX + 0x28) = uVar4;
    if (uVar4 == *(ulonglong *)(in_RDX + 0x20)) break;
LAB_1413ff9a5:
    uVar4 = uVar4 + 1;
    uVar5 = (ulonglong)*(byte *)(*(longlong *)(in_RDX + 0x18) + -1 + uVar4);
    if (0x3a < uVar5) break;
  }
LAB_1413ff9d8:
  serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>();
  local_48 = extraout_RAX_00;
LAB_1413ff969:
  local_30 = (undefined4)local_48;
  uStack_2c = (undefined4)((ulonglong)local_48 >> 0x20);
LAB_1413ffade:
  local_48 = CONCAT44(uStack_2c,local_30);
LAB_1413ffae3:
  *(undefined8 *)(in_RCX + 2) = local_48;
  *in_RCX = 1;
  return;
}

