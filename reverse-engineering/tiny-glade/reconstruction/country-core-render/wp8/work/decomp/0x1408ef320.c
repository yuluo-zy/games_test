
void __cdecl country_core::resources::mouse_terrain_raycast::mouse_terrain_raycast(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  int *piVar5;
  int *piVar6;
  undefined8 *puVar7;
  char extraout_AL;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar8;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  uint uVar9;
  longlong *plVar10;
  undefined8 *in_RDX;
  ulonglong uVar11;
  undefined8 *in_R9;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  uint uVar18;
  float fVar20;
  undefined1 auVar19 [16];
  float fVar21;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  uint local_7c;
  float fStack_74;
  float local_70;
  float local_6c;
  longlong *local_68;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar11 = 0x45;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar11 != 0) {
        if (uVar11 < 0x45) {
          if ((char)(&DAT_142aa6ac0)[uVar11] < -0x40) {
LAB_1408ef3a2:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 != 0x45) goto LAB_1408ef3a2;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar11 != 0xfffffffffffffffe)) {
        if (uVar11 + 2 < 0x45) {
          if ((char)(&UNK_142aa6ac2)[uVar11] < -0x40) {
LAB_1408ef400:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar11 + 2 != 0x45) goto LAB_1408ef400;
      }
    }
    puVar8 = &DAT_142aa7492;
    do {
      bVar2 = puVar8[-1];
      uVar9 = (uint)(char)bVar2;
      if ((char)bVar2 < '\0') {
        bVar3 = puVar8[-2];
        if ((char)bVar3 < -0x40) {
          bVar4 = puVar8[-3];
          if ((char)bVar4 < -0x40) {
            pbVar1 = puVar8 + -4;
            puVar8 = puVar8 + -4;
            uVar9 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar8 = puVar8 + -3;
            uVar9 = bVar4 & 0xf;
          }
          uVar9 = bVar3 & 0x3f | uVar9 << 6;
        }
        else {
          puVar8 = puVar8 + -2;
          uVar9 = bVar3 & 0x1f;
        }
        uVar9 = (uint)(bVar2 & 0x3f) | uVar9 << 6;
        if (uVar9 != 0x5c) goto LAB_1408ef4c9;
LAB_1408ef4ea:
        if (puVar8 == &UNK_142aa7457) break;
        if (puVar8 + -0x142aa7457 < (undefined *)0x3a) {
          if (-0x41 < (char)puVar8[1]) break;
        }
        else if (puVar8 + -0x142aa7457 == (undefined *)0x3a) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar8 = puVar8 + -1;
      if (uVar9 == 0x5c) goto LAB_1408ef4ea;
LAB_1408ef4c9:
      if (uVar9 == 0x2f) goto LAB_1408ef4ea;
    } while (puVar8 != &DAT_142aa7458);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar10 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar10 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0))
    goto LAB_1408ef933;
    if (*plVar10 != 0) goto LAB_1408ef93f;
    *plVar10 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar10 = *plVar10 + 1;
  }
  if (*(int *)*in_stack_00000028 == 0) {
    piVar5 = (int *)*in_R9;
    if (*piVar5 == 1) {
      *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
      piVar6 = (int *)*in_RDX;
      puVar7 = (undefined8 *)*in_stack_00000030;
      fVar14 = (float)((ulonglong)*puVar7 >> 0x20);
      fVar15 = (float)((ulonglong)*(undefined8 *)((longlong)puVar7 + 0xc) >> 0x20);
      fVar12 = ((float)piVar5[1] - fVar14) / fVar15;
      fVar13 = fVar12 * (float)*(undefined8 *)((longlong)puVar7 + 0xc) + (float)*puVar7;
      fVar14 = fVar12 * fVar15 + fVar14;
      fVar12 = fVar12 * *(float *)((longlong)puVar7 + 0x14) + *(float *)(puVar7 + 1);
      if (*piVar6 == 1) {
        fVar15 = *(float *)(*in_stack_00000038 + 0x58);
        fVar17 = fVar13 - (float)*(undefined8 *)(piVar6 + 1);
        fVar20 = fVar14 - (float)((ulonglong)*(undefined8 *)(piVar6 + 1) >> 0x20);
        fVar16 = fVar12 - (float)piVar6[3];
        *(ulonglong *)(piVar6 + 4) = CONCAT44(fVar20,fVar17);
        piVar6[6] = (int)fVar16;
        if (fVar15 != 0.0) {
          piVar6[7] = (int)(SQRT(fVar16 * fVar16 + fVar20 * fVar20 + fVar17 * fVar17) / fVar15);
        }
      }
      *piVar6 = 1;
      *(ulonglong *)(piVar6 + 1) = CONCAT44(fVar14,fVar13);
      piVar6[3] = (int)fVar12;
      mouse_terrain_raycast::closure_0();
      if (extraout_AL == '\0') {
        return;
      }
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_03 == 1) {
        local_68 = extraout_RAX_03 + 1;
      }
      else if ((*extraout_RAX_03 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), local_68 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
      goto LAB_1408ef933;
      if (*local_68 != 0) goto LAB_1408ef93f;
      *local_68 = -1;
      puffin::ThreadProfiler::end_scope();
      goto LAB_1408ef60e;
    }
    terrain_height::TerrainHeightsData::
    intersect_ray<country_core::resources::terrain_height::TerrainHeightSsbo>();
    if ((local_7c & 1) != 0) {
      puVar7 = (undefined8 *)*in_stack_00000030;
      fVar12 = *(float *)(puVar7 + 1);
      fVar16 = (float)*puVar7;
      fVar17 = (float)((ulonglong)*puVar7 >> 0x20);
      fVar15 = (float)((ulonglong)*(undefined8 *)((longlong)puVar7 + 0xc) >> 0x20);
      fVar13 = (0.0 - fVar17) / fVar15;
      fVar14 = fVar13 * (float)*(undefined8 *)((longlong)puVar7 + 0xc) + fVar16;
      fVar15 = fVar13 * fVar15 + fVar17;
      fVar13 = fVar13 * *(float *)((longlong)puVar7 + 0x14) + fVar12;
      fVar20 = fVar16 - fVar14;
      fVar21 = fVar17 - fVar15;
      fVar17 = (fVar16 - fStack_74) * (fVar16 - fStack_74) +
               (fVar17 - local_70) * (fVar17 - local_70);
      fVar16 = fVar12 - fVar13;
      auVar19._0_4_ = fVar16 * fVar16 + fVar20 * fVar20 + fVar21 * fVar21;
      auVar19._4_4_ = (fVar12 - local_6c) * (fVar12 - local_6c) + fVar17;
      auVar19._8_4_ = fVar17 + 0.0;
      auVar19._12_4_ = 0;
      auVar19 = sqrtps(auVar19,auVar19);
      uVar9 = (uint)(auVar19._0_4_ < auVar19._4_4_);
      uVar18 = (int)(uVar9 << 0x1f) >> 0x1f;
      uVar9 = (int)(uVar9 << 0x1f) >> 0x1f;
      if (auVar19._4_4_ <= auVar19._0_4_) {
        fVar13 = local_6c;
      }
      uVar11 = CONCAT44((uint)fVar15 & uVar9,(uint)fVar14 & uVar18) |
               CONCAT44(~uVar9 & (uint)local_70,~uVar18 & (uint)fStack_74);
      *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
      piVar5 = (int *)*in_RDX;
      if (*piVar5 == 1) {
        fVar12 = *(float *)(*in_stack_00000038 + 0x58);
        fVar15 = (float)uVar11 - (float)*(undefined8 *)(piVar5 + 1);
        fVar16 = (float)(uVar11 >> 0x20) - (float)((ulonglong)*(undefined8 *)(piVar5 + 1) >> 0x20);
        fVar14 = fVar13 - (float)piVar5[3];
        *(ulonglong *)(piVar5 + 4) = CONCAT44(fVar16,fVar15);
        piVar5[6] = (int)fVar14;
        if (fVar12 != 0.0) {
          piVar5[7] = (int)(SQRT(fVar14 * fVar14 + fVar16 * fVar16 + fVar15 * fVar15) / fVar12);
        }
      }
      *piVar5 = 1;
      *(ulonglong *)(piVar5 + 1) = uVar11;
      piVar5[3] = (int)fVar13;
    }
    mouse_terrain_raycast::closure_0();
  }
  if (extraout_AL == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_02 == 1) {
    local_68 = extraout_RAX_02 + 1;
  }
  else if ((*extraout_RAX_02 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), local_68 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1408ef933:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*local_68 != 0) {
LAB_1408ef93f:
                    // WARNING: Subroutine does not return
    core::cell::panic_already_borrowed();
  }
  *local_68 = -1;
  puffin::ThreadProfiler::end_scope();
LAB_1408ef60e:
  *local_68 = *local_68 + 1;
  return;
}


