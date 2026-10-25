1407e1430: 55 push rbp
1407e1431: 4157 push r15
1407e1433: 4156 push r14
1407e1435: 4155 push r13
1407e1437: 4154 push r12
1407e1439: 56 push rsi
1407e143a: 57 push rdi
1407e143b: 53 push rbx
1407e143c: 4881ecc8010000 sub rsp, 0x1c8
1407e1443: 488dac2480000000 lea rbp, [rsp + 0x80]
1407e144b: 440f29a530010000 movaps xmmword ptr [rbp + 0x130], xmm12
1407e1453: 440f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm11
1407e145b: 440f299510010000 movaps xmmword ptr [rbp + 0x110], xmm10
1407e1463: 440f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm9
1407e146b: 440f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm8
1407e1473: 0f29bde0000000 movaps xmmword ptr [rbp + 0xe0], xmm7
1407e147a: 0f29b5d0000000 movaps xmmword ptr [rbp + 0xd0], xmm6
1407e1481: 48c785c8000000feffffff mov qword ptr [rbp + 0xc8], 0xfffffffffffffffe
1407e148c: 4c894de0 mov qword ptr [rbp - 0x20], r9
1407e1490: 4c8985b0000000 mov qword ptr [rbp + 0xb0], r8
1407e1497: 488995a8000000 mov qword ptr [rbp + 0xa8], rdx
1407e149e: 4989ce mov r14, rcx
1407e14a1: e85a554300 call 0x140c16a00
1407e14a6: 84c0 test al, al
1407e14a8: 8885c7000000 mov byte ptr [rbp + 0xc7], al
1407e14ae: 745f je 0x1407e150f
1407e14b0: 4c8d3deb742a02 lea r15, [rip + 0x22a74eb]
1407e14b7: be46000000 mov esi, 0x46
1407e14bc: ba46000000 mov edx, 0x46
1407e14c1: 4c89f9 mov rcx, r15
1407e14c4: e887dbffff call 0x1407df050
1407e14c9: 4883f801 cmp rax, 1
1407e14cd: 0f85b1000000 jne 0x1407e1584
1407e14d3: 4989d1 mov r9, rdx
1407e14d6: 4885d2 test rdx, rdx
1407e14d9: 743d je 0x1407e1518
1407e14db: 4983f946 cmp r9, 0x46
1407e14df: 7335 jae 0x1407e1516
1407e14e1: 488d05ba742a02 lea rax, [rip + 0x22a74ba]
1407e14e8: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1407e14ed: 7f29 jg 0x1407e1518
1407e14ef: 488d057a752a02 lea rax, [rip + 0x22a757a]
1407e14f6: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e14fb: 488d0da0742a02 lea rcx, [rip + 0x22a74a0]
1407e1502: ba46000000 mov edx, 0x46
1407e1507: 4531c0 xor r8d, r8d
1407e150a: e831840f02 call 0x1428d9940
1407e150f: 31c0 xor eax, eax
1407e1511: e9cc010000 jmp 0x1407e16e2
1407e1516: 75d7 jne 0x1407e14ef
1407e1518: 4c8d3d83742a02 lea r15, [rip + 0x22a7483]
1407e151f: 4c89f9 mov rcx, r15
1407e1522: 4c89ca mov rdx, r9
1407e1525: e826dbffff call 0x1407df050
1407e152a: 4883f801 cmp rax, 1
1407e152e: 7554 jne 0x1407e1584
1407e1530: 4989d0 mov r8, rdx
1407e1533: 4983c002 add r8, 2
1407e1537: 7439 je 0x1407e1572
1407e1539: 4983f846 cmp r8, 0x46
1407e153d: 7331 jae 0x1407e1570
1407e153f: 488d055c742a02 lea rax, [rip + 0x22a745c]
1407e1546: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
1407e154b: 7f25 jg 0x1407e1572
1407e154d: 488d0534752a02 lea rax, [rip + 0x22a7534]
1407e1554: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e1559: 488d0d42742a02 lea rcx, [rip + 0x22a7442]
1407e1560: ba46000000 mov edx, 0x46
1407e1565: 41b946000000 mov r9d, 0x46
1407e156b: e8d0830f02 call 0x1428d9940
1407e1570: 75db jne 0x1407e154d
1407e1572: be44000000 mov esi, 0x44
1407e1577: 4829d6 sub rsi, rdx
1407e157a: 4c8d3d21742a02 lea r15, [rip + 0x22a7421]
1407e1581: 4d01c7 add r15, r8
1407e1584: 4c8d056e752a02 lea r8, [rip + 0x22a756e]
1407e158b: 4c8d2526752a02 lea r12, [rip + 0x22a7526]
1407e1592: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e15a0: 410fbe40ff movsx eax, byte ptr [r8 - 1]
1407e15a5: 85c0 test eax, eax
1407e15a7: 7817 js 0x1407e15c0
1407e15a9: 49ffc8 dec r8
1407e15ac: 83f85c cmp eax, 0x5c
1407e15af: 7563 jne 0x1407e1614
1407e15b1: eb72 jmp 0x1407e1625
1407e15b3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e15c0: 410fb648fe movzx ecx, byte ptr [r8 - 2]
1407e15c5: 80f9c0 cmp cl, 0xc0
1407e15c8: 7d23 jge 0x1407e15ed
1407e15ca: 410fb650fd movzx edx, byte ptr [r8 - 3]
1407e15cf: 80fac0 cmp dl, 0xc0
1407e15d2: 7d22 jge 0x1407e15f6
1407e15d4: 450fb648fc movzx r9d, byte ptr [r8 - 4]
1407e15d9: 4983c0fc add r8, -4
1407e15dd: 4183e107 and r9d, 7
1407e15e1: 41c1e106 shl r9d, 6
1407e15e5: 83e23f and edx, 0x3f
1407e15e8: 4409ca or edx, r9d
1407e15eb: eb10 jmp 0x1407e15fd
1407e15ed: 4983c0fe add r8, -2
1407e15f1: 83e11f and ecx, 0x1f
1407e15f4: eb0f jmp 0x1407e1605
1407e15f6: 4983c0fd add r8, -3
1407e15fa: 83e20f and edx, 0xf
1407e15fd: c1e206 shl edx, 6
1407e1600: 83e13f and ecx, 0x3f
1407e1603: 09d1 or ecx, edx
1407e1605: c1e106 shl ecx, 6
1407e1608: 243f and al, 0x3f
1407e160a: 0fb6c0 movzx eax, al
1407e160d: 09c8 or eax, ecx
1407e160f: 83f85c cmp eax, 0x5c
1407e1612: 7411 je 0x1407e1625
1407e1614: 83f82f cmp eax, 0x2f
1407e1617: 740c je 0x1407e1625
1407e1619: 4d39e0 cmp r8, r12
1407e161c: 7582 jne 0x1407e15a0
1407e161e: bf41000000 mov edi, 0x41
1407e1623: eb4c jmp 0x1407e1671
1407e1625: 4c8d258c742a02 lea r12, [rip + 0x22a748c]
1407e162c: 4d29e0 sub r8, r12
1407e162f: 49ffc0 inc r8
1407e1632: 7432 je 0x1407e1666
1407e1634: 4983f841 cmp r8, 0x41
1407e1638: 732a jae 0x1407e1664
1407e163a: 43803c20bf cmp byte ptr [r8 + r12], 0xbf
1407e163f: 7f25 jg 0x1407e1666
1407e1641: 488d0508742a02 lea rax, [rip + 0x22a7408]
1407e1648: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e164d: 488d0d64742a02 lea rcx, [rip + 0x22a7464]
1407e1654: ba41000000 mov edx, 0x41
1407e1659: 41b941000000 mov r9d, 0x41
1407e165f: e8dc820f02 call 0x1428d9940
1407e1664: 75db jne 0x1407e1641
1407e1666: bf41000000 mov edi, 0x41
1407e166b: 4c29c7 sub rdi, r8
1407e166e: 4d01c4 add r12, r8
1407e1671: e8ba6a4300 call 0x140c18130
1407e1676: 4889c1 mov rcx, rax
1407e1679: 488b00 mov rax, qword ptr [rax]
1407e167c: 4883f801 cmp rax, 1
1407e1680: 0f8509090000 jne 0x1407e1f8f
1407e1686: 4883c108 add rcx, 8
1407e168a: 48833900 cmp qword ptr [rcx], 0
1407e168e: 0f8583090000 jne 0x1407e2017
1407e1694: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407e169b: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
1407e16a2: 4883c108 add rcx, 8
1407e16a6: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1407e16ab: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1407e16b4: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1407e16bd: 4c89fa mov rdx, r15
1407e16c0: 4989f0 mov r8, rsi
1407e16c3: 4d89e1 mov r9, r12
1407e16c6: e875564300 call 0x140c16d40
1407e16cb: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
1407e16d2: 48ff01 inc qword ptr [rcx]
1407e16d5: 48894518 mov qword ptr [rbp + 0x18], rax
1407e16d9: 488945c8 mov qword ptr [rbp - 0x38], rax
1407e16dd: b801000000 mov eax, 1
1407e16e2: 488bb5b0000000 mov rsi, qword ptr [rbp + 0xb0]
1407e16e9: 488945c0 mov qword ptr [rbp - 0x40], rax
1407e16ed: 488d8580000000 lea rax, [rbp + 0x80]
1407e16f4: 48894520 mov qword ptr [rbp + 0x20], rax
1407e16f8: 4883fe02 cmp rsi, 2
1407e16fc: 0f83f1080000 jae 0x1407e1ff3
1407e1702: 48c7452000000000 mov qword ptr [rbp + 0x20], 0
1407e170a: 48c7452808000000 mov qword ptr [rbp + 0x28], 8
1407e1712: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
1407e171a: 4c8d0daf742a02 lea r9, [rip + 0x22a74af]
1407e1721: 488d4df8 lea rcx, [rbp - 8]
1407e1725: 488d5520 lea rdx, [rbp + 0x20]
1407e1729: 4989f0 mov r8, rsi
1407e172c: e8efd2a700 call 0x14125ea20
1407e1731: 4c8975e8 mov qword ptr [rbp - 0x18], r14
1407e1735: f30f10b5d0010000 movss xmm6, dword ptr [rbp + 0x1d0]
1407e173d: f30f10bdc8010000 movss xmm7, dword ptr [rbp + 0x1c8]
1407e1745: f3440f1085c0010000 movss xmm8, dword ptr [rbp + 0x1c0]
1407e174e: 0fb6bdb8010000 movzx edi, byte ptr [rbp + 0x1b8]
1407e1755: f3450f59c0 mulss xmm8, xmm8
1407e175a: f30f59ff mulss xmm7, xmm7
1407e175e: f30f59f6 mulss xmm6, xmm6
1407e1762: 4c8d14b6 lea r10, [rsi + rsi*4]
1407e1766: 49c1e204 shl r10, 4
1407e176a: 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
1407e1771: 4901c2 add r10, rax
1407e1774: 31d2 xor edx, edx
1407e1776: f3440f100df9b81e02 movss xmm9, dword ptr [rip + 0x21eb8f9]
1407e177f: 4c895570 mov qword ptr [rbp + 0x70], r10
1407e1783: eb19 jmp 0x1407e179e
1407e1785: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e1790: 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
1407e1797: 488b85a0000000 mov rax, qword ptr [rbp + 0xa0]
1407e179e: 4c39d0 cmp rax, r10
1407e17a1: 0f84cc010000 je 0x1407e1973
1407e17a7: 4989c5 mov r13, rax
1407e17aa: 4989d3 mov r11, rdx
1407e17ad: 4883c050 add rax, 0x50
1407e17b1: 48ffc2 inc rdx
1407e17b4: 41807d4d00 cmp byte ptr [r13 + 0x4d], 0
1407e17b9: 74e3 je 0x1407e179e
1407e17bb: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
1407e17c2: 4b8d045b lea rax, [r11 + r11*2]
1407e17c6: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
1407e17cd: 4c8bb5a8000000 mov r14, qword ptr [rbp + 0xa8]
1407e17d4: 48899598000000 mov qword ptr [rbp + 0x98], rdx
1407e17db: 4531ff xor r15d, r15d
1407e17de: 4c895d68 mov qword ptr [rbp + 0x68], r11
1407e17e2: eb2c jmp 0x1407e1810
1407e17e4: 488b4308 mov rax, qword ptr [rbx + 8]
1407e17e8: 4c89e1 mov rcx, r12
1407e17eb: 48c1e104 shl rcx, 4
1407e17ef: 48893408 mov qword ptr [rax + rcx], rsi
1407e17f3: f3440f11540808 movss dword ptr [rax + rcx + 8], xmm10
1407e17fa: 49ffc4 inc r12
1407e17fd: 4c896310 mov qword ptr [rbx + 0x10], r12
1407e1801: 31d2 xor edx, edx
1407e1803: 4c8b5570 mov r10, qword ptr [rbp + 0x70]
1407e1807: 4c8b5d68 mov r11, qword ptr [rbp + 0x68]
1407e180b: 0f1f440000 nop dword ptr [rax + rax]
1407e1810: 4885d2 test rdx, rdx
1407e1813: 0f8520010000 jne 0x1407e1939
1407e1819: 4d89f0 mov r8, r14
1407e181c: 4c89fe mov rsi, r15
1407e181f: 4d39d6 cmp r14, r10
1407e1822: 0f8468ffffff je 0x1407e1790
1407e1828: 4d8d7050 lea r14, [r8 + 0x50]
1407e182c: 4c8d7e01 lea r15, [rsi + 1]
1407e1830: 4180784d01 cmp byte ptr [r8 + 0x4d], 1
1407e1835: ba00000000 mov edx, 0
1407e183a: 75d4 jne 0x1407e1810
1407e183c: 498b4030 mov rax, qword ptr [r8 + 0x30]
1407e1840: 498b4d30 mov rcx, qword ptr [r13 + 0x30]
1407e1844: 4084ff test dil, dil
1407e1847: 750a jne 0x1407e1853
1407e1849: ba00000000 mov edx, 0
1407e184e: 4839c8 cmp rax, rcx
1407e1851: 74bd je 0x1407e1810
1407e1853: f3410f104d14 movss xmm1, dword ptr [r13 + 0x14]
1407e1859: f3410f5c4814 subss xmm1, dword ptr [r8 + 0x14]
1407e185f: f30f59c9 mulss xmm1, xmm1
1407e1863: f3410f104518 movss xmm0, dword ptr [r13 + 0x18]
1407e1869: f3410f105510 movss xmm2, dword ptr [r13 + 0x10]
1407e186f: 0f14d0 unpcklps xmm2, xmm0
1407e1872: f3410f104010 movss xmm0, dword ptr [r8 + 0x10]
1407e1878: f3410f105818 movss xmm3, dword ptr [r8 + 0x18]
1407e187e: 0f14c3 unpcklps xmm0, xmm3
1407e1881: 0f5cd0 subps xmm2, xmm0
1407e1884: 0f59d2 mulps xmm2, xmm2
1407e1887: f30f58ca addss xmm1, xmm2
1407e188b: f30f16c2 movshdup xmm0, xmm2
1407e188f: f30f58c1 addss xmm0, xmm1
1407e1893: 4839c8 cmp rax, rcx
1407e1896: 7406 je 0x1407e189e
1407e1898: 440f2ec0 ucomiss xmm8, xmm0
1407e189c: 772a ja 0x1407e18c8
1407e189e: 0f2ec6 ucomiss xmm0, xmm6
1407e18a1: ba00000000 mov edx, 0
1407e18a6: 0f8664ffffff jbe 0x1407e1810
1407e18ac: ba00000000 mov edx, 0
1407e18b1: 4839c8 cmp rax, rcx
1407e18b4: 0f8556ffffff jne 0x1407e1810
1407e18ba: 0f2ef8 ucomiss xmm7, xmm0
1407e18bd: ba00000000 mov edx, 0
1407e18c2: 0f8648ffffff jbe 0x1407e1810
1407e18c8: f30f51c0 sqrtss xmm0, xmm0
1407e18cc: 440f28d0 movaps xmm10, xmm0
1407e18d0: f3450f58d1 addss xmm10, xmm9
1407e18d5: 4084ff test dil, dil
1407e18d8: 7413 je 0x1407e18ed
1407e18da: 4839c8 cmp rax, rcx
1407e18dd: 7517 jne 0x1407e18f6
1407e18df: 488b5508 mov rdx, qword ptr [rbp + 8]
1407e18e3: 4939d3 cmp r11, rdx
1407e18e6: 721f jb 0x1407e1907
1407e18e8: e936070000 jmp 0x1407e2023
1407e18ed: 440f28d0 movaps xmm10, xmm0
1407e18f1: 4839c8 cmp rax, rcx
1407e18f4: 74e9 je 0x1407e18df
1407e18f6: 440f28d0 movaps xmm10, xmm0
1407e18fa: 488b5508 mov rdx, qword ptr [rbp + 8]
1407e18fe: 4939d3 cmp r11, rdx
1407e1901: 0f831c070000 jae 0x1407e2023
1407e1907: 488b4500 mov rax, qword ptr [rbp]
1407e190b: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
1407e1912: 488d1cc8 lea rbx, [rax + rcx*8]
1407e1916: 4c8b64c810 mov r12, qword ptr [rax + rcx*8 + 0x10]
1407e191b: 4c3b24c8 cmp r12, qword ptr [rax + rcx*8]
1407e191f: 0f85bffeffff jne 0x1407e17e4
1407e1925: 4889d9 mov rcx, rbx
1407e1928: 488d1531732a02 lea rdx, [rip + 0x22a7331]
1407e192f: e8fc0cd8ff call 0x140562630
1407e1934: e9abfeffff jmp 0x1407e17e4
1407e1939: 4c89d0 mov rax, r10
1407e193c: 4c29f0 sub rax, r14
1407e193f: 48c1e804 shr rax, 4
1407e1943: 48b9cdcccccccccccccc movabs rcx, 0xcccccccccccccccd
1407e194d: 480fafc1 imul rax, rcx
1407e1951: 4839d0 cmp rax, rdx
1407e1954: 0f8636feffff jbe 0x1407e1790
1407e195a: 488d0492 lea rax, [rdx + rdx*4]
1407e195e: 48c1e004 shl rax, 4
1407e1962: 4901c6 add r14, rax
1407e1965: 4901d7 add r15, rdx
1407e1968: 4d89f0 mov r8, r14
1407e196b: 4c89fe mov rsi, r15
1407e196e: e9b5feffff jmp 0x1407e1828
1407e1973: 488b7d00 mov rdi, qword ptr [rbp]
1407e1977: 4c8b7508 mov r14, qword ptr [rbp + 8]
1407e197b: 4b8d0476 lea rax, [r14 + r14*2]
1407e197f: 4c8d24c7 lea r12, [rdi + rax*8]
1407e1983: 488d5d20 lea rbx, [rbp + 0x20]
1407e1987: 4889fe mov rsi, rdi
1407e198a: 4c39e6 cmp rsi, r12
1407e198d: 7445 je 0x1407e19d4
1407e198f: 90 nop 
1407e1990: 488b4e08 mov rcx, qword ptr [rsi + 8]
1407e1994: 488b5610 mov rdx, qword ptr [rsi + 0x10]
1407e1998: 488d8580000000 lea rax, [rbp + 0x80]
1407e199f: 48894520 mov qword ptr [rbp + 0x20], rax
1407e19a3: 4883fa02 cmp rdx, 2
1407e19a7: 730b jae 0x1407e19b4
1407e19a9: 4883c618 add rsi, 0x18
1407e19ad: 4c39e6 cmp rsi, r12
1407e19b0: 75de jne 0x1407e1990
1407e19b2: eb20 jmp 0x1407e19d4
1407e19b4: 4883fa15 cmp rdx, 0x15
1407e19b8: 7310 jae 0x1407e19ca
1407e19ba: 41b801000000 mov r8d, 1
1407e19c0: 4989d9 mov r9, rbx
1407e19c3: e8c89ea700 call 0x14125b890
1407e19c8: ebdf jmp 0x1407e19a9
1407e19ca: 4989d8 mov r8, rbx
1407e19cd: e84e0bab00 call 0x141292520
1407e19d2: ebd5 jmp 0x1407e19a9
1407e19d4: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
1407e19dc: 48c7455808000000 mov qword ptr [rbp + 0x58], 8
1407e19e4: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
1407e19ec: 31c9 xor ecx, ecx
1407e19ee: e82d818fff call 0x1400d9b20
1407e19f3: 4885c0 test rax, rax
1407e19f6: 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
1407e19fd: 0f8445060000 je 0x1407e2048
1407e1a03: 0f1000 movups xmm0, xmmword ptr [rax]
1407e1a06: 48ff00 inc qword ptr [rax]
1407e1a09: 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
1407e1a0d: 0f1005cc074402 movups xmm0, xmmword ptr [rip + 0x24407cc]
1407e1a14: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
1407e1a18: f30f6f05d0074402 movdqu xmm0, xmmword ptr [rip + 0x24407d0]
1407e1a20: 660f7f4530 movdqa xmmword ptr [rbp + 0x30], xmm0
1407e1a25: 4889c8 mov rax, rcx
1407e1a28: 488d1ccd00000000 lea rbx, [rcx*8]
1407e1a30: 4531ff xor r15d, r15d
1407e1a33: 48c1e83d shr rax, 0x3d
1407e1a37: 0f85f7050000 jne 0x1407e2034
1407e1a3d: 48b8f8ffffffffffff7f movabs rax, 0x7ffffffffffffff8
1407e1a47: 4839c3 cmp rbx, rax
1407e1a4a: 0f87e4050000 ja 0x1407e2034
1407e1a50: 4885db test rbx, rbx
1407e1a53: 4c8975f0 mov qword ptr [rbp - 0x10], r14
1407e1a57: 7437 je 0x1407e1a90
1407e1a59: 0fb605a931e502 movzx eax, byte ptr [rip + 0x2e531a9]
1407e1a60: 41bf08000000 mov r15d, 8
1407e1a66: ba08000000 mov edx, 8
1407e1a6b: 4889d9 mov rcx, rbx
1407e1a6e: e8cd21e3ff call 0x140613c40
1407e1a73: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
1407e1a7a: 4885c0 test rax, rax
1407e1a7d: 0f84b1050000 je 0x1407e2034
1407e1a83: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
1407e1a8a: 48895578 mov qword ptr [rbp + 0x78], rdx
1407e1a8e: eb1b jmp 0x1407e1aab
1407e1a90: b808000000 mov eax, 8
1407e1a95: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
1407e1a9c: 48c7457800000000 mov qword ptr [rbp + 0x78], 0
1407e1aa4: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
1407e1aab: 4531ed xor r13d, r13d
1407e1aae: 0f2835cbce1402 movaps xmm6, xmmword ptr [rip + 0x214cecb]
1407e1ab5: f30f103d13b81e02 movss xmm7, dword ptr [rip + 0x21eb813]
1407e1abd: f3440f10053e3e1402 movss xmm8, dword ptr [rip + 0x2143e3e]
1407e1ac6: f3440f100d41092a02 movss xmm9, dword ptr [rip + 0x22a0941]
1407e1acf: 66450f76d2 pcmpeqd xmm10, xmm10
1407e1ad4: 48897d10 mov qword ptr [rbp + 0x10], rdi
1407e1ad8: 4889f8 mov rax, rdi
1407e1adb: 488bbdb0010000 mov rdi, qword ptr [rbp + 0x1b0]
1407e1ae2: 4c89a5a0000000 mov qword ptr [rbp + 0xa0], r12
1407e1ae9: eb17 jmp 0x1407e1b02
1407e1aeb: 0f1f440000 nop dword ptr [rax + rax]
1407e1af0: 488b4570 mov rax, qword ptr [rbp + 0x70]
1407e1af4: 4c8bad98000000 mov r13, qword ptr [rbp + 0x98]
1407e1afb: 4c8ba5a0000000 mov r12, qword ptr [rbp + 0xa0]
1407e1b02: 4c39e0 cmp rax, r12
1407e1b05: 0f8428030000 je 0x1407e1e33
1407e1b0b: 488d4818 lea rcx, [rax + 0x18]
1407e1b0f: 48894d70 mov qword ptr [rbp + 0x70], rcx
1407e1b13: 498d4d01 lea rcx, [r13 + 1]
1407e1b17: 48898d98000000 mov qword ptr [rbp + 0x98], rcx
1407e1b1e: 4c8b7008 mov r14, qword ptr [rax + 8]
1407e1b22: 488b7010 mov rsi, qword ptr [rax + 0x10]
1407e1b26: 48c1e604 shl rsi, 4
1407e1b2a: 4c01f6 add rsi, r14
1407e1b2d: 4a8d04ad00000000 lea rax, [r13*4]
1407e1b35: 4c01e8 add rax, r13
1407e1b38: 48c1e004 shl rax, 4
1407e1b3c: 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
1407e1b43: 4801c8 add rax, rcx
1407e1b46: 4883c010 add rax, 0x10
1407e1b4a: 48894568 mov qword ptr [rbp + 0x68], rax
1407e1b4e: e99d000000 jmp 0x1407e1bf0
1407e1b53: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e1b60: f3420f6f0c01 movdqu xmm1, xmmword ptr [rcx + r8]
1407e1b66: 660f6fd1 movdqa xmm2, xmm1
1407e1b6a: 660f74d0 pcmpeqb xmm2, xmm0
1407e1b6e: 660fd7da pmovmskb ebx, xmm2
1407e1b72: 85db test ebx, ebx
1407e1b74: 743a je 0x1407e1bb0
1407e1b76: f3440fbcfb tzcnt r15d, ebx
1407e1b7b: 4d01c7 add r15, r8
1407e1b7e: 4d21df and r15, r11
1407e1b81: 49c1e704 shl r15, 4
1407e1b85: 4889c7 mov rdi, rax
1407e1b88: 4c29ff sub rdi, r15
1407e1b8b: 4c3b0f cmp r9, qword ptr [rdi]
1407e1b8e: 750a jne 0x1407e1b9a
1407e1b90: 4c3b5708 cmp r10, qword ptr [rdi + 8]
1407e1b94: 0f8486020000 je 0x1407e1e20
1407e1b9a: 8d7bff lea edi, [rbx - 1]
1407e1b9d: 6621df and di, bx
1407e1ba0: 89fb mov ebx, edi
1407e1ba2: 75d2 jne 0x1407e1b76
1407e1ba4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e1bb0: 4889d3 mov rbx, rdx
1407e1bb3: 66410f74ca pcmpeqb xmm1, xmm10
1407e1bb8: 660fd7f9 pmovmskb edi, xmm1
1407e1bbc: 85ff test edi, edi
1407e1bbe: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
1407e1bc5: 488bbdb0010000 mov rdi, qword ptr [rbp + 0x1b0]
1407e1bcc: 0f85ce000000 jne 0x1407e1ca0
1407e1bd2: 4889da mov rdx, rbx
1407e1bd5: 4901d8 add r8, rbx
1407e1bd8: 4983c010 add r8, 0x10
1407e1bdc: 4883c210 add rdx, 0x10
1407e1be0: 4d21d8 and r8, r11
1407e1be3: e978ffffff jmp 0x1407e1b60
1407e1be8: 0f1f840000000000 nop dword ptr [rax + rax]
1407e1bf0: 4939f6 cmp r14, rsi
1407e1bf3: 0f84f7feffff je 0x1407e1af0
1407e1bf9: 4939d5 cmp r13, rdx
1407e1bfc: 0f83e0030000 jae 0x1407e1fe2
1407e1c02: 4d8b26 mov r12, qword ptr [r14]
1407e1c05: 4939d4 cmp r12, rdx
1407e1c08: 0f83c3030000 jae 0x1407e1fd1
1407e1c0e: 4983c610 add r14, 0x10
1407e1c12: 4d39e5 cmp r13, r12
1407e1c15: 74d9 je 0x1407e1bf0
1407e1c17: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1407e1c1e: 4a393ce8 cmp qword ptr [rax + r13*8], rdi
1407e1c22: 73cc jae 0x1407e1bf0
1407e1c24: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1407e1c2b: 4a393ce0 cmp qword ptr [rax + r12*8], rdi
1407e1c2f: 73bf jae 0x1407e1bf0
1407e1c31: 4c89ad80000000 mov qword ptr [rbp + 0x80], r13
1407e1c38: 4c89a588000000 mov qword ptr [rbp + 0x88], r12
1407e1c3f: 48837d3800 cmp qword ptr [rbp + 0x38], 0
1407e1c44: 745a je 0x1407e1ca0
1407e1c46: 488d4d40 lea rcx, [rbp + 0x40]
1407e1c4a: 488d9580000000 lea rdx, [rbp + 0x80]
1407e1c51: e85a4da700 call 0x1412569b0
1407e1c56: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
1407e1c5a: 4c8b5d28 mov r11, qword ptr [rbp + 0x28]
1407e1c5e: 4d89d8 mov r8, r11
1407e1c61: 4921c0 and r8, rax
1407e1c64: 48c1e839 shr rax, 0x39
1407e1c68: 660f6ec0 movd xmm0, eax
1407e1c6c: 660f60c0 punpcklbw xmm0, xmm0
1407e1c70: f20f70c000 pshuflw xmm0, xmm0, 0
1407e1c75: 660f70c000 pshufd xmm0, xmm0, 0
1407e1c7a: 488d41f0 lea rax, [rcx - 0x10]
1407e1c7e: 4c8b8d80000000 mov r9, qword ptr [rbp + 0x80]
1407e1c85: 4c8b9588000000 mov r10, qword ptr [rbp + 0x88]
1407e1c8c: 31d2 xor edx, edx
1407e1c8e: e9cdfeffff jmp 0x1407e1b60
1407e1c93: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e1ca0: 488b4d68 mov rcx, qword ptr [rbp + 0x68]
1407e1ca4: 8b01 mov eax, dword ptr [rcx]
1407e1ca6: f30f104104 movss xmm0, dword ptr [rcx + 4]
1407e1cab: 8b4908 mov ecx, dword ptr [rcx + 8]
1407e1cae: 4f8d04a4 lea r8, [r12 + r12*4]
1407e1cb2: 49c1e004 shl r8, 4
1407e1cb6: 4c8b8da8000000 mov r9, qword ptr [rbp + 0xa8]
1407e1cbd: f2430f104c0110 movsd xmm1, qword ptr [r9 + r8 + 0x10]
1407e1cc4: f30f16e9 movshdup xmm5, xmm1
1407e1cc8: f3430f105c0118 movss xmm3, dword ptr [r9 + r8 + 0x18]
1407e1ccf: 660f6ee0 movd xmm4, eax
1407e1cd3: 660f6ed1 movd xmm2, ecx
1407e1cd7: 66440f6fdc movdqa xmm11, xmm4
1407e1cdc: f3440f5cd9 subss xmm11, xmm1
1407e1ce1: 66440f6fe2 movdqa xmm12, xmm2
1407e1ce6: f3440f5ce3 subss xmm12, xmm3
1407e1ceb: f3450f59db mulss xmm11, xmm11
1407e1cf0: f3450f59e4 mulss xmm12, xmm12
1407e1cf5: f3450f58e3 addss xmm12, xmm11
1407e1cfa: 450f57db xorps xmm11, xmm11
1407e1cfe: f3450f51dc sqrtss xmm11, xmm12
1407e1d03: 440f28e0 movaps xmm12, xmm0
1407e1d07: f3440f5ce5 subss xmm12, xmm5
1407e1d0c: 440f54e6 andps xmm12, xmm6
1407e1d10: f3450f58e4 addss xmm12, xmm12
1407e1d15: 450f2ee3 ucomiss xmm12, xmm11
1407e1d19: 0f87d1feffff ja 0x1407e1bf0
1407e1d1f: 660f62e0 punpckldq xmm4, xmm0
1407e1d23: 0f5ccc subps xmm1, xmm4
1407e1d26: f30f5cda subss xmm3, xmm2
1407e1d2a: 0f28d1 movaps xmm2, xmm1
1407e1d2d: 0f59d1 mulps xmm2, xmm1
1407e1d30: f30f16e2 movshdup xmm4, xmm2
1407e1d34: f30f58e2 addss xmm4, xmm2
1407e1d38: 0f28d3 movaps xmm2, xmm3
1407e1d3b: f30f59d3 mulss xmm2, xmm3
1407e1d3f: f30f58d4 addss xmm2, xmm4
1407e1d43: f30f51d2 sqrtss xmm2, xmm2
1407e1d47: 0f2ed7 ucomiss xmm2, xmm7
1407e1d4a: 0f86a0feffff jbe 0x1407e1bf0
1407e1d50: 410f28e0 movaps xmm4, xmm8
1407e1d54: f30f5ee2 divss xmm4, xmm2
1407e1d58: f30f59dc mulss xmm3, xmm4
1407e1d5c: 898580000000 mov dword ptr [rbp + 0x80], eax
1407e1d62: f30f118584000000 movss dword ptr [rbp + 0x84], xmm0
1407e1d6a: 898d88000000 mov dword ptr [rbp + 0x88], ecx
1407e1d70: f30f12c4 movsldup xmm0, xmm4
1407e1d74: 0f59c8 mulps xmm1, xmm0
1407e1d77: 0f138d8c000000 movlps qword ptr [rbp + 0x8c], xmm1
1407e1d7e: f30f119d94000000 movss dword ptr [rbp + 0x94], xmm3
1407e1d86: f3410f58d1 addss xmm2, xmm9
1407e1d8b: 488b4de0 mov rcx, qword ptr [rbp - 0x20]
1407e1d8f: 488d9580000000 lea rdx, [rbp + 0x80]
1407e1d96: 41b9db300000 mov r9d, 0x30db
1407e1d9c: e80fb80e00 call 0x1408cd5b0
1407e1da1: 84c0 test al, al
1407e1da3: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
1407e1daa: 488bbdb0010000 mov rdi, qword ptr [rbp + 0x1b0]
1407e1db1: 0f8539feffff jne 0x1407e1bf0
1407e1db7: 4d39ec cmp r12, r13
1407e1dba: 4c89eb mov rbx, r13
1407e1dbd: 490f42dc cmovb rbx, r12
1407e1dc1: 4d89ef mov r15, r13
1407e1dc4: 4d0f47fc cmova r15, r12
1407e1dc8: 488b7d60 mov rdi, qword ptr [rbp + 0x60]
1407e1dcc: 483b7d50 cmp rdi, qword ptr [rbp + 0x50]
1407e1dd0: 7510 jne 0x1407e1de2
1407e1dd2: 488d4d50 lea rcx, [rbp + 0x50]
1407e1dd6: 488d15536e2a02 lea rdx, [rip + 0x22a6e53]
1407e1ddd: e86ec7ecff call 0x1406ae550
1407e1de2: 488b4558 mov rax, qword ptr [rbp + 0x58]
1407e1de6: 4889f9 mov rcx, rdi
1407e1de9: 48c1e104 shl rcx, 4
1407e1ded: 48891c08 mov qword ptr [rax + rcx], rbx
1407e1df1: 4c897c0808 mov qword ptr [rax + rcx + 8], r15
1407e1df6: 48ffc7 inc rdi
1407e1df9: 48897d60 mov qword ptr [rbp + 0x60], rdi
1407e1dfd: 488d4d20 lea rcx, [rbp + 0x20]
1407e1e01: 4889da mov rdx, rbx
1407e1e04: 4d89f8 mov r8, r15
1407e1e07: e8e45ca800 call 0x141267af0
1407e1e0c: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1407e1e13: 4aff04e8 inc qword ptr [rax + r13*8]
1407e1e17: 4aff04e0 inc qword ptr [rax + r12*8]
1407e1e1b: 0f1f440000 nop dword ptr [rax + rax]
1407e1e20: 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
1407e1e27: 488bbdb0010000 mov rdi, qword ptr [rbp + 0x1b0]
1407e1e2e: e9bdfdffff jmp 0x1407e1bf0
1407e1e33: 488b4560 mov rax, qword ptr [rbp + 0x60]
1407e1e37: 488b7de8 mov rdi, qword ptr [rbp - 0x18]
1407e1e3b: 48894710 mov qword ptr [rdi + 0x10], rax
1407e1e3f: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
1407e1e43: 0f1107 movups xmmword ptr [rdi], xmm0
1407e1e46: 488b4578 mov rax, qword ptr [rbp + 0x78]
1407e1e4a: 48894718 mov qword ptr [rdi + 0x18], rax
1407e1e4e: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1407e1e55: 48894720 mov qword ptr [rdi + 0x20], rax
1407e1e59: 48895728 mov qword ptr [rdi + 0x28], rdx
1407e1e5d: 488b5528 mov rdx, qword ptr [rbp + 0x28]
1407e1e61: 4885d2 test rdx, rdx
1407e1e64: 7429 je 0x1407e1e8f
1407e1e66: 4889d0 mov rax, rdx
1407e1e69: 48c1e004 shl rax, 4
1407e1e6d: 4801c2 add rdx, rax
1407e1e70: 4883c221 add rdx, 0x21
1407e1e74: 7419 je 0x1407e1e8f
1407e1e76: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
1407e1e7a: 48f7d8 neg rax
1407e1e7d: 4801c1 add rcx, rax
1407e1e80: 4883c1f0 add rcx, -0x10
1407e1e84: 41b810000000 mov r8d, 0x10
1407e1e8a: e8911de3ff call 0x140613c20
1407e1e8f: 4c8b75f0 mov r14, qword ptr [rbp - 0x10]
1407e1e93: 4d85f6 test r14, r14
1407e1e96: 0fb69dc7000000 movzx ebx, byte ptr [rbp + 0xc7]
1407e1e9d: 7437 je 0x1407e1ed6
1407e1e9f: 488b4510 mov rax, qword ptr [rbp + 0x10]
1407e1ea3: 488d7008 lea rsi, [rax + 8]
1407e1ea7: eb10 jmp 0x1407e1eb9
1407e1ea9: 0f1f8000000000 nop dword ptr [rax]
1407e1eb0: 4883c618 add rsi, 0x18
1407e1eb4: 49ffce dec r14
1407e1eb7: 741d je 0x1407e1ed6
1407e1eb9: 488b56f8 mov rdx, qword ptr [rsi - 8]
1407e1ebd: 4885d2 test rdx, rdx
1407e1ec0: 74ee je 0x1407e1eb0
1407e1ec2: 488b0e mov rcx, qword ptr [rsi]
1407e1ec5: 48c1e204 shl rdx, 4
1407e1ec9: 41b808000000 mov r8d, 8
1407e1ecf: e84c1de3ff call 0x140613c20
1407e1ed4: ebda jmp 0x1407e1eb0
1407e1ed6: 488b45f8 mov rax, qword ptr [rbp - 8]
1407e1eda: 4885c0 test rax, rax
1407e1edd: 7417 je 0x1407e1ef6
1407e1edf: 48c1e003 shl rax, 3
1407e1ee3: 488d1440 lea rdx, [rax + rax*2]
1407e1ee7: 41b808000000 mov r8d, 8
1407e1eed: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
1407e1ef1: e82a1de3ff call 0x140613c20
1407e1ef6: 84db test bl, bl
1407e1ef8: 7448 je 0x1407e1f42
1407e1efa: e831624300 call 0x140c18130
1407e1eff: 4889c1 mov rcx, rax
1407e1f02: 488b00 mov rax, qword ptr [rax]
1407e1f05: 4883f801 cmp rax, 1
1407e1f09: 0f859a000000 jne 0x1407e1fa9
1407e1f0f: 4883c108 add rcx, 8
1407e1f13: 488b5518 mov rdx, qword ptr [rbp + 0x18]
1407e1f17: 48833900 cmp qword ptr [rcx], 0
1407e1f1b: 0f85f6000000 jne 0x1407e2017
1407e1f21: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407e1f28: 48898db8000000 mov qword ptr [rbp + 0xb8], rcx
1407e1f2f: 4883c108 add rcx, 8
1407e1f33: e8b8514300 call 0x140c170f0
1407e1f38: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1407e1f3f: 48ff00 inc qword ptr [rax]
1407e1f42: 4889f8 mov rax, rdi
1407e1f45: 0f28b5d0000000 movaps xmm6, xmmword ptr [rbp + 0xd0]
1407e1f4c: 0f28bde0000000 movaps xmm7, xmmword ptr [rbp + 0xe0]
1407e1f53: 440f2885f0000000 movaps xmm8, xmmword ptr [rbp + 0xf0]
1407e1f5b: 440f288d00010000 movaps xmm9, xmmword ptr [rbp + 0x100]
1407e1f63: 440f289510010000 movaps xmm10, xmmword ptr [rbp + 0x110]
1407e1f6b: 440f289d20010000 movaps xmm11, xmmword ptr [rbp + 0x120]
1407e1f73: 440f28a530010000 movaps xmm12, xmmword ptr [rbp + 0x130]
1407e1f7b: 4881c4c8010000 add rsp, 0x1c8
1407e1f82: 5b pop rbx
1407e1f83: 5f pop rdi
1407e1f84: 5e pop rsi
1407e1f85: 415c pop r12
1407e1f87: 415d pop r13
1407e1f89: 415e pop r14
1407e1f8b: 415f pop r15
1407e1f8d: 5d pop rbp
1407e1f8e: c3 ret 
1407e1f8f: 4885c0 test rax, rax
1407e1f92: 7531 jne 0x1407e1fc5
1407e1f94: 31d2 xor edx, edx
1407e1f96: e8e5051102 call 0x1428f2580
1407e1f9b: 4889c1 mov rcx, rax
1407e1f9e: 4885c0 test rax, rax
1407e1fa1: 0f85e3f6ffff jne 0x1407e168a
1407e1fa7: eb1c jmp 0x1407e1fc5
1407e1fa9: 4885c0 test rax, rax
1407e1fac: 7517 jne 0x1407e1fc5
1407e1fae: 31d2 xor edx, edx
1407e1fb0: e8cb051102 call 0x1428f2580
1407e1fb5: 4889c1 mov rcx, rax
1407e1fb8: 488b5518 mov rdx, qword ptr [rbp + 0x18]
1407e1fbc: 4885c0 test rax, rax
1407e1fbf: 0f8552ffffff jne 0x1407e1f17
1407e1fc5: 488d0dfcf34302 lea rcx, [rip + 0x243f3fc]
1407e1fcc: e8ff600f02 call 0x1428d80d0
1407e1fd1: 4c8d05406c2a02 lea r8, [rip + 0x22a6c40]
1407e1fd8: 4c89e1 mov rcx, r12
1407e1fdb: e838750f02 call 0x1428d9518
1407e1fe0: eb72 jmp 0x1407e2054
1407e1fe2: 4c8d05176c2a02 lea r8, [rip + 0x22a6c17]
1407e1fe9: 4c89e9 mov rcx, r13
1407e1fec: e827750f02 call 0x1428d9518
1407e1ff1: eb61 jmp 0x1407e2054
1407e1ff3: 4883fe15 cmp rsi, 0x15
1407e1ff7: 735d jae 0x1407e2056
1407e1ff9: 4c8d4d20 lea r9, [rbp + 0x20]
1407e1ffd: 41b801000000 mov r8d, 1
1407e2003: 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
1407e200a: 4889f2 mov rdx, rsi
1407e200d: e83e97a700 call 0x14125b750
1407e2012: e9ebf6ffff jmp 0x1407e1702
1407e2017: 488d0d1af44302 lea rcx, [rip + 0x243f41a]
1407e201e: e82d720f02 call 0x1428d9250
1407e2023: 4c8d051e6c2a02 lea r8, [rip + 0x22a6c1e]
1407e202a: 4c89d9 mov rcx, r11
1407e202d: e8e6740f02 call 0x1428d9518
1407e2032: eb20 jmp 0x1407e2054
1407e2034: 4c8d05ad6b2a02 lea r8, [rip + 0x22a6bad]
1407e203b: 4c89f9 mov rcx, r15
1407e203e: 4889da mov rdx, rbx
1407e2041: e89d6f0f02 call 0x1428d8fe3
1407e2046: eb0c jmp 0x1407e2054
1407e2048: 488d0d79f34302 lea rcx, [rip + 0x243f379]
1407e204f: e87c600f02 call 0x1428d80d0
1407e2054: 0f0b ud2 
1407e2056: 4c8d4520 lea r8, [rbp + 0x20]
1407e205a: 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
1407e2061: 4889f2 mov rdx, rsi
1407e2064: e8e702ab00 call 0x141292350
1407e2069: e994f6ffff jmp 0x1407e1702
1407e206e: 6690 nop 
1407e2070: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e2075: 55 push rbp
1407e2076: 4157 push r15
1407e2078: 4156 push r14
1407e207a: 4155 push r13
1407e207c: 4154 push r12
1407e207e: 56 push rsi
1407e207f: 57 push rdi
1407e2080: 53 push rbx
1407e2081: 4881eca8000000 sub rsp, 0xa8
1407e2088: 488daa80000000 lea rbp, [rdx + 0x80]
1407e208f: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1407e2095: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1407e209b: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
1407e20a1: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1407e20a7: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1407e20ad: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1407e20b5: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
1407e20bd: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1407e20c4: 48ff00 inc qword ptr [rax]
1407e20c7: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
1407e20cf: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
1407e20d7: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
1407e20dd: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
1407e20e3: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
1407e20e9: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
1407e20ef: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
1407e20f5: 4881c4a8000000 add rsp, 0xa8
1407e20fc: 5b pop rbx
1407e20fd: 5f pop rdi
1407e20fe: 5e pop rsi
1407e20ff: 415c pop r12
1407e2101: 415d pop r13
1407e2103: 415e pop r14
1407e2105: 415f pop r15
1407e2107: 5d pop rbp
1407e2108: c3 ret 
1407e2109: 0f1f8000000000 nop dword ptr [rax]
1407e2110: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e2115: 55 push rbp
1407e2116: 4157 push r15
1407e2118: 4156 push r14
1407e211a: 4155 push r13
1407e211c: 4154 push r12
1407e211e: 56 push rsi
1407e211f: 57 push rdi
1407e2120: 53 push rbx
1407e2121: 4881eca8000000 sub rsp, 0xa8
1407e2128: 488daa80000000 lea rbp, [rdx + 0x80]
1407e212f: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1407e2135: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1407e213b: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
1407e2141: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1407e2147: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1407e214d: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1407e2155: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
1407e215d: 80bdc700000000 cmp byte ptr [rbp + 0xc7], 0
1407e2164: 7420 je 0x1407e2186
1407e2166: 488d45c8 lea rax, [rbp - 0x38]
1407e216a: 488945d8 mov qword ptr [rbp - 0x28], rax
1407e216e: 488d45d8 lea rax, [rbp - 0x28]
1407e2172: 488945d0 mov qword ptr [rbp - 0x30], rax
1407e2176: 488d0d73682a02 lea rcx, [rip + 0x22a6873]
1407e217d: 488d55d0 lea rdx, [rbp - 0x30]
1407e2181: e83a5aa700 call 0x141257bc0
1407e2186: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
1407e218e: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
1407e2196: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
1407e219c: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
1407e21a2: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
1407e21a8: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
1407e21ae: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
1407e21b4: 4881c4a8000000 add rsp, 0xa8
1407e21bb: 5b pop rbx
1407e21bc: 5f pop rdi
1407e21bd: 5e pop rsi
1407e21be: 415c pop r12
1407e21c0: 415d pop r13
1407e21c2: 415e pop r14
1407e21c4: 415f pop r15
1407e21c6: 5d pop rbp
1407e21c7: c3 ret 
1407e21c8: 0f1f840000000000 nop dword ptr [rax + rax]
1407e21d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e21d5: 55 push rbp
1407e21d6: 4157 push r15
1407e21d8: 4156 push r14
1407e21da: 4155 push r13
1407e21dc: 4154 push r12
1407e21de: 56 push rsi
1407e21df: 57 push rdi
1407e21e0: 53 push rbx
1407e21e1: 4881eca8000000 sub rsp, 0xa8
1407e21e8: 488daa80000000 lea rbp, [rdx + 0x80]
1407e21ef: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1407e21f5: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1407e21fb: 66440f7f542450 movdqa xmmword ptr [rsp + 0x50], xmm10
1407e2202: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1407e2208: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1407e220e: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1407e2216: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
1407e221e: 488d4d20 lea rcx, [rbp + 0x20]
1407e2222: e8497a96ff call 0x140149c70
1407e2227: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
1407e222f: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
1407e2237: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
1407e223d: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
1407e2243: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
1407e2249: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
1407e224f: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
1407e2255: 4881c4a8000000 add rsp, 0xa8
1407e225c: 5b pop rbx
1407e225d: 5f pop rdi
1407e225e: 5e pop rsi
1407e225f: 415c pop r12
1407e2261: 415d pop r13
1407e2263: 415e pop r14
1407e2265: 415f pop r15
1407e2267: 5d pop rbp
1407e2268: c3 ret 
1407e2269: 0f1f8000000000 nop dword ptr [rax]
1407e2270: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e2275: 55 push rbp
1407e2276: 4157 push r15
1407e2278: 4156 push r14
1407e227a: 4155 push r13
1407e227c: 4154 push r12
1407e227e: 56 push rsi
1407e227f: 57 push rdi
1407e2280: 53 push rbx
1407e2281: 4881eca8000000 sub rsp, 0xa8
1407e2288: 488daa80000000 lea rbp, [rdx + 0x80]
1407e228f: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1407e2295: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1407e229b: 66440f7f542450 movdqa xmmword ptr [rsp + 0x50], xmm10
1407e22a2: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1407e22a8: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1407e22ae: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1407e22b6: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
1407e22be: 488b5578 mov rdx, qword ptr [rbp + 0x78]
1407e22c2: 4885d2 test rdx, rdx
1407e22c5: 7416 je 0x1407e22dd
1407e22c7: 48c1e203 shl rdx, 3
1407e22cb: 41b808000000 mov r8d, 8
1407e22d1: 488b8db8000000 mov rcx, qword ptr [rbp + 0xb8]
1407e22d8: e84319e3ff call 0x140613c20
1407e22dd: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
1407e22e5: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
1407e22ed: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
1407e22f3: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
1407e22f9: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
1407e22ff: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
1407e2305: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
1407e230b: 4881c4a8000000 add rsp, 0xa8
1407e2312: 5b pop rbx
1407e2313: 5f pop rdi
1407e2314: 5e pop rsi
1407e2315: 415c pop r12
1407e2317: 415d pop r13
1407e2319: 415e pop r14
1407e231b: 415f pop r15
1407e231d: 5d pop rbp
1407e231e: c3 ret 
1407e231f: 90 nop 
1407e2320: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e2325: 55 push rbp
1407e2326: 4157 push r15
1407e2328: 4156 push r14
1407e232a: 4155 push r13
1407e232c: 4154 push r12
1407e232e: 56 push rsi
1407e232f: 57 push rdi
1407e2330: 53 push rbx
1407e2331: 4881eca8000000 sub rsp, 0xa8
1407e2338: 488daa80000000 lea rbp, [rdx + 0x80]
1407e233f: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1407e2345: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1407e234b: 66440f7f542450 movdqa xmmword ptr [rsp + 0x50], xmm10
1407e2352: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1407e2358: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1407e235e: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1407e2366: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
1407e236e: 488b85b8000000 mov rax, qword ptr [rbp + 0xb8]
1407e2375: 48ff00 inc qword ptr [rax]
1407e2378: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
1407e2380: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
1407e2388: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
1407e238e: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
1407e2394: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
1407e239a: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
1407e23a0: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
1407e23a6: 4881c4a8000000 add rsp, 0xa8
1407e23ad: 5b pop rbx
1407e23ae: 5f pop rdi
1407e23af: 5e pop rsi
1407e23b0: 415c pop r12
1407e23b2: 415d pop r13
1407e23b4: 415e pop r14
1407e23b6: 415f pop r15
1407e23b8: 5d pop rbp
1407e23b9: c3 ret 
1407e23ba: 660f1f440000 nop word ptr [rax + rax]
1407e23c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e23c5: 55 push rbp
1407e23c6: 4157 push r15
1407e23c8: 4156 push r14
1407e23ca: 4155 push r13
1407e23cc: 4154 push r12
1407e23ce: 56 push rsi
1407e23cf: 57 push rdi
1407e23d0: 53 push rbx
1407e23d1: 4881eca8000000 sub rsp, 0xa8
1407e23d8: 488daa80000000 lea rbp, [rdx + 0x80]
1407e23df: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1407e23e5: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1407e23eb: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
1407e23f1: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1407e23f7: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1407e23fd: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1407e2405: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
1407e240d: 488b5550 mov rdx, qword ptr [rbp + 0x50]
1407e2411: 4885d2 test rdx, rdx
1407e2414: 7413 je 0x1407e2429
1407e2416: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
1407e241a: 48c1e204 shl rdx, 4
1407e241e: 41b808000000 mov r8d, 8
1407e2424: e8f717e3ff call 0x140613c20
1407e2429: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
1407e2431: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
1407e2439: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
1407e243f: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
1407e2445: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
1407e244b: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
1407e2451: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
1407e2457: 4881c4a8000000 add rsp, 0xa8
1407e245e: 5b pop rbx
1407e245f: 5f pop rdi
1407e2460: 5e pop rsi
1407e2461: 415c pop r12
1407e2463: 415d pop r13
1407e2465: 415e pop r14
1407e2467: 415f pop r15
1407e2469: 5d pop rbp
1407e246a: c3 ret 
1407e246b: 0f1f440000 nop dword ptr [rax + rax]
1407e2470: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e2475: 55 push rbp
1407e2476: 4157 push r15
1407e2478: 4156 push r14
1407e247a: 4155 push r13
1407e247c: 4154 push r12
1407e247e: 56 push rsi
1407e247f: 57 push rdi
1407e2480: 53 push rbx
1407e2481: 4881eca8000000 sub rsp, 0xa8
1407e2488: 488daa80000000 lea rbp, [rdx + 0x80]
1407e248f: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
1407e2495: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
1407e249b: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
1407e24a1: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
1407e24a7: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1407e24ad: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
1407e24b5: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
1407e24bd: 488d4df8 lea rcx, [rbp - 8]
1407e24c1: e8ba7ba700 call 0x14125a080
1407e24c6: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
1407e24ce: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
1407e24d6: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
1407e24dc: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
1407e24e2: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
1407e24e8: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
1407e24ee: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
1407e24f4: 4881c4a8000000 add rsp, 0xa8
1407e24fb: 5b pop rbx
1407e24fc: 5f pop rdi
1407e24fd: 5e pop rsi
1407e24fe: 415c pop r12
1407e2500: 415d pop r13
1407e2502: 415e pop r14
1407e2504: 415f pop r15
1407e2506: 5d pop rbp
1407e2507: c3 ret 
