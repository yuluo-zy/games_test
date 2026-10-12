
void FUN_140bd4560(longlong param_1,longlong param_2)

{
  uint uVar1;
  code *pcVar2;
  undefined *puVar3;
  uint *puVar4;
  uint uVar5;
  uint uVar6;
  longlong lVar7;
  ulonglong uVar8;
  byte *pbVar9;
  uint uStack_7c;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined4 **ppuStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  uint *puStack_48;
  undefined *puStack_40;
  
  if (*(longlong *)(param_2 + 0x28) != 0) {
    pbVar9 = *(byte **)(param_2 + 0x20);
    lVar7 = *(longlong *)(param_2 + 0x28) * 0x30;
    do {
      puVar4 = (uint *)(pbVar9 + 4);
      if (*pbVar9 != 0) {
        puVar4 = (uint *)(pbVar9 + 0x18);
      }
      uVar8 = (ulonglong)*puVar4;
      func_0x000140c10dd0(&uStack_78,pbVar9[0x28]);
      puVar3 = uStack_78;
      if (0xffff < uVar8) {
code_r0x000140bd4787:
        FUN_1428d9760(&UNK_142b0f5a8,0x2b,&uStack_78,&UNK_142b0f588,&UNK_142b0f660);
code_r0x000140bd47b0:
        FUN_1428d9518(uVar8);
code_r0x000140bd47bf:
        FUN_1428d9518(uVar8);
        pcVar2 = (code *)swi(3);
        (*pcVar2)();
        return;
      }
      uVar1 = uStack_78._4_4_;
      FUN_1418adc70(param_1 + 0x90,uVar8);
      if (uVar1 != 0) {
        if ((*pbVar9 & 1) == 0) {
          if (*(ulonglong *)(param_1 + 0x58) <= uVar8) goto code_r0x000140bd47bf;
          uStack_7c = uVar1;
          if ((int)uVar1 < 0x220) {
            if ((int)uVar1 < 0x100) {
              if (uVar1 == 0x20) {
                uVar5 = 4;
              }
              else {
                if (uVar1 != 0x40) {
                  if (uVar1 != 0x80) goto code_r0x000140bd472f;
                  goto code_r0x000140bd46ce;
                }
code_r0x000140bd46b0:
                uVar5 = 8;
              }
            }
            else if ((uVar1 == 0x100) || (uVar1 == 0x180)) {
code_r0x000140bd46ce:
              uVar5 = 0x10;
            }
            else {
              if (uVar1 != 0x200) goto code_r0x000140bd472f;
code_r0x000140bd45a4:
              uVar5 = 0x20;
            }
          }
          else if ((int)uVar1 < 0x800) {
            if ((uVar1 == 0x400) || (uVar1 == 0x600)) goto code_r0x000140bd45a4;
            if (uVar1 != 0x220) goto code_r0x000140bd472f;
            uVar5 = 0x24;
          }
          else if (uVar1 == 0x800) {
            uVar5 = 1;
          }
          else {
            if (uVar1 != 0x1000) {
              if (uVar1 == 0x18000) goto code_r0x000140bd46b0;
code_r0x000140bd472f:
              puStack_48 = &uStack_7c;
              puStack_40 = &UNK_140c87a90;
              uStack_78 = &UNK_142b0f710;
              uStack_70 = 1;
              uStack_58 = 0;
              ppuStack_68 = &puStack_48;
              uStack_60 = 1;
              FUN_1428d9390(&uStack_78,&UNK_142b0f720);
              goto code_r0x000140bd4787;
            }
            uVar5 = 2;
          }
          puVar4 = (uint *)(*(longlong *)(param_1 + 0x50) + uVar8 * 4);
          *puVar4 = *puVar4 | uVar5;
          uStack_7c = uVar1;
        }
        else {
          if (*(ulonglong *)(param_1 + 0x70) <= uVar8) goto code_r0x000140bd47b0;
          uVar6 = (uVar1 & 6) << 5 | (uVar1 & 1) << 8;
          uVar5 = uVar6 + 0x20 + (uVar1 & 8) * 2;
          if (((ulonglong)puVar3 & 0x1806000000000) == 0) {
            uVar5 = uVar6 + (uVar1 & 8) * 2;
          }
          puVar4 = (uint *)(*(longlong *)(param_1 + 0x68) + uVar8 * 4);
          *puVar4 = *puVar4 | uVar1 >> 0xb & 3 | uVar5;
        }
      }
      pbVar9 = pbVar9 + 0x30;
      lVar7 = lVar7 + -0x30;
    } while (lVar7 != 0);
  }
  return;
}

