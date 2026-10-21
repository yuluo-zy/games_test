# trap碰撞尺寸原字节勘误（2026-10-21）

绑定TRAP-COLLIDER-CONTRACT.md SHA4a4751ac2c6c645e9106227866b54b27201eec41b2e58386980252ca35a24db3；旧正文/冻结不回写。

旧“142a85d70为[0,1.3]、cuboid尺寸(0,1.3,1.3)”结论错误，被tree独立重核及本worker再次直接读取RNE纠正：本次全文件SHA仍f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。原bea8d指令九字节f2440f100dda722c02，下一RIP1407bea96加有符号disp022c72da，目标142a85d70。按当前PEsection映射读取该目标8B，实际6666a63f0000003f，即f32 bits3fa66666(1.3)、3f000000(.5)。原1407bef89将该XMM9的XY作为dims.xy，原1407bef92写dims.z的1.3，因此真实完整cuboid尺寸为(1.3,.5,1.3)。不得沿旧错误尺寸实现/测试。

直接复核的tree证据位于TG-0e52e2ec64528325e0/core-visual-data-20261021对应work的trap-constant-direct-correction.json（tree唯一writer）；原corner/roof/loader无需因本项重复采全程序。此勘误仅纠正trap尺寸，newArc两外层/同形状、无缩放、cached pose的其他证据仍各按原完整body审查，不能拿其成立掩盖旧尺寸错误。已通知lead/root/唯一源码writer，无生产/Cargo/GPU修改或运行验收。
