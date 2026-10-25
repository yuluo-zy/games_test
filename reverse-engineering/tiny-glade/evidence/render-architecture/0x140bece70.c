bool FUN_140bece70(int *param_1,int *param_2)
{
  char cVar1;
  char cVar2;
  char cVar3;
  bool bVar4;
  cVar1 = (char)param_1[1];
  cVar3 = func_0x000140c10e10(cVar1);
  cVar2 = (char)param_2[1];
  if ((cVar3 == '\0') && (cVar3 = func_0x000140c10e10(cVar2), cVar3 == '\0')) {
    return cVar1 == cVar2;
  }
  if (cVar1 == cVar2) {
    bVar4 = *param_1 != 0 && *param_1 == *param_2;
  }
  else {
    bVar4 = false;
  }
  return bVar4;
}
