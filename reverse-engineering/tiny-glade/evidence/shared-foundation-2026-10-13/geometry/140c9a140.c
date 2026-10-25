utils::geometry::rectangle::Rectangle2d::get_corner_normal

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 FUN_140c9a140(undefined8 *param_1,undefined8 param_2)

{
  code *pcVar1;
  float fVar2;
  float fVar3;
  float fVar5;
  undefined8 uVar4;
  float fVar6;
  float fVar7;
  
  fVar5 = _DAT_142925904;
  fVar2 = _DAT_14295a0e4;
  switch(param_2) {
  case 0:
    fVar5 = _DAT_14295a0e4;
    break;
  case 1:
    fVar5 = _DAT_14295a0e4;
    fVar2 = _DAT_142925904;
    break;
  case 2:
    fVar2 = _DAT_142925904;
    break;
  case 3:
    break;
  default:
    FUN_1428d9430(&UNK_142b2e848,0x1f,&UNK_142b2e868);
    pcVar1 = (code *)swi(3);
    uVar4 = (*pcVar1)();
    return uVar4;
  }
  fVar7 = (float)((ulonglong)*param_1 >> 0x20);
  fVar6 = (float)*param_1;
  fVar3 = (float)((uint)fVar7 ^ _DAT_1429258f0) * fVar2 + fVar6 * fVar5;
  fVar5 = fVar6 * fVar2 + fVar7 * fVar5;
  return CONCAT44(fVar5,fVar3 * (_DAT_142925904 / SQRT(fVar5 * fVar5 + fVar3 * fVar3)));
}

