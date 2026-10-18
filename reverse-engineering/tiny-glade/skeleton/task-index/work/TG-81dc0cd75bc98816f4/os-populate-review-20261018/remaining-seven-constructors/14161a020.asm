14161a020: push rbp
14161a021: push r15
14161a023: push r14
14161a025: push r12
14161a027: push rsi
14161a028: push rdi
14161a029: push rbx
14161a02a: sub rsp, 0x100
14161a031: lea rbp, [rsp + 0x80]
14161a039: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14161a041: mov rsi, rcx
14161a044: test r8, r8
14161a047: sete al
14161a04a: cmp r9, 0x1001
14161a051: setb cl
14161a054: or cl, al
14161a056: je 0x14161a0f3
14161a05c: mov r14, r9
14161a05f: mov rbx, r8
14161a062: mov rdi, rdx
14161a065: lea r15, [rbp + 0x10]
14161a069: mov edx, 0x40000
14161a06e: mov rcx, r15
14161a071: mov r8d, 0x22
14161a077: mov r9b, 1
14161a07a: call 0x140bd4de0
14161a07f: lea r12, [rbp + 0x68]
14161a083: mov r8d, 0x10
14161a089: mov rcx, r12
14161a08c: mov rdx, r15
14161a08f: call 0x140bd4f20
14161a094: shl r14, 6
14161a098: mov qword ptr [rsp + 0x30], r14
14161a09d: mov qword ptr [rsp + 0x28], rbx
14161a0a2: mov qword ptr [rsp + 0x20], r12
14161a0a7: lea r8, [rip + 0x1665f22]
14161a0ae: lea rcx, [rbp - 0x40]
14161a0b2: mov r9d, 5
14161a0b8: mov rdx, rdi
14161a0bb: call 0x1416a2dc0
14161a0c0: cmp dword ptr [rbp - 0x40], 0xb
14161a0c4: jne 0x14161a154
14161a0ca: mov rax, qword ptr [rbp - 0x18]
14161a0ce: mov qword ptr [rsi + 0x20], rax
14161a0d2: movups xmm0, xmmword ptr [rbp - 0x38]
14161a0d6: movups xmm1, xmmword ptr [rbp - 0x28]
14161a0da: movups xmmword ptr [rsi + 0x10], xmm1
14161a0de: movups xmmword ptr [rsi], xmm0
14161a0e1: add rsp, 0x100
14161a0e8: pop rbx
14161a0e9: pop rdi
14161a0ea: pop rsi
14161a0eb: pop r12
14161a0ed: pop r14
14161a0ef: pop r15
14161a0f1: pop rbp
14161a0f2: ret 
14161a0f3: lea rax, [rip + 0x1483071]
14161a0fa: mov qword ptr [rbp - 0x40], rax
14161a0fe: mov qword ptr [rbp - 0x38], 0x49
14161a106: lea rax, [rbp - 0x40]
14161a10a: mov qword ptr [rbp + 0x68], rax
14161a10e: lea rax, [rip - 0x1510285]
14161a115: mov qword ptr [rbp + 0x70], rax
14161a119: lea rax, [rip + 0x16656a8]
14161a120: mov qword ptr [rbp + 0x10], rax
14161a124: mov qword ptr [rbp + 0x18], 1
14161a12c: mov qword ptr [rbp + 0x30], 0
14161a134: lea rax, [rbp + 0x68]
14161a138: mov qword ptr [rbp + 0x20], rax
14161a13c: mov qword ptr [rbp + 0x28], 1
14161a144: lea rdx, [rip + 0x166568d]
14161a14b: lea rcx, [rbp + 0x10]
14161a14f: call 0x1428d9390
14161a154: movups xmm0, xmmword ptr [rbp]
14161a158: movaps xmmword ptr [rbp + 0x50], xmm0
14161a15c: movups xmm0, xmmword ptr [rbp - 0x40]
14161a160: movups xmm1, xmmword ptr [rbp - 0x30]
14161a164: movups xmm2, xmmword ptr [rbp - 0x20]
14161a168: movups xmm3, xmmword ptr [rbp - 0x10]
14161a16c: movaps xmmword ptr [rbp + 0x40], xmm3
14161a170: movaps xmmword ptr [rbp + 0x30], xmm2
14161a174: movaps xmmword ptr [rbp + 0x20], xmm1
14161a178: movaps xmmword ptr [rbp + 0x10], xmm0
14161a17c: lea rax, [rip + 0x1666025]
14161a183: mov qword ptr [rsp + 0x20], rax
14161a188: lea rcx, [rip + 0x1664959]
14161a18f: lea r9, [rip + 0x1664932]
14161a196: lea r8, [rbp + 0x10]
14161a19a: mov edx, 0x2b
14161a19f: call 0x1428d9760
14161a1a4: ud2 
14161a1a6: nop word ptr cs:[rax + rax]
14161a1b0: mov qword ptr [rsp + 0x10], rdx
14161a1b5: push rbp
14161a1b6: push r15
14161a1b8: push r14
14161a1ba: push r12
14161a1bc: push rsi
14161a1bd: push rdi
14161a1be: push rbx
14161a1bf: sub rsp, 0x40
14161a1c3: lea rbp, [rdx + 0x80]
14161a1ca: lea rcx, [rbp + 0x10]
14161a1ce: call 0x1407cea70
14161a1d3: nop 
14161a1d4: add rsp, 0x40
14161a1d8: pop rbx
14161a1d9: pop rdi
14161a1da: pop rsi
14161a1db: pop r12
14161a1dd: pop r14
14161a1df: pop r15
14161a1e1: pop rbp
14161a1e2: ret 
