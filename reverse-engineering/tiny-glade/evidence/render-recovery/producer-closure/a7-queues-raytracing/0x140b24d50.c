
void FUN_140b24d50(longlong *param_1)

{
  longlong *plVar1;
  undefined1 auVar2 [16];
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong *puVar8;
  longlong *plVar9;
  byte bVar10;
  char cVar11;
  undefined8 *puVar12;
  bool bVar13;
  longlong lStack_220;
  undefined *puStack_218;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  longlong lStack_1e0;
  longlong lStack_1d8;
  undefined *puStack_1d0;
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  longlong lStack_198;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined *puStack_188;
  undefined8 uStack_180;
  longlong ***ppplStack_178;
  undefined8 uStack_170;
  ulonglong *puStack_168;
  undefined8 uStack_160;
  undefined1 (*pauStack_138) [16];
  undefined1 (*pauStack_130) [16];
  undefined1 *puStack_128;
  ushort uStack_120;
  undefined8 uStack_118;
  undefined1 (*pauStack_108) [16];
  longlong lStack_100;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  longlong lStack_d8;
  char cStack_d0;
  undefined4 uStack_cf;
  undefined3 uStack_cb;
  ulonglong uStack_c8;
  longlong *plStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined1 uStack_98;
  longlong lStack_90;
  longlong *plStack_88;
  byte bStack_80;
  longlong **pplStack_78;
  code *pcStack_70;
  undefined8 uStack_68;
  undefined1 uStack_59;
  longlong *plStack_58;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  puVar8 = puRam000000014362c420;
  uStack_48 = 0xfffffffffffffffe;
  plStack_58 = param_1;
  while( true ) {
    plVar9 = plStack_58;
    plVar1 = plStack_58 + 2;
    LOCK();
    bVar13 = (int)plStack_58[2] == 0;
    if (bVar13) {
      *(int *)(plStack_58 + 2) = 0x3fffffff;
    }
    UNLOCK();
    if (!bVar13) {
      FUN_1428d8c10(plVar1);
    }
    if ((*puVar8 & 0x7fffffffffffffff) == 0) {
      bStack_80 = 0;
      cVar11 = (char)plVar9[3];
    }
    else {
      bStack_80 = FUN_1428d86b0();
      bStack_80 = bStack_80 ^ 1;
      cVar11 = (char)plVar9[3];
    }
    if (cVar11 != '\0') break;
    cVar11 = (char)plVar9[0x14];
    *(undefined1 *)(plVar9 + 0x14) = 2;
    if (cVar11 == '\x02') {
      if (((bStack_80 == 0) && ((*puVar8 & 0x7fffffffffffffff) != 0)) &&
         (cVar11 = FUN_1428d86b0(), cVar11 == '\0')) {
        *(undefined1 *)(plVar9 + 3) = 1;
      }
      LOCK();
      lVar7 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -0x3fffffff;
      UNLOCK();
      if (0x3fffffff < (int)lVar7 + 0xc0000001U) {
        FUN_1428d8d40(plVar1);
      }
      FUN_140d55980(0,1000000);
    }
    else {
      lStack_d8 = plVar9[0x13];
      pauStack_108 = (undefined1 (*) [16])plVar9[0xd];
      lStack_100 = plVar9[0xe];
      uStack_f8 = (undefined4)plVar9[0xf];
      uStack_f4 = *(undefined4 *)((longlong)plVar9 + 0x7c);
      uStack_f0 = (undefined4)plVar9[0x10];
      uStack_ec = *(undefined4 *)((longlong)plVar9 + 0x84);
      uStack_e8 = (undefined4)plVar9[0x11];
      uStack_e4 = *(undefined4 *)((longlong)plVar9 + 0x8c);
      uStack_e0 = (undefined4)plVar9[0x12];
      uStack_dc = *(undefined4 *)((longlong)plVar9 + 0x94);
      uStack_cf = *(undefined4 *)((longlong)plVar9 + 0xa1);
      uStack_cb = (undefined3)((uint)*(undefined4 *)((longlong)plVar9 + 0xa4) >> 8);
      *(undefined1 *)(plVar9 + 0x15) = 1;
      cStack_d0 = cVar11;
      if (((bStack_80 == 0) && ((*puVar8 & 0x7fffffffffffffff) != 0)) &&
         (cVar11 = FUN_1428d86b0(), cVar11 == '\0')) {
        *(undefined1 *)(plVar9 + 3) = 1;
      }
      LOCK();
      lVar7 = *plVar1;
      *(int *)plVar1 = (int)*plVar1 + -0x3fffffff;
      UNLOCK();
      if (0x3fffffff < (int)lVar7 + 0xc0000001U) {
        FUN_1428d8d40(plVar1);
      }
      puStack_128 = *pauStack_108 + lStack_100 + 1;
      auVar2 = *pauStack_108;
      uStack_120 = ~((ushort)(SUB161(auVar2 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
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
                    (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
      uStack_118 = CONCAT44(uStack_ec,uStack_f0);
      pauStack_138 = pauStack_108;
      pauStack_130 = pauStack_108 + 1;
      lStack_1d8 = 0;
      puStack_188 = (undefined *)0x0;
      FUN_14130d1b0(&pplStack_78,&lStack_1d8,&UNK_142ae96e0);
      FUN_140b0da50(&lStack_220,pcStack_70,uStack_68);
      plVar9 = plStack_58;
      lStack_198 = lStack_1e0;
      uStack_1a8 = CONCAT44(uStack_1ec,uStack_1f0);
      uStack_1a0 = (undefined *)CONCAT44(uStack_1e4,uStack_1e8);
      uStack_1b8 = CONCAT44(uStack_1fc,uStack_200);
      uStack_1b0 = CONCAT44(uStack_1f4,uStack_1f8);
      uStack_1c8 = CONCAT44(uStack_20c,uStack_210);
      uStack_1c0 = CONCAT44(uStack_204,uStack_208);
      lStack_1d8 = lStack_220;
      puStack_1d0 = puStack_218;
      plVar1 = plStack_58 + 2;
      LOCK();
      bVar13 = (int)plStack_58[2] == 0;
      if (bVar13) {
        *(int *)(plStack_58 + 2) = 0x3fffffff;
      }
      UNLOCK();
      if (!bVar13) {
        uStack_49 = 1;
        FUN_1428d8c10(plVar1);
      }
      if ((*puVar8 & 0x7fffffffffffffff) == 0) {
        bVar10 = 0;
      }
      else {
        uStack_49 = 1;
        bVar10 = FUN_1428d86b0();
        bVar10 = bVar10 ^ 1;
      }
      bVar13 = (char)plVar9[3] == '\0';
      uStack_c8 = (ulonglong)!bVar13;
      uStack_b8 = CONCAT71(uStack_b8._1_7_,bVar10);
      plStack_c0 = plVar1;
      if (bVar13) {
        lVar7 = plVar9[4];
        lVar3 = plVar9[5];
        lVar4 = plVar9[7];
        lVar5 = plVar9[8];
        lVar6 = plVar9[10];
        lStack_90 = plVar9[0xb];
        plVar9[0xc] = lStack_198;
        plVar9[4] = lStack_1d8;
        plVar9[5] = (longlong)puStack_1d0;
        *(undefined4 *)(plVar9 + 6) = (undefined4)uStack_1c8;
        *(undefined4 *)((longlong)plVar9 + 0x34) = uStack_1c8._4_4_;
        *(undefined4 *)(plVar9 + 7) = (undefined4)uStack_1c0;
        *(undefined4 *)((longlong)plVar9 + 0x3c) = uStack_1c0._4_4_;
        *(undefined4 *)(plVar9 + 8) = (undefined4)uStack_1b8;
        *(undefined4 *)((longlong)plVar9 + 0x44) = uStack_1b8._4_4_;
        *(undefined4 *)(plVar9 + 9) = (undefined4)uStack_1b0;
        *(undefined4 *)((longlong)plVar9 + 0x4c) = uStack_1b0._4_4_;
        *(undefined4 *)(plVar9 + 10) = (undefined4)uStack_1a8;
        *(undefined4 *)((longlong)plVar9 + 0x54) = uStack_1a8._4_4_;
        *(undefined4 *)(plVar9 + 0xb) = (undefined4)uStack_1a0;
        *(undefined4 *)((longlong)plVar9 + 0x5c) = uStack_1a0._4_4_;
        if (!SBORROW8(0,lVar7)) {
          if (lVar7 != 0) {
            func_0x000140613c20(lVar3,lVar7 * 0x50,4);
          }
          if (lVar4 != 0) {
            func_0x000140613c20(lVar5,lVar4 * 0x18,4);
          }
          if (lVar6 != 0) {
            func_0x000140613c20(lStack_90,lVar6 << 2,4);
          }
        }
        *(undefined1 *)(plVar9 + 0x15) = 0;
        if (((bVar10 == 0) && ((*puVar8 & 0x7fffffffffffffff) != 0)) &&
           (cVar11 = FUN_1428d86b0(), cVar11 == '\0')) {
          *(undefined1 *)(plVar9 + 3) = 1;
        }
        LOCK();
        lVar7 = *plVar1;
        *(int *)plVar1 = (int)*plVar1 + -0x3fffffff;
        UNLOCK();
        if (0x3fffffff < (int)lVar7 + 0xc0000001U) {
          FUN_1428d8d40(plVar1);
        }
        plVar1 = plStack_c0;
        if ((uStack_c8 & 1) != 0) {
          if (((char)uStack_b8 == '\0') && ((*puVar8 & 0x7fffffffffffffff) != 0)) {
            uStack_49 = 0;
            cVar11 = FUN_1428d86b0();
            if (cVar11 == '\0') {
              *(undefined1 *)(plVar1 + 1) = 1;
            }
          }
          LOCK();
          lVar7 = *plVar1;
          *(int *)plVar1 = (int)*plVar1 + -0x3fffffff;
          UNLOCK();
          if (0x3fffffff < (int)lVar7 + 0xc0000001U) {
            uStack_49 = 0;
            FUN_1428d8d40();
          }
        }
      }
      else {
        if ((bVar10 == 0) && ((*puVar8 & 0x7fffffffffffffff) != 0)) {
          uStack_49 = 1;
          cVar11 = FUN_1428d86b0();
          if (cVar11 == '\0') {
            *(undefined1 *)(plVar9 + 3) = 1;
          }
        }
        LOCK();
        lVar7 = *plVar1;
        *(int *)plVar1 = (int)*plVar1 + -0x3fffffff;
        UNLOCK();
        if (0x3fffffff < (int)lVar7 + 0xc0000001U) {
          uStack_49 = 1;
          FUN_1428d8d40(plVar1);
        }
        if (lStack_1d8 != 0) {
          func_0x000140613c20(puStack_1d0,lStack_1d8 * 0x50,4);
        }
        if (uStack_1c0 != 0) {
          func_0x000140613c20(uStack_1b8,uStack_1c0 * 0x18,4);
        }
        if (uStack_1a8 != 0) {
          func_0x000140613c20(uStack_1a0,uStack_1a8 << 2,4);
        }
      }
      if (pplStack_78 != (longlong **)0x0) {
        func_0x000140613c20(pcStack_70,(longlong)pplStack_78 * 0x28,4);
      }
      FUN_1400524e0(&pauStack_108,&uStack_e8,0x20,0x10);
    }
  }
  plStack_88 = plVar1;
  if (2 < *puRam000000014362c3a8) {
    pplStack_78 = &plStack_88;
    pcStack_70 = FUN_140b6d1a0;
    uStack_c8 = 2;
    uStack_b8 = 2;
    uStack_a8 = 0;
    uStack_a0 = 0x400000020;
    uStack_98 = 3;
    puVar12 = (undefined8 *)func_0x00014067dd80(&UNK_142ae9aa8);
    uStack_1b8 = *puVar12;
    uStack_1b0 = puVar12[1];
    uStack_18c = *(undefined4 *)(puVar12 + 2);
    uStack_1a8 = 3;
    uStack_1a0 = &UNK_142ae9ac0;
    lStack_198 = 0x28;
    puStack_188 = &UNK_142ae9a60;
    uStack_180 = 1;
    uStack_170 = 1;
    puStack_168 = &uStack_c8;
    uStack_160 = 1;
    lStack_1d8 = 0;
    puStack_1d0 = &UNK_142ae9ac0;
    uStack_1c8 = 0x28;
    uStack_1c0 = 0;
    uStack_190 = 1;
    ppplStack_178 = &pplStack_78;
    func_0x000140d4a330(&uStack_59,&lStack_1d8);
  }
  plVar1 = plStack_88;
  if ((((bStack_80 & 1) == 0) && ((*puVar8 & 0x7fffffffffffffff) != 0)) &&
     (cVar11 = FUN_1428d86b0(), cVar11 == '\0')) {
    *(undefined1 *)(plVar1 + 1) = 1;
  }
  LOCK();
  lVar7 = *plVar1;
  *(int *)plVar1 = (int)*plVar1 + -0x3fffffff;
  UNLOCK();
  if (0x3fffffff < (int)lVar7 + 0xc0000001U) {
    FUN_1428d8d40(plVar1);
  }
  LOCK();
  *plStack_58 = *plStack_58 + -1;
  UNLOCK();
  if (*plStack_58 == 0) {
    FUN_140b5c7a0(&plStack_58);
  }
  return;
}

