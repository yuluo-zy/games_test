0x140fe12a0 push rbp
0x140fe12a1 push r15
0x140fe12a3 push r14
0x140fe12a5 push r13
0x140fe12a7 push r12
0x140fe12a9 push rsi
0x140fe12aa push rdi
0x140fe12ab push rbx
0x140fe12ac sub rsp, 0x128
0x140fe12b3 lea rbp, [rsp + 0x80]
0x140fe12bb movaps xmmword ptr [rbp + 0x90], xmm15
0x140fe12c3 movaps xmmword ptr [rbp + 0x80], xmm14
0x140fe12cb movaps xmmword ptr [rbp + 0x70], xmm13
0x140fe12d0 movaps xmmword ptr [rbp + 0x60], xmm12
0x140fe12d5 movaps xmmword ptr [rbp + 0x50], xmm11
0x140fe12da movaps xmmword ptr [rbp + 0x40], xmm10
0x140fe12df movaps xmmword ptr [rbp + 0x30], xmm9
0x140fe12e4 movaps xmmword ptr [rbp + 0x20], xmm8
0x140fe12e9 movaps xmmword ptr [rbp + 0x10], xmm7
0x140fe12ed movaps xmmword ptr [rbp], xmm6
0x140fe12f1 mov qword ptr [rbp - 8], 0xfffffffffffffffe
0x140fe12f9 mov r13, qword ptr [rbp + 0x110]
0x140fe1300 mov r15, r13
0x140fe1303 shl r15, 4
0x140fe1307 xor r12d, r12d
0x140fe130a mov rax, r13
0x140fe130d shr rax, 0x3c
0x140fe1311 jne 0x140fe189f
0x140fe1317 movabs rax, 0x7ffffffffffffffc
0x140fe1321 cmp r15, rax
0x140fe1324 ja 0x140fe189f
0x140fe132a mov rdi, r9
0x140fe132d mov r14, r8
0x140fe1330 mov rbx, rdx
0x140fe1333 mov rsi, rcx
0x140fe1336 test r15, r15
0x140fe1339 je 0x140fe1363
0x140fe133b movzx eax, byte ptr [rip + 0x26538c7]
0x140fe1342 mov r12d, 4
0x140fe1348 mov edx, 4
0x140fe134d mov rcx, r15
0x140fe1350 call 0x140613c10
0x140fe1355 test rax, rax
0x140fe1358 je 0x140fe189f
0x140fe135e mov rcx, r13
0x140fe1361 jmp 0x140fe136a
0x140fe1363 mov eax, 4
0x140fe1368 xor ecx, ecx
0x140fe136a mov qword ptr [rbp - 0x30], rcx
0x140fe136e mov qword ptr [rbp - 0x28], rax
0x140fe1372 mov qword ptr [rbp - 0x20], 0
0x140fe137a movss xmm11, dword ptr [r14]
0x140fe137f movss xmm9, dword ptr [rbx + 4]
0x140fe1385 cmp byte ptr [rbx], 0
0x140fe1388 jne 0x140fe1514
0x140fe138e movss xmm13, dword ptr [r14 + 4]
0x140fe1394 movss xmm15, dword ptr [rbx + 0xc]
0x140fe139a cmp byte ptr [rbx + 8], 0
0x140fe139e jne 0x140fe152f
0x140fe13a4 movss xmm1, dword ptr [rbx + 0x14]
0x140fe13a9 cmp byte ptr [rbx + 0x10], 0
0x140fe13ad jne 0x140fe1543
0x140fe13b3 movss xmm0, dword ptr [rbx + 0x1c]
0x140fe13b8 cmp byte ptr [rbx + 0x18], 0
0x140fe13bc je 0x140fe13c3
0x140fe13be mulss xmm0, xmm13
0x140fe13c3 addss xmm1, xmm9
0x140fe13c8 movaps xmm10, xmm11
0x140fe13cc subss xmm10, xmm1
0x140fe13d1 movss xmm14, dword ptr [rbx + 0x24]
0x140fe13d7 cmp byte ptr [rbx + 0x20], 0
0x140fe13db je 0x140fe13e2
0x140fe13dd mulss xmm14, xmm10
0x140fe13e2 xor eax, eax
0x140fe13e4 mov rcx, r13
0x140fe13e7 sub rcx, 1
0x140fe13eb cmovae rax, rcx
0x140fe13ef cvtsi2ss xmm3, rax
0x140fe13f4 mulss xmm3, xmm14
0x140fe13f9 subss xmm10, xmm3
0x140fe13fe xorps xmm2, xmm2
0x140fe1401 xorps xmm4, xmm4
0x140fe1404 xorps xmm12, xmm12
0x140fe1408 test r13, r13
0x140fe140b je 0x140fe14a6
0x140fe1411 lea rax, [r13*2]
0x140fe1419 add rax, r13
0x140fe141c lea rax, [rdi + rax*8]
0x140fe1420 mov rcx, rdi
0x140fe1423 jmp 0x140fe147e
0x140fe1425 nop word ptr cs:[rax + rax]
0x140fe1430 addss xmm12, dword ptr [rdx + 0x38]
0x140fe1436 movaps xmm7, xmm5
0x140fe1439 cmpunordss xmm7, xmm5
0x140fe143e movaps xmm8, xmm7
0x140fe1442 andps xmm8, xmm6
0x140fe1446 maxss xmm6, xmm5
0x140fe144a andnps xmm7, xmm6
0x140fe144d orps xmm7, xmm8
0x140fe1451 movss xmm5, dword ptr [rcx + 0x10]
0x140fe1456 movaps xmm6, xmm5
0x140fe1459 maxss xmm6, xmm7
0x140fe145d cmpunordss xmm7, xmm7
0x140fe1462 movaps xmm8, xmm7
0x140fe1466 andnps xmm8, xmm6
0x140fe146a andps xmm7, xmm5
0x140fe146d orps xmm7, xmm8
0x140fe1471 addss xmm4, xmm7
0x140fe1475 add rcx, 0x18
0x140fe1479 cmp rcx, rax
0x140fe147c je 0x140fe14a6
0x140fe147e mov rdx, qword ptr [rcx + 8]
0x140fe1482 movss xmm5, dword ptr [rdx + 4]
0x140fe1487 cmp byte ptr [rdx], 0
0x140fe148a je 0x140fe1491
0x140fe148c mulss xmm5, xmm10
0x140fe1491 mov rdx, qword ptr [rcx]
0x140fe1494 movss xmm6, dword ptr [rdx + 0x2c]
0x140fe1499 cmp byte ptr [rdx + 0x28], 0
0x140fe149d je 0x140fe1430
0x140fe149f mulss xmm6, xmm10
0x140fe14a4 jmp 0x140fe1430
0x140fe14a6 addss xmm3, xmm4
0x140fe14aa subss xmm11, xmm3
0x140fe14af subss xmm11, xmm1
0x140fe14b4 movss xmm1, dword ptr [rip + 0x1944448]
0x140fe14bc movaps xmm3, xmm1
0x140fe14bf subss xmm3, xmm12
0x140fe14c4 maxss xmm2, xmm3
0x140fe14c8 minss xmm1, xmm2
0x140fe14cc mulss xmm1, xmm11
0x140fe14d1 movzx eax, byte ptr [rbx + 0x46]
0x140fe14d5 lea rcx, [rip + 0x1be0d60]
0x140fe14dc movsxd rax, dword ptr [rcx + rax*4]
0x140fe14e0 add rax, rcx
0x140fe14e3 mov qword ptr [rbp - 0x38], rsi
0x140fe14e7 movss dword ptr [rbp - 0xc], xmm10
0x140fe14ed jmp rax
0x140fe14ef cmp r13, 1
0x140fe14f3 jbe 0x140fe155c
0x140fe14f5 lea rax, [r13 - 1]
0x140fe14f9 xorps xmm2, xmm2
0x140fe14fc cvtsi2ss xmm2, rax
0x140fe1501 divss xmm1, xmm2
0x140fe1505 addss xmm14, xmm1
0x140fe150a test r13, r13
0x140fe150d jne 0x140fe1572
0x140fe150f jmp 0x140fe1842
0x140fe1514 mulss xmm9, xmm11
0x140fe1519 movss xmm13, dword ptr [r14 + 4]
0x140fe151f movss xmm15, dword ptr [rbx + 0xc]
0x140fe1525 cmp byte ptr [rbx + 8], 0
0x140fe1529 je 0x140fe13a4
0x140fe152f mulss xmm15, xmm13
0x140fe1534 movss xmm1, dword ptr [rbx + 0x14]
0x140fe1539 cmp byte ptr [rbx + 0x10], 0
0x140fe153d je 0x140fe13b3
0x140fe1543 mulss xmm1, xmm11
0x140fe1548 movss xmm0, dword ptr [rbx + 0x1c]
0x140fe154d cmp byte ptr [rbx + 0x18], 0
0x140fe1551 jne 0x140fe13be
0x140fe1557 jmp 0x140fe13c3
0x140fe155c mulss xmm1, dword ptr [rip + 0x194430c]
0x140fe1564 addss xmm9, xmm1
0x140fe1569 test r13, r13
0x140fe156c je 0x140fe1842
0x140fe1572 addss xmm0, xmm15
0x140fe1577 subss xmm13, xmm0
0x140fe157c maxss xmm12, dword ptr [rip + 0x1a9a51b]
0x140fe1585 xorps xmm0, xmm0
0x140fe1588 maxss xmm11, xmm0
0x140fe158d movss dword ptr [rbp - 0x14], xmm11
0x140fe1593 movzx eax, byte ptr [rbx + 0x45]
0x140fe1597 movsd xmm0, qword ptr [r14 + 8]
0x140fe159d movaps xmmword ptr [rbp - 0x60], xmm0
0x140fe15a1 movsd xmm0, qword ptr [rbx + 0x3c]
0x140fe15a6 movaps xmmword ptr [rbp - 0x50], xmm0
0x140fe15aa shl r13, 3
0x140fe15ae lea r15, [r13*2]
0x140fe15b6 add r15, r13
0x140fe15b9 mov r12d, 8
0x140fe15bf xor r13d, r13d
0x140fe15c2 mov eax, eax
0x140fe15c4 lea rcx, [rip + 0x1be0c85]
0x140fe15cb movsxd rsi, dword ptr [rcx + rax*4]
0x140fe15cf add rsi, rcx
0x140fe15d2 lea r14, [rbp - 0x30]
0x140fe15d6 xor ebx, ebx
0x140fe15d8 movss dword ptr [rbp - 0x10], xmm13
0x140fe15de jmp 0x140fe1696
0x140fe15e3 nop word ptr cs:[rax + rax]
0x140fe15f0 movaps xmm0, xmm15
0x140fe15f4 cmpunordss xmm0, xmm15
0x140fe15fa movaps xmm1, xmm0
0x140fe15fd andps xmm1, xmm11
0x140fe1601 maxss xmm11, xmm15
0x140fe1606 andnps xmm0, xmm11
0x140fe160a orps xmm0, xmm1
0x140fe160d movaps xmm1, xmm13
0x140fe1611 maxss xmm1, xmm0
0x140fe1615 cmpunordss xmm0, xmm0
0x140fe161a movaps xmm2, xmm0
0x140fe161d andnps xmm2, xmm1
0x140fe1620 andps xmm0, xmm13
0x140fe1624 orps xmm0, xmm2
0x140fe1627 divss xmm6, xmm12
0x140fe162c mulss xmm6, dword ptr [rbp - 0x14]
0x140fe1631 addss xmm6, xmm0
0x140fe1635 movaps xmm15, xmm14
0x140fe1639 addss xmm8, xmm14
0x140fe163e movaps xmm14, xmm10
0x140fe1642 movaps xmm0, xmm10
0x140fe1646 addss xmm0, xmm9
0x140fe164b unpcklps xmm9, xmm8
0x140fe164f addps xmm9, xmmword ptr [rbp - 0x60]
0x140fe1654 addps xmm9, xmmword ptr [rbp - 0x50]
0x140fe1659 mov rax, qword ptr [rbp - 0x28]
0x140fe165d movss dword ptr [rax + r12 - 8], xmm6
0x140fe1664 movss dword ptr [rax + r12 - 4], xmm7
0x140fe166b movlps qword ptr [rax + r12], xmm9
0x140fe1670 inc rbx
0x140fe1673 mov qword ptr [rbp - 0x20], rbx
0x140fe1677 addss xmm0, xmm6
0x140fe167b add r12, 0x10
0x140fe167f add r13, 0x18
0x140fe1683 movaps xmm9, xmm0
0x140fe1687 cmp r15, r13
0x140fe168a movss xmm13, dword ptr [rbp - 0x10]
0x140fe1690 je 0x140fe1842
0x140fe1696 movaps xmm10, xmm14
0x140fe169a movaps xmm14, xmm15
0x140fe169e mov rax, qword ptr [rdi + r13 + 8]
0x140fe16a3 movss xmm15, dword ptr [rax + 4]
0x140fe16a9 cmp byte ptr [rax], 0
0x140fe16ac jne 0x140fe1750
0x140fe16b2 movss xmm0, dword ptr [rax + 0xc]
0x140fe16b7 cmp byte ptr [rax + 8], 0
0x140fe16bb jne 0x140fe1765
0x140fe16c1 mov rax, qword ptr [rdi + r13]
0x140fe16c5 movss xmm11, dword ptr [rax + 0x2c]
0x140fe16cb cmp byte ptr [rax + 0x28], 0
0x140fe16cf jne 0x140fe177e
0x140fe16d5 movss xmm1, dword ptr [rax + 0x34]
0x140fe16da cmp byte ptr [rax + 0x30], 0
0x140fe16de je 0x140fe16e5
0x140fe16e0 mulss xmm1, xmm13
0x140fe16e5 movaps xmm7, xmm0
0x140fe16e8 cmpunordss xmm7, xmm0
0x140fe16ed movaps xmm2, xmm7
0x140fe16f0 andps xmm2, xmm1
0x140fe16f3 maxss xmm1, xmm0
0x140fe16f7 andnps xmm7, xmm1
0x140fe16fa orps xmm7, xmm2
0x140fe16fd movss xmm0, dword ptr [rdi + r13 + 0x14]
0x140fe1704 movaps xmm1, xmm0
0x140fe1707 maxss xmm1, xmm7
0x140fe170b cmpunordss xmm7, xmm7
0x140fe1710 movaps xmm2, xmm7
0x140fe1713 andnps xmm2, xmm1
0x140fe1716 andps xmm7, xmm0
0x140fe1719 orps xmm7, xmm2
0x140fe171c xorps xmm8, xmm8
0x140fe1720 addss xmm7, xmm8
0x140fe1725 jmp rsi
0x140fe1727 movaps xmm7, xmm13
0x140fe172b movss xmm13, dword ptr [rdi + r13 + 0x10]
0x140fe1732 movss xmm6, dword ptr [rax + 0x38]
0x140fe1737 cmp rbx, qword ptr [rbp - 0x30]
0x140fe173b jne 0x140fe15f0
0x140fe1741 jmp 0x140fe17c8
0x140fe1746 nop word ptr cs:[rax + rax]
0x140fe1750 mulss xmm15, dword ptr [rbp - 0xc]
0x140fe1756 movss xmm0, dword ptr [rax + 0xc]
0x140fe175b cmp byte ptr [rax + 8], 0
0x140fe175f je 0x140fe16c1
0x140fe1765 mulss xmm0, xmm13
0x140fe176a mov rax, qword ptr [rdi + r13]
0x140fe176e movss xmm11, dword ptr [rax + 0x2c]
0x140fe1774 cmp byte ptr [rax + 0x28], 0
0x140fe1778 je 0x140fe16d5
0x140fe177e mulss xmm11, dword ptr [rbp - 0xc]
0x140fe1784 movss xmm1, dword ptr [rax + 0x34]
0x140fe1789 cmp byte ptr [rax + 0x30], 0
0x140fe178d jne 0x140fe16e0
0x140fe1793 jmp 0x140fe16e5
0x140fe1798 nop dword ptr [rax + rax]
0x140fe17a0 movaps xmm8, xmm13
0x140fe17a4 subss xmm8, xmm7
0x140fe17a9 mulss xmm8, dword ptr [rip + 0x19440be]
0x140fe17b2 movss xmm13, dword ptr [rdi + r13 + 0x10]
0x140fe17b9 movss xmm6, dword ptr [rax + 0x38]
0x140fe17be cmp rbx, qword ptr [rbp - 0x30]
0x140fe17c2 jne 0x140fe15f0
0x140fe17c8 mov rcx, r14
0x140fe17cb lea rdx, [rip + 0x1be075e]
0x140fe17d2 call 0x14068d3d0
0x140fe17d7 jmp 0x140fe15f0
0x140fe17dc nop dword ptr [rax]
0x140fe17e0 movaps xmm8, xmm13
0x140fe17e4 subss xmm8, xmm7
0x140fe17e9 movss xmm13, dword ptr [rdi + r13 + 0x10]
0x140fe17f0 movss xmm6, dword ptr [rax + 0x38]
0x140fe17f5 cmp rbx, qword ptr [rbp - 0x30]
0x140fe17f9 jne 0x140fe15f0
0x140fe17ff jmp 0x140fe17c8
0x140fe1801 mov rax, r13
0x140fe1804 inc rax
0x140fe1807 js 0x140fe1813
0x140fe1809 xorps xmm2, xmm2
0x140fe180c cvtsi2ss xmm2, rax
0x140fe1811 jmp 0x140fe182b
0x140fe1813 mov rcx, rax
0x140fe1816 shr rcx, 1
0x140fe1819 and eax, 1
0x140fe181c or rax, rcx
0x140fe181f xorps xmm2, xmm2
0x140fe1822 cvtsi2ss xmm2, rax
0x140fe1827 addss xmm2, xmm2
0x140fe182b divss xmm1, xmm2
0x140fe182f addss xmm14, xmm1
0x140fe1834 addss xmm9, xmm1
0x140fe1839 test r13, r13
0x140fe183c jne 0x140fe1572
0x140fe1842 mov rcx, qword ptr [rbp - 0x20]
0x140fe1846 mov rax, qword ptr [rbp - 0x38]
0x140fe184a mov qword ptr [rax + 0x10], rcx
0x140fe184e movups xmm0, xmmword ptr [rbp - 0x30]
0x140fe1852 movups xmmword ptr [rax], xmm0
0x140fe1855 movaps xmm6, xmmword ptr [rbp]
0x140fe1859 movaps xmm7, xmmword ptr [rbp + 0x10]
0x140fe185d movaps xmm8, xmmword ptr [rbp + 0x20]
0x140fe1862 movaps xmm9, xmmword ptr [rbp + 0x30]
0x140fe1867 movaps xmm10, xmmword ptr [rbp + 0x40]
0x140fe186c movaps xmm11, xmmword ptr [rbp + 0x50]
0x140fe1871 movaps xmm12, xmmword ptr [rbp + 0x60]
0x140fe1876 movaps xmm13, xmmword ptr [rbp + 0x70]
0x140fe187b movaps xmm14, xmmword ptr [rbp + 0x80]
0x140fe1883 movaps xmm15, xmmword ptr [rbp + 0x90]
0x140fe188b add rsp, 0x128
0x140fe1892 pop rbx
0x140fe1893 pop rdi
0x140fe1894 pop rsi
0x140fe1895 pop r12
0x140fe1897 pop r13
0x140fe1899 pop r14
0x140fe189b pop r15
0x140fe189d pop rbp
0x140fe189e ret 
0x140fe189f lea r8, [rip + 0x1be0672]
0x140fe18a6 mov rcx, r12
0x140fe18a9 mov rdx, r15
0x140fe18ac call 0x1428d8fe3
0x140fe18b1 int3 
0x140fe18b2 nop word ptr cs:[rax + rax]
0x140fe18c0 mov qword ptr [rsp + 0x10], rdx
0x140fe18c5 push rbp
0x140fe18c6 push r15
0x140fe18c8 push r14
0x140fe18ca push r13
0x140fe18cc push r12
0x140fe18ce push rsi
0x140fe18cf push rdi
0x140fe18d0 push rbx
0x140fe18d1 sub rsp, 0xc8
0x140fe18d8 lea rbp, [rdx + 0x80]
0x140fe18df movaps xmmword ptr [rsp + 0x20], xmm15
0x140fe18e5 movaps xmmword ptr [rsp + 0x30], xmm14
0x140fe18eb movaps xmmword ptr [rsp + 0x40], xmm13
0x140fe18f1 movaps xmmword ptr [rsp + 0x50], xmm12
0x140fe18f7 movaps xmmword ptr [rsp + 0x60], xmm11
0x140fe18fd movaps xmmword ptr [rsp + 0x70], xmm10
0x140fe1903 movaps xmmword ptr [rsp + 0x80], xmm9
0x140fe190c movaps xmmword ptr [rsp + 0x90], xmm8
0x140fe1915 movaps xmmword ptr [rsp + 0xa0], xmm7
0x140fe191d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140fe1925 mov rdx, qword ptr [rbp - 0x30]
0x140fe1929 test rdx, rdx
0x140fe192c je 0x140fe1941
0x140fe192e mov rcx, qword ptr [rbp - 0x28]
0x140fe1932 shl rdx, 4
0x140fe1936 mov r8d, 4
0x140fe193c call 0x140613c20
0x140fe1941 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140fe1949 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140fe1951 movaps xmm8, xmmword ptr [rsp + 0x90]
0x140fe195a movaps xmm9, xmmword ptr [rsp + 0x80]
0x140fe1963 movaps xmm10, xmmword ptr [rsp + 0x70]
0x140fe1969 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140fe196f movaps xmm12, xmmword ptr [rsp + 0x50]
0x140fe1975 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140fe197b movaps xmm14, xmmword ptr [rsp + 0x30]
0x140fe1981 movaps xmm15, xmmword ptr [rsp + 0x20]
0x140fe1987 add rsp, 0xc8
0x140fe198e pop rbx
0x140fe198f pop rdi
0x140fe1990 pop rsi
0x140fe1991 pop r12
0x140fe1993 pop r13
0x140fe1995 pop r14
0x140fe1997 pop r15
0x140fe1999 pop rbp
0x140fe199a ret 
