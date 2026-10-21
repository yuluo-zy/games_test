# NaN坡度分类勘误（2026-10-21）

绑定旧COMPUTE-CONTRACT.md（SHA f4ecb3483a68d627f14594d70b16a8c763868d30977b4b286661ed92f8f6f175），只纠正其collapse段与建议验证中“NaN坡度kind3”的错误，旧freeze不修改。

原compute-full.asm.txt完整RNE证据：140808002 `ucomiss xmm13(.25),xmm0(slope)`；140808006 `seta al`；140808009 `ucomiss xmm14(2.5),xmm0`；14080800d `mov esi,3`；140808012 `ja 140808019`；140808014 `inc al`；140808016 `movzx esi,al`。NaN时有序above两次均为false，故不跳ja，AL从0加1，kind=1。正确高层表达是`if slope > 2.5 {3} else if slope > 0.25 {2} else {1}`，不能倒写`if slope <= 2.5 {...} else {3}`。零弦且零高度差给NaN，同样kind1，不应擅自classify成梯段。

本次walk_shape::closure0（1412b2540/1264B）也有独立相同事实：1412b2676比较.25、267d seta、2680 inc；2682比较2.5、268c默认3、2691 cmovbe选AL。unordered使cmovbe成立，kind1。两函数都按上述strict有序表达分类。其余merge/count/replace分支仍须按各自原码，不能把这处同值当全函数同码证明。

已通知duck唯一writer与lead立即复核候选。此勘误只读，没有运行Cargo/原目标，没有修改源代码；作者先前NaN结论被此原指令更正。
