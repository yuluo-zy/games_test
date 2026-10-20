
void FUN_140912a40(longlong *param_1,undefined8 *param_2,longlong *param_3)

{
  longlong lVar1;
  longlong *plVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined4 *puVar5;
  undefined8 uStack_518;
  undefined *puStack_510;
  undefined8 uStack_508;
  undefined8 uStack_500;
  undefined4 uStack_4f8;
  undefined4 uStack_4f4;
  undefined4 uStack_4f0;
  undefined4 uStack_4ec;
  undefined8 uStack_4e8;
  undefined *puStack_4e0;
  undefined8 uStack_4d8;
  undefined4 uStack_4d0;
  undefined4 uStack_4cc;
  undefined *puStack_4c8;
  undefined8 uStack_4c0;
  undefined8 uStack_4b8;
  undefined8 uStack_4b0;
  undefined8 uStack_4a8;
  longlong lStack_38;
  undefined1 uStack_29;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  if (4 < *puRam000000014362c3a8) {
    puVar5 = (undefined4 *)func_0x00014067dd80(&UNK_142aabb08);
    uStack_4f8 = *puVar5;
    uStack_4f4 = puVar5[1];
    uStack_4f0 = puVar5[2];
    uStack_4ec = puVar5[3];
    uStack_4cc = puVar5[4];
    uStack_4e8 = 5;
    puStack_4e0 = &UNK_142aabb20;
    uStack_4d8 = 0x22;
    puStack_4c8 = &UNK_142aabaf8;
    uStack_4c0 = 1;
    uStack_4b8 = 8;
    uStack_518 = 0;
    uStack_4b0 = 0;
    uStack_4a8 = 0;
    puStack_510 = &UNK_142aabb20;
    uStack_508 = 0x22;
    uStack_500 = 0;
    uStack_4d0 = 1;
    func_0x000140d4a330(&uStack_29,&uStack_518);
  }
  *(undefined4 *)param_1[2] = *(undefined4 *)((longlong)param_1 + 0x1c);
  lVar1 = *param_1;
  FUN_14090ec70(&uStack_518,*(undefined4 *)(lVar1 + 0x4d8));
  lStack_38 = lVar1;
  FUN_1405f9ce0(lVar1);
  func_0x000142923c00(lStack_38,&uStack_518,0x4e0);
  *(undefined4 *)param_2[2] = *(undefined4 *)((longlong)param_2 + 0x1c);
  plVar2 = (longlong *)*param_2;
  if (*plVar2 != 0) {
    func_0x000140613c20(plVar2[1],*plVar2 * 0x24,4);
  }
  *plVar2 = 0;
  plVar2[1] = 4;
  plVar2[2] = 0;
  *(undefined4 *)(plVar2 + 3) = 0;
  plVar2[4] = 0;
  *(undefined4 *)param_3[2] = *(undefined4 *)((longlong)param_3 + 0x1c);
  lVar1 = *param_3;
  lVar3 = *(longlong *)(lVar1 + 0x30);
  uVar4 = *(undefined8 *)(lVar1 + 0x40);
  if (lVar3 == *(longlong *)(lVar1 + 0x20)) {
    FUN_140562870(lVar1 + 0x20,&UNK_142c89078);
  }
  *(undefined8 *)(*(longlong *)(lVar1 + 0x28) + lVar3 * 8) = uVar4;
  *(longlong *)(lVar1 + 0x30) = lVar3 + 1;
  *(longlong *)(lVar1 + 0x40) = *(longlong *)(lVar1 + 0x40) + 1;
  return;
}

