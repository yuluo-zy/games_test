   1	
   2	undefined1 * FUN_1415d2b10(undefined1 *param_1,longlong *param_2)
   3	
   4	{
   5	  longlong lVar1;
   6	  longlong *plVar2;
   7	  longlong lVar3;
   8	  bool bVar4;
   9	  bool bVar5;
  10	  undefined1 unaff_BL;
  11	  undefined1 uVar6;
  12	  undefined8 unaff_RDI;
  13	  undefined4 uStack_b8;
  14	  undefined4 uStack_b4;
  15	  undefined4 uStack_b0;
  16	  undefined4 uStack_ac;
  17	  undefined4 uStack_a8;
  18	  undefined4 uStack_a4;
  19	  undefined4 uStack_a0;
  20	  undefined4 uStack_9c;
  21	  undefined7 uStack_8f;
  22	  undefined1 uStack_88;
  23	  undefined7 uStack_87;
  24	  undefined1 uStack_80;
  25	  undefined7 uStack_7f;
  26	  undefined1 uStack_78;
  27	  undefined7 uStack_77;
  28	  undefined1 uStack_70;
  29	  undefined7 uStack_6f;
  30	  undefined4 uStack_68;
  31	  undefined4 uStack_64;
  32	  undefined4 uStack_60;
  33	  undefined4 uStack_5c;
  34	  undefined4 uStack_58;
  35	  undefined4 uStack_54;
  36	  undefined4 uStack_50;
  37	  undefined4 uStack_4c;
  38	  longlong *plStack_48;
  39	  longlong lStack_40;
  40	  undefined1 uStack_31;
  41	  undefined8 uStack_30;
  42	  
  43	  uStack_30 = 0xfffffffffffffffe;
  44	  uStack_68 = (undefined4)param_2[3];
  45	  uStack_64 = *(undefined4 *)((longlong)param_2 + 0x1c);
  46	  uStack_60 = (undefined4)param_2[4];
  47	  uStack_5c = *(undefined4 *)((longlong)param_2 + 0x24);
  48	  uStack_58 = (undefined4)param_2[5];
  49	  uStack_54 = *(undefined4 *)((longlong)param_2 + 0x2c);
  50	  uStack_50 = (undefined4)param_2[6];
  51	  uStack_4c = *(undefined4 *)((longlong)param_2 + 0x34);
  52	  lVar1 = *param_2;
  53	  plVar2 = (longlong *)param_2[1];
  54	  lVar3 = param_2[2];
  55	  if (lVar3 == 0xd) {
  56	    if (*(longlong *)((longlong)plVar2 + 5) == 0x343230326e656577 && *plVar2 == 0x6565776f6c6c6148)
  57	    {
  58	      unaff_BL = 0;
  59	      bVar5 = false;
  60	      bVar4 = false;
  61	      uVar6 = unaff_BL;
  62	      goto joined_r0x0001415d2bc3;
  63	    }
  64	    if (*(longlong *)((longlong)plVar2 + 5) != 0x353230326e656577 || *plVar2 != 0x6565776f6c6c6148)
  65	    goto code_r0x0001415d2c39;
  66	    uVar6 = 1;
  67	    unaff_BL = 1;
  68	  }
  69	  else {
  70	    if ((lVar3 != 8) || (*plVar2 != 0x3432303273616d58)) {
  71	code_r0x0001415d2c39:
  72	      plStack_48 = plVar2;
  73	      lStack_40 = lVar1;
  74	      unaff_RDI = FUN_1428f9e20(plVar2,lVar3,&UNK_142c3a840,3);
  75	      bVar5 = true;
  76	      bVar4 = true;
  77	      lVar1 = lStack_40;
  78	      plVar2 = plStack_48;
  79	      uVar6 = unaff_BL;
  80	      goto joined_r0x0001415d2bc3;
  81	    }
  82	    uVar6 = 2;
  83	    unaff_BL = 2;
  84	  }
  85	  bVar5 = false;
  86	  bVar4 = false;
  87	joined_r0x0001415d2bc3:
  88	  if (lVar1 != 0) {
  89	    func_0x000140613c20(plVar2,lVar1,1);
  90	    bVar4 = bVar5;
  91	    unaff_BL = uVar6;
  92	  }
  93	  uStack_a8 = uStack_58;
  94	  uStack_a4 = uStack_54;
  95	  uStack_a0 = uStack_50;
  96	  uStack_9c = uStack_4c;
  97	  uStack_b8 = uStack_68;
  98	  uStack_b4 = uStack_64;
  99	  uStack_b0 = uStack_60;
 100	  uStack_ac = uStack_5c;
 101	  if (bVar4) {
 102	    *(undefined8 *)(param_1 + 8) = unaff_RDI;
 103	    *param_1 = 3;
 104	    uStack_31 = 0;
 105	    FUN_14088e430(&uStack_b8);
 106	  }
 107	  else {
 108	    uStack_78 = (undefined1)uStack_58;
 109	    uStack_77 = (undefined7)(CONCAT44(uStack_54,uStack_58) >> 8);
 110	    uStack_70 = (undefined1)uStack_50;
 111	    uStack_6f = (undefined7)(CONCAT44(uStack_4c,uStack_50) >> 8);
 112	    uStack_88 = (undefined1)uStack_68;
 113	    uStack_87 = (undefined7)(CONCAT44(uStack_64,uStack_68) >> 8);
 114	    uStack_80 = (undefined1)uStack_60;
 115	    uStack_7f = (undefined7)(CONCAT44(uStack_5c,uStack_60) >> 8);
 116	    *param_1 = unaff_BL;
 117	    *(ulonglong *)(param_1 + 1) = CONCAT17(uStack_88,uStack_8f);
 118	    *(ulonglong *)(param_1 + 9) = CONCAT17(uStack_80,uStack_87);
 119	    *(ulonglong *)(param_1 + 0x11) = CONCAT17(uStack_78,uStack_7f);
 120	    *(ulonglong *)(param_1 + 0x19) = CONCAT17(uStack_70,uStack_77);
 121	    *(ulonglong *)(param_1 + 0x20) = CONCAT71(uStack_6f,uStack_70);
 122	  }
 123	  return param_1;
 124	}
 125	