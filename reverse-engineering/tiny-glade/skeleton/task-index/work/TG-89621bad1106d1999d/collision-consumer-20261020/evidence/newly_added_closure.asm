0x1417d1440 55 push rbp
0x1417d1441 4157 push r15
0x1417d1443 4156 push r14
0x1417d1445 4154 push r12
0x1417d1447 56 push rsi
0x1417d1448 57 push rdi
0x1417d1449 53 push rbx
0x1417d144a 4881ecf0000000 sub rsp, 0xf0
0x1417d1451 488dac2480000000 lea rbp, [rsp + 0x80]
0x1417d1459 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
0x1417d1461 4889d0 mov rax, rdx
0x1417d1464 4c8b39 mov r15, qword ptr [rcx]
0x1417d1467 8b12 mov edx, dword ptr [rdx]
0x1417d1469 448b4004 mov r8d, dword ptr [rax + 4]
0x1417d146d 498b07 mov rax, qword ptr [r15]
0x1417d1470 0fb608 movzx ecx, byte ptr [rax]
0x1417d1473 884d60 mov byte ptr [rbp + 0x60], cl
0x1417d1476 895558 mov dword ptr [rbp + 0x58], edx
0x1417d1479 4489455c mov dword ptr [rbp + 0x5c], r8d
0x1417d147d 4d8b6708 mov r12, qword ptr [r15 + 8]
0x1417d1481 450fb64c2408 movzx r9d, byte ptr [r12 + 8]
0x1417d1487 4138c9 cmp r9b, cl
0x1417d148a 7511 jne 0x1417d149d
0x1417d148c 41391424 cmp dword ptr [r12], edx
0x1417d1490 750b jne 0x1417d149d
0x1417d1492 4539442404 cmp dword ptr [r12 + 4], r8d
0x1417d1497 0f844a030000 je 0x1417d17e7
0x1417d149d 31c0 xor eax, eax
0x1417d149f 4138c9 cmp r9b, cl
0x1417d14a2 0f95c0 setne al
0x1417d14a5 b9ff000000 mov ecx, 0xff
0x1417d14aa 0f42c1 cmovb eax, ecx
0x1417d14ad 84c0 test al, al
0x1417d14af 7409 je 0x1417d14ba
0x1417d14b1 3cff cmp al, 0xff
0x1417d14b3 7530 jne 0x1417d14e5
0x1417d14b5 e9e6000000 jmp 0x1417d15a0
0x1417d14ba 4531c9 xor r9d, r9d
0x1417d14bd 41391424 cmp dword ptr [r12], edx
0x1417d14c1 410f95c1 setne r9b
0x1417d14c5 440f42c9 cmovb r9d, ecx
0x1417d14c9 31c0 xor eax, eax
0x1417d14cb 4539442404 cmp dword ptr [r12 + 4], r8d
0x1417d14d0 0f95c0 setne al
0x1417d14d3 0f42c1 cmovb eax, ecx
0x1417d14d6 4584c9 test r9b, r9b
0x1417d14d9 410f45c1 cmovne eax, r9d
0x1417d14dd 3cff cmp al, 0xff
0x1417d14df 0f84bb000000 je 0x1417d15a0
0x1417d14e5 498b7710 mov rsi, qword ptr [r15 + 0x10]
0x1417d14e9 48837e1800 cmp qword ptr [rsi + 0x18], 0
0x1417d14ee 0f84ac000000 je 0x1417d15a0
0x1417d14f4 488d4e20 lea rcx, [rsi + 0x20]
0x1417d14f8 488d5558 lea rdx, [rbp + 0x58]
0x1417d14fc e89f57a8ff call 0x141256ca0
0x1417d1501 488b0e mov rcx, qword ptr [rsi]
0x1417d1504 4c8b4e08 mov r9, qword ptr [rsi + 8]
0x1417d1508 4d89ca mov r10, r9
0x1417d150b 4921c2 and r10, rax
0x1417d150e 48c1e839 shr rax, 0x39
0x1417d1512 660f6ec0 movd xmm0, eax
0x1417d1516 660f60c0 punpcklbw xmm0, xmm0
0x1417d151a f20f70c000 pshuflw xmm0, xmm0, 0
0x1417d151f 660f70c000 pshufd xmm0, xmm0, 0
0x1417d1524 488d41f4 lea rax, [rcx - 0xc]
0x1417d1528 440fb65d60 movzx r11d, byte ptr [rbp + 0x60]
0x1417d152d 8b5558 mov edx, dword ptr [rbp + 0x58]
0x1417d1530 448b455c mov r8d, dword ptr [rbp + 0x5c]
0x1417d1534 31f6 xor esi, esi
0x1417d1536 660f76c9 pcmpeqd xmm1, xmm1
0x1417d153a f3420f6f1411 movdqu xmm2, xmmword ptr [rcx + r10]
0x1417d1540 660f6fda movdqa xmm3, xmm2
0x1417d1544 660f74d8 pcmpeqb xmm3, xmm0
0x1417d1548 660fd7fb pmovmskb edi, xmm3
0x1417d154c 85ff test edi, edi
0x1417d154e 7434 je 0x1417d1584
0x1417d1550 f30fbcdf tzcnt ebx, edi
0x1417d1554 4c01d3 add rbx, r10
0x1417d1557 4c21cb and rbx, r9
0x1417d155a 48f7db neg rbx
0x1417d155d 488d1c5b lea rbx, [rbx + rbx*2]
0x1417d1561 443a5c9808 cmp r11b, byte ptr [rax + rbx*4 + 8]
0x1417d1566 7512 jne 0x1417d157a
0x1417d1568 488d1c98 lea rbx, [rax + rbx*4]
0x1417d156c 3b13 cmp edx, dword ptr [rbx]
0x1417d156e 750a jne 0x1417d157a
0x1417d1570 443b4304 cmp r8d, dword ptr [rbx + 4]
0x1417d1574 0f846d020000 je 0x1417d17e7
0x1417d157a 8d5fff lea ebx, [rdi - 1]
0x1417d157d 6621fb and bx, di
0x1417d1580 89df mov edi, ebx
0x1417d1582 75cc jne 0x1417d1550
0x1417d1584 660f74d1 pcmpeqb xmm2, xmm1
0x1417d1588 660fd7fa pmovmskb edi, xmm2
0x1417d158c 85ff test edi, edi
0x1417d158e 7510 jne 0x1417d15a0
0x1417d1590 4901f2 add r10, rsi
0x1417d1593 4983c210 add r10, 0x10
0x1417d1597 4883c610 add rsi, 0x10
0x1417d159b 4d21ca and r10, r9
0x1417d159e eb9a jmp 0x1417d153a
0x1417d15a0 498b4f18 mov rcx, qword ptr [r15 + 0x18]
0x1417d15a4 e897233cff call 0x140b93940
0x1417d15a9 4885c0 test rax, rax
0x1417d15ac 0f8449020000 je 0x1417d17fb
0x1417d15b2 4889c6 mov rsi, rax
0x1417d15b5 498b5f20 mov rbx, qword ptr [r15 + 0x20]
0x1417d15b9 498b7f28 mov rdi, qword ptr [r15 + 0x28]
0x1417d15bd 4889d9 mov rcx, rbx
0x1417d15c0 4889c2 mov rdx, rax
0x1417d15c3 4989f8 mov r8, rdi
0x1417d15c6 e8f59d2cff call 0x140a9b3c0
0x1417d15cb 84c0 test al, al
0x1417d15cd 0f8414020000 je 0x1417d17e7
0x1417d15d3 488b4310 mov rax, qword ptr [rbx + 0x10]
0x1417d15d7 4c8b4318 mov r8, qword ptr [rbx + 0x18]
0x1417d15db 498b4810 mov rcx, qword ptr [r8 + 0x10]
0x1417d15df 48ffc9 dec rcx
0x1417d15e2 4883e1f0 and rcx, 0xfffffffffffffff0
0x1417d15e6 488d1408 lea rdx, [rax + rcx]
0x1417d15ea 4883c210 add rdx, 0x10
0x1417d15ee 488d4db8 lea rcx, [rbp - 0x48]
0x1417d15f2 41ff9030010000 call qword ptr [r8 + 0x130]
0x1417d15f9 488b4610 mov rax, qword ptr [rsi + 0x10]
0x1417d15fd 4c8b4618 mov r8, qword ptr [rsi + 0x18]
0x1417d1601 498b4810 mov rcx, qword ptr [r8 + 0x10]
0x1417d1605 48ffc9 dec rcx
0x1417d1608 4883e1f0 and rcx, 0xfffffffffffffff0
0x1417d160c 488d1408 lea rdx, [rax + rcx]
0x1417d1610 4883c210 add rdx, 0x10
0x1417d1614 488d4da0 lea rcx, [rbp - 0x60]
0x1417d1618 41ff9030010000 call qword ptr [r8 + 0x130]
0x1417d161f 837db805 cmp dword ptr [rbp - 0x48], 5
0x1417d1623 0f85be010000 jne 0x1417d17e7
0x1417d1629 48837da005 cmp qword ptr [rbp - 0x60], 5
0x1417d162e 0f85b3010000 jne 0x1417d17e7
0x1417d1634 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x1417d1638 488b55a8 mov rdx, qword ptr [rbp - 0x58]
0x1417d163c 4d8b7730 mov r14, qword ptr [r15 + 0x30]
0x1417d1640 49c7461000000000 mov qword ptr [r14 + 0x10], 0
0x1417d1648 4d89f0 mov r8, r14
0x1417d164b e850992cff call 0x140a9afa0
0x1417d1650 48c7454000000000 mov qword ptr [rbp + 0x40], 0
0x1417d1658 48c7454808000000 mov qword ptr [rbp + 0x48], 8
0x1417d1660 48c7455000000000 mov qword ptr [rbp + 0x50], 0
0x1417d1668 498b4e08 mov rcx, qword ptr [r14 + 8]
0x1417d166c 498b5610 mov rdx, qword ptr [r14 + 0x10]
0x1417d1670 4d8b4f38 mov r9, qword ptr [r15 + 0x38]
0x1417d1674 c6456701 mov byte ptr [rbp + 0x67], 1
0x1417d1678 4c8d4540 lea r8, [rbp + 0x40]
0x1417d167c e8cf1112ff call 0x1408f2850
0x1417d1681 48837d5000 cmp qword ptr [rbp + 0x50], 0
0x1417d1686 0f843b010000 je 0x1417d17c7
0x1417d168c 4c8b03 mov r8, qword ptr [rbx]
0x1417d168f 488b4f10 mov rcx, qword ptr [rdi + 0x10]
0x1417d1693 4489c2 mov edx, r8d
0x1417d1696 4839d1 cmp rcx, rdx
0x1417d1699 0f8668010000 jbe 0x1417d1807
0x1417d169f 488b4708 mov rax, qword ptr [rdi + 8]
0x1417d16a3 4885c0 test rax, rax
0x1417d16a6 0f845b010000 je 0x1417d1807
0x1417d16ac 49c1e820 shr r8, 0x20
0x1417d16b0 4183c801 or r8d, 1
0x1417d16b4 488d1492 lea rdx, [rdx + rdx*4]
0x1417d16b8 443944d020 cmp dword ptr [rax + rdx*8 + 0x20], r8d
0x1417d16bd 0f8544010000 jne 0x1417d1807
0x1417d16c3 4c8b06 mov r8, qword ptr [rsi]
0x1417d16c6 4589c1 mov r9d, r8d
0x1417d16c9 4c39c9 cmp rcx, r9
0x1417d16cc 0f8653010000 jbe 0x1417d1825
0x1417d16d2 49c1e820 shr r8, 0x20
0x1417d16d6 4183c801 or r8d, 1
0x1417d16da 4f8d0c89 lea r9, [r9 + r9*4]
0x1417d16de 463944c820 cmp dword ptr [rax + r9*8 + 0x20], r8d
0x1417d16e3 0f853c010000 jne 0x1417d1825
0x1417d16e9 488d0cd0 lea rcx, [rax + rdx*8]
0x1417d16ed 440fb601 movzx r8d, byte ptr [rcx]
0x1417d16f1 488b4908 mov rcx, qword ptr [rcx + 8]
0x1417d16f5 4a8d04c8 lea rax, [rax + r9*8]
0x1417d16f9 0fb610 movzx edx, byte ptr [rax]
0x1417d16fc 488b4008 mov rax, qword ptr [rax + 8]
0x1417d1700 498b7740 mov rsi, qword ptr [r15 + 0x40]
0x1417d1704 458b4c2408 mov r9d, dword ptr [r12 + 8]
0x1417d1709 44894d38 mov dword ptr [rbp + 0x38], r9d
0x1417d170d 4d8b0c24 mov r9, qword ptr [r12]
0x1417d1711 4c894d30 mov qword ptr [rbp + 0x30], r9
0x1417d1715 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
0x1417d1719 4c894de0 mov qword ptr [rbp - 0x20], r9
0x1417d171d f30f6f4540 movdqu xmm0, xmmword ptr [rbp + 0x40]
0x1417d1722 660f7f45d0 movdqa xmmword ptr [rbp - 0x30], xmm0
0x1417d1727 41f6c0fd test r8b, 0xfd
0x1417d172b 0f8512010000 jne 0x1417d1843
0x1417d1731 f6c2fd test dl, 0xfd
0x1417d1734 0f8523010000 jne 0x1417d185d
0x1417d173a 8b5538 mov edx, dword ptr [rbp + 0x38]
0x1417d173d 895520 mov dword ptr [rbp + 0x20], edx
0x1417d1740 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1417d1744 48895518 mov qword ptr [rbp + 0x18], rdx
0x1417d1748 488b5558 mov rdx, qword ptr [rbp + 0x58]
0x1417d174c 48895524 mov qword ptr [rbp + 0x24], rdx
0x1417d1750 8b5560 mov edx, dword ptr [rbp + 0x60]
0x1417d1753 89552c mov dword ptr [rbp + 0x2c], edx
0x1417d1756 48894d08 mov qword ptr [rbp + 8], rcx
0x1417d175a 48894510 mov qword ptr [rbp + 0x10], rax
0x1417d175e f30f6f4540 movdqu xmm0, xmmword ptr [rbp + 0x40]
0x1417d1763 660f7f45f0 movdqa xmmword ptr [rbp - 0x10], xmm0
0x1417d1768 488b4550 mov rax, qword ptr [rbp + 0x50]
0x1417d176c 48894500 mov qword ptr [rbp], rax
0x1417d1770 488b7e10 mov rdi, qword ptr [rsi + 0x10]
0x1417d1774 483b3e cmp rdi, qword ptr [rsi]
0x1417d1777 750f jne 0x1417d1788
0x1417d1779 488d1528335101 lea rdx, [rip + 0x1513328]
0x1417d1780 4889f1 mov rcx, rsi
0x1417d1783 e82810d9fe call 0x1405627b0
0x1417d1788 488b4608 mov rax, qword ptr [rsi + 8]
0x1417d178c 4889f9 mov rcx, rdi
0x1417d178f 48c1e106 shl rcx, 6
0x1417d1793 660f6f45f0 movdqa xmm0, xmmword ptr [rbp - 0x10]
0x1417d1798 660f6f4d00 movdqa xmm1, xmmword ptr [rbp]
0x1417d179d 660f6f5510 movdqa xmm2, xmmword ptr [rbp + 0x10]
0x1417d17a2 660f6f5d20 movdqa xmm3, xmmword ptr [rbp + 0x20]
0x1417d17a7 f30f7f5c0830 movdqu xmmword ptr [rax + rcx + 0x30], xmm3
0x1417d17ad f30f7f540820 movdqu xmmword ptr [rax + rcx + 0x20], xmm2
0x1417d17b3 f30f7f4c0810 movdqu xmmword ptr [rax + rcx + 0x10], xmm1
0x1417d17b9 f30f7f0408 movdqu xmmword ptr [rax + rcx], xmm0
0x1417d17be 48ffc7 inc rdi
0x1417d17c1 48897e10 mov qword ptr [rsi + 0x10], rdi
0x1417d17c5 eb20 jmp 0x1417d17e7
0x1417d17c7 488b4540 mov rax, qword ptr [rbp + 0x40]
0x1417d17cb 4885c0 test rax, rax
0x1417d17ce 7417 je 0x1417d17e7
0x1417d17d0 488b4d48 mov rcx, qword ptr [rbp + 0x48]
0x1417d17d4 48c1e003 shl rax, 3
0x1417d17d8 488d1480 lea rdx, [rax + rax*4]
0x1417d17dc 41b808000000 mov r8d, 8
0x1417d17e2 e83924e4fe call 0x140613c20
0x1417d17e7 b001 mov al, 1
0x1417d17e9 4881c4f0000000 add rsp, 0xf0
0x1417d17f0 5b pop rbx
0x1417d17f1 5f pop rdi
0x1417d17f2 5e pop rsi
0x1417d17f3 415c pop r12
0x1417d17f5 415e pop r14
0x1417d17f7 415f pop r15
0x1417d17f9 5d pop rbp
0x1417d17fa c3 ret
0x1417d17fb 488d0d5e325101 lea rcx, [rip + 0x151325e]
0x1417d1802 e8097b1001 call 0x1428d9310
0x1417d1807 c6456701 mov byte ptr [rbp + 0x67], 1
0x1417d180b 488d0dcec04901 lea rcx, [rip + 0x149c0ce]
0x1417d1812 4c8d055f325101 lea r8, [rip + 0x151325f]
0x1417d1819 ba18000000 mov edx, 0x18
0x1417d181e e8026f1001 call 0x1428d8725
0x1417d1823 eb50 jmp 0x1417d1875
0x1417d1825 c6456701 mov byte ptr [rbp + 0x67], 1
0x1417d1829 488d0db0c04901 lea rcx, [rip + 0x149c0b0]
0x1417d1830 4c8d0559325101 lea r8, [rip + 0x1513259]
0x1417d1837 ba18000000 mov edx, 0x18
0x1417d183c e8e46e1001 call 0x1428d8725
0x1417d1841 eb32 jmp 0x1417d1875
0x1417d1843 488d0d76325101 lea rcx, [rip + 0x1513276]
0x1417d184a 4c8d05af325101 lea r8, [rip + 0x15132af]
0x1417d1851 ba28000000 mov edx, 0x28
0x1417d1856 e8d57b1001 call 0x1428d9430
0x1417d185b eb18 jmp 0x1417d1875
0x1417d185d 488d0d5c325101 lea rcx, [rip + 0x151325c]
0x1417d1864 4c8d057d325101 lea r8, [rip + 0x151327d]
0x1417d186b ba28000000 mov edx, 0x28
0x1417d1870 e8bb7b1001 call 0x1428d9430
0x1417d1875 0f0b ud2
0x1417d1877 660f1f840000000000 nop word ptr [rax + rax]
0x1417d1880 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1417d1885 55 push rbp
0x1417d1886 4157 push r15
0x1417d1888 4156 push r14
0x1417d188a 4154 push r12
0x1417d188c 56 push rsi
0x1417d188d 57 push rdi
0x1417d188e 53 push rbx
0x1417d188f 4883ec20 sub rsp, 0x20
0x1417d1893 488daa80000000 lea rbp, [rdx + 0x80]
0x1417d189a 488d4dd0 lea rcx, [rbp - 0x30]
0x1417d189e e8ad7b0aff call 0x140879450
0x1417d18a3 c6456700 mov byte ptr [rbp + 0x67], 0
0x1417d18a7 4883c420 add rsp, 0x20
0x1417d18ab 5b pop rbx
0x1417d18ac 5f pop rdi
0x1417d18ad 5e pop rsi
0x1417d18ae 415c pop r12
0x1417d18b0 415e pop r14
0x1417d18b2 415f pop r15
0x1417d18b4 5d pop rbp
0x1417d18b5 c3 ret
0x1417d18b6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1417d18c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1417d18c5 55 push rbp
0x1417d18c6 4157 push r15
0x1417d18c8 4156 push r14
0x1417d18ca 4154 push r12
0x1417d18cc 56 push rsi
0x1417d18cd 57 push rdi
0x1417d18ce 53 push rbx
0x1417d18cf 4883ec20 sub rsp, 0x20
0x1417d18d3 488daa80000000 lea rbp, [rdx + 0x80]
0x1417d18da 807d6700 cmp byte ptr [rbp + 0x67], 0
0x1417d18de 750f jne 0x1417d18ef
0x1417d18e0 4883c420 add rsp, 0x20
0x1417d18e4 5b pop rbx
0x1417d18e5 5f pop rdi
0x1417d18e6 5e pop rsi
0x1417d18e7 415c pop r12
0x1417d18e9 415e pop r14
0x1417d18eb 415f pop r15
0x1417d18ed 5d pop rbp
0x1417d18ee c3 ret
0x1417d18ef 488d4d40 lea rcx, [rbp + 0x40]
0x1417d18f3 e8587b0aff call 0x140879450
0x1417d18f8 ebe6 jmp 0x1417d18e0
0x1417d18fa 660f1f440000 nop word ptr [rax + rax]
0x1417d1900 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1417d1905 55 push rbp
0x1417d1906 4157 push r15
0x1417d1908 4156 push r14
0x1417d190a 4154 push r12
0x1417d190c 56 push rsi
0x1417d190d 57 push rdi
0x1417d190e 53 push rbx
0x1417d190f 4883ec20 sub rsp, 0x20
0x1417d1913 488daa80000000 lea rbp, [rdx + 0x80]
0x1417d191a c6456700 mov byte ptr [rbp + 0x67], 0
0x1417d191e 488d4df0 lea rcx, [rbp - 0x10]
0x1417d1922 e8297b0aff call 0x140879450
0x1417d1927 c6456700 mov byte ptr [rbp + 0x67], 0
0x1417d192b 4883c420 add rsp, 0x20
0x1417d192f 5b pop rbx
0x1417d1930 5f pop rdi
0x1417d1931 5e pop rsi
0x1417d1932 415c pop r12
0x1417d1934 415e pop r14
0x1417d1936 415f pop r15
0x1417d1938 5d pop rbp
0x1417d1939 c3 ret
