
undefined8 FUN_141478b50(undefined8 *param_1,undefined8 *param_2,ulonglong *param_3)

{
  undefined4 uVar1;
  int iVar2;
  longlong *plVar3;
  undefined4 *puVar4;
  longlong lVar5;
  code *pcVar6;
  undefined8 uVar7;
  uint uVar8;
  ulonglong *puVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong in_R10;
  ulonglong in_R11;
  uint *puVar14;
  undefined8 *puVar15;
  ulonglong uVar16;
  longlong lVar17;
  int iStack_108;
  undefined4 uStack_104;
  undefined8 uStack_100;
  ulonglong uStack_e0;
  ulonglong uStack_d8;
  ulonglong uStack_d0;
  ulonglong uStack_c8;
  ulonglong uStack_c0;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  undefined4 uStack_9c;
  ulonglong uStack_98;
  ulonglong uStack_90;
  ulonglong uStack_88;
  ulonglong uStack_80;
  ulonglong *puStack_78;
  undefined4 *puStack_70;
  undefined8 *puStack_68;
  undefined8 *puStack_60;
  ulonglong uStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  uVar16 = param_1[1];
  if (uVar16 < (ulonglong)param_1[2]) {
    puVar15 = (undefined8 *)*param_1;
    uStack_88 = *param_2;
    lVar17 = -param_1[2];
    puStack_78 = param_3;
    puStack_68 = param_1;
    puStack_60 = puVar15;
    lStack_50 = lVar17;
    do {
      uVar16 = uVar16 + 1;
      param_1[1] = uVar16;
      plVar3 = (longlong *)*puVar15;
      puVar4 = (undefined4 *)*plVar3;
      lVar5 = plVar3[1];
      if ((ulonglong)(lVar5 - (longlong)puVar4) < 4) {
        uVar13 = 7;
        uVar10 = 0;
LAB_141478dc7:
        puVar9 = &uStack_98;
        uVar11 = 0x8000000000000000;
      }
      else {
        uVar1 = *puVar4;
        *plVar3 = (longlong)(puVar4 + 1);
        uVar13 = 7;
        if ((ulonglong)(lVar5 - (longlong)(puVar4 + 1)) < 4) {
LAB_141478c78:
          uVar10 = 0;
          uVar8 = 0;
LAB_141478dc0:
          in_R10 = (ulonglong)((uint)uVar10 & 0xff | uVar8);
          goto LAB_141478dc7;
        }
        uVar11 = (ulonglong)(uint)puVar4[1];
        *plVar3 = (longlong)(puVar4 + 2);
        if (5 < uVar11) {
          uVar13 = 1;
          goto LAB_141478c78;
        }
        if ((ulonglong)(lVar5 - (longlong)(puVar4 + 2)) < 4) goto LAB_141478c78;
        iVar2 = puVar4[2];
        puVar14 = puVar4 + 3;
        *plVar3 = (longlong)puVar14;
        puVar15 = puStack_60;
        if (iVar2 != 1) {
          if (iVar2 == 0) {
            if (3 < (ulonglong)(lVar5 - (longlong)puVar14)) {
              uVar8 = puVar4[3];
              puVar14 = puVar4 + 4;
              *plVar3 = (longlong)puVar14;
              uVar10 = 0;
              puStack_70 = (undefined4 *)(ulonglong)uVar8;
              goto LAB_141478c85;
            }
          }
          else {
            uVar13 = 1;
          }
LAB_141478ce2:
          uVar10 = 0;
          uVar8 = 0;
          param_1 = puStack_68;
          lVar17 = lStack_50;
          goto LAB_141478dc0;
        }
        uVar10 = 1;
        puStack_70 = puVar4;
LAB_141478c85:
        if ((ulonglong)(lVar5 - (longlong)puVar14) < 4) goto LAB_141478ce2;
        uVar8 = *puVar14;
        uVar12 = (ulonglong)uVar8;
        puVar14 = puVar14 + 1;
        *plVar3 = (longlong)puVar14;
        if ((ulonglong)(lVar5 - (longlong)puVar14) < uVar12) {
          uStack_a8 = uStack_58;
          uStack_b0 = uStack_90;
LAB_141478da2:
          uVar8 = (uint)uStack_a8 & 0xffffff00;
          uVar10 = uStack_a8;
          param_1 = puStack_68;
          in_R11 = uStack_b0;
          lVar17 = lStack_50;
          goto LAB_141478dc0;
        }
        uStack_9c = uVar1;
        if (uVar8 == 0) {
          uVar13 = 1;
        }
        else {
          uVar13 = func_0x000140613c10(uVar12,1);
          if (uVar13 == 0) {
            FUN_1428d8fe3(1,uVar12,&UNK_142c65160);
            pcVar6 = (code *)swi(3);
            uVar7 = (*pcVar6)();
            return uVar7;
          }
        }
        func_0x000142923c00(uVar13,puVar14);
        *plVar3 = (longlong)puVar14 + uVar12;
        uStack_80 = uVar12;
        uStack_58 = uVar13;
        FUN_140d88f60(&iStack_108,uVar13,uVar12);
        puVar15 = puStack_60;
        uVar13 = uStack_80;
        if (iStack_108 == 1) {
          if ((int)uStack_80 == 0) {
            uStack_90 = 0;
          }
          else {
            func_0x000140613c20(uStack_58,uStack_80,1);
            uStack_90 = uVar13;
          }
          uVar13 = 2;
          goto LAB_141478da2;
        }
        in_R11 = (longlong)puStack_70 << 0x20 | uVar10;
        uStack_98 = uStack_80;
        uStack_90 = uStack_80;
        uStack_a8 = uStack_58;
        uVar10 = 0;
        puVar9 = &uStack_b8;
        param_1 = puStack_68;
        uVar13 = uStack_58;
        in_R10 = uStack_80;
        lVar17 = lStack_50;
      }
      *puVar9 = uVar11;
      uVar10 = in_R10 & 0xffffffff | uVar10 & 0xffffffff00000000;
      if (SBORROW8(0,uStack_98)) {
        uStack_88 = uVar13;
        puStack_70 = (undefined4 *)in_R11;
        uStack_58 = uVar10;
        if ((int)*puStack_78 == 0xf) {
          FUN_1405e7a00(puStack_78 + 1);
        }
        *puStack_78 = uStack_88;
        puStack_78[1] = uStack_58;
        puStack_78[2] = (ulonglong)puStack_70;
        return 1;
      }
      uStack_e0 = uStack_98;
      uStack_c0 = uStack_b8;
      uStack_d8 = uVar13;
      uStack_d0 = uVar10;
      uStack_c8 = in_R11;
      FUN_141529330(&iStack_108,uStack_88);
      lVar5 = CONCAT44(uStack_104,iStack_108);
      if ((!SBORROW8(0,lVar5)) && (lVar5 != 0)) {
        func_0x000140613c20(uStack_100,lVar5,1);
      }
    } while (lVar17 + uVar16 != 0);
  }
  return 0;
}

