   1	
   2	undefined1 * FUN_14085fc20(undefined1 *param_1,longlong param_2)
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
  35	  lVar1 = FUN_141599080(&uStack_58,&UNK_142a95996,0xd,param_2);
  36	  if (lVar1 == 0) {
  37	    uStack_21 = 1;
  38	    lVar1 = FUN_141599080(&uStack_58,&UNK_142a959a3,0xd,param_2 + 1);
  39	    if (lVar1 == 0) {
  40	      uStack_68 = uStack_38;
  41	      uStack_64 = uStack_34;
  42	      uStack_60 = (undefined4)uStack_30;
  43	      uStack_5c = uStack_30._4_4_;
  44	      uStack_78 = uStack_48;
  45	      uStack_74 = uStack_44;
  46	      uStack_70 = (undefined4)uStack_40;
  47	      uStack_6c = uStack_40._4_4_;
  48	      uStack_88 = (undefined4)uStack_58;
  49	      uStack_84 = uStack_58._4_4_;
  50	      uStack_80 = uStack_50;
  51	      uStack_7c = uStack_4c;
  52	      uStack_21 = 0;
  53	      FUN_140b864c0(param_1,&uStack_88);
  54	      return param_1;
  55	    }
  56	  }
  57	  *(longlong *)(param_1 + 8) = lVar1;
  58	  *param_1 = 6;
  59	  FUN_140898f40(&uStack_40);
  60	  if ((!SBORROW8(0,uStack_58)) && (uStack_58 != 0)) {
  61	    func_0x000140613c20(CONCAT44(uStack_4c,uStack_50),uStack_58,1);
  62	  }
  63	  return param_1;
  64	}
  65	