
void __cdecl
country_core::game_config::_::impl$0::deserialize::impl$0::visit_str<enum2_<ron::error::Error>_>
          (void)

{
  int iVar1;
  undefined1 *in_RCX;
  longlong *in_RDX;
  size_t in_R8;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  
  switch(in_R8) {
  case 9:
    iVar1 = memcmp(in_RDX,&anon_40bca413fac50b2ca11692b4f63ba39c_57_llvm_8667993948624408409,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 8;
      goto code_r0x0001402e8259;
    }
    break;
  case 10:
    iVar1 = memcmp(in_RDX,&anon_40bca413fac50b2ca11692b4f63ba39c_56_llvm_8667993948624408409,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 7;
      goto code_r0x0001402e8259;
    }
    break;
  case 0xf:
    if (*(longlong *)((longlong)in_RDX + 7) == 0x785f747265766e69 && *in_RDX == 0x695f6172656d6163)
    {
      in_RCX[1] = 2;
      goto code_r0x0001402e8259;
    }
    if (*(longlong *)((longlong)in_RDX + 7) == 0x795f747265766e69 && *in_RDX == 0x695f6172656d6163)
    {
      in_RCX[1] = 3;
      goto code_r0x0001402e8259;
    }
    break;
  case 0x11:
    iVar1 = memcmp(in_RDX,&anon_40bca413fac50b2ca11692b4f63ba39c_54_llvm_8667993948624408409,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 5;
      goto code_r0x0001402e8259;
    }
    iVar1 = memcmp(in_RDX,&anon_40bca413fac50b2ca11692b4f63ba39c_58_llvm_8667993948624408409,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 9;
      goto code_r0x0001402e8259;
    }
    break;
  case 0x15:
    auVar6[0] = -(*(char *)((longlong)in_RDX + 5) == 'a');
    auVar6[1] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar6[2] = -(*(char *)((longlong)in_RDX + 7) == 'm');
    auVar6[3] = -((char)in_RDX[1] == 'o');
    auVar6[4] = -(*(char *)((longlong)in_RDX + 9) == 'v');
    auVar6[5] = -(*(char *)((longlong)in_RDX + 10) == 'e');
    auVar6[6] = -(*(char *)((longlong)in_RDX + 0xb) == 'm');
    auVar6[7] = -(*(char *)((longlong)in_RDX + 0xc) == 'e');
    auVar6[8] = -(*(char *)((longlong)in_RDX + 0xd) == 'n');
    auVar6[9] = -(*(char *)((longlong)in_RDX + 0xe) == 't');
    auVar6[10] = -(*(char *)((longlong)in_RDX + 0xf) == '_');
    auVar6[0xb] = -((char)in_RDX[2] == 's');
    auVar6[0xc] = -(*(char *)((longlong)in_RDX + 0x11) == 'p');
    auVar6[0xd] = -(*(char *)((longlong)in_RDX + 0x12) == 'e');
    auVar6[0xe] = -(*(char *)((longlong)in_RDX + 0x13) == 'e');
    auVar6[0xf] = -(*(char *)((longlong)in_RDX + 0x14) == 'd');
    auVar3[0] = -((char)*in_RDX == 'c');
    auVar3[1] = -(*(char *)((longlong)in_RDX + 1) == 'a');
    auVar3[2] = -(*(char *)((longlong)in_RDX + 2) == 'm');
    auVar3[3] = -(*(char *)((longlong)in_RDX + 3) == 'e');
    auVar3[4] = -(*(char *)((longlong)in_RDX + 4) == 'r');
    auVar3[5] = -(*(char *)((longlong)in_RDX + 5) == 'a');
    auVar3[6] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar3[7] = -(*(char *)((longlong)in_RDX + 7) == 'm');
    auVar3[8] = -((char)in_RDX[1] == 'o');
    auVar3[9] = -(*(char *)((longlong)in_RDX + 9) == 'v');
    auVar3[10] = -(*(char *)((longlong)in_RDX + 10) == 'e');
    auVar3[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'm');
    auVar3[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'e');
    auVar3[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'n');
    auVar3[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 't');
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
      goto code_r0x0001402e8259;
    }
    auVar7[0] = -(*(char *)((longlong)in_RDX + 5) == 'a');
    auVar7[1] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar7[2] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar7[3] = -((char)in_RDX[1] == 'o');
    auVar7[4] = -(*(char *)((longlong)in_RDX + 9) == 't');
    auVar7[5] = -(*(char *)((longlong)in_RDX + 10) == 'a');
    auVar7[6] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar7[7] = -(*(char *)((longlong)in_RDX + 0xc) == 'i');
    auVar7[8] = -(*(char *)((longlong)in_RDX + 0xd) == 'o');
    auVar7[9] = -(*(char *)((longlong)in_RDX + 0xe) == 'n');
    auVar7[10] = -(*(char *)((longlong)in_RDX + 0xf) == '_');
    auVar7[0xb] = -((char)in_RDX[2] == 'p');
    auVar7[0xc] = -(*(char *)((longlong)in_RDX + 0x11) == 'i');
    auVar7[0xd] = -(*(char *)((longlong)in_RDX + 0x12) == 'v');
    auVar7[0xe] = -(*(char *)((longlong)in_RDX + 0x13) == 'o');
    auVar7[0xf] = -(*(char *)((longlong)in_RDX + 0x14) == 't');
    auVar4[0] = -((char)*in_RDX == 'c');
    auVar4[1] = -(*(char *)((longlong)in_RDX + 1) == 'a');
    auVar4[2] = -(*(char *)((longlong)in_RDX + 2) == 'm');
    auVar4[3] = -(*(char *)((longlong)in_RDX + 3) == 'e');
    auVar4[4] = -(*(char *)((longlong)in_RDX + 4) == 'r');
    auVar4[5] = -(*(char *)((longlong)in_RDX + 5) == 'a');
    auVar4[6] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar4[7] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar4[8] = -((char)in_RDX[1] == 'o');
    auVar4[9] = -(*(char *)((longlong)in_RDX + 9) == 't');
    auVar4[10] = -(*(char *)((longlong)in_RDX + 10) == 'a');
    auVar4[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar4[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'i');
    auVar4[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'o');
    auVar4[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'n');
    auVar4[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == '_');
    auVar4 = auVar4 & auVar7;
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
      in_RCX[1] = 4;
      goto code_r0x0001402e8259;
    }
    break;
  case 0x18:
    iVar1 = memcmp(in_RDX,&anon_40bca413fac50b2ca11692b4f63ba39c_55_llvm_8667993948624408409,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 6;
      goto code_r0x0001402e8259;
    }
    break;
  case 0x19:
    iVar1 = memcmp(in_RDX,&anon_40bca413fac50b2ca11692b4f63ba39c_59_llvm_8667993948624408409,in_R8);
    if (iVar1 == 0) {
      in_RCX[1] = 10;
      goto code_r0x0001402e8259;
    }
    goto code_r0x0001402e8256;
  case 0x1b:
    auVar5[0] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar5[1] = -(*(char *)((longlong)in_RDX + 0xc) == 'i');
    auVar5[2] = -(*(char *)((longlong)in_RDX + 0xd) == 'o');
    auVar5[3] = -(*(char *)((longlong)in_RDX + 0xe) == 'n');
    auVar5[4] = -(*(char *)((longlong)in_RDX + 0xf) == '_');
    auVar5[5] = -((char)in_RDX[2] == 's');
    auVar5[6] = -(*(char *)((longlong)in_RDX + 0x11) == 'e');
    auVar5[7] = -(*(char *)((longlong)in_RDX + 0x12) == 'n');
    auVar5[8] = -(*(char *)((longlong)in_RDX + 0x13) == 's');
    auVar5[9] = -(*(char *)((longlong)in_RDX + 0x14) == 'i');
    auVar5[10] = -(*(char *)((longlong)in_RDX + 0x15) == 't');
    auVar5[0xb] = -(*(char *)((longlong)in_RDX + 0x16) == 'i');
    auVar5[0xc] = -(*(char *)((longlong)in_RDX + 0x17) == 'v');
    auVar5[0xd] = -((char)in_RDX[3] == 'i');
    auVar5[0xe] = -(*(char *)((longlong)in_RDX + 0x19) == 't');
    auVar5[0xf] = -(*(char *)((longlong)in_RDX + 0x1a) == 'y');
    auVar2[0] = -((char)*in_RDX == 'c');
    auVar2[1] = -(*(char *)((longlong)in_RDX + 1) == 'a');
    auVar2[2] = -(*(char *)((longlong)in_RDX + 2) == 'm');
    auVar2[3] = -(*(char *)((longlong)in_RDX + 3) == 'e');
    auVar2[4] = -(*(char *)((longlong)in_RDX + 4) == 'r');
    auVar2[5] = -(*(char *)((longlong)in_RDX + 5) == 'a');
    auVar2[6] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar2[7] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar2[8] = -((char)in_RDX[1] == 'o');
    auVar2[9] = -(*(char *)((longlong)in_RDX + 9) == 't');
    auVar2[10] = -(*(char *)((longlong)in_RDX + 10) == 'a');
    auVar2[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
    auVar2[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'i');
    auVar2[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'o');
    auVar2[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'n');
    auVar2[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == '_');
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
      in_RCX[1] = 0;
      goto code_r0x0001402e8259;
    }
  }
code_r0x0001402e8256:
  in_RCX[1] = 0xb;
code_r0x0001402e8259:
  *in_RCX = 0x32;
  return;
}

