
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void particle_transform(void *p0,void *p1,void *p2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  undefined8 *puVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  
  puVar5 = (undefined8 *)**(undefined8 **)p1;
  fVar13 = (float)*puVar5;
  fVar14 = (float)((ulonglong)*puVar5 >> 0x20);
  fVar1 = *(float *)(puVar5 + 1);
  if ((_DAT_142934620 <= SQRT(fVar1 * fVar1 + fVar14 * fVar14 + fVar13 * fVar13)) ||
     (fVar2 = *(float *)((longlong)puVar5 + 0xc), fVar2 <= _DAT_1429cd030)) {
    *(undefined4 *)p0 = 0;
  }
  else {
    fVar12 = *(float *)(puVar5 + 2) * _DAT_142925870;
    fVar6 = (float)func_0x000142923fd0(fVar12);
    fVar7 = (float)func_0x000142923e80(fVar12);
    fVar8 = fVar6 * 0.0;
    fVar9 = fVar6 + fVar6;
    fVar10 = fVar8 + fVar8;
    fVar15 = fVar8 * fVar10;
    fVar11 = fVar10 * fVar7;
    fVar16 = _DAT_142925904 - (fVar6 * fVar9 + fVar15);
    fVar12 = *(float *)p2;
    fVar3 = *(float *)((longlong)p2 + 4);
    fVar4 = *(float *)((longlong)p2 + 8);
    *(ulonglong *)((longlong)p0 + 4) =
         CONCAT44(fVar4 * fVar2 * (fVar6 * fVar10 - fVar11) +
                  fVar3 * fVar2 * (_DAT_142925904 - (fVar15 + fVar15)) +
                  fVar12 * fVar2 * (fVar8 * fVar9 + fVar11) + fVar14,
                  fVar4 * fVar2 * (fVar9 * fVar7 + fVar15) +
                  fVar3 * fVar2 * (fVar8 * fVar9 - fVar11) + fVar12 * fVar2 * fVar16 + fVar13);
    *(float *)((longlong)p0 + 0xc) =
         fVar4 * fVar2 * fVar16 +
         fVar3 * fVar2 * (fVar6 * fVar10 + fVar11) + fVar12 * fVar2 * (fVar15 - fVar9 * fVar7) +
         fVar1;
    *(undefined4 *)((longlong)p0 + 0x10) = 0;
    *(undefined4 *)p0 = 1;
  }
  return;
}

