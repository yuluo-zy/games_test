# 本次业务闭包审查得到的可复用经验

1. **参数不是猜十进制。** 原MOVSD搬两lane必须逐byte重算RIP=nextInstruction+signedDisp32，再映射相同SHA的PE原始段取完整8B。trap142a85d70为6666a63f0000003f，即(1.3,.5)，不能把另一表中零/1.3或反编译栈槽误当前两轴。wall1429258d8的.3也不能沿现代旧source猜成.1。
2. **predicate要连原返回编码看caller分支。** trap和cottage同调Roof::is_flat，但JE/JNE相反；同一helper被调用不代表相同前置门。circle Option必须从完整callee锁tag0/1，再看callerCMP/JNE，不能凭“交点应存在”的直觉决定允许极性。
3. **失败次序属于业务。** 原entry在动态mesh构造前，interaction在entry后，缺ID/缺cache/缺PublicWall/缺interaction的部分mutation不同。用insert_for保真实allocation/source/flags，不能先本地new再末尾legacyinsert；否则consumer的Mesh槽required source会panic。
4. **同几何不等于同所有权。** library由borrowed record提取同outer Arc；trap原dataClone是不同outer/sameinner SharedShape；wall烘焙worldverts后相同outer Arc。禁止统一成“总Arc::clone”或复制两次primitive。
5. **浮点异常值不能随意规范化。** circle原207B只strict ordered >退出，同圆/NaN可返回Some(NaN)；trap只消费tag，改None会翻业务门。clamp、finite、zero-distance“改良”必须有原证，不因数学更漂亮就加入。
6. **混合modified集合是实际producer域。** flag/arrow需原Info.ty前门，subtype23/rank1在loader故意不存在，先查库就是错。World夹具应使用真实ty与extra组合（3bChimney/3dFlag/38,3aWindow），不能靠本轮函数恰未读extra而塞非法tuple。
7. **审查结果要能配对应用。** 原before→候选patch、阶段after→nextpatch、完整hash都保存；独审纠正常量/测试后更新唯一delivery。v1/v2冻结留追溯，不原地覆盖历史审核。合同和source不能把已接通data/collider说成全视觉/GPU验收，复杂剩余桥接逐点列明。
