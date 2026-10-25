
void FUN_142879de0(uint *param_1,uint *param_2)

{
  byte bVar1;
  char cVar2;
  uint uVar3;
  byte bVar4;
  undefined8 uStack_10;
  uint uStack_8;
  
  if ((char)param_1[0x10] == '\0') {
    *(char *)(param_1 + 0x10) = (char)param_2[0x10];
    uVar3 = *param_1;
  }
  else {
    uVar3 = *param_1;
  }
  if (uVar3 == 0) {
    if (1 < *param_2) {
      uStack_8 = param_2[3];
      uStack_10 = *(undefined8 *)(param_2 + 1);
    }
    *param_1 = *param_2;
    *(undefined8 *)(param_1 + 1) = uStack_10;
    param_1[3] = uStack_8;
    uVar3 = param_1[4];
  }
  else {
    uVar3 = param_1[4];
  }
  if (uVar3 == 0) {
    if (1 < param_2[4]) {
      uStack_8 = param_2[7];
      uStack_10 = *(undefined8 *)(param_2 + 5);
    }
    param_1[4] = param_2[4];
    *(undefined8 *)(param_1 + 5) = uStack_10;
    param_1[7] = uStack_8;
    uVar3 = param_1[8];
  }
  else {
    uVar3 = param_1[8];
  }
  if (uVar3 == 0) {
    if (1 < param_2[8]) {
      uStack_8 = param_2[0xb];
      uStack_10 = *(undefined8 *)(param_2 + 9);
    }
    param_1[8] = param_2[8];
    *(undefined8 *)(param_1 + 9) = uStack_10;
    param_1[0xb] = uStack_8;
    uVar3 = param_1[0xc];
  }
  else {
    uVar3 = param_1[0xc];
  }
  if (uVar3 == 0) {
    if (1 < param_2[0xc]) {
      uStack_8 = param_2[0xf];
      uStack_10 = *(undefined8 *)(param_2 + 0xd);
    }
    param_1[0xc] = param_2[0xc];
    *(undefined8 *)(param_1 + 0xd) = uStack_10;
    param_1[0xf] = uStack_8;
    cVar2 = *(char *)((longlong)param_1 + 0x41);
  }
  else {
    cVar2 = *(char *)((longlong)param_1 + 0x41);
  }
  if (cVar2 == '\x02') {
    bVar1 = *(byte *)((longlong)param_2 + 0x41) - 2;
    bVar4 = 2;
    if (bVar1 < 2) {
      bVar4 = bVar1;
    }
    if (bVar4 != 0) {
      if (bVar4 == 1) {
        *(undefined1 *)((longlong)param_1 + 0x41) = 3;
        return;
      }
      *(byte *)((longlong)param_1 + 0x41) = *(byte *)((longlong)param_2 + 0x41) & 1;
      return;
    }
    *(undefined1 *)((longlong)param_1 + 0x41) = 2;
    return;
  }
  return;
}

