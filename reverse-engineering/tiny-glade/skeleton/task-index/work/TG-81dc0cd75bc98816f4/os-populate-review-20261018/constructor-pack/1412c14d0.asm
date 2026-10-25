1412c14d0: push rbp
1412c14d1: push rsi
1412c14d2: sub rsp, 0xd8
1412c14d9: lea rbp, [rsp + 0x80]
1412c14e1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c14e9: mov rsi, rcx
1412c14ec: lea rcx, [rbp - 0x58]
1412c14f0: xor r8d, r8d
1412c14f3: call 0x141615bb0
1412c14f8: mov qword ptr [rbp - 0x60], 0
1412c1500: movzx eax, byte ptr [rip + 0x2373702]
1412c1507: mov ecx, 0x30
1412c150c: mov edx, 8
1412c1511: call 0x140613c10
1412c1516: test rax, rax
1412c1519: je 0x1412c15cf
1412c151f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c1523: movups xmm1, xmmword ptr [rbp - 0x50]
1412c1527: movups xmm2, xmmword ptr [rbp - 0x40]
1412c152b: movups xmmword ptr [rax + 0x20], xmm2
1412c152f: movups xmmword ptr [rax + 0x10], xmm1
1412c1533: movups xmmword ptr [rax], xmm0
1412c1536: lea rcx, [rip + 0x17dbdc1]
1412c153d: mov qword ptr [rbp], rcx
1412c1541: mov qword ptr [rbp + 8], 0x35
1412c1549: mov qword ptr [rbp + 0x10], rax
1412c154d: lea rax, [rip + 0x196b764]
1412c1554: mov qword ptr [rbp + 0x18], rax
1412c1558: mov qword ptr [rbp + 0x30], 0
1412c1560: mov byte ptr [rbp + 0x46], 5
1412c1564: mov qword ptr [rbp + 0x20], rcx
1412c1568: mov qword ptr [rbp + 0x28], 0x35
1412c1570: movzx eax, byte ptr [rip + 0x2373692]
1412c1577: mov ecx, 0x40
1412c157c: mov edx, 8
1412c1581: call 0x140613c10
1412c1586: test rax, rax
1412c1589: je 0x1412c15e0
1412c158b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c158f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c1593: movups xmm2, xmmword ptr [rbp + 0x30]
1412c1597: movups xmm3, xmmword ptr [rbp + 0x40]
1412c159b: movups xmmword ptr [rax + 0x30], xmm3
1412c159f: movups xmmword ptr [rax + 0x20], xmm2
1412c15a3: movups xmmword ptr [rax + 0x10], xmm1
1412c15a7: movups xmmword ptr [rax], xmm0
1412c15aa: movabs rdx, 0x8b2da1ef1bff9619
1412c15b4: mov rcx, rsi
1412c15b7: mov r8, rax
1412c15ba: call 0x140969bf0
1412c15bf: cmp rax, 1
1412c15c3: je 0x1412c15f1
1412c15c5: add rsp, 0xd8
1412c15cc: pop rsi
1412c15cd: pop rbp
1412c15ce: ret 
1412c15cf: mov ecx, 8
1412c15d4: mov edx, 0x30
1412c15d9: call 0x1428d9000
1412c15de: jmp 0x1412c15ef
1412c15e0: mov ecx, 8
1412c15e5: mov edx, 0x40
1412c15ea: call 0x1428d9000
1412c15ef: ud2 
1412c15f1: mov rax, rdx
1412c15f4: add rdx, 0x10
1412c15f8: mov rsi, rdx
1412c15fb: cmp qword ptr [rax + 0x18], 0x35
1412c1600: jne 0x1412c167e
1412c1602: mov rdx, qword ptr [rsi]
1412c1605: lea rcx, [rip + 0x17dbcf2]
1412c160c: mov r8d, 0x35
1412c1612: call 0x142923bf0
1412c1617: test eax, eax
1412c1619: jne 0x1412c167e
1412c161b: lea rax, [rip + 0x196c446]
1412c1622: mov qword ptr [rbp - 0x30], rax
1412c1626: lea rax, [rip - 0xd8dd8d]
1412c162d: mov qword ptr [rbp - 0x28], rax
1412c1631: mov rax, rbp
1412c1634: mov qword ptr [rbp - 0x20], rax
1412c1638: lea rax, [rip - 0x11b77af]
1412c163f: mov qword ptr [rbp - 0x18], rax
1412c1643: lea rax, [rip + 0x196bfde]
1412c164a: mov qword ptr [rbp + 0x10], rax
1412c164e: mov qword ptr [rbp + 0x18], 3
1412c1656: mov qword ptr [rbp + 0x30], 0
1412c165e: lea rax, [rbp - 0x30]
1412c1662: mov qword ptr [rbp + 0x20], rax
1412c1666: mov qword ptr [rbp + 0x28], 2
1412c166e: lea rdx, [rip + 0x196bfe3]
1412c1675: lea rcx, [rbp + 0x10]
1412c1679: call 0x1428d9390
1412c167e: lea rax, [rip + 0x196c3e3]
1412c1685: mov qword ptr [rbp + 0x10], rax
1412c1689: lea rax, [rip - 0xd8ddf0]
1412c1690: mov qword ptr [rbp + 0x18], rax
1412c1694: mov qword ptr [rbp + 0x20], rsi
1412c1698: lea rax, [rip - 0x11b780f]
1412c169f: mov qword ptr [rbp + 0x28], rax
1412c16a3: mov rcx, rbp
1412c16a6: mov qword ptr [rbp + 0x30], rcx
1412c16aa: mov qword ptr [rbp + 0x38], rax
1412c16ae: lea rax, [rip + 0x196beb3]
1412c16b5: mov qword ptr [rbp - 0x30], rax
1412c16b9: mov qword ptr [rbp - 0x28], 3
1412c16c1: mov qword ptr [rbp - 0x10], 0
1412c16c9: lea rax, [rbp + 0x10]
1412c16cd: mov qword ptr [rbp - 0x20], rax
1412c16d1: mov qword ptr [rbp - 0x18], 3
1412c16d9: lea rdx, [rip + 0x196bef0]
1412c16e0: lea rcx, [rbp - 0x30]
1412c16e4: call 0x1428d9390
1412c16e9: int3 
1412c16ea: nop word ptr [rax + rax]
1412c16f0: mov qword ptr [rsp + 0x10], rdx
1412c16f5: push rbp
1412c16f6: push rsi
1412c16f7: sub rsp, 0x28
1412c16fb: lea rbp, [rdx + 0x80]
1412c1702: lea rcx, [rbp - 0x58]
1412c1706: call 0x140be24b0
1412c170b: nop 
1412c170c: add rsp, 0x28
1412c1710: pop rsi
1412c1711: pop rbp
1412c1712: ret 
1412c1713: nop word ptr cs:[rax + rax]
1412c1720: mov qword ptr [rsp + 0x10], rdx
1412c1725: push rbp
1412c1726: push rsi
1412c1727: sub rsp, 0x28
1412c172b: lea rbp, [rdx + 0x80]
1412c1732: lea rcx, [rbp + 0x10]
1412c1736: call 0x1400c04b0
1412c173b: nop 
1412c173c: add rsp, 0x28
1412c1740: pop rsi
1412c1741: pop rbp
1412c1742: ret 
