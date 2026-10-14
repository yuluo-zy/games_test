
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_14067c3d0(undefined8 *param_1)

{
  code *pcVar1;
  longlong *******ppppppplVar2;
  longlong *******ppppppplVar3;
  longlong *plVar4;
  longlong ******pppppplVar5;
  char cVar6;
  undefined8 uVar7;
  byte *pbVar8;
  undefined8 *puVar9;
  longlong *******ppppppplVar10;
  longlong lVar11;
  longlong *plVar12;
  ulonglong uVar13;
  undefined1 auStack_1d8 [88];
  longlong *plStack_180;
  longlong *plStack_178;
  longlong *plStack_170;
  longlong *plStack_168;
  longlong lStack_160;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined4 uStack_148;
  undefined4 uStack_144;
  int iStack_140;
  undefined4 uStack_13c;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  int iStack_120;
  undefined4 uStack_11c;
  longlong *******ppppppplStack_118;
  longlong *******ppppppplStack_110;
  longlong *******ppppppplStack_108;
  longlong *******ppppppplStack_100;
  longlong *******ppppppplStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  longlong *******ppppppplStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  undefined8 uStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  longlong *******ppppppplStack_a8;
  longlong *******ppppppplStack_a0;
  longlong *******ppppppplStack_98;
  longlong *******ppppppplStack_90;
  longlong *******ppppppplStack_88;
  longlong *******ppppppplStack_80;
  longlong *******ppppppplStack_78;
  longlong *******ppppppplStack_70;
  longlong *******ppppppplStack_68;
  longlong *******ppppppplStack_60;
  longlong *plStack_58;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  FUN_140d58eb0(&uStack_e8,&UNK_142a16af0,0x18);
  ppppppplVar2 = ppppppplStack_d8;
  if (SBORROW8(0,(longlong)uStack_e8)) {
    ppppppplStack_108 = uStack_e0;
    uStack_138 = (longlong ******)0x0;
    uStack_130 = (longlong *)0x1;
    uStack_128 = 0;
    uStack_c8 = (longlong *******)0x0;
    uStack_b8 = (longlong *******)0x0;
    ppppppplStack_a8 = (longlong *******)0x20;
    ppppppplStack_a0 = (longlong *******)CONCAT71(ppppppplStack_a0._1_7_,3);
    ppppppplStack_98 = (longlong *******)&uStack_138;
    ppppppplStack_90 = (longlong *******)&UNK_142a169e0;
    cVar6 = FUN_140d5b010(&ppppppplStack_108,&uStack_c8);
    lVar11 = uStack_128;
    plVar4 = uStack_130;
    pppppplVar5 = uStack_138;
    if (cVar6 == '\0') {
      if (((uint)uStack_e0 & 3) == 1) {
        ppppppplStack_68 = (longlong *******)((longlong)uStack_e0 + -1);
        plStack_58 = *(longlong **)((longlong)uStack_e0 + -1);
        ppppppplStack_60 = *(longlong ********)((longlong)uStack_e0 + 7);
        if (*ppppppplStack_60 != (longlong ******)0x0) {
          (*(code *)*ppppppplStack_60)(plStack_58);
        }
        if (ppppppplStack_60[1] != (longlong ******)0x0) {
          func_0x000140613c20(plStack_58,ppppppplStack_60[1],ppppppplStack_60[2]);
        }
        func_0x000140613c20(ppppppplStack_68,0x18,8);
      }
      param_1[1] = pppppplVar5;
      param_1[2] = plVar4;
      param_1[3] = lVar11;
      *param_1 = 0x8000000000000001;
      return param_1;
    }
    FUN_1428d9760(&UNK_142a16a10,0x37,&uStack_4a,&UNK_142a16ab0,&UNK_142a16a98);
LAB_14067cd5d:
                    /* WARNING: Does not return */
    pcVar1 = (code *)invalidInstructionException();
    (*pcVar1)();
  }
  ppppppplStack_68 = uStack_e8;
  ppppppplStack_60 = uStack_e0;
  FUN_140db6090(auStack_1d8,uStack_e0,ppppppplStack_d8);
  uStack_c8 = (longlong *******)0xf52dad71b327e45a;
  func_0x000140f737f0(auStack_1d8,&uStack_c8);
  if ((short)uStack_c8 == 0) {
    ppppppplVar10 = ppppppplStack_60;
    FUN_140f739a0(&uStack_138,ppppppplStack_60,ppppppplVar2);
    ppppppplVar3 = ppppppplStack_60;
    ppppppplVar2 = ppppppplStack_68;
    plVar4 = uStack_130;
    if (!SBORROW8(0,(longlong)uStack_138)) {
      uStack_148 = (undefined4)uStack_128;
      uStack_144 = uStack_128._4_4_;
      iStack_140 = iStack_120;
      uStack_13c = uStack_11c;
      uStack_158 = (undefined4)uStack_138;
      uStack_154 = uStack_138._4_4_;
      uStack_150 = (undefined4)uStack_130;
      uStack_14c = uStack_130._4_4_;
      if (iStack_120 == 1) {
        pbVar8 = (byte *)FUN_140d4d740();
        if ((*pbVar8 & 1) == 0) {
          uStack_49 = 1;
          ppppppplStack_a8 = (longlong *******)FUN_140d64e00();
          pbVar8[0] = 1;
          pbVar8[1] = 0;
          pbVar8[2] = 0;
          pbVar8[3] = 0;
          pbVar8[4] = 0;
          pbVar8[5] = 0;
          pbVar8[6] = 0;
          pbVar8[7] = 0;
          *(longlong ********)(pbVar8 + 8) = ppppppplStack_a8;
          *(longlong ********)(pbVar8 + 0x10) = ppppppplVar10;
        }
        else {
          ppppppplStack_a8 = *(longlong ********)(pbVar8 + 8);
          ppppppplVar10 = *(longlong ********)(pbVar8 + 0x10);
        }
        *(undefined **)(pbVar8 + 8) = (undefined *)((longlong)ppppppplStack_a8 + 1);
        uStack_b8 = (longlong *******)_DAT_142a169d0;
        uStack_b0 = (longlong *******)_UNK_142a169d8;
        uStack_c8 = (longlong *******)CONCAT44(_UNK_142a169c4,_DAT_142a169c0);
        uStack_c0 = (longlong *******)CONCAT44(_UNK_142a169cc,_UNK_142a169c8);
        ppppppplStack_a0 = ppppppplVar10;
        if (uStack_128 != 0) {
          FUN_1428e5220(&uStack_c8,uStack_128,&ppppppplStack_a8,1);
          lVar11 = 0;
          do {
            FUN_140d7bd90(&uStack_e8,uStack_130 + lVar11 * 6);
            FUN_140f7a7a0(&uStack_c8,&uStack_e8);
            lVar11 = lVar11 + 1;
          } while (lVar11 != uStack_128);
        }
        ppppppplStack_118 = ppppppplStack_a8;
        ppppppplStack_110 = ppppppplStack_a0;
        uStack_128 = (longlong)uStack_b8;
        iStack_120 = (int)uStack_b0;
        uStack_11c = uStack_b0._4_4_;
        uStack_138 = (longlong ******)uStack_c8;
        uStack_130 = (longlong *)uStack_c0;
        uStack_e8 = (longlong *******)0x0;
        uStack_e0 = (longlong *******)0x8;
        ppppppplStack_d8 = (longlong *******)0x0;
        uStack_b8 = (longlong *******)CONCAT44(uStack_144,uStack_148);
        uStack_c8 = (longlong *******)CONCAT44(uStack_154,uStack_158);
        uStack_c0 = (longlong *******)CONCAT44(uStack_14c,uStack_150);
        FUN_140f77ad0(&ppppppplStack_108,&uStack_e8,&uStack_c8);
        FUN_140f76810(&plStack_170,&ppppppplStack_108);
        plVar4 = plStack_170;
        ppppppplStack_80 = (longlong *******)0x0;
        ppppppplStack_78 = (longlong *******)0x8;
        ppppppplStack_70 = (longlong *******)0x0;
        ppppppplStack_108 = (longlong *******)0x0;
        ppppppplStack_100 = (longlong *******)0x8;
        ppppppplStack_f8 = (longlong *******)0x0;
        uStack_b0 = (longlong *******)(plStack_168 + lStack_160 * 4);
        uStack_c8 = (longlong *******)plStack_168;
        uStack_c0 = (longlong *******)plStack_168;
        uStack_b8 = (longlong *******)plStack_170;
        if (lStack_160 != 0) {
          plStack_180 = plStack_170;
          plStack_178 = plStack_168;
          plStack_58 = plStack_168 + 4;
                    /* WARNING: Could not recover jumptable at 0x00014067c9b8. Too many branches */
                    /* WARNING: Treating indirect jump as call */
          puVar9 = (undefined8 *)
                   (*(code *)(&UNK_142a16b78 + *(int *)(&UNK_142a16b78 + *plStack_168 * 4)))
                             (&UNK_142a16b78 + *(int *)(&UNK_142a16b78 + *plStack_168 * 4));
          return puVar9;
        }
        if ((longlong)uStack_b0 - (longlong)plStack_168 != 0) {
          uVar13 = (ulonglong)((longlong)uStack_b0 - (longlong)plStack_168) >> 5;
          plVar12 = plStack_168 + 2;
          do {
            if ((plVar12[-2] != 0) && (plVar12[-1] != 0)) {
              func_0x000140613c20(*plVar12,plVar12[-1],1);
            }
            plVar12 = plVar12 + 4;
            uVar13 = uVar13 - 1;
          } while (uVar13 != 0);
        }
        if (plVar4 != (longlong *)0x0) {
          func_0x000140613c20(plStack_168,(longlong)plVar4 << 5,8);
        }
        uStack_e8 = (longlong *******)0x0;
        uStack_e0 = (longlong *******)0x8;
        ppppppplStack_d8 = (longlong *******)0x0;
        uStack_c8 = (longlong *******)0x0;
        uStack_c0 = (longlong *******)0x1;
        uStack_b8 = (longlong *******)0x0;
        FUN_140f79650(&UNK_142a16b08,1,&uStack_c8,&uStack_138,&uStack_e8);
        if (((ppppppplStack_70 == (longlong *******)0x0) &&
            (ppppppplStack_f8 == (longlong *******)0x0)) &&
           (ppppppplStack_d8 == (longlong *******)0x0)) {
          *param_1 = 0x8000000000000004;
          FUN_1405f8f70(&uStack_e8);
          FUN_1405f8f70(&ppppppplStack_108);
          FUN_1405f8f70(&ppppppplStack_80);
          uStack_49 = 0;
          FUN_14004e480(&uStack_138,&ppppppplStack_118,0x18,0x10);
          ppppppplVar2 = ppppppplStack_68;
          ppppppplVar3 = ppppppplStack_60;
        }
        else {
          uStack_b8 = ppppppplStack_70;
          ppppppplVar2 = uStack_b8;
          uStack_c8 = ppppppplStack_80;
          uStack_c0 = ppppppplStack_78;
          ppppppplStack_a0 = ppppppplStack_f8;
          uStack_b0 = ppppppplStack_108;
          ppppppplVar10 = uStack_b0;
          ppppppplStack_a8 = ppppppplStack_100;
          ppppppplStack_98 = uStack_e8;
          ppppppplStack_90 = uStack_e0;
          ppppppplStack_88 = ppppppplStack_d8;
          param_1[8] = ppppppplStack_d8;
          *(undefined4 *)(param_1 + 6) = (undefined4)uStack_e8;
          *(undefined4 *)((longlong)param_1 + 0x34) = uStack_e8._4_4_;
          *(undefined4 *)(param_1 + 7) = (undefined4)uStack_e0;
          *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_e0._4_4_;
          *param_1 = ppppppplStack_80;
          param_1[1] = ppppppplStack_78;
          param_1[4] = ppppppplStack_100;
          param_1[5] = ppppppplStack_f8;
          uStack_b8._0_4_ = SUB84(ppppppplStack_70,0);
          uStack_b8._4_4_ = (undefined4)((ulonglong)ppppppplStack_70 >> 0x20);
          uStack_b0._0_4_ = (int)ppppppplStack_108;
          uStack_b0._4_4_ = (undefined4)((ulonglong)ppppppplStack_108 >> 0x20);
          *(undefined4 *)(param_1 + 2) = (undefined4)uStack_b8;
          *(undefined4 *)((longlong)param_1 + 0x14) = uStack_b8._4_4_;
          *(int *)(param_1 + 3) = (int)uStack_b0;
          *(undefined4 *)((longlong)param_1 + 0x1c) = uStack_b0._4_4_;
          uStack_49 = 0;
          uStack_b8 = ppppppplVar2;
          uStack_b0 = ppppppplVar10;
          FUN_14004e480(&uStack_138,&ppppppplStack_118,0x18,0x10);
          ppppppplVar2 = ppppppplStack_68;
          ppppppplVar3 = ppppppplStack_60;
        }
      }
      else {
        *param_1 = 0x8000000000000003;
        *(int *)(param_1 + 1) = iStack_120;
        if (uStack_128 != 0) {
          plVar12 = uStack_130 + 4;
          lVar11 = uStack_128;
          do {
            if (plVar12[-4] != 0) {
              func_0x000140613c20(plVar12[-3],plVar12[-4],1);
            }
            if (plVar12[-1] != 0) {
              func_0x000140613c20(*plVar12,plVar12[-1],1);
            }
            plVar12 = plVar12 + 6;
            lVar11 = lVar11 + -1;
          } while (lVar11 != 0);
        }
        if (CONCAT44(uStack_154,uStack_158) != 0) {
          func_0x000140613c20(plVar4,CONCAT44(uStack_154,uStack_158) * 0x30,8);
        }
      }
      goto joined_r0x00014067cc49;
    }
    plStack_170 = uStack_130;
    ppppppplStack_80 = (longlong *******)0x0;
    ppppppplStack_78 = (longlong *******)0x1;
    ppppppplStack_70 = (longlong *******)0x0;
    uStack_c8 = (longlong *******)0x0;
    uStack_b8 = (longlong *******)0x0;
    ppppppplStack_a8 = (longlong *******)0x20;
    ppppppplStack_a0 = (longlong *******)CONCAT71(ppppppplStack_a0._1_7_,3);
    ppppppplStack_98 = (longlong *******)&ppppppplStack_80;
    ppppppplStack_90 = (longlong *******)&UNK_142a169e0;
    cVar6 = FUN_140b87050(&plStack_170,&uStack_c8);
    if (cVar6 != '\0') {
      FUN_1428d9760(&UNK_142a16a10,0x37,&uStack_4a,&UNK_142a16ab0,&UNK_142a16a98);
      goto LAB_14067cd5d;
    }
    uStack_e8 = ppppppplStack_80;
    uStack_e0 = ppppppplStack_78;
    ppppppplStack_d8 = ppppppplStack_70;
    plStack_58 = plVar4;
    if (*plVar4 == 1) {
      FUN_1405e7a00(plVar4 + 1);
    }
    else if ((*plVar4 == 0) && (plVar4[2] != 0)) {
      func_0x000140613c20(plVar4[1],plVar4[2],1);
    }
    func_0x000140613c20(plStack_58,0x28,8);
    ppppppplStack_f8 = ppppppplStack_d8;
    uStack_f0 = uStack_d0;
    ppppppplStack_108 = uStack_e8;
    ppppppplStack_100 = uStack_e0;
    param_1[3] = ppppppplStack_d8;
    param_1[4] = uStack_d0;
    *(undefined4 *)(param_1 + 1) = (undefined4)uStack_e8;
    *(undefined4 *)((longlong)param_1 + 0xc) = uStack_e8._4_4_;
    *(undefined4 *)(param_1 + 2) = (undefined4)uStack_e0;
    *(undefined4 *)((longlong)param_1 + 0x14) = uStack_e0._4_4_;
    uVar7 = 0x8000000000000002;
  }
  else {
    uVar7 = 0x8000000000000000;
  }
  *param_1 = uVar7;
  ppppppplVar2 = ppppppplStack_68;
  ppppppplVar3 = ppppppplStack_60;
joined_r0x00014067cc49:
  if (ppppppplVar2 != (longlong *******)0x0) {
    func_0x000140613c20(ppppppplVar3,ppppppplVar2,1);
  }
  return param_1;
}

