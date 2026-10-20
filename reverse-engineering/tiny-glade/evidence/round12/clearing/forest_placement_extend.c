
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void forest_placement_extend(void *p0,void *p1,void *p2)

{
  uint *puVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined4 uVar4;
  uint *puVar5;
  undefined8 *puVar6;
  undefined4 *puVar7;
  longlong *plVar8;
  undefined8 uVar9;
  longlong lVar10;
  code *pcVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  uint uVar18;
  float fVar19;
  uint *puVar20;
  longlong lVar21;
  undefined4 uStack_84;
  
  fVar19 = _DAT_142a83814;
  uVar18 = _DAT_14292e980;
  puVar5 = *(uint **)((longlong)p1 + 8);
  if (*(uint **)p1 != puVar5) {
    puVar6 = *(undefined8 **)((longlong)p1 + 0x10);
    puVar7 = *(undefined4 **)((longlong)p1 + 0x18);
    puVar20 = *(uint **)p1;
    do {
      while ((fVar19 <= (float)(*puVar20 & uVar18) || (fVar19 <= (float)(puVar20[2] & uVar18)))) {
        puVar1 = puVar20 + 0xc;
        *(uint **)p1 = puVar1;
        plVar8 = (longlong *)*puVar6;
        uVar9 = puVar6[1];
        LOCK();
        lVar3 = *plVar8;
        *plVar8 = *plVar8 + 1;
        UNLOCK();
        if (*plVar8 == 0 || SCARRY8(lVar3,1) != *plVar8 < 0) {
                    /* WARNING: Does not return */
          pcVar11 = (code *)invalidInstructionException();
          (*pcVar11)();
        }
        uVar4 = *puVar7;
        uVar12 = *(undefined8 *)puVar20;
        uVar13 = *(undefined8 *)(puVar20 + 2);
        uVar14 = *(undefined8 *)(puVar20 + 4);
        uVar15 = *(undefined8 *)(puVar20 + 6);
        uVar16 = *(undefined8 *)(puVar20 + 8);
        uVar17 = *(undefined8 *)(puVar20 + 10);
        lVar3 = *(longlong *)((longlong)p0 + 0x10);
        if (lVar3 == *(longlong *)p0) {
          FUN_1428a9a60(*puVar20,lVar3,1,8,0x50);
        }
        lVar10 = *(longlong *)((longlong)p0 + 8);
        lVar21 = lVar3 * 0x50;
        puVar2 = (undefined8 *)(lVar10 + 0x40 + lVar21);
        *puVar2 = 0;
        puVar2[1] = CONCAT44(uStack_84,uVar4);
        puVar2 = (undefined8 *)(lVar10 + 0x30 + lVar21);
        *puVar2 = uVar16;
        puVar2[1] = uVar17;
        puVar2 = (undefined8 *)(lVar10 + 0x20 + lVar21);
        *puVar2 = uVar14;
        puVar2[1] = uVar15;
        puVar2 = (undefined8 *)(lVar10 + 0x10 + lVar21);
        *puVar2 = uVar12;
        puVar2[1] = uVar13;
        *(undefined8 *)(lVar10 + lVar21) = plVar8;
        ((undefined8 *)(lVar10 + lVar21))[1] = uVar9;
        *(longlong *)((longlong)p0 + 0x10) = lVar3 + 1;
        puVar20 = puVar1;
        if (puVar1 == puVar5) {
          return;
        }
      }
      puVar20 = puVar20 + 0xc;
    } while (puVar20 != puVar5);
    *(uint **)p1 = puVar20;
  }
  return;
}

