1416f11a0 push     rbp
1416f11a1 push     r15
1416f11a3 push     r14
1416f11a5 push     r13
1416f11a7 push     r12
1416f11a9 push     rsi
1416f11aa push     rdi
1416f11ab push     rbx
1416f11ac sub      rsp, 0x438
1416f11b3 lea      rbp, [rsp + 0x80]
1416f11bb movaps   xmmword ptr [rbp + 0x3a0], xmm9
1416f11c3 movaps   xmmword ptr [rbp + 0x390], xmm8
1416f11cb movaps   xmmword ptr [rbp + 0x380], xmm7
1416f11d2 movaps   xmmword ptr [rbp + 0x370], xmm6
1416f11d9 mov      qword ptr [rbp + 0x368], 0xfffffffffffffffe
1416f11e4 mov      qword ptr [rbp + 0x350], r9
1416f11eb mov      qword ptr [rbp + 0x338], rdx
1416f11f2 mov      r15, qword ptr [r8]
1416f11f5 mov      rax, qword ptr [r8 + 8]
1416f11f9 movsd    xmm0, qword ptr [r8 + 0x10]
1416f11ff mov      rdx, qword ptr [r15 + 0xf8]
1416f1206 mov      r8, qword ptr [r15 + 0x100]
1416f120d movzx    r9d, byte ptr [r15 + 0x108]
1416f1215 test     r9b, r9b
1416f1218 je       0x1416f12a6
1416f121e lea      r10, [rbp - 0x48]
1416f1222 lea      r11, [rbp - 0x40]
1416f1226 lea      rsi, [rbp - 0x38]
1416f122a lea      r12, [r8*4]
1416f1232 xor      ebx, ebx
1416f1234 mov      rcx, rdx
1416f1237 nop      word ptr [rax + rax]
1416f1240 test     r12, r12
1416f1243 je       0x1416f13b9
1416f1249 mov      edi, dword ptr [rcx]
1416f124b add      rcx, 4
1416f124f mov      r14, qword ptr [rax + 0x1a8]
1416f1256 lea      r13, [rdi + rdi*8]
1416f125a mov      rdi, qword ptr [r14 + r13*8 + 0x10]
1416f125f add      r12, -4
1416f1263 test     rdi, rdi
1416f1266 je       0x1416f1240
1416f1268 lea      rbx, [r14 + r13*8]
1416f126c mov      r14, qword ptr [r15 + 0x110]
1416f1273 mov      r15, qword ptr [rbx + 0x18]
1416f1277 mov      rbx, qword ptr [rbx + 0x38]
1416f127b mov      rbx, qword ptr [rbx + r14*8]
1416f127f not      rbx
1416f1282 lea      r14, [rbx + rbx*2]
1416f1286 shl      r14, 4
1416f128a mov      rbx, qword ptr [r15 + r14 + 0x28]
1416f128f mov      r13, qword ptr [r15 + r14 + 0x10]
1416f1294 mov      r14, qword ptr [r15 + r14 + 0x20]
1416f1299 mov      qword ptr [rbp - 0x50], r13
1416f129d mov      qword ptr [rbp - 0x48], r14
1416f12a1 jmp      0x1416f1365
1416f12a6 lea      r10, [rbp + 0x160]
1416f12ad lea      r11, [rbp + 0x168]
1416f12b4 lea      rsi, [rbp + 0x170]
1416f12bb lea      r12, [r8*4]
1416f12c3 xor      ebx, ebx
1416f12c5 mov      rcx, rdx
1416f12c8 nop      dword ptr [rax + rax]
1416f12d0 test     r12, r12
1416f12d3 je       0x1416f13b9
1416f12d9 mov      edi, dword ptr [rcx]
1416f12db add      rcx, 4
1416f12df mov      r13, qword ptr [rax + 0x100]
1416f12e6 lea      r14, [rdi + rdi*4]
1416f12ea shl      r14, 5
1416f12ee mov      rdi, qword ptr [r13 + r14 + 0x58]
1416f12f3 add      r12, -4
1416f12f7 test     rdi, rdi
1416f12fa je       0x1416f12d0
1416f12fc add      r13, r14
1416f12ff mov      ebx, dword ptr [r13 + 0x94]
1416f1306 mov      r14, qword ptr [rax + 0x1a8]
1416f130d lea      rbx, [rbx + rbx*8]
1416f1311 mov      r15, qword ptr [r15 + 0x110]
1416f1318 mov      r12, qword ptr [r14 + rbx*8 + 0x18]
1416f131d mov      rbx, qword ptr [r14 + rbx*8 + 0x38]
1416f1322 mov      rbx, qword ptr [rbx + r15*8]
1416f1326 not      rbx
1416f1329 mov      r14, qword ptr [r13 + 0x50]
1416f132d lea      r15, [rbx + rbx*2]
1416f1331 shl      r15, 4
1416f1335 mov      ebx, dword ptr [r14 + 8]
1416f1339 lea      r13, [rbx + rbx*4]
1416f133d shl      r13, 4
1416f1341 add      r13, qword ptr [r12 + r15 + 0x10]
1416f1346 shl      rbx, 2
1416f134a mov      r14, qword ptr [r12 + r15 + 0x20]
1416f134f add      r14, rbx
1416f1352 add      rbx, qword ptr [r12 + r15 + 0x28]
1416f1357 mov      qword ptr [rbp + 0x158], r13
1416f135e mov      qword ptr [rbp + 0x160], r14
1416f1365 mov      qword ptr [r11], rbx
1416f1368 movlps   qword ptr [rsi], xmm0
1416f136b mov      r11, qword ptr [r10 + 8]
1416f136f mov      r10, qword ptr [r10 + 0x10]
1416f1373 mov      ebx, 1
1416f1378 lea      rdx, [rdx + r8*4]
1416f137c test     r9b, r9b
1416f137f jne      0x1416f13c7
1416f1381 cmp      rbx, rdi
1416f1384 jne      0x1416f13ef
1416f1386 nop      word ptr cs:[rax + rax]
1416f1390 cmp      rcx, rdx
1416f1393 je       0x1416f1445
1416f1399 mov      r8d, dword ptr [rcx]
1416f139c add      rcx, 4
1416f13a0 mov      r9, qword ptr [rax + 0x100]
1416f13a7 lea      r8, [r8 + r8*4]
1416f13ab shl      r8, 5
1416f13af cmp      qword ptr [r9 + r8 + 0x58], 0
1416f13b5 je       0x1416f1390
1416f13b7 jmp      0x1416f13ef
1416f13b9 xor      edi, edi
1416f13bb xor      r13d, r13d
1416f13be lea      rdx, [rdx + r8*4]
1416f13c2 test     r9b, r9b
1416f13c5 je       0x1416f1381
1416f13c7 cmp      rbx, rdi
1416f13ca jne      0x1416f13ef
1416f13cc nop      dword ptr [rax]
1416f13d0 cmp      rcx, rdx
1416f13d3 je       0x1416f1445
1416f13d5 mov      r8d, dword ptr [rcx]
1416f13d8 add      rcx, 4
1416f13dc mov      r9, qword ptr [rax + 0x1a8]
1416f13e3 lea      r8, [r8 + r8*8]
1416f13e7 cmp      qword ptr [r9 + r8*8 + 0x10], 0
1416f13ed je       0x1416f13d0
1416f13ef test     r13, r13
1416f13f2 sete     al
1416f13f5 xor      al, 1
1416f13f7 movzx    eax, al
1416f13fa mov      qword ptr [rbp + 0x158], rax
1416f1401 lea      rax, [rip + 0x159ee60]
1416f1408 mov      qword ptr [rbp + 0x160], rax
1416f140f mov      qword ptr [rbp + 0x168], 0x5f
1416f141a lea      rax, [rip + 0x159efaf]
1416f1421 mov      qword ptr [rsp + 0x20], rax
1416f1426 lea      rcx, [rip + 0x159eb83]
1416f142d lea      r9, [rip + 0x159eb5c]
1416f1434 lea      r8, [rbp + 0x158]
1416f143b mov      edx, 0x2b
1416f1440 call     0x1428d9760
1416f1445 test     r13, r13
1416f1448 sete     al
1416f144b je       0x1416f13f5
1416f144d shr      r10, 0x20
1416f1451 mov      dword ptr [r11], r10d
1416f1454 xor      eax, eax
1416f1456 cmp      rax, qword ptr [r13]
1416f145a jno      0x1416f1468
1416f145c movabs   rax, 0x8000000000000000
1416f1466 jmp      0x1416f147b
1416f1468 mov      rcx, qword ptr [rbp + 0x338]
1416f146f mov      rdx, r13
1416f1472 call     0x1416f24f0
1416f1477 mov      rax, qword ptr [r13]
1416f147b neg      rax
1416f147e setno    byte ptr [rbp + 0x367]
1416f1485 mov      rdi, qword ptr [rbp + 0x430]
1416f148c setno    bl
1416f148f cmp      byte ptr [rip + 0x1f43842], 0
1416f1496 je       0x1416f15be
1416f149c mov      rax, qword ptr [rbp + 0x440]
1416f14a3 mov      rcx, qword ptr [rax]
1416f14a6 mov      dword ptr [rbp + 0x158], 0x28
1416f14b0 add      rcx, 0x20
1416f14b4 lea      rdx, [rbp + 0x158]
1416f14bb call     0x14089aef0
1416f14c0 test     rax, rax
1416f14c3 je       0x1416f15be
1416f14c9 xor      bl, 1
1416f14cc mov      byte ptr [rbp + 0x367], bl
1416f14d2 mov      rax, qword ptr [rip + 0x1f3aecf]
1416f14d9 mov      rax, qword ptr [rax]
1416f14dc cmp      rax, 3
1416f14e0 jb       0x1416f15be
1416f14e6 lea      rax, [rbp + 0x367]
1416f14ed mov      qword ptr [rbp - 0x50], rax
1416f14f1 lea      rax, [rip - 0x9691d8]
1416f14f8 mov      qword ptr [rbp - 0x48], rax
1416f14fc lea      rcx, [rip + 0x159ef05]
1416f1503 call     0x14067dd80
1416f1508 movups   xmm0, xmmword ptr [rax]
1416f150b mov      eax, dword ptr [rax + 0x10]
1416f150e mov      qword ptr [rbp + 0x188], 3
1416f1519 lea      rcx, [rip + 0x159ef00]
1416f1520 mov      qword ptr [rbp + 0x190], rcx
1416f1527 mov      qword ptr [rbp + 0x198], 0x2c
1416f1532 lea      rdx, [rip + 0x159eebf]
1416f1539 mov      qword ptr [rbp + 0x1a8], rdx
1416f1540 mov      qword ptr [rbp + 0x1b0], 1
1416f154b lea      rdx, [rbp - 0x50]
1416f154f mov      qword ptr [rbp + 0x1b8], rdx
1416f1556 mov      qword ptr [rbp + 0x1c0], 1
1416f1561 mov      qword ptr [rbp + 0x1c8], 0
1416f156c mov      qword ptr [rbp + 0x158], 0
1416f1577 mov      qword ptr [rbp + 0x160], rcx
1416f157e mov      qword ptr [rbp + 0x168], 0x2c
1416f1589 mov      qword ptr [rbp + 0x170], 0
1416f1594 movups   xmmword ptr [rbp + 0x178], xmm0
1416f159b mov      dword ptr [rbp + 0x1a0], 1
1416f15a5 mov      dword ptr [rbp + 0x1a4], eax
1416f15ab lea      rcx, [rbp + 0x138]
1416f15b2 lea      rdx, [rbp + 0x158]
1416f15b9 call     0x140d4a330
1416f15be mov      rsi, qword ptr [rbp + 0x438]
1416f15c5 mov      r12, qword ptr [rdi]
1416f15c8 cmp      byte ptr [r12 + 0xf5], 0
1416f15d1 je       0x1416f17e1
1416f15d7 cmp      byte ptr [r12 + 0xf0], 0
1416f15e0 je       0x1416f17e1
1416f15e6 mov      rax, qword ptr [rbp + 0x428]
1416f15ed mov      rbx, qword ptr [rax]
1416f15f0 test     rbx, rbx
1416f15f3 je       0x1416f186d
1416f15f9 add      r12, 0xf5
1416f1600 lea      rdx, [rip + 0x159ee45]
1416f1607 lea      r15, [rbp - 0x50]
1416f160b mov      r8d, 0xd
1416f1611 mov      rcx, r15
1416f1614 call     0x140adb2f0
1416f1619 mov      eax, dword ptr [rdi + 0x1c]
1416f161c mov      rcx, qword ptr [rdi + 0x10]
1416f1620 mov      dword ptr [rcx], eax
1416f1622 mov      qword ptr [rbp + 0xd8], r12
1416f1629 lea      rcx, [rbp + 0x158]
1416f1630 mov      r8d, 0x188
1416f1636 mov      rdx, r15
1416f1639 call     0x142923c00
1416f163e movzx    eax, byte ptr [rip + 0x1f435c4]
1416f1645 mov      ecx, 0x48
1416f164a mov      edx, 8
1416f164f call     0x140613c10
1416f1654 test     rax, rax
1416f1657 mov      qword ptr [rbp + 0x358], rax
1416f165e je       0x1416f245e
1416f1664 mov      r11, rax
1416f1667 lea      r10, [r13 + 0x40]
1416f166b lea      r9, [r13 + 0x48]
1416f166f lea      rax, [r13 + 0x18]
1416f1673 lea      rcx, [r13 + 0x38]
1416f1677 lea      rdx, [r13 + 0x30]
1416f167b lea      r8, [rbp + 0x367]
1416f1682 mov      qword ptr [r11], r8
1416f1685 mov      qword ptr [rbp + 0x318], r10
1416f168c mov      qword ptr [r11 + 8], r10
1416f1690 mov      qword ptr [rbp + 0x2f0], r9
1416f1697 mov      qword ptr [r11 + 0x10], r9
1416f169b mov      qword ptr [r11 + 0x18], rdx
1416f169f mov      qword ptr [r11 + 0x20], rcx
1416f16a3 mov      qword ptr [r11 + 0x28], r13
1416f16a7 mov      qword ptr [r11 + 0x30], rax
1416f16ab mov      rax, qword ptr [rbp + 0x338]
1416f16b2 mov      qword ptr [r11 + 0x38], rax
1416f16b6 mov      qword ptr [r11 + 0x40], rsi
1416f16ba lea      rax, [rip + 0x159e95f]
1416f16c1 mov      qword ptr [rsp + 0x20], rax
1416f16c6 lea      rcx, [rbp - 0x50]
1416f16ca lea      rdx, [rbp + 0x158]
1416f16d1 mov      r8, rbx
1416f16d4 mov      r9, r11
1416f16d7 call     0x140adb610
1416f16dc mov      qword ptr [rbp + 0x328], r13
1416f16e3 mov      rsi, qword ptr [rbp + 0x420]
1416f16ea cmp      dword ptr [rbp - 0x10], 2
1416f16ee je       0x1416f1703
1416f16f0 mov      rax, qword ptr [rbp - 0x40]
1416f16f4 lock dec qword ptr [rax]
1416f16f8 jne      0x1416f1703
1416f16fa lea      rcx, [rbp - 0x40]
1416f16fe call     0x140c41170
1416f1703 mov      rax, qword ptr [rsi]
1416f1706 mov      qword ptr [rbp + 0x320], rax
1416f170d mov      rax, qword ptr [rbp + 0x350]
1416f1714 mov      rcx, qword ptr [rax]
1416f1717 mov      rax, qword ptr [rbp + 0x338]
1416f171e mov      rdx, qword ptr [rax]
1416f1721 mov      r8, qword ptr [rax + 8]
1416f1725 mov      eax, dword ptr [rax + 0x14]
1416f1728 mov      dword ptr [rbp + 0x344], eax
1416f172e mov      r14, qword ptr [rdx + 0xf8]
1416f1735 mov      rax, qword ptr [rdx + 0x100]
1416f173c lea      r10, [r14 + rax*4]
1416f1740 mov      qword ptr [rbp + 0x300], rdx
1416f1747 movzx    eax, byte ptr [rdx + 0x108]
1416f174e mov      byte ptr [rbp + 0x350], al
1416f1754 movss    xmm0, dword ptr [rcx + 0x10]
1416f1759 mulss    xmm0, xmm0
1416f175d movsd    xmm1, qword ptr [rcx + 0x14]
1416f1762 mulps    xmm1, xmm1
1416f1765 addss    xmm0, xmm1
1416f1769 movshdup xmm1, xmm1
1416f176d addss    xmm1, xmm0
1416f1771 sqrtss   xmm6, xmm1
1416f1775 mov      qword ptr [rbp + 0x330], rcx
1416f177c lea      rax, [rcx + 4]
1416f1780 mov      qword ptr [rbp + 0x308], rax
1416f1787 mov      eax, 8
1416f178c mov      qword ptr [rbp + 0x2e8], rax
1416f1793 mov      qword ptr [rbp + 0x358], 0
1416f179e xorps    xmm7, xmm7
1416f17a1 movsd    xmm8, qword ptr [rip + 0x138fca6]
1416f17aa movss    xmm9, dword ptr [rip + 0x126eec1]
1416f17b3 mov      qword ptr [rbp + 0x348], 0
1416f17be xor      ebx, ebx
1416f17c0 xor      r12d, r12d
1416f17c3 mov      qword ptr [rbp + 0x2f8], 0
1416f17ce mov      qword ptr [rbp + 0x148], r8
1416f17d5 mov      qword ptr [rbp + 0x150], r10
1416f17dc jmp      0x1416f1c68
1416f17e1 mov      rbx, qword ptr [rsi + 8]
1416f17e5 test     rbx, rbx
1416f17e8 jne      0x1416f17ee
1416f17ea mov      rbx, qword ptr [rsi + 0x10]
1416f17ee mov      r12, qword ptr [r13 + 0x30]
1416f17f2 mov      rax, qword ptr [rbx]
1416f17f5 mov      r14, qword ptr [rbx + 0x10]
1416f17f9 sub      rax, r14
1416f17fc cmp      rax, 0x17
1416f1800 jbe      0x1416f22c1
1416f1806 mov      rax, qword ptr [rbx + 8]
1416f180a lea      rdi, [rip - 0x4149a1]
1416f1811 mov      qword ptr [rax + r14], rdi
1416f1815 lea      r15, [rip - 0x413c4c]
1416f181c mov      qword ptr [rax + r14 + 8], r15
1416f1821 mov      qword ptr [rax + r14 + 0x10], r12
1416f1826 add      r14, 0x18
1416f182a mov      qword ptr [rbx + 0x10], r14
1416f182e mov      rbx, qword ptr [rsi + 8]
1416f1832 test     rbx, rbx
1416f1835 jne      0x1416f183b
1416f1837 mov      rbx, qword ptr [rsi + 0x10]
1416f183b mov      rsi, qword ptr [r13 + 0x38]
1416f183f mov      rax, qword ptr [rbx]
1416f1842 mov      r14, qword ptr [rbx + 0x10]
1416f1846 sub      rax, r14
1416f1849 cmp      rax, 0x17
1416f184d jbe      0x1416f22e6
1416f1853 mov      rax, qword ptr [rbx + 8]
1416f1857 mov      qword ptr [rax + r14], rdi
1416f185b mov      qword ptr [rax + r14 + 8], r15
1416f1860 mov      qword ptr [rax + r14 + 0x10], rsi
1416f1865 add      r14, 0x18
1416f1869 mov      qword ptr [rbx + 0x10], r14
1416f186d mov      rsi, qword ptr [rbp + 0x448]
1416f1874 mov      rax, qword ptr [r13]
1416f1878 cmp      byte ptr [rbp + 0x367], 0
1416f187f je       0x1416f19b9
1416f1885 neg      rax
1416f1888 jno      0x1416f19e8
1416f188e mov      r8, qword ptr [rbp + 0x338]
1416f1895 mov      rax, qword ptr [r8]
1416f1898 mov      rcx, qword ptr [r8 + 8]
1416f189c mov      edx, dword ptr [r8 + 0x10]
1416f18a0 mov      r8d, dword ptr [r8 + 0x14]
1416f18a4 lea      r9, [rcx + 0x1a0]
1416f18ab lea      r10, [rcx + 0xf8]
1416f18b2 mov      r11, qword ptr [rax + 0xf8]
1416f18b9 mov      rdi, qword ptr [rax + 0x100]
1416f18c0 lea      rdi, [r11 + rdi*4]
1416f18c4 movzx    ebx, byte ptr [rax + 0x108]
1416f18cb mov      byte ptr [rbp + 0x218], bl
1416f18d1 mov      qword ptr [rbp + 0x178], r11
1416f18d8 mov      qword ptr [rbp + 0x180], rdi
1416f18df mov      qword ptr [rbp + 0x188], 8
1416f18ea mov      qword ptr [rbp + 0x190], 0
1416f18f5 mov      qword ptr [rbp + 0x198], 8
1416f1900 xorps    xmm0, xmm0
1416f1903 movups   xmmword ptr [rbp + 0x1a0], xmm0
1416f190a mov      dword ptr [rbp + 0x1c0], edx
1416f1910 mov      dword ptr [rbp + 0x1c4], r8d
1416f1917 mov      qword ptr [rbp + 0x1c8], 0
1416f1922 mov      dword ptr [rbp + 0x1e0], edx
1416f1928 mov      dword ptr [rbp + 0x1e4], r8d
1416f192f mov      qword ptr [rbp + 0x1e8], 0
1416f193a mov      dword ptr [rbp + 0x200], edx
1416f1940 mov      dword ptr [rbp + 0x204], r8d
1416f1947 movups   xmmword ptr [rbp + 0x208], xmm0
1416f194e mov      qword ptr [rbp + 0x158], rcx
1416f1955 mov      qword ptr [rbp + 0x160], r9
1416f195c mov      qword ptr [rbp + 0x168], r10
1416f1963 mov      qword ptr [rbp + 0x170], rax
1416f196a lea      r8, [rip + 0x157c4b7]
1416f1971 lea      rcx, [rbp - 0x50]
1416f1975 lea      rdx, [rbp + 0x158]
1416f197c call     0x141304bd0
1416f1981 mov      rax, qword ptr [r13]
1416f1985 mov      rcx, rax
1416f1988 neg      rcx
1416f198b jo       0x1416f19a6
1416f198d jae      0x1416f19a6
1416f198f mov      rcx, qword ptr [r13 + 8]
1416f1993 shl      rax, 3
1416f1997 lea      rdx, [rax + rax*4]
1416f199b mov      r8d, 4
1416f19a1 call     0x140613c20
1416f19a6 mov      rax, qword ptr [rbp - 0x40]
1416f19aa mov      qword ptr [r13 + 0x10], rax
1416f19ae movups   xmm0, xmmword ptr [rbp - 0x50]
1416f19b2 movups   xmmword ptr [r13], xmm0
1416f19b7 jmp      0x1416f19e8
1416f19b9 mov      rcx, rax
1416f19bc neg      rcx
1416f19bf jo       0x1416f19da
1416f19c1 jae      0x1416f19da
1416f19c3 mov      rcx, qword ptr [r13 + 8]
1416f19c7 shl      rax, 3
1416f19cb lea      rdx, [rax + rax*4]
1416f19cf mov      r8d, 4
1416f19d5 call     0x140613c20
1416f19da movabs   rax, 0x8000000000000000
1416f19e4 mov      qword ptr [r13], rax
1416f19e8 mov      rax, qword ptr [rsi]
1416f19eb mov      rcx, qword ptr [rsi + 8]
1416f19ef mov      rdx, qword ptr [rax]
1416f19f2 xor      r8d, r8d
1416f19f5 mov      r9, rdx
1416f19f8 sub      r9, qword ptr [rcx + 0x18]
1416f19fc cmovb    r9, r8
1416f1a00 sub      rdx, qword ptr [rcx + 0x38]
1416f1a04 cmovb    rdx, r8
1416f1a08 mov      r10, qword ptr [rcx + 0x30]
1416f1a0c sub      r10, rdx
1416f1a0f cmovb    r10, r8
1416f1a13 setbe    dl
1416f1a16 mov      r11, qword ptr [rcx + 0x10]
1416f1a1a sub      r11, r9
1416f1a1d cmovb    r11, r8
1416f1a21 mov      rcx, qword ptr [rcx + 0x40]
1416f1a25 setbe    r8b
1416f1a29 add      r11, r10
1416f1a2c mov      r9, rcx
1416f1a2f sub      r9, r11
1416f1a32 mov      qword ptr [rax], r9
1416f1a35 test     r8b, dl
1416f1a38 jne      0x1416f1a70
1416f1a3a mov      qword ptr [rax], rcx
1416f1a3d mov      rax, qword ptr [r13]
1416f1a41 mov      rcx, rax
1416f1a44 neg      rcx
1416f1a47 jo       0x1416f1a62
1416f1a49 jae      0x1416f1a62
1416f1a4b mov      rcx, qword ptr [r13 + 8]
1416f1a4f shl      rax, 3
1416f1a53 lea      rdx, [rax + rax*4]
1416f1a57 mov      r8d, 4
1416f1a5d call     0x140613c20
1416f1a62 movabs   rax, 0x8000000000000000
1416f1a6c mov      qword ptr [r13], rax
1416f1a70 movaps   xmm6, xmmword ptr [rbp + 0x370]
1416f1a77 movaps   xmm7, xmmword ptr [rbp + 0x380]
1416f1a7e movaps   xmm8, xmmword ptr [rbp + 0x390]
1416f1a86 movaps   xmm9, xmmword ptr [rbp + 0x3a0]
1416f1a8e add      rsp, 0x438
1416f1a95 pop      rbx
1416f1a96 pop      rdi
1416f1a97 pop      rsi
1416f1a98 pop      r12
1416f1a9a pop      r13
1416f1a9c pop      r14
1416f1a9e pop      r15
1416f1aa0 pop      rbp
1416f1aa1 ret      
1416f1aa2 lea      rax, [rax + rcx*8]
1416f1aa6 mov      rcx, qword ptr [rax + 0x18]
1416f1aaa mov      rax, qword ptr [rax + 0x38]
1416f1aae mov      r8, qword ptr [rbp + 0x300]
1416f1ab5 mov      rdx, qword ptr [r8 + 0x110]
1416f1abc mov      r8, qword ptr [r8 + 0x118]
1416f1ac3 mov      rdx, qword ptr [rax + rdx*8]
1416f1ac7 not      rdx
1416f1aca lea      rdx, [rdx + rdx*2]
1416f1ace shl      rdx, 4
1416f1ad2 mov      r9, qword ptr [rcx + rdx + 0x10]
1416f1ad7 mov      qword ptr [rbp + 0x348], r9
1416f1ade mov      rdx, qword ptr [rcx + rdx + 0x28]
1416f1ae3 mov      qword ptr [rbp + 0x310], rdx
1416f1aea mov      rax, qword ptr [rax + r8*8]
1416f1aee not      rax
1416f1af1 lea      rax, [rax + rax*2]
1416f1af5 shl      rax, 4
1416f1af9 mov      rax, qword ptr [rcx + rax + 0x10]
1416f1afe mov      qword ptr [rbp + 0x358], rax
1416f1b05 xor      r12d, r12d
1416f1b08 mov      eax, r12d
1416f1b0b lea      rsi, [rax + rax*2]
1416f1b0f shl      rsi, 4
1416f1b13 add      rsi, qword ptr [rbp + 0x348]
1416f1b1a mov      rcx, qword ptr [rbp + 0x310]
1416f1b21 lea      rcx, [rcx + rax*4]
1416f1b25 mov      qword ptr [rbp + 0x2e0], rcx
1416f1b2c imul     r15, rax, 0x68
1416f1b30 add      r15, qword ptr [rbp + 0x358]
1416f1b37 inc      r12
1416f1b3a lea      rax, [r15 + 0x20]
1416f1b3e mov      qword ptr [rbp + 0x138], rax
1416f1b45 lea      rax, [rip - 0x96448c]
1416f1b4c mov      qword ptr [rbp + 0x140], rax
1416f1b53 lea      rax, [rip + 0x159ea4e]
1416f1b5a mov      qword ptr [rbp + 0x158], rax
1416f1b61 mov      qword ptr [rbp + 0x160], 1
1416f1b6c lea      rax, [rbp + 0x138]
1416f1b73 mov      qword ptr [rbp + 0x168], rax
1416f1b7a mov      qword ptr [rbp + 0x170], 1
1416f1b85 mov      qword ptr [rbp + 0x178], 0
1416f1b90 lea      rdi, [rbp - 0x50]
1416f1b94 mov      rcx, rdi
1416f1b97 lea      r13, [rbp + 0x158]
1416f1b9e mov      rdx, r13
1416f1ba1 call     0x140d79f90
1416f1ba6 movss    xmm0, dword ptr [rsi + 0x18]
1416f1bab addss    xmm0, xmm7
1416f1baf movsd    xmm1, qword ptr [rsi + 0x10]
1416f1bb4 addps    xmm1, xmm8
1416f1bb8 mov      byte ptr [rbp + 0x1e0], 0
1416f1bbf mov      dword ptr [rbp + 0x1e4], 0x3fa00000
1416f1bc9 movlps   qword ptr [rbp + 0x1e8], xmm1
1416f1bd0 movss    dword ptr [rbp + 0x1f0], xmm0
1416f1bd8 mov      word ptr [rbp + 0x1d0], 0xe01
1416f1be1 mov      dword ptr [rbp + 0x170], 1
1416f1beb mov      qword ptr [rbp + 0x178], 0
1416f1bf6 mov      dword ptr [rbp + 0x188], 2
1416f1c00 mov      dword ptr [rbp + 0x1a0], 3
1416f1c0a mov      dword ptr [rbp + 0x1b8], 3
1416f1c14 mov      qword ptr [rbp + 0x158], 0
1416f1c1f mov      qword ptr [rbp + 0x160], 8
1416f1c2a mov      qword ptr [rbp + 0x168], 0
1416f1c35 mov      word ptr [rbp + 0x200], 0x101
1416f1c3e mov      rcx, qword ptr [rbp + 0x320]
1416f1c45 mov      rdx, rdi
1416f1c48 mov      r8, r13
1416f1c4b call     0x1409231f0
1416f1c50 cmp      byte ptr [rax + 0x18], 2
1416f1c54 mov      r8, qword ptr [rbp + 0x148]
1416f1c5b mov      r10, qword ptr [rbp + 0x150]
1416f1c62 jae      0x1416f1d83
1416f1c68 cmp      byte ptr [rbp + 0x350], 0
1416f1c6f je       0x1416f1cb0
1416f1c71 cmp      r12, rbx
1416f1c74 jne      0x1416f1b08
1416f1c7a nop      word ptr [rax + rax]
1416f1c80 cmp      r14, r10
1416f1c83 je       0x1416f1e18
1416f1c89 mov      ecx, dword ptr [r14]
1416f1c8c add      r14, 4
1416f1c90 mov      rax, qword ptr [r8 + 0x1a8]
1416f1c97 lea      rcx, [rcx + rcx*8]
1416f1c9b mov      rbx, qword ptr [rax + rcx*8 + 0x10]
1416f1ca0 test     rbx, rbx
1416f1ca3 je       0x1416f1c80
1416f1ca5 jmp      0x1416f1aa2
1416f1caa nop      word ptr [rax + rax]
1416f1cb0 cmp      r12, rbx
1416f1cb3 jne      0x1416f1d6c
1416f1cb9 nop      dword ptr [rax]
1416f1cc0 cmp      r14, r10
1416f1cc3 je       0x1416f1e18
1416f1cc9 mov      ecx, dword ptr [r14]
1416f1ccc add      r14, 4
1416f1cd0 mov      rax, qword ptr [r8 + 0x100]
1416f1cd7 lea      rcx, [rcx + rcx*4]
1416f1cdb shl      rcx, 5
1416f1cdf mov      rbx, qword ptr [rax + rcx + 0x58]
1416f1ce4 test     rbx, rbx
1416f1ce7 je       0x1416f1cc0
1416f1ce9 add      rax, rcx
1416f1cec mov      ecx, dword ptr [rax + 0x94]
1416f1cf2 mov      rdx, qword ptr [r8 + 0x1a8]
1416f1cf9 lea      rcx, [rcx + rcx*8]
1416f1cfd mov      r8, qword ptr [rdx + rcx*8 + 0x18]
1416f1d02 mov      rcx, qword ptr [rdx + rcx*8 + 0x38]
1416f1d07 mov      r9, qword ptr [rbp + 0x300]
1416f1d0e mov      rdx, qword ptr [r9 + 0x110]
1416f1d15 mov      r9, qword ptr [r9 + 0x118]
1416f1d1c mov      rdx, qword ptr [rcx + rdx*8]
1416f1d20 not      rdx
1416f1d23 lea      rdx, [rdx + rdx*2]
1416f1d27 shl      rdx, 4
1416f1d2b mov      r10, qword ptr [r8 + rdx + 0x10]
1416f1d30 mov      qword ptr [rbp + 0x348], r10
1416f1d37 mov      rdx, qword ptr [r8 + rdx + 0x28]
1416f1d3c mov      qword ptr [rbp + 0x310], rdx
1416f1d43 mov      rcx, qword ptr [rcx + r9*8]
1416f1d47 not      rcx
1416f1d4a lea      rcx, [rcx + rcx*2]
1416f1d4e shl      rcx, 4
1416f1d52 mov      rcx, qword ptr [r8 + rcx + 0x10]
1416f1d57 mov      qword ptr [rbp + 0x358], rcx
1416f1d5e mov      rax, qword ptr [rax + 0x50]
1416f1d62 mov      qword ptr [rbp + 0x2e8], rax
1416f1d69 xor      r12d, r12d
1416f1d6c mov      rax, r12
1416f1d6f shl      rax, 4
1416f1d73 mov      rcx, qword ptr [rbp + 0x2e8]
1416f1d7a mov      eax, dword ptr [rcx + rax + 8]
1416f1d7e jmp      0x1416f1b0b
1416f1d83 cmp      qword ptr [rax + 0x38], rdx
1416f1d87 jne      0x1416f1de5
1416f1d89 cmp      byte ptr [rax + 0x19], 4
1416f1d8d jne      0x1416f1de5
1416f1d8f ucomiss  xmm6, xmm9
1416f1d93 jbe      0x1416f1de5
1416f1d95 xorps    xmm0, xmm0
1416f1d98 mov      rcx, qword ptr [rbp + 0x330]
1416f1d9f test     byte ptr [rcx], 1
1416f1da2 xorps    xmm1, xmm1
1416f1da5 je       0x1416f1db8
1416f1da7 mov      rcx, qword ptr [rbp + 0x330]
1416f1dae movss    xmm1, dword ptr [rcx + 4]
1416f1db3 movss    xmm0, dword ptr [rcx + 0xc]
1416f1db8 movss    dword ptr [rsi + 0x10], xmm1
1416f1dbd mov      ecx, dword ptr [rbp + 0x344]
1416f1dc3 mov      r11, qword ptr [rbp + 0x2e0]
1416f1dca mov      dword ptr [r11], ecx
1416f1dcd movss    dword ptr [rsi + 0x18], xmm0
1416f1dd2 mov      cl, 1
1416f1dd4 mov      qword ptr [rbp + 0x2f8], rcx
1416f1ddb cmp      byte ptr [rax + 0x18], 2
1416f1ddf jb       0x1416f1c68
1416f1de5 movzx    ecx, byte ptr [rax + 0x19]
1416f1de9 cmp      cl, 1
1416f1dec ja       0x1416f1c68
1416f1df2 test     cl, 1
1416f1df5 je       0x1416f1c68
1416f1dfb cmp      qword ptr [rax + 0x38], rdx
1416f1dff jne      0x1416f1c68
1416f1e05 mov      rax, qword ptr [r15 + 0x20]
1416f1e09 mov      rcx, qword ptr [rbp + 0x318]
1416f1e10 mov      qword ptr [rcx], rax
1416f1e13 jmp      0x1416f1c68
1416f1e18 mov      rax, qword ptr [rbp + 0x2f0]
1416f1e1f movzx    ebx, byte ptr [rax]
1416f1e22 test     bl, bl
1416f1e24 mov      r13, qword ptr [rbp + 0x328]
1416f1e2b mov      rax, qword ptr [rbp + 0x330]
1416f1e32 je       0x1416f2183
1416f1e38 test     byte ptr [rax], 1
1416f1e3b je       0x1416f1e53
1416f1e3d mov      rcx, qword ptr [rbp + 0x308]
1416f1e44 mov      eax, dword ptr [rcx + 8]
1416f1e47 mov      dword ptr [rbp - 0x48], eax
1416f1e4a mov      rax, qword ptr [rcx]
1416f1e4d mov      qword ptr [rbp - 0x50], rax
1416f1e51 jmp      0x1416f1e62
1416f1e53 mov      dword ptr [rbp - 0x48], 0
1416f1e5a mov      qword ptr [rbp - 0x50], 0
1416f1e62 mov      eax, dword ptr [rbp - 0x48]
1416f1e65 mov      dword ptr [rbp + 0x1f0], eax
1416f1e6b mov      rax, qword ptr [rbp - 0x50]
1416f1e6f mov      qword ptr [rbp + 0x1e8], rax
1416f1e76 mov      qword ptr [rbp + 0x158], 0
1416f1e81 mov      qword ptr [rbp + 0x160], 8
1416f1e8c mov      qword ptr [rbp + 0x168], 0
1416f1e97 mov      dword ptr [rbp + 0x170], 1
1416f1ea1 mov      qword ptr [rbp + 0x178], 0
1416f1eac mov      dword ptr [rbp + 0x188], 2
1416f1eb6 mov      dword ptr [rbp + 0x1a0], 3
1416f1ec0 mov      dword ptr [rbp + 0x1b8], 3
1416f1eca mov      word ptr [rbp + 0x1d0], 0x101
1416f1ed3 mov      byte ptr [rbp + 0x1e0], 0
1416f1eda mov      dword ptr [rbp + 0x1e4], 0x3e800000
1416f1ee4 mov      word ptr [rbp + 0x200], 0x100
1416f1eed lea      rdx, [rip + 0x159e694]
1416f1ef4 lea      r9, [rbp + 0x158]
1416f1efb mov      r8d, 0x10
1416f1f01 mov      rcx, qword ptr [rbp + 0x320]
1416f1f08 call     0x1409236d0
1416f1f0d cmp      byte ptr [rax + 0x18], 2
1416f1f11 jb       0x1416f2183
1416f1f17 movzx    ecx, byte ptr [rax + 0x19]
1416f1f1b cmp      cl, 1
1416f1f1e ja       0x1416f2183
1416f1f24 test     cl, 1
1416f1f27 je       0x1416f2183
1416f1f2d cmp      qword ptr [rax + 0x38], rdx
1416f1f31 jne      0x1416f2183
1416f1f37 mov      rcx, qword ptr [rbp + 0x338]
1416f1f3e mov      rax, qword ptr [rcx]
1416f1f41 mov      rcx, qword ptr [rcx + 8]
1416f1f45 mov      rdx, qword ptr [rax + 0xf8]
1416f1f4c mov      r8, qword ptr [rax + 0x100]
1416f1f53 lea      r8, [rdx + r8*4]
1416f1f57 cmp      byte ptr [rax + 0x108], 0
1416f1f5e je       0x1416f1f7c
1416f1f60 xor      r12d, r12d
1416f1f63 xor      r15d, r15d
1416f1f66 xor      r14d, r14d
1416f1f69 mov      qword ptr [rbp + 0x350], 0
1416f1f74 xor      r10d, r10d
1416f1f77 jmp      0x1416f2041
1416f1f7c mov      r12d, 8
1416f1f82 xor      r13d, r13d
1416f1f85 xor      r15d, r15d
1416f1f88 xor      r14d, r14d
1416f1f8b mov      qword ptr [rbp + 0x350], 0
1416f1f96 xor      r10d, r10d
1416f1f99 jmp      0x1416f2143
1416f1f9e lea      r9, [r9 + r11*8]
1416f1fa2 mov      r11, qword ptr [r9 + 0x18]
1416f1fa6 mov      rdi, qword ptr [r9 + 0x38]
1416f1faa mov      r9, qword ptr [rax + 0x110]
1416f1fb1 mov      rsi, qword ptr [rax + 0x118]
1416f1fb8 mov      r9, qword ptr [rdi + r9*8]
1416f1fbc not      r9
1416f1fbf lea      r14, [r9 + r9*2]
1416f1fc3 shl      r14, 4
1416f1fc7 mov      r9, qword ptr [r11 + r14 + 0x10]
1416f1fcc mov      qword ptr [rbp + 0x350], r9
1416f1fd3 mov      r9, qword ptr [r11 + r14 + 0x28]
1416f1fd8 mov      qword ptr [rbp + 0x358], r9
1416f1fdf mov      rsi, qword ptr [rdi + rsi*8]
1416f1fe3 not      rsi
1416f1fe6 lea      rsi, [rsi + rsi*2]
1416f1fea shl      rsi, 4
1416f1fee mov      r14, qword ptr [r11 + rsi + 0x10]
1416f1ff3 mov      r9, qword ptr [r11 + rsi + 0x28]
1416f1ff8 mov      qword ptr [rbp + 0x348], r9
1416f1fff mov      r15, qword ptr [rax + 0x120]
1416f2006 mov      rdi, qword ptr [rdi + r15*8]
1416f200a not      rdi
1416f200d lea      rdi, [rdi + rdi*2]
1416f2011 shl      rdi, 4
1416f2015 mov      r15, qword ptr [r11 + rdi + 0x10]
1416f201a mov      rdi, qword ptr [r11 + rdi + 0x28]
1416f201f xor      r12d, r12d
1416f2022 mov      r13d, r12d
1416f2025 imul     r11, r13, 0x68
1416f2029 inc      r12
1416f202c mov      rsi, qword ptr [r14 + r11 + 0x20]
1416f2031 mov      r9, qword ptr [rbp + 0x318]
1416f2038 cmp      rsi, qword ptr [r9]
1416f203b je       0x1416f230b
1416f2041 cmp      r12, r10
1416f2044 jne      0x1416f2022
1416f2046 mov      r13, qword ptr [rbp + 0x328]
1416f204d cmp      rdx, r8
1416f2050 je       0x1416f2179
1416f2056 mov      r10d, dword ptr [rdx]
1416f2059 add      rdx, 4
1416f205d mov      r9, qword ptr [rcx + 0x1a8]
1416f2064 lea      r11, [r10 + r10*8]
1416f2068 mov      r10, qword ptr [r9 + r11*8 + 0x10]
1416f206d test     r10, r10
1416f2070 je       0x1416f204d
1416f2072 jmp      0x1416f1f9e
1416f2077 add      r12, r9
1416f207a mov      r9d, dword ptr [r12 + 0x94]
1416f2082 mov      r11, qword ptr [rcx + 0x1a8]
1416f2089 lea      r9, [r9 + r9*8]
1416f208d mov      rsi, qword ptr [r11 + r9*8 + 0x18]
1416f2092 mov      r11, qword ptr [r11 + r9*8 + 0x38]
1416f2097 mov      r9, qword ptr [rax + 0x110]
1416f209e mov      rdi, qword ptr [rax + 0x118]
1416f20a5 mov      r9, qword ptr [r11 + r9*8]
1416f20a9 not      r9
1416f20ac lea      r14, [r9 + r9*2]
1416f20b0 shl      r14, 4
1416f20b4 mov      r9, qword ptr [rsi + r14 + 0x10]
1416f20b9 mov      qword ptr [rbp + 0x350], r9
1416f20c0 mov      r9, qword ptr [rsi + r14 + 0x28]
1416f20c5 mov      qword ptr [rbp + 0x358], r9
1416f20cc mov      rdi, qword ptr [r11 + rdi*8]
1416f20d0 not      rdi
1416f20d3 lea      rdi, [rdi + rdi*2]
1416f20d7 shl      rdi, 4
1416f20db mov      r14, qword ptr [rsi + rdi + 0x10]
1416f20e0 mov      r9, qword ptr [rsi + rdi + 0x28]
1416f20e5 mov      qword ptr [rbp + 0x348], r9
1416f20ec mov      rdi, qword ptr [rax + 0x120]
1416f20f3 mov      r11, qword ptr [r11 + rdi*8]
1416f20f7 not      r11
1416f20fa lea      r11, [r11 + r11*2]
1416f20fe shl      r11, 4
1416f2102 mov      r15, qword ptr [rsi + r11 + 0x10]
1416f2107 mov      r9, qword ptr [rsi + r11 + 0x28]
1416f210c mov      qword ptr [rbp + 0x320], r9
1416f2113 mov      r12, qword ptr [r12 + 0x50]
1416f2118 xor      r13d, r13d
1416f211b mov      r11, r13
1416f211e shl      r11, 4
1416f2122 mov      r11d, dword ptr [r12 + r11 + 8]
1416f2127 imul     rsi, r11, 0x68
1416f212b inc      r13
1416f212e mov      rdi, qword ptr [r14 + rsi + 0x20]
1416f2133 mov      r9, qword ptr [rbp + 0x318]
1416f213a cmp      rdi, qword ptr [r9]
1416f213d je       0x1416f234a
1416f2143 cmp      r13, r10
1416f2146 jne      0x1416f211b
1416f2148 mov      r13, qword ptr [rbp + 0x328]
1416f214f cmp      rdx, r8
1416f2152 je       0x1416f2179
1416f2154 mov      r9d, dword ptr [rdx]
1416f2157 add      rdx, 4
1416f215b mov      r12, qword ptr [rcx + 0x100]
1416f2162 lea      r9, [r9 + r9*4]
1416f2166 shl      r9, 5
1416f216a mov      r10, qword ptr [r12 + r9 + 0x58]
1416f216f test     r10, r10
1416f2172 je       0x1416f214f
1416f2174 jmp      0x1416f2077
1416f2179 mov      rax, qword ptr [rbp + 0x2f0]
1416f2180 mov      byte ptr [rax], 0
1416f2183 xor      eax, eax
1416f2185 cmp      rax, qword ptr [r13]
1416f2189 setno    al
1416f218c test     byte ptr [rbp + 0x2f8], al
1416f2192 je       0x1416f186d
1416f2198 mov      r8, qword ptr [rbp + 0x338]
1416f219f mov      rax, qword ptr [r8]
1416f21a2 mov      rcx, qword ptr [r8 + 8]
1416f21a6 mov      edx, dword ptr [r8 + 0x10]
1416f21aa mov      r8d, dword ptr [r8 + 0x14]
1416f21ae lea      r9, [rcx + 0x1a0]
1416f21b5 lea      r10, [rcx + 0xf8]
1416f21bc mov      r11, qword ptr [rax + 0xf8]
1416f21c3 mov      rsi, qword ptr [rax + 0x100]
1416f21ca lea      rsi, [r11 + rsi*4]
1416f21ce movzx    ebx, byte ptr [rax + 0x108]
1416f21d5 mov      byte ptr [rbp + 0x218], bl
1416f21db mov      qword ptr [rbp + 0x178], r11
1416f21e2 mov      qword ptr [rbp + 0x180], rsi
1416f21e9 mov      qword ptr [rbp + 0x188], 8
1416f21f4 mov      qword ptr [rbp + 0x190], 0
1416f21ff mov      qword ptr [rbp + 0x198], 8
1416f220a xorps    xmm0, xmm0
1416f220d movups   xmmword ptr [rbp + 0x1a0], xmm0
1416f2214 mov      dword ptr [rbp + 0x1c0], edx
1416f221a mov      dword ptr [rbp + 0x1c4], r8d
1416f2221 mov      qword ptr [rbp + 0x1c8], 0
1416f222c mov      dword ptr [rbp + 0x1e0], edx
1416f2232 mov      dword ptr [rbp + 0x1e4], r8d
1416f2239 mov      qword ptr [rbp + 0x1e8], 0
1416f2244 mov      dword ptr [rbp + 0x200], edx
1416f224a mov      dword ptr [rbp + 0x204], r8d
1416f2251 movups   xmmword ptr [rbp + 0x208], xmm0
1416f2258 mov      qword ptr [rbp + 0x158], rcx
1416f225f mov      qword ptr [rbp + 0x160], r9
1416f2266 mov      qword ptr [rbp + 0x168], r10
1416f226d mov      qword ptr [rbp + 0x170], rax
1416f2274 lea      r8, [rip + 0x157bbad]
1416f227b lea      rcx, [rbp - 0x50]
1416f227f lea      rdx, [rbp + 0x158]
1416f2286 call     0x141304bd0
1416f228b mov      rax, qword ptr [r13]
1416f228f test     rax, rax
1416f2292 je       0x1416f22ab
1416f2294 mov      rcx, qword ptr [r13 + 8]
1416f2298 shl      rax, 3
1416f229c lea      rdx, [rax + rax*4]
1416f22a0 mov      r8d, 4
1416f22a6 call     0x140613c20
1416f22ab mov      rax, qword ptr [rbp - 0x40]
1416f22af mov      qword ptr [r13 + 0x10], rax
1416f22b3 movups   xmm0, xmmword ptr [rbp - 0x50]
1416f22b7 movups   xmmword ptr [r13], xmm0
1416f22bc jmp      0x1416f186d
1416f22c1 mov      qword ptr [rsp + 0x20], 1
1416f22ca mov      r8d, 0x18
1416f22d0 mov      r9d, 1
1416f22d6 mov      rcx, rbx
1416f22d9 mov      rdx, r14
1416f22dc call     0x1428a9a60
1416f22e1 jmp      0x1416f1806
1416f22e6 mov      qword ptr [rsp + 0x20], 1
1416f22ef mov      r8d, 0x18
1416f22f5 mov      r9d, 1
1416f22fb mov      rcx, rbx
1416f22fe mov      rdx, r14
1416f2301 call     0x1428a9a60
1416f2306 jmp      0x1416f1853
1416f230b add      r14, r11
1416f230e lea      rax, [r13*2]
1416f2316 add      rax, r13
1416f2319 shl      rax, 4
1416f231d mov      rdx, qword ptr [rbp + 0x350]
1416f2324 add      rdx, rax
1416f2327 mov      rax, qword ptr [rbp + 0x358]
1416f232e lea      rax, [rax + r13*4]
1416f2332 mov      rcx, qword ptr [rbp + 0x348]
1416f2339 lea      rsi, [rcx + r13*4]
1416f233d lea      rdi, [rdi + r13*4]
1416f2341 shl      r13, 4
1416f2345 add      r15, r13
1416f2348 jmp      0x1416f2387
1416f234a add      r14, rsi
1416f234d lea      rax, [r11 + r11*2]
1416f2351 shl      rax, 4
1416f2355 mov      rdx, qword ptr [rbp + 0x350]
1416f235c add      rdx, rax
1416f235f mov      rax, qword ptr [rbp + 0x358]
1416f2366 lea      rax, [rax + r11*4]
1416f236a mov      rcx, qword ptr [rbp + 0x348]
1416f2371 lea      rsi, [rcx + r11*4]
1416f2375 mov      rcx, qword ptr [rbp + 0x320]
1416f237c lea      rdi, [rcx + r11*4]
1416f2380 shl      r11, 4
1416f2384 add      r15, r11
1416f2387 cmp      bl, 2
1416f238a mov      r13, qword ptr [rbp + 0x328]
1416f2391 jne      0x1416f23c6
1416f2393 xorps    xmm0, xmm0
1416f2396 mov      rcx, qword ptr [rbp + 0x330]
1416f239d test     byte ptr [rcx], 1
1416f23a0 xorps    xmm1, xmm1
1416f23a3 je       0x1416f23af
1416f23a5 movss    xmm1, dword ptr [rcx + 4]
1416f23aa movss    xmm0, dword ptr [rcx + 0xc]
1416f23af movss    dword ptr [rdx + 0x10], xmm1
1416f23b4 mov      ecx, dword ptr [rbp + 0x344]
1416f23ba mov      dword ptr [rax], ecx
1416f23bc movss    dword ptr [rdx + 0x18], xmm0
1416f23c1 jmp      0x1416f2448
1416f23c6 mov      rax, qword ptr [rbp + 0x330]
1416f23cd test     byte ptr [rax], 1
1416f23d0 je       0x1416f23ee
1416f23d2 mov      rcx, qword ptr [rbp + 0x308]
1416f23d9 mov      eax, dword ptr [rcx + 8]
1416f23dc mov      dword ptr [rbp + 0x160], eax
1416f23e2 mov      rax, qword ptr [rcx]
1416f23e5 mov      qword ptr [rbp + 0x158], rax
1416f23ec jmp      0x1416f2403
1416f23ee mov      dword ptr [rbp + 0x160], 0
1416f23f8 mov      qword ptr [rbp + 0x158], 0
1416f2403 lea      rcx, [rbp - 0x50]
1416f2407 lea      rdx, [rbp + 0x158]
1416f240e call     0x140c90030
1416f2413 mov      ecx, dword ptr [rbp + 0x344]
1416f2419 mov      dword ptr [rdi], ecx
1416f241b mov      dword ptr [r15], 1
1416f2422 mov      rax, qword ptr [rbp - 0x50]
1416f2426 mov      qword ptr [r15 + 4], rax
1416f242a mov      eax, dword ptr [rbp - 0x48]
1416f242d mov      dword ptr [r15 + 0xc], eax
1416f2431 mov      dword ptr [rsi], ecx
1416f2433 mov      word ptr [r14 + 0x30], 0x101
1416f243a movsd    xmm0, qword ptr [rip + 0x13ca6de]
1416f2442 movsd    qword ptr [r14 + 0x34], xmm0
1416f2448 mov      byte ptr [r13 + 0x48], 0
1416f244d xor      eax, eax
1416f244f cmp      rax, qword ptr [r13]
1416f2453 jo       0x1416f186d
1416f2459 jmp      0x1416f2198
1416f245e mov      ecx, 8
1416f2463 mov      edx, 0x48
1416f2468 call     0x1428d9000
1416f246d ud2      
1416f246f nop      
1416f2470 mov      qword ptr [rsp + 0x10], rdx
1416f2475 push     rbp
1416f2476 push     r15
1416f2478 push     r14
1416f247a push     r13
1416f247c push     r12
1416f247e push     rsi
1416f247f push     rdi
1416f2480 push     rbx
1416f2481 sub      rsp, 0x68
1416f2485 lea      rbp, [rdx + 0x80]
1416f248c movaps   xmmword ptr [rsp + 0x20], xmm9
1416f2492 movaps   xmmword ptr [rsp + 0x30], xmm8
1416f2498 movaps   xmmword ptr [rsp + 0x40], xmm7
1416f249d movaps   xmmword ptr [rsp + 0x50], xmm6
1416f24a2 cmp      qword ptr [rbp + 0x358], 0
1416f24aa je       0x1416f24d3
1416f24ac movaps   xmm6, xmmword ptr [rsp + 0x50]
1416f24b1 movaps   xmm7, xmmword ptr [rsp + 0x40]
1416f24b6 movaps   xmm8, xmmword ptr [rsp + 0x30]
1416f24bc movaps   xmm9, xmmword ptr [rsp + 0x20]
1416f24c2 add      rsp, 0x68
1416f24c6 pop      rbx
1416f24c7 pop      rdi
1416f24c8 pop      rsi
1416f24c9 pop      r12
1416f24cb pop      r13
1416f24cd pop      r14
1416f24cf pop      r15
1416f24d1 pop      rbp
1416f24d2 ret      
1416f24d3 lea      rcx, [rbp + 0x158]
1416f24da call     0x140053a80
1416f24df jmp      0x1416f24ac
