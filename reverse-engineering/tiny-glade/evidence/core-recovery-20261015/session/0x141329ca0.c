   1	
   2	undefined1 * FUN_141329ca0(undefined1 *param_1,longlong param_2)
   3	
   4	{
   5	  longlong lVar1;
   6	  longlong lVar2;
   7	  undefined4 uStack_98;
   8	  undefined4 uStack_94;
   9	  undefined4 uStack_90;
  10	  undefined4 uStack_8c;
  11	  undefined4 uStack_88;
  12	  undefined4 uStack_84;
  13	  undefined4 uStack_80;
  14	  undefined4 uStack_7c;
  15	  undefined4 uStack_78;
  16	  undefined4 uStack_74;
  17	  undefined4 uStack_70;
  18	  undefined4 uStack_6c;
  19	  undefined8 uStack_60;
  20	  undefined4 uStack_58;
  21	  undefined4 uStack_54;
  22	  undefined4 uStack_50;
  23	  undefined4 uStack_4c;
  24	  undefined8 uStack_48;
  25	  undefined4 uStack_40;
  26	  undefined4 uStack_3c;
  27	  undefined8 uStack_38;
  28	  undefined1 uStack_29;
  29	  undefined8 uStack_28;
  30	  
  31	  uStack_28 = 0xfffffffffffffffe;
  32	  lVar1 = *(longlong *)(param_2 + 0x18);
  33	  uStack_60 = -0x8000000000000000;
  34	  uStack_48 = 0;
  35	  uStack_38 = 0;
  36	  uStack_29 = 1;
  37	  lVar2 = FUN_1415976f0(&uStack_60,&UNK_142c3a87b,5,param_2 + 0x58);
  38	  if (lVar2 == 0) {
  39	    uStack_29 = 1;
  40	    lVar2 = FUN_1415984f0(&uStack_60,&UNK_142c3a880,8,param_2);
  41	    if (lVar2 == 0) {
  42	      if (!SBORROW8(0,lVar1)) {
  43	        uStack_29 = 1;
  44	        lVar2 = FUN_141598750(&uStack_60,&UNK_142c3a888,5,param_2 + 0x18);
  45	        if (lVar2 != 0) goto code_r0x000141329d28;
  46	      }
  47	      uStack_78 = uStack_40;
  48	      uStack_74 = uStack_3c;
  49	      uStack_70 = (undefined4)uStack_38;
  50	      uStack_6c = uStack_38._4_4_;
  51	      uStack_88 = uStack_50;
  52	      uStack_84 = uStack_4c;
  53	      uStack_80 = (undefined4)uStack_48;
  54	      uStack_7c = uStack_48._4_4_;
  55	      uStack_98 = (undefined4)uStack_60;
  56	      uStack_94 = uStack_60._4_4_;
  57	      uStack_90 = uStack_58;
  58	      uStack_8c = uStack_54;
  59	      uStack_29 = 0;
  60	      FUN_140b864c0(param_1,&uStack_98);
  61	      return param_1;
  62	    }
  63	  }
  64	code_r0x000141329d28:
  65	  *(longlong *)(param_1 + 8) = lVar2;
  66	  *param_1 = 6;
  67	  FUN_140898f40(&uStack_48);
  68	  if ((!SBORROW8(0,uStack_60)) && (uStack_60 != 0)) {
  69	    func_0x000140613c20(CONCAT44(uStack_54,uStack_58),uStack_60,1);
  70	  }
  71	  return param_1;
  72	}
  73	