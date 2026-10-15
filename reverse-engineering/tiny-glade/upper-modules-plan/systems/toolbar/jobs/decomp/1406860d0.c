// 0x1406860d0 on_clipboard_paste
// signature: void __cdecl on_clipboard_paste(void)

void __cdecl toolbar::mei::core::MeiContext::on_clipboard_paste(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong in_RCX;
  longlong lVar7;
  undefined8 *in_RDX;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined4 uStack_4c;
  undefined8 local_48;
  undefined8 uStack_40;
  
  uVar2 = in_RDX[2];
  uVar5 = *in_RDX;
  uVar6 = in_RDX[1];
  lVar3 = *(longlong *)(in_RCX + 0x28);
  if (lVar3 == *(longlong *)(in_RCX + 0x18)) {
    alloc::raw_vec::RawVec::
    grow_one<tuple_<rhapsody::raw::command_buffer::CommandBuffer,alloc::vec::Vec<rhapsody::rg::executor::PassPgoDuration,alloc::alloc::Global>_>,alloc::alloc::Global>
              ();
  }
  lVar4 = *(longlong *)(in_RCX + 0x20);
  lVar7 = lVar3 * 0x70;
  puVar1 = (undefined8 *)(lVar4 + 0x60 + lVar7);
  *puVar1 = local_48;
  puVar1[1] = uStack_40;
  puVar1 = (undefined8 *)(lVar4 + 0x50 + lVar7);
  *puVar1 = local_58;
  puVar1[1] = CONCAT44(uStack_4c,0xc5);
  puVar1 = (undefined8 *)(lVar4 + 0x40 + lVar7);
  *puVar1 = local_68;
  puVar1[1] = uStack_60;
  puVar1 = (undefined8 *)(lVar4 + 0x30 + lVar7);
  *puVar1 = local_78;
  puVar1[1] = uStack_70;
  puVar1 = (undefined8 *)(lVar4 + 0x20 + lVar7);
  *puVar1 = local_88;
  puVar1[1] = uStack_80;
  puVar1 = (undefined8 *)(lVar4 + 0x10 + lVar7);
  *puVar1 = uVar2;
  puVar1[1] = uStack_90;
  *(undefined8 *)(lVar4 + lVar7) = uVar5;
  ((undefined8 *)(lVar4 + lVar7))[1] = uVar6;
  *(longlong *)(in_RCX + 0x28) = lVar3 + 1;
  return;
}

