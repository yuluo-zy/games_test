
undefined8 * FUN_140be2880(undefined8 *param_1,byte *param_2,longlong param_3)

{
  byte *pbVar1;
  code *pcVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  byte *pbVar5;
  byte bVar6;
  uint uVar7;
  uint uVar8;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  uStack_18 = 0;
  uStack_10 = 0;
  if (param_3 != 0) {
    pbVar5 = param_2 + param_3;
    uVar3 = 0;
    do {
      bVar6 = *param_2;
      if ((char)bVar6 < '\0') {
        uVar7 = bVar6 & 0x1f;
        pbVar1 = param_2 + 1;
        if (bVar6 < 0xe0) {
          param_2 = param_2 + 2;
          uVar8 = uVar7 << 6 | *pbVar1 & 0x3f;
        }
        else {
          uVar8 = param_2[2] & 0x3f | (*pbVar1 & 0x3f) << 6;
          if (bVar6 < 0xf0) {
            param_2 = param_2 + 3;
            uVar8 = uVar8 | uVar7 << 0xc;
          }
          else {
            uVar8 = param_2[3] & 0x3f | uVar8 << 6 | (bVar6 & 7) << 0x12;
            if (uVar8 == 0x110000) break;
            param_2 = param_2 + 4;
          }
        }
        bVar6 = (byte)uVar8;
        if (uVar8 < 0x80) goto code_r0x000140be2950;
      }
      else {
        param_2 = param_2 + 1;
code_r0x000140be2950:
        if (0xf < uVar3) {
          FUN_1428d9518(uVar3,0x10,&UNK_142b12f20);
          pcVar2 = (code *)swi(3);
          puVar4 = (undefined8 *)(*pcVar2)();
          return puVar4;
        }
        *(byte *)((longlong)&uStack_18 + uVar3) = bVar6;
        if (uVar3 == 0xf) break;
        uVar3 = uVar3 + 1;
      }
    } while (param_2 != pbVar5);
  }
  *param_1 = uStack_18;
  param_1[1] = uStack_10;
  return param_1;
}

