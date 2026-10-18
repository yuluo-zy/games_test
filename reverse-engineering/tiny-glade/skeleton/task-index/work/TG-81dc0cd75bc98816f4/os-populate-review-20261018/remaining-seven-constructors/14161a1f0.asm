14161a1f0: push rbp
14161a1f1: push r15
14161a1f3: push r14
14161a1f5: push r12
14161a1f7: push rsi
14161a1f8: push rdi
14161a1f9: push rbx
14161a1fa: sub rsp, 0x100
14161a201: lea rbp, [rsp + 0x80]
14161a209: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14161a211: mov rsi, rcx
14161a214: test r8, r8
14161a217: sete al
14161a21a: cmp r9, 0xc5c2
14161a221: setb cl
14161a224: or cl, al
14161a226: je 0x14161a2c3
14161a22c: mov r14, r9
14161a22f: mov rbx, r8
14161a232: mov rdi, rdx
14161a235: lea r15, [rbp + 0x10]
14161a239: mov edx, 0x317040
14161a23e: mov rcx, r15
14161a241: mov r8d, 0x22
14161a247: mov r9b, 1
14161a24a: call 0x140bd4de0
14161a24f: lea r12, [rbp + 0x68]
14161a253: mov r8d, 0x10
14161a259: mov rcx, r12
14161a25c: mov rdx, r15
14161a25f: call 0x140bd4f20
14161a264: shl r14, 6
14161a268: mov qword ptr [rsp + 0x30], r14
14161a26d: mov qword ptr [rsp + 0x28], rbx
14161a272: mov qword ptr [rsp + 0x20], r12
14161a277: lea r8, [rip + 0x1665d57]
14161a27e: lea rcx, [rbp - 0x40]
14161a282: mov r9d, 0xf
14161a288: mov rdx, rdi
14161a28b: call 0x1416a2dc0
14161a290: cmp dword ptr [rbp - 0x40], 0xb
14161a294: jne 0x14161a324
14161a29a: mov rax, qword ptr [rbp - 0x18]
14161a29e: mov qword ptr [rsi + 0x20], rax
14161a2a2: movups xmm0, xmmword ptr [rbp - 0x38]
14161a2a6: movups xmm1, xmmword ptr [rbp - 0x28]
14161a2aa: movups xmmword ptr [rsi + 0x10], xmm1
14161a2ae: movups xmmword ptr [rsi], xmm0
14161a2b1: add rsp, 0x100
14161a2b8: pop rbx
14161a2b9: pop rdi
14161a2ba: pop rsi
14161a2bb: pop r12
14161a2bd: pop r14
14161a2bf: pop r15
14161a2c1: pop rbp
14161a2c2: ret 
14161a2c3: lea rax, [rip + 0x14849dd]
14161a2ca: mov qword ptr [rbp - 0x40], rax
14161a2ce: mov qword ptr [rbp - 0x38], 0x54
14161a2d6: lea rax, [rbp - 0x40]
14161a2da: mov qword ptr [rbp + 0x68], rax
14161a2de: lea rax, [rip - 0x1510455]
14161a2e5: mov qword ptr [rbp + 0x70], rax
14161a2e9: lea rax, [rip + 0x16654d8]
14161a2f0: mov qword ptr [rbp + 0x10], rax
14161a2f4: mov qword ptr [rbp + 0x18], 1
14161a2fc: mov qword ptr [rbp + 0x30], 0
14161a304: lea rax, [rbp + 0x68]
14161a308: mov qword ptr [rbp + 0x20], rax
14161a30c: mov qword ptr [rbp + 0x28], 1
14161a314: lea rdx, [rip + 0x16654bd]
14161a31b: lea rcx, [rbp + 0x10]
14161a31f: call 0x1428d9390
14161a324: movups xmm0, xmmword ptr [rbp]
14161a328: movaps xmmword ptr [rbp + 0x50], xmm0
14161a32c: movups xmm0, xmmword ptr [rbp - 0x40]
14161a330: movups xmm1, xmmword ptr [rbp - 0x30]
14161a334: movups xmm2, xmmword ptr [rbp - 0x20]
14161a338: movups xmm3, xmmword ptr [rbp - 0x10]
14161a33c: movaps xmmword ptr [rbp + 0x40], xmm3
14161a340: movaps xmmword ptr [rbp + 0x30], xmm2
14161a344: movaps xmmword ptr [rbp + 0x20], xmm1
14161a348: movaps xmmword ptr [rbp + 0x10], xmm0
14161a34c: lea rax, [rip + 0x1665e55]
14161a353: mov qword ptr [rsp + 0x20], rax
14161a358: lea rcx, [rip + 0x1664789]
14161a35f: lea r9, [rip + 0x1664762]
14161a366: lea r8, [rbp + 0x10]
14161a36a: mov edx, 0x2b
14161a36f: call 0x1428d9760
14161a374: ud2 
14161a376: nop word ptr cs:[rax + rax]
14161a380: mov qword ptr [rsp + 0x10], rdx
14161a385: push rbp
14161a386: push r15
14161a388: push r14
14161a38a: push r12
14161a38c: push rsi
14161a38d: push rdi
14161a38e: push rbx
14161a38f: sub rsp, 0x40
14161a393: lea rbp, [rdx + 0x80]
14161a39a: lea rcx, [rbp + 0x10]
14161a39e: call 0x1407cea70
14161a3a3: nop 
14161a3a4: add rsp, 0x40
14161a3a8: pop rbx
14161a3a9: pop rdi
14161a3aa: pop rsi
14161a3ab: pop r12
14161a3ad: pop r14
14161a3af: pop r15
14161a3b1: pop rbp
14161a3b2: ret 
