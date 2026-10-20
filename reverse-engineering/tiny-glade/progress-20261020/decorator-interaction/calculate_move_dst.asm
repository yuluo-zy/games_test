0x1412801f0: push r15
0x1412801f2: push r14
0x1412801f4: push r12
0x1412801f6: push rsi
0x1412801f7: push rdi
0x1412801f8: push rbp
0x1412801f9: push rbx
0x1412801fa: sub rsp, 0x50
0x1412801fe: movaps xmmword ptr [rsp + 0x40], xmm7
0x141280203: movaps xmmword ptr [rsp + 0x30], xmm6
0x141280208: mov eax, dword ptr [rdx + 0x20]
0x14128020b: lea r9, [rip + 0x19a469e]
0x141280212: movsxd rax, dword ptr [r9 + rax*4]
0x141280216: add rax, r9
0x141280219: jmp rax
0x14128021b: movss xmm0, dword ptr [rdx + 0x24]
0x141280220: movq xmm1, qword ptr [rdx + 0x28]
0x141280225: movq qword ptr [rcx], xmm1
0x141280229: movss dword ptr [rcx + 8], xmm0
0x14128022e: mov byte ptr [rcx + 0x22], 2
0x141280232: jmp 0x1412803af
0x141280237: movss xmm0, dword ptr [rdx + 0x24]
0x14128023c: movq xmm1, qword ptr [rdx + 0x28]
0x141280241: movq qword ptr [rcx], xmm1
0x141280245: movss dword ptr [rcx + 8], xmm0
0x14128024a: mov byte ptr [rcx + 0x22], 3
0x14128024e: jmp 0x1412803af
0x141280253: movss xmm0, dword ptr [rdx + 0x24]
0x141280258: mov rax, qword ptr [rdx + 0x28]
0x14128025c: mov qword ptr [rcx], rax
0x14128025f: movq xmm1, qword ptr [rdx + 0x30]
0x141280264: movq qword ptr [rcx + 8], xmm1
0x141280269: movss dword ptr [rcx + 0x10], xmm0
0x14128026e: mov byte ptr [rcx + 0x22], 4
0x141280272: jmp 0x1412803af
0x141280277: mov r12, r8
0x14128027a: mov r15, rcx
0x14128027d: movsd xmm6, qword ptr [rdx + 0x28]
0x141280282: movsd qword ptr [rsp + 0x20], xmm6
0x141280288: movzx ebx, byte ptr [rdx + 0x38]
0x14128028c: mov r14, qword ptr [rdx + 0x30]
0x141280290: movzx edi, byte ptr [rdx + 0x3a]
0x141280294: movzx ebp, byte ptr [rdx + 0x39]
0x141280298: add rdx, 0x10
0x14128029c: mov rcx, rdx
0x14128029f: call 0x140a1c290
0x1412802a4: test al, 1
0x1412802a6: je 0x1412803e3
0x1412802ac: mov rsi, rdx
0x1412802af: mov qword ptr [rsp + 0x28], rdx
0x1412802b4: cmp qword ptr [r12 + 0x18], 0
0x1412802ba: je 0x1412803cb
0x1412802c0: lea rcx, [r12 + 0x20]
0x1412802c5: lea rdx, [rsp + 0x28]
0x1412802ca: call 0x1405b0d70
0x1412802cf: mov rcx, qword ptr [r12]
0x1412802d3: mov rdx, qword ptr [r12 + 8]
0x1412802d8: mov r8, rdx
0x1412802db: and r8, rax
0x1412802de: shr rax, 0x39
0x1412802e2: movd xmm0, eax
0x1412802e6: punpcklbw xmm0, xmm0
0x1412802ea: pshuflw xmm0, xmm0, 0
0x1412802ef: pshufd xmm0, xmm0, 0
0x1412802f4: lea rax, [rcx - 0x2a8]
0x1412802fb: xor r9d, r9d
0x1412802fe: pcmpeqd xmm1, xmm1
0x141280302: movdqu xmm2, xmmword ptr [rcx + r8]
0x141280308: movdqa xmm3, xmm2
0x14128030c: pcmpeqb xmm3, xmm0
0x141280310: pmovmskb r10d, xmm3
0x141280315: test r10d, r10d
0x141280318: je 0x141280342
0x14128031a: tzcnt r11d, r10d
0x14128031f: add r11, r8
0x141280322: and r11, rdx
0x141280325: neg r11
0x141280328: imul r11, r11, 0x2a8
0x14128032f: cmp rsi, qword ptr [rax + r11]
0x141280333: je 0x141280360
0x141280335: lea r11d, [r10 - 1]
0x141280339: and r11w, r10w
0x14128033d: mov r10d, r11d
0x141280340: jne 0x14128031a
0x141280342: pcmpeqb xmm2, xmm1
0x141280346: pmovmskb r10d, xmm2
0x14128034b: test r10d, r10d
0x14128034e: jne 0x1412803cb
0x141280350: add r8, r9
0x141280353: add r8, 0x10
0x141280357: add r9, 0x10
0x14128035b: and r8, rdx
0x14128035e: jmp 0x141280302
0x141280360: lea rsi, [rcx + r11]
0x141280364: add rsi, -0x2a0
0x14128036b: movshdup xmm7, xmm6
0x14128036f: mov rcx, rsi
0x141280372: movaps xmm1, xmm6
0x141280375: movaps xmm2, xmm7
0x141280378: call 0x140b2a890
0x14128037d: subss xmm7, xmm0
0x141280381: lea rcx, [r15 + 0x14]
0x141280385: lea rdx, [rsp + 0x20]
0x14128038a: mov r8, rsi
0x14128038d: call 0x140c92090
0x141280392: mov rcx, r15
0x141280395: movlps qword ptr [r15], xmm6
0x141280399: mov qword ptr [r15 + 8], r14
0x14128039d: movss dword ptr [r15 + 0x10], xmm7
0x1412803a3: mov byte ptr [r15 + 0x20], bl
0x1412803a7: mov byte ptr [r15 + 0x21], bpl
0x1412803ab: mov byte ptr [r15 + 0x22], dil
0x1412803af: mov rax, rcx
0x1412803b2: movaps xmm6, xmmword ptr [rsp + 0x30]
0x1412803b7: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1412803bc: add rsp, 0x50
0x1412803c0: pop rbx
0x1412803c1: pop rbp
0x1412803c2: pop rdi
0x1412803c3: pop rsi
0x1412803c4: pop r12
0x1412803c6: pop r14
0x1412803c8: pop r15
0x1412803ca: ret 
0x1412803cb: lea rcx, [rip + 0x19a4356]
0x1412803d2: lea r8, [rip + 0x19a44bf]
0x1412803d9: mov edx, 0x16
0x1412803de: call 0x1428d9330
0x1412803e3: lea rcx, [rip + 0x19a4496]
0x1412803ea: call 0x1428d9310
0x1412803ef: int3 
