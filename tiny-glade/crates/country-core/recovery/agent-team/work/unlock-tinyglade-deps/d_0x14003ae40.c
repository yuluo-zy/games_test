
void __cdecl
country_core::render::render_config::_::impl$0::deserialize::impl$0::
visit_str<enum2_<ron::error::Error>_>(void)

{
  undefined1 *in_RCX;
  longlong *in_RDX;
  undefined8 in_R8;
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  
  switch(in_R8) {
  case 7:
    if (*(int *)((longlong)in_RDX + 3) != 0x726f7469 || (int)*in_RDX != 0x696e6f6d) {
      in_RCX[1] = 5;
      *in_RCX = 0x32;
      return;
    }
    in_RCX[1] = 4;
    *in_RCX = 0x32;
    return;
  case 10:
    if ((short)in_RDX[1] == 0x6e65 && *in_RDX == 0x657263736c6c7566) {
      in_RCX[1] = 0;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0x11:
    auVar1[0] = -((char)*in_RDX == 'f');
    auVar1[1] = -(*(char *)((longlong)in_RDX + 1) == 'u');
    auVar1[2] = -(*(char *)((longlong)in_RDX + 2) == 'l');
    auVar1[3] = -(*(char *)((longlong)in_RDX + 3) == 'l');
    auVar1[4] = -(*(char *)((longlong)in_RDX + 4) == 's');
    auVar1[5] = -(*(char *)((longlong)in_RDX + 5) == 'c');
    auVar1[6] = -(*(char *)((longlong)in_RDX + 6) == 'r');
    auVar1[7] = -(*(char *)((longlong)in_RDX + 7) == 'e');
    auVar1[8] = -((char)in_RDX[1] == 'e');
    auVar1[9] = -(*(char *)((longlong)in_RDX + 9) == 'n');
    auVar1[10] = -(*(char *)((longlong)in_RDX + 10) == '_');
    auVar1[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'c');
    auVar1[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar1[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'n');
    auVar1[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'f');
    auVar1[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'i');
    auVar4[0] = -((char)in_RDX[2] == 'g');
    auVar4[1] = 0xff;
    auVar4[2] = 0xff;
    auVar4[3] = 0xff;
    auVar4[4] = 0xff;
    auVar4[5] = 0xff;
    auVar4[6] = 0xff;
    auVar4[7] = 0xff;
    auVar4[8] = 0xff;
    auVar4[9] = 0xff;
    auVar4[10] = 0xff;
    auVar4[0xb] = 0xff;
    auVar4[0xc] = 0xff;
    auVar4[0xd] = 0xff;
    auVar4[0xe] = 0xff;
    auVar4[0xf] = 0xff;
    auVar4 = auVar4 & auVar1;
    if ((ushort)((ushort)(SUB161(auVar4 >> 7,0) & 1) | (ushort)(SUB161(auVar4 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar4 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar4 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar4 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar4 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar4 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar4 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar4 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar4 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar4 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar4 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar4 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar4 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar4 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar4[0xf] >> 7) << 0xf) == 0xffff) {
      in_RCX[1] = 2;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0x13:
    auVar5[0] = -(*(char *)((longlong)in_RDX + 3) == 'd');
    auVar5[1] = -(*(char *)((longlong)in_RDX + 4) == 'o');
    auVar5[2] = -(*(char *)((longlong)in_RDX + 5) == 'w');
    auVar5[3] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar5[4] = -(*(char *)((longlong)in_RDX + 7) == 'd');
    auVar5[5] = -((char)in_RDX[1] == '_');
    auVar5[6] = -(*(char *)((longlong)in_RDX + 9) == 'r');
    auVar5[7] = -(*(char *)((longlong)in_RDX + 10) == 'e');
    auVar5[8] = -(*(char *)((longlong)in_RDX + 0xb) == 's');
    auVar5[9] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar5[10] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar5[0xb] = -(*(char *)((longlong)in_RDX + 0xe) == 'u');
    auVar5[0xc] = -(*(char *)((longlong)in_RDX + 0xf) == 't');
    auVar5[0xd] = -((char)in_RDX[2] == 'i');
    auVar5[0xe] = -(*(char *)((longlong)in_RDX + 0x11) == 'o');
    auVar5[0xf] = -(*(char *)((longlong)in_RDX + 0x12) == 'n');
    auVar2[0] = -((char)*in_RDX == 'w');
    auVar2[1] = -(*(char *)((longlong)in_RDX + 1) == 'i');
    auVar2[2] = -(*(char *)((longlong)in_RDX + 2) == 'n');
    auVar2[3] = -(*(char *)((longlong)in_RDX + 3) == 'd');
    auVar2[4] = -(*(char *)((longlong)in_RDX + 4) == 'o');
    auVar2[5] = -(*(char *)((longlong)in_RDX + 5) == 'w');
    auVar2[6] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar2[7] = -(*(char *)((longlong)in_RDX + 7) == 'd');
    auVar2[8] = -((char)in_RDX[1] == '_');
    auVar2[9] = -(*(char *)((longlong)in_RDX + 9) == 'r');
    auVar2[10] = -(*(char *)((longlong)in_RDX + 10) == 'e');
    auVar2[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 's');
    auVar2[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar2[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar2[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'u');
    auVar2[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 't');
    auVar2 = auVar2 & auVar5;
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
                (ushort)(byte)(auVar2[0xf] >> 7) << 0xf) == 0xffff) {
      in_RCX[1] = 3;
      *in_RCX = 0x32;
      return;
    }
    break;
  case 0x1a:
    auVar6[0] = -(*(char *)((longlong)in_RDX + 10) == 'u');
    auVar6[1] = -(*(char *)((longlong)in_RDX + 0xb) == 's');
    auVar6[2] = -(*(char *)((longlong)in_RDX + 0xc) == 'i');
    auVar6[3] = -(*(char *)((longlong)in_RDX + 0xd) == 'v');
    auVar6[4] = -(*(char *)((longlong)in_RDX + 0xe) == 'e');
    auVar6[5] = -(*(char *)((longlong)in_RDX + 0xf) == '_');
    auVar6[6] = -((char)in_RDX[2] == 'f');
    auVar6[7] = -(*(char *)((longlong)in_RDX + 0x11) == 'u');
    auVar6[8] = -(*(char *)((longlong)in_RDX + 0x12) == 'l');
    auVar6[9] = -(*(char *)((longlong)in_RDX + 0x13) == 'l');
    auVar6[10] = -(*(char *)((longlong)in_RDX + 0x14) == 's');
    auVar6[0xb] = -(*(char *)((longlong)in_RDX + 0x15) == 'c');
    auVar6[0xc] = -(*(char *)((longlong)in_RDX + 0x16) == 'r');
    auVar6[0xd] = -(*(char *)((longlong)in_RDX + 0x17) == 'e');
    auVar6[0xe] = -((char)in_RDX[3] == 'e');
    auVar6[0xf] = -(*(char *)((longlong)in_RDX + 0x19) == 'n');
    auVar3[0] = -((char)*in_RDX == 'a');
    auVar3[1] = -(*(char *)((longlong)in_RDX + 1) == 'l');
    auVar3[2] = -(*(char *)((longlong)in_RDX + 2) == 'l');
    auVar3[3] = -(*(char *)((longlong)in_RDX + 3) == 'o');
    auVar3[4] = -(*(char *)((longlong)in_RDX + 4) == 'w');
    auVar3[5] = -(*(char *)((longlong)in_RDX + 5) == '_');
    auVar3[6] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar3[7] = -(*(char *)((longlong)in_RDX + 7) == 'x');
    auVar3[8] = -((char)in_RDX[1] == 'c');
    auVar3[9] = -(*(char *)((longlong)in_RDX + 9) == 'l');
    auVar3[10] = -(*(char *)((longlong)in_RDX + 10) == 'u');
    auVar3[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 's');
    auVar3[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'i');
    auVar3[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'v');
    auVar3[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'e');
    auVar3[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == '_');
    auVar3 = auVar3 & auVar6;
    if ((ushort)((ushort)(SUB161(auVar3 >> 7,0) & 1) | (ushort)(SUB161(auVar3 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar3 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar3 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar3 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar3 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar3 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar3 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar3 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar3 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar3 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar3 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar3 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar3 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar3 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar3[0xf] >> 7) << 0xf) == 0xffff) {
      in_RCX[1] = 1;
      *in_RCX = 0x32;
      return;
    }
  }
  in_RCX[1] = 5;
  *in_RCX = 0x32;
  return;
}

