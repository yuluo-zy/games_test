
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

float FUN_141800de0(longlong param_1,undefined8 param_2,undefined4 *param_3)

{
  undefined4 uVar1;
  longlong lVar2;
  longlong lVar3;
  float fVar4;
  float fVar5;
  longlong lStack_98;
  undefined8 uStack_90;
  undefined1 auStack_88 [48];
  float fStack_58;
  char cStack_54;
  undefined1 auStack_50 [64];
  
  lVar2 = 0;
  if (*(longlong *)(param_1 + 0x38) < -0x7ffffffffffffffe) {
    lVar2 = *(longlong *)(param_1 + 0x38) + -0x7fffffffffffffff;
  }
  uStack_90 = param_2;
  if (lVar2 == 0) {
    lStack_98 = param_1 + 0x38;
    uVar1 = *param_3;
    if (*(longlong *)(param_1 + 0x48) != 0) {
      lVar3 = *(longlong *)(param_1 + 0x48) * 0xb0;
      lVar2 = *(longlong *)(param_1 + 0x40);
      do {
        FUN_141a96310(lVar2,5);
        lVar3 = lVar3 + -0xb0;
        lVar2 = lVar2 + 0xb0;
      } while (lVar3 != 0);
    }
    FUN_1417fbdb0(&lStack_98,2);
    FUN_1417fbe00(&lStack_98,uVar1,_DAT_142925980);
    func_0x0001417fbd90(&lStack_98,0);
    FUN_140e170c0(auStack_50,lStack_98);
    FUN_140e14f50(auStack_88,auStack_50);
    fVar5 = 0.0;
    if (cStack_54 != '\x02') {
      fVar5 = 0.0;
      do {
        fVar4 = fStack_58;
        if (fStack_58 <= fVar5) {
          fVar4 = fVar5;
        }
        fVar5 = (float)(~-(uint)NAN(fVar5) & (uint)fVar4 | -(uint)NAN(fVar5) & (uint)fStack_58);
        FUN_140e14f50(auStack_88,auStack_50);
      } while (cStack_54 != '\x02');
    }
  }
  else if (lVar2 == 1) {
    lStack_98 = *(longlong *)(param_1 + 0x40);
    uVar1 = *param_3;
    if (*(longlong *)(lStack_98 + 0x10) != 0) {
      lVar3 = *(longlong *)(lStack_98 + 0x10) * 0xb0;
      lVar2 = *(longlong *)(lStack_98 + 8);
      do {
        FUN_141a96310(lVar2,5);
        lVar3 = lVar3 + -0xb0;
        lVar2 = lVar2 + 0xb0;
      } while (lVar3 != 0);
    }
    FUN_1417fbdb0(&lStack_98,2);
    FUN_1417fbe00(&lStack_98,uVar1,_DAT_142925980);
    func_0x0001417fbd90(&lStack_98,0);
    FUN_140e170c0(auStack_50,lStack_98);
    FUN_140e14f50(auStack_88,auStack_50);
    fVar5 = 0.0;
    if (cStack_54 != '\x02') {
      fVar5 = 0.0;
      do {
        fVar4 = fStack_58;
        if (fStack_58 <= fVar5) {
          fVar4 = fVar5;
        }
        fVar5 = (float)(~-(uint)NAN(fVar5) & (uint)fVar4 | -(uint)NAN(fVar5) & (uint)fStack_58);
        FUN_140e14f50(auStack_88,auStack_50);
      } while (cStack_54 != '\x02');
    }
  }
  else {
    lStack_98 = FUN_141801600(param_1 + 0x40);
    uVar1 = *param_3;
    if (*(longlong *)(lStack_98 + 0x10) != 0) {
      lVar3 = *(longlong *)(lStack_98 + 0x10) * 0xb0;
      lVar2 = *(longlong *)(lStack_98 + 8);
      do {
        FUN_141a96310(lVar2,5);
        lVar3 = lVar3 + -0xb0;
        lVar2 = lVar2 + 0xb0;
      } while (lVar3 != 0);
    }
    FUN_1417fbdb0(&lStack_98,2);
    FUN_1417fbe00(&lStack_98,uVar1,_DAT_142925980);
    func_0x0001417fbd90(&lStack_98,0);
    FUN_140e170c0(auStack_50,lStack_98);
    FUN_140e14f50(auStack_88,auStack_50);
    fVar5 = 0.0;
    if (cStack_54 != '\x02') {
      fVar5 = 0.0;
      do {
        fVar4 = fStack_58;
        if (fStack_58 <= fVar5) {
          fVar4 = fVar5;
        }
        fVar5 = (float)(~-(uint)NAN(fVar5) & (uint)fVar4 | -(uint)NAN(fVar5) & (uint)fStack_58);
        FUN_140e14f50(auStack_88,auStack_50);
      } while (cStack_54 != '\x02');
    }
  }
  return fVar5;
}

