
undefined8 * asset__prefab__PrefabName__new(undefined8 param_1,longlong param_2)
{
  code *pcVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  longlong lVar4;
  undefined1 uStack_21;
  if (-1 < param_2) {
    lVar4 = param_2;
    puVar2 = (undefined8 *)FUN_140d7c180(1);
    puVar3 = puVar2;
    if (lVar4 != 0) {
      puVar3 = (undefined8 *)func_0x000140613c10(lVar4,puVar2);
    }
    if (puVar3 != (undefined8 *)0x0) {
      *puVar3 = 1;
      puVar3[1] = 1;
      func_0x000142923c00(puVar3 + 2,param_1,param_2);
      return puVar3;
    }
    FUN_1428d9000(puVar2,lVar4);
  }
  FUN_1428d9760(&UNK_142bb2e90,0x2b,&uStack_21,&UNK_142bb2e70,&UNK_142bb2f08);
  pcVar1 = (code *)swi(3);
  puVar3 = (undefined8 *)(*pcVar1)();
  return puVar3;
}
