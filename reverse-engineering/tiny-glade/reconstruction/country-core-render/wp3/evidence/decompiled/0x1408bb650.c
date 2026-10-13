// address 0x1408bb650
// name default_value
// signature void __cdecl default_value(void)

// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x0001408c0671) overlaps instruction at (ram,0x0001408c066e)
// 
// WARNING: Removing unreachable block (ram,0x0001408c091c)
// WARNING: Removing unreachable block (ram,0x0001408c08f0)

void __cdecl country_core::render::nani::buffers::impl$9::default_value(void)

{
  byte *pbVar1;
  byte bVar2;
  undefined4 uVar3;
  ulonglong uVar4;
  undefined4 *puVar5;
  byte bVar6;
  char cVar7;
  uint uVar8;
  longlong *plVar9;
  void *_Dst;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  byte *in_RCX;
  undefined4 *puVar10;
  uint uVar11;
  code *UNRECOVERED_JUMPTABLE_00;
  undefined7 uVar12;
  byte unaff_BL;
  longlong *unaff_RBP;
  undefined7 uVar14;
  undefined4 *puVar13;
  char *_Src;
  undefined4 *in_R9;
  undefined4 *in_R10;
  undefined4 *in_R11;
  longlong unaff_R12;
  undefined *unaff_R13;
  longlong *unaff_R15;
  uint local_1c0 [104];
  uint uStack_20;
  int iStack_1c;
  int iStack_18;
  int iStack_14;
  
  puVar13 = (undefined4 *)0x0;
  _Src = (char *)0x1b0;
  memset(local_1c0,0,0x1b0);
  plVar9 = (longlong *)&DAT_142aa1618;
  puVar10 = (undefined4 *)0x0;
LAB_1408bb6b8:
  cVar7 = (char)puVar13;
  UNRECOVERED_JUMPTABLE_00 =
       (code *)((longlong)&DAT_142aa1618 +
               (longlong)(int)(&DAT_142aa1618)[(ulonglong)puVar13 & 0xff]);
  uVar12 = (undefined7)((ulonglong)UNRECOVERED_JUMPTABLE_00 >> 8);
  uVar14 = (undefined7)((ulonglong)puVar13 >> 8);
  uVar8 = (uint)puVar10;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
  switch((ulonglong)puVar13 & 0xff) {
  case 0:
    uVar11 = (uint)CONCAT71(uVar12,1);
    goto LAB_1408bb680;
  case 1:
    uVar11 = (uint)CONCAT71(uVar12,2);
    goto LAB_1408bb680;
  case 2:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x1000;
    _Src = (char *)(ulonglong)(uVar8 + 0x2000);
    local_1c0[uVar4 + 2] = uVar8 + 0x2000;
    local_1c0[uVar4 + 3] = uVar8 + 0x3000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,3);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x4000);
    goto LAB_1408bb6b8;
  case 3:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x8000;
    _Src = (char *)(ulonglong)(uVar8 + 0x10000);
    local_1c0[uVar4 + 2] = uVar8 + 0x10000;
    local_1c0[uVar4 + 3] = uVar8 + 0x18000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,4);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x20000);
    goto LAB_1408bb6b8;
  case 4:
    uVar11 = (uint)CONCAT71(uVar12,5);
    goto LAB_1408bb680;
  case 5:
    uVar11 = (uint)CONCAT71(uVar12,6);
    goto LAB_1408bb680;
  case 6:
    uVar11 = (uint)CONCAT71(uVar12,7);
    goto LAB_1408bb680;
  case 7:
    uVar11 = (uint)CONCAT71(uVar12,8);
    goto LAB_1408bb680;
  case 8:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x40000;
    _Src = (char *)(ulonglong)(uVar8 + 0x80000);
    local_1c0[uVar4 + 2] = uVar8 + 0x80000;
    local_1c0[uVar4 + 3] = uVar8 + 0xc0000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,9);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x100000);
    goto LAB_1408bb6b8;
  case 9:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0xa000;
    _Src = (char *)(ulonglong)(uVar8 + 0x14000);
    local_1c0[uVar4 + 2] = uVar8 + 0x14000;
    local_1c0[uVar4 + 3] = uVar8 + 0x1e000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,10);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x28000);
    goto LAB_1408bb6b8;
  case 10:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x400;
    _Src = (char *)(ulonglong)(uVar8 + 0x800);
    local_1c0[uVar4 + 2] = uVar8 + 0x800;
    local_1c0[uVar4 + 3] = uVar8 + 0xc00;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0xb);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x1000);
    goto LAB_1408bb6b8;
  case 0xb:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0xcf80;
    _Src = (char *)(ulonglong)(uVar8 + 0x19f00);
    local_1c0[uVar4 + 2] = uVar8 + 0x19f00;
    local_1c0[uVar4 + 3] = uVar8 + 0x26e80;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0xc);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x33e00);
    goto LAB_1408bb6b8;
  case 0xc:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x2580;
    _Src = (char *)(ulonglong)(uVar8 + 0x4b00);
    local_1c0[uVar4 + 2] = uVar8 + 0x4b00;
    local_1c0[uVar4 + 3] = uVar8 + 0x7080;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0xd);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x9600);
    goto LAB_1408bb6b8;
  case 0xd:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x20000;
    _Src = (char *)(ulonglong)(uVar8 + 0x40000);
    local_1c0[uVar4 + 2] = uVar8 + 0x40000;
    local_1c0[uVar4 + 3] = uVar8 + 0x60000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0xe);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x80000);
    goto LAB_1408bb6b8;
  case 0xe:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x200;
    _Src = (char *)(ulonglong)(uVar8 + 0x400);
    local_1c0[uVar4 + 2] = uVar8 + 0x400;
    local_1c0[uVar4 + 3] = uVar8 + 0x600;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0xf);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x800);
    goto LAB_1408bb6b8;
  case 0xf:
    uVar11 = (uint)CONCAT71(uVar12,0x10);
LAB_1408bb680:
    _Src = (char *)(ulonglong)(byte)(cVar7 << 2);
    local_1c0[(longlong)_Src] = uVar8;
    local_1c0[(longlong)(_Src + 1)] = uVar8 + 0x2000;
    in_R9 = (undefined4 *)(ulonglong)(uVar8 + 0x4000);
    local_1c0[(longlong)(_Src + 2)] = uVar8 + 0x4000;
    local_1c0[(longlong)(_Src + 3)] = uVar8 + 0x6000;
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x8000);
    puVar13 = (undefined4 *)(ulonglong)uVar11;
    goto LAB_1408bb6b8;
  case 0x10:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x1000;
    _Src = (char *)(ulonglong)(uVar8 + 0x2000);
    local_1c0[uVar4 + 2] = uVar8 + 0x2000;
    local_1c0[uVar4 + 3] = uVar8 + 0x3000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x11);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x4000);
    goto LAB_1408bb6b8;
  case 0x11:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x400;
    _Src = (char *)(ulonglong)(uVar8 + 0x800);
    local_1c0[uVar4 + 2] = uVar8 + 0x800;
    local_1c0[uVar4 + 3] = uVar8 + 0xc00;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x12);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x1000);
    goto LAB_1408bb6b8;
  case 0x12:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x8000;
    _Src = (char *)(ulonglong)(uVar8 + 0x10000);
    local_1c0[uVar4 + 2] = uVar8 + 0x10000;
    local_1c0[uVar4 + 3] = uVar8 + 0x18000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x13);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x20000);
    goto LAB_1408bb6b8;
  case 0x13:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x10000;
    _Src = (char *)(ulonglong)(uVar8 + 0x20000);
    local_1c0[uVar4 + 2] = uVar8 + 0x20000;
    local_1c0[uVar4 + 3] = uVar8 + 0x30000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x14);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x40000);
    goto LAB_1408bb6b8;
  case 0x14:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x800;
    _Src = (char *)(ulonglong)(uVar8 + 0x1000);
    local_1c0[uVar4 + 2] = uVar8 + 0x1000;
    local_1c0[uVar4 + 3] = uVar8 + 0x1800;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x15);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x2000);
    goto LAB_1408bb6b8;
  case 0x15:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x400;
    _Src = (char *)(ulonglong)(uVar8 + 0x800);
    local_1c0[uVar4 + 2] = uVar8 + 0x800;
    local_1c0[uVar4 + 3] = uVar8 + 0xc00;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x16);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x1000);
    goto LAB_1408bb6b8;
  case 0x16:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x800;
    _Src = (char *)(ulonglong)(uVar8 + 0x1000);
    local_1c0[uVar4 + 2] = uVar8 + 0x1000;
    local_1c0[uVar4 + 3] = uVar8 + 0x1800;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x17);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x2000);
    goto LAB_1408bb6b8;
  case 0x17:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x20000;
    _Src = (char *)(ulonglong)(uVar8 + 0x40000);
    local_1c0[uVar4 + 2] = uVar8 + 0x40000;
    local_1c0[uVar4 + 3] = uVar8 + 0x60000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x18);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x80000);
    goto LAB_1408bb6b8;
  case 0x18:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x14000;
    _Src = (char *)(ulonglong)(uVar8 + 0x28000);
    local_1c0[uVar4 + 2] = uVar8 + 0x28000;
    local_1c0[uVar4 + 3] = uVar8 + 0x3c000;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x19);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x50000);
    goto LAB_1408bb6b8;
  case 0x19:
    uVar4 = (ulonglong)(byte)(cVar7 << 2);
    local_1c0[uVar4] = uVar8;
    local_1c0[uVar4 + 1] = uVar8 + 0x100;
    _Src = (char *)(ulonglong)(uVar8 + 0x200);
    local_1c0[uVar4 + 2] = uVar8 + 0x200;
    local_1c0[uVar4 + 3] = uVar8 + 0x300;
    puVar13 = (undefined4 *)CONCAT71(uVar14,0x1a);
    puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x400);
    goto LAB_1408bb6b8;
  case 0x1a:
    goto code_r0x0001408bbb5d;
  case 0x1b:
    memcpy(in_RCX + 4,local_1c0,0x1b0);
    in_RCX[0] = 1;
    in_RCX[1] = 0;
    in_RCX[2] = 0;
    in_RCX[3] = 0;
    return;
  case 0x1c:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x1d:
    goto code_r0x0001408c0619;
  case 0x1e:
    *in_RCX = *in_RCX | unaff_BL;
    plVar9 = (longlong *)(ulonglong)(0x42aa1618U - *(int *)UNRECOVERED_JUMPTABLE_00);
    *(char *)plVar9 = (char)*plVar9 + (char)(0x42aa1618U - *(int *)UNRECOVERED_JUMPTABLE_00);
    in_R9 = (undefined4 *)&DAT_0000001b;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Bucketevent:/SFX/Cosmetics/Cartevent:/SFX/Cosmetics/Chairsevent:/SFX/Cosmetics/ClayPotevent:/SFX/Cosmetics/FlowerPotevent:/SFX/Cosmetics/Ladderevent:/SFX/Cosmetics/Wheelevent:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x1f:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001b;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Ladderevent:/SFX/Cosmetics/Wheelevent:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x20:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_00000020;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x21:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_00000020;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/BoxWaterCanevent:/SFX/Cosmetics/Bucketevent:/SFX/Cosmetics/Cartevent:/SFX/Cosmetics/Chairsevent:/SFX/Cosmetics/ClayPotevent:/SFX/Cosmetics/FlowerPotevent:/SFX/Cosmetics/Ladderevent:/SFX/Cosmetics/Wheelevent:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x22:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001f;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x23:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001c;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x24:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001d;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x25:
    *in_RCX = *in_RCX | unaff_BL;
    uVar3 = in((short)UNRECOVERED_JUMPTABLE_00);
    *puVar13 = uVar3;
    pbVar1 = (byte *)((longlong)puVar10 + -0x47);
    bVar2 = *pbVar1;
    *pbVar1 = *pbVar1 - 0x3b;
    plVar9 = (longlong *)(ulonglong)(0xfaaa16c5 - (0x3a < bVar2));
    *(uint *)(unaff_RBP + 0xf) = uVar8;
    _Src = 
    "event:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    puVar13 = puVar13 + 1;
    break;
  case 0x26:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_00000019;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Cartevent:/SFX/Cosmetics/Chairsevent:/SFX/Cosmetics/ClayPotevent:/SFX/Cosmetics/FlowerPotevent:/SFX/Cosmetics/Ladderevent:/SFX/Cosmetics/Wheelevent:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x27:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001d;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x28:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)0x2e;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x29:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001a;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x2a:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001c;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell";
    break;
  case 0x2b:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x2c:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001d;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = "event:/SFX/Cosmetics/DoorBell";
    break;
  case 0x2d:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001c;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/BoxSackevent:/SFX/Cosmetics/BoxWaterCanevent:/SFX/Cosmetics/Bucketevent:/SFX/Cosmetics/Cartevent:/SFX/Cosmetics/Chairsevent:/SFX/Cosmetics/ClayPotevent:/SFX/Cosmetics/FlowerPotevent:/SFX/Cosmetics/Ladderevent:/SFX/Cosmetics/Wheelevent:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x2e:
    if (0xfffffffebd55e9e7 < (ulonglong)(longlong)(int)(&DAT_142aa1618)[(ulonglong)puVar13 & 0xff])
    {
      in_R11 = (undefined4 *)&DAT_00000008;
      in_R9 = puVar10;
    }
    unaff_R12 = (longlong)UNRECOVERED_JUMPTABLE_00 * 0x18 + -0x21e5ed9021e5be7;
    puVar5 = (undefined4 *)(-0x21e5c65021e5e9e - (longlong)UNRECOVERED_JUMPTABLE_00);
    if ((code *)0xfde1a39afde1a162 < UNRECOVERED_JUMPTABLE_00) {
      unaff_R12 = 8;
      puVar5 = puVar10;
    }
    *(longlong *)in_RCX = (-0x21e5d20021e5f4f - (longlong)puVar5) - (longlong)in_R9;
    unaff_RBP[0xf] = (longlong)in_R11;
    puVar13 = in_R11 + (longlong)in_R9 * 6;
    *unaff_RBP = unaff_R12 + (longlong)puVar5 * 0x18;
    unaff_BL = 0x4c;
    unaff_R13 = &DAT_142aa1688;
    goto code_r0x0001408c0600;
  case 0x2f:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001f;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x30:
    if ((longlong)UNRECOVERED_JUMPTABLE_00 < 0) {
                    // WARNING (jumptable): Read-only address (ram,0x000142aa15a1) is written
                    // WARNING: Read-only address (ram,0x000142aa15a1) is written
      UNK_142aa15a1 = 0xff;
    }
    else {
      plVar9 = (longlong *)0x42aa1060;
    }
    break;
  case 0x31:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_00000024;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x32:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001c;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/ClayPotevent:/SFX/Cosmetics/FlowerPotevent:/SFX/Cosmetics/Ladderevent:/SFX/Cosmetics/Wheelevent:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x33:
    plVar9 = (longlong *)0x42aa0ef2;
    break;
  case 0x34:
    pbVar1 = (byte *)((longlong)puVar10 + -0x47);
    bVar2 = *pbVar1;
    *pbVar1 = *pbVar1 + 0x18;
    cVar7 = 'k' - (0xe7 < bVar2);
    plVar9 = (longlong *)CONCAT71(0x142aa16,cVar7);
    *(char *)plVar9 = (char)*plVar9 + cVar7;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Wheelevent:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x35:
    unaff_BL = unaff_BL | *in_RCX;
    bVar6 = *_Src + 0x18;
    pbVar1 = (byte *)((longlong)puVar10 + -0x47);
    bVar2 = *pbVar1;
    *pbVar1 = *pbVar1 + bVar6;
    plVar9 = (longlong *)
             (ulonglong)(((int)CONCAT71(0x142aa16,bVar6) + -0x48000000) - (uint)CARRY1(bVar2,bVar6))
    ;
    *(uint *)(unaff_RBP + 0xf) = uVar8;
    _Src = 
    "event:/SFX/Cosmetics/Firewoodevent:/SFX/Cosmetics/Pitchforkevent:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x36:
    *in_RCX = *in_RCX | unaff_BL;
    *(char *)((longlong)puVar10 + -0x47) = *(char *)((longlong)puVar10 + -0x47) + -0x3b;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x37:
    plVar9 = (longlong *)0x44c81ecd;
  case 0x3a:
    *(char *)((longlong)puVar10 + -0x47) = *(char *)((longlong)puVar10 + -0x47) + (char)plVar9;
    plVar9 = (longlong *)((ulonglong)plVar9 & 0xffffffffffffff00);
    *(char *)plVar9 = (char)*plVar9;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x38:
    goto code_r0x0001408c065b;
  case 0x39:
    *puVar13 = *(undefined4 *)in_RCX;
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
                    // WARNING (jumptable): Read-only address (ram,0x000142aa15a1) is written
                    // WARNING: Read-only address (ram,0x000142aa15a1) is written
    UNK_142aa15a1 = 0xff;
    plVar9 = (longlong *)0x42aa107a;
    in_RCX = in_RCX + 4;
    puVar13 = puVar13 + 1;
    break;
  case 0x3b:
    *in_RCX = *in_RCX | unaff_BL;
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    pbVar1 = (byte *)((longlong)puVar10 + -0x47);
    bVar2 = *pbVar1;
    *pbVar1 = *pbVar1 + 0x18;
    uVar8 = 0x44c8756b - (0xe7 < bVar2);
    plVar9 = (longlong *)(ulonglong)uVar8;
    *(char *)plVar9 = (char)*plVar9 + (char)uVar8;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Shovelevent:/SFX/Cosmetics/Shuttersevent:/SFX/Cosmetics/ClothlineAppearevent:/SFX/Cosmetics/ClothlineConnectionAppearevent:/SFX/Cosmetics/WaterPlantevent:/SFX/Cosmetics/Benchevent:/SFX/Cosmetics/Birdhouseevent:/SFX/Cosmetics/PotatoSackevent:/SFX/Cosmetics/RoadSignevent:/SFX/Cosmetics/Barrelevent:/SFX/Cosmetics/Haystackevent:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x3c:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001b;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = 
    "event:/SFX/Cosmetics/Basketevent:/SFX/Cosmetics/Pumpkinevent:/SFX/Cosmetics/PumpkinWithFaceevent:/SFX/Cosmetics/OldenLogevent:/SFX/Cosmetics/OldenStumpevent:/SFX/Cosmetics/OldenRockevent:/SFX/Cosmetics/WindowFlowerbedevent:/SFX/Cosmetics/Anvilevent:/SFX/Cosmetics/FabricStackevent:/SFX/Cosmetics/BreadStallevent:/SFX/Cosmetics/Veggiestallevent:/SFX/Cosmetics/BoxFishevent:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell"
    ;
    break;
  case 0x3d:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x3e:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1618) is written
                    // WARNING: Read-only address (ram,0x000142aa1618) is written
    DAT_142aa1618._0_1_ = 0xc5;
    in_R9 = (undefined4 *)&DAT_0000001f;
    unaff_RBP[0xf] = (longlong)puVar10;
    _Src = "event:/SFX/Cosmetics/RackBroomsevent:/SFX/Cosmetics/DoorBell";
    break;
  case 0x3f:
    goto code_r0x0001408c0687;
  case 0x40:
                    // WARNING (jumptable): Read-only address (ram,0x000142aa1604) is written
                    // WARNING: Read-only address (ram,0x000142aa1604) is written
    UNK_142aa1604 = 0x67;
    goto code_r0x0001408c0649;
  case 0x41:
    goto code_r0x0001408c065b;
  case 0x42:
code_r0x0001408c062f:
    unaff_RBP[0xf] = (longlong)in_R10;
                    // WARNING: Could not recover jumptable at 0x0001408c0636. Too many branches
                    // WARNING: Treating indirect jump as call
    (*UNRECOVERED_JUMPTABLE_00)(puVar10,UNRECOVERED_JUMPTABLE_00,unaff_BL);
    return;
  }
  unaff_RBP[-2] = (longlong)in_R11;
  *(int *)(unaff_RBP + -3) = (int)plVar9[1];
  unaff_RBP[-4] = *plVar9;
  if (*unaff_R15 == 0) {
    unaff_RBP[-1] = unaff_R12;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    memcpy(_Dst,_Src,(size_t)in_R9);
    unaff_RBP[2] = 0;
    unaff_RBP[4] = (longlong)in_R9;
    unaff_RBP[5] = (longlong)_Dst;
    unaff_RBP[6] = (longlong)in_R9;
    *(undefined1 *)(unaff_RBP + 7) = 2;
    unaff_R13 = &DAT_142aa1688;
    unaff_R12 = unaff_RBP[-1];
  }
  else {
    resources::audio_resources::AudioCoreInner::instantiate_audio();
    if (unaff_RBP[2] == 2) {
      unaff_RBP[8] = unaff_RBP[3];
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
  }
  unaff_RBP[0xc] = unaff_RBP[6];
  *(char *)(unaff_RBP + 0xd) = (char)unaff_RBP[7];
  *(undefined4 *)((longlong)unaff_RBP + 0x69) = *(undefined4 *)((longlong)unaff_RBP + 0x39);
  *(undefined2 *)((longlong)unaff_RBP + 0x6d) = *(undefined2 *)((longlong)unaff_RBP + 0x3d);
  *(undefined1 *)((longlong)unaff_RBP + 0x6f) = *(undefined1 *)((longlong)unaff_RBP + 0x3f);
  unaff_RBP[10] = unaff_RBP[4];
  unaff_RBP[0xb] = unaff_RBP[5];
  unaff_RBP[8] = unaff_RBP[2];
  unaff_RBP[9] = unaff_RBP[3];
  resources::audio_resources::impl$10::set_position();
  resources::audio_resources::impl$10::set_parameter();
  resources::audio_resources::AudioInstance::start_if_not_playing();
  resources::audio_resources::impl$13::drop();
  if (unaff_RBP[10] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
code_r0x0001408c0600:
  in_R11 = (undefined4 *)unaff_RBP[0xf];
  if ((in_R11 != (undefined4 *)0x0) && (in_R11 != puVar13)) {
    *(longlong *)in_RCX = *(longlong *)in_RCX + 1;
    in_R10 = in_R11 + 6;
code_r0x0001408c0619:
    puVar10 = in_R10;
    UNRECOVERED_JUMPTABLE_00 =
         (code *)(unaff_R13 + *(int *)(unaff_R13 + (ulonglong)*(byte *)(in_R11 + 5) * 4));
    in_R10 = puVar10;
    goto code_r0x0001408c062f;
  }
  if (unaff_R12 != 0) {
code_r0x0001408c0649:
    if (unaff_R12 != *unaff_RBP) {
      *(longlong *)in_RCX = *(longlong *)in_RCX + 1;
code_r0x0001408c065b:
      UNRECOVERED_JUMPTABLE_00 =
           (code *)(unaff_R13 + *(int *)(unaff_R13 + (ulonglong)*(byte *)(unaff_R12 + 0x14) * 4));
      unaff_RBP[0xf] = 0;
      puVar10 = (undefined4 *)0x0;
code_r0x0001408c0687:
                    // WARNING: Could not recover jumptable at 0x0001408c0687. Too many branches
                    // WARNING: Treating indirect jump as call
      (*UNRECOVERED_JUMPTABLE_00)(puVar10,UNRECOVERED_JUMPTABLE_00);
      return;
    }
  }
  if (*(char *)((longlong)unaff_RBP + 0x77) != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX == 1) {
      plVar9 = extraout_RAX + 1;
    }
    else if ((*extraout_RAX != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar9 = extraout_RAX_00, extraout_RAX_00 == (longlong *)0x0)) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar9 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar9 = -1;
    unaff_RBP[0xf] = (longlong)plVar9;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)unaff_RBP[0xf] = *(longlong *)unaff_RBP[0xf] + 1;
  }
  return;
code_r0x0001408bbb5d:
  iStack_1c = uVar8 + 0x8000;
  iStack_18 = uVar8 + 0x10000;
  iStack_14 = uVar8 + 0x18000;
  puVar13 = (undefined4 *)CONCAT71(uVar14,0x1b);
  puVar10 = (undefined4 *)(ulonglong)(uVar8 + 0x20000);
  uStack_20 = uVar8;
  goto LAB_1408bb6b8;
}

