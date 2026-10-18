/* place_tree @0x0x142269970 size 1455 */

void __cdecl system_tree::ui_place_tree::place_tree(void)

{
  longlong lVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  ulonglong *puVar7;
  longlong lVar8;
  undefined4 *puVar9;
  size_t sVar10;
  undefined4 *puVar11;
  longlong lVar12;
  undefined4 *puVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  longlong lVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  longlong extraout_RAX;
  longlong *extraout_RAX_00;
  void *_Src;
  void *extraout_RAX_01;
  void *_Dst;
  undefined8 *extraout_RAX_02;
  longlong in_RCX;
  undefined8 *in_RDX;
  size_t sVar21;
  size_t _Size;
  longlong lVar22;
  undefined4 uVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  longlong *in_R8;
  ulonglong uVar26;
  longlong in_R9;
  ulonglong uVar27;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar28;
  float fVar29;
  size_t *in_stack_00000028;
  undefined7 uStack_137;
  longlong local_130;
  undefined8 local_108;
  undefined4 local_f8;
  undefined4 uStack_f4;
  undefined7 uStack_e7;
  longlong local_c0;
  
  puVar7 = (ulonglong *)*in_RDX;
  lVar8 = in_RDX[1];
  uVar25 = *puVar7;
  uVar27 = 0;
  uVar26 = uVar25 - *(ulonglong *)(lVar8 + 0x18);
  if (uVar25 < *(ulonglong *)(lVar8 + 0x18)) {
    uVar26 = uVar27;
  }
  uVar24 = uVar25 - *(ulonglong *)(lVar8 + 0x38);
  if (uVar25 < *(ulonglong *)(lVar8 + 0x38)) {
    uVar24 = uVar27;
  }
  local_130 = uVar26 * 0x18 + *(longlong *)(lVar8 + 8);
  uVar25 = *(ulonglong *)(lVar8 + 0x10) - uVar26;
  if (*(ulonglong *)(lVar8 + 0x10) < uVar26) {
    local_130 = 8;
    uVar25 = uVar27;
  }
  local_c0 = uVar24 * 0x18 + *(longlong *)(lVar8 + 0x28);
  uVar26 = *(ulonglong *)(lVar8 + 0x30) - uVar24;
  if (*(ulonglong *)(lVar8 + 0x30) < uVar24) {
    local_c0 = 8;
    uVar26 = uVar27;
  }
  *puVar7 = (*(longlong *)(lVar8 + 0x40) - uVar26) - uVar25;
  lVar8 = local_130 + uVar25 * 0x18;
  lVar1 = local_c0 + uVar26 * 0x18;
  uVar3 = *(undefined4 *)(in_RCX + 0x1c);
  puVar9 = *(undefined4 **)(in_RCX + 0x10);
  sVar10 = *in_stack_00000028;
  uVar4 = *(undefined4 *)(in_R9 + 0x1c);
  puVar11 = *(undefined4 **)(in_R9 + 0x10);
  uVar5 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar12 = *in_R8;
  puVar13 = (undefined4 *)in_R8[2];
  if (local_130 == 0) goto LAB_142269b10;
LAB_142269af0:
  if (local_130 == lVar8) goto LAB_142269b10;
  lVar22 = local_130;
  local_130 = local_130 + 0x18;
  do {
    *puVar7 = *puVar7 + 1;
    *puVar9 = uVar3;
    country_core::resources::tree_storage::TreeStorage::next_index();
    fastrand::Rng::f32();
    auVar17._8_8_ = 0;
    auVar17._0_8_ = extraout_RAX + 0xa0761d6478bd642fU ^ 0xe7037ed1a0b428db;
    auVar19._8_8_ = 0;
    auVar19._0_8_ = extraout_RAX + 0xa0761d6478bd642fU;
    uVar25 = SUB168(auVar17 * auVar19,8) ^ SUB168(auVar17 * auVar19,0);
    if ((uint)((int)uVar25 * 10) < 6) {
      uVar26 = extraout_RAX + 0x40ec3ac8f17ac85e;
      do {
        auVar18._8_8_ = 0;
        auVar18._0_8_ = uVar26 ^ 0xe7037ed1a0b428db;
        auVar20._8_8_ = 0;
        auVar20._0_8_ = uVar26;
        uVar25 = SUB168(auVar18 * auVar20,8) ^ SUB168(auVar18 * auVar20,0);
        uVar26 = uVar26 + 0xa0761d6478bd642f;
      } while ((uint)((int)uVar25 * 10) < 6);
    }
    uVar6 = *(undefined4 *)(lVar22 + 0x10);
    uVar14 = *(undefined8 *)(lVar22 + 8);
    fastrand::Rng::f32();
    sVar21 = sVar10;
    random_tree_prefab();
    _Size = sVar21;
    asset::prefab::PrefabName::as_name_ref();
    if ((longlong)_Size < 0) {
LAB_142269f24:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (_Size == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_01;
      if (extraout_RAX_01 == (void *)0x0) goto LAB_142269f24;
    }
    fVar28 = extraout_XMM0_Da * 0.9 + (1.0 - extraout_XMM0_Da) * 0.5;
    fVar29 = extraout_XMM0_Da_00 * 360.0 + (1.0 - extraout_XMM0_Da_00) * 0.0;
    memcpy(_Dst,_Src,_Size);
    uVar23 = (undefined4)((uVar25 & 0xffffffff) * 10 >> 0x20);
    LOCK();
    *extraout_RAX_00 = *extraout_RAX_00 + -1;
    UNLOCK();
    if (*extraout_RAX_00 == 0) {
      alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
    }
    alloc::string::impl$6::clone();
    *puVar13 = uVar5;
    lVar22 = *(longlong *)(lVar12 + 0x30);
    uVar15 = *(undefined8 *)(lVar12 + 0x40);
    if (lVar22 == *(longlong *)(lVar12 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                ();
    }
    lVar16 = *(longlong *)(lVar12 + 0x28);
    *(longlong **)(lVar16 + lVar22 * 0x48) = extraout_RAX_00;
    *(size_t *)(lVar16 + 8 + lVar22 * 0x48) = sVar21;
    puVar2 = (undefined8 *)(lVar16 + 0x10 + lVar22 * 0x48);
    *puVar2 = local_108;
    puVar2[1] = uVar14;
    puVar2 = (undefined8 *)(lVar16 + 0x20 + lVar22 * 0x48);
    *puVar2 = CONCAT44(fVar28,uVar6);
    puVar2[1] = CONCAT44(uVar23,fVar29);
    *(ulonglong *)(lVar16 + 0x30 + lVar22 * 0x48) = CONCAT71(uStack_e7,1);
    *(longlong *)(lVar16 + 0x38 + lVar22 * 0x48) = extraout_RAX;
    *(undefined8 *)(lVar16 + 0x40 + lVar22 * 0x48) = uVar15;
    *(longlong *)(lVar12 + 0x30) = lVar22 + 1;
    *(longlong *)(lVar12 + 0x40) = *(longlong *)(lVar12 + 0x40) + 1;
    *puVar11 = uVar4;
    local_108 = CONCAT44(fVar28,uVar6);
    local_f8 = (undefined4)CONCAT71(uStack_137,1);
    uStack_f4 = (undefined4)((uint7)uStack_137 >> 0x18);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_02 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_02 = 1;
    extraout_RAX_02[1] = 1;
    *(undefined4 *)(extraout_RAX_02 + 2) = 0;
    extraout_RAX_02[3] = extraout_RAX;
    extraout_RAX_02[4] = _Size;
    extraout_RAX_02[5] = _Dst;
    extraout_RAX_02[10] = CONCAT44(uStack_f4,local_f8);
    extraout_RAX_02[8] = local_108;
    extraout_RAX_02[9] = CONCAT44(uVar23,fVar29);
    extraout_RAX_02[6] = _Size;
    extraout_RAX_02[7] = uVar14;
    country_core::resources::history::History::add();
    if (local_130 != 0) goto LAB_142269af0;
LAB_142269b10:
    if ((local_c0 == 0) || (local_c0 == lVar1)) {
      return;
    }
    local_130 = 0;
    lVar22 = local_c0;
    local_c0 = local_c0 + 0x18;
  } while( true );
}

