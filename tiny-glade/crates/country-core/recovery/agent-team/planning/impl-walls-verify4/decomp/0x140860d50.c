
void __cdecl
country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::
visit_str<serde_json::error::Error>(void)

{
  undefined8 extraout_RAX;
  undefined1 uVar1;
  undefined1 *in_RCX;
  ulonglong uVar2;
  ulonglong *in_RDX;
  undefined8 in_R8;
  ulonglong uVar3;
  
  switch(in_R8) {
  case 5:
    if (*(char *)((longlong)in_RDX + 4) != 'e' || (int)*in_RDX != 0x6e6f7453) break;
    in_RCX[1] = 0;
    goto code_r0x000140860df5;
  case 0xc:
    uVar2 = *in_RDX ^ 0x626d6954666c6148;
    uVar3 = (ulonglong)(uint)in_RDX[1] ^ 0x64657265;
    goto code_r0x000140860dec;
  case 0xd:
    uVar2 = *in_RDX ^ 0x6e41656c74746157;
    uVar3 = *(ulonglong *)((longlong)in_RDX + 5) ^ 0x62756144646e4165;
code_r0x000140860dec:
    if (uVar3 == 0 && uVar2 == 0) {
      in_RCX[1] = 2;
code_r0x000140860df5:
      uVar1 = 0;
      goto LAB_140860df7;
    }
    break;
  case 0xf:
    if (*(longlong *)((longlong)in_RDX + 7) == 0x737472696b536874 && *in_RDX == 0x746957656e6f7453)
    {
      in_RCX[1] = 1;
      goto code_r0x000140860df5;
    }
  }
  serde::de::Error::unknown_variant<serde_json::error::Error>();
  *(undefined8 *)(in_RCX + 8) = extraout_RAX;
  uVar1 = 1;
LAB_140860df7:
  *in_RCX = uVar1;
  return;
}

