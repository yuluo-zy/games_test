
undefined8 FUN_1418c04b0(undefined8 param_1)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  code *pcVar4;
  undefined8 uVar5;
  uint uVar6;
  ulonglong uVar7;
  byte bVar8;
  undefined8 auStack_90 [3];
  byte abStack_78 [8];
  undefined1 auStack_70 [8];
  ulonglong uStack_68;
  byte bStack_5f;
  char cStack_5e;
  longlong lStack_40;
  longlong lStack_30;
  ulonglong uStack_28;
  undefined8 uStack_20;
  undefined8 uStack_18;
  
  FUN_140d89940(abStack_78,&UNK_142d05f38,param_1,&UNK_142d06000,2);
  if ((abStack_78[0] & 1) == 0) {
    bVar8 = bStack_5f;
    if (cStack_5e == '\0') {
      for (; uStack_68 != 0; uStack_68 = uStack_68 + uVar7) {
        if (uStack_68 < uStack_28) {
          if (*(char *)(lStack_30 + uStack_68) < -0x40) {
code_r0x0001418c068b:
            FUN_1428d9940(lStack_30,uStack_28,0,uStack_68,&UNK_142d06038);
            pcVar4 = (code *)swi(3);
            uVar5 = (*pcVar4)();
            return uVar5;
          }
        }
        else if (uStack_68 != uStack_28) goto code_r0x0001418c068b;
        bVar1 = *(byte *)(lStack_30 + -1 + uStack_68);
        uVar6 = (uint)(char)bVar1;
        if ((char)bVar1 < '\0') {
          bVar2 = *(byte *)(lStack_30 + -2 + uStack_68);
          if ((char)bVar2 < -0x40) {
            bVar3 = *(byte *)(lStack_30 + -3 + uStack_68);
            if ((char)bVar3 < -0x40) {
              uVar6 = bVar3 & 0x3f | (*(byte *)(lStack_30 + -4 + uStack_68) & 7) << 6;
            }
            else {
              uVar6 = bVar3 & 0xf;
            }
            uVar6 = bVar2 & 0x3f | uVar6 << 6;
          }
          else {
            uVar6 = bVar2 & 0x1f;
          }
          uVar6 = (uint)(bVar1 & 0x3f) | uVar6 << 6;
        }
        if ((bVar8 & 1) != 0) goto code_r0x0001418c0667;
        uVar7 = 0xffffffffffffffff;
        if ((0x7f < uVar6) && (uVar7 = 0xfffffffffffffffe, 0x7ff < uVar6)) {
          uVar7 = (ulonglong)(uVar6 < 0x10000) | 0xfffffffffffffffc;
        }
        bVar8 = bStack_5f != 1;
        bStack_5f = bStack_5f ^ 1;
      }
      auStack_90[0] = 0;
      if ((bVar8 & 1) != 0) {
code_r0x0001418c0667:
        auStack_90[0] = 1;
      }
    }
    else {
      auStack_90[0] = 0;
    }
  }
  else {
    FUN_1418c06a0(auStack_90,auStack_70,lStack_30,uStack_28,uStack_20,uStack_18,lStack_40 == -1);
  }
  return auStack_90[0];
}

