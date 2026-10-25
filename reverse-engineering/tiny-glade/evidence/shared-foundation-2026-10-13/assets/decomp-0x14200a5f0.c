
/* WARNING: Type propagation algorithm not settling */

longlong *******
FUN_14200a5f0(undefined8 param_1,undefined8 param_2,undefined4 *param_3,undefined8 param_4,
             undefined8 param_5,longlong *param_6,undefined1 *param_7)

{
  ulonglong uVar1;
  code *pcVar2;
  longlong *******ppppppplVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  longlong *******ppppppplVar6;
  longlong *plVar7;
  longlong *plVar8;
  undefined1 *puVar9;
  longlong lVar10;
  longlong *******ppppppplVar11;
  undefined4 *puVar12;
  undefined1 *puVar13;
  undefined8 unaff_RBX;
  ulonglong uVar14;
  longlong lVar15;
  char cVar16;
  longlong lVar17;
  undefined8 *puVar18;
  undefined8 uStack_908;
  undefined8 uStack_900;
  longlong *******ppppppplStack_8f8;
  undefined8 uStack_8f0;
  undefined4 uStack_8e8;
  undefined4 uStack_8e4;
  undefined4 uStack_8e0;
  undefined4 uStack_8dc;
  undefined8 uStack_698;
  undefined8 uStack_690;
  longlong *******ppppppplStack_688;
  undefined8 uStack_680;
  undefined4 uStack_678;
  undefined4 uStack_674;
  undefined4 uStack_670;
  undefined4 uStack_66c;
  longlong *******ppppppplStack_428;
  longlong *******ppppppplStack_420;
  undefined4 uStack_418;
  undefined4 uStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  longlong *******ppppppplStack_1b8;
  longlong *******ppppppplStack_1b0;
  undefined4 uStack_1a8;
  undefined4 uStack_1a4;
  undefined4 uStack_1a0;
  undefined4 uStack_19c;
  longlong *******ppppppplStack_198;
  undefined8 uStack_190;
  undefined8 uStack_188;
  undefined8 uStack_180;
  longlong *******ppppppplStack_178;
  longlong *******ppppppplStack_170;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  longlong *******ppppppplStack_130;
  longlong *******ppppppplStack_128;
  longlong *******ppppppplStack_120;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined4 uStack_110;
  undefined4 uStack_10c;
  longlong *******ppppppplStack_108;
  longlong *******ppppppplStack_100;
  longlong lStack_f8;
  undefined1 auStack_e8 [16];
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  uint uStack_8c;
  uint uStack_88;
  longlong *plStack_80;
  longlong *******ppppppplStack_78;
  undefined4 *puStack_70;
  longlong *******ppppppplStack_68;
  undefined *puStack_60;
  longlong *******ppppppplStack_58;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  char cStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  cStack_49 = 0;
  uStack_148 = param_1;
  uStack_140 = param_2;
  uStack_138 = param_4;
  puStack_70 = param_3;
  FUN_140d62810(&ppppppplStack_428,param_1,param_2,&UNK_142fce658,0xb);
  uStack_4d = 1;
  uVar4 = FUN_140d76ed0(&ppppppplStack_428);
  uStack_4d = 1;
  FUN_140d6a710(&uStack_698,uVar4,param_1);
  if ((int)uStack_698 == 2) {
    if (((uint)uStack_690 & 3) == 1) {
      puStack_60 = (undefined *)((longlong)uStack_690 + -1);
      ppppppplStack_58 = *(longlong ********)((longlong)uStack_690 + -1);
      ppppppplStack_68 = *(longlong ********)((longlong)uStack_690 + 7);
      if (*ppppppplStack_68 != (longlong ******)0x0) {
        (*(code *)*ppppppplStack_68)(ppppppplStack_58);
      }
      if (ppppppplStack_68[1] != (longlong ******)0x0) {
        func_0x000140613c20(ppppppplStack_58,ppppppplStack_68[1],ppppppplStack_68[2]);
      }
      func_0x000140613c20(puStack_60,0x18,8);
    }
    if (ppppppplStack_428 != (longlong *******)0x0) {
      func_0x000140613c20(ppppppplStack_420,ppppppplStack_428,1);
    }
    uVar14 = 0;
  }
  else {
    uStack_4d = 1;
    uVar5 = FUN_140d76ed0(&ppppppplStack_428);
    uStack_4d = 1;
    FUN_14200ba50(&uStack_698,uVar5,uVar4);
    if (uStack_698 == (longlong *******)0x0) {
      if (ppppppplStack_428 == (longlong *******)0x0) {
        return uStack_690;
      }
      func_0x000140613c20(ppppppplStack_420,ppppppplStack_428,1);
      return uStack_690;
    }
    uStack_8e8 = uStack_678;
    uStack_8e4 = uStack_674;
    uStack_8e0 = uStack_670;
    uStack_8dc = uStack_66c;
    ppppppplStack_8f8 = ppppppplStack_688;
    uStack_8f0 = uStack_680;
    uStack_908 = uStack_698;
    uStack_900 = uStack_690;
    ppppppplStack_58 = ppppppplStack_428;
    ppppppplStack_68 = ppppppplStack_420;
    uStack_698 = (longlong *******)CONCAT44(uStack_414,uStack_418);
    uStack_690 = (longlong *******)CONCAT44(uStack_40c,uStack_410);
    lVar15 = param_6[2];
    if (lVar15 == *param_6) {
      FUN_140562c30(param_6,&UNK_142fce668);
    }
    lVar17 = param_6[1];
    lVar10 = lVar15 * 0x20;
    *(longlong ********)(lVar17 + lVar10) = ppppppplStack_58;
    *(longlong ********)(lVar17 + 8 + lVar10) = ppppppplStack_68;
    puVar12 = (undefined4 *)(lVar17 + 0x10 + lVar10);
    *puVar12 = (int)uStack_698;
    puVar12[1] = uStack_698._4_4_;
    puVar12[2] = (undefined4)uStack_690;
    puVar12[3] = uStack_690._4_4_;
    param_6[2] = lVar15 + 1;
    FUN_142424d90(&uStack_908,puStack_70);
    uStack_188 = CONCAT44(uStack_8e4,uStack_8e8);
    uStack_180 = CONCAT44(uStack_8dc,uStack_8e0);
    ppppppplStack_198 = ppppppplStack_8f8;
    uStack_190 = uStack_8f0;
    uStack_1a8 = (undefined4)uStack_908;
    uStack_1a4 = uStack_908._4_4_;
    uStack_1a0 = (undefined4)uStack_900;
    uStack_19c = uStack_900._4_4_;
    puStack_70 = &uStack_1a8;
    uVar14 = CONCAT71((int7)((ulonglong)unaff_RBX >> 8),1);
  }
  uVar1 = param_6[2];
  cStack_49 = (char)uVar14;
  FUN_140d68f80(&uStack_698,uStack_148,uStack_140);
  if ((int)uStack_698 == 2) {
    uStack_900 = uStack_690;
    uStack_908 = (longlong *******)0x2;
  }
  else {
    func_0x000142923c00(&uStack_908,&uStack_698,0x270);
  }
  cStack_49 = (char)uVar14;
  FUN_142425b00(&ppppppplStack_428,&uStack_908,&uStack_148);
  ppppppplVar6 = ppppppplStack_420;
  ppppppplVar3 = ppppppplStack_428;
  if (ppppppplStack_428 == (longlong *******)0x2) {
LAB_14200b2a7:
    if ((char)uVar14 != '\0') {
      FUN_140b742a0(&uStack_1a8,&uStack_188,0xd8,0x10);
    }
  }
  else {
    puStack_60 = (undefined *)CONCAT44(puStack_60._4_4_,(int)uVar14);
    func_0x000142923c00(&ppppppplStack_688,&uStack_418,0x260);
    uStack_698 = ppppppplVar3;
    uStack_690 = ppppppplVar6;
    ppppppplStack_68 = (longlong *******)(uVar1 << 5 | 8);
    while (FUN_140d59e80(&uStack_908,&uStack_698), ppppppplVar3 = ppppppplStack_8f8,
          ppppppplVar6 = uStack_900, ((ulonglong)uStack_908 & 1) != 0) {
      lVar15 = param_6[2] - uVar1;
      if (uVar1 <= (ulonglong)param_6[2]) {
        param_6[2] = uVar1;
        if (lVar15 != 0) {
          puVar18 = (undefined8 *)(param_6[1] + (longlong)ppppppplStack_68);
          do {
            if (puVar18[-1] != 0) {
              func_0x000140613c20(*puVar18,puVar18[-1],1);
            }
            puVar18 = puVar18 + 4;
            lVar15 = lVar15 + -1;
          } while (lVar15 != 0);
        }
      }
      if (ppppppplVar6 == (longlong *******)0x0) {
        ppppppplVar6 = (longlong *******)FUN_14291c770(ppppppplVar3);
LAB_14200b26c:
        if (uStack_698 != (longlong *******)0x0) {
          FUN_140d685b0(&uStack_690);
        }
        LOCK();
        *ppppppplStack_688 = (longlong ******)((longlong)*ppppppplStack_688 + -1);
        UNLOCK();
        uVar14 = (ulonglong)puStack_60 & 0xffffffff;
        if (*ppppppplStack_688 == (longlong ******)0x0) {
          cStack_49 = (char)puStack_60;
          FUN_140d53bc0(&ppppppplStack_688);
        }
        goto LAB_14200b2a7;
      }
      func_0x000142923c00(&uStack_418,&uStack_8f0,0x248);
      ppppppplStack_428 = ppppppplVar6;
      ppppppplStack_420 = ppppppplVar3;
      FUN_140d5a0b0(&uStack_d8,&ppppppplStack_428);
      if ((int)uStack_d8 == 2) {
        ppppppplVar6 = (longlong *******)FUN_14291c770(uStack_d0);
LAB_14200b253:
        LOCK();
        *ppppppplStack_428 = (longlong ******)((longlong)*ppppppplStack_428 + -1);
        UNLOCK();
        if (*ppppppplStack_428 == (longlong ******)0x0) {
          FUN_140d53bc0(&ppppppplStack_428);
        }
        goto LAB_14200b26c;
      }
      if (((uStack_8c & 0x10) == 0) || ((uStack_8c & 0x400) != 0 && (uStack_88 & 0x20000000) != 0))
      {
        ppppppplVar6 = (longlong *******)&ppppppplStack_428;
        FUN_140d5a130(&uStack_d8,ppppppplVar6);
        uVar4 = FUN_140d76ed0(&uStack_d8);
        FUN_140d58e70(&ppppppplStack_108,uVar4,ppppppplVar6);
        if ((int)ppppppplStack_108 == 1) {
          FUN_1428d9310(&UNK_142fce680);
          goto LAB_14200b388;
        }
        if ((lStack_f8 != 0xb) ||
           (ppppppplVar6 = uStack_d8, ppppppplVar3 = uStack_d0,
           *(longlong *)((longlong)ppppppplStack_100 + 3) != 0x6e6f722e746c7561 ||
           *ppppppplStack_100 != (longlong ******)0x2e746c7561666564)) {
          if (uStack_d8 != (longlong *******)0x0) {
            func_0x000140613c20(uStack_d0,uStack_d8,1);
          }
          ppppppplVar11 = (longlong *******)&ppppppplStack_428;
          FUN_140d59f10(&ppppppplStack_128,ppppppplVar11);
          uStack_4c = 1;
          uVar4 = FUN_140d76ed0(&ppppppplStack_128);
          uStack_4c = 1;
          lVar15 = FUN_140d627a0(uVar4);
          ppppppplVar6 = ppppppplStack_128;
          ppppppplVar3 = ppppppplStack_120;
          if (lVar15 != 0) {
            uStack_4c = 1;
            FUN_140d58e70(&uStack_d8,lVar15,ppppppplVar11);
            if ((int)uStack_d8 == 1) {
              uStack_4c = 1;
              FUN_1428d9310(&UNK_142fce730);
              goto LAB_14200b388;
            }
            ppppppplStack_1b8 = uStack_d0;
            ppppppplStack_1b0 = uStack_c8;
            if (uStack_c8 == (longlong *******)0xc) {
              ppppppplVar6 = ppppppplStack_128;
              ppppppplVar3 = ppppppplStack_120;
              if (*(int *)(uStack_d0 + 1) == 0x7265746c &&
                  *uStack_d0 == (longlong ******)0x69662d7465737361) goto joined_r0x00014200ae72;
            }
            else if ((uStack_c8 == (longlong *******)0x3) &&
                    (*(char *)((longlong)uStack_d0 + 2) == 'n' && *(short *)uStack_d0 == 0x6f72)) {
              uStack_4c = 1;
              uVar4 = FUN_140d76ed0(&ppppppplStack_128);
              uStack_4c = 1;
              FUN_14200ba50(&uStack_d8,uVar4,lVar15);
              ppppppplVar6 = uStack_d0;
              if (uStack_d8 == (longlong *******)0x0) goto joined_r0x00014200b23f;
              uStack_158 = (undefined4)uStack_b8;
              uStack_154 = uStack_b8._4_4_;
              uStack_150 = uStack_b0;
              uStack_14c = uStack_ac;
              uStack_168 = (undefined4)uStack_c8;
              uStack_164 = uStack_c8._4_4_;
              uStack_160 = (undefined4)uStack_c0;
              uStack_15c = uStack_c0._4_4_;
              ppppppplStack_178 = uStack_d8;
              ppppppplStack_170 = uStack_d0;
              uStack_4a = 1;
              uStack_4b = 1;
              puVar12 = puStack_70;
              FUN_142424d90(&ppppppplStack_178,puStack_70);
              uStack_4a = 1;
              uStack_4b = 1;
              uVar4 = FUN_140d76ed0(&ppppppplStack_128);
              uStack_4a = 1;
              uStack_4b = 1;
              lVar15 = FUN_140d62700(uVar4);
              if (lVar15 == 0) {
                uStack_4a = 1;
                uStack_4b = 1;
                FUN_1428d9310(&UNK_142fce6d0);
LAB_14200b388:
                    /* WARNING: Does not return */
                pcVar2 = (code *)invalidInstructionException();
                (*pcVar2)();
              }
              uStack_4a = 1;
              uStack_4b = 1;
              FUN_140d58e70(&uStack_d8,lVar15,puVar12);
              if ((int)uStack_d8 == 1) {
                uStack_4a = 1;
                uStack_4b = 1;
                FUN_1428d9310(&UNK_142fce6e8);
                goto LAB_14200b388;
              }
              uStack_4a = 1;
              uStack_4b = 1;
              FUN_142425c70(&uStack_d8,uStack_138,param_5,uStack_d0,uStack_c8);
              ppppppplVar6 = uStack_c8;
              ppppppplStack_58 = uStack_d0;
              if ((longlong)uStack_c8 < 0) {
                FUN_1428d9760(&UNK_142bb2e90,0x2b,&ppppppplStack_108,&UNK_142bb2e70,&UNK_142bb2f08);
                goto LAB_14200b388;
              }
              ppppppplVar11 = uStack_c8;
              plVar7 = (longlong *)FUN_140d7c180(1);
              plVar8 = plVar7;
              ppppppplVar3 = ppppppplStack_58;
              if (ppppppplVar11 != (longlong *******)0x0) {
                plVar8 = (longlong *)func_0x000140613c10(ppppppplVar11,plVar7);
                ppppppplVar3 = ppppppplStack_58;
              }
              ppppppplStack_58 = ppppppplVar3;
              if (plVar8 == (longlong *)0x0) {
                FUN_1428d9000(plVar7,ppppppplVar11);
                goto LAB_14200b388;
              }
              *plVar8 = 1;
              plVar8[1] = 1;
              func_0x000142923c00(plVar8 + 2,ppppppplVar3,ppppppplVar6);
              ppppppplStack_78 = ppppppplVar6;
              plStack_80 = plVar8;
              if (uStack_d8 != (longlong *******)0x0) {
                func_0x000140613c20(ppppppplVar3,uStack_d8,1);
              }
              ppppppplStack_58 = ppppppplStack_128;
              ppppppplStack_130 = ppppppplStack_120;
              uStack_d8 = (longlong *******)CONCAT44(uStack_114,uStack_118);
              uStack_d0 = (longlong *******)CONCAT44(uStack_10c,uStack_110);
              lVar15 = param_6[2];
              if (lVar15 == *param_6) {
                FUN_140562c30(param_6,&UNK_142fce700);
              }
              lVar17 = param_6[1];
              lVar10 = lVar15 * 0x20;
              *(longlong ********)(lVar17 + lVar10) = ppppppplStack_58;
              *(longlong ********)(lVar17 + 8 + lVar10) = ppppppplStack_130;
              puVar18 = (undefined8 *)(lVar17 + 0x10 + lVar10);
              *puVar18 = uStack_d8;
              puVar18[1] = uStack_d0;
              param_6[2] = lVar15 + 1;
              LOCK();
              lVar15 = *plStack_80;
              *plStack_80 = *plStack_80 + 1;
              UNLOCK();
              if (*plStack_80 == 0 || SCARRY8(lVar15,1) != *plStack_80 < 0) goto LAB_14200b388;
              uStack_b8 = CONCAT44(uStack_154,uStack_158);
              uStack_b0 = uStack_150;
              uStack_ac = uStack_14c;
              uStack_c8 = (longlong *******)CONCAT44(uStack_164,uStack_168);
              uStack_c0 = CONCAT44(uStack_15c,uStack_160);
              uStack_d8 = ppppppplStack_178;
              uStack_d0 = ppppppplStack_170;
              uStack_4e = 0;
              puVar13 = param_7;
              FUN_142423000(&ppppppplStack_108,param_7,plStack_80,ppppppplStack_78,&uStack_d8);
              if (ppppppplStack_108 != (longlong *******)0x0) {
                uStack_4e = 0;
                puVar13 = auStack_e8;
                FUN_140b742a0(&ppppppplStack_108,puVar13,0xd8,0x10);
              }
              lVar15 = param_6[1];
              lVar17 = param_6[2] * 0x20 + lVar15;
              for (; lVar15 != lVar17; lVar15 = lVar15 + 0x20) {
                uStack_4e = 0;
                puVar9 = (undefined1 *)FUN_140d76ed0(lVar15);
                LOCK();
                lVar10 = *plStack_80;
                *plStack_80 = *plStack_80 + 1;
                UNLOCK();
                if (*plStack_80 == 0 || SCARRY8(lVar10,1) != *plStack_80 < 0) goto LAB_14200b388;
                uStack_4e = 0;
                FUN_142009df0(param_7 + 0x30,puVar9,puVar13,plStack_80,ppppppplStack_78);
                puVar13 = puVar9;
              }
              LOCK();
              *plStack_80 = *plStack_80 + -1;
              UNLOCK();
              if (*plStack_80 == 0) {
                uStack_4a = 0;
                uStack_4b = 0;
                func_0x00014064eb30(&plStack_80);
              }
              goto LAB_14200ab27;
            }
            ppppppplStack_108 = (longlong *******)&ppppppplStack_1b8;
            ppppppplStack_100 = (longlong *******)FUN_142009d90;
            uStack_d8 = (longlong *******)&UNK_142fce6c0;
            uStack_d0 = (longlong *******)0x1;
            uStack_b8 = 0;
            uStack_c8 = (longlong *******)&ppppppplStack_108;
            uStack_c0 = 1;
            uStack_4c = 1;
            ppppppplVar6 = (longlong *******)FUN_142914d30(&uStack_d8);
            goto joined_r0x00014200b23f;
          }
        }
joined_r0x00014200ae72:
        if (ppppppplVar6 != (longlong *******)0x0) {
          func_0x000140613c20(ppppppplVar3,ppppppplVar6,1);
        }
        LOCK();
        *ppppppplStack_428 = (longlong ******)((longlong)*ppppppplStack_428 + -1);
        UNLOCK();
        if (*ppppppplStack_428 == (longlong ******)0x0) {
          FUN_140d53bc0(&ppppppplStack_428);
        }
      }
      else {
        ppppppplVar6 = (longlong *******)&ppppppplStack_428;
        FUN_140d5a130(&ppppppplStack_108,ppppppplVar6);
        uVar4 = FUN_140d76ed0(&ppppppplStack_108);
        FUN_140d58e70(&uStack_d8,uVar4,ppppppplVar6);
        ppppppplVar3 = uStack_c8;
        ppppppplVar6 = uStack_d0;
        if ((int)uStack_d8 == 1) {
          FUN_1428d9310(&UNK_142fce718);
          goto LAB_14200b388;
        }
        ppppppplVar11 = (longlong *******)&ppppppplStack_428;
        FUN_140d59f10(&uStack_d8,ppppppplVar11);
        uVar4 = FUN_140d76ed0(&uStack_d8);
        FUN_142425c70(&ppppppplStack_178,uStack_138,param_5,ppppppplVar6,ppppppplVar3);
        ppppppplStack_58 = ppppppplStack_170;
        ppppppplVar6 = (longlong *******)
                       FUN_14200a5f0(uVar4,ppppppplVar11,puStack_70,ppppppplStack_170,
                                     CONCAT44(uStack_164,uStack_168),param_6,param_7);
        if (ppppppplVar6 != (longlong *******)0x0) {
          if (ppppppplStack_178 != (longlong *******)0x0) {
            func_0x000140613c20(ppppppplStack_58,ppppppplStack_178,1);
          }
          ppppppplStack_128 = ppppppplStack_108;
          ppppppplStack_120 = ppppppplStack_100;
          if (uStack_d8 != (longlong *******)0x0) {
            func_0x000140613c20(uStack_d0,uStack_d8,1);
            ppppppplStack_128 = ppppppplStack_108;
            ppppppplStack_120 = ppppppplStack_100;
          }
joined_r0x00014200b23f:
          if (ppppppplStack_128 != (longlong *******)0x0) {
            func_0x000140613c20(ppppppplStack_120,ppppppplStack_128,1);
          }
          goto LAB_14200b253;
        }
        if (ppppppplStack_178 != (longlong *******)0x0) {
          func_0x000140613c20(ppppppplStack_58,ppppppplStack_178,1);
        }
        if (uStack_d8 != (longlong *******)0x0) {
          func_0x000140613c20(uStack_d0,uStack_d8,1);
        }
        if (ppppppplStack_108 != (longlong *******)0x0) {
          func_0x000140613c20(ppppppplStack_100,ppppppplStack_108,1);
        }
LAB_14200ab27:
        LOCK();
        *ppppppplStack_428 = (longlong ******)((longlong)*ppppppplStack_428 + -1);
        UNLOCK();
        if (*ppppppplStack_428 == (longlong ******)0x0) {
          FUN_140d53bc0(&ppppppplStack_428);
        }
      }
    }
    if (uStack_698 != (longlong *******)0x0) {
      FUN_140d685b0(&uStack_690);
    }
    LOCK();
    *ppppppplStack_688 = (longlong ******)((longlong)*ppppppplStack_688 + -1);
    UNLOCK();
    cVar16 = (char)puStack_60;
    if (*ppppppplStack_688 == (longlong ******)0x0) {
      cStack_49 = cVar16;
      FUN_140d53bc0(&ppppppplStack_688);
    }
    if (cVar16 != '\0') {
      FUN_140b742a0(&uStack_1a8,&uStack_188,0xd8,0x10);
    }
    ppppppplVar6 = (longlong *******)0x0;
  }
  return ppppppplVar6;
}

