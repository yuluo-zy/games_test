
void __cdecl
country_core::render::render_config::_::impl$0::deserialize::impl$0::
visit_str<enum2_<ron::error::Error>_>(void)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  int iVar5;
  undefined1 *in_RCX;
  longlong *in_RDX;
  size_t in_R8;
  bool bVar6;
  bool bVar7;
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  
  if (in_R8 == 0x17) {
    auVar16[0] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar16[1] = -((char)in_RDX[1] == 'e');
    auVar16[2] = -(*(char *)((longlong)in_RDX + 9) == 's');
    auVar16[3] = -(*(char *)((longlong)in_RDX + 10) == 'o');
    auVar16[4] = -(*(char *)((longlong)in_RDX + 0xb) == 'l');
    auVar16[5] = -(*(char *)((longlong)in_RDX + 0xc) == 'u');
    auVar16[6] = -(*(char *)((longlong)in_RDX + 0xd) == 't');
    auVar16[7] = -(*(char *)((longlong)in_RDX + 0xe) == 'i');
    auVar16[8] = -(*(char *)((longlong)in_RDX + 0xf) == 'o');
    auVar16[9] = -((char)in_RDX[2] == 'n');
    auVar16[10] = -(*(char *)((longlong)in_RDX + 0x11) == '_');
    auVar16[0xb] = -(*(char *)((longlong)in_RDX + 0x12) == 's');
    auVar16[0xc] = -(*(char *)((longlong)in_RDX + 0x13) == 'c');
    auVar16[0xd] = -(*(char *)((longlong)in_RDX + 0x14) == 'a');
    auVar16[0xe] = -(*(char *)((longlong)in_RDX + 0x15) == 'l');
    auVar16[0xf] = -(*(char *)((longlong)in_RDX + 0x16) == 'e');
    auVar8[0] = -((char)*in_RDX == 'l');
    auVar8[1] = -(*(char *)((longlong)in_RDX + 1) == 'e');
    auVar8[2] = -(*(char *)((longlong)in_RDX + 2) == 'g');
    auVar8[3] = -(*(char *)((longlong)in_RDX + 3) == 'a');
    auVar8[4] = -(*(char *)((longlong)in_RDX + 4) == 'c');
    auVar8[5] = -(*(char *)((longlong)in_RDX + 5) == 'y');
    auVar8[6] = -(*(char *)((longlong)in_RDX + 6) == '_');
    auVar8[7] = -(*(char *)((longlong)in_RDX + 7) == 'r');
    auVar8[8] = -((char)in_RDX[1] == 'e');
    auVar8[9] = -(*(char *)((longlong)in_RDX + 9) == 's');
    auVar8[10] = -(*(char *)((longlong)in_RDX + 10) == 'o');
    auVar8[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'l');
    auVar8[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'u');
    auVar8[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 't');
    auVar8[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'i');
    auVar8[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'o');
    auVar8 = auVar8 & auVar16;
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
LAB_14003b0ec:
      in_RCX[1] = 2;
      goto LAB_14003b576;
    }
  }
  else if (in_R8 == 7) {
    if (*(int *)((longlong)in_RDX + 3) == 0x6e6f6973 && (int)*in_RDX == 0x73726576) {
      in_RCX[1] = 0;
      goto LAB_14003b576;
    }
    if (*(int *)((longlong)in_RDX + 3) == 0x79616c70 && (int)*in_RDX == 0x70736964) {
      in_RCX[1] = 1;
      goto LAB_14003b576;
    }
    if (*(int *)((longlong)in_RDX + 3) == 0x6e657072 && (int)*in_RDX == 0x72616873) {
      in_RCX[1] = 10;
      goto LAB_14003b576;
    }
  }
  else {
    bVar1 = in_R8 != 0x10;
    if (in_R8 == 0x10) {
      auVar10[0] = -((char)*in_RDX == 'r');
      auVar10[1] = -(*(char *)((longlong)in_RDX + 1) == 'e');
      auVar10[2] = -(*(char *)((longlong)in_RDX + 2) == 's');
      auVar10[3] = -(*(char *)((longlong)in_RDX + 3) == 'o');
      auVar10[4] = -(*(char *)((longlong)in_RDX + 4) == 'l');
      auVar10[5] = -(*(char *)((longlong)in_RDX + 5) == 'u');
      auVar10[6] = -(*(char *)((longlong)in_RDX + 6) == 't');
      auVar10[7] = -(*(char *)((longlong)in_RDX + 7) == 'i');
      auVar10[8] = -((char)in_RDX[1] == 'o');
      auVar10[9] = -(*(char *)((longlong)in_RDX + 9) == 'n');
      auVar10[10] = -(*(char *)((longlong)in_RDX + 10) == '_');
      auVar10[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 's');
      auVar10[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'c');
      auVar10[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'a');
      auVar10[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'l');
      auVar10[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'e');
      if ((ushort)((ushort)(SUB161(auVar10 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar10 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar10 >> 0x17,0) & 1) << 2 |
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
                  (ushort)(auVar10[0xf] >> 7) << 0xf) == 0xffff) goto LAB_14003b0ec;
LAB_14003b07f:
      bVar6 = in_R8 == 0x10;
joined_r0x00014003b1ae:
      if (in_R8 == 0xd) {
        if (*(longlong *)((longlong)in_RDX + 5) == 0x676e696c706d6173 &&
            *in_RDX == 0x6d61737265707573) {
LAB_14003b253:
          in_RCX[1] = 7;
          goto LAB_14003b576;
        }
        bVar2 = true;
        bVar7 = false;
        bVar3 = false;
        bVar4 = false;
        if (!bVar6) {
LAB_14003b3b0:
          iVar5 = memcmp(in_RDX,&anon_8b72ae6db50a5d17841e0e1559951e3c_51_llvm_1670512067297649760,
                         in_R8);
          if (iVar5 == 0) {
            in_RCX[1] = 0x10;
            goto LAB_14003b576;
          }
          goto LAB_14003b3de;
        }
LAB_14003b1ec:
        iVar5 = memcmp(in_RDX,&anon_8b72ae6db50a5d17841e0e1559951e3c_43_llvm_1670512067297649760,
                       in_R8);
        if (iVar5 == 0) {
          in_RCX[1] = 8;
          goto LAB_14003b576;
        }
        if ((in_R8 == 9) && ((char)in_RDX[1] == 't' && *in_RDX == 0x696d696c5f737066)) {
LAB_14003b2a7:
          in_RCX[1] = 9;
          goto LAB_14003b576;
        }
        iVar5 = memcmp(in_RDX,&anon_8b72ae6db50a5d17841e0e1559951e3c_46_llvm_1670512067297649760,
                       in_R8);
        bVar3 = true;
        if (iVar5 == 0) {
          in_RCX[1] = 0xb;
          goto LAB_14003b576;
        }
LAB_14003b2be:
        if (in_R8 == 0x1b) {
          auVar19[0] = -(*(char *)((longlong)in_RDX + 0xb) == 'g');
          auVar19[1] = -(*(char *)((longlong)in_RDX + 0xc) == 'r');
          auVar19[2] = -(*(char *)((longlong)in_RDX + 0xd) == 'a');
          auVar19[3] = -(*(char *)((longlong)in_RDX + 0xe) == 'p');
          auVar19[4] = -(*(char *)((longlong)in_RDX + 0xf) == 'h');
          auVar19[5] = -((char)in_RDX[2] == 'i');
          auVar19[6] = -(*(char *)((longlong)in_RDX + 0x11) == 'c');
          auVar19[7] = -(*(char *)((longlong)in_RDX + 0x12) == 's');
          auVar19[8] = -(*(char *)((longlong)in_RDX + 0x13) == '_');
          auVar19[9] = -(*(char *)((longlong)in_RDX + 0x14) == 'd');
          auVar19[10] = -(*(char *)((longlong)in_RDX + 0x15) == 'e');
          auVar19[0xb] = -(*(char *)((longlong)in_RDX + 0x16) == 'v');
          auVar19[0xc] = -(*(char *)((longlong)in_RDX + 0x17) == 'i');
          auVar19[0xd] = -((char)in_RDX[3] == 'c');
          auVar19[0xe] = -(*(char *)((longlong)in_RDX + 0x19) == 'e');
          auVar19[0xf] = -(*(char *)((longlong)in_RDX + 0x1a) == 's');
          auVar12[0] = -((char)*in_RDX == 'l');
          auVar12[1] = -(*(char *)((longlong)in_RDX + 1) == 'a');
          auVar12[2] = -(*(char *)((longlong)in_RDX + 2) == 's');
          auVar12[3] = -(*(char *)((longlong)in_RDX + 3) == 't');
          auVar12[4] = -(*(char *)((longlong)in_RDX + 4) == '_');
          auVar12[5] = -(*(char *)((longlong)in_RDX + 5) == 'k');
          auVar12[6] = -(*(char *)((longlong)in_RDX + 6) == 'n');
          auVar12[7] = -(*(char *)((longlong)in_RDX + 7) == 'o');
          auVar12[8] = -((char)in_RDX[1] == 'w');
          auVar12[9] = -(*(char *)((longlong)in_RDX + 9) == 'n');
          auVar12[10] = -(*(char *)((longlong)in_RDX + 10) == '_');
          auVar12[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'g');
          auVar12[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'r');
          auVar12[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'a');
          auVar12[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'p');
          auVar12[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'h');
          auVar12 = auVar12 & auVar19;
          if ((ushort)((ushort)(SUB161(auVar12 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar12 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar12 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar12 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar12 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar12 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar12 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar12 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar12 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar12 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar12 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar12 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar12 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar12 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar12 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar12[0xf] >> 7) << 0xf) == 0xffff) {
            in_RCX[1] = 0xd;
            goto LAB_14003b576;
          }
LAB_14003b33f:
          bVar4 = false;
          if (bVar7) goto LAB_14003b346;
          goto LAB_14003b374;
        }
        if (in_R8 != 0xb) goto LAB_14003b33f;
        if (*(longlong *)((longlong)in_RDX + 3) == 0x74666968735f616d &&
            *in_RDX == 0x68735f616d6d6167) {
          in_RCX[1] = 0xc;
          goto LAB_14003b576;
        }
        bVar4 = true;
        if (bVar7) goto LAB_14003b346;
        bVar4 = true;
        if (bVar2) {
          bVar7 = false;
          goto LAB_14003b3b0;
        }
        if (bVar3) goto LAB_14003b453;
        bVar4 = true;
        if (bVar1) goto LAB_14003b4bc;
      }
      else {
        if (in_R8 == 0xe) {
          if (*(longlong *)((longlong)in_RDX + 6) == 0x65646f6d5f676e69 &&
              *in_RDX == 0x6e696c6163737075) goto LAB_14003b253;
          if (bVar6) goto LAB_14003b145;
          if (!bVar1) goto LAB_14003b001;
LAB_14003b297:
          bVar4 = false;
LAB_14003b4bc:
          iVar5 = memcmp(in_RDX,&anon_8b72ae6db50a5d17841e0e1559951e3c_56_llvm_1670512067297649760,
                         in_R8);
          if ((iVar5 == 0) ||
             (iVar5 = memcmp(in_RDX,
                             "vulkan_layer_whitelistUpscalingPresetUltraPerformancePerformanceBalancedQualityUltraQuality"
                             ,in_R8), iVar5 == 0)) {
            in_RCX[1] = 0x15;
            goto LAB_14003b576;
          }
        }
        else {
          if (in_R8 != 0x14) {
            if (bVar6) {
LAB_14003b145:
              bVar2 = false;
              bVar7 = false;
              goto LAB_14003b1ec;
            }
            if (in_R8 == 9) {
              if ((char)in_RDX[1] != 't' || *in_RDX != 0x696d696c5f737066) {
                if (bVar1) goto LAB_14003b297;
                goto LAB_14003b001;
              }
              goto LAB_14003b2a7;
            }
            bVar3 = false;
            bVar7 = false;
            bVar2 = false;
            goto LAB_14003b2be;
          }
          iVar5 = (int)in_RDX[2];
          auVar18[0] = -((char)iVar5 == 'a');
          auVar18[1] = -((char)((uint)iVar5 >> 8) == 'b');
          auVar18[2] = -((char)((uint)iVar5 >> 0x10) == 'l');
          auVar18[3] = -((char)((uint)iVar5 >> 0x18) == 'e');
          auVar18[4] = 0xff;
          auVar18[5] = 0xff;
          auVar18[6] = 0xff;
          auVar18[7] = 0xff;
          auVar18[8] = 0xff;
          auVar18[9] = 0xff;
          auVar18[10] = 0xff;
          auVar18[0xb] = 0xff;
          auVar18[0xc] = 0xff;
          auVar18[0xd] = 0xff;
          auVar18[0xe] = 0xff;
          auVar18[0xf] = 0xff;
          auVar11[0] = -((char)*in_RDX == 'p');
          auVar11[1] = -(*(char *)((longlong)in_RDX + 1) == 'h');
          auVar11[2] = -(*(char *)((longlong)in_RDX + 2) == 'o');
          auVar11[3] = -(*(char *)((longlong)in_RDX + 3) == 't');
          auVar11[4] = -(*(char *)((longlong)in_RDX + 4) == 'o');
          auVar11[5] = -(*(char *)((longlong)in_RDX + 5) == 'm');
          auVar11[6] = -(*(char *)((longlong)in_RDX + 6) == 'o');
          auVar11[7] = -(*(char *)((longlong)in_RDX + 7) == 'd');
          auVar11[8] = -((char)in_RDX[1] == 'e');
          auVar11[9] = -(*(char *)((longlong)in_RDX + 9) == '_');
          auVar11[10] = -(*(char *)((longlong)in_RDX + 10) == 'd');
          auVar11[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'o');
          auVar11[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'f');
          auVar11[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == '_');
          auVar11[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'e');
          auVar11[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'n');
          auVar11 = auVar11 & auVar18;
          if ((ushort)((ushort)(SUB161(auVar11 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar11 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar11 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar11 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar11 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar11 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar11 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar11 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar11 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar11 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar11 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar11 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar11 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar11 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar11 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar11[0xf] >> 7) << 0xf) == 0xffff) {
            in_RCX[1] = 6;
            goto LAB_14003b576;
          }
          bVar7 = true;
          bVar2 = false;
          bVar3 = false;
          bVar4 = false;
          if (bVar6) goto LAB_14003b1ec;
LAB_14003b346:
          iVar5 = memcmp(in_RDX,&anon_8b72ae6db50a5d17841e0e1559951e3c_49_llvm_1670512067297649760,
                         in_R8);
          if (iVar5 == 0) {
            in_RCX[1] = 0xe;
            goto LAB_14003b576;
          }
LAB_14003b374:
          bVar7 = in_R8 == 0x13;
          if ((bVar7) &&
             (auVar20[0] = -(*(char *)((longlong)in_RDX + 3) == 'p'),
             auVar20[1] = -(*(char *)((longlong)in_RDX + 4) == 'h'),
             auVar20[2] = -(*(char *)((longlong)in_RDX + 5) == 'i'),
             auVar20[3] = -(*(char *)((longlong)in_RDX + 6) == 'c'),
             auVar20[4] = -(*(char *)((longlong)in_RDX + 7) == 's'),
             auVar20[5] = -((char)in_RDX[1] == '_'),
             auVar20[6] = -(*(char *)((longlong)in_RDX + 9) == 'v'),
             auVar20[7] = -(*(char *)((longlong)in_RDX + 10) == 'a'),
             auVar20[8] = -(*(char *)((longlong)in_RDX + 0xb) == 'l'),
             auVar20[9] = -(*(char *)((longlong)in_RDX + 0xc) == 'i'),
             auVar20[10] = -(*(char *)((longlong)in_RDX + 0xd) == 'd'),
             auVar20[0xb] = -(*(char *)((longlong)in_RDX + 0xe) == 'a'),
             auVar20[0xc] = -(*(char *)((longlong)in_RDX + 0xf) == 't'),
             auVar20[0xd] = -((char)in_RDX[2] == 'i'),
             auVar20[0xe] = -(*(char *)((longlong)in_RDX + 0x11) == 'o'),
             auVar20[0xf] = -(*(char *)((longlong)in_RDX + 0x12) == 'n'),
             auVar13[0] = -((char)*in_RDX == 'g'),
             auVar13[1] = -(*(char *)((longlong)in_RDX + 1) == 'r'),
             auVar13[2] = -(*(char *)((longlong)in_RDX + 2) == 'a'),
             auVar13[3] = -(*(char *)((longlong)in_RDX + 3) == 'p'),
             auVar13[4] = -(*(char *)((longlong)in_RDX + 4) == 'h'),
             auVar13[5] = -(*(char *)((longlong)in_RDX + 5) == 'i'),
             auVar13[6] = -(*(char *)((longlong)in_RDX + 6) == 'c'),
             auVar13[7] = -(*(char *)((longlong)in_RDX + 7) == 's'),
             auVar13[8] = -((char)in_RDX[1] == '_'),
             auVar13[9] = -(*(char *)((longlong)in_RDX + 9) == 'v'),
             auVar13[10] = -(*(char *)((longlong)in_RDX + 10) == 'a'),
             auVar13[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'l'),
             auVar13[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'i'),
             auVar13[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'd'),
             auVar13[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'a'),
             auVar13[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 't'), auVar13 = auVar13 & auVar20
             , (ushort)((ushort)(SUB161(auVar13 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar13 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar13 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar13 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar13 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar13 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar13 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar13 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar13 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar13 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar13 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar13 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar13 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar13 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar13 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar13[0xf] >> 7) << 0xf) == 0xffff)) {
            in_RCX[1] = 0xf;
            goto LAB_14003b576;
          }
          if (bVar2) goto LAB_14003b3b0;
LAB_14003b3de:
          if (in_R8 == 0x19) {
            auVar21[0] = -(*(char *)((longlong)in_RDX + 9) == 'm');
            auVar21[1] = -(*(char *)((longlong)in_RDX + 10) == 'a');
            auVar21[2] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
            auVar21[3] = -(*(char *)((longlong)in_RDX + 0xc) == 'h');
            auVar21[4] = -(*(char *)((longlong)in_RDX + 0xd) == '_');
            auVar21[5] = -(*(char *)((longlong)in_RDX + 0xe) == 'c');
            auVar21[6] = -(*(char *)((longlong)in_RDX + 0xf) == 'h');
            auVar21[7] = -((char)in_RDX[2] == 'e');
            auVar21[8] = -(*(char *)((longlong)in_RDX + 0x11) == 'c');
            auVar21[9] = -(*(char *)((longlong)in_RDX + 0x12) == 'k');
            auVar21[10] = -(*(char *)((longlong)in_RDX + 0x13) == 'p');
            auVar21[0xb] = -(*(char *)((longlong)in_RDX + 0x14) == 'o');
            auVar21[0xc] = -(*(char *)((longlong)in_RDX + 0x15) == 'i');
            auVar21[0xd] = -(*(char *)((longlong)in_RDX + 0x16) == 'n');
            auVar21[0xe] = -(*(char *)((longlong)in_RDX + 0x17) == 't');
            auVar21[0xf] = -((char)in_RDX[3] == 's');
            auVar14[0] = -((char)*in_RDX == 'u');
            auVar14[1] = -(*(char *)((longlong)in_RDX + 1) == 's');
            auVar14[2] = -(*(char *)((longlong)in_RDX + 2) == 'e');
            auVar14[3] = -(*(char *)((longlong)in_RDX + 3) == '_');
            auVar14[4] = -(*(char *)((longlong)in_RDX + 4) == 'a');
            auVar14[5] = -(*(char *)((longlong)in_RDX + 5) == 'f');
            auVar14[6] = -(*(char *)((longlong)in_RDX + 6) == 't');
            auVar14[7] = -(*(char *)((longlong)in_RDX + 7) == 'e');
            auVar14[8] = -((char)in_RDX[1] == 'r');
            auVar14[9] = -(*(char *)((longlong)in_RDX + 9) == 'm');
            auVar14[10] = -(*(char *)((longlong)in_RDX + 10) == 'a');
            auVar14[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 't');
            auVar14[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'h');
            auVar14[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == '_');
            auVar14[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'c');
            auVar14[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'h');
            auVar14 = auVar14 & auVar21;
            if ((ushort)((ushort)(SUB161(auVar14 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar14 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar14 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar14 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar14 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar14 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar14 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar14 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar14 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar14 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar14 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar14 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar14 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar14 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar14 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar14[0xf] >> 7) << 0xf) == 0xffff) {
              in_RCX[1] = 0x11;
              goto LAB_14003b576;
            }
            if (bVar3) {
LAB_14003b453:
              iVar5 = memcmp(in_RDX,&
                                    anon_8b72ae6db50a5d17841e0e1559951e3c_54_llvm_1670512067297649760
                             ,in_R8);
              if (iVar5 == 0) {
                in_RCX[1] = 0x13;
                goto LAB_14003b576;
              }
              goto LAB_14003b481;
            }
          }
          else {
            if ((bVar7) &&
               (iVar5 = memcmp(in_RDX,&
                                      anon_8b72ae6db50a5d17841e0e1559951e3c_53_llvm_1670512067297649760
                               ,in_R8), iVar5 == 0)) {
              in_RCX[1] = 0x12;
              goto LAB_14003b576;
            }
            if (bVar3) goto LAB_14003b453;
LAB_14003b481:
            if ((in_R8 == 0x12) &&
               (auVar15[0] = -((char)*in_RDX == 'r'),
               auVar15[1] = -(*(char *)((longlong)in_RDX + 1) == 'e'),
               auVar15[2] = -(*(char *)((longlong)in_RDX + 2) == 'n'),
               auVar15[3] = -(*(char *)((longlong)in_RDX + 3) == 'd'),
               auVar15[4] = -(*(char *)((longlong)in_RDX + 4) == 'e'),
               auVar15[5] = -(*(char *)((longlong)in_RDX + 5) == 'r'),
               auVar15[6] = -(*(char *)((longlong)in_RDX + 6) == '_'),
               auVar15[7] = -(*(char *)((longlong)in_RDX + 7) == 'u'),
               auVar15[8] = -((char)in_RDX[1] == 'p'),
               auVar15[9] = -(*(char *)((longlong)in_RDX + 9) == 's'),
               auVar15[10] = -(*(char *)((longlong)in_RDX + 10) == 'i'),
               auVar15[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'd'),
               auVar15[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 'e'),
               auVar15[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == '_'),
               auVar15[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'd'),
               auVar15[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'o'),
               auVar22[0] = -((char)(short)in_RDX[2] == 'w'),
               auVar22[1] = -((char)((ushort)(short)in_RDX[2] >> 8) == 'n'), auVar22[2] = 0xff,
               auVar22[3] = 0xff, auVar22[4] = 0xff, auVar22[5] = 0xff, auVar22[6] = 0xff,
               auVar22[7] = 0xff, auVar22[8] = 0xff, auVar22[9] = 0xff, auVar22[10] = 0xff,
               auVar22[0xb] = 0xff, auVar22[0xc] = 0xff, auVar22[0xd] = 0xff, auVar22[0xe] = 0xff,
               auVar22[0xf] = 0xff, auVar22 = auVar22 & auVar15,
               (ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar22[0xf] >> 7) << 0xf) == 0xffff)) {
              in_RCX[1] = 0x14;
              goto LAB_14003b576;
            }
          }
          if (bVar1) goto LAB_14003b4bc;
        }
        if (!bVar4) goto LAB_14003b001;
      }
      iVar5 = memcmp(in_RDX,"workarounds",in_R8);
      if (iVar5 == 0) {
        in_RCX[1] = 0x16;
      }
      else {
        in_RCX[1] = 0x17;
      }
      goto LAB_14003b576;
    }
    if (in_R8 == 10) {
      iVar5 = memcmp(in_RDX,&anon_8b72ae6db50a5d17841e0e1559951e3c_40_llvm_1670512067297649760,10);
      if (iVar5 == 0) {
        in_RCX[1] = 5;
        goto LAB_14003b576;
      }
    }
    else {
      if (in_R8 != 0xc) {
        if (in_R8 == 0x16) {
          auVar17[0] = -(*(char *)((longlong)in_RDX + 6) == 'r');
          auVar17[1] = -(*(char *)((longlong)in_RDX + 7) == 'e');
          auVar17[2] = -((char)in_RDX[1] == 's');
          auVar17[3] = -(*(char *)((longlong)in_RDX + 9) == 'o');
          auVar17[4] = -(*(char *)((longlong)in_RDX + 10) == 'l');
          auVar17[5] = -(*(char *)((longlong)in_RDX + 0xb) == 'u');
          auVar17[6] = -(*(char *)((longlong)in_RDX + 0xc) == 't');
          auVar17[7] = -(*(char *)((longlong)in_RDX + 0xd) == 'i');
          auVar17[8] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
          auVar17[9] = -(*(char *)((longlong)in_RDX + 0xf) == 'n');
          auVar17[10] = -((char)in_RDX[2] == '_');
          auVar17[0xb] = -(*(char *)((longlong)in_RDX + 0x11) == 's');
          auVar17[0xc] = -(*(char *)((longlong)in_RDX + 0x12) == 'c');
          auVar17[0xd] = -(*(char *)((longlong)in_RDX + 0x13) == 'a');
          auVar17[0xe] = -(*(char *)((longlong)in_RDX + 0x14) == 'l');
          auVar17[0xf] = -(*(char *)((longlong)in_RDX + 0x15) == 'e');
          auVar9[0] = -((char)*in_RDX == 'w');
          auVar9[1] = -(*(char *)((longlong)in_RDX + 1) == 'o');
          auVar9[2] = -(*(char *)((longlong)in_RDX + 2) == 'r');
          auVar9[3] = -(*(char *)((longlong)in_RDX + 3) == 'l');
          auVar9[4] = -(*(char *)((longlong)in_RDX + 4) == 'd');
          auVar9[5] = -(*(char *)((longlong)in_RDX + 5) == '_');
          auVar9[6] = -(*(char *)((longlong)in_RDX + 6) == 'r');
          auVar9[7] = -(*(char *)((longlong)in_RDX + 7) == 'e');
          auVar9[8] = -((char)in_RDX[1] == 's');
          auVar9[9] = -(*(char *)((longlong)in_RDX + 9) == 'o');
          auVar9[10] = -(*(char *)((longlong)in_RDX + 10) == 'l');
          auVar9[0xb] = -(*(char *)((longlong)in_RDX + 0xb) == 'u');
          auVar9[0xc] = -(*(char *)((longlong)in_RDX + 0xc) == 't');
          auVar9[0xd] = -(*(char *)((longlong)in_RDX + 0xd) == 'i');
          auVar9[0xe] = -(*(char *)((longlong)in_RDX + 0xe) == 'o');
          auVar9[0xf] = -(*(char *)((longlong)in_RDX + 0xf) == 'n');
          auVar9 = auVar9 & auVar17;
          if ((ushort)((ushort)(SUB161(auVar9 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar9 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar9 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar9 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar9 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar9 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar9 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar9 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar9 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar9 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar9 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar9 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar9 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar9 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar9 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar9[0xf] >> 7) << 0xf) == 0xffff) {
            in_RCX[1] = 3;
            goto LAB_14003b576;
          }
          goto LAB_14003b07f;
        }
        bVar1 = false;
        bVar6 = false;
        goto joined_r0x00014003b1ae;
      }
      iVar5 = memcmp(in_RDX,&anon_8b72ae6db50a5d17841e0e1559951e3c_39_llvm_1670512067297649760,0xc);
      if (iVar5 == 0) {
        in_RCX[1] = 4;
        goto LAB_14003b576;
      }
    }
  }
LAB_14003b001:
  in_RCX[1] = 0x17;
LAB_14003b576:
  *in_RCX = 0x32;
  return;
}

