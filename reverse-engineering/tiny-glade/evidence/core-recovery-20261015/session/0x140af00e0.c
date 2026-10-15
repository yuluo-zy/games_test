   1	
   2	undefined1 * FUN_140af00e0(undefined1 *param_1,longlong param_2)
   3	
   4	{
   5	  longlong lVar1;
   6	  undefined4 uStack_88;
   7	  undefined4 uStack_84;
   8	  undefined4 uStack_80;
   9	  undefined4 uStack_7c;
  10	  undefined4 uStack_78;
  11	  undefined4 uStack_74;
  12	  undefined4 uStack_70;
  13	  undefined4 uStack_6c;
  14	  undefined4 uStack_68;
  15	  undefined4 uStack_64;
  16	  undefined4 uStack_60;
  17	  undefined4 uStack_5c;
  18	  undefined8 uStack_58;
  19	  undefined4 uStack_50;
  20	  undefined4 uStack_4c;
  21	  undefined4 uStack_48;
  22	  undefined4 uStack_44;
  23	  undefined8 uStack_40;
  24	  undefined4 uStack_38;
  25	  undefined4 uStack_34;
  26	  undefined8 uStack_30;
  27	  undefined1 uStack_21;
  28	  undefined8 uStack_20;
  29	  
  30	  uStack_20 = 0xfffffffffffffffe;
  31	  uStack_58 = -0x8000000000000000;
  32	  uStack_40 = 0;
  33	  uStack_30 = 0;
  34	  uStack_21 = 1;
  35	  lVar1 = FUN_141599080(&uStack_58,&UNK_142ae3daf,0xc,param_2 + 0x28);
  36	  if (lVar1 == 0) {
  37	    uStack_21 = 1;
  38	    lVar1 = FUN_141595790(&uStack_58,&UNK_142ae3dbb,0x13,param_2);
  39	    if (lVar1 == 0) {
  40	      uStack_21 = 1;
  41	      lVar1 = FUN_141595a20(&uStack_58,&UNK_142ae3dce,0x10,param_2 + 0x18);
  42	      if (lVar1 == 0) {
  43	        uStack_21 = 1;
  44	        lVar1 = FUN_141595a20(&uStack_58,&UNK_142ae3dde,8,param_2 + 0x1c);
  45	        if (lVar1 == 0) {
  46	          uStack_21 = 1;
  47	          lVar1 = FUN_141595a20(&uStack_58,&UNK_142ae3de6,8,param_2 + 0x20);
  48	          if (lVar1 == 0) {
  49	            uStack_21 = 1;
  50	            lVar1 = FUN_141595a20(&uStack_58,&UNK_142ae3dee,10,param_2 + 0x24);
  51	            if (lVar1 == 0) {
  52	              uStack_68 = uStack_38;
  53	              uStack_64 = uStack_34;
  54	              uStack_60 = (undefined4)uStack_30;
  55	              uStack_5c = uStack_30._4_4_;
  56	              uStack_78 = uStack_48;
  57	              uStack_74 = uStack_44;
  58	              uStack_70 = (undefined4)uStack_40;
  59	              uStack_6c = uStack_40._4_4_;
  60	              uStack_88 = (undefined4)uStack_58;
  61	              uStack_84 = uStack_58._4_4_;
  62	              uStack_80 = uStack_50;
  63	              uStack_7c = uStack_4c;
  64	              uStack_21 = 0;
  65	              FUN_140b864c0(param_1,&uStack_88);
  66	              return param_1;
  67	            }
  68	          }
  69	        }
  70	      }
  71	    }
  72	  }
  73	  *(longlong *)(param_1 + 8) = lVar1;
  74	  *param_1 = 6;
  75	  FUN_140898f40(&uStack_40);
  76	  if ((!SBORROW8(0,uStack_58)) && (uStack_58 != 0)) {
  77	    func_0x000140613c20(CONCAT44(uStack_4c,uStack_50),uStack_58,1);
  78	  }
  79	  return param_1;
  80	}
  81	