
undefined8 * FUN_140bff5a0(undefined8 *param_1,longlong *param_2,undefined8 param_3)

{
  char cVar1;
  longlong lVar2;
  longlong lVar3;
  code *pcVar4;
  undefined8 uVar5;
  char *pcVar6;
  int iVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  bool bVar10;
  undefined8 auStack_160 [2];
  undefined *puStack_150;
  undefined8 uStack_148;
  undefined4 uStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined8 uStack_130;
  undefined2 uStack_128;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_118;
  undefined4 uStack_114;
  undefined8 uStack_110;
  undefined4 auStack_108 [2];
  undefined8 uStack_100;
  undefined4 uStack_f8;
  undefined8 uStack_f0;
  undefined8 uStack_e8;
  undefined4 uStack_e0;
  undefined8 uStack_d8;
  int iStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
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
  undefined4 uStack_90;
  undefined4 uStack_8c;
  int iStack_88;
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
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined8 uStack_48;
  char *pcStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  auStack_108[0] = 0xc;
  uStack_100 = 0;
  uStack_f8 = 0;
  uStack_e8 = 2;
  uStack_e0 = 0;
  uStack_d8 = 0;
  lVar2 = *param_2;
  uStack_f0 = param_3;
  iVar7 = (**(code **)(lVar2 + 0x180))(*(undefined8 *)(lVar2 + 0x640),auStack_108,0,&iStack_d0);
  if (iVar7 == 0) {
    uVar5 = CONCAT44(uStack_cc,iStack_d0);
    (**(code **)(lVar2 + 0xe8))(*(undefined8 *)(lVar2 + 0x640),uVar5,&uStack_120);
    lVar3 = *(longlong *)(lVar2 + 0x678);
    pcStack_40 = (char *)(lVar3 + 0x10);
    LOCK();
    bVar10 = *(char *)(lVar3 + 0x10) == '\0';
    if (bVar10) {
      *(char *)(lVar3 + 0x10) = '\x01';
    }
    UNLOCK();
    if (!bVar10) {
      FUN_1428cd210(pcStack_40);
    }
    FUN_140c81c70(pcStack_40);
    puStack_150 = &UNK_142b172f8;
    uStack_148 = 6;
    uStack_140 = uStack_120;
    uStack_13c = uStack_11c;
    uStack_138 = uStack_118;
    uStack_134 = uStack_114;
    uStack_130 = uStack_110;
    uStack_128 = 0x401;
    auStack_160[0] = 2;
    FUN_140c0da40(&iStack_d0,lVar3 + 0x18,auStack_160);
    pcVar6 = pcStack_40;
    if ((char)uStack_8c != '\x02') {
      uStack_48 = CONCAT44(uStack_8c,uStack_90);
      uStack_58 = uStack_a0;
      uStack_54 = uStack_9c;
      uStack_50 = uStack_98;
      uStack_4c = uStack_94;
      uStack_68 = uStack_b0;
      uStack_64 = uStack_ac;
      uStack_60 = uStack_a8;
      uStack_5c = uStack_a4;
      uStack_78 = uStack_c0;
      uStack_74 = uStack_bc;
      uStack_70 = uStack_b8;
      uStack_6c = uStack_b4;
      iStack_88 = iStack_d0;
      uStack_84 = uStack_cc;
      uStack_80 = uStack_c8;
      uStack_7c = uStack_c4;
      FUN_140c81e10();
      LOCK();
      cVar1 = *pcVar6;
      if (cVar1 == '\x01') {
        *pcVar6 = '\0';
      }
      UNLOCK();
      if (cVar1 != '\x01') {
        FUN_1428cdb90(pcVar6,0);
      }
      uVar8 = func_0x0001406d0cb0(&iStack_88);
      uVar9 = func_0x0001406868b0(&iStack_88);
      iStack_d0 = (**(code **)(lVar2 + 0xd8))(*(undefined8 *)(lVar2 + 0x640),uVar5,uVar8,uVar9);
      if (iStack_d0 == 0) {
        param_1[9] = uStack_48;
        param_1[7] = CONCAT44(uStack_54,uStack_58);
        param_1[8] = CONCAT44(uStack_4c,uStack_50);
        param_1[5] = CONCAT44(uStack_64,uStack_68);
        param_1[6] = CONCAT44(uStack_5c,uStack_60);
        param_1[3] = CONCAT44(uStack_74,uStack_78);
        param_1[4] = CONCAT44(uStack_6c,uStack_70);
        param_1[1] = CONCAT44(uStack_84,iStack_88);
        param_1[2] = CONCAT44(uStack_7c,uStack_80);
        *param_1 = uVar5;
        return param_1;
      }
      FUN_1428d9760(&UNK_142b15888,0x12,&iStack_d0,&UNK_142b15128,&UNK_142b17318);
      goto code_r0x000140bff840;
    }
  }
  else {
    iStack_d0 = iVar7;
    FUN_1428d9760(&UNK_142b157f0,0xd,&iStack_d0,&UNK_142b15128,&UNK_142b172e0);
  }
  uStack_78 = uStack_c0;
  uStack_74 = uStack_bc;
  uStack_70 = uStack_b8;
  uStack_6c = uStack_b4;
  iStack_88 = iStack_d0;
  uStack_84 = uStack_cc;
  uStack_80 = uStack_c8;
  uStack_7c = uStack_c4;
  FUN_1428d9760(&UNK_142b15188,0x2b,&iStack_88,&UNK_142b15148,&UNK_142b17300);
code_r0x000140bff840:
                    /* WARNING: Does not return */
  pcVar4 = (code *)invalidInstructionException();
  (*pcVar4)();
}

