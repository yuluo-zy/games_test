
void FUN_140bec400(longlong param_1)

{
  char cVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  undefined8 uVar5;
  ulonglong *puVar6;
  code *pcVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  longlong *plVar10;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  int *piVar16;
  bool bVar17;
  int *piStack_208;
  code *pcStack_200;
  longlong *plStack_1f8;
  undefined8 uStack_1f0;
  int **ppiStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined4 uStack_1d0;
  undefined4 uStack_1cc;
  undefined4 uStack_1c8;
  undefined4 uStack_1c4;
  undefined4 uStack_1c0;
  undefined4 uStack_1bc;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  undefined8 uStack_198;
  undefined8 uStack_190;
  undefined4 uStack_188;
  undefined4 uStack_184;
  undefined4 uStack_180;
  undefined4 uStack_17c;
  int iStack_178;
  undefined8 uStack_174;
  undefined4 uStack_16c;
  longlong *plStack_168;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  int **ppiStack_158;
  undefined8 uStack_150;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  undefined8 uStack_100;
  undefined8 uStack_f8;
  undefined8 uStack_f0;
  longlong *plStack_e8;
  undefined8 uStack_e0;
  int **ppiStack_d8;
  longlong lStack_d0;
  undefined8 uStack_c8;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  longlong lStack_68;
  undefined4 uStack_60;
  undefined2 uStack_5c;
  undefined2 uStack_5a;
  int iStack_58;
  undefined2 uStack_54;
  char cStack_52;
  undefined1 uStack_51;
  undefined8 uStack_50;
  longlong lStack_48;
  char cStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  plStack_168 = (longlong *)0x8000000000000000;
  uStack_39 = 0;
  lStack_48 = param_1;
  FUN_140bebc90();
  if (*(longlong *)(param_1 + 0x50) != 0) {
    lVar11 = *(longlong *)(param_1 + 0x50) * 0x58;
    piVar16 = (int *)(*(longlong *)(param_1 + 0x48) + lVar11 + -0x58);
    if (piVar16 != (int *)0x0) {
      lVar11 = *(longlong *)(param_1 + 0x48) + lVar11;
      lVar2 = *(longlong *)(param_1 + 0xe8);
      *(undefined8 *)(param_1 + 0xe8) = 0x8000000000000000;
      if (!SBORROW8(0,lVar2)) {
        uStack_50 = *(undefined8 *)(param_1 + 0x100);
        uStack_60 = *(undefined4 *)(param_1 + 0xf0);
        iStack_58 = *(int *)(param_1 + 0xf8);
        uVar8 = *(undefined4 *)(param_1 + 0xfc);
        uStack_5c = (undefined2)*(undefined4 *)(param_1 + 0xf4);
        uStack_5a = (undefined2)((uint)*(undefined4 *)(param_1 + 0xf4) >> 0x10);
        uStack_54 = (undefined2)uVar8;
        cStack_52 = (char)((uint)uVar8 >> 0x10);
        uStack_51 = (undefined1)((uint)uVar8 >> 0x18);
        uVar3 = *(undefined8 *)(lVar11 + -0x10);
        cStack_3a = *(char *)(param_1 + 0x164);
        *(undefined1 *)(param_1 + 0x164) = 2;
        lStack_68 = lVar2;
        if (cStack_3a == '\x02') {
          FUN_1428d9310(&UNK_142b148f0);
          goto code_r0x000140beca53;
        }
        uStack_a8 = (undefined4)((ulonglong)*(undefined8 *)(param_1 + 0x154) >> 0x20);
        uStack_a4 = (undefined4)*(undefined8 *)(param_1 + 0x15c);
        uStack_a0 = (undefined4)((ulonglong)*(undefined8 *)(param_1 + 0x15c) >> 0x20);
        plStack_e8 = *(longlong **)(param_1 + 0x118);
        uStack_e0 = *(longlong **)(param_1 + 0x120);
        ppiStack_d8 = *(int ***)(param_1 + 0x128);
        lStack_d0 = *(undefined8 *)(param_1 + 0x130);
        uStack_c8 = *(undefined8 *)(param_1 + 0x138);
        uStack_c0 = *(undefined4 *)(param_1 + 0x140);
        uStack_bc = *(undefined4 *)(param_1 + 0x144);
        uStack_b8 = *(undefined4 *)(param_1 + 0x148);
        uStack_b4 = *(undefined4 *)(param_1 + 0x14c);
        uStack_b0 = *(undefined4 *)(param_1 + 0x150);
        uStack_ac = *(undefined4 *)(param_1 + 0x154);
        uStack_9c = CONCAT31((int3)*(undefined8 *)(param_1 + 0x165),cStack_3a);
        uStack_98 = (undefined4)((ulonglong)*(undefined8 *)(param_1 + 0x165) >> 0x18);
        uStack_94 = *(undefined4 *)(param_1 + 0x16c);
        FUN_1418c3910(&plStack_1f8,&lStack_68,*(longlong *)(param_1 + 0x178) + 0x78,uVar3,
                      &plStack_e8);
        plStack_e8 = plStack_1f8;
        uStack_e0 = (longlong *)uStack_1f0;
        ppiStack_d8 = ppiStack_1e8;
        lStack_d0 = uStack_1e0;
        uStack_c8 = uStack_1d8;
        uStack_c0 = uStack_1d0;
        uStack_bc = uStack_1cc;
        uStack_b8 = uStack_1c8;
        uStack_b4 = uStack_1c4;
        uStack_b0 = uStack_1c0;
        uStack_ac = uStack_1bc;
        uStack_a8 = (undefined4)uStack_1b8;
        uStack_a4 = (undefined4)((ulonglong)uStack_1b8 >> 0x20);
        uStack_a0 = (undefined4)uStack_1b0;
        uStack_9c = (undefined4)((ulonglong)uStack_1b0 >> 0x20);
        uStack_98 = (undefined4)uStack_1a8;
        uStack_94 = (undefined4)((ulonglong)uStack_1a8 >> 0x20);
        uStack_90 = uStack_1a0;
        uStack_88 = uStack_198;
        uStack_80 = uStack_190;
        uStack_78 = uStack_188;
        uStack_74 = uStack_184;
        uStack_70 = uStack_180;
        uStack_6c = uStack_17c;
        if (!SBORROW8(0,(longlong)plStack_168)) {
          FUN_140bde030(&plStack_168);
        }
        uStack_f8 = CONCAT44(uStack_74,uStack_78);
        uStack_f0 = CONCAT44(uStack_6c,uStack_70);
        uStack_108 = uStack_88;
        uStack_100 = uStack_80;
        uStack_118 = CONCAT44(uStack_94,uStack_98);
        uStack_110 = uStack_90;
        uStack_128 = CONCAT44(uStack_a4,uStack_a8);
        uStack_120 = CONCAT44(uStack_9c,uStack_a0);
        uStack_140 = CONCAT44(uStack_bc,uStack_c0);
        uStack_138 = CONCAT44(uStack_b4,uStack_b8);
        uStack_130 = CONCAT44(uStack_ac,uStack_b0);
        uStack_148 = uStack_c8;
        ppiStack_158 = ppiStack_d8;
        uStack_150 = lStack_d0;
        plStack_168 = plStack_e8;
        uStack_160 = (undefined4)uStack_e0;
        uStack_15c = uStack_e0._4_4_;
        param_1 = lStack_48;
      }
      ppiStack_d8 = (int **)(param_1 + 0x28);
      plStack_e8 = *(longlong **)(param_1 + 0x30);
      lStack_d0 = *(longlong *)(param_1 + 0x38);
      *(undefined8 *)(param_1 + 0x38) = 0;
      uStack_e0 = plStack_e8 + lStack_d0 * 3;
      uStack_c8 = 0;
      plVar10 = plStack_e8;
      if (plStack_e8 != uStack_e0) {
        do {
          plVar10 = plStack_e8 + 3;
          cVar1 = *(char *)((longlong)plStack_e8 + 0x16);
          if (cVar1 == '\b') break;
          uStack_51 = *(undefined1 *)((longlong)plStack_e8 + 0x17);
          iStack_58 = (int)((ulonglong)*(undefined8 *)((longlong)plStack_e8 + 0xe) >> 0x10);
          uStack_54 = (undefined2)((ulonglong)*(undefined8 *)((longlong)plStack_e8 + 0xe) >> 0x30);
          lStack_68 = *plStack_e8;
          uStack_60 = (undefined4)plStack_e8[1];
          uStack_5c = (undefined2)*(undefined4 *)((longlong)plStack_e8 + 0xc);
          uStack_5a = (undefined2)((uint)*(undefined4 *)((longlong)plStack_e8 + 0xc) >> 0x10);
          puVar6 = (ulonglong *)CONCAT26(uStack_5a,(int6)plStack_e8[1]);
          plStack_e8 = plVar10;
          cStack_52 = cVar1;
          if (*(int *)((longlong)puVar6 + 0xc) == iStack_58) {
            if (*piVar16 != 1) {
              pcStack_200 = FUN_140bed620;
              plStack_1f8 = (longlong *)&UNK_142b14bb0;
              uStack_1f0 = 1;
              uStack_1d8 = 0;
              ppiStack_1e8 = &piStack_208;
              uStack_1e0 = 1;
              piStack_208 = piVar16;
              FUN_1428d9390(&plStack_1f8,&UNK_142b14bf0);
              goto code_r0x000140beca53;
            }
            uVar14 = *(ulonglong *)(lVar11 + -0x50);
            uVar13 = *puVar6;
            do {
              uVar15 = uVar14;
              if (uVar14 < uVar13) {
                uVar15 = uVar13;
              }
              LOCK();
              uVar12 = *puVar6;
              bVar17 = uVar13 == uVar12;
              if (bVar17) {
                *puVar6 = uVar15;
                uVar12 = uVar13;
              }
              UNLOCK();
              uVar13 = uVar12;
            } while (!bVar17);
          }
          FUN_140be24b0(&lStack_68);
          plVar10 = plStack_e8;
        } while (plStack_e8 != uStack_e0);
      }
      plStack_e8 = plVar10;
      lVar4 = lStack_48;
      uStack_39 = 0;
      FUN_140bdd590(&plStack_e8);
      lVar2 = *(longlong *)(lVar4 + 0x170);
      lVar4 = *(longlong *)(lVar4 + 0x178);
      uVar3 = *(undefined8 *)(lVar11 + -0x10);
      uVar5 = *(undefined8 *)(lVar2 + 200);
      (**(code **)(lVar4 + 0x400))(uVar3,0x2000,uVar5,1);
      uStack_39 = 0;
      lVar11 = FUN_1418aeba0(*(undefined8 *)(lVar4 + 0x648),lVar2 + 0x20,piVar16);
      if (lVar11 == 0) {
        uStack_39 = 0;
        FUN_1428d9310(&UNK_142b14768);
      }
      else {
        (**(code **)(lVar4 + 0x408))(uVar3,uVar5,0,2,*(undefined8 *)(lVar11 + 0x20),0,8,3);
        lVar11 = lStack_48;
        uStack_39 = 0;
        FUN_140bebdf0(lStack_48,*(undefined8 *)(lStack_48 + 0x178));
        iStack_178 = *(int *)(lVar11 + 0x108);
        *(undefined4 *)(lVar11 + 0x108) = 2;
        if (iStack_178 != 2) {
          uStack_16c = *(undefined4 *)(lVar11 + 0x114);
          uStack_174 = *(undefined8 *)(lVar11 + 0x10c);
          puVar6 = *(ulonglong **)(lVar11 + 0x170);
          ppiStack_1e8 = ppiStack_158;
          uStack_1e0 = uStack_150;
          uStack_1d8 = uStack_148;
          uStack_1d0 = (undefined4)uStack_140;
          uStack_1cc = uStack_140._4_4_;
          uStack_1c8 = (undefined4)uStack_138;
          uStack_1c4 = uStack_138._4_4_;
          uStack_1c0 = (undefined4)uStack_130;
          uStack_1bc = uStack_130._4_4_;
          uStack_1b8 = uStack_128;
          uStack_1b0 = uStack_120;
          uStack_1a8 = uStack_118;
          uStack_1a0 = uStack_110;
          uStack_198 = uStack_108;
          uStack_190 = uStack_100;
          uStack_188 = (undefined4)uStack_f8;
          uStack_184 = uStack_f8._4_4_;
          uStack_180 = (undefined4)uStack_f0;
          uStack_17c = uStack_f0._4_4_;
          plStack_1f8 = plStack_168;
          uStack_1f0 = CONCAT44(uStack_15c,uStack_160);
          uVar14 = *puVar6;
          uVar13 = puVar6[3];
          if (uVar13 == uVar14) {
            FUN_140631140(puVar6,&UNK_142b14920);
            uVar14 = *puVar6;
            uVar13 = puVar6[3];
            lVar11 = lStack_48;
          }
          uVar15 = 0;
          if (uVar14 <= uVar13 + puVar6[2]) {
            uVar15 = uVar14;
          }
          func_0x000142923c10(((uVar13 + puVar6[2]) - uVar15) * 0x90 + puVar6[1],&plStack_1f8,0x90);
          puVar6[3] = puVar6[3] + 1;
          lVar2 = *(longlong *)(lVar11 + 0x170);
          uVar3 = *(undefined8 *)(lVar2 + 0x88);
          uVar5 = *(undefined8 *)(lVar2 + 0x90);
          *(undefined4 *)(lVar2 + 0x88) = *(undefined4 *)(lVar2 + 0x48);
          *(undefined4 *)(lVar2 + 0x8c) = *(undefined4 *)(lVar2 + 0x4c);
          *(undefined4 *)(lVar2 + 0x90) = *(undefined4 *)(lVar2 + 0x50);
          *(undefined4 *)(lVar2 + 0x94) = *(undefined4 *)(lVar2 + 0x54);
          *(undefined8 *)(lVar2 + 0x48) = uVar3;
          *(undefined8 *)(lVar2 + 0x50) = uVar5;
          uVar3 = *(undefined8 *)(lVar2 + 0x98);
          uVar5 = *(undefined8 *)(lVar2 + 0xa0);
          *(undefined4 *)(lVar2 + 0x98) = *(undefined4 *)(lVar2 + 0x58);
          *(undefined4 *)(lVar2 + 0x9c) = *(undefined4 *)(lVar2 + 0x5c);
          *(undefined4 *)(lVar2 + 0xa0) = *(undefined4 *)(lVar2 + 0x60);
          *(undefined4 *)(lVar2 + 0xa4) = *(undefined4 *)(lVar2 + 100);
          *(undefined8 *)(lVar2 + 0x58) = uVar3;
          *(undefined8 *)(lVar2 + 0x60) = uVar5;
          uVar3 = *(undefined8 *)(lVar2 + 0xa8);
          uVar5 = *(undefined8 *)(lVar2 + 0xb0);
          *(undefined4 *)(lVar2 + 0xa8) = *(undefined4 *)(lVar2 + 0x68);
          *(undefined4 *)(lVar2 + 0xac) = *(undefined4 *)(lVar2 + 0x6c);
          *(undefined4 *)(lVar2 + 0xb0) = *(undefined4 *)(lVar2 + 0x70);
          *(undefined4 *)(lVar2 + 0xb4) = *(undefined4 *)(lVar2 + 0x74);
          *(undefined8 *)(lVar2 + 0x68) = uVar3;
          *(undefined8 *)(lVar2 + 0x70) = uVar5;
          uVar8 = *(undefined4 *)(lVar2 + 0xb8);
          uVar9 = *(undefined4 *)(lVar2 + 0xc0);
          *(undefined8 *)(lVar2 + 0xb8) = *(undefined8 *)(lVar2 + 0x78);
          *(undefined8 *)(lVar2 + 0xc0) = *(undefined8 *)(lVar2 + 0x80);
          *(undefined4 *)(lVar2 + 0x78) = uVar8;
          *(undefined4 *)(lVar2 + 0x7c) = *(undefined4 *)(lVar2 + 0xbc);
          *(undefined4 *)(lVar2 + 0x80) = uVar9;
          *(undefined4 *)(lVar2 + 0x84) = *(undefined4 *)(lVar2 + 0xc4);
          lVar2 = *(longlong *)(lVar11 + 0x170);
          if (*(longlong *)(lVar2 + 0xa0) != 0) {
            uVar14 = *(ulonglong *)(lVar2 + 0x90);
            if (uVar14 != 0) {
              func_0x000142923c20(*(undefined8 *)(lVar2 + 0x88),0xff,uVar14 + 0x11);
            }
            *(undefined8 *)(lVar2 + 0xa0) = 0;
            uVar13 = (uVar14 + 1 & 0xfffffffffffffff8) - (uVar14 + 1 >> 3);
            if (uVar14 < 8) {
              uVar13 = uVar14;
            }
            *(ulonglong *)(lVar2 + 0x98) = uVar13;
            lVar11 = lStack_48;
          }
          *(undefined8 *)(lVar2 + 0xb8) = 0;
          *(undefined8 *)(lVar2 + 0xc0) = 0;
          FUN_140be0590(lVar11);
          return;
        }
        uStack_39 = 0;
        FUN_1428d9310(&UNK_142b14908);
      }
      goto code_r0x000140beca53;
    }
  }
  uStack_39 = 0;
  FUN_1428d9310(&UNK_142b148d8);
code_r0x000140beca53:
                    /* WARNING: Does not return */
  pcVar7 = (code *)invalidInstructionException();
  (*pcVar7)();
}

