
undefined2 *
FUN_140bd5a80(undefined2 *param_1,uint *param_2,undefined1 param_3,char param_4,ushort param_5)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  code *pcVar4;
  undefined *puVar5;
  undefined2 *puVar6;
  undefined1 uStack_1;
  
  if (param_4 == '\0') {
    FUN_1428d9430(&UNK_142b0fb20,0x25,&UNK_142b0fb48);
code_r0x000140bd5b34:
    FUN_1428d9430(&UNK_142b0fb60,0x1e,&UNK_142b0fb80);
code_r0x000140bd5b4c:
    FUN_1428d9430(&UNK_142b0fb98,0x1e,&UNK_142b0fbb8);
code_r0x000140bd5b64:
    FUN_1428d9430(&UNK_142b0fbd0,0x1f,&UNK_142b0fbf0);
code_r0x000140bd5b7c:
    FUN_1428d9430(&UNK_142b0fc08,0x1f,&UNK_142b0fc28);
code_r0x000140bd5b94:
    FUN_1428d9430(&UNK_142b0fc40,0x1f,&UNK_142b0fc60);
  }
  else {
    if ((char)param_5 == '\0') goto code_r0x000140bd5b34;
    if (param_5 < 0x100) goto code_r0x000140bd5b4c;
    uVar1 = *param_2;
    if (uVar1 == 0) goto code_r0x000140bd5b64;
    uVar2 = param_2[1];
    if (uVar2 == 0) goto code_r0x000140bd5b7c;
    uVar3 = param_2[2];
    if (uVar3 == 0) goto code_r0x000140bd5b94;
    if (uVar1 < 0x10000) {
      if (uVar2 < 0x10000) {
        if (uVar3 < 0x10000) {
          *param_1 = (short)uVar1;
          param_1[1] = (short)uVar2;
          param_1[2] = (short)uVar3;
          *(undefined1 *)(param_1 + 4) = 0;
          *(undefined1 *)((longlong)param_1 + 9) = param_3;
          *(char *)(param_1 + 5) = param_4;
          *(char *)(param_1 + 3) = (char)param_5;
          *(char *)((longlong)param_1 + 7) = (char)(param_5 >> 8);
          return param_1;
        }
        puVar5 = &UNK_142b0fca8;
      }
      else {
        puVar5 = &UNK_142b0fc90;
      }
      goto code_r0x000140bd5bc5;
    }
  }
  puVar5 = &UNK_142b0fc78;
code_r0x000140bd5bc5:
  FUN_1428d9760(&UNK_142b0f9a0,0x2b,&uStack_1,&UNK_142b0f980,puVar5);
  pcVar4 = (code *)swi(3);
  puVar6 = (undefined2 *)(*pcVar4)();
  return puVar6;
}

