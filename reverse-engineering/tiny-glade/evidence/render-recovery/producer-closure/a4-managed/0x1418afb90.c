
void FUN_1418afb90(longlong param_1,longlong param_2)

{
  undefined8 *puVar1;
  char cVar2;
  undefined1 uVar3;
  undefined2 uVar4;
  undefined4 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  longlong lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  
  cVar2 = *(char *)(param_1 + 0x6c);
  *(undefined1 *)(param_1 + 0x6c) = 2;
  if (cVar2 != '\x02') {
    uVar5 = *(undefined4 *)(param_1 + 0x68);
    uVar9 = *(undefined8 *)(param_1 + 0x28);
    uVar10 = *(undefined8 *)(param_1 + 0x30);
    uVar11 = *(undefined8 *)(param_1 + 0x38);
    uVar12 = *(undefined8 *)(param_1 + 0x40);
    uVar13 = *(undefined8 *)(param_1 + 0x48);
    uVar14 = *(undefined8 *)(param_1 + 0x50);
    uVar15 = *(undefined8 *)(param_1 + 0x58);
    uVar16 = *(undefined8 *)(param_1 + 0x60);
    uVar3 = *(undefined1 *)(param_1 + 0x6f);
    uVar4 = *(undefined2 *)(param_1 + 0x6d);
    uVar6 = *(undefined8 *)(param_1 + 0x20);
    lVar7 = *(longlong *)(param_2 + 0x40);
    if (lVar7 == *(longlong *)(param_2 + 0x30)) {
      FUN_140562870(param_2 + 0x30,&UNK_142d05308);
    }
    *(undefined8 *)(*(longlong *)(param_2 + 0x38) + lVar7 * 8) = uVar6;
    *(longlong *)(param_2 + 0x40) = lVar7 + 1;
    lVar7 = *(longlong *)(param_2 + 0xe8);
    if (lVar7 == *(longlong *)(param_2 + 0xd8)) {
      FUN_140b5f9f0(param_2 + 0xd8,&UNK_142d053d8);
    }
    lVar8 = *(longlong *)(param_2 + 0xe0);
    *(ulonglong *)(lVar8 + 0x40 + lVar7 * 0x48) =
         CONCAT17(uVar3,CONCAT25(uVar4,CONCAT14(cVar2,uVar5)));
    puVar1 = (undefined8 *)(lVar8 + 0x30 + lVar7 * 0x48);
    *puVar1 = uVar15;
    puVar1[1] = uVar16;
    puVar1 = (undefined8 *)(lVar8 + 0x20 + lVar7 * 0x48);
    *puVar1 = uVar13;
    puVar1[1] = uVar14;
    puVar1 = (undefined8 *)(lVar8 + 0x10 + lVar7 * 0x48);
    *puVar1 = uVar11;
    puVar1[1] = uVar12;
    puVar1 = (undefined8 *)(lVar8 + lVar7 * 0x48);
    *puVar1 = uVar9;
    puVar1[1] = uVar10;
    *(longlong *)(param_2 + 0xe8) = lVar7 + 1;
  }
  return;
}

