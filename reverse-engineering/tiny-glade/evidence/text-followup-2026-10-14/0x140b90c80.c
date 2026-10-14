
void FUN_140b90c80(longlong param_1,char param_2)

{
  undefined8 *puVar1;
  float fVar2;
  float fVar3;
  longlong lVar4;
  undefined1 auVar5 [16];
  longlong *plVar6;
  code *pcVar7;
  uint uVar8;
  uint uVar9;
  ulonglong uVar10;
  longlong lVar11;
  uint uVar12;
  ushort uVar13;
  ulonglong uVar14;
  byte bVar15;
  char cVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  longlong *plStack_58;
  undefined8 uStack_50;
  
  uStack_50 = 0xfffffffffffffffe;
  if (*(char *)(param_1 + 0x255) == param_2) {
    return;
  }
  plVar6 = *(longlong **)(param_1 + 0x230);
  LOCK();
  lVar4 = *plVar6;
  *plVar6 = *plVar6 + 1;
  UNLOCK();
  if (*plVar6 != 0 && SCARRY8(lVar4,1) == *plVar6 < 0) {
    uStack_70 = CONCAT71(uStack_70._1_7_,param_2);
    plStack_58 = plVar6;
    if (plVar6[5] != 0) {
      uVar10 = FUN_1405b1790(plVar6 + 6,&uStack_70);
      lVar4 = plVar6[2];
      bVar15 = (byte)(uVar10 >> 0x39);
      auVar17 = ZEXT216(CONCAT11(bVar15,bVar15));
      auVar17 = pshuflw(auVar17,auVar17,0);
      lVar11 = 0;
      while( true ) {
        uVar10 = plVar6[3] & uVar10;
        auVar5 = *(undefined1 (*) [16])(lVar4 + uVar10);
        cVar16 = auVar17[0];
        auVar22[0] = -(auVar5[0] == cVar16);
        cVar18 = auVar17[1];
        auVar22[1] = -(auVar5[1] == cVar18);
        cVar19 = auVar17[2];
        auVar22[2] = -(auVar5[2] == cVar19);
        cVar20 = auVar17[3];
        auVar22[3] = -(auVar5[3] == cVar20);
        auVar22[4] = -(auVar5[4] == cVar16);
        auVar22[5] = -(auVar5[5] == cVar18);
        auVar22[6] = -(auVar5[6] == cVar19);
        auVar22[7] = -(auVar5[7] == cVar20);
        auVar22[8] = -(auVar5[8] == cVar16);
        auVar22[9] = -(auVar5[9] == cVar18);
        auVar22[10] = -(auVar5[10] == cVar19);
        auVar22[0xb] = -(auVar5[0xb] == cVar20);
        auVar22[0xc] = -(auVar5[0xc] == cVar16);
        auVar22[0xd] = -(auVar5[0xd] == cVar18);
        auVar22[0xe] = -(auVar5[0xe] == cVar19);
        auVar22[0xf] = -(auVar5[0xf] == cVar20);
        uVar13 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(auVar22[0xf] >> 7) << 0xf
        ;
        uVar12 = (uint)uVar13;
        while (uVar13 != 0) {
          uVar8 = 0;
          for (uVar9 = uVar12; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            uVar8 = uVar8 + 1;
          }
          uVar14 = uVar8 + uVar10 & plVar6[3];
          if (*(char *)(lVar4 + -0x28 + uVar14 * -0x28) == param_2) {
            puVar1 = (undefined8 *)(lVar4 + -0x18 + uVar14 * -0x28);
            uStack_68 = *puVar1;
            uStack_60 = puVar1[1];
            fVar2 = *(float *)(lVar4 + -8 + uVar14 * -0x28);
            fVar3 = *(float *)(param_1 + 0x240);
            uStack_70 = 0;
            FUN_141804770(&uStack_88,&uStack_70);
            lVar4 = *(longlong *)(param_1 + 0x200);
            if ((-0x7ffffffffffffffc < lVar4) && (lVar4 != 0)) {
              func_0x000140613c20(*(undefined8 *)(param_1 + 0x208),lVar4,1);
            }
            *(undefined8 *)(param_1 + 0x210) = uStack_78;
            *(undefined8 *)(param_1 + 0x200) = uStack_88;
            *(undefined8 *)(param_1 + 0x208) = uStack_80;
            FUN_141801290(param_1);
            FUN_140b90780(param_1,fVar3 * fVar2);
            LOCK();
            *plStack_58 = *plStack_58 + -1;
            UNLOCK();
            if (*plStack_58 != 0) {
              return;
            }
            FUN_140b8e120(&plStack_58);
            return;
          }
          uVar13 = (ushort)(uVar12 - 1) & (ushort)uVar12;
          uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar13);
        }
        auVar21[0] = -(auVar5[0] == -1);
        auVar21[1] = -(auVar5[1] == -1);
        auVar21[2] = -(auVar5[2] == -1);
        auVar21[3] = -(auVar5[3] == -1);
        auVar21[4] = -(auVar5[4] == -1);
        auVar21[5] = -(auVar5[5] == -1);
        auVar21[6] = -(auVar5[6] == -1);
        auVar21[7] = -(auVar5[7] == -1);
        auVar21[8] = -(auVar5[8] == -1);
        auVar21[9] = -(auVar5[9] == -1);
        auVar21[10] = -(auVar5[10] == -1);
        auVar21[0xb] = -(auVar5[0xb] == -1);
        auVar21[0xc] = -(auVar5[0xc] == -1);
        auVar21[0xd] = -(auVar5[0xd] == -1);
        auVar21[0xe] = -(auVar5[0xe] == -1);
        auVar21[0xf] = -(auVar5[0xf] == -1);
        if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar21 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar21 >> 0x1f,0) & 1) != 0) || (SUB161(auVar21 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x37,0) & 1) != 0) || (SUB161(auVar21 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x47,0) & 1) != 0) || (SUB161(auVar21 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar21 >> 0x57,0) & 1) != 0) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') break;
        uVar10 = uVar10 + lVar11 + 0x10;
        lVar11 = lVar11 + 0x10;
      }
    }
    FUN_1428d9310(&UNK_142afc3e8);
  }
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

