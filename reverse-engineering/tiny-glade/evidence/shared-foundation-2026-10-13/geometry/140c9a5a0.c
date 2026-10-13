utils::geometry::rectangle::Rectangle2d::get_edge_normal

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c9a5a0(undefined8 *param_1,ulonglong param_2)

{
  code *pcVar1;
  float fVar3;
  undefined8 uVar2;
  float fVar4;
  float fVar5;
  
  if (3 < param_2) {
    FUN_1428d9430(&UNK_142b2e8b0,0x2b,&UNK_142b2e8e0);
    pcVar1 = (code *)swi(3);
    uVar2 = (*pcVar1)();
    return uVar2;
  }
  uVar2 = _DAT_1429ddd00;
  if (param_2 != 3) {
    uVar2 = CONCAT44(*(undefined4 *)(&UNK_142b2ea84 + param_2 * 4),
                     *(undefined4 *)(&UNK_142b2ea90 + param_2 * 4));
  }
  fVar3 = (float)((ulonglong)uVar2 >> 0x20);
  fVar5 = (float)((ulonglong)*param_1 >> 0x20);
  fVar4 = (float)*param_1;
  return CONCAT44(fVar4 * fVar3 + fVar5 * (float)uVar2,
                  (float)((uint)fVar5 ^ _DAT_1429258f0) * fVar3 + fVar4 * (float)uVar2);
}

