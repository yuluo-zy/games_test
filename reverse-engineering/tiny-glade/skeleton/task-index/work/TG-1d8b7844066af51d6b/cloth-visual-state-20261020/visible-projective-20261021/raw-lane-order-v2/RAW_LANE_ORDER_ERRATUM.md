# 513B第二范数求和lane勘误

绑定旧合同8a2f9d32b81d989322ca0af9ed1c0ed8a84dbbe5c1306a18c95b8b5ae1ef6d1e和旧候选d67e38d2eaf44af7cb0768f9c14e950561de6478d37d1ed234c05045f71f8c58；旧freeze不回写。lead源码独审发现此前将raw SIMD lane直接当canonical right lane，求和顺序记错。

原c91286乘减后raw=(right.y,right.z,right.x,_)；c9128c SHUFPS d2将raw重排为canonical right=(raw2,raw0,raw1)。c91290先平方raw；c91296 UNPCKHPD取raw2²；c9129a加raw0²，形成right.x²+right.y²；c9129e MOVSHDUP取raw1²；c912a2把前和加到raw1²。因此正确范数为sqrt(right.z²+(right.x²+right.y²))，不是旧合同的sqrt(right.y²+(right.z²+right.x²))。

本增量只更正该一行算序；可证明有效平面right.y²为+0，两式对有限正常输出相同仍不能称旧式为逐原算序。所有513首XZ门、Y忽略、四分支、607与其它lib字节、4测试均保留。未Cargo/原运行oracle，不借有限等价夸全NaN位等价。combined-after-607.patch供root从35dd一次整合；candidate.patch仅d67→v2。
