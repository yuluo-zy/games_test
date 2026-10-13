
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_140620470(undefined8 *param_1,longlong param_2,ulonglong *param_3)

{
  ulonglong *puVar1;
  code *pcVar2;
  undefined4 uVar3;
  longlong lVar4;
  undefined8 *******pppppppuVar5;
  undefined4 *puVar6;
  ulonglong uVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  undefined8 *puVar10;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined4 uStack_158;
  undefined4 uStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  undefined8 uStack_148;
  undefined8 ******ppppppuStack_140;
  ulonglong uStack_138;
  undefined8 *******pppppppuStack_130;
  undefined4 uStack_128;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined8 *******pppppppuStack_118;
  code *pcStack_110;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 *******pppppppuStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined8 uStack_e0;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined8 uStack_c8;
  undefined *puStack_c0;
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined *puStack_a8;
  undefined8 uStack_a0;
  undefined8 *******pppppppuStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 *******pppppppuStack_78;
  code *pcStack_70;
  ulonglong uStack_68;
  ulonglong uStack_58;
  undefined8 *puStack_50;
  undefined8 *******pppppppuStack_48;
  undefined1 uStack_3b;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  puVar1 = _DAT_14362c3a8;
  uStack_38 = 0xfffffffffffffffe;
  switch(*param_3 ^ 0x8000000000000000) {
  case 0:
    *param_1 = 0x8000000000000001;
    param_1[1] = 0;
    return param_1;
  default:
    uStack_178 = (undefined4)*param_3;
    uStack_174 = *(undefined4 *)((longlong)param_3 + 4);
    uStack_170 = (undefined4)param_3[1];
    uStack_16c = *(undefined4 *)((longlong)param_3 + 0xc);
    uStack_168 = (undefined4)param_3[2];
    uStack_164 = *(undefined4 *)((longlong)param_3 + 0x14);
    uStack_160 = (undefined4)param_3[3];
    uStack_15c = *(undefined4 *)((longlong)param_3 + 0x1c);
    lVar4 = FUN_140dbf680(&uStack_178);
    if (((uint)lVar4 & 3) != 1) goto LAB_140620e22;
    uStack_58 = lVar4 - 1;
    puStack_50 = *(undefined8 **)(lVar4 + -1);
    pppppppuStack_48 = *(undefined8 ********)(lVar4 + 7);
    if (*pppppppuStack_48 != (undefined8 ******)0x0) {
      (*(code *)*pppppppuStack_48)(puStack_50);
    }
    break;
  case 2:
    lVar4 = func_0x000140b7f2d0(&UNK_142a0b407,0x22);
    if (((uint)lVar4 & 3) != 1) goto LAB_140620e22;
    uStack_58 = lVar4 - 1;
    puStack_50 = *(undefined8 **)(lVar4 + -1);
    pppppppuStack_48 = *(undefined8 ********)(lVar4 + 7);
    if (*pppppppuStack_48 != (undefined8 ******)0x0) {
      (*(code *)*pppppppuStack_48)(puStack_50);
    }
    break;
  case 3:
    pppppppuStack_98 = *(undefined8 ********)(param_2 + 0xa0);
    puStack_a8 = *(undefined **)(param_2 + 0x90);
    uStack_a0 = *(undefined8 *)(param_2 + 0x98);
    uStack_b8 = *(undefined8 *)(param_2 + 0x80);
    uStack_b0 = *(undefined8 *)(param_2 + 0x88);
    pppppppuStack_f8 = *(undefined8 ********)(param_2 + 0x40);
    uStack_f0 = *(code **)(param_2 + 0x48);
    uStack_e8 = *(undefined8 *)(param_2 + 0x50);
    uStack_e0 = *(undefined8 *)(param_2 + 0x58);
    uStack_d8 = *(undefined4 *)(param_2 + 0x60);
    uStack_d4 = *(undefined4 *)(param_2 + 100);
    uStack_d0 = *(undefined4 *)(param_2 + 0x68);
    uStack_cc = *(undefined4 *)(param_2 + 0x6c);
    uStack_c8 = *(undefined8 *)(param_2 + 0x70);
    puStack_c0 = *(undefined **)(param_2 + 0x78);
    *(undefined8 *)(param_2 + 0x40) = 0x8000000000000010;
    if (pppppppuStack_f8 + 0xffffffffffffffe == (undefined8 *******)0x1 ||
        (undefined8 ******)0x3 < pppppppuStack_f8 + 0xffffffffffffffe) {
      uStack_58 = uStack_a0;
      pppppppuStack_48 = pppppppuStack_98;
      (**(code **)((longlong)pppppppuStack_98 + 0x18))(&pppppppuStack_118);
      if (*(longlong *)((longlong)pppppppuStack_48 + 8) != 0) {
        func_0x000140613c20(uStack_58,*(longlong *)((longlong)pppppppuStack_48 + 8),
                            *(undefined8 *)((longlong)pppppppuStack_48 + 0x10));
      }
      uStack_68 = CONCAT44(uStack_104,uStack_108);
      pppppppuStack_78 = pppppppuStack_118;
      pcStack_70 = pcStack_110;
      if (pppppppuStack_f8 + 0xffffffffffffffe < (undefined8 ******)0x4 &&
          pppppppuStack_f8 + 0xffffffffffffffe != (undefined8 *******)0x1) {
        puStack_50 = (undefined8 *)param_2;
        FUN_14061ea80(&pppppppuStack_f8);
      }
      else {
        puStack_50 = (undefined8 *)param_2;
        if ((pppppppuStack_f8 < (undefined8 ******)0x8000000000000005) &&
           (2 < (longlong)pppppppuStack_f8 + 0x7ffffffffffffff3U ||
            (longlong)pppppppuStack_f8 + 0x7ffffffffffffff3U == 1)) {
          FUN_140dba290(&pppppppuStack_f8);
        }
      }
      FUN_14061ea80(param_2 + 0x40);
      *(undefined8 *)((longlong)puStack_50 + 0x40) = 0x8000000000000012;
      *(undefined8 ********)((longlong)puStack_50 + 0x48) = pppppppuStack_78;
      *(code **)((longlong)puStack_50 + 0x50) = pcStack_70;
      *(ulonglong *)((longlong)puStack_50 + 0x58) = uStack_68;
      goto LAB_140620e22;
    }
    FUN_14061ea80(&pppppppuStack_f8);
    FUN_1428d9430(&UNK_142a0b429,0x28,&UNK_142a0b478);
    goto LAB_140620e63;
  case 4:
    uVar7 = *(ulonglong *)(param_2 + 0x40);
    if (uVar7 + 0x7ffffffffffffff0 == 1 || 3 < uVar7 + 0x7ffffffffffffff0) {
      puStack_50 = (undefined8 *)(param_2 + 0x40);
      if ((uVar7 < 0x8000000000000005) &&
         (2 < uVar7 + 0x7ffffffffffffff3 || uVar7 + 0x7ffffffffffffff3 == 1)) {
        FUN_140dba290(puStack_50);
      }
      *puStack_50 = 0x800000000000000f;
    }
    goto LAB_140620e22;
  case 5:
    pppppppuStack_48 = (undefined8 *******)param_3[1];
    uStack_58 = param_3[2];
    uVar7 = param_3[3];
    uStack_3b = 0;
    FUN_14062eb60(&pppppppuStack_118,&uStack_3a);
    pppppppuStack_78 = pppppppuStack_48;
    pcStack_70 = (code *)uStack_58;
    uStack_e8 = CONCAT44(uStack_104,uStack_108);
    uStack_e0 = CONCAT44(uStack_fc,uStack_100);
    pppppppuStack_f8 = pppppppuStack_118;
    uStack_f0 = pcStack_110;
    uStack_3b = 1;
    pppppppuVar5 = &pppppppuStack_f8;
    uStack_68 = uVar7;
    FUN_140dc0160(&pppppppuStack_130,pppppppuVar5,&pppppppuStack_78);
    if (pppppppuStack_130 == (undefined8 *******)0x8000000000000004) {
      pppppppuStack_130 = (undefined8 *******)0x0;
      uVar3 = (int)pppppppuVar5;
LAB_14062090b:
      uVar8 = uVar3;
      uStack_39 = 0;
      pppppppuVar5 = (undefined8 *******)FUN_140d635e0();
    }
    else {
      uStack_f0 = (code *)CONCAT44(uStack_124,uStack_128);
      uStack_e8 = CONCAT44(uStack_11c,uStack_120);
      pppppppuStack_f8 = pppppppuStack_130;
      uStack_3b = 1;
      pppppppuVar5 = (undefined8 *******)FUN_1428dabd0(&pppppppuStack_f8);
      pppppppuStack_130 = pppppppuVar5;
      pppppppuStack_78 = &pppppppuStack_130;
      if (*_DAT_14362c3a8 == 0) {
        uVar8 = 1000000000;
      }
      else {
        pcStack_110 = (code *)&UNK_1405b7a50;
        uStack_39 = 1;
        pppppppuStack_118 = &pppppppuStack_78;
        puVar6 = (undefined4 *)func_0x00014067dd80(&UNK_142a0b4b8);
        uStack_d8 = *puVar6;
        uStack_d4 = puVar6[1];
        uStack_d0 = puVar6[2];
        uStack_cc = puVar6[3];
        uStack_c8 = 1;
        puStack_c0 = &UNK_142a0b4d0;
        uStack_b8 = 0x12;
        puStack_a8 = &UNK_142a0b4a8;
        uStack_a0 = 1;
        uStack_90 = 1;
        uStack_88 = 0;
        pppppppuStack_f8 = (undefined8 *******)0x0;
        uStack_f0 = (code *)&UNK_142a0b4d0;
        uStack_e8 = 0x12;
        uStack_e0 = 0;
        uStack_b0 = CONCAT44(puVar6[4],1);
        uStack_39 = 1;
        pppppppuStack_98 = &pppppppuStack_118;
        func_0x000140d4a330(&uStack_3a,&pppppppuStack_f8);
        uVar8 = 1000000000;
        pppppppuVar5 = pppppppuStack_130;
        uVar3 = 1000000000;
        if (pppppppuStack_130 == (undefined8 *******)0x0) goto LAB_14062090b;
      }
    }
    uStack_58 = CONCAT44(uStack_58._4_4_,uVar8);
    puStack_50 = (undefined8 *)param_2;
    pppppppuStack_48 = pppppppuVar5;
    if (*(int *)(param_2 + 0xb0) == 1000000000) {
      func_0x000140d4cc30(param_2 + 0xa8);
    }
    *(undefined8 ********)((longlong)puStack_50 + 0xa8) = pppppppuStack_48;
    *(undefined4 *)((longlong)puStack_50 + 0xb0) = (undefined4)uStack_58;
    goto LAB_140620e22;
  case 6:
    ppppppuStack_140 = (undefined8 ******)param_3[1];
    uStack_138 = param_3[2];
    if (2 < *_DAT_14362c3a8) {
      pppppppuStack_118 = &ppppppuStack_140;
      pcStack_110 = (code *)&UNK_1405338a0;
      puVar6 = (undefined4 *)func_0x00014067dd80(&UNK_142a0b500);
      uStack_d8 = *puVar6;
      uStack_d4 = puVar6[1];
      uStack_d0 = puVar6[2];
      uStack_cc = puVar6[3];
      uStack_c8 = 3;
      puStack_c0 = &UNK_142a0b4d0;
      uStack_b8 = 0x12;
      puStack_a8 = &UNK_142a0b4f0;
      uStack_a0 = 1;
      pppppppuStack_98 = &pppppppuStack_118;
      uStack_90 = 1;
      uStack_88 = 0;
      pppppppuStack_f8 = (undefined8 *******)0x0;
      uStack_f0 = (code *)&UNK_142a0b4d0;
      uStack_e8 = 0x12;
      uStack_e0 = 0;
      uStack_b0 = CONCAT44(puVar6[4],1);
      func_0x000140d4a330(&uStack_3a,&pppppppuStack_f8);
    }
    pppppppuStack_78 = (undefined8 *******)func_0x000140b7f2d0(ppppppuStack_140,uStack_138);
    if (pppppppuStack_78 == (undefined8 *******)0x0) goto LAB_140620e22;
    if (*puVar1 != 0) {
      pppppppuStack_118 = &pppppppuStack_78;
      pcStack_110 = FUN_140d5ace0;
      puVar6 = (undefined4 *)func_0x00014067dd80(&UNK_142a0b548);
      uStack_d8 = *puVar6;
      uStack_d4 = puVar6[1];
      uStack_d0 = puVar6[2];
      uStack_cc = puVar6[3];
      uStack_c8 = 1;
      puStack_c0 = &UNK_142a0b4d0;
      uStack_b8 = 0x12;
      puStack_a8 = &UNK_142a0b538;
      uStack_a0 = 1;
      pppppppuStack_98 = &pppppppuStack_118;
      uStack_90 = 1;
      uStack_88 = 0;
      pppppppuStack_f8 = (undefined8 *******)0x0;
      uStack_f0 = (code *)&UNK_142a0b4d0;
      uStack_e8 = 0x12;
      uStack_e0 = 0;
      uStack_b0 = CONCAT44(puVar6[4],1);
      func_0x000140d4a330(&uStack_3a,&pppppppuStack_f8);
    }
    if (((uint)pppppppuStack_78 & 3) != 1) goto LAB_140620e22;
    uStack_58 = (longlong)pppppppuStack_78 - 1;
    puStack_50 = *(undefined8 **)((longlong)pppppppuStack_78 + -1);
    pppppppuStack_48 = *(undefined8 ********)((longlong)pppppppuStack_78 + 7);
    if (*pppppppuStack_48 != (undefined8 ******)0x0) {
      (*(code *)*pppppppuStack_48)(puStack_50);
    }
    break;
  case 7:
    uVar7 = *(longlong *)(param_2 + 0x40) + 0x7ffffffffffffff0;
    if (uVar7 == 1 || 3 < uVar7) {
      uVar7 = *(longlong *)(param_2 + 0x40) + 0x7ffffffffffffff3;
      if ((uVar7 < 0xfffffffffffffff8) && (2 < uVar7 || uVar7 == 1)) {
        *(byte *)(param_2 + 0x90) = *(byte *)(param_2 + 0x90) ^ 1;
      }
    }
    goto LAB_140620e22;
  case 8:
    if ((*(longlong *)(param_2 + 0x40) != -0x7fffffffffffffee) ||
       (**(longlong **)(param_2 + 0x50) != 1)) goto LAB_140620e22;
    puVar1 = (ulonglong *)(param_2 + 0x40);
    pppppppuStack_98 = *(undefined8 ********)(param_2 + 0xa0);
    puStack_a8 = *(undefined **)(param_2 + 0x90);
    uStack_a0 = *(undefined8 *)(param_2 + 0x98);
    uStack_b8 = *(undefined8 *)(param_2 + 0x80);
    uStack_b0 = *(undefined8 *)(param_2 + 0x88);
    pppppppuStack_f8 = (undefined8 *******)*puVar1;
    uStack_f0 = *(code **)(param_2 + 0x48);
    uStack_e8 = *(undefined8 *)(param_2 + 0x50);
    uStack_e0 = *(undefined8 *)(param_2 + 0x58);
    uStack_d8 = *(undefined4 *)(param_2 + 0x60);
    uStack_d4 = *(undefined4 *)(param_2 + 100);
    uStack_d0 = *(undefined4 *)(param_2 + 0x68);
    uStack_cc = *(undefined4 *)(param_2 + 0x6c);
    uStack_c8 = *(undefined8 *)(param_2 + 0x70);
    puStack_c0 = *(undefined **)(param_2 + 0x78);
    *puVar1 = 0x8000000000000010;
    if (pppppppuStack_f8 == (undefined8 *******)0x8000000000000012) {
      puVar10 = (undefined8 *)(param_2 + 0x48);
      uStack_148 = *(undefined8 *)(param_2 + 0x58);
      uStack_158 = *(undefined4 *)puVar10;
      uStack_154 = *(undefined4 *)(param_2 + 0x4c);
      uStack_150 = *(undefined4 *)(param_2 + 0x50);
      uStack_14c = *(undefined4 *)(param_2 + 0x54);
      FUN_140dc0570(&pppppppuStack_118,&uStack_158);
      if (pppppppuStack_118 != (undefined8 *******)0x8000000000000001) {
        uStack_68 = CONCAT44(uStack_104,uStack_108);
        pppppppuStack_78 = pppppppuStack_118;
        pcStack_70 = pcStack_110;
        puStack_50 = puVar10;
        if (pppppppuStack_f8 != (undefined8 *******)0x8000000000000012) {
          FUN_14061ea80(&pppppppuStack_f8);
        }
        pppppppuStack_48 = (undefined8 *******)puVar1;
        FUN_14061ea80(puVar1);
        *pppppppuStack_48 = (undefined8 ******)0x8000000000000013;
        puStack_50[2] = uStack_68;
        *puStack_50 = pppppppuStack_78;
        puStack_50[1] = pcStack_70;
        goto LAB_140620e22;
      }
      goto LAB_140620e84;
    }
LAB_140620e63:
    FUN_14061ea80(&pppppppuStack_f8);
    FUN_1428d9430(&UNK_142a0b429,0x28,&UNK_142a0b578);
LAB_140620e84:
    FUN_1428d9760(&UNK_142a0b250,0x2b,&pppppppuStack_78,&UNK_142a0b230,&UNK_142a0b560);
                    /* WARNING: Does not return */
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  case 9:
    pppppppuStack_48 = (undefined8 *******)param_3[1];
    uStack_58 = param_3[2];
    uVar7 = *(longlong *)(param_2 + 0x40) + 0x7ffffffffffffff0;
    if (uVar7 == 1 || 3 < uVar7) {
      uVar7 = *(longlong *)(param_2 + 0x40) + 0x7ffffffffffffff3;
      if ((uVar7 < 0xfffffffffffffff8) && (2 < uVar7 || uVar7 == 1)) {
        FUN_140dbaed0(param_2 + 0x40,uStack_58,param_3[3]);
      }
    }
    if (pppppppuStack_48 == (undefined8 *******)0x0) goto LAB_140620e22;
    uVar9 = 1;
    pppppppuVar5 = pppppppuStack_48;
    goto LAB_140620b46;
  }
  if (pppppppuStack_48[1] != (undefined8 ******)0x0) {
    func_0x000140613c20(puStack_50,pppppppuStack_48[1],pppppppuStack_48[2]);
  }
  uVar9 = 8;
  pppppppuVar5 = (undefined8 *******)(undefined8 *)0x18;
LAB_140620b46:
  func_0x000140613c20(uStack_58,pppppppuVar5,uVar9);
LAB_140620e22:
  *param_1 = 0x800000000000001e;
  return param_1;
}

