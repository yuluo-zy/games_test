
void __cdecl system_onboarding::tutorial_completion::tutorial_completion(void)

{
  undefined1 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  ulonglong *puVar4;
  longlong lVar5;
  ulonglong uVar6;
  undefined4 *puVar7;
  longlong lVar8;
  undefined4 *puVar9;
  undefined8 uVar10;
  undefined8 *in_RCX;
  ulonglong uVar11;
  longlong *in_RDX;
  longlong lVar12;
  longlong lVar13;
  longlong *in_R8;
  ulonglong uVar14;
  ulonglong uVar15;
  longlong lVar16;
  longlong lVar17;
  longlong lVar18;
  
  puVar4 = (ulonglong *)*in_RCX;
  lVar5 = in_RCX[1];
  uVar6 = *puVar4;
  uVar11 = 0;
  uVar15 = uVar6 - *(ulonglong *)(lVar5 + 0x18);
  if (uVar6 < *(ulonglong *)(lVar5 + 0x18)) {
    uVar15 = uVar11;
  }
  uVar14 = uVar6 - *(ulonglong *)(lVar5 + 0x38);
  if (uVar6 < *(ulonglong *)(lVar5 + 0x38)) {
    uVar14 = uVar11;
  }
  lVar17 = uVar15 * 0x10 + *(longlong *)(lVar5 + 8);
  uVar6 = *(ulonglong *)(lVar5 + 0x10) - uVar15;
  if (*(ulonglong *)(lVar5 + 0x10) < uVar15) {
    lVar17 = 8;
    uVar6 = uVar11;
  }
  lVar16 = uVar14 * 0x10 + *(longlong *)(lVar5 + 0x28);
  uVar15 = *(ulonglong *)(lVar5 + 0x30) - uVar14;
  if (*(ulonglong *)(lVar5 + 0x30) < uVar14) {
    lVar16 = 8;
    uVar15 = uVar11;
  }
  *puVar4 = (*(longlong *)(lVar5 + 0x40) - uVar15) - uVar6;
  lVar12 = uVar6 * 0x10 + lVar17;
  lVar13 = uVar15 * 0x10 + lVar16;
  uVar2 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar5 = *in_R8;
  puVar7 = (undefined4 *)in_R8[2];
  uVar3 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  lVar8 = *in_RDX;
  puVar9 = (undefined4 *)in_RDX[2];
  do {
    if ((lVar17 == 0) || (lVar17 == lVar12)) {
      if ((lVar16 == 0) || (lVar16 == lVar13)) {
        return;
      }
      *puVar4 = *puVar4 + 1;
      puVar1 = (undefined1 *)(lVar16 + 8);
      lVar16 = lVar16 + 0x10;
      lVar18 = 0;
      lVar17 = 0;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
      switch(*puVar1) {
      case 0:
        goto code_r0x00014227c36f;
      case 1:
        goto code_r0x00014227c433;
      case 2:
        goto code_r0x00014227c46b;
      case 3:
        goto code_r0x00014227c3df;
      case 4:
        goto code_r0x00014227c4bf;
      case 5:
        goto code_r0x00014227c4f4;
      case 6:
        goto code_r0x00014227c487;
      case 7:
        goto code_r0x00014227c53f;
      case 8:
        goto code_r0x00014227c417;
      case 9:
        goto code_r0x00014227c526;
      case 10:
        goto code_r0x00014227c3c3;
      case 0xb:
        goto code_r0x00014227c3fb;
      case 0xc:
        goto code_r0x00014227c4db;
      case 0xd:
        goto code_r0x00014227c3a7;
      case 0xe:
        goto code_r0x00014227c44f;
      case 0xf:
        goto code_r0x00014227c38b;
      case 0x10:
        goto code_r0x00014227c4a3;
      case 0x11:
        goto code_r0x00014227c50d;
      }
    }
    else {
      *puVar4 = *puVar4 + 1;
      puVar1 = (undefined1 *)(lVar17 + 8);
      lVar18 = lVar17 + 0x10;
      lVar17 = lVar18;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
      switch(*puVar1) {
      case 0:
code_r0x00014227c36f:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 500) = 1;
        lVar17 = lVar18;
        break;
      case 1:
code_r0x00014227c433:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1f5) = 1;
        lVar17 = lVar18;
        break;
      case 2:
code_r0x00014227c46b:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1fe) = 1;
        lVar17 = lVar18;
        break;
      case 3:
code_r0x00014227c3df:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1fc) = 1;
        lVar17 = lVar18;
        break;
      case 4:
code_r0x00014227c4bf:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1fa) = 1;
        lVar17 = lVar18;
        break;
      case 5:
code_r0x00014227c4f4:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1fb) = 1;
        lVar17 = lVar18;
        break;
      case 6:
code_r0x00014227c487:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1f9) = 1;
        lVar17 = lVar18;
        break;
      case 7:
code_r0x00014227c53f:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1ff) = 1;
        lVar17 = lVar18;
        break;
      case 8:
code_r0x00014227c417:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1fd) = 1;
        lVar17 = lVar18;
        break;
      case 9:
code_r0x00014227c526:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x200) = 1;
        lVar17 = lVar18;
        break;
      case 10:
code_r0x00014227c3c3:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1f6) = 1;
        lVar17 = lVar18;
        break;
      case 0xb:
code_r0x00014227c3fb:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1f7) = 1;
        lVar17 = lVar18;
        break;
      case 0xc:
code_r0x00014227c4db:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x1f8) = 1;
        lVar17 = lVar18;
        break;
      case 0xd:
code_r0x00014227c3a7:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x201) = 1;
        lVar17 = lVar18;
        break;
      case 0xe:
code_r0x00014227c44f:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x202) = 1;
        lVar17 = lVar18;
        break;
      case 0xf:
code_r0x00014227c38b:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x203) = 1;
        lVar17 = lVar18;
        break;
      case 0x10:
code_r0x00014227c4a3:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x204) = 1;
        lVar17 = lVar18;
        break;
      case 0x11:
code_r0x00014227c50d:
        *puVar7 = uVar2;
        *(undefined1 *)(lVar5 + 0x205) = 1;
        lVar17 = lVar18;
      }
    }
    *puVar9 = uVar3;
    lVar18 = *(longlong *)(lVar8 + 0x30);
    uVar10 = *(undefined8 *)(lVar8 + 0x40);
    if (lVar18 == *(longlong *)(lVar8 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar8 + 0x28) + lVar18 * 8) = uVar10;
    *(longlong *)(lVar8 + 0x30) = lVar18 + 1;
    *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
  } while( true );
}

