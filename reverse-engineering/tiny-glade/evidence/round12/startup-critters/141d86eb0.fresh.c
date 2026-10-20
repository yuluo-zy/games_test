
void FUN_141d86eb0(longlong *param_1)

{
  uint *puVar1;
  uint uVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  uint *puVar14;
  undefined8 uStack_50;
  undefined4 uStack_48;
  
  lVar3 = *param_1;
  lVar4 = param_1[1];
  puVar14 = *(uint **)(lVar3 + 0xf8);
  puVar1 = puVar14 + *(longlong *)(lVar3 + 0x100);
  if (*(char *)(lVar3 + 0x108) == '\0') {
    lVar10 = 8;
    lVar13 = 0;
    lVar11 = 0;
    lVar9 = 0;
    lVar6 = 0;
    do {
      if (lVar6 == lVar9) {
        do {
          if (puVar14 == puVar1) {
            return;
          }
          uVar2 = *puVar14;
          puVar14 = puVar14 + 1;
          lVar9 = *(longlong *)(*(longlong *)(lVar4 + 0x100) + 0x58 + (ulonglong)uVar2 * 0xa0);
        } while (lVar9 == 0);
        lVar6 = *(longlong *)(lVar4 + 0x100) + (ulonglong)uVar2 * 0xa0;
        uVar7 = (ulonglong)*(uint *)(lVar6 + 0x94);
        lVar10 = *(longlong *)(lVar4 + 0x1a8) + uVar7 * 0x48;
        uVar7 = *(ulonglong *)(*(longlong *)(lVar4 + 0x1a8) + 0x40 + uVar7 * 0x48);
        if ((*(ulonglong *)(lVar3 + 0x110) < uVar7) &&
           (uVar12 = *(ulonglong *)
                      (*(longlong *)(lVar10 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
           uVar12 != 0)) {
          lVar13 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar12 * 0x30);
          uVar12 = *(ulonglong *)(lVar3 + 0x118);
          if (uVar7 <= uVar12) goto LAB_141d87008;
LAB_141d870f7:
          uVar7 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + uVar12 * 8);
          if (uVar7 == 0) goto LAB_141d87008;
          lVar11 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar7 * 0x30);
        }
        else {
          lVar13 = 0;
          uVar12 = *(ulonglong *)(lVar3 + 0x118);
          if (uVar12 < uVar7) goto LAB_141d870f7;
LAB_141d87008:
          lVar11 = 0;
        }
        lVar10 = *(longlong *)(lVar6 + 0x50);
        lVar6 = 0;
      }
      lVar5 = (ulonglong)*(uint *)(lVar10 + 8 + lVar6 * 0x10) * 0x30;
      lVar6 = lVar6 + 1;
      uStack_48 = *(undefined4 *)(lVar13 + 0x18 + lVar5);
      uStack_50 = *(undefined8 *)(lVar13 + 0x10 + lVar5);
      func_0x000140f85ba0(lVar11 + lVar5,&uStack_50);
    } while( true );
  }
  lVar10 = 0;
  lVar9 = 0;
  uVar7 = 0;
  uVar12 = 0;
  do {
    if (uVar12 == uVar7) {
      do {
        if (puVar14 == puVar1) {
          return;
        }
        uVar2 = *puVar14;
        puVar14 = puVar14 + 1;
        uVar7 = *(ulonglong *)(*(longlong *)(lVar4 + 0x1a8) + 0x10 + (ulonglong)uVar2 * 0x48);
      } while (uVar7 == 0);
      lVar9 = *(longlong *)(lVar4 + 0x1a8) + (ulonglong)uVar2 * 0x48;
      uVar12 = *(ulonglong *)(lVar9 + 0x40);
      if ((*(ulonglong *)(lVar3 + 0x110) < uVar12) &&
         (uVar8 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + *(ulonglong *)(lVar3 + 0x110) * 8),
         uVar8 != 0)) {
        lVar10 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar8 * 0x30);
        uVar8 = *(ulonglong *)(lVar3 + 0x118);
        if (uVar12 <= uVar8) goto LAB_141d86f0c;
LAB_141d86fde:
        uVar12 = *(ulonglong *)(*(longlong *)(lVar9 + 0x38) + uVar8 * 8);
        if (uVar12 == 0) goto LAB_141d86f0c;
        lVar9 = *(longlong *)(*(longlong *)(lVar9 + 0x18) + 0x10 + ~uVar12 * 0x30);
      }
      else {
        lVar10 = 0;
        uVar8 = *(ulonglong *)(lVar3 + 0x118);
        if (uVar8 < uVar12) goto LAB_141d86fde;
LAB_141d86f0c:
        lVar9 = 0;
      }
      uVar12 = 0;
    }
    lVar13 = (uVar12 & 0xffffffff) * 0x30;
    uVar12 = uVar12 + 1;
    uStack_48 = *(undefined4 *)(lVar10 + 0x18 + lVar13);
    uStack_50 = *(undefined8 *)(lVar10 + 0x10 + lVar13);
    func_0x000140f85ba0(lVar13 + lVar9,&uStack_50);
  } while( true );
}

