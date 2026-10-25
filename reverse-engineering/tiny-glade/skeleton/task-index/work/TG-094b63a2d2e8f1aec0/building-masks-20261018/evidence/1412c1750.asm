1412c1750: push rbp
1412c1751: push rsi
1412c1752: sub rsp, 0xe8
1412c1759: lea rbp, [rsp + 0x80]
1412c1761: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c1769: mov rsi, rcx
1412c176c: lea rcx, [rbp - 0x58]
1412c1770: xor r8d, r8d
1412c1773: call 0x14161c080
1412c1778: movzx eax, byte ptr [rip + 0x237348a]
1412c177f: mov ecx, 0x38
1412c1784: mov edx, 8
1412c1789: call 0x140613c10
1412c178e: test rax, rax
1412c1791: je 0x1412c184f
1412c1797: mov rcx, qword ptr [rbp - 0x28]
1412c179b: mov qword ptr [rax + 0x30], rcx
1412c179f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c17a3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c17a7: movups xmm2, xmmword ptr [rbp - 0x38]
1412c17ab: movups xmmword ptr [rax + 0x20], xmm2
1412c17af: movups xmmword ptr [rax + 0x10], xmm1
1412c17b3: movups xmmword ptr [rax], xmm0
1412c17b6: lea rcx, [rip + 0x17dd134]
1412c17bd: mov qword ptr [rbp + 0x10], rcx
1412c17c1: mov qword ptr [rbp + 0x18], 0x50
1412c17c9: mov qword ptr [rbp + 0x20], rax
1412c17cd: lea rax, [rip + 0x196b51c]
1412c17d4: mov qword ptr [rbp + 0x28], rax
1412c17d8: mov qword ptr [rbp + 0x40], 0
1412c17e0: mov byte ptr [rbp + 0x56], 5
1412c17e4: mov qword ptr [rbp + 0x30], rcx
1412c17e8: mov qword ptr [rbp + 0x38], 0x50
1412c17f0: movzx eax, byte ptr [rip + 0x2373412]
1412c17f7: mov ecx, 0x40
1412c17fc: mov edx, 8
1412c1801: call 0x140613c10
1412c1806: test rax, rax
1412c1809: je 0x1412c1860
1412c180b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c180f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c1813: movups xmm2, xmmword ptr [rbp + 0x40]
1412c1817: movups xmm3, xmmword ptr [rbp + 0x50]
1412c181b: movups xmmword ptr [rax + 0x30], xmm3
1412c181f: movups xmmword ptr [rax + 0x20], xmm2
1412c1823: movups xmmword ptr [rax + 0x10], xmm1
1412c1827: movups xmmword ptr [rax], xmm0
1412c182a: movabs rdx, 0x8dbc70d368e43c84
1412c1834: mov rcx, rsi
1412c1837: mov r8, rax
1412c183a: call 0x140969bf0
1412c183f: cmp rax, 1
1412c1843: je 0x1412c1871
1412c1845: add rsp, 0xe8
1412c184c: pop rsi
1412c184d: pop rbp
1412c184e: ret 
1412c184f: mov ecx, 8
1412c1854: mov edx, 0x38
1412c1859: call 0x1428d9000
1412c185e: jmp 0x1412c186f
1412c1860: mov ecx, 8
1412c1865: mov edx, 0x40
1412c186a: call 0x1428d9000
1412c186f: ud2 
1412c1871: mov rax, rdx
1412c1874: add rdx, 0x10
1412c1878: mov rsi, rdx
1412c187b: cmp qword ptr [rax + 0x18], 0x50
1412c1880: jne 0x1412c18ff
1412c1882: mov rdx, qword ptr [rsi]
1412c1885: lea rcx, [rip + 0x17dd065]
1412c188c: mov r8d, 0x50
1412c1892: call 0x142923bf0
1412c1897: test eax, eax
1412c1899: jne 0x1412c18ff
1412c189b: lea rax, [rip + 0x196c4d6]
1412c18a2: mov qword ptr [rbp - 0x20], rax
1412c18a6: lea rax, [rip - 0xd8e00d]
1412c18ad: mov qword ptr [rbp - 0x18], rax
1412c18b1: lea rax, [rbp + 0x10]
1412c18b5: mov qword ptr [rbp - 0x10], rax
1412c18b9: lea rax, [rip - 0x11b7a30]
1412c18c0: mov qword ptr [rbp - 8], rax
1412c18c4: lea rax, [rip + 0x196bd5d]
1412c18cb: mov qword ptr [rbp + 0x20], rax
1412c18cf: mov qword ptr [rbp + 0x28], 3
1412c18d7: mov qword ptr [rbp + 0x40], 0
1412c18df: lea rax, [rbp - 0x20]
1412c18e3: mov qword ptr [rbp + 0x30], rax
1412c18e7: mov qword ptr [rbp + 0x38], 2
1412c18ef: lea rdx, [rip + 0x196bd62]
1412c18f6: lea rcx, [rbp + 0x20]
1412c18fa: call 0x1428d9390
1412c18ff: lea rax, [rip + 0x196c472]
1412c1906: mov qword ptr [rbp + 0x20], rax
1412c190a: lea rax, [rip - 0xd8e071]
1412c1911: mov qword ptr [rbp + 0x28], rax
1412c1915: mov qword ptr [rbp + 0x30], rsi
1412c1919: lea rax, [rip - 0x11b7a90]
1412c1920: mov qword ptr [rbp + 0x38], rax
1412c1924: lea rcx, [rbp + 0x10]
1412c1928: mov qword ptr [rbp + 0x40], rcx
1412c192c: mov qword ptr [rbp + 0x48], rax
1412c1930: lea rax, [rip + 0x196bc31]
1412c1937: mov qword ptr [rbp - 0x20], rax
1412c193b: mov qword ptr [rbp - 0x18], 3
1412c1943: mov qword ptr [rbp], 0
1412c194b: lea rax, [rbp + 0x20]
1412c194f: mov qword ptr [rbp - 0x10], rax
1412c1953: mov qword ptr [rbp - 8], 3
1412c195b: lea rdx, [rip + 0x196bc6e]
1412c1962: lea rcx, [rbp - 0x20]
1412c1966: call 0x1428d9390
1412c196b: int3 
1412c196c: nop dword ptr [rax]
1412c1970: mov qword ptr [rsp + 0x10], rdx
1412c1975: push rbp
1412c1976: push rsi
1412c1977: sub rsp, 0x28
1412c197b: lea rbp, [rdx + 0x80]
1412c1982: lea rcx, [rbp - 0x48]
1412c1986: call 0x140be24b0
1412c198b: nop 
1412c198c: add rsp, 0x28
1412c1990: pop rsi
1412c1991: pop rbp
1412c1992: ret 
1412c1993: nop word ptr cs:[rax + rax]
1412c19a0: mov qword ptr [rsp + 0x10], rdx
1412c19a5: push rbp
1412c19a6: push rsi
1412c19a7: sub rsp, 0x28
1412c19ab: lea rbp, [rdx + 0x80]
1412c19b2: lea rcx, [rbp + 0x20]
1412c19b6: call 0x1400c04b0
1412c19bb: nop 
1412c19bc: add rsp, 0x28
1412c19c0: pop rsi
1412c19c1: pop rbp
1412c19c2: ret 
