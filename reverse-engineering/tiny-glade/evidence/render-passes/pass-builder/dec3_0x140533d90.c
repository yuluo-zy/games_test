// address: 0x140533d90
// pdb name: render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>
// signature: void __cdecl render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env$0>(void)

void __cdecl
rhapsody::rg::pass_builder::PassBuilder::
render<country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::screenshot::handle_screenshot_opportunity::closure_env_0>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong *in_RCX;
  longlong lVar13;
  
  lVar2 = *in_RCX;
  if (SBORROW8(0,lVar2)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar3 = in_RCX[5];
  in_RCX[5] = 1;
  in_RCX[6] = (longlong)&anon_7bbbd406453a1ddc191dc88141fb67b8_118_llvm_18078733583021761503;
  if (lVar3 == 0) {
    lVar3 = in_RCX[8];
    *in_RCX = -0x8000000000000000;
    lVar4 = in_RCX[7];
    lVar9 = in_RCX[1];
    lVar10 = in_RCX[2];
    lVar11 = in_RCX[3];
    lVar12 = in_RCX[4];
    lVar5 = in_RCX[5];
    lVar6 = in_RCX[6];
    lVar7 = *(longlong *)(lVar3 + 0xa0);
    if (lVar7 == *(longlong *)(lVar3 + 0x90)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
    }
    lVar8 = *(longlong *)(lVar3 + 0x98);
    lVar13 = lVar7 * 0x40;
    plVar1 = (longlong *)(lVar8 + 0x30 + lVar13);
    *plVar1 = lVar6;
    plVar1[1] = lVar4;
    plVar1 = (longlong *)(lVar8 + 0x20 + lVar13);
    *plVar1 = lVar12;
    plVar1[1] = lVar5;
    plVar1 = (longlong *)(lVar8 + 0x10 + lVar13);
    *plVar1 = lVar10;
    plVar1[1] = lVar11;
    *(longlong *)(lVar8 + lVar13) = lVar2;
    ((longlong *)(lVar8 + lVar13))[1] = lVar9;
    *(longlong *)(lVar3 + 0xa0) = lVar7 + 1;
    return;
  }
                    // WARNING: Subroutine does not return
  core::panicking::panic();
}


