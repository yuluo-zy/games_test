// address 0x141572870
// name visit_str<enum2$<ron::error::Error>_>
// signature void __cdecl visit_str<enum2$<ron::error::Error>_>(void)

void __cdecl
country_core::render::nani::gpu_world::_::impl$0::deserialize::impl$0::
visit_str<enum2_<ron::error::Error>_>(void)

{
  int iVar1;
  undefined2 *in_RCX;
  longlong *in_RDX;
  size_t in_R8;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  
  switch(in_R8) {
  case 4:
    if ((int)*in_RDX == 0x68737542) {
      *in_RCX = 0xc32;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_104_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1132;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_110_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1732;
      return;
    }
    break;
  case 5:
    if (*(char *)((longlong)in_RDX + 4) == 't' && (int)*in_RDX == 0x6e616c50) {
      *in_RCX = 0xa32;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_107_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1432;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_108_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1532;
      return;
    }
    break;
  case 6:
    if (*(short *)((longlong)in_RDX + 4) == 0x7970 && (int)*in_RDX == 0x6f6e6143) {
      *in_RCX = 0x32;
      return;
    }
    if (*(short *)((longlong)in_RDX + 4) == 0x7265 && (int)*in_RDX == 0x776f6c46) {
      *in_RCX = 0xb32;
      return;
    }
    break;
  case 7:
    goto code_r0x000141572c63;
  case 8:
    if (*in_RDX == 0x656c6954666f6f52) {
      *in_RCX = 0x832;
      return;
    }
    break;
  case 9:
    if ((char)in_RDX[1] == 'k' && *in_RDX == 0x6e75725465657254) {
      *in_RCX = 0x132;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_96_llvm_14765964051527200705,in_R8)
    ;
    if (iVar1 == 0) {
      *in_RCX = 0x932;
      return;
    }
    break;
  case 0xb:
    if (*(longlong *)((longlong)in_RDX + 3) == 0x6b6e616c506e6564 && *in_RDX == 0x6c506e65646f6f57)
    {
      *in_RCX = 0xd32;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_109_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1632;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_113_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1a32;
      return;
    }
    break;
  case 0xc:
    goto code_r0x000141572c83;
  case 0xe:
    if (*(longlong *)((longlong)in_RDX + 6) == 0x65657254646e756f && *in_RDX == 0x756f72676b636142)
    {
      *in_RCX = 0x232;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_102_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0xf32;
      return;
    }
    goto code_r0x000141572c4e;
  case 0xf:
    if (*(longlong *)((longlong)in_RDX + 7) == 0x7265776f6c466465 && *in_RDX == 0x6563616c70657250)
    {
      *in_RCX = 0xe32;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_105_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1232;
      return;
    }
    goto code_r0x000141572c4e;
  case 0x10:
    auVar2[0] = -((char)*in_RDX == 'S');
    auVar2[1] = -(*(char *)((longlong)in_RDX + 1) == 'o');
    auVar2[2] = -(*(char *)((longlong)in_RDX + 2) == 'l');
    auVar2[3] = -(*(char *)((longlong)in_RDX + 3) == 'i');
    auVar2[4] = -(*(char *)((longlong)in_RDX + 4) == 'd');
    auVar2[5] = -(*(char *)((longlong)in_RDX + 5) == 'V');
    auVar2[6] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar2[7] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar2[8] = -((char)in_RDX[1] == 't');
    auVar2[9] = -(*(char *)((longlong)in_RDX + 9) == 'e');
    auVar2[10] = -(*(char *)((longlong)in_RDX + 10) == 'x');
    auVar2[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'C');
    auVar2[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar2[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar2[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
    auVar2[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'r');
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
      *in_RCX = 0x332;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_111_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1832;
      return;
    }
    if (in_R8 != 0xc) break;
    goto code_r0x000141572c83;
  case 0x12:
    auVar4[0] = -((char)*in_RDX == 'G');
    auVar4[1] = -(*(char *)((longlong)in_RDX + 1) == 'o');
    auVar4[2] = -(*(char *)((longlong)in_RDX + 2) == 't');
    auVar4[3] = -(*(char *)((longlong)in_RDX + 3) == 'h');
    auVar4[4] = -(*(char *)((longlong)in_RDX + 4) == 'i');
    auVar4[5] = -(*(char *)((longlong)in_RDX + 5) == 'c');
    auVar4[6] = -(*(char *)((longlong)in_RDX + 6) == 'W');
    auVar4[7] = -(*(char *)((longlong)in_RDX + 7) == 'i');
    auVar4[8] = -((char)in_RDX[1] == 'n');
    auVar4[9] = -(*(char *)((longlong)in_RDX + 9) == 'd');
    auVar4[10] = -(*(char *)((longlong)in_RDX + 10) == 'o');
    auVar4[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'w');
    auVar4[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'B');
    auVar4[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'r');
    auVar4[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'i');
    auVar4[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'c');
    auVar8[0] = -((char)(short)in_RDX[2] == 'k');
    auVar8[1] = -((char)((ushort)(short)in_RDX[2] >> 8) == 's');
    auVar8[2] = 0xff;
    auVar8[3] = 0xff;
    auVar8[4] = 0xff;
    auVar8[5] = 0xff;
    auVar8[6] = 0xff;
    auVar8[7] = 0xff;
    auVar8[8] = 0xff;
    auVar8[9] = 0xff;
    auVar8[10] = 0xff;
    auVar8[0xb] = 0xff;
    auVar8[0xc] = 0xff;
    auVar8[0xd] = 0xff;
    auVar8[0xe] = 0xff;
    auVar8[0xf] = 0xff;
    auVar8 = auVar8 & auVar4;
    if ((ushort)((ushort)(SUB161(auVar8 >> 7,0) & 1) | (ushort)(SUB161(auVar8 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar8 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar8 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar8 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar8 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar8 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar8 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar8 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar8 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar8 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar8 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar8 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar8 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar8 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar8[0xf] >> 7) << 0xf) == 0xffff) {
      *in_RCX = 0x732;
      return;
    }
    break;
  case 0x16:
    auVar6[0] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar6[1] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar6[2] = -((char)in_RDX[1] == 't');
    auVar6[3] = -(*(char *)((longlong)in_RDX + 9) == 'e');
    auVar6[4] = -(*(char *)((longlong)in_RDX + 10) == 'x');
    auVar6[5] = -(*(char *)((longlong)in_RDX + 0xb) == 'C');
    auVar6[6] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar6[7] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar6[8] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
    auVar6[9] = -(*(char *)((longlong)in_RDX + 0xf) == 'r');
    auVar6[10] = -((char)in_RDX[2] == 'W');
    auVar6[0xb] = -(*(char *)((longlong)in_RDX + 0x11) == 'i');
    auVar6[0xc] = -(*(char *)((longlong)in_RDX + 0x12) == 'n');
    auVar6[0xd] = -(*(char *)((longlong)in_RDX + 0x13) == 'd');
    auVar6[0xe] = -(*(char *)((longlong)in_RDX + 0x14) == 'o');
    auVar6[0xf] = -(*(char *)((longlong)in_RDX + 0x15) == 'w');
    auVar3[0] = -((char)*in_RDX == 'S');
    auVar3[1] = -(*(char *)((longlong)in_RDX + 1) == 'o');
    auVar3[2] = -(*(char *)((longlong)in_RDX + 2) == 'l');
    auVar3[3] = -(*(char *)((longlong)in_RDX + 3) == 'i');
    auVar3[4] = -(*(char *)((longlong)in_RDX + 4) == 'd');
    auVar3[5] = -(*(char *)((longlong)in_RDX + 5) == 'V');
    auVar3[6] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar3[7] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar3[8] = -((char)in_RDX[1] == 't');
    auVar3[9] = -(*(char *)((longlong)in_RDX + 9) == 'e');
    auVar3[10] = -(*(char *)((longlong)in_RDX + 10) == 'x');
    auVar3[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'C');
    auVar3[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar3[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar3[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
    auVar3[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'r');
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
      *in_RCX = 0x432;
      return;
    }
    auVar10[0] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar10[1] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar10[2] = -((char)in_RDX[1] == 't');
    auVar10[3] = -(*(char *)((longlong)in_RDX + 9) == 'e');
    auVar10[4] = -(*(char *)((longlong)in_RDX + 10) == 'x');
    auVar10[5] = -(*(char *)((longlong)in_RDX + 0xb) == 'C');
    auVar10[6] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar10[7] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar10[8] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
    auVar10[9] = -(*(char *)((longlong)in_RDX + 0xf) == 'r');
    auVar10[10] = -((char)in_RDX[2] == 'B');
    auVar10[0xb] = -(*(char *)((longlong)in_RDX + 0x11) == 'r');
    auVar10[0xc] = -(*(char *)((longlong)in_RDX + 0x12) == 'i');
    auVar10[0xd] = -(*(char *)((longlong)in_RDX + 0x13) == 'c');
    auVar10[0xe] = -(*(char *)((longlong)in_RDX + 0x14) == 'k');
    auVar10[0xf] = -(*(char *)((longlong)in_RDX + 0x15) == 's');
    auVar7[0] = -((char)*in_RDX == 'S');
    auVar7[1] = -(*(char *)((longlong)in_RDX + 1) == 'o');
    auVar7[2] = -(*(char *)((longlong)in_RDX + 2) == 'l');
    auVar7[3] = -(*(char *)((longlong)in_RDX + 3) == 'i');
    auVar7[4] = -(*(char *)((longlong)in_RDX + 4) == 'd');
    auVar7[5] = -(*(char *)((longlong)in_RDX + 5) == 'V');
    auVar7[6] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar7[7] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar7[8] = -((char)in_RDX[1] == 't');
    auVar7[9] = -(*(char *)((longlong)in_RDX + 9) == 'e');
    auVar7[10] = -(*(char *)((longlong)in_RDX + 10) == 'x');
    auVar7[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'C');
    auVar7[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar7[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar7[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
    auVar7[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'r');
    auVar10 = auVar10 & auVar7;
    if ((ushort)((ushort)(SUB161(auVar10 >> 7,0) & 1) | (ushort)(SUB161(auVar10 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar10 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar10 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar10 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar10 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar10 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar10 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar10 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar10 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar10 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar10 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar10 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar10 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar10 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar10[0xf] >> 7) << 0xf) == 0xffff) {
      *in_RCX = 0x632;
      return;
    }
    iVar1 = memcmp(in_RDX,&anon_1c1a639a7b8c984c3b37b13664f64d34_103_llvm_14765964051527200705,in_R8
                  );
    if (iVar1 == 0) {
      *in_RCX = 0x1032;
      return;
    }
code_r0x000141572c4e:
    if (in_R8 == 0xc) {
code_r0x000141572c83:
      if ((int)in_RDX[1] == 0x796c6e4f && *in_RDX == 0x797469636f6c6556) {
        *in_RCX = 0x1932;
        return;
      }
    }
    else if (in_R8 == 7) {
code_r0x000141572c63:
      if (*(int *)((longlong)in_RDX + 3) == 0x72657473 && (int)*in_RDX == 0x73616c50) {
        *in_RCX = 0x1332;
        return;
      }
    }
    break;
  case 0x1b:
    auVar9[0] = -(*(char *)((longlong)in_RDX + 0xb) == 'C');
    auVar9[1] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar9[2] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar9[3] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
    auVar9[4] = -(*(char *)((longlong)in_RDX + 0xf) == 'r');
    auVar9[5] = -((char)in_RDX[2] == 'W');
    auVar9[6] = -(*(char *)((longlong)in_RDX + 0x11) == 'i');
    auVar9[7] = -(*(char *)((longlong)in_RDX + 0x12) == 'n');
    auVar9[8] = -(*(char *)((longlong)in_RDX + 0x13) == 'd');
    auVar9[9] = -(*(char *)((longlong)in_RDX + 0x14) == 'o');
    auVar9[10] = -(*(char *)((longlong)in_RDX + 0x15) == 'w');
    auVar9[0xb] = -(*(char *)((longlong)in_RDX + 0x16) == 'G');
    auVar9[0xc] = -(*(char *)((longlong)in_RDX + 0x17) == 'l');
    auVar9[0xd] = -((char)in_RDX[3] == 'a');
    auVar9[0xe] = -(*(char *)((longlong)in_RDX + 0x19) == 's');
    auVar9[0xf] = -(*(char *)((longlong)in_RDX + 0x1a) == 's');
    auVar5[0] = -((char)*in_RDX == 'S');
    auVar5[1] = -(*(char *)((longlong)in_RDX + 1) == 'o');
    auVar5[2] = -(*(char *)((longlong)in_RDX + 2) == 'l');
    auVar5[3] = -(*(char *)((longlong)in_RDX + 3) == 'i');
    auVar5[4] = -(*(char *)((longlong)in_RDX + 4) == 'd');
    auVar5[5] = -(*(char *)((longlong)in_RDX + 5) == 'V');
    auVar5[6] = -(*(char *)((longlong)in_RDX + 6) == 'e');
    auVar5[7] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar5[8] = -((char)in_RDX[1] == 't');
    auVar5[9] = -(*(char *)((longlong)in_RDX + 9) == 'e');
    auVar5[10] = -(*(char *)((longlong)in_RDX + 10) == 'x');
    auVar5[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'C');
    auVar5[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'o');
    auVar5[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'l');
    auVar5[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
    auVar5[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'r');
    auVar5 = auVar5 & auVar9;
    if ((ushort)((ushort)(SUB161(auVar5 >> 7,0) & 1) | (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                (ushort)(byte)(auVar5[0xf] >> 7) << 0xf) == 0xffff) {
      *in_RCX = 0x532;
      return;
    }
  }
  ron::error::impl$5::unknown_variant();
  return;
}

