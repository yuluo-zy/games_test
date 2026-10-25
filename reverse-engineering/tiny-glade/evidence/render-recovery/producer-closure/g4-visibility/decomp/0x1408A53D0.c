
void FUN_1408a53d0(longlong param_1,longlong param_2,longlong param_3)

{
  char cVar1;
  code *pcVar2;
  int iVar3;
  byte *pbVar4;
  longlong lVar5;
  undefined8 *puVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  char cVar9;
  undefined1 auStack_428 [248];
  undefined1 auStack_330 [248];
  undefined *puStack_238;
  undefined8 uStack_230;
  undefined1 auStack_180 [38];
  char cStack_15a;
  longlong lStack_150;
  int iStack_140;
  int iStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  longlong lStack_130;
  ulonglong uStack_128;
  longlong lStack_48;
  undefined1 uStack_3a;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lStack_48 = param_1;
  if (1 < *(byte *)(param_1 + 0xf0)) {
    func_0x000142923c00(&iStack_140,param_1,0xf8);
    uStack_39 = 0;
    FUN_1409082c0(&iStack_140);
    return;
  }
  uVar8 = *(ulonglong *)(param_2 + 8);
  uVar7 = uVar8 & 0xffffffff;
  iStack_13c = (int)(uVar8 >> 0x20);
  if ((uVar7 < *(ulonglong *)(param_3 + 0x10)) &&
     (*(int *)(*(longlong *)(param_3 + 8) + uVar7 * 0x14) == iStack_13c)) {
    lVar5 = *(longlong *)(param_3 + 8) + uVar7 * 0x14;
    iStack_140 = *(int *)(lVar5 + 4);
    if (iStack_140 != -1) {
      uStack_134 = *(undefined4 *)(lVar5 + 0x10);
      iStack_13c = (int)*(undefined8 *)(lVar5 + 8);
      uStack_138 = (undefined4)((ulonglong)*(undefined8 *)(lVar5 + 8) >> 0x20);
      uStack_39 = 1;
      lStack_130 = param_3;
      uStack_128 = uVar8;
      pbVar4 = (byte *)FUN_1408a4050(&iStack_140);
      if (pbVar4 == (byte *)0x0) {
        uStack_39 = 1;
        FUN_1428d9310(&UNK_142a9b418);
        goto code_r0x0001408a5948;
      }
      if (pbVar4[1] == 1) {
        func_0x000142923c00(&iStack_140,lStack_48,0xf8);
        uStack_39 = 0;
        FUN_1409082c0(&iStack_140);
        return;
      }
      *(byte *)(lStack_48 + 0xa8) = *(byte *)(lStack_48 + 0xa8) & 0xf3 | 4;
      uStack_39 = 1;
      uVar8 = 0x8226cc786e679355;
      lVar5 = FUN_1400dd6f0(param_3 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
      if (((lVar5 != 0) && (uVar8 < *(ulonglong *)(param_3 + 0x218))) &&
         (uVar8 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar8 * 8), uVar8 != 0)) {
        lVar5 = ~uVar8 * 0x60;
        if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar5) != 0) {
          uStack_39 = 1;
          puVar6 = (undefined8 *)
                   FUN_140969720(*(undefined8 *)(*(longlong *)(param_3 + 0x1e0) + lVar5 + 0x10),
                                 &UNK_142c2e2b8);
          if (puVar6 != (undefined8 *)0x0) {
            puStack_238 = &UNK_142a9d7ff;
            uStack_230 = 0x3a;
            if ((puVar6[3] != 0x3a) ||
               (iVar3 = func_0x000142923bf0(&UNK_142a9d7ff,puVar6[2],0x3a), iVar3 != 0)) {
              iStack_140 = 0;
              iStack_13c = 0;
              uStack_39 = 1;
              FUN_1428cab41(0,&puStack_238,puVar6 + 2,&iStack_140,&UNK_142c2e180);
              goto code_r0x0001408a5948;
            }
            uStack_39 = 1;
            lVar5 = (**(code **)(puVar6[1] + 0x18))(*puVar6);
            if (lVar5 != 0) {
              func_0x000142923c00(&iStack_140,lStack_48,0xf8);
              uStack_39 = 0;
              FUN_140906400(&puStack_238,&iStack_140,0xac77ba3d359698bc,lVar5 + 0x68);
              uStack_39 = 0;
              FUN_14162d190(auStack_330,&puStack_238,0xca16df0f469c8dff);
              uStack_3a = 1;
              uVar8 = 0x850dbd51eafef756;
              lVar5 = FUN_1400dd6f0(param_3 + 0x38,0x850dbd51eafef756,0x85782a4b39854c35);
              if (((lVar5 != 0) && (uVar8 < *(ulonglong *)(param_3 + 0x218))) &&
                 (uVar8 = *(ulonglong *)(*(longlong *)(param_3 + 0x210) + uVar8 * 8), uVar8 != 0)) {
                lVar5 = ~uVar8 * 0x60;
                if (*(longlong *)(*(longlong *)(param_3 + 0x1e0) + 0x20 + lVar5) != 0) {
                  cVar1 = *(char *)(*(longlong *)(*(longlong *)(param_3 + 0x1e0) + lVar5 + 0x10) +
                                   0x10);
                  if (cVar1 == '\0') {
                    cVar9 = '\0';
                    if (*pbVar4 != 0) {
                      func_0x000142923c00(&iStack_140,auStack_330,0xf8);
                      uStack_3a = 0;
                      FUN_1409082c0(&iStack_140);
                      return;
                    }
                  }
                  else {
                    cVar9 = '\0';
                    if ((*pbVar4 & 1) != 0) {
                      cVar9 = cVar1;
                    }
                  }
                  func_0x000142923c00(&iStack_140,auStack_330,0xf8);
                  uStack_3a = 0;
                  FUN_14090be10(&puStack_238,&iStack_140,param_3);
                  uStack_3a = 0;
                  FUN_14090bb70(auStack_428,&puStack_238);
                  puStack_238 = (undefined *)0x7db9e7483fa24078;
                  iStack_140 = 2;
                  FUN_140908610(&puStack_238,auStack_428,0xaa8f0490dba0d03c,0x1d,&iStack_140);
                  func_0x000142923c00(&iStack_140,auStack_428,0xf8);
                  uStack_3a = 0;
                  FUN_14162ad90(&puStack_238,&iStack_140,0xd56729333fc94f92);
                  if (cStack_15a != '\b') {
                    FUN_14096b570(auStack_180,lStack_150 + 0xe8,0x52e7aa4e36b047b,cVar9);
                  }
                  func_0x000142923c00(auStack_330,&puStack_238,0xf8);
                  func_0x000142923c00(&iStack_140,auStack_330,0xf8);
                  uStack_3a = 0;
                  FUN_140907990(&iStack_140,(cVar9 != '\0') * '\x02' + '\x11',1,0);
                  return;
                }
              }
              uStack_3a = 1;
              FUN_1428d9310(&UNK_142a9b4a0);
              goto code_r0x0001408a5948;
            }
          }
          uStack_39 = 1;
          FUN_1428d9310(&UNK_142a9b448);
          goto code_r0x0001408a5948;
        }
      }
      uStack_39 = 1;
      FUN_1428d9310(&UNK_142a9b430);
      goto code_r0x0001408a5948;
    }
  }
  iStack_140 = (int)uVar8;
  uStack_39 = 1;
  FUN_1428d9760(&UNK_142a9b0c0,0x2b,&iStack_140,&UNK_142a9b0f0,&UNK_142a9b400);
code_r0x0001408a5948:
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

