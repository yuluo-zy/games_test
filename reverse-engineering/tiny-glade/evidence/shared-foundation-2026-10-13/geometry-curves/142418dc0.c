
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_142418dc0(undefined4 *param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  float fVar5;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  
  uStack_80 = param_2[2];
  uVar1 = param_2[3];
  uVar3 = *(ulonglong *)(param_2 + 4);
  uStack_78 = param_2[3];
  uStack_74 = param_2[4];
  uStack_70 = (undefined4)(uVar3 >> 0x20);
  uVar2 = param_2[6];
  uVar4 = *(ulonglong *)(param_2 + 7);
  uStack_68 = param_2[6];
  uStack_64 = param_2[7];
  uStack_60 = (undefined4)(uVar4 >> 0x20);
  uStack_88 = *param_2;
  uStack_84 = param_2[1];
  uStack_50 = param_1[2];
  uStack_58 = *param_1;
  uStack_54 = param_1[1];
  uStack_7c = _UNK_14293e0cc;
  uStack_6c = _UNK_14293e0cc;
  uStack_5c = _UNK_14293e0cc;
  uStack_4c = _UNK_14293e0cc;
  fVar5 = (float)func_0x0001412b7c50(&uStack_88);
  if (fVar5 < _DAT_142fcb220) {
    *(char *)(param_2 + 6) = (char)uVar1;
    *(char *)(param_2 + 3) = (char)uVar2;
    *(char *)((longlong)param_2 + 0x19) = (char)((uint)uVar1 >> 8);
    *(char *)((longlong)param_2 + 0xd) = (char)((uint)uVar2 >> 8);
    *(char *)((longlong)param_2 + 0x1a) = (char)((uint)uVar1 >> 0x10);
    *(char *)((longlong)param_2 + 0xe) = (char)((uint)uVar2 >> 0x10);
    *(char *)((longlong)param_2 + 0x1b) = (char)((uint)uVar1 >> 0x18);
    *(char *)((longlong)param_2 + 0xf) = (char)((uint)uVar2 >> 0x18);
    *(char *)(param_2 + 7) = (char)(uVar3 & 0xffffffff);
    *(char *)(param_2 + 4) = (char)(uVar4 & 0xffffffff);
    *(char *)((longlong)param_2 + 0x1d) = (char)((uVar3 & 0xffffffff) >> 8);
    *(char *)((longlong)param_2 + 0x11) = (char)((uVar4 & 0xffffffff) >> 8);
    *(char *)((longlong)param_2 + 0x1e) = (char)(uVar3 >> 0x10);
    *(char *)((longlong)param_2 + 0x12) = (char)(uVar4 >> 0x10);
    *(char *)((longlong)param_2 + 0x1f) = (char)(uVar3 >> 0x18);
    *(char *)((longlong)param_2 + 0x13) = (char)(uVar4 >> 0x18);
    *(char *)(param_2 + 8) = (char)(uVar3 >> 0x20);
    *(char *)(param_2 + 5) = (char)(uVar4 >> 0x20);
    *(char *)((longlong)param_2 + 0x21) = (char)(uVar3 >> 0x28);
    *(char *)((longlong)param_2 + 0x15) = (char)(uVar4 >> 0x28);
    *(char *)((longlong)param_2 + 0x22) = (char)(uVar3 >> 0x30);
    *(char *)((longlong)param_2 + 0x16) = (char)(uVar4 >> 0x30);
    *(undefined1 *)((longlong)param_2 + 0x23) = *(undefined1 *)((longlong)param_2 + 0x17);
    *(char *)((longlong)param_2 + 0x17) = (char)(uVar4 >> 0x38);
  }
  return;
}

