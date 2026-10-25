// 0x142340d20 undefined FUN_142340d20(void) body_bytes=820

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142340d20(longlong *param_1,undefined8 *param_2)

{
  uint *puVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  longlong lVar8;
  code *pcVar9;
  float fVar10;
  float fVar11;
  longlong lVar12;
  ulonglong uVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  uint *puVar20;
  int iStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  int iStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  
  fVar11 = _UNK_1429258e4;
  fVar10 = _DAT_1429258e0;
  lVar5 = *param_1;
  lVar6 = param_1[1];
  uVar4 = *(undefined4 *)((longlong)param_1 + 0x14);
  puVar20 = *(uint **)(lVar5 + 0xf8);
  puVar1 = puVar20 + *(longlong *)(lVar5 + 0x100);
  uVar7 = *param_2;
  if (*(char *)(lVar5 + 0x108) == '\0') {
    while (puVar20 != puVar1) {
      while( true ) {
        lVar16 = *(longlong *)(*(longlong *)(lVar6 + 0x100) + 0x58 + (ulonglong)*puVar20 * 0xa0);
        if (lVar16 != 0) break;
        puVar20 = puVar20 + 1;
        if (puVar20 == puVar1) {
          return;
        }
      }
      lVar12 = *(longlong *)(lVar6 + 0x100) + (ulonglong)*puVar20 * 0xa0;
      uVar13 = (ulonglong)*(uint *)(lVar12 + 0x94);
      lVar19 = *(longlong *)(lVar6 + 0x1a8);
      lVar8 = *(longlong *)(lVar19 + 0x18 + uVar13 * 0x48);
      lVar18 = *(longlong *)(lVar19 + 0x38 + uVar13 * 0x48);
      if ((*(ulonglong *)(lVar5 + 0x118) < *(ulonglong *)(lVar19 + 0x40 + uVar13 * 0x48)) &&
         (uVar13 = *(ulonglong *)(lVar18 + *(ulonglong *)(lVar5 + 0x118) * 8), uVar13 != 0)) {
        lVar19 = *(longlong *)(lVar8 + 0x10 + ~uVar13 * 0x30);
      }
      else {
        lVar19 = 0;
      }
      lVar14 = ~*(ulonglong *)(lVar18 + *(longlong *)(lVar5 + 0x110) * 8) * 0x30;
      lVar18 = *(longlong *)(lVar8 + 0x10 + lVar14);
      lVar8 = *(longlong *)(lVar8 + 0x28 + lVar14);
      puVar20 = puVar20 + 1;
      lVar12 = *(longlong *)(lVar12 + 0x50);
      lVar14 = 0;
      do {
        uVar13 = (ulonglong)*(uint *)(lVar12 + 8 + lVar14 * 0x10);
        FUN_140fe7d00(&iStack_98,uVar7,*(undefined4 *)(lVar19 + uVar13 * 0x14),
                      *(undefined4 *)(lVar19 + 4 + uVar13 * 0x14));
        if (iStack_98 != 4) goto LAB_142341033;
        lVar14 = lVar14 + 1;
        uVar2 = *(undefined8 *)CONCAT44(uStack_8c,uStack_90);
        uVar3 = ((undefined8 *)CONCAT44(uStack_8c,uStack_90))[1];
        *(ulonglong *)(lVar18 + uVar13 * 0x10) =
             CONCAT44((float)((ulonglong)uVar2 >> 0x20) * fVar11 + (float)((ulonglong)uVar3 >> 0x20)
                      ,(float)uVar2 * fVar10 + (float)uVar3);
        *(undefined4 *)(lVar8 + uVar13 * 4) = uVar4;
        *(undefined8 *)(lVar18 + 8 + uVar13 * 0x10) = uVar2;
      } while (lVar14 != lVar16);
    }
  }
  else {
    while (puVar20 != puVar1) {
      while( true ) {
        uVar13 = *(ulonglong *)(*(longlong *)(lVar6 + 0x1a8) + 0x10 + (ulonglong)*puVar20 * 0x48);
        if (uVar13 != 0) break;
        puVar20 = puVar20 + 1;
        if (puVar20 == puVar1) {
          return;
        }
      }
      lVar16 = *(longlong *)(lVar6 + 0x1a8) + (ulonglong)*puVar20 * 0x48;
      lVar19 = *(longlong *)(lVar16 + 0x18);
      lVar18 = ~*(ulonglong *)(*(longlong *)(lVar16 + 0x38) + *(longlong *)(lVar5 + 0x110) * 8) *
               0x30;
      lVar8 = *(longlong *)(lVar19 + 0x10 + lVar18);
      lVar18 = *(longlong *)(lVar19 + 0x28 + lVar18);
      puVar20 = puVar20 + 1;
      if ((*(ulonglong *)(lVar5 + 0x118) < *(ulonglong *)(lVar16 + 0x40)) &&
         (uVar17 = *(ulonglong *)(*(longlong *)(lVar16 + 0x38) + *(ulonglong *)(lVar5 + 0x118) * 8),
         uVar17 != 0)) {
        lVar16 = *(longlong *)(lVar19 + 0x10 + ~uVar17 * 0x30);
      }
      else {
        lVar16 = 0;
      }
      uVar17 = 0;
      do {
        uVar15 = uVar17 & 0xffffffff;
        FUN_140fe7d00(&iStack_98,uVar7,*(undefined4 *)(lVar16 + uVar15 * 0x14),
                      *(undefined4 *)(lVar16 + 4 + uVar15 * 0x14));
        if (iStack_98 != 4) {
LAB_142341033:
          uStack_68 = uStack_88;
          uStack_64 = uStack_84;
          uStack_60 = uStack_80;
          uStack_5c = uStack_7c;
          iStack_78 = iStack_98;
          uStack_74 = uStack_94;
          uStack_70 = uStack_90;
          uStack_6c = uStack_8c;
          FUN_1428d9760(&UNK_14304df08,0x2b,&iStack_78,&UNK_14304dee8,&UNK_14304e080);
          pcVar9 = (code *)swi(3);
          (*pcVar9)();
          return;
        }
        uVar17 = uVar17 + 1;
        uVar2 = *(undefined8 *)CONCAT44(uStack_8c,uStack_90);
        uVar3 = ((undefined8 *)CONCAT44(uStack_8c,uStack_90))[1];
        *(ulonglong *)(lVar8 + uVar15 * 0x10) =
             CONCAT44((float)((ulonglong)uVar2 >> 0x20) * fVar11 + (float)((ulonglong)uVar3 >> 0x20)
                      ,(float)uVar2 * fVar10 + (float)uVar3);
        *(undefined4 *)(lVar18 + uVar15 * 4) = uVar4;
        *(undefined8 *)(lVar8 + 8 + uVar15 * 0x10) = uVar2;
      } while (uVar17 != uVar13);
    }
  }
  return;
}

