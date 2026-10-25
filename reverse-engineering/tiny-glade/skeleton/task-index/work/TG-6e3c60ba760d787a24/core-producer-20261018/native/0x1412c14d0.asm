1412c14d0: 55 push rbp
1412c14d1: 56 push rsi
1412c14d2: 4881ecd8000000 sub rsp, 0xd8
1412c14d9: 488dac2480000000 lea rbp, [rsp + 0x80]
1412c14e1: 48c74550feffffff mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c14e9: 4889ce mov rsi, rcx
1412c14ec: 488d4da8 lea rcx, [rbp - 0x58]
1412c14f0: 4531c0 xor r8d, r8d
1412c14f3: e8b8463500 call 0x141615bb0
1412c14f8: 48c745a000000000 mov qword ptr [rbp - 0x60], 0
1412c1500: 0fb60502373702 movzx eax, byte ptr [rip + 0x2373702]
1412c1507: b930000000 mov ecx, 0x30
1412c150c: ba08000000 mov edx, 8
1412c1511: e8fa2635ff call 0x140613c10
1412c1516: 4885c0 test rax, rax
1412c1519: 0f84b0000000 je 0x1412c15cf
1412c151f: 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
1412c1523: 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
1412c1527: 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
1412c152b: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
1412c152f: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1412c1533: 0f1100 movups xmmword ptr [rax], xmm0
1412c1536: 488d0dc1bd7d01 lea rcx, [rip + 0x17dbdc1]
1412c153d: 48894d00 mov qword ptr [rbp], rcx
1412c1541: 48c7450835000000 mov qword ptr [rbp + 8], 0x35
1412c1549: 48894510 mov qword ptr [rbp + 0x10], rax
1412c154d: 488d0564b79601 lea rax, [rip + 0x196b764]
1412c1554: 48894518 mov qword ptr [rbp + 0x18], rax
1412c1558: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
1412c1560: c6454605 mov byte ptr [rbp + 0x46], 5
1412c1564: 48894d20 mov qword ptr [rbp + 0x20], rcx
1412c1568: 48c7452835000000 mov qword ptr [rbp + 0x28], 0x35
1412c1570: 0fb60592363702 movzx eax, byte ptr [rip + 0x2373692]
1412c1577: b940000000 mov ecx, 0x40
1412c157c: ba08000000 mov edx, 8
1412c1581: e88a2635ff call 0x140613c10
1412c1586: 4885c0 test rax, rax
1412c1589: 7455 je 0x1412c15e0
1412c158b: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
1412c158f: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
1412c1593: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
1412c1597: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
1412c159b: 0f115830 movups xmmword ptr [rax + 0x30], xmm3
1412c159f: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
1412c15a3: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1412c15a7: 0f1100 movups xmmword ptr [rax], xmm0
1412c15aa: 48ba1996ff1befa12d8b movabs rdx, 0x8b2da1ef1bff9619
1412c15b4: 4889f1 mov rcx, rsi
1412c15b7: 4989c0 mov r8, rax
1412c15ba: e831866aff call 0x140969bf0
1412c15bf: 4883f801 cmp rax, 1
1412c15c3: 742c je 0x1412c15f1
1412c15c5: 4881c4d8000000 add rsp, 0xd8
1412c15cc: 5e pop rsi
1412c15cd: 5d pop rbp
1412c15ce: c3 ret 
1412c15cf: b908000000 mov ecx, 8
1412c15d4: ba30000000 mov edx, 0x30
1412c15d9: e8227a6101 call 0x1428d9000
1412c15de: eb0f jmp 0x1412c15ef
1412c15e0: b908000000 mov ecx, 8
1412c15e5: ba40000000 mov edx, 0x40
1412c15ea: e8117a6101 call 0x1428d9000
1412c15ef: 0f0b ud2 
1412c15f1: 4889d0 mov rax, rdx
1412c15f4: 4883c210 add rdx, 0x10
1412c15f8: 4889d6 mov rsi, rdx
1412c15fb: 4883781835 cmp qword ptr [rax + 0x18], 0x35
1412c1600: 757c jne 0x1412c167e
1412c1602: 488b16 mov rdx, qword ptr [rsi]
1412c1605: 488d0df2bc7d01 lea rcx, [rip + 0x17dbcf2]
1412c160c: 41b835000000 mov r8d, 0x35
1412c1612: e8d9256601 call 0x142923bf0
1412c1617: 85c0 test eax, eax
1412c1619: 7563 jne 0x1412c167e
1412c161b: 488d0546c49601 lea rax, [rip + 0x196c446]
1412c1622: 488945d0 mov qword ptr [rbp - 0x30], rax
1412c1626: 488d05732227ff lea rax, [rip - 0xd8dd8d]
1412c162d: 488945d8 mov qword ptr [rbp - 0x28], rax
1412c1631: 4889e8 mov rax, rbp
1412c1634: 488945e0 mov qword ptr [rbp - 0x20], rax
1412c1638: 488d055188e4fe lea rax, [rip - 0x11b77af]
1412c163f: 488945e8 mov qword ptr [rbp - 0x18], rax
1412c1643: 488d05debf9601 lea rax, [rip + 0x196bfde]
1412c164a: 48894510 mov qword ptr [rbp + 0x10], rax
1412c164e: 48c7451803000000 mov qword ptr [rbp + 0x18], 3
1412c1656: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
1412c165e: 488d45d0 lea rax, [rbp - 0x30]
1412c1662: 48894520 mov qword ptr [rbp + 0x20], rax
1412c1666: 48c7452802000000 mov qword ptr [rbp + 0x28], 2
1412c166e: 488d15e3bf9601 lea rdx, [rip + 0x196bfe3]
1412c1675: 488d4d10 lea rcx, [rbp + 0x10]
1412c1679: e8127d6101 call 0x1428d9390
1412c167e: 488d05e3c39601 lea rax, [rip + 0x196c3e3]
1412c1685: 48894510 mov qword ptr [rbp + 0x10], rax
1412c1689: 488d05102227ff lea rax, [rip - 0xd8ddf0]
1412c1690: 48894518 mov qword ptr [rbp + 0x18], rax
1412c1694: 48897520 mov qword ptr [rbp + 0x20], rsi
1412c1698: 488d05f187e4fe lea rax, [rip - 0x11b780f]
1412c169f: 48894528 mov qword ptr [rbp + 0x28], rax
1412c16a3: 4889e9 mov rcx, rbp
1412c16a6: 48894d30 mov qword ptr [rbp + 0x30], rcx
1412c16aa: 48894538 mov qword ptr [rbp + 0x38], rax
1412c16ae: 488d05b3be9601 lea rax, [rip + 0x196beb3]
1412c16b5: 488945d0 mov qword ptr [rbp - 0x30], rax
1412c16b9: 48c745d803000000 mov qword ptr [rbp - 0x28], 3
1412c16c1: 48c745f000000000 mov qword ptr [rbp - 0x10], 0
1412c16c9: 488d4510 lea rax, [rbp + 0x10]
1412c16cd: 488945e0 mov qword ptr [rbp - 0x20], rax
1412c16d1: 48c745e803000000 mov qword ptr [rbp - 0x18], 3
1412c16d9: 488d15f0be9601 lea rdx, [rip + 0x196bef0]
1412c16e0: 488d4dd0 lea rcx, [rbp - 0x30]
1412c16e4: e8a77c6101 call 0x1428d9390
1412c16e9: cc int3 
1412c16ea: 660f1f440000 nop word ptr [rax + rax]
1412c16f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412c16f5: 55 push rbp
1412c16f6: 56 push rsi
1412c16f7: 4883ec28 sub rsp, 0x28
1412c16fb: 488daa80000000 lea rbp, [rdx + 0x80]
1412c1702: 488d4da8 lea rcx, [rbp - 0x58]
1412c1706: e8a50d92ff call 0x140be24b0
1412c170b: 90 nop 
1412c170c: 4883c428 add rsp, 0x28
1412c1710: 5e pop rsi
1412c1711: 5d pop rbp
1412c1712: c3 ret 
1412c1713: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1412c1720: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412c1725: 55 push rbp
1412c1726: 56 push rsi
1412c1727: 4883ec28 sub rsp, 0x28
1412c172b: 488daa80000000 lea rbp, [rdx + 0x80]
1412c1732: 488d4d10 lea rcx, [rbp + 0x10]
1412c1736: e875eddffe call 0x1400c04b0
1412c173b: 90 nop 
1412c173c: 4883c428 add rsp, 0x28
1412c1740: 5e pop rsi
1412c1741: 5d pop rbp
1412c1742: c3 ret 
