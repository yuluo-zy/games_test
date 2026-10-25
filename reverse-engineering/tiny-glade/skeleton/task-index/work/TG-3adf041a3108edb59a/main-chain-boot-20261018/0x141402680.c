
void __cdecl
country_core::resources::history::wall_compound_edit::_::impl$0::deserialize::impl$2::visit_enum::
impl$3::visit_str<serde_json::error::Error>(void)

{
  undefined1 *in_RCX;
  longlong *in_RDX;
  undefined8 in_R8;
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  
  switch(in_R8) {
  case 5:
    if (*(char *)((longlong)in_RDX + 4) == 'r' && (int)*in_RDX == 0x65746661) {
code_r0x00014140270c:
      in_RCX[1] = 3;
      *in_RCX = 0;
      return;
    }
    break;
  case 8:
    if (*in_RDX != 0x64695f726f6c6f63) {
      in_RCX[1] = 5;
      *in_RCX = 0;
      return;
    }
    in_RCX[1] = 4;
    *in_RCX = 0;
    return;
  case 9:
    if ((char)in_RDX[1] == 'n' && *in_RDX == 0x6f69746176656c65) {
      in_RCX[1] = 2;
      *in_RCX = 0;
      return;
    }
    break;
  case 10:
    if ((short)in_RDX[1] == 0x7865 && *in_RDX == 0x646e695f6c6c6177) {
      in_RCX[1] = 0;
      *in_RCX = 0;
      return;
    }
    break;
  case 0xb:
    if (*(longlong *)((longlong)in_RDX + 3) == 0x7468676965685f6c && *in_RDX == 0x6965685f6c6c6177)
    {
      in_RCX[1] = 1;
      *in_RCX = 0;
      return;
    }
    break;
  case 0x12:
    auVar1[0] = -((char)*in_RDX == 'r');
    auVar1[1] = -(*(char *)((longlong)in_RDX + 1) == 'e');
    auVar1[2] = -(*(char *)((longlong)in_RDX + 2) == 'd');
    auVar1[3] = -(*(char *)((longlong)in_RDX + 3) == 'o');
    auVar1[4] = -(*(char *)((longlong)in_RDX + 4) == '_');
    auVar1[5] = -(*(char *)((longlong)in_RDX + 5) == 'i');
    auVar1[6] = -(*(char *)((longlong)in_RDX + 6) == 'n');
    auVar1[7] = -(*(char *)((longlong)in_RDX + 7) == 's');
    auVar1[8] = -((char)in_RDX[1] == 't');
    auVar1[9] = -(*(char *)((longlong)in_RDX + 9) == 'a');
    auVar1[10] = -(*(char *)((longlong)in_RDX + 10) == 'n');
    auVar1[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar1[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == '_');
    auVar1[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 's');
    auVar1[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 't');
    auVar1[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'a');
    auVar2[0] = -((char)(short)in_RDX[2] == 't');
    auVar2[1] = -((char)((ushort)(short)in_RDX[2] >> 8) == 'e');
    auVar2[2] = 0xff;
    auVar2[3] = 0xff;
    auVar2[4] = 0xff;
    auVar2[5] = 0xff;
    auVar2[6] = 0xff;
    auVar2[7] = 0xff;
    auVar2[8] = 0xff;
    auVar2[9] = 0xff;
    auVar2[10] = 0xff;
    auVar2[0xb] = 0xff;
    auVar2[0xc] = 0xff;
    auVar2[0xd] = 0xff;
    auVar2[0xe] = 0xff;
    auVar2[0xf] = 0xff;
    auVar2 = auVar2 & auVar1;
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
                (ushort)(byte)(auVar2[0xf] >> 7) << 0xf) == 0xffff) goto code_r0x00014140270c;
  }
  in_RCX[1] = 5;
  *in_RCX = 0;
  return;
}

