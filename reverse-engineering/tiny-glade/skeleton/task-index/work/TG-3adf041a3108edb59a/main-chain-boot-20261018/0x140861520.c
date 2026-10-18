
void __cdecl
country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::
visit_str<serde_json::error::Error>(void)

{
  int iVar1;
  undefined1 *in_RCX;
  longlong *in_RDX;
  size_t in_R8;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  
  switch(in_R8) {
  case 5:
    if (*(char *)((longlong)in_RDX + 4) == 'e' && (int)*in_RDX == 0x76727563) {
      in_RCX[1] = 0;
      goto code_r0x00014086174c;
    }
    if (*(char *)((longlong)in_RDX + 4) == 'e' && (int)*in_RDX == 0x6c797473)
    goto code_r0x0001408616ed;
    break;
  case 6:
    if (*(short *)((longlong)in_RDX + 4) == 0x7468 && (int)*in_RDX == 0x67696568) {
      in_RCX[1] = 1;
      goto code_r0x00014086174c;
    }
    break;
  case 7:
    if (*(int *)((longlong)in_RDX + 3) == 0x79745f68 && (int)*in_RDX == 0x68637261) {
      in_RCX[1] = 5;
      goto code_r0x00014086174c;
    }
    break;
  case 9:
    if ((char)in_RDX[1] == 'n' && *in_RDX == 0x6f69746176656c65) {
      in_RCX[1] = 3;
      goto code_r0x00014086174c;
    }
    break;
  case 10:
    if ((short)in_RDX[1] == 0x7374 && *in_RDX == 0x72696b735f736168) {
code_r0x0001408616ed:
      in_RCX[1] = 6;
      goto code_r0x00014086174c;
    }
    break;
  case 0xb:
    if (*(longlong *)((longlong)in_RDX + 3) == 0x6465736f6c636e65 && *in_RDX == 0x6f6c636e655f7369)
    {
code_r0x00014086171f:
      in_RCX[1] = 2;
      goto code_r0x00014086174c;
    }
    if (*(longlong *)((longlong)in_RDX + 3) == 0x7468676965685f68 && *in_RDX == 0x6965685f68637261)
    {
      in_RCX[1] = 4;
      goto code_r0x00014086174c;
    }
    iVar1 = memcmp(in_RDX,&anon_fd26506bd31d5e4909a9375e3935cf05_33_llvm_1874562925060143905,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 9;
      goto code_r0x00014086174c;
    }
    goto code_r0x000140861678;
  case 0xc:
    if ((int)in_RDX[1] == 0x79745f65 && *in_RDX == 0x7275746375727473) {
      in_RCX[1] = 7;
      goto code_r0x00014086174c;
    }
    iVar1 = memcmp(in_RDX,&anon_fd26506bd31d5e4909a9375e3935cf05_80_llvm_1874562925060143905,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 10;
      goto code_r0x00014086174c;
    }
code_r0x000140861678:
    if (in_R8 != 0x10) break;
code_r0x000140861688:
    auVar2[0] = -((char)*in_RDX == 'r');
    auVar2[1] = -(*(char *)((longlong)in_RDX + 1) == 'e');
    auVar2[2] = -(*(char *)((longlong)in_RDX + 2) == 'm');
    auVar2[3] = -(*(char *)((longlong)in_RDX + 3) == 'o');
    auVar2[4] = -(*(char *)((longlong)in_RDX + 4) == 'v');
    auVar2[5] = -(*(char *)((longlong)in_RDX + 5) == 'e');
    auVar2[6] = -(*(char *)((longlong)in_RDX + 6) == 'd');
    auVar2[7] = -(*(char *)((longlong)in_RDX + 7) == '_');
    auVar2[8] = -((char)in_RDX[1] == 's');
    auVar2[9] = -(*(char *)((longlong)in_RDX + 9) == 'u');
    auVar2[10] = -(*(char *)((longlong)in_RDX + 10) == 'p');
    auVar2[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'p');
    auVar2[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar2[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'r');
    auVar2[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 't');
    auVar2[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 's');
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
                 (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe | (ushort)(auVar2[0xf] >> 7) << 0xf)
        == 0xffff) {
      in_RCX[1] = 0xb;
      goto code_r0x00014086174c;
    }
    goto code_r0x000140861749;
  case 0xf:
    if (*(longlong *)((longlong)in_RDX + 7) == 0x6e6f69737265765f && *in_RDX == 0x5f72657474756c63)
    {
      in_RCX[1] = 8;
      goto code_r0x00014086174c;
    }
    break;
  case 0x10:
    goto code_r0x000140861688;
  case 0x12:
    auVar3[0] = -((char)*in_RDX == 'i');
    auVar3[1] = -(*(char *)((longlong)in_RDX + 1) == 's');
    auVar3[2] = -(*(char *)((longlong)in_RDX + 2) == '_');
    auVar3[3] = -(*(char *)((longlong)in_RDX + 3) == 'v');
    auVar3[4] = -(*(char *)((longlong)in_RDX + 4) == 'a');
    auVar3[5] = -(*(char *)((longlong)in_RDX + 5) == 'l');
    auVar3[6] = -(*(char *)((longlong)in_RDX + 6) == 'i');
    auVar3[7] = -(*(char *)((longlong)in_RDX + 7) == 'd');
    auVar3[8] = -((char)in_RDX[1] == '_');
    auVar3[9] = -(*(char *)((longlong)in_RDX + 9) == 'e');
    auVar3[10] = -(*(char *)((longlong)in_RDX + 10) == 'n');
    auVar3[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'c');
    auVar3[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'l');
    auVar3[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'o');
    auVar3[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 's');
    auVar3[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'u');
    auVar4[0] = -((char)(short)in_RDX[2] == 'r');
    auVar4[1] = -((char)((ushort)(short)in_RDX[2] >> 8) == 'e');
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
    auVar4 = auVar4 & auVar3;
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
                (ushort)(byte)(auVar4[0xf] >> 7) << 0xf) == 0xffff) goto code_r0x00014086171f;
  }
code_r0x000140861749:
  in_RCX[1] = 0xc;
code_r0x00014086174c:
  *in_RCX = 0;
  return;
}

