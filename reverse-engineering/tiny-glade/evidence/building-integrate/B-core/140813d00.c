
/* WARNING: Instruction at (ram,0x000140814166) overlaps instruction at (ram,0x000140814162)
    */
/* WARNING: Control flow encountered bad instruction data */

void __cdecl system_decorator::derive_decorator_subtype::derive_decorator_subtype(void)

{
  longlong lVar1;
  byte *pbVar2;
  uint uVar3;
  undefined8 *puVar4;
  longlong lVar5;
  undefined *puVar6;
  char cVar7;
  char extraout_AL;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  byte bVar8;
  undefined8 *in_RCX;
  ulonglong uVar9;
  undefined *puVar10;
  longlong in_RDX;
  longlong lVar11;
  longlong unaff_RBP;
  undefined *puVar12;
  longlong *in_R9;
  bool bVar13;
  undefined1 auStack_90 [8];
  undefined *puStack_88;
  undefined8 uStack_80;
  undefined1 *puStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  char *local_50;
  undefined8 local_48;
  
  puVar4 = (undefined8 *)*in_RCX;
  local_50 = (char *)in_RCX[1];
  local_48 = in_RCX[2];
  bVar8 = *(char *)((longlong)puVar4 + 0x46) - 0x38;
  bVar13 = bVar8 < 8;
  puVar10 = (undefined *)(ulonglong)bVar8;
  puVar12 = (undefined *)0x6;
code_r0x000140813d41:
  puVar6 = puVar12;
  if (bVar13) {
    puVar6 = puVar10;
  }
  puVar12 = &DAT_142a8f990;
  uVar9 = (ulonglong)*(int *)(&DAT_142a8f990 + ((ulonglong)puVar6 & 0xff) * 4);
  bVar13 = 0xfffffffebd57066f < uVar9;
  puVar10 = &DAT_142a8f990 + uVar9;
  cVar7 = (char)local_50;
                    /* WARNING (jumptable): Sanity check requires truncation of jumptable */
                    /* WARNING: Could not find normalized switch variable to match jumptable */
  switch((ulonglong)puVar6 & 0xff) {
  default:
    goto code_r0x000140813d59;
  case 2:
    if (1 < *(byte *)((longlong)puVar4 + 0x22)) {
      puStack_88 = &UNK_142a8f6c8;
      uStack_80 = 1;
      puStack_78 = auStack_90;
      uStack_70 = 0;
      uStack_68 = 0;
      goto code_r0x000140814337;
    }
    puStack_88 = (undefined *)*puVar4;
    if (local_50 != (char *)0x0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX == 0) {
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
    country_core::resources::walls::public_walls::PublicWalls::roof_entity();
    if (extraout_RAX_00 >> 0x20 != 0) {
      lVar11 = in_R9[1];
      uVar9 = extraout_RAX_00 & 0xffffffff;
      if ((uVar9 < *(ulonglong *)(lVar11 + 0x10)) &&
         (*(int *)(*(longlong *)(lVar11 + 8) + uVar9 * 0x14) == (int)(extraout_RAX_00 >> 0x20))) {
        lVar1 = *(longlong *)(lVar11 + 8) + uVar9 * 0x14;
        uVar3 = *(uint *)(lVar1 + 4);
        uVar9 = (ulonglong)uVar3;
        if ((uVar9 != 0xffffffff) &&
           ((lVar5 = *in_R9, uVar9 < *(ulonglong *)(lVar5 + 0x38) &&
            ((*(ulonglong *)(*(longlong *)(lVar5 + 0x28) + (ulonglong)(uVar3 >> 6) * 8) >>
              (uVar9 & 0x3f) & 1) != 0)))) {
          uVar9 = (ulonglong)*(uint *)(lVar1 + 0xc);
          if ((*(ulonglong *)(lVar5 + 0x110) <
               *(ulonglong *)(*(longlong *)(lVar11 + 0x1a8) + 0x40 + uVar9 * 0x48)) &&
             (*(longlong *)
               (*(longlong *)(*(longlong *)(lVar11 + 0x1a8) + uVar9 * 0x48 + 0x38) +
               *(ulonglong *)(lVar5 + 0x110) * 8) != 0)) goto code_r0x000140813fc5;
          goto code_r0x0001408142fc;
        }
      }
    }
    goto code_r0x00014081413b;
  case 3:
    lVar11 = 0;
    if ((byte)(*(byte *)((longlong)puVar4 + 0x22) - 2) < 3) {
      lVar11 = (ulonglong)*(byte *)((longlong)puVar4 + 0x22) - 1;
    }
                    /* WARNING: Could not emulate address calculation at 0x000140813fe3 */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142a8f9c0 + *(int *)(&UNK_142a8f9c0 + lVar11 * 4)))
              (&UNK_142a8f9c0,&UNK_142a8f9c0 + *(int *)(&UNK_142a8f9c0 + lVar11 * 4));
    return;
  case 4:
    break;
  case 5:
    goto code_r0x000140814003;
  case 6:
    break;
  case 7:
    break;
  case 8:
    pbVar2 = (byte *)(local_50 + -0x73);
    *pbVar2 = *pbVar2 >> 1 | *pbVar2 << 7;
    goto code_r0x000140814017;
  case 9:
    *local_50 = *local_50 + cVar7;
    goto code_r0x000140814167;
  case 10:
    if ((longlong)uVar9 < -0x142a8f990) {
      local_50[0x156e90e] = local_50[0x156e90e] + (char)((ulonglong)in_RDX >> 8);
      *local_50 = *local_50 + cVar7;
      country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
      country_core::resources::walls::public_walls::PublicWalls::roof_entity();
      if (extraout_RAX_01 >> 0x20 == 0) {
        return;
      }
      uVar9 = extraout_RAX_01 & 0xffffffff;
      if (*(ulonglong *)(in_R9[1] + 0x10) <= uVar9) {
        return;
      }
      lVar11 = *(longlong *)(in_R9[1] + 8);
      if (*(int *)(lVar11 + uVar9 * 0x14) != (int)(extraout_RAX_01 >> 0x20)) {
        return;
      }
      uVar3 = *(uint *)(lVar11 + uVar9 * 0x14 + 4);
      uVar9 = (ulonglong)uVar3;
      if (uVar9 == 0xffffffff) {
        return;
      }
      if (*(ulonglong *)(*in_R9 + 0x38) <= uVar9) {
        return;
      }
      if ((*(ulonglong *)(*(longlong *)(*in_R9 + 0x28) + (ulonglong)(uVar3 >> 6) * 8) >>
           (uVar9 & 0x3f) & 1) == 0) {
        return;
      }
      country_core::resources::roofs::roof_shape::Roof::is_flat();
      return;
    }
    pbVar2 = (byte *)(unaff_RBP + 0x44 + (longlong)puVar10 * 4);
    *pbVar2 = *pbVar2 & (byte)puVar10;
    goto code_r0x000140814167;
  case 0xb:
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  case 0xc:
    is_half_dormer();
    break;
  case 0xd:
code_r0x000140813fc5:
  case 0x13:
code_r0x0001408142fc:
code_r0x00014081413b:
    country_core::resources::walls::wall_attachment::WallCornerAttachment::is_attached();
    if (extraout_AL != '\0') {
      return;
    }
code_r0x000140814167:
    is_half_dormer();
    break;
  case 0xe:
    puStack_88 = (undefined *)CONCAT44(puStack_88._4_4_,(int)local_50);
    uStack_80 = 1;
    puStack_78 = auStack_90;
    uStack_70 = 0;
    uStack_68 = 0;
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  case 0xf:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  case 0x10:
    goto code_r0x000140813d41;
  case 0x11:
    if (puVar10 < (undefined *)0x2) {
      *local_50 = *local_50 + cVar7;
    }
    goto code_r0x000140814003;
  case 0x12:
code_r0x000140814337:
                    /* WARNING: Subroutine does not return */
    core::panicking::panic_fmt();
  }
  return;
code_r0x000140813d59:
  lVar11 = 0;
  if ((byte)(*(byte *)((longlong)puVar4 + 0x22) - 2) < 3) {
    lVar11 = (ulonglong)*(byte *)((longlong)puVar4 + 0x22) - 1;
  }
                    /* WARNING: Could not emulate address calculation at 0x000140813d70 */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&UNK_142a8f9d0 + *(int *)(&UNK_142a8f9d0 + lVar11 * 4)))
            (&UNK_142a8f9d0,in_RDX,&UNK_142a8f9d0 + *(int *)(&UNK_142a8f9d0 + lVar11 * 4));
  return;
code_r0x000140814003:
  in_RDX = 0;
  if ((byte)(*(byte *)((longlong)puVar4 + 0x22) - 2) < 3) {
    in_RDX = (ulonglong)*(byte *)((longlong)puVar4 + 0x22) - 1;
  }
code_r0x000140814017:
                    /* WARNING: Could not recover jumptable at 0x000140814025. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&UNK_142a8f9b0 + *(int *)(&UNK_142a8f9b0 + in_RDX * 4)))
            (&UNK_142a8f9b0 + *(int *)(&UNK_142a8f9b0 + in_RDX * 4));
  return;
}

