   1	
   2	undefined1 * FUN_1415a0330(undefined1 *param_1,byte *param_2)
   3	
   4	{
   5	  byte bVar1;
   6	  bool bVar2;
   7	  bool bVar3;
   8	  undefined8 uVar4;
   9	  longlong lVar5;
  10	  undefined8 uVar6;
  11	  undefined4 uStack_78;
  12	  undefined4 uStack_74;
  13	  undefined4 uStack_70;
  14	  undefined4 uStack_6c;
  15	  undefined8 uStack_68;
  16	  undefined4 uStack_58;
  17	  undefined4 uStack_54;
  18	  undefined4 uStack_50;
  19	  undefined4 uStack_4c;
  20	  undefined8 uStack_48;
  21	  undefined8 uStack_38;
  22	  undefined1 uStack_29;
  23	  byte *pbStack_28;
  24	  undefined1 uStack_1a;
  25	  undefined1 uStack_19;
  26	  undefined8 uStack_18;
  27	  
  28	  uStack_18 = 0xfffffffffffffffe;
  29	  pbStack_28 = param_2;
  30	  if (*param_2 == 4) {
  31	    uStack_48 = *(undefined8 *)(param_2 + 0x18);
  32	    uStack_58 = *(undefined4 *)(param_2 + 8);
  33	    uStack_54 = *(undefined4 *)(param_2 + 0xc);
  34	    uStack_50 = *(undefined4 *)(param_2 + 0x10);
  35	    uStack_4c = *(undefined4 *)(param_2 + 0x14);
  36	    uStack_19 = 1;
  37	    uStack_1a = 0;
  38	    FUN_1415a7e30(param_1,&uStack_58);
  39	    bVar3 = false;
  40	    bVar1 = *pbStack_28;
  41	  }
  42	  else {
  43	    if (*param_2 == 5) {
  44	      uStack_68 = *(undefined8 *)(param_2 + 0x18);
  45	      uStack_78 = *(undefined4 *)(param_2 + 8);
  46	      uStack_74 = *(undefined4 *)(param_2 + 0xc);
  47	      uStack_70 = *(undefined4 *)(param_2 + 0x10);
  48	      uStack_6c = *(undefined4 *)(param_2 + 0x14);
  49	      uStack_19 = 0;
  50	      uStack_1a = 1;
  51	      FUN_1415bcdd0(param_1,&uStack_78);
  52	      bVar3 = true;
  53	      bVar2 = false;
  54	      bVar1 = *pbStack_28;
  55	      goto joined_r0x0001415a03eb;
  56	    }
  57	    uStack_19 = 1;
  58	    uStack_1a = 1;
  59	    uVar4 = FUN_1428f9a60(param_2,&uStack_29,&UNK_142c79258);
  60	    *(undefined8 *)(param_1 + 8) = uVar4;
  61	    *param_1 = 1;
  62	    bVar3 = true;
  63	    bVar1 = *pbStack_28;
  64	  }
  65	  bVar2 = true;
  66	joined_r0x0001415a03eb:
  67	  if (bVar1 == 4) {
  68	    if (!bVar3) {
  69	      return param_1;
  70	    }
  71	    uStack_38 = *(undefined8 *)(pbStack_28 + 0x10);
  72	    FUN_140888d70(uStack_38,*(undefined8 *)(pbStack_28 + 0x18));
  73	    if (*(longlong *)(pbStack_28 + 8) == 0) {
  74	      return param_1;
  75	    }
  76	    lVar5 = *(longlong *)(pbStack_28 + 8) << 5;
  77	    uVar6 = 8;
  78	    uVar4 = uStack_38;
  79	  }
  80	  else {
  81	    if (bVar1 == 5) {
  82	      if (bVar2) {
  83	        FUN_140898f40(pbStack_28 + 8);
  84	        return param_1;
  85	      }
  86	      return param_1;
  87	    }
  88	    if (bVar1 < 3) {
  89	      return param_1;
  90	    }
  91	    lVar5 = *(longlong *)(pbStack_28 + 8);
  92	    if (lVar5 == 0) {
  93	      return param_1;
  94	    }
  95	    uVar6 = 1;
  96	    uVar4 = *(undefined8 *)(pbStack_28 + 0x10);
  97	  }
  98	  func_0x000140613c20(uVar4,lVar5,uVar6);
  99	  return param_1;
 100	}
 101	