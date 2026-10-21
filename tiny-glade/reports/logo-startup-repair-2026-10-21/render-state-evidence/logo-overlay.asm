0x1408a5f50: push rbp
0x1408a5f51: push r15
0x1408a5f53: push r14
0x1408a5f55: push rsi
0x1408a5f56: push rdi
0x1408a5f57: push rbx
0x1408a5f58: sub rsp, 0x928
0x1408a5f5f: lea rbp, [rsp + 0x80]
0x1408a5f67: mov qword ptr [rbp + 0x8a0], 0xfffffffffffffffe
0x1408a5f72: mov r15, r8
0x1408a5f75: mov rdi, rdx
0x1408a5f78: mov rbx, rcx
0x1408a5f7b: lea rcx, [rdx + 0x38]
0x1408a5f7f: movabs rdx, 0x8597ee0727b461b3
0x1408a5f89: movabs r8, 0x8b297ec5ae15f15c
0x1408a5f93: call 0x1400dd6f0
0x1408a5f98: test rax, rax
0x1408a5f9b: je 0x1408a62c0
0x1408a5fa1: cmp qword ptr [rdi + 0x218], rdx
0x1408a5fa8: jbe 0x1408a62c0
0x1408a5fae: mov rax, qword ptr [rdi + 0x210]
0x1408a5fb5: mov rcx, qword ptr [rax + rdx*8]
0x1408a5fb9: test rcx, rcx
0x1408a5fbc: je 0x1408a62c0
0x1408a5fc2: mov rax, qword ptr [rdi + 0x1e0]
0x1408a5fc9: not rcx
0x1408a5fcc: lea rcx, [rcx + rcx*2]
0x1408a5fd0: shl rcx, 5
0x1408a5fd4: cmp qword ptr [rax + rcx + 0x20], 0
0x1408a5fda: je 0x1408a62c0
0x1408a5fe0: add rax, rcx
0x1408a5fe3: mov rcx, qword ptr [rax + 0x10]
0x1408a5fe7: lea rdx, [rip + 0x21f55bd]
0x1408a5fee: mov r8d, 0xc
0x1408a5ff4: call 0x140937aa0
0x1408a5ff9: test al, 1
0x1408a5ffb: je 0x1408a62cc
0x1408a6001: mov r14, rdx
0x1408a6004: lea rsi, [r15 + 4]
0x1408a6008: mov rcx, rsi
0x1408a600b: call 0x140bd5790
0x1408a6010: mov dword ptr [rbp + 0x3b0], 0
0x1408a601a: mov qword ptr [rbp + 0x3b8], r15
0x1408a6021: mov dword ptr [rbp + 0x798], 2
0x1408a602b: mov qword ptr [rsp + 0x40], rbx
0x1408a6030: mov qword ptr [rsp + 0x38], rdi
0x1408a6035: mov qword ptr [rsp + 0x30], rax
0x1408a603a: lea rax, [rbp + 0x798]
0x1408a6041: mov qword ptr [rsp + 0x28], rax
0x1408a6046: mov qword ptr [rsp + 0x20], 1
0x1408a604f: lea rdx, [rip + 0x21f55da]
0x1408a6056: lea rcx, [rbp - 0x38]
0x1408a605a: lea r9, [rbp + 0x3b0]
0x1408a6061: mov r8d, 0xc
0x1408a6067: call 0x140867970
0x1408a606c: mov byte ptr [rbp + 0x89f], 1
0x1408a6073: call 0x140be6750
0x1408a6078: mov edi, eax
0x1408a607a: mov byte ptr [rbp + 0x89f], 1
0x1408a6081: xor ecx, ecx
0x1408a6083: call 0x1400d9b20
0x1408a6088: test rax, rax
0x1408a608b: je 0x1408a62d8
0x1408a6091: mov dword ptr [rbp + 0x840], edi
0x1408a6097: mov word ptr [rbp + 0x848], 0
0x1408a60a0: mov dword ptr [rbp + 0x844], 0x3f800000
0x1408a60aa: mov qword ptr [rbp + 0x798], 0
0x1408a60b5: mov qword ptr [rbp + 0x838], r14
0x1408a60bc: mov qword ptr [rbp + 0x7a8], 0
0x1408a60c7: mov qword ptr [rbp + 0x7b0], 4
0x1408a60d2: mov qword ptr [rbp + 0x7b8], 0
0x1408a60dd: movups xmm0, xmmword ptr [rax]
0x1408a60e0: inc qword ptr [rax]
0x1408a60e3: movups xmm1, xmmword ptr [rip + 0x21f0646]
0x1408a60ea: movups xmmword ptr [rbp + 0x818], xmm1
0x1408a60f1: movups xmm1, xmmword ptr [rip + 0x21f0628]
0x1408a60f8: movups xmmword ptr [rbp + 0x808], xmm1
0x1408a60ff: movups xmmword ptr [rbp + 0x828], xmm0
0x1408a6106: movabs rax, 0x100000003
0x1408a6110: mov qword ptr [rbp + 0x7d8], rax
0x1408a6117: mov qword ptr [rbp + 0x7c0], 0
0x1408a6122: mov qword ptr [rbp + 0x7c8], 1
0x1408a612d: mov qword ptr [rbp + 0x7d0], 0
0x1408a6138: mov byte ptr [rbp + 0x89f], 1
0x1408a613f: lea rcx, [rbp + 0x3b0]
0x1408a6146: lea rdx, [rbp + 0x798]
0x1408a614d: lea r8, [rbp - 0x38]
0x1408a6151: xor r9d, r9d
0x1408a6154: call 0x140905be0
0x1408a6159: mov byte ptr [rbp + 0x89e], 1
0x1408a6160: mov rcx, rsi
0x1408a6163: call 0x140bd5790
0x1408a6168: mov byte ptr [rbp + 0x89e], 0
0x1408a616f: lea rcx, [rbp + 0x798]
0x1408a6176: lea rsi, [rbp + 0x3b0]
0x1408a617d: movabs r8, 0x531bb9318eea4971
0x1408a6187: mov rdx, rsi
0x1408a618a: mov r9, rax
0x1408a618d: call 0x141629d20
0x1408a6192: movabs rax, 0x75b4be0d27cfedd3
0x1408a619c: mov qword ptr [rbp + 0x890], rax
0x1408a61a3: mov dword ptr [rbp + 0x3b0], 2
0x1408a61ad: mov qword ptr [rsp + 0x20], rsi
0x1408a61b2: lea rcx, [rbp + 0x890]
0x1408a61b9: lea rdx, [rbp + 0x798]
0x1408a61c0: movabs r8, 0x11ba5697cc40b3dd
0x1408a61ca: mov r9b, 0x1d
0x1408a61cd: call 0x140908610
0x1408a61d2: mov rsi, qword ptr [rbp + 0x798]
0x1408a61d9: lea rdx, [rbp + 0x7a0]
0x1408a61e0: lea rcx, [rbp + 0x3b8]
0x1408a61e7: mov r8d, 0xa0
0x1408a61ed: call 0x142923c00
0x1408a61f2: movzx eax, byte ptr [rbp + 0x840]
0x1408a61f9: movzx ecx, word ptr [rbp + 0x841]
0x1408a6200: movzx edx, byte ptr [rbp + 0x843]
0x1408a6207: movups xmm0, xmmword ptr [rbp + 0x844]
0x1408a620e: movups xmmword ptr [rbp + 0x45c], xmm0
0x1408a6215: movups xmm0, xmmword ptr [rbp + 0x854]
0x1408a621c: movups xmmword ptr [rbp + 0x46c], xmm0
0x1408a6223: movups xmm0, xmmword ptr [rbp + 0x864]
0x1408a622a: movups xmmword ptr [rbp + 0x47c], xmm0
0x1408a6231: movups xmm0, xmmword ptr [rbp + 0x874]
0x1408a6238: movups xmmword ptr [rbp + 0x48c], xmm0
0x1408a623f: movups xmm0, xmmword ptr [rbp + 0x880]
0x1408a6246: movups xmmword ptr [rbp + 0x498], xmm0
0x1408a624d: and al, 0x7f
0x1408a624f: and dl, 0x9f
0x1408a6252: or dl, 0x40
0x1408a6255: mov qword ptr [rbp + 0x3b0], rsi
0x1408a625c: mov byte ptr [rbp + 0x458], al
0x1408a6262: mov word ptr [rbp + 0x459], cx
0x1408a6269: mov byte ptr [rbp + 0x45b], dl
0x1408a626f: mov byte ptr [rbp + 0x89f], 1
0x1408a6276: lea rcx, [rbp + 0x3b0]
0x1408a627d: mov edx, 6
0x1408a6282: call 0x140906a90
0x1408a6287: lea rsi, [rbp + 0x3b0]
0x1408a628e: lea rdx, [rbp - 0x38]
0x1408a6292: mov r8d, 0x3e8
0x1408a6298: mov rcx, rsi
0x1408a629b: call 0x142923c00
0x1408a62a0: mov byte ptr [rbp + 0x89f], 0
0x1408a62a7: mov rcx, rsi
0x1408a62aa: call 0x140868370
0x1408a62af: nop 
0x1408a62b0: add rsp, 0x928
0x1408a62b7: pop rbx
0x1408a62b8: pop rdi
0x1408a62b9: pop rsi
0x1408a62ba: pop r14
0x1408a62bc: pop r15
0x1408a62be: pop rbp
0x1408a62bf: ret 
0x1408a62c0: lea rcx, [rip + 0x21f5339]
0x1408a62c7: call 0x1428d9310
0x1408a62cc: lea rcx, [rip + 0x21f5345]
0x1408a62d3: call 0x1428d9310
0x1408a62d8: mov byte ptr [rbp + 0x89f], 1
0x1408a62df: lea rcx, [rip + 0x23947ba]
0x1408a62e6: call 0x1428d80d0
0x1408a62eb: ud2 
0x1408a62ed: nop dword ptr [rax]
0x1408a62f0: mov qword ptr [rsp + 0x10], rdx
0x1408a62f5: push rbp
0x1408a62f6: push r15
0x1408a62f8: push r14
0x1408a62fa: push rsi
0x1408a62fb: push rdi
0x1408a62fc: push rbx
0x1408a62fd: sub rsp, 0x48
0x1408a6301: lea rbp, [rdx + 0x80]
0x1408a6308: cmp byte ptr [rbp + 0x89e], 0
0x1408a630f: je 0x1408a6331
0x1408a6311: lea rcx, [rbp + 0x3b0]
0x1408a6318: call 0x1408935c0
0x1408a631d: mov byte ptr [rbp + 0x89f], 1
0x1408a6324: add rsp, 0x48
0x1408a6328: pop rbx
0x1408a6329: pop rdi
0x1408a632a: pop rsi
0x1408a632b: pop r14
0x1408a632d: pop r15
0x1408a632f: pop rbp
0x1408a6330: ret 
0x1408a6331: mov byte ptr [rbp + 0x89f], 1
0x1408a6338: add rsp, 0x48
0x1408a633c: pop rbx
0x1408a633d: pop rdi
0x1408a633e: pop rsi
0x1408a633f: pop r14
0x1408a6341: pop r15
0x1408a6343: pop rbp
0x1408a6344: ret 
0x1408a6345: nop word ptr cs:[rax + rax]
0x1408a6350: mov qword ptr [rsp + 0x10], rdx
0x1408a6355: push rbp
0x1408a6356: push r15
0x1408a6358: push r14
0x1408a635a: push rsi
0x1408a635b: push rdi
0x1408a635c: push rbx
0x1408a635d: sub rsp, 0x48
0x1408a6361: lea rbp, [rdx + 0x80]
0x1408a6368: cmp byte ptr [rbp + 0x89f], 0
0x1408a636f: jne 0x1408a637e
0x1408a6371: add rsp, 0x48
0x1408a6375: pop rbx
0x1408a6376: pop rdi
0x1408a6377: pop rsi
0x1408a6378: pop r14
0x1408a637a: pop r15
0x1408a637c: pop rbp
0x1408a637d: ret 
0x1408a637e: lea rcx, [rbp - 0x38]
0x1408a6382: call 0x1408923c0
0x1408a6387: jmp 0x1408a6371
0x1408a6389: nop dword ptr [rax]
0x1408a6390: mov qword ptr [rsp + 0x10], rdx
0x1408a6395: push rbp
0x1408a6396: push r15
0x1408a6398: push r14
0x1408a639a: push rsi
0x1408a639b: push rdi
0x1408a639c: push rbx
0x1408a639d: sub rsp, 0x48
0x1408a63a1: lea rbp, [rdx + 0x80]
0x1408a63a8: mov byte ptr [rbp + 0x89f], 1
0x1408a63af: lea rcx, [rbp + 0x798]
0x1408a63b6: call 0x1408935c0
0x1408a63bb: mov byte ptr [rbp + 0x89f], 1
0x1408a63c2: add rsp, 0x48
0x1408a63c6: pop rbx
0x1408a63c7: pop rdi
0x1408a63c8: pop rsi
0x1408a63c9: pop r14
0x1408a63cb: pop r15
0x1408a63cd: pop rbp
0x1408a63ce: ret 
