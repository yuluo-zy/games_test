
longlong FUN_140bd7bc0(longlong param_1)

{
  ulonglong *puVar1;
  code *pcVar2;
  ulonglong uVar3;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong lVar8;
  
  if (8 < *(ulonglong *)(param_1 + 0x10)) {
    puVar1 = *(ulonglong **)(param_1 + 8);
    uVar3 = *puVar1;
    uVar3 = uVar3 >> 0x38 | (uVar3 & 0xff000000000000) >> 0x28 | (uVar3 & 0xff0000000000) >> 0x18 |
            (uVar3 & 0xff00000000) >> 8 | (uVar3 & 0xff000000) << 8 | (uVar3 & 0xff0000) << 0x18 |
            (uVar3 & 0xff00) << 0x28 | uVar3 << 0x38;
    if (uVar3 == 0x564b5f4c41594552) {
      if ((char)puVar1[1] == '_') {
code_r0x000140bd7c17:
        uVar7 = *(ulonglong *)(param_1 + 0x10) - 9;
        lVar8 = (longlong)puVar1 + 9;
        uVar3 = 0;
        while( true ) {
          uVar6 = lVar8 + uVar3;
          if (uVar7 - uVar3 < 0x10) {
            if (uVar7 == uVar3) {
              return 0;
            }
            uVar5 = 0;
            while (*(char *)(uVar6 + uVar5) != '_') {
              uVar5 = uVar5 + 1;
              if (uVar7 - uVar3 == uVar5) {
                return 0;
              }
            }
          }
          else {
            lVar4 = FUN_140d88a40(0x5f);
            uVar5 = uVar6;
            if (lVar4 != 1) {
              return 0;
            }
          }
          uVar6 = uVar5 + uVar3;
          if ((uVar6 < uVar7) && (*(char *)(lVar8 + uVar6) == '_')) break;
          uVar3 = uVar3 + uVar5 + 1;
          if (uVar7 < uVar3) {
            return 0;
          }
        }
        if (uVar6 == 0) {
          return 0;
        }
        if (uVar6 < uVar7) {
          return lVar8;
        }
        if (uVar6 == uVar7) {
          return lVar8;
        }
        FUN_1428d9940(lVar8,uVar7,0,uVar6,&UNK_142b11068);
        pcVar2 = (code *)swi(3);
        lVar8 = (*pcVar2)();
        return lVar8;
      }
    }
    else if ((char)((uVar3 < 0x564b5f4c41594553) * '\x02') == '\x01') goto code_r0x000140bd7c17;
  }
  return 0;
}

