
void FUN_140bed230(longlong *param_1,longlong param_2,uint param_3,int param_4)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  code *pcVar4;
  int iVar5;
  ulonglong uVar6;
  int iStack_54;
  int aiStack_50 [2];
  undefined8 uStack_48;
  int iStack_40;
  undefined8 uStack_38;
  
  aiStack_50[0] = CONCAT31(aiStack_50[0]._1_3_,0x17);
  FUN_1418c1380(0x143634e40,aiStack_50,1);
  if ((*param_1 != 0) && ((int)*param_1 != 3)) goto code_r0x000140bed3b4;
  iVar5 = *(int *)(*(longlong *)(param_2 + 0x658) + 0x78);
  lVar3 = param_1[9];
  aiStack_50[0] = (**(code **)(param_2 + 0x2d0))(lVar3,0);
  if (aiStack_50[0] == 0) {
    iStack_40 = (uint)(iVar5 == 0) * 3 + 1;
    aiStack_50[0] = 0x2a;
    uStack_48 = 0;
    uStack_38 = 0;
    iVar5 = (**(code **)(param_2 + 0x2c0))(lVar3,aiStack_50);
    if (iVar5 != 0) goto code_r0x000140bed37b;
    uVar6 = 0;
    if ((param_3 & 1) != 0) {
      uVar6 = (ulonglong)(param_4 + 1);
    }
    if (uVar6 < *(ulonglong *)(param_2 + 0x670)) {
      LOCK();
      plVar1 = (longlong *)(*(longlong *)(param_2 + 0x668) + uVar6 * 8);
      lVar2 = *plVar1;
      *plVar1 = *plVar1 + 1;
      UNLOCK();
      *param_1 = 1;
      param_1[1] = lVar2;
      *(uint *)(param_1 + 2) = param_3;
      *(int *)((longlong)param_1 + 0x14) = param_4;
      *(undefined1 *)((longlong)param_1 + 0x56) = 0;
      (**(code **)(param_2 + 800))(lVar3,3,0);
      FUN_1418c4260(param_2 + 0xa50,lVar2);
      return;
    }
  }
  else {
    iVar5 = FUN_1428d9760(&UNK_142b14c38,0x14,aiStack_50,&UNK_142b14b30,&UNK_142b14c50);
code_r0x000140bed37b:
    iStack_54 = iVar5;
    FUN_1428d9760(&UNK_142b14c68,0x14,&iStack_54,&UNK_142b14b30,&UNK_142b14c80);
  }
  FUN_1428d9518();
code_r0x000140bed3b4:
  FUN_1428d9430(&UNK_142b14c98,0x6e,&UNK_142b14d08);
  pcVar4 = (code *)swi(3);
  (*pcVar4)();
  return;
}

