
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined1 * FUN_1417c0cf0(undefined1 *param_1,longlong param_2)

{
  float fVar1;
  char cVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  longlong lVar7;
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
  undefined4 uStack_ac;
  int iStack_9c;
  undefined8 uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 uStack_80;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined8 uStack_70;
  undefined1 uStack_61;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  iVar4 = *(int *)(param_2 + 0x28);
  fVar1 = *(float *)(param_2 + 0x30);
  iVar5 = *(int *)(param_2 + 0x34);
  iVar6 = *(int *)(param_2 + 0x40);
  cVar2 = *(char *)(param_2 + 0x4b);
  cVar3 = *(char *)(param_2 + 0x4e);
  iStack_9c = *(int *)(param_2 + 0x44);
  uStack_98 = -0x8000000000000000;
  uStack_80 = 0;
  uStack_70 = 0;
  uStack_61 = 1;
  lVar7 = FUN_141593290(&uStack_98,&UNK_142ce172d,3,param_2);
  if (lVar7 == 0) {
    uStack_61 = 1;
    lVar7 = FUN_141598030(&uStack_98,&UNK_142ce1730,2,param_2 + 0x4a);
    if (lVar7 == 0) {
      uStack_61 = 1;
      lVar7 = FUN_141595530(&uStack_98,&UNK_142ce1732,4,param_2 + 0x48);
      if (lVar7 == 0) {
        if (iVar4 != 0) {
          uStack_61 = 1;
          lVar7 = FUN_141599250(&uStack_98,&UNK_142ce1736,8,param_2 + 0x28);
          if (lVar7 != 0) goto code_r0x0001417c0dc1;
        }
        uStack_61 = 1;
        lVar7 = FUN_141599a30(&uStack_98,&UNK_142ce173e,4,param_2 + 0x2c);
        if (lVar7 == 0) {
          if (*(char *)(param_2 + 0x49) != '\0') {
            uStack_61 = 1;
            lVar7 = FUN_141597e60(&uStack_98,&UNK_142ce1742,3);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if ((fVar1 != 0.0) || (NAN(fVar1))) {
            uStack_61 = 1;
            lVar7 = FUN_141595a20(&uStack_98,&UNK_142ce1745,8,param_2 + 0x30);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if (*(char *)(param_2 + 0x4c) == '\0') {
            uStack_61 = 1;
            lVar7 = FUN_141599080(&uStack_98,&UNK_142ce174d,0x10);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if (iVar5 != 0) {
            uStack_61 = 1;
            lVar7 = FUN_141599250(&uStack_98,&UNK_142ce175d,0x12,param_2 + 0x34);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if ((*(float *)(param_2 + 0x38) != _UNK_142925904) ||
             (NAN(*(float *)(param_2 + 0x38)) || NAN(_UNK_142925904))) {
            uStack_61 = 1;
            lVar7 = FUN_141595a20(&uStack_98,&UNK_142ce176f,0x12);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if (*(char *)(param_2 + 0x4d) == '\0') {
            uStack_61 = 1;
            lVar7 = FUN_141599080(&uStack_98,&UNK_142ce1781,0xd);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if ((*(float *)(param_2 + 0x3c) != _UNK_142925904) ||
             (NAN(*(float *)(param_2 + 0x3c)) || NAN(_UNK_142925904))) {
            uStack_61 = 1;
            lVar7 = FUN_141595a20(&uStack_98,&UNK_142ce178e,0xb);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if (iVar6 != 0) {
            uStack_61 = 1;
            lVar7 = FUN_141599250(&uStack_98,&UNK_142ce1799,0xc,param_2 + 0x40);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if (cVar2 != '\0') {
            uStack_61 = 1;
            lVar7 = FUN_141592dd0(&uStack_98,&UNK_142ce17a5,7,param_2 + 0x4b);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if (cVar3 != '\0') {
            uStack_61 = 1;
            lVar7 = FUN_141595090(&uStack_98,&UNK_142ce17ac,0xf,param_2 + 0x4e);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          if (iStack_9c != 0) {
            uStack_61 = 1;
            lVar7 = FUN_141599250(&uStack_98,&UNK_142ce17bb,0xb,param_2 + 0x44);
            if (lVar7 != 0) goto code_r0x0001417c0dc1;
          }
          uStack_b8 = uStack_78;
          uStack_b4 = uStack_74;
          uStack_b0 = (undefined4)uStack_70;
          uStack_ac = uStack_70._4_4_;
          uStack_c8 = uStack_88;
          uStack_c4 = uStack_84;
          uStack_c0 = (undefined4)uStack_80;
          uStack_bc = uStack_80._4_4_;
          uStack_d8 = (undefined4)uStack_98;
          uStack_d4 = uStack_98._4_4_;
          uStack_d0 = uStack_90;
          uStack_cc = uStack_8c;
          uStack_61 = 0;
          FUN_140b864c0(param_1,&uStack_d8);
          return param_1;
        }
      }
    }
  }
code_r0x0001417c0dc1:
  *(longlong *)(param_1 + 8) = lVar7;
  *param_1 = 6;
  FUN_140898f40(&uStack_80);
  if ((!SBORROW8(0,uStack_98)) && (uStack_98 != 0)) {
    func_0x000140613c20(CONCAT44(uStack_8c,uStack_90),uStack_98,1);
  }
  return param_1;
}

