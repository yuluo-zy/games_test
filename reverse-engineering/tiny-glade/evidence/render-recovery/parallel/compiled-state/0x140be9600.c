
void FUN_140be9600(longlong *param_1,longlong param_2)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  
  if (param_1[3] != param_2) {
    param_1[4] = param_2;
    lVar2 = param_1[5];
    lVar5 = param_1[3];
    lVar6 = param_1[4];
    param_1[3] = 0;
    param_1[4] = 0;
    *(undefined1 *)(param_1 + 5) = 0;
    lVar3 = param_1[2];
    if (lVar3 == *param_1) {
      FUN_140562db0(0,&UNK_142b14278);
    }
    lVar4 = param_1[1];
    *(longlong *)(lVar4 + 0x10 + lVar3 * 0x18) = lVar2;
    plVar1 = (longlong *)(lVar4 + lVar3 * 0x18);
    *plVar1 = lVar5;
    plVar1[1] = lVar6;
    param_1[2] = lVar3 + 1;
    param_1[3] = param_2;
  }
  return;
}

