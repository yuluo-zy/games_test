
void __cdecl country_core::systems::collision::world::RaycastWorld::flush_colliders(void)

{
  byte *pbVar1;
  longlong lVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char extraout_AL;
  uint uVar6;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong in_RCX;
  longlong *plVar7;
  ulonglong uVar8;
  char cVar9;
  undefined *puVar10;
  void *_Src;
  longlong lVar11;
  undefined8 local_298;
  char *local_290;
  undefined8 local_288;
  undefined8 local_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  char *local_260;
  undefined8 local_258;
  undefined4 local_250;
  undefined4 local_24c;
  undefined **local_248;
  undefined8 local_240;
  char **local_238;
  undefined8 local_230;
  undefined8 local_228;
  char *local_98;
  code *local_90;
  longlong local_78;
  longlong local_70;
  longlong *local_58;
  char local_49 [9];
  
  local_49[1] = -2;
  local_49[2] = -1;
  local_49[3] = -1;
  local_49[4] = -1;
  local_49[5] = -1;
  local_49[6] = -1;
  local_49[7] = -1;
  local_49[8] = -1;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_70 = 0;
  }
  else {
    uVar8 = 0x46;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar8 != 0) {
        if (uVar8 < 0x46) {
          if ((char)(&DAT_142aa3695)[uVar8] < -0x40) {
LAB_1408cd631:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar8 != 0x46) goto LAB_1408cd631;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar8 != 0xfffffffffffffffe)) {
        if (uVar8 + 2 < 0x46) {
          if ((char)(&UNK_142aa3697)[uVar8] < -0x40) {
LAB_1408cd68f:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar8 + 2 != 0x46) goto LAB_1408cd68f;
      }
    }
    puVar10 = &DAT_142aa38ea;
    do {
      bVar3 = puVar10[-1];
      uVar6 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar10[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar10[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar10 + -4;
            puVar10 = puVar10 + -4;
            uVar6 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar10 = puVar10 + -3;
            uVar6 = bVar5 & 0xf;
          }
          uVar6 = bVar4 & 0x3f | uVar6 << 6;
        }
        else {
          puVar10 = puVar10 + -2;
          uVar6 = bVar4 & 0x1f;
        }
        uVar6 = (uint)(bVar3 & 0x3f) | uVar6 << 6;
        if (uVar6 != 0x5c) goto LAB_1408cd754;
LAB_1408cd765:
        if (puVar10 == (undefined *)0x142aa38b7) break;
        if (puVar10 + -0x142aa38b7 < (undefined *)0x32) {
          if (-0x41 < (char)puVar10[1]) break;
        }
        else if (puVar10 + -0x142aa38b7 == (undefined *)0x32) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar10 = puVar10 + -1;
      if (uVar6 == 0x5c) goto LAB_1408cd765;
LAB_1408cd754:
      if (uVar6 == 0x2f) goto LAB_1408cd765;
    } while (puVar10 != &anon_b111d695fd3e0f72064e00b504fdf12a_34_llvm_7010674786476989468);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar7 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar7 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_1408cda38;
    if (*plVar7 != 0) goto LAB_1408cda44;
    *plVar7 = -1;
    local_58 = plVar7;
    puffin::ThreadProfiler::begin_scope();
    *local_58 = *local_58 + 1;
    local_70 = 1;
  }
  local_58 = (longlong *)(in_RCX + 0x20);
  local_78 = in_RCX + 0x50;
  lVar2 = in_RCX + 0x3a8;
  cVar9 = '\0';
  lVar11 = 0;
  do {
    if (*(char *)(lVar2 + lVar11) != '\0') {
      _Src = (void *)(lVar2 + lVar11 + -0x200);
      memcpy(&local_298,_Src,0x200);
      memset(_Src,0,0x200);
      tabloid::Publisher::
      publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>
                ();
      *(undefined1 *)(lVar2 + lVar11) = 0;
      if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        local_98 = local_49;
        local_90 = impl$99::fmt;
        local_49[0] = cVar9;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        local_278 = *extraout_RAX_02;
        uStack_270 = extraout_RAX_02[1];
        local_24c = *(undefined4 *)(extraout_RAX_02 + 2);
        local_268 = 5;
        local_260 = "country_core::systems::collision::world";
        local_258 = 0x27;
        local_248 = &PTR_DAT_142aa3900;
        local_240 = 1;
        local_238 = &local_98;
        local_230 = 1;
        local_228 = 0;
        local_298 = 0;
        local_290 = "country_core::systems::collision::world";
        local_288 = 0x27;
        local_280 = 0;
        local_250 = 1;
        ::log::__private_api::impl$2::log();
      }
      rapier3d::pipeline::query_pipeline::QueryPipeline::update();
    }
    lVar11 = lVar11 + 0x390;
    cVar9 = cVar9 + '\x01';
  } while (lVar11 != 0x3c90);
  if (local_70 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_03 == 1) {
      plVar7 = extraout_RAX_03 + 1;
    }
    else if ((*extraout_RAX_03 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar7 = extraout_RAX_04, extraout_RAX_04 == (longlong *)0x0)) {
LAB_1408cda38:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar7 != 0) {
LAB_1408cda44:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar7 = -1;
    local_58 = plVar7;
    puffin::ThreadProfiler::end_scope();
    *local_58 = *local_58 + 1;
  }
  return;
}

