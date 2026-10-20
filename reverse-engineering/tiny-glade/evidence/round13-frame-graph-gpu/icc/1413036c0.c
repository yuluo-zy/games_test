
/* WARNING: Removing unreachable block (ram,0x000141303be6) */

ulonglong * FUN_1413036c0(ulonglong *param_1,byte *param_2,undefined8 param_3)

{
  uint uVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  code *pcVar5;
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  ulonglong uVar15;
  undefined8 *puVar16;
  ulonglong uVar17;
  ulonglong *puVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  int iVar21;
  ulonglong uVar22;
  uint uVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  uint *puVar26;
  longlong lVar27;
  undefined8 uVar28;
  ulonglong uVar29;
  uint uVar30;
  ulonglong uStack_f0;
  ulonglong uStack_d8;
  undefined8 *puStack_d0;
  ulonglong uStack_c8;
  ulonglong uStack_c0;
  uint auStack_b8 [2];
  longlong lStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  ulonglong uStack_98;
  ulonglong uStack_90;
  ulonglong uStack_88;
  ulonglong uStack_80;
  ulonglong uStack_78;
  ulonglong uStack_70;
  uint uStack_64;
  byte bStack_60;
  undefined1 uStack_5f;
  undefined6 uStack_5e;
  undefined2 uStack_58;
  ushort uStack_52;
  uint uStack_50;
  byte bStack_4a;
  byte bStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_1412f2310();
  if ((bStack_60 & 1) == 0) {
    *param_1 = 0;
    param_1[1] = 2;
    param_1[2] = 0;
    return param_1;
  }
  uVar24 = *(ulonglong *)(param_2 + 0x18);
  if (uVar24 == 2) {
    uVar13 = 0;
  }
  else {
    if (*(int *)(param_2 + 0x38) == 1) {
      uVar15 = 0;
      if (*(ulonglong *)(param_2 + 0x40) <= *(ulonglong *)(param_2 + 0x48)) {
        uVar15 = *(ulonglong *)(param_2 + 0x48) - *(ulonglong *)(param_2 + 0x40);
      }
      if (*(int *)(param_2 + 0x58) == 1) goto LAB_14130374e;
LAB_141303741:
      uVar12 = 0;
    }
    else {
      uVar15 = 0;
      if (*(int *)(param_2 + 0x58) != 1) goto LAB_141303741;
LAB_14130374e:
      uVar12 = 0;
      if (*(ulonglong *)(param_2 + 0x60) <= *(ulonglong *)(param_2 + 0x68)) {
        uVar12 = *(ulonglong *)(param_2 + 0x68) - *(ulonglong *)(param_2 + 0x60);
      }
    }
    uVar13 = 0xffffffffffffffff;
    if (!CARRY8(uVar15,uVar12)) {
      uVar13 = uVar15 + uVar12;
    }
  }
  lStack_b0 = *(longlong *)(param_2 + 0x78);
  if (lStack_b0 == 2) {
    uVar15 = 0;
    goto LAB_1413037da;
  }
  if (*(int *)(param_2 + 0x98) == 1) {
    uVar12 = 0;
    if (*(ulonglong *)(param_2 + 0xa0) <= *(ulonglong *)(param_2 + 0xa8)) {
      uVar12 = *(ulonglong *)(param_2 + 0xa8) - *(ulonglong *)(param_2 + 0xa0);
    }
    if (*(int *)(param_2 + 0xb8) == 1) goto LAB_1413037b8;
LAB_1413037a8:
    uVar25 = 0;
  }
  else {
    uVar12 = 0;
    if (*(int *)(param_2 + 0xb8) != 1) goto LAB_1413037a8;
LAB_1413037b8:
    uVar25 = 0;
    if (*(ulonglong *)(param_2 + 0xc0) <= *(ulonglong *)(param_2 + 200)) {
      uVar25 = *(ulonglong *)(param_2 + 200) - *(ulonglong *)(param_2 + 0xc0);
    }
  }
  uVar15 = 0xffffffffffffffff;
  if (!CARRY8(uVar12,uVar25)) {
    uVar15 = uVar12 + uVar25;
  }
LAB_1413037da:
  lVar14 = uVar13 + uVar15;
  if (CARRY8(uVar13,uVar15)) {
    lVar14 = -1;
  }
  uVar15 = lVar14 + 1;
  if (uVar15 == 0) {
    uVar15 = 0xffffffffffffffff;
  }
  uVar12 = 4;
  if (4 < uVar15) {
    uVar12 = uVar15;
  }
  uVar13 = uVar12 * 8;
  uVar28 = 0;
  if ((uVar15 >> 0x3d == 0) && (uVar13 < 0x7fffffffffffffff)) {
    uVar28 = 2;
    puVar16 = (undefined8 *)func_0x000140613c10(uVar13,2);
    if (puVar16 != (undefined8 *)0x0) {
      *puVar16 = CONCAT26(uStack_58,uStack_5e);
      uStack_c8 = 1;
      bStack_4a = *param_2;
      uVar15 = *(ulonglong *)(param_2 + 8);
      uVar13 = *(ulonglong *)(param_2 + 0x10);
      uStack_90 = *(ulonglong *)(param_2 + 0x20);
      uStack_f0 = *(ulonglong *)(param_2 + 0x28);
      uVar23 = (uint)*(ushort *)(param_2 + 0x30);
      uVar25 = *(ulonglong *)(param_2 + 0x38);
      uVar28 = *(undefined8 *)(param_2 + 0x40);
      bStack_60 = (byte)uVar28;
      uStack_5f = (undefined1)((ulonglong)uVar28 >> 8);
      uStack_5e = (undefined6)((ulonglong)uVar28 >> 0x10);
      uStack_98 = *(ulonglong *)(param_2 + 0x48);
      uStack_50 = *(uint *)(param_2 + 0x50);
      uStack_78 = *(ulonglong *)(param_2 + 0x58);
      uStack_c0 = *(ulonglong *)(param_2 + 0x60);
      uVar2 = *(ulonglong *)(param_2 + 0x68);
      auStack_b8[0] = *(uint *)(param_2 + 0x70);
      uVar30 = *(uint *)(param_2 + 0xb0);
      uStack_64 = uVar30 >> 0x10;
      bStack_49 = param_2[0xb8];
      uStack_a0 = *(ulonglong *)(param_2 + 0x80);
      uVar3 = *(ulonglong *)(param_2 + 0x88);
      uStack_52 = *(ushort *)(param_2 + 0x90);
      uStack_80 = *(ulonglong *)(param_2 + 0x98);
      uStack_88 = *(ulonglong *)(param_2 + 0xa0);
      uStack_a8 = *(ulonglong *)(param_2 + 0xa8);
      uStack_70 = *(ulonglong *)(param_2 + 0xc0);
      uVar4 = *(ulonglong *)(param_2 + 200);
      uVar1 = *(uint *)(param_2 + 0xd0);
      uStack_d8 = uVar12;
      puStack_d0 = puVar16;
      do {
        uVar12 = uStack_c8;
        if (uVar24 == 2) goto LAB_14130397b;
        uVar29 = uVar25;
        if ((uVar25 & 1) == 0) {
LAB_141303aa3:
          uVar25 = uVar29;
          if (((uVar24 & 1) != 0) && (uStack_90 < uStack_f0)) {
            auVar8._8_8_ = 0;
            auVar8._0_8_ = uStack_90 * 0xffff;
            iVar21 = SUB164(auVar8 * ZEXT816(0x842108421084211),8);
            bStack_60 = 0;
            uStack_5f = 0;
            uStack_5e = 0;
            uStack_50 = CONCAT22((short)uVar23,
                                 (short)(((uint)((int)(uStack_90 * 0xffff) - iVar21) >> 1) + iVar21
                                        >> 4));
            uVar25 = 1;
            uStack_90 = uStack_90 + 1;
            uVar22 = 0;
            uStack_98 = 0x20;
            goto LAB_141303b10;
          }
          if ((uStack_78 & 1) != 0) {
            if (uVar2 <= uStack_c0) {
              uStack_78 = 0;
              goto LAB_14130397b;
            }
            puVar18 = &uStack_c0;
            uVar22 = uStack_c0;
            puVar26 = auStack_b8;
            goto LAB_141303b18;
          }
LAB_14130397b:
          if ((uVar15 < uVar13 & bStack_4a) == 1) {
            uVar24 = uVar15 * 0xffff;
            uVar15 = uVar15 + 1;
            auVar6._8_8_ = 0;
            auVar6._0_8_ = uVar24;
            iVar21 = SUB164(auVar6 * ZEXT816(0x842108421084211),8);
            uVar23 = ((uint)((int)uVar24 - iVar21) >> 1) + iVar21 >> 4;
            uStack_50 = uVar23 << 0x10;
            uVar25 = 1;
            uStack_98 = 0x20;
            uStack_78 = 0;
            uStack_f0 = 0x20;
            uStack_90 = 1;
            uVar24 = 1;
            uVar22 = 0;
            goto LAB_141303b10;
          }
          if ((int)lStack_b0 == 2) {
LAB_141303d21:
            param_1[2] = uStack_c8;
            *param_1 = uStack_d8;
            param_1[1] = (ulonglong)puStack_d0;
            return param_1;
          }
          if ((uStack_80 & 1) == 0) {
LAB_141303b95:
            if (((byte)lStack_b0 & uStack_a0 < uVar3) != 0) {
              uVar24 = uStack_a0 * 0xffff;
              uStack_a0 = uStack_a0 + 1;
              auVar10._8_8_ = 0;
              auVar10._0_8_ = uVar24;
              iVar21 = SUB164(auVar10 * ZEXT816(0x842108421084211),8);
              uVar30 = ((uint)((int)uVar24 - iVar21) >> 1) + iVar21 >> 4;
              uStack_80 = 1;
              uStack_a8 = 0x20;
              uStack_88 = 0;
              uStack_64 = (uint)uStack_52;
              goto LAB_141303a46;
            }
            if ((bStack_49 & uStack_70 < uVar4) == 0) goto LAB_141303d21;
            auVar11._8_8_ = 0;
            auVar11._0_8_ = uStack_70 * 0xffff;
            iVar21 = SUB164(auVar11 * ZEXT816(0x842108421084211),8);
            uVar29 = (ulonglong)
                     (((uint)((int)(uStack_70 * 0xffff) - iVar21) >> 1) + iVar21 >> 4 & 0xffff) |
                     (ulonglong)uVar1 << 0x10;
            uStack_70 = uStack_70 + 1;
          }
          else {
            if (uStack_a8 <= uStack_88) {
              uStack_80 = 0;
              goto LAB_141303b95;
            }
LAB_141303a46:
            auVar7._8_8_ = 0;
            auVar7._0_8_ = uStack_88 * 0xffff;
            iVar21 = SUB164(auVar7 * ZEXT816(0x842108421084211),8);
            uVar29 = (ulonglong)
                     (((uint)((int)(uStack_88 * 0xffff) - iVar21) >> 1) + iVar21 >> 4 & 0xffff) |
                     (ulonglong)CONCAT24((undefined2)uStack_64,uVar30 << 0x10);
            uStack_88 = uStack_88 + 1;
          }
          uVar24 = 2;
        }
        else {
          uVar22 = CONCAT62(uStack_5e,CONCAT11(uStack_5f,bStack_60));
          uVar29 = 0;
          if (uStack_98 <= uVar22) goto LAB_141303aa3;
LAB_141303b10:
          puVar18 = (ulonglong *)&bStack_60;
          puVar26 = &uStack_50;
LAB_141303b18:
          *puVar18 = uVar22 + 1;
          auVar9._8_8_ = 0;
          auVar9._0_8_ = uVar22 * 0xffff;
          iVar21 = SUB164(auVar9 * ZEXT816(0x842108421084211),8);
          uVar29 = (ulonglong)
                   (((uint)((int)(uVar22 * 0xffff) - iVar21) >> 1) + iVar21 >> 4 & 0xffff) |
                   (ulonglong)*puVar26 << 0x10;
        }
        if (uStack_c8 == uStack_d8) {
          if (uVar24 == 2) {
            uVar17 = 0;
            if ((int)lStack_b0 == 2) goto LAB_141303ce5;
LAB_141303c59:
            uVar22 = uStack_a8 - uStack_88;
            if (uStack_a8 < uStack_88) {
              uVar22 = 0;
            }
            if ((uStack_80 & 1) == 0) {
              uVar22 = 0;
            }
            uVar19 = uVar4 - uStack_70;
            if (uVar4 < uStack_70) {
              uVar19 = 0;
            }
            if ((bStack_49 & 1) == 0) {
              uVar19 = 0;
            }
            uVar20 = uVar19 + uVar22;
            if (CARRY8(uVar19,uVar22)) {
              uVar20 = 0xffffffffffffffff;
            }
          }
          else {
            uVar22 = uStack_98 - CONCAT62(uStack_5e,CONCAT11(uStack_5f,bStack_60));
            if (uStack_98 < CONCAT62(uStack_5e,CONCAT11(uStack_5f,bStack_60))) {
              uVar22 = 0;
            }
            if ((uVar25 & 1) == 0) {
              uVar22 = 0;
            }
            uVar19 = uVar2 - uStack_c0;
            if (uVar2 < uStack_c0) {
              uVar19 = 0;
            }
            if ((uStack_78 & 1) == 0) {
              uVar19 = 0;
            }
            uVar17 = uVar19 + uVar22;
            if (CARRY8(uVar19,uVar22)) {
              uVar17 = 0xffffffffffffffff;
            }
            if ((int)lStack_b0 != 2) goto LAB_141303c59;
LAB_141303ce5:
            uVar20 = 0;
          }
          lVar14 = uVar17 + uVar20;
          if (CARRY8(uVar17,uVar20)) {
            lVar14 = -1;
          }
          lVar27 = -1;
          if (lVar14 + 1 != 0) {
            lVar27 = lVar14 + 1;
          }
          FUN_1428a9a60(&uStack_d8,uStack_c8,lVar27,2,8);
          puVar16 = puStack_d0;
        }
        puVar16[uVar12] = uVar29 | 0xff000000000000;
        uStack_c8 = uVar12 + 1;
      } while( true );
    }
  }
  FUN_1428d8fe3(uVar28,uVar13,param_3);
  pcVar5 = (code *)swi(3);
  puVar18 = (ulonglong *)(*pcVar5)();
  return puVar18;
}

