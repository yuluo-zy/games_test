1415f20d0: sub rsp, 0x58
1415f20d4: cmp byte ptr [rcx], 6
1415f20d7: je 0x1415f20fa
1415f20d9: movups xmm0, xmmword ptr [rcx]
1415f20dc: movups xmm1, xmmword ptr [rcx + 0x10]
1415f20e0: movaps xmmword ptr [rsp + 0x30], xmm1
1415f20e5: movaps xmmword ptr [rsp + 0x20], xmm0
1415f20ea: lea rcx, [rsp + 0x20]
1415f20ef: call 0x140b5e020
1415f20f4: nop 
1415f20f5: add rsp, 0x58
1415f20f9: ret 
1415f20fa: mov byte ptr [rsp + 0x40], 0xd
1415f20ff: lea rdx, [rip + 0x16881a2]
1415f2106: lea r8, [rip + 0x1687f53]
1415f210d: lea rcx, [rsp + 0x40]
1415f2112: call 0x1428c9a00
1415f2117: mov rdx, rax
1415f211a: mov eax, 1
1415f211f: add rsp, 0x58
1415f2123: ret 
1415f2124: int3 
1415f2125: int3 
1415f2126: int3 
1415f2127: int3 
1415f2128: int3 
1415f2129: int3 
1415f212a: int3 
1415f212b: int3 
1415f212c: int3 
1415f212d: int3 
1415f212e: int3 
1415f212f: int3 
1415f2130: push rsi
1415f2131: sub rsp, 0x60
1415f2135: mov rsi, rcx
1415f2138: cmp byte ptr [rdx], 6
1415f213b: je 0x1415f2177
1415f213d: movups xmm0, xmmword ptr [rdx]
1415f2140: movups xmm1, xmmword ptr [rdx + 0x10]
1415f2144: movaps xmmword ptr [rsp + 0x50], xmm1
1415f2149: movaps xmmword ptr [rsp + 0x40], xmm0
1415f214e: lea rcx, [rsp + 0x28]
1415f2153: lea rdx, [rsp + 0x40]
1415f2158: call 0x1416970f0
1415f215d: mov rcx, qword ptr [rsp + 0x28]
1415f2162: mov rax, qword ptr [rsp + 0x30]
1415f2167: mov qword ptr [rsi + 8], rax
1415f216b: mov qword ptr [rsi], rcx
1415f216e: mov rax, rsi
1415f2171: add rsp, 0x60
1415f2175: pop rsi
1415f2176: ret 
1415f2177: mov byte ptr [rsp + 0x28], 0xd
1415f217c: lea rdx, [rip + 0x1688125]
1415f2183: lea r8, [rip + 0x1687ed6]
1415f218a: lea rcx, [rsp + 0x28]
1415f218f: call 0x1428c9a00
1415f2194: xor ecx, ecx
1415f2196: jmp 0x1415f2167
1415f2198: int3 
1415f2199: int3 
1415f219a: int3 
1415f219b: int3 
1415f219c: int3 
1415f219d: int3 
1415f219e: int3 
1415f219f: int3 
1415f21a0: sub rsp, 0x58
1415f21a4: cmp byte ptr [rcx], 6
1415f21a7: je 0x1415f21ca
1415f21a9: movups xmm0, xmmword ptr [rcx]
