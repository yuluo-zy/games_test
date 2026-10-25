
void FUN_141800c00(longlong param_1,float *param_2)

{
  longlong lVar1;
  float fVar2;
  float fVar3;
  undefined1 auStack_98 [44];
  float fStack_6c;
  char cStack_64;
  undefined1 auStack_60 [72];
  
  lVar1 = 0;
  if (*(longlong *)(param_1 + 0x38) < -0x7ffffffffffffffe) {
    lVar1 = *(longlong *)(param_1 + 0x38) + -0x7fffffffffffffff;
  }
  if (lVar1 == 0) {
    FUN_140e170c0(auStack_60,param_1 + 0x38);
    fVar3 = *param_2;
    FUN_140e14f50(auStack_98,auStack_60);
    if (cStack_64 != '\x02') {
      do {
        fVar2 = fStack_6c;
        if (fStack_6c <= fVar3) {
          fVar2 = fVar3;
        }
        fVar3 = (float)(~-(uint)NAN(fVar3) & (uint)fVar2 | -(uint)NAN(fVar3) & (uint)fStack_6c);
        *param_2 = fVar3;
        FUN_140e14f50(auStack_98,auStack_60);
      } while (cStack_64 != '\x02');
    }
  }
  else if (lVar1 == 1) {
    FUN_140e170c0(auStack_60,*(undefined8 *)(param_1 + 0x40));
    fVar3 = *param_2;
    FUN_140e14f50(auStack_98,auStack_60);
    if (cStack_64 != '\x02') {
      do {
        fVar2 = fStack_6c;
        if (fStack_6c <= fVar3) {
          fVar2 = fVar3;
        }
        fVar3 = (float)(~-(uint)NAN(fVar3) & (uint)fVar2 | -(uint)NAN(fVar3) & (uint)fStack_6c);
        *param_2 = fVar3;
        FUN_140e14f50(auStack_98,auStack_60);
      } while (cStack_64 != '\x02');
    }
  }
  else {
    FUN_140e170c0(auStack_60,*(longlong *)(param_1 + 0x40) + 0x10);
    fVar3 = *param_2;
    FUN_140e14f50(auStack_98,auStack_60);
    if (cStack_64 != '\x02') {
      do {
        fVar2 = fStack_6c;
        if (fStack_6c <= fVar3) {
          fVar2 = fVar3;
        }
        fVar3 = (float)(~-(uint)NAN(fVar3) & (uint)fVar2 | -(uint)NAN(fVar3) & (uint)fStack_6c);
        *param_2 = fVar3;
        FUN_140e14f50(auStack_98,auStack_60);
      } while (cStack_64 != '\x02');
    }
  }
  return;
}

