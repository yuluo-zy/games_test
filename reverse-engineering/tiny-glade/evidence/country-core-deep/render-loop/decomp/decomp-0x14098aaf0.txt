
void FUN_14098aaf0(undefined8 param_1,longlong param_2)

{
  undefined4 uVar1;
  ulonglong *puVar2;
  code *pcVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined1 auStack_6d8 [400];
  undefined1 auStack_548 [400];
  undefined1 auStack_3b8 [400];
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined1 auStack_118 [38];
  char cStack_f2;
  longlong lStack_f0;
  undefined8 uStack_98;
  undefined8 uStack_90;
  undefined8 uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  undefined1 uStack_4a;
  undefined1 uStack_49;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  lVar6 = param_2 + 0x38;
  uVar8 = 0x8226cc786e679355;
  lVar4 = FUN_1400dd6f0(lVar6,0x8226cc786e679355,0x65cfda6a55a841ac);
  if (((lVar4 == 0) || (*(ulonglong *)(param_2 + 0x218) <= uVar8)) ||
     (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 == 0)) {
code_r0x00014098b0f6:
    FUN_1428d9310(&UNK_142ab8e80);
  }
  else {
    lVar4 = ~uVar8 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar4) == 0)
    goto code_r0x00014098b0f6;
    lVar4 = *(longlong *)(param_2 + 0x1e0) + lVar4;
    uVar10 = *(undefined8 *)(lVar4 + 0x10);
    uVar1 = *(undefined4 *)(param_2 + 0x5b0);
    FUN_1412d04c0(&uStack_228,uVar10,param_1);
    FUN_1409ddbb0(param_1,param_2,&uStack_228);
    *(undefined4 *)(lVar4 + 0x5c) = uVar1;
    lVar4 = FUN_1412c71b0(uVar10);
    if (lVar4 == 0) {
      FUN_1428d9310(&UNK_142ab8e98);
code_r0x00014098b12a:
      uStack_49 = 1;
      uStack_4a = 1;
      FUN_1428d9310(&UNK_142ab8ec8);
      goto code_r0x00014098b144;
    }
    uStack_49 = 1;
    uStack_4a = 1;
    uVar8 = 0x850dbd51eafef756;
    lVar5 = FUN_1400dd6f0(lVar6,0x850dbd51eafef756,0x85782a4b39854c35);
    if (((lVar5 != 0) && (uVar8 < *(ulonglong *)(param_2 + 0x218))) &&
       (uVar8 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar8 * 8), uVar8 != 0)) {
      lVar5 = ~uVar8 * 0x60;
      if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar5) != 0) {
        puVar2 = *(ulonglong **)(*(longlong *)(param_2 + 0x1e0) + lVar5 + 0x10);
        if ((char)puVar2[2] != '\0') {
          uVar8 = *puVar2;
          uVar7 = uVar8 & 0xffffffff;
          if (((uVar7 < *(ulonglong *)(param_2 + 0x10)) &&
              (*(int *)(*(longlong *)(param_2 + 8) + uVar7 * 0x14) == (int)(uVar8 >> 0x20))) &&
             (lVar5 = *(longlong *)(param_2 + 8) + uVar7 * 0x14, *(int *)(lVar5 + 4) != -1)) {
            uVar8 = (ulonglong)*(uint *)(lVar5 + 0xc);
            uStack_49 = 1;
            uStack_4a = 1;
            lVar6 = FUN_1409871a0(lVar6,0x3d6ebc022226fa52,0xb20558f07a23c39d);
            if ((lVar6 != 0) && (uVar8 < *(ulonglong *)(param_2 + 0x1b0))) {
              uStack_49 = 1;
              uStack_4a = 1;
              lVar6 = func_0x000140d00b50(uVar8 * 0x48 + *(longlong *)(param_2 + 0x1a8));
              if (lVar6 != 0) {
                if (*(longlong *)(lVar6 + 0x20) != 0) {
                  lVar6 = lVar6 + 0x20;
                  uStack_49 = 1;
                  uStack_4a = 1;
                  FUN_1412e3a60(&uStack_228,lVar4,lVar6);
                  uVar9 = uStack_220;
                  uVar10 = uStack_228;
                  uStack_98 = uStack_218;
                  uStack_58 = uStack_220;
                  uStack_60 = uStack_228;
                  uStack_49 = 0;
                  uStack_4a = 1;
                  FUN_1412e3c00(lVar4,lVar6);
                  uStack_58 = uVar9;
                  uStack_60 = uVar10;
                  uStack_49 = 0;
                  uStack_4a = 1;
                  FUN_1412e4250(&uStack_228,lVar4);
                  uStack_58 = uVar9;
                  uStack_60 = uVar10;
                  uStack_68 = uStack_220;
                  uStack_70 = uStack_228;
                  uStack_49 = 0;
                  uStack_4a = 0;
                  FUN_1412e3c80(lVar4,lVar6);
                  uStack_80 = uStack_228;
                  uStack_78 = uStack_220;
                  goto code_r0x00014098ae2f;
                }
                goto code_r0x00014098b12a;
              }
            }
          }
        }
        uStack_49 = 1;
        uStack_4a = 1;
        FUN_1412e4250(&uStack_228,lVar4);
        uVar9 = 4;
        uStack_98 = 0;
        uVar10 = 0;
        uStack_80 = uStack_228;
        uStack_78 = uStack_220;
code_r0x00014098ae2f:
        uStack_49 = 0;
        uStack_4a = 0;
        uStack_60 = uVar10;
        uStack_58 = uVar9;
        FUN_1416032a0(&uStack_228,&UNK_142ab8ee0,0x18,&UNK_142ab8ef8,0x2a,param_2,param_1);
        uStack_49 = 0;
        uStack_4a = 0;
        uStack_60 = uVar10;
        uStack_58 = uVar9;
        FUN_14160cf40(auStack_3b8,&uStack_228,0xa2f23ed82ac49a49);
        uStack_49 = 0;
        uStack_4a = 0;
        uStack_60 = uVar10;
        uStack_58 = uVar9;
        FUN_14160e160(&uStack_228,auStack_3b8,0x35ab36bc43c7b8ed);
        uStack_49 = 0;
        uStack_4a = 0;
        uStack_60 = uVar10;
        uStack_58 = uVar9;
        FUN_141606e70(auStack_3b8,&uStack_228,0x3783a9903b7f67c5);
        uStack_49 = 0;
        uStack_4a = 0;
        uStack_60 = uVar10;
        uStack_58 = uVar9;
        FUN_141607cf0(auStack_548,auStack_3b8,0x7d0cc6f7423d51b2);
        uStack_49 = 0;
        uStack_4a = 0;
        uStack_60 = uVar10;
        uStack_58 = uVar9;
        FUN_141608c10(&uStack_228,auStack_548,0xf1c07a2b8012765a,1);
        uStack_90 = uVar9;
        uStack_88 = uVar10;
        if (cStack_f2 != '\b') {
          FUN_14096b570(auStack_118,lStack_f0 + 0xe8,0xe9511a616b75789,0x11);
        }
        func_0x000142923c00(auStack_6d8,&uStack_228,400);
        uStack_228 = uStack_80;
        uStack_220 = uStack_78;
        uStack_210 = uStack_88;
        uStack_208 = uStack_90;
        uStack_200 = uStack_98;
        uStack_58 = uStack_90;
        uStack_60 = uStack_88;
        uStack_68 = uStack_78;
        uStack_70 = uStack_80;
        uStack_49 = 1;
        uStack_4a = 1;
        FUN_14160ad40(auStack_6d8,&uStack_228);
        return;
      }
    }
  }
  uStack_49 = 1;
  uStack_4a = 1;
  FUN_1428d9310(&UNK_142ab8eb0);
code_r0x00014098b144:
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

