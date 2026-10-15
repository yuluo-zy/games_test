   1	
   2	void FUN_140145860(longlong *param_1,undefined8 param_2)
   3	
   4	{
   5	  undefined8 *puVar1;
   6	  undefined8 uVar2;
   7	  code *pcVar3;
   8	  undefined4 *puVar4;
   9	  longlong *plVar5;
  10	  undefined8 auStack_f8 [2];
  11	  longlong *plStack_e8;
  12	  undefined8 uStack_e0;
  13	  undefined8 uStack_c8;
  14	  undefined4 *puStack_c0;
  15	  undefined8 uStack_b8;
  16	  undefined4 uStack_b0;
  17	  undefined4 uStack_a8;
  18	  undefined8 uStack_a0;
  19	  undefined1 uStack_98;
  20	  undefined4 uStack_93;
  21	  undefined3 uStack_8f;
  22	  undefined2 uStack_88;
  23	  undefined1 uStack_86;
  24	  undefined1 uStack_83;
  25	  undefined4 auStack_80 [4];
  26	  longlong *aplStack_70 [10];
  27	  
  28	  puVar4 = (undefined4 *)func_0x000140613c10(0x2b,1);
  29	  if (puVar4 == (undefined4 *)0x0) {
  30	    FUN_1428d8fe3(1,0x2b,&UNK_142951128);
  31	  }
  32	  else {
  33	    *(undefined8 *)((longlong)puVar4 + 0x1b) = 0x65646f6d20646c69;
  34	    *(undefined8 *)((longlong)puVar4 + 0x23) = 0x2e6172656d616320;
  35	    *(undefined8 *)(puVar4 + 4) = 0x6874697720736b72;
  36	    *(undefined8 *)(puVar4 + 6) = 0x6d20646c69756220;
  37	    *puVar4 = 0x65746f4e;
  38	    puVar4[1] = 0x7469203a;
  39	    puVar4[2] = 0x6c6e6f20;
  40	    puVar4[3] = 0x6f772079;
  41	    uStack_98 = 0;
  42	    uStack_93 = 0;
  43	    uStack_8f = 0;
  44	    auStack_f8[0] = 3;
  45	    uStack_e0 = 6;
  46	    uStack_c8 = 0x2b;
  47	    uStack_b8 = 0x2b;
  48	    uStack_b0 = 0;
  49	    uStack_a8 = 0;
  50	    uStack_a0 = 0;
  51	    uStack_88 = 0x200;
  52	    uStack_86 = 2;
  53	    uStack_83 = 2;
  54	    puStack_c0 = puVar4;
  55	    FUN_140c28d40(auStack_80,auStack_f8,param_2);
  56	    LOCK();
  57	    *aplStack_70[0] = *aplStack_70[0] + -1;
  58	    UNLOCK();
  59	    if (*aplStack_70[0] == 0) {
  60	      FUN_140c41170(aplStack_70);
  61	    }
  62	    puVar1 = (undefined8 *)*param_1;
  63	    auStack_80[0] = func_0x0001409114d0(*puVar1);
  64	    plVar5 = (longlong *)func_0x000140613c10(0x10,8);
  65	    if (plVar5 != (longlong *)0x0) {
  66	      *plVar5 = (longlong)auStack_80;
  67	      plVar5[1] = (longlong)puVar1;
  68	      FUN_1400c4230(auStack_f8,param_2,0,plVar5,&UNK_142950f98);
  69	      LOCK();
  70	      *plStack_e8 = *plStack_e8 + -1;
  71	      UNLOCK();
  72	      if (*plStack_e8 == 0) {
  73	        FUN_140c41170(&plStack_e8);
  74	      }
  75	      puVar1 = (undefined8 *)param_1[1];
  76	      *(undefined4 *)puVar1[2] = *(undefined4 *)((longlong)puVar1 + 0x1c);
  77	      uVar2 = *puVar1;
  78	      puVar1 = (undefined8 *)param_1[2];
  79	      *(undefined4 *)puVar1[2] = *(undefined4 *)((longlong)puVar1 + 0x1c);
  80	      FUN_140148cb0(param_2,uVar2,*puVar1);
  81	      return;
  82	    }
  83	  }
  84	  FUN_1428d9000(8,0x10);
  85	  pcVar3 = (code *)swi(3);
  86	  (*pcVar3)();
  87	  return;
  88	}
  89	
