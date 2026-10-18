1414e1740: add al, byte ptr [rax]
1414e1742: add byte ptr [rbx - 0x75], cl
1414e1745: cmp al, 0xd3
1414e1747: test rdi, rdi
1414e174a: je 0x1414e1963
1414e1750: mov r10, qword ptr [rdx + 0x1e0]
1414e1757: not rdi
1414e175a: lea rdx, [rdi + rdi*2]
1414e175e: shl rdx, 5
1414e1762: cmp qword ptr [r10 + rdx + 0x20], 0
1414e1768: je 0x1414e1963
1414e176e: mov rdi, qword ptr [rcx + 0x18]
1414e1772: cmp r9, rdi
1414e1775: jbe 0x1414e1981
1414e177b: mov r9, qword ptr [r11 + rdi*8]
1414e177f: test r9, r9
1414e1782: je 0x1414e1981
1414e1788: not r9
1414e178b: lea r11, [r9 + r9*2]
1414e178f: shl r11, 5
1414e1793: cmp qword ptr [r10 + r11 + 0x20], 0
1414e1799: je 0x1414e1981
1414e179f: mov edi, dword ptr [rcx + 0x158]
1414e17a5: add rdx, r10
1414e17a8: mov r9, qword ptr [rdx + 0x10]
1414e17ac: add r11, r10
1414e17af: lea rbx, [rdx + 0x5c]
1414e17b3: add rdx, 0x58
1414e17b7: lea r10, [r8 + 0x5c]
1414e17bb: add r8, 0x58
1414e17bf: mov r14, qword ptr [r11 + 0x10]
1414e17c3: lea r15, [r11 + 0x58]
1414e17c7: add r11, 0x5c
1414e17cb: mov qword ptr [rsp + 0x60], rax
1414e17d0: mov qword ptr [rsp + 0x68], r8
1414e17d5: mov qword ptr [rsp + 0x70], r10
1414e17da: mov dword ptr [rsp + 0x78], edi
1414e17de: mov dword ptr [rsp + 0x7c], esi
1414e17e2: mov qword ptr [rsp + 0x80], r9
1414e17ea: mov qword ptr [rsp + 0x88], rdx
1414e17f2: mov qword ptr [rsp + 0x90], rbx
1414e17fa: mov dword ptr [rsp + 0x98], edi
1414e1801: mov dword ptr [rsp + 0x9c], esi
1414e1808: mov qword ptr [rsp + 0xa0], r14
1414e1810: mov qword ptr [rsp + 0xa8], r15
1414e1818: mov qword ptr [rsp + 0xb0], r11
1414e1820: mov dword ptr [rsp + 0xb8], edi
1414e1827: mov dword ptr [rsp + 0xbc], esi
1414e182e: cmp dword ptr [r14], 0xd
1414e1832: je 0x1414e1868
1414e1834: cmp byte ptr [r9 + 0x18], 2
1414e1839: jb 0x1414e1868
1414e183b: mov rdx, qword ptr [r9 + 0x20]
1414e183f: test byte ptr [rdx + 0xb9], 1
1414e1846: je 0x1414e1868
1414e1848: cmp byte ptr [r9 + 0x19], 3
1414e184d: jne 0x1414e1868
1414e184f: lea rdx, [rsp + 0xa0]
1414e1857: mov dword ptr [r10], esi
1414e185a: mov rdi, rcx
1414e185d: mov rcx, rax
1414e1860: call 0x140a86270
1414e1865: mov rcx, rdi
1414e1868: mov dword ptr [rcx + 0x158], esi
1414e186e: xor eax, eax
1414e1870: add rsp, 0xc0
1414e1877: pop rbx
1414e1878: pop rdi
1414e1879: pop rsi
1414e187a: pop r14
1414e187c: pop r15
1414e187e: ret 
1414e187f: lea rax, [rip + 0x17760da]
1414e1886: mov qword ptr [rsp + 0x60], rax
1414e188b: mov qword ptr [rsp + 0x68], 1
1414e1894: mov qword ptr [rsp + 0x70], 8
1414e189d: xorps xmm0, xmm0
1414e18a0: movups xmmword ptr [rsp + 0x78], xmm0
1414e18a5: add rcx, 0x20
1414e18a9: lea rax, [rip + 0x17760c0]
1414e18b0: mov qword ptr [rsp + 0x20], rax
1414e18b5: lea r8, [rsp + 0x40]
1414e18ba: lea r9, [rsp + 0x60]
