
longlong * FUN_140bd63f0(longlong *param_1,longlong *param_2)

{
  code *pcVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  
  if (*param_2 == 0) {
    param_1[6] = param_2[7];
    lVar2 = param_2[1];
    lVar3 = param_2[2];
    lVar4 = param_2[3];
    lVar5 = param_2[4];
    lVar6 = param_2[6];
    param_1[4] = param_2[5];
    param_1[5] = lVar6;
    param_1[2] = lVar4;
    param_1[3] = lVar5;
    *param_1 = lVar2;
    param_1[1] = lVar3;
    return param_1;
  }
  FUN_1428ee9f0(&UNK_142b0fef0);
                    /* WARNING: Does not return */
  pcVar1 = (code *)invalidInstructionException();
  (*pcVar1)();
}

