
void __cdecl
country_core::game_config::_::impl$0::deserialize::impl$0::visit_str<enum2_<ron::error::Error>_>
          (void)

{
  undefined1 *in_RCX;
  longlong *in_RDX;
  undefined8 in_R8;
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  
  switch(in_R8) {
  case 4:
    if ((int)*in_RDX == 0x6f6d6564) {
      in_RCX[1] = 6;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0xb:
    if (*(longlong *)((longlong)in_RDX + 3) == 0x6873616c70735f70 && *in_RDX == 0x6c70735f70696b73)
    {
      in_RCX[1] = 4;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0xc:
    if ((int)in_RDX[1] == 0x656d6f63 && *in_RDX == 0x6c65775f70696b73) {
      in_RCX[1] = 5;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0xe:
    if (*(longlong *)((longlong)in_RDX + 6) == 0x736568736172635f && *in_RDX == 0x635f74726f706572)
    {
      in_RCX[1] = 1;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0xf:
    if (*(longlong *)((longlong)in_RDX + 7) == 0x736568736172635f && *in_RDX == 0x5f65727574706163)
    {
      in_RCX[1] = 0;
      *in_RCX = 0x32;
      return;
    }
    if (*(longlong *)((longlong)in_RDX + 7) == 0x656c6379635f7468 && *in_RDX == 0x6867696e5f796164)
    {
      in_RCX[1] = 3;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0x10:
    auVar1[0] = -((char)*in_RDX == 'i');
    auVar1[1] = -(*(char *)((longlong)in_RDX + 1) == 'n');
    auVar1[2] = -(*(char *)((longlong)in_RDX + 2) == 't');
    auVar1[3] = -(*(char *)((longlong)in_RDX + 3) == 'e');
    auVar1[4] = -(*(char *)((longlong)in_RDX + 4) == 'r');
    auVar1[5] = -(*(char *)((longlong)in_RDX + 5) == 'n');
    auVar1[6] = -(*(char *)((longlong)in_RDX + 6) == 'a');
    auVar1[7] = -(*(char *)((longlong)in_RDX + 7) == 'l');
    auVar1[8] = -((char)in_RDX[1] == '_');
    auVar1[9] = -(*(char *)((longlong)in_RDX + 9) == 'h');
    auVar1[10] = -(*(char *)((longlong)in_RDX + 10) == 'o');
    auVar1[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar1[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'k');
    auVar1[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'e');
    auVar1[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'y');
    auVar1[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 's');
    if ((ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe | (ushort)(auVar1[0xf] >> 7) << 0xf)
        == 0xffff) {
      in_RCX[1] = 2;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0x15:
    auVar3[0] = -(*(char *)((longlong)in_RDX + 5) == 'e');
    auVar3[1] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar3[2] = -(*(char *)((longlong)in_RDX + 7) == 'a');
    auVar3[3] = -((char)in_RDX[1] == 's');
    auVar3[4] = -(*(char *)((longlong)in_RDX + 9) == 's');
    auVar3[5] = -(*(char *)((longlong)in_RDX + 10) == 'e');
    auVar3[6] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar3[7] = -(*(char *)((longlong)in_RDX + 0xc) == '_');
    auVar3[8] = -(*(char *)((longlong)in_RDX + 0xd) == 'p');
    auVar3[9] = -(*(char *)((longlong)in_RDX + 0xe) == 'i');
    auVar3[10] = -(*(char *)((longlong)in_RDX + 0xf) == 'p');
    auVar3[0xb] = -((char)in_RDX[2] == 'e');
    auVar3[0xc] = -(*(char *)((longlong)in_RDX + 0x11) == 'l');
    auVar3[0xd] = -(*(char *)((longlong)in_RDX + 0x12) == 'i');
    auVar3[0xe] = -(*(char *)((longlong)in_RDX + 0x13) == 'n');
    auVar3[0xf] = -(*(char *)((longlong)in_RDX + 0x14) == 'e');
    auVar2[0] = -((char)*in_RDX == 'e');
    auVar2[1] = -(*(char *)((longlong)in_RDX + 1) == 'n');
    auVar2[2] = -(*(char *)((longlong)in_RDX + 2) == 'a');
    auVar2[3] = -(*(char *)((longlong)in_RDX + 3) == 'b');
    auVar2[4] = -(*(char *)((longlong)in_RDX + 4) == 'l');
    auVar2[5] = -(*(char *)((longlong)in_RDX + 5) == 'e');
    auVar2[6] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar2[7] = -(*(char *)((longlong)in_RDX + 7) == 'a');
    auVar2[8] = -((char)in_RDX[1] == 's');
    auVar2[9] = -(*(char *)((longlong)in_RDX + 9) == 's');
    auVar2[10] = -(*(char *)((longlong)in_RDX + 10) == 'e');
    auVar2[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar2[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == '_');
    auVar2[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'p');
    auVar2[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'i');
    auVar2[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'p');
    auVar2 = auVar2 & auVar3;
    if ((ushort)((ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar2[0xf] >> 7) << 0xf) != 0xffff) {
      in_RCX[1] = 8;
      *in_RCX = 0x32;
      return;
    }
    in_RCX[1] = 7;
    *in_RCX = 0x32;
    return;
  }
  in_RCX[1] = 8;
  *in_RCX = 0x32;
  return;
}

