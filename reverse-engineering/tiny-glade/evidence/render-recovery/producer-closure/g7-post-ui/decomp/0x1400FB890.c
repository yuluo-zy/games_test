
void FUN_1400fb890(undefined8 param_1,undefined8 param_2,longlong param_3,longlong param_4,
                  undefined4 param_5,undefined4 param_6)

{
  undefined8 uVar1;
  code *pcVar2;
  longlong lVar3;
  ulonglong uVar4;
  uint *puVar5;
  int iVar6;
  longlong unaff_RDI;
  uint uVar7;
  uint uVar8;
  int iVar9;
  undefined8 uStack_438;
  undefined8 uStack_430;
  undefined4 uStack_428;
  undefined4 uStack_424;
  undefined4 uStack_420;
  undefined4 uStack_41c;
  undefined4 uStack_418;
  undefined4 uStack_414;
  undefined4 uStack_410;
  undefined4 uStack_40c;
  undefined4 uStack_408;
  undefined4 uStack_404;
  undefined4 uStack_400;
  undefined4 uStack_2a8;
  undefined4 uStack_2a4;
  undefined8 uStack_2a0;
  undefined8 uStack_298;
  undefined8 uStack_290;
  undefined8 uStack_288;
  longlong lStack_260;
  uint *puStack_258;
  undefined8 uStack_250;
  undefined4 auStack_118 [2];
  undefined4 uStack_110;
  undefined4 uStack_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined8 *puStack_f8;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  int iStack_ac;
  int iStack_a8;
  undefined4 uStack_a4;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  uint uStack_50;
  undefined1 uStack_4b;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  auStack_118[0] = param_5;
  uStack_110 = param_6;
  uStack_50 = (uint)*(byte *)(param_3 + 0x1c);
  if (uStack_50 < 0xe) {
    unaff_RDI = param_3;
    if (*(int *)(param_3 + 8) != 0x7a) goto code_r0x0001400fbdf8;
    uStack_438 = auStack_118;
    uStack_430 = &UNK_140109e90;
    uStack_2a8 = 0x4294cb08;
    uStack_2a4 = 1;
    uStack_2a0 = (undefined *)0x1;
    uStack_298 = &uStack_438;
    uStack_290 = 1;
    uStack_288 = 0;
    FUN_140d79f90(&uStack_98,&uStack_2a8);
    uStack_a0 = CONCAT44(uStack_8c,uStack_90);
    lVar3 = FUN_140c0f3f0(uStack_a0,CONCAT44(uStack_84,uStack_88));
    uVar1 = *(undefined8 *)(lVar3 + -8);
    FUN_140bd4e20(&uStack_438,0x18,0x20);
    FUN_1404f62d0(&uStack_2a8,param_1,lVar3,uVar1,&uStack_438);
    if ((char)uStack_290 != '\x02') {
      uStack_428 = (undefined4)uStack_298;
      uStack_424 = uStack_298._4_4_;
      uStack_420 = (undefined4)uStack_290;
      uStack_41c = uStack_290._4_4_;
      uStack_438._0_4_ = uStack_2a8;
      uStack_438._4_4_ = uStack_2a4;
      uStack_430 = uStack_2a0;
      uStack_108 = uStack_2a8;
      uStack_104 = uStack_2a4;
      uStack_100 = (undefined4)uStack_2a0;
      uStack_fc = uStack_2a0._4_4_;
      puStack_f8 = uStack_298;
      if (uStack_98 != 0) {
        func_0x000140613c20(uStack_a0,uStack_98,1);
      }
      uVar4 = func_0x000140bd5790(param_3);
      iVar9 = (((uint)(uVar4 >> 6) & 0x3ffffff) + 1) - (uint)((uVar4 & 0x3f) == 0);
      iVar6 = ((uint)(uVar4 >> 0x26) + 1) - (uint)((uVar4 & 0x3f00000000) == 0);
      uStack_a4 = 1;
      iStack_ac = iVar9;
      iStack_a8 = iVar6;
      FUN_140587eb0(&uStack_2a8,&UNK_14294cb68,0xd,&UNK_14294cb75,0x16,param_2,param_1);
      uStack_4b = 1;
      func_0x000140bd57a0(&uStack_438);
      uStack_4b = 1;
      func_0x000140bd58e0(&uStack_98,&uStack_438,0);
      uStack_4b = 1;
      func_0x000140bd5920(&uStack_438,&uStack_98,1,1);
      uStack_98._0_4_ = (undefined4)uStack_438;
      uStack_98._4_4_ = uStack_438._4_4_;
      uStack_90 = (undefined4)uStack_430;
      uStack_8c = uStack_430._4_4_;
      uStack_88 = uStack_428;
      uStack_84 = uStack_424;
      uStack_80 = uStack_420;
      uStack_7c = uStack_41c;
      uStack_78 = uStack_418;
      uStack_74 = uStack_414;
      uStack_70 = uStack_410;
      uStack_6c = uStack_40c;
      uStack_68 = uStack_408;
      uStack_64 = uStack_404;
      uStack_60 = uStack_400;
      uStack_e8 = param_4;
      FUN_1402af750(&uStack_e8,&uStack_2a8,0xb466b30ac6fc0292,0x37,&uStack_98);
      uVar8 = uStack_50;
      func_0x000142923c00(&uStack_438,&uStack_2a8,400);
      uStack_49 = 1;
      func_0x000140bd57a0(&uStack_2a8);
      uStack_49 = 1;
      func_0x000140bd59f0(&uStack_98,&uStack_2a8,1);
      uStack_49 = 1;
      func_0x000140bd58e0(&uStack_2a8,&uStack_98,1);
      uStack_49 = 1;
      func_0x000140bd57e0(&uStack_98,&uStack_2a8,5);
      uStack_e8 = CONCAT44(uStack_98._4_4_,(undefined4)uStack_98);
      uStack_e0 = uStack_90;
      uStack_dc = uStack_8c;
      uStack_d8 = uStack_88;
      uStack_d4 = uStack_84;
      uStack_d0 = uStack_80;
      uStack_cc = uStack_7c;
      uStack_c8 = uStack_78;
      uStack_c4 = uStack_74;
      uStack_c0 = uStack_70;
      uStack_bc = uStack_6c;
      uStack_b8 = uStack_68;
      uStack_b4 = uStack_64;
      uStack_b0 = uStack_60;
      uStack_98 = param_4;
      FUN_1402af750(&uStack_98,&uStack_438,0xed9d1f0a5683f687,0x37,&uStack_e8);
      func_0x000142923c00(&uStack_2a8,&uStack_438,400);
      uStack_4a = 1;
      func_0x000140bd57a0(&uStack_438);
      uStack_4a = 1;
      func_0x000140bd57e0(&uStack_98,&uStack_438,5);
      uVar8 = uVar8 - 1;
      uVar7 = 6;
      if (uVar8 < 6) {
        uVar7 = uVar8;
      }
      uStack_4a = 1;
      func_0x000140bd58e0(&uStack_438,&uStack_98,uVar7);
      uStack_4a = 1;
      func_0x000140bd5920(&uStack_98,&uStack_438,1,1);
      uStack_e8 = uStack_98;
      uStack_e0 = uStack_90;
      uStack_dc = uStack_8c;
      uStack_d8 = uStack_88;
      uStack_d4 = uStack_84;
      uStack_d0 = uStack_80;
      uStack_cc = uStack_7c;
      uStack_c8 = uStack_78;
      uStack_c4 = uStack_74;
      uStack_c0 = uStack_70;
      uStack_bc = uStack_6c;
      uStack_b8 = uStack_68;
      uStack_b4 = uStack_64;
      uStack_b0 = uStack_60;
      uStack_98 = param_4;
      FUN_1402af750(&uStack_98,&uStack_2a8,0x45f26f91023c2ec3,0x37,&uStack_e8);
      func_0x000142923c00(&uStack_438,&uStack_2a8,400);
      FUN_14096dd60(&uStack_2a8,&uStack_438,0x68d4413b4de4f24,&uStack_108);
      puVar5 = (uint *)func_0x000140613c10(0x10,1);
      if (puVar5 != (uint *)0x0) {
        *puVar5 = uVar8;
        puVar5[1] = iVar6 * iVar9;
        puVar5[2] = 0;
        puVar5[3] = 0;
        if (lStack_260 != 0) {
          func_0x000140613c20(puStack_258,lStack_260,1);
        }
        lStack_260 = 0x10;
        uStack_250 = 0x10;
        puStack_258 = puVar5;
        func_0x000142923c00(&uStack_438,&uStack_2a8,400);
        FUN_14096ee10(&uStack_438,&iStack_ac);
        return;
      }
      FUN_1428d8fe3(1,0x10,&UNK_1429fd088);
      goto code_r0x0001400fbe76;
    }
  }
  else {
    FUN_1428d9430(&UNK_14294ca14,0x2f,&UNK_14294cac0);
code_r0x0001400fbdf8:
    uStack_2a8 = 0;
    uStack_2a4 = 0;
    FUN_1428a9914(0,unaff_RDI + 8,&UNK_14294cad8,&uStack_2a8,&UNK_14294cae0);
  }
  uStack_438._0_4_ = uStack_2a8;
  uStack_438._4_4_ = uStack_2a4;
  FUN_1428d9760(&UNK_14294cb18,0x35,&uStack_438,&UNK_14294cb90,&UNK_14294cb50);
code_r0x0001400fbe76:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

