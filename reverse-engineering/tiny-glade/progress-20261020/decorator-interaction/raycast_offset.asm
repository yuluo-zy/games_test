0x141278050: push r15
0x141278052: push r14
0x141278054: push rsi
0x141278055: push rdi
0x141278056: push rbp
0x141278057: push rbx
0x141278058: sub rsp, 0x138
0x14127805f: movaps xmmword ptr [rsp + 0x120], xmm7
0x141278067: movaps xmmword ptr [rsp + 0x110], xmm6
0x14127806f: movaps xmm6, xmm2
0x141278072: movaps xmm7, xmm1
0x141278075: mov rsi, rcx
0x141278078: mov rdi, qword ptr [rsp + 0x1a8]
0x141278080: movzx ebp, byte ptr [rsp + 0x198]
0x141278088: mov rbx, qword ptr [rsp + 0x1a0]
0x141278090: addss xmm7, xmm3
0x141278094: addss xmm6, dword ptr [rsp + 0x190]
0x14127809d: mov r14, qword ptr [rbx]
0x1412780a0: mov rcx, qword ptr [rbx + 0x20]
0x1412780a4: call 0x140949c10
0x1412780a9: movss dword ptr [rsp + 0x28], xmm1
0x1412780af: movss dword ptr [rsp + 0x20], xmm0
0x1412780b5: lea r15, [rsp + 0x38]
0x1412780ba: mov rcx, r15
0x1412780bd: movaps xmm1, xmm7
0x1412780c0: movaps xmm2, xmm6
0x1412780c3: mov r9, r14
0x1412780c6: call 0x14127cab0
0x1412780cb: add rbx, 0x40
0x1412780cf: cmp bpl, 0x3f
0x1412780d3: mov eax, 0x23
0x1412780d8: mov ecx, 0x30ab
0x1412780dd: cmove ecx, eax
0x1412780e0: mov dword ptr [rsp + 0x20], ecx
0x1412780e4: lea rcx, [rsp + 0x90]
0x1412780ec: movss xmm3, dword ptr [rip + 0x16ad88c]
0x1412780f4: mov rdx, rbx
0x1412780f7: mov r8, r15
0x1412780fa: call 0x1408cc2e0
0x1412780ff: cmp dword ptr [rsp + 0xc4], 2
0x141278107: jne 0x141278116
0x141278109: mov dword ptr [rsp + 0x100], 2
0x141278114: jmp 0x14127815f
0x141278116: movups xmm0, xmmword ptr [rsp + 0x90]
0x14127811e: movups xmm1, xmmword ptr [rsp + 0xa0]
0x141278126: movups xmm2, xmmword ptr [rsp + 0xb0]
0x14127812e: movups xmm3, xmmword ptr [rsp + 0xc0]
0x141278136: movaps xmmword ptr [rsp + 0x80], xmm3
0x14127813e: movaps xmmword ptr [rsp + 0x70], xmm2
0x141278143: movaps xmmword ptr [rsp + 0x60], xmm1
0x141278148: movaps xmmword ptr [rsp + 0x50], xmm0
0x14127814d: lea rcx, [rsp + 0xd0]
0x141278155: lea rdx, [rsp + 0x50]
0x14127815a: call 0x1408cc630
0x14127815f: lea rbx, [rsp + 0x50]
0x141278164: lea r8, [rsp + 0x38]
0x141278169: movss xmm3, dword ptr [rip + 0x16ad80f]
0x141278171: mov rcx, rbx
0x141278174: mov rdx, rdi
0x141278177: call 0x1412773f0
0x14127817c: test byte ptr [rsp + 0x50], 1
0x141278181: je 0x141278191
0x141278183: movsd xmm1, qword ptr [rsp + 0x58]
0x141278189: movss xmm0, dword ptr [rsp + 0x60]
0x14127818f: jmp 0x1412781c6
0x141278191: movsd xmm2, qword ptr [rsp + 0x38]
0x141278197: movsd xmm3, qword ptr [rsp + 0x44]
0x14127819d: movshdup xmm1, xmm2
0x1412781a1: xorps xmm0, xmm0
0x1412781a4: subss xmm0, xmm1
0x1412781a8: movshdup xmm1, xmm3
0x1412781ac: divss xmm0, xmm1
0x1412781b0: movsldup xmm1, xmm0
0x1412781b4: mulss xmm0, dword ptr [rsp + 0x4c]
0x1412781ba: mulps xmm1, xmm3
0x1412781bd: addps xmm1, xmm2
0x1412781c0: addss xmm0, dword ptr [rsp + 0x40]
0x1412781c6: movlps qword ptr [rsp + 0x68], xmm1
0x1412781cb: movss dword ptr [rsp + 0x70], xmm0
0x1412781d1: mov byte ptr [rsp + 0x50], 0xb
0x1412781d6: movabs rax, 0x3f80000000000000
0x1412781e0: mov qword ptr [rsp + 0x74], rax
0x1412781e5: mov qword ptr [rsp + 0x7c], 0
0x1412781ee: cmp dword ptr [rsp + 0x100], 2
0x1412781f6: lea rax, [rsp + 0xd0]
0x1412781fe: cmove rax, rbx
0x141278202: movups xmm0, xmmword ptr [rax]
0x141278205: movups xmm1, xmmword ptr [rax + 0x10]
0x141278209: movups xmm2, xmmword ptr [rax + 0x20]
0x14127820d: movups xmm3, xmmword ptr [rax + 0x30]
0x141278211: movups xmmword ptr [rsi], xmm0
0x141278214: movups xmmword ptr [rsi + 0x10], xmm1
0x141278218: movups xmmword ptr [rsi + 0x20], xmm2
0x14127821c: movups xmmword ptr [rsi + 0x30], xmm3
0x141278220: mov rax, rsi
0x141278223: movaps xmm6, xmmword ptr [rsp + 0x110]
0x14127822b: movaps xmm7, xmmword ptr [rsp + 0x120]
0x141278233: add rsp, 0x138
0x14127823a: pop rbx
0x14127823b: pop rbp
0x14127823c: pop rdi
0x14127823d: pop rsi
0x14127823e: pop r14
0x141278240: pop r15
0x141278242: ret 
