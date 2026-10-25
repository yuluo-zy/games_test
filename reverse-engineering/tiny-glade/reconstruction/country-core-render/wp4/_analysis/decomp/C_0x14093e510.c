// ADDRESS: 0x14093e510
// SIZE: 2522
// SIGNATURE: void __cdecl deserialize_from(void)

void __cdecl country_core::asset_libraries::color_library::ColorLibrarySsbo::deserialize_from(void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *plVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *_Buf1;
  size_t _Size;
  int iVar7;
  undefined4 *in_RCX;
  undefined4 *puVar8;
  longlong *in_RDX;
  undefined4 *puVar9;
  longlong *plVar10;
  ulonglong uVar11;
  undefined4 *puVar12;
  undefined4 *puVar13;
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined4 local_548;
  undefined4 uStack_544;
  undefined4 uStack_540;
  
  lVar2 = *in_RDX;
  plVar10 = (longlong *)in_RDX[1];
  plVar1 = plVar10 + in_RDX[2] * 5;
  if (in_RDX[2] * 0x28 != 0) {
    plVar4 = plVar10;
    do {
      plVar10 = plVar4 + 5;
      lVar3 = *plVar4;
      lVar5 = plVar4[3];
      lVar6 = plVar4[4];
      _Buf1 = (longlong *)plVar4[1];
      _Size = plVar4[2];
      if (SBORROW8(0,lVar3)) break;
      switch(_Size) {
      case 4:
        puVar8 = in_RCX + 0x50;
        puVar9 = in_RCX + 0x51;
        puVar12 = in_RCX + 0x52;
        puVar13 = in_RCX + 0x53;
        if ((int)*_Buf1 != 0x68746170) goto LAB_14093f828;
        break;
      default:
        goto LAB_14093f828;
      case 6:
        puVar8 = in_RCX + 0x70;
        puVar9 = in_RCX + 0x71;
        puVar12 = in_RCX + 0x72;
        puVar13 = in_RCX + 0x73;
        if ((*(short *)((longlong)_Buf1 + 4) != 0x315f || (int)*_Buf1 != 0x6c6c6177) &&
           (puVar8 = in_RCX + 0x74, puVar9 = in_RCX + 0x75, puVar12 = in_RCX + 0x76,
           puVar13 = in_RCX + 0x77,
           *(short *)((longlong)_Buf1 + 4) != 0x325f || (int)*_Buf1 != 0x6c6c6177))
        goto LAB_14093f828;
        break;
      case 7:
        puVar8 = in_RCX + 0x48;
        puVar9 = in_RCX + 0x49;
        puVar12 = in_RCX + 0x4a;
        puVar13 = in_RCX + 0x4b;
        if ((*(int *)((longlong)_Buf1 + 3) != 0x315f7373 || (int)*_Buf1 != 0x73617267) &&
           (puVar8 = in_RCX + 0x4c, puVar9 = in_RCX + 0x4d, puVar12 = in_RCX + 0x4e,
           puVar13 = in_RCX + 0x4f,
           *(int *)((longlong)_Buf1 + 3) != 0x325f7373 || (int)*_Buf1 != 0x73617267))
        goto LAB_14093f828;
        break;
      case 8:
        puVar8 = in_RCX + 0x54;
        puVar9 = in_RCX + 0x55;
        puVar12 = in_RCX + 0x56;
        puVar13 = in_RCX + 0x57;
        if (*_Buf1 != 0x657361625f796b73) goto LAB_14093f828;
        break;
      case 9:
        iVar7 = memcmp(_Buf1,
                       "terrain_1terrain_2wall_mosswall_bricksterrain_highlightnight_tintInit Color Library failed:  entry is missing from ColorLibrarySSBO"
                       ,_Size);
        puVar8 = in_RCX + 100;
        puVar9 = in_RCX + 0x65;
        puVar12 = in_RCX + 0x66;
        puVar13 = in_RCX + 0x67;
        if ((iVar7 != 0) &&
           (iVar7 = memcmp(_Buf1,
                           "terrain_2wall_mosswall_bricksterrain_highlightnight_tintInit Color Library failed:  entry is missing from ColorLibrarySSBO"
                           ,_Size), puVar8 = in_RCX + 0x68, puVar9 = in_RCX + 0x69,
           puVar12 = in_RCX + 0x6a, puVar13 = in_RCX + 0x6b, iVar7 != 0)) {
          iVar7 = memcmp(_Buf1,
                         "wall_mosswall_bricksterrain_highlightnight_tintInit Color Library failed:  entry is missing from ColorLibrarySSBO"
                         ,_Size);
          puVar8 = in_RCX + 0x78;
          puVar9 = in_RCX + 0x79;
          puVar12 = in_RCX + 0x7a;
          puVar13 = in_RCX + 0x7b;
code_r0x00014093f705:
          if (iVar7 != 0) {
LAB_14093f828:
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
        }
        break;
      case 10:
        puVar8 = in_RCX + 0x80;
        puVar9 = in_RCX + 0x81;
        puVar12 = in_RCX + 0x82;
        puVar13 = in_RCX + 0x83;
        if ((((short)_Buf1[1] != 0x6863 || *_Buf1 != 0x6e6172625f797669) &&
            (puVar8 = in_RCX + 0x84, puVar9 = in_RCX + 0x85, puVar12 = in_RCX + 0x86,
            puVar13 = in_RCX + 0x87, (short)_Buf1[1] != 0x315f || *_Buf1 != 0x6661656c5f797669)) &&
           (puVar8 = in_RCX + 0x88, puVar9 = in_RCX + 0x89, puVar12 = in_RCX + 0x8a,
           puVar13 = in_RCX + 0x8b, (short)_Buf1[1] != 0x325f || *_Buf1 != 0x6661656c5f797669)) {
          iVar7 = memcmp(_Buf1,
                         "night_tintInit Color Library failed:  entry is missing from ColorLibrarySSBO"
                         ,_Size);
          puVar8 = in_RCX + 0x90;
          puVar9 = in_RCX + 0x91;
          puVar12 = in_RCX + 0x92;
          puVar13 = in_RCX + 0x93;
          goto code_r0x00014093f705;
        }
        break;
      case 0xb:
        puVar8 = in_RCX + 0x14;
        puVar9 = in_RCX + 0x15;
        puVar12 = in_RCX + 0x16;
        puVar13 = in_RCX + 0x17;
        if ((*(longlong *)((longlong)_Buf1 + 3) != 0x7961645f776f6c5f ||
             *_Buf1 != 0x5f776f6c5f796b73) &&
           (puVar8 = in_RCX + 0x58, puVar9 = in_RCX + 0x59, puVar12 = in_RCX + 0x5a,
           puVar13 = in_RCX + 0x5b,
           *(longlong *)((longlong)_Buf1 + 3) != 0x6e6f7a69726f685f || *_Buf1 != 0x69726f685f796b73)
           ) {
          iVar7 = memcmp(_Buf1,
                         "wall_bricksterrain_highlightnight_tintInit Color Library failed:  entry is missing from ColorLibrarySSBO"
                         ,_Size);
          puVar8 = in_RCX + 0x7c;
          puVar9 = in_RCX + 0x7d;
          puVar12 = in_RCX + 0x7e;
          puVar13 = in_RCX + 0x7f;
          goto code_r0x00014093f705;
        }
        break;
      case 0xc:
        puVar8 = in_RCX;
        puVar9 = in_RCX + 1;
        puVar12 = in_RCX + 2;
        puVar13 = in_RCX + 3;
        if ((((int)_Buf1[1] != 0x315f726f || *_Buf1 != 0x6c6f635f706d6574) &&
            (puVar8 = in_RCX + 4, puVar9 = in_RCX + 5, puVar12 = in_RCX + 6, puVar13 = in_RCX + 7,
            (int)_Buf1[1] != 0x325f726f || *_Buf1 != 0x6c6f635f706d6574)) &&
           ((puVar8 = in_RCX + 8, puVar9 = in_RCX + 9, puVar12 = in_RCX + 10, puVar13 = in_RCX + 0xb
            , (int)_Buf1[1] != 0x335f726f || *_Buf1 != 0x6c6f635f706d6574 &&
            ((((puVar8 = in_RCX + 0xc, puVar9 = in_RCX + 0xd, puVar12 = in_RCX + 0xe,
               puVar13 = in_RCX + 0xf, (int)_Buf1[1] != 0x315f726f || *_Buf1 != 0x6c6f635f666f6f72
               && (puVar8 = in_RCX + 0x10, puVar9 = in_RCX + 0x11, puVar12 = in_RCX + 0x12,
                  puVar13 = in_RCX + 0x13,
                  (int)_Buf1[1] != 0x325f726f || *_Buf1 != 0x6c6f635f666f6f72)) &&
              (puVar8 = in_RCX + 0x18, puVar9 = in_RCX + 0x19, puVar12 = in_RCX + 0x1a,
              puVar13 = in_RCX + 0x1b, (int)_Buf1[1] != 0x7961645f || *_Buf1 != 0x686769685f796b73))
             && (((puVar8 = in_RCX + 0x1c, puVar9 = in_RCX + 0x1d, puVar12 = in_RCX + 0x1e,
                  puVar13 = in_RCX + 0x1f,
                  (int)_Buf1[1] != 0x7961645f || *_Buf1 != 0x746e69745f6e7573 &&
                  (puVar8 = in_RCX + 0x20, puVar9 = in_RCX + 0x21, puVar12 = in_RCX + 0x22,
                  puVar13 = in_RCX + 0x23,
                  (int)_Buf1[1] != 0x6b737564 || *_Buf1 != 0x5f776f6c5f796b73)) &&
                 ((puVar8 = in_RCX + 0x5c, puVar9 = in_RCX + 0x5d, puVar12 = in_RCX + 0x5e,
                  puVar13 = in_RCX + 0x5f,
                  (int)_Buf1[1] != 0x315f7364 || *_Buf1 != 0x756f6c635f796b73 &&
                  (puVar8 = in_RCX + 0x60, puVar9 = in_RCX + 0x61, puVar12 = in_RCX + 0x62,
                  puVar13 = in_RCX + 99, (int)_Buf1[1] != 0x325f7364 || *_Buf1 != 0x756f6c635f796b73
                  )))))))))) goto LAB_14093f828;
        break;
      case 0xd:
        puVar8 = in_RCX + 0x24;
        puVar9 = in_RCX + 0x25;
        puVar12 = in_RCX + 0x26;
        puVar13 = in_RCX + 0x27;
        if (((*(longlong *)((longlong)_Buf1 + 5) != 0x6b7375645f686769 ||
              *_Buf1 != 0x686769685f796b73) &&
            (puVar8 = in_RCX + 0x28, puVar9 = in_RCX + 0x29, puVar12 = in_RCX + 0x2a,
            puVar13 = in_RCX + 0x2b,
            *(longlong *)((longlong)_Buf1 + 5) != 0x6b7375645f746e69 || *_Buf1 != 0x746e69745f6e7573
            )) && ((puVar8 = in_RCX + 0x30, puVar9 = in_RCX + 0x31, puVar12 = in_RCX + 0x32,
                   puVar13 = in_RCX + 0x33,
                   *(longlong *)((longlong)_Buf1 + 5) != 0x315f726f6c6f635f ||
                   *_Buf1 != 0x6f635f65636e6566 &&
                   (puVar8 = in_RCX + 0x34, puVar9 = in_RCX + 0x35, puVar12 = in_RCX + 0x36,
                   puVar13 = in_RCX + 0x37,
                   *(longlong *)((longlong)_Buf1 + 5) != 0x325f726f6c6f635f ||
                   *_Buf1 != 0x6f635f65636e6566)))) goto LAB_14093f828;
        break;
      case 0xe:
        puVar8 = in_RCX + 0x38;
        puVar9 = in_RCX + 0x39;
        puVar12 = in_RCX + 0x3a;
        puVar13 = in_RCX + 0x3b;
        if ((((*(longlong *)((longlong)_Buf1 + 6) != 0x315f726f6c6f635f ||
               *_Buf1 != 0x635f7265776f6c66) &&
             (puVar8 = in_RCX + 0x3c, puVar9 = in_RCX + 0x3d, puVar12 = in_RCX + 0x3e,
             puVar13 = in_RCX + 0x3f,
             *(longlong *)((longlong)_Buf1 + 6) != 0x325f726f6c6f635f ||
             *_Buf1 != 0x635f7265776f6c66)) &&
            (puVar8 = in_RCX + 0x40, puVar9 = in_RCX + 0x41, puVar12 = in_RCX + 0x42,
            puVar13 = in_RCX + 0x43,
            *(longlong *)((longlong)_Buf1 + 6) != 0x315f726f6c6f635f || *_Buf1 != 0x635f79706f6e6163
            )) && (puVar8 = in_RCX + 0x44, puVar9 = in_RCX + 0x45, puVar12 = in_RCX + 0x46,
                  puVar13 = in_RCX + 0x47,
                  *(longlong *)((longlong)_Buf1 + 6) != 0x325f726f6c6f635f ||
                  *_Buf1 != 0x635f79706f6e6163)) goto LAB_14093f828;
        break;
      case 0x11:
        auVar15[0] = -((char)*_Buf1 == 't');
        auVar15[1] = -(*(char *)((longlong)_Buf1 + 1) == 'e');
        auVar15[2] = -(*(char *)((longlong)_Buf1 + 2) == 'r');
        auVar15[3] = -(*(char *)((longlong)_Buf1 + 3) == 'r');
        auVar15[4] = -(*(char *)((longlong)_Buf1 + 4) == 'a');
        auVar15[5] = -(*(char *)((longlong)_Buf1 + 5) == 'i');
        auVar15[6] = -(*(char *)((longlong)_Buf1 + 6) == 'n');
        auVar15[7] = -(*(char *)((longlong)_Buf1 + 7) == '_');
        auVar15[8] = -((char)_Buf1[1] == 'f');
        auVar15[9] = -(*(char *)((longlong)_Buf1 + 9) == 'l');
        auVar15[10] = -(*(char *)((longlong)_Buf1 + 10) == 'a');
        auVar15[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 't');
        auVar15[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 't');
        auVar15[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'e');
        auVar15[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'n');
        auVar15[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'e');
        auVar17[0] = -((char)_Buf1[2] == 'd');
        auVar17[1] = 0xff;
        auVar17[2] = 0xff;
        auVar17[3] = 0xff;
        auVar17[4] = 0xff;
        auVar17[5] = 0xff;
        auVar17[6] = 0xff;
        auVar17[7] = 0xff;
        auVar17[8] = 0xff;
        auVar17[9] = 0xff;
        auVar17[10] = 0xff;
        auVar17[0xb] = 0xff;
        auVar17[0xc] = 0xff;
        auVar17[0xd] = 0xff;
        auVar17[0xe] = 0xff;
        auVar17[0xf] = 0xff;
        auVar17 = auVar17 & auVar15;
        puVar8 = in_RCX + 0x6c;
        puVar9 = in_RCX + 0x6d;
        puVar12 = in_RCX + 0x6e;
        puVar13 = in_RCX + 0x6f;
        if ((ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar17[0xf] >> 7) << 0xf) != 0xffff) {
          iVar7 = memcmp(_Buf1,
                         "terrain_highlightnight_tintInit Color Library failed:  entry is missing from ColorLibrarySSBO"
                         ,_Size);
          puVar8 = in_RCX + 0x8c;
          puVar9 = in_RCX + 0x8d;
          puVar12 = in_RCX + 0x8e;
          puVar13 = in_RCX + 0x8f;
          goto code_r0x00014093f705;
        }
        break;
      case 0x13:
        auVar14[0] = -((char)*_Buf1 == 'i');
        auVar14[1] = -(*(char *)((longlong)_Buf1 + 1) == 'n');
        auVar14[2] = -(*(char *)((longlong)_Buf1 + 2) == 'd');
        auVar14[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
        auVar14[4] = -(*(char *)((longlong)_Buf1 + 4) == 'r');
        auVar14[5] = -(*(char *)((longlong)_Buf1 + 5) == 'e');
        auVar14[6] = -(*(char *)((longlong)_Buf1 + 6) == 'c');
        auVar14[7] = -(*(char *)((longlong)_Buf1 + 7) == 't');
        auVar14[8] = -((char)_Buf1[1] == '_');
        auVar14[9] = -(*(char *)((longlong)_Buf1 + 9) == 'l');
        auVar14[10] = -(*(char *)((longlong)_Buf1 + 10) == 'i');
        auVar14[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'g');
        auVar14[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'h');
        auVar14[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 't');
        auVar14[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == '_');
        auVar14[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 't');
        auVar16[0] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
        auVar16[1] = -(*(char *)((longlong)_Buf1 + 4) == 'r');
        auVar16[2] = -(*(char *)((longlong)_Buf1 + 5) == 'e');
        auVar16[3] = -(*(char *)((longlong)_Buf1 + 6) == 'c');
        auVar16[4] = -(*(char *)((longlong)_Buf1 + 7) == 't');
        auVar16[5] = -((char)_Buf1[1] == '_');
        auVar16[6] = -(*(char *)((longlong)_Buf1 + 9) == 'l');
        auVar16[7] = -(*(char *)((longlong)_Buf1 + 10) == 'i');
        auVar16[8] = -(*(char *)((longlong)_Buf1 + 0xb) == 'g');
        auVar16[9] = -(*(char *)((longlong)_Buf1 + 0xc) == 'h');
        auVar16[10] = -(*(char *)((longlong)_Buf1 + 0xd) == 't');
        auVar16[0xb] = -(*(char *)((longlong)_Buf1 + 0xe) == '_');
        auVar16[0xc] = -(*(char *)((longlong)_Buf1 + 0xf) == 't');
        auVar16[0xd] = -((char)_Buf1[2] == 'i');
        auVar16[0xe] = -(*(char *)((longlong)_Buf1 + 0x11) == 'n');
        auVar16[0xf] = -(*(char *)((longlong)_Buf1 + 0x12) == 't');
        auVar16 = auVar16 & auVar14;
        puVar8 = in_RCX + 0x2c;
        puVar9 = in_RCX + 0x2d;
        puVar12 = in_RCX + 0x2e;
        puVar13 = in_RCX + 0x2f;
        if ((ushort)((ushort)(SUB161(auVar16 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar16 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar16 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar16 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar16 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar16 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar16 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar16 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar16 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar16 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar16 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar16 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar16 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar16[0xf] >> 7) << 0xf) != 0xffff) goto LAB_14093f828;
      }
      local_548 = (undefined4)lVar5;
      uStack_544 = (undefined4)((ulonglong)lVar5 >> 0x20);
      uStack_540 = (undefined4)lVar6;
      *puVar8 = local_548;
      *puVar9 = uStack_544;
      *puVar12 = uStack_540;
      *puVar13 = 0x3f800000;
      if (lVar3 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar4 = plVar10;
    } while (plVar10 != plVar1);
  }
  if (plVar1 != plVar10) {
    uVar11 = (ulonglong)((longlong)plVar1 - (longlong)plVar10) / 0x28;
    plVar10 = plVar10 + 1;
    do {
      if (plVar10[-1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      plVar10 = plVar10 + 5;
      uVar11 = uVar11 - 1;
    } while (uVar11 != 0);
  }
  if (lVar2 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  }
  return;
}

