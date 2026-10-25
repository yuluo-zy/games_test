
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 *
FUN_14090d000(undefined4 *param_1,undefined8 param_2,longlong param_3,ulonglong param_4,
             undefined8 *param_5,undefined8 *param_6,undefined1 param_7,longlong *param_8,
             undefined4 param_9,undefined4 param_10)

{
  code *pcVar1;
  longlong lVar2;
  undefined4 *puVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong *plVar8;
  float fVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined *puStack_1c8;
  longlong lStack_1c0;
  longlong lStack_1b8;
  longlong lStack_1b0;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  longlong *plStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  undefined4 uStack_178;
  undefined8 uStack_174;
  undefined8 uStack_16c;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  int iStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined8 uStack_144;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined1 uStack_134;
  undefined1 uStack_133;
  undefined1 uStack_132;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined8 uStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  longlong lStack_110;
  undefined4 *puStack_108;
  byte *pbStack_e0;
  longlong lStack_d8;
  longlong lStack_d0;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined8 uStack_a0;
  longlong lStack_98;
  int iStack_8c;
  undefined *puStack_88;
  uint uStack_7c;
  undefined4 uStack_78;
  undefined1 uStack_74;
  undefined1 uStack_73;
  undefined1 uStack_72;
  undefined1 uStack_71;
  undefined8 uStack_70;
  
  uStack_70 = 0xfffffffffffffffe;
  lVar5 = param_3 + 0x38;
  uStack_72 = 1;
  uVar7 = 0x7e3bf54835a54748;
  uStack_a0 = param_2;
  lVar2 = FUN_1400dd6f0(lVar5,0x7e3bf54835a54748,0x56b9895f6e41fabb);
  if (((lVar2 != 0) && (uVar7 < *(ulonglong *)(param_3 + 0x218))) &&
     (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar7 * 8), uVar7 != 0)) {
    lVar2 = ~uVar7 * 0x60;
    if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar2) != 0) {
      pbStack_e0 = *(byte **)(*(longlong *)(param_3 + 0x1e0) + lVar2 + 0x10);
      uStack_72 = 1;
      uVar7 = 0x4a142e09642375b9;
      lVar2 = FUN_1400dd6f0(lVar5,0x4a142e09642375b9,0x43f6154cfa8c847f);
      if (((lVar2 != 0) && (uVar7 < *(ulonglong *)(param_3 + 0x218))) &&
         (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar7 * 8), uVar7 != 0)) {
        lVar2 = ~uVar7 * 0x60;
        if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar2) != 0) {
          lVar4 = *(longlong *)(*(longlong *)(param_3 + 0x1e0) + lVar2 + 0x10);
          lVar6 = (ulonglong)*(byte *)(lVar4 + 0xc0) * 0x40;
          lVar2 = lVar4 + lVar6;
          if (*(char *)(lVar4 + 0x28 + lVar6) == '\0') {
            uStack_72 = 1;
            FUN_140d7bd90(&lStack_110,lVar2);
            uVar10 = (undefined4)*(undefined8 *)(lVar2 + 0x20);
            uVar11 = (undefined4)((ulonglong)*(undefined8 *)(lVar2 + 0x20) >> 0x20);
          }
          else {
            puVar3 = (undefined4 *)func_0x000140613c10(7,1);
            if (puVar3 == (undefined4 *)0x0) {
              uStack_72 = 1;
              FUN_1428d8fe3(1,7,&UNK_142ae67e0);
              goto code_r0x00014090d6a2;
            }
            *(uint *)((longlong)puVar3 + 3) = CONCAT31(_UNK_142ae6bf3,UNK_142ae6bf2);
            *puVar3 = _UNK_142ae6bef;
            lStack_110 = 7;
            uVar10 = _UNK_14292e6b0;
            uVar11 = _UNK_14292e6b4;
            puStack_108 = puVar3;
          }
          uStack_73 = 1;
          uVar7 = 0xc2b8e941c91a6428;
          lVar4 = FUN_1400dd6f0(lVar5,0xc2b8e941c91a6428,0x59f3c6d1d3da43a0);
          if (((lVar4 != 0) && (uVar7 < *(ulonglong *)(param_3 + 0x218))) &&
             (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar7 * 8), uVar7 != 0)) {
            lVar4 = ~uVar7 * 0x60;
            if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar4) != 0) {
              iStack_8c = **(int **)(*(longlong *)(param_3 + 0x1e0) + lVar4 + 0x10);
              puStack_88 = (undefined *)*param_8;
              uStack_74 = SBORROW8(0,(longlong)puStack_88);
              lStack_d8 = lVar2;
              if ((bool)uStack_74) {
                uVar12 = 0;
                if ((char)param_8[1] == '\x01') {
                  uStack_78 = 1;
                }
                else {
                  puVar3 = (undefined4 *)func_0x000140613c10(7,1);
                  if (puVar3 == (undefined4 *)0x0) {
                    uStack_73 = 1;
                    FUN_1428d8fe3(1,7,&UNK_142ae67e0);
                    goto code_r0x00014090d6a2;
                  }
                  *(uint *)((longlong)puVar3 + 3) = CONCAT31(_UNK_142ae6bf3,UNK_142ae6bf2);
                  *puVar3 = _UNK_142ae6bef;
                  if (lStack_110 != 0) {
                    func_0x000140613c20(puStack_108,lStack_110,1);
                  }
                  lStack_110 = 7;
                  uStack_78 = 3;
                  uVar10 = _UNK_14292e6b0;
                  uVar11 = _UNK_14292e6b4;
                  puStack_108 = puVar3;
                }
                uStack_7c = 0;
              }
              else {
                lStack_98 = param_8[1];
                lStack_d0 = param_8[2];
                uVar12 = (undefined4)param_8[3];
                uStack_7c = (uint)*(byte *)((longlong)param_8 + 0x1c);
                uStack_78 = 0;
              }
              uStack_71 = 1;
              uVar7 = 0x5f06b298510fb768;
              lVar5 = FUN_1400dd6f0(lVar5,0x5f06b298510fb768,0x54e14bacecb858d);
              if (((lVar5 != 0) && (uVar7 < *(ulonglong *)(param_3 + 0x218))) &&
                 (uVar7 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar7 * 8), uVar7 != 0)) {
                lVar5 = ~uVar7 * 0x60;
                if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar5) != 0) {
                  plVar8 = *(longlong **)(*(longlong *)(param_3 + 0x1e0) + lVar5 + 0x10);
                  lVar5 = *plVar8;
                  if (SBORROW8(0,lVar5)) {
                    plVar8 = (longlong *)0x0;
                  }
                  plVar8 = plVar8 + 3;
                  if (SBORROW8(0,lVar5)) {
                    plVar8 = (longlong *)0x0;
                  }
                  uStack_71 = 1;
                  func_0x000140bd52a0(&puStack_1c8,param_4,0x2b);
                  uStack_71 = 1;
                  func_0x000140bd55a0(&uStack_130,&puStack_1c8,4);
                  uStack_b8 = uStack_120;
                  uStack_b4 = uStack_11c;
                  uStack_b0 = uStack_118;
                  uStack_ac = uStack_114;
                  uStack_c8 = uStack_130;
                  uStack_c4 = uStack_12c;
                  uStack_c0 = (undefined4)uStack_128;
                  uStack_bc = uStack_128._4_4_;
                  uStack_71 = 1;
                  func_0x000140bd6ce0(&lStack_1b8,&uStack_c8);
                  puStack_1c8 = &UNK_142aab0e0;
                  lStack_1c0 = 0xe;
                  uStack_71 = 1;
                  uStack_c8 = FUN_140be94d0(uStack_a0,&puStack_1c8);
                  uStack_c4 = uStack_130;
                  uStack_c0 = uStack_12c;
                  uStack_bc = (undefined4)uStack_128;
                  uStack_b8 = uStack_128._4_4_;
                  uStack_b4 = uStack_120;
                  uStack_b0 = uStack_11c;
                  uStack_ac = uStack_118;
                  uStack_a8 = uStack_114;
                  if ((*pbStack_e0 & 1) != 0) {
                    uStack_1a8 = (undefined4)param_4;
                    fVar9 = (float)*(uint *)(param_5 + 2) / (float)(param_4 & 0xffffffff);
                    iStack_150 = 4;
                    if (_UNK_142a7d8d0 <= fVar9) {
                      iStack_150 = (_UNK_142aaa1f0 <= fVar9) + 1;
                    }
                    uStack_133 = iStack_8c == 0x13;
                    uStack_130 = 2;
                    uStack_178 = *(undefined4 *)(param_5 + 4);
                    uStack_198 = *param_5;
                    uStack_190 = param_5[1];
                    uStack_188 = param_5[2];
                    uStack_180 = param_5[3];
                    uStack_174 = *param_6;
                    uStack_16c = param_6[1];
                    uStack_164 = *(undefined4 *)(param_6 + 2);
                    uStack_160 = *(undefined4 *)((longlong)param_6 + 0x14);
                    uStack_15c = *(undefined4 *)(param_6 + 3);
                    uStack_158 = *(undefined4 *)((longlong)param_6 + 0x1c);
                    uStack_154 = *(undefined4 *)(param_6 + 4);
                    uStack_134 = param_7;
                    uStack_148 = uStack_78;
                    uStack_144 = CONCAT44(uVar11,uVar10);
                    uStack_13c = param_10;
                    lStack_1b0 = lStack_d8;
                    uStack_138 = param_9;
                    uStack_1a4 = (undefined4)(param_4 >> 0x20);
                    puStack_1c8 = puStack_88;
                    lStack_1c0 = lStack_98;
                    lStack_1b8 = lStack_d0;
                    uStack_132 = (undefined1)uStack_7c;
                    uStack_71 = 0;
                    plStack_1a0 = plVar8;
                    uStack_14c = uVar12;
                    uStack_128 = &uStack_c8;
                    FUN_1409ce780(uStack_a0,param_3,&UNK_142aab108,10,
                                  *(undefined8 *)(pbStack_e0 + 8),&uStack_130,&puStack_1c8);
                    param_1[8] = uStack_a8;
                    param_1[4] = uStack_b8;
                    param_1[5] = uStack_b4;
                    param_1[6] = uStack_b0;
                    param_1[7] = uStack_ac;
                    *param_1 = uStack_c8;
                    param_1[1] = uStack_c4;
                    param_1[2] = uStack_c0;
                    param_1[3] = uStack_bc;
                    if (lStack_110 != 0) {
                      func_0x000140613c20(puStack_108,lStack_110,1);
                    }
                    return param_1;
                  }
                  uStack_71 = 1;
                  FUN_1428d9310(&UNK_142aab0f0);
                  goto code_r0x00014090d6a2;
                }
              }
              uStack_71 = 1;
              FUN_1428d9310(&UNK_142aab0c8);
              goto code_r0x00014090d6a2;
            }
          }
          uStack_73 = 1;
          FUN_1428d9310(&UNK_142aab0b0);
          goto code_r0x00014090d6a2;
        }
      }
      uStack_72 = 1;
      FUN_1428d9310(&UNK_142aab098);
      goto code_r0x00014090d6a2;
    }
  }
  uStack_72 = 1;
  FUN_1428d9310(&UNK_142aab080);
code_r0x00014090d6a2:
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

