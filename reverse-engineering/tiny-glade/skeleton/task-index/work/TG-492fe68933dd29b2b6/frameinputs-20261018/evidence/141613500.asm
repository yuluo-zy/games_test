141613500: push rbp
141613501: push r15
141613503: push r14
141613505: push r12
141613507: push rsi
141613508: push rdi
141613509: push rbx
14161350a: sub rsp, 0x100
141613511: lea rbp, [rsp + 0x80]
141613519: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141613521: mov rbx, r8
141613524: mov rdi, rdx
141613527: mov rsi, rcx
14161352a: test r8, r8
14161352d: je 0x1416135c2
141613533: mov r14, r9
141613536: mov rax, r9
141613539: and rax, 3
14161353d: mov qword ptr [rbp - 0x40], rax
141613541: jne 0x14161364e
141613547: mov rax, rbx
14161354a: and rax, 3
14161354e: mov qword ptr [rbp - 0x40], rax
141613552: jne 0x14161365f
141613558: cmp r14, 0x1b4
14161355f: jb 0x1416135c5
141613561: lea rax, [rip + 0x148a348]
141613568: mov qword ptr [rbp - 0x40], rax
14161356c: mov qword ptr [rbp - 0x38], 0x49
141613574: lea rax, [rbp - 0x40]
141613578: mov qword ptr [rbp + 0x68], rax
14161357c: lea rax, [rip - 0x15096f3]
141613583: mov qword ptr [rbp + 0x70], rax
141613587: lea rax, [rip + 0x166c23a]
14161358e: mov qword ptr [rbp + 0x10], rax
141613592: mov qword ptr [rbp + 0x18], 1
14161359a: mov qword ptr [rbp + 0x30], 0
1416135a2: lea rax, [rbp + 0x68]
1416135a6: mov qword ptr [rbp + 0x20], rax
1416135aa: mov qword ptr [rbp + 0x28], 1
1416135b2: lea rdx, [rip + 0x166c21f]
1416135b9: lea rcx, [rbp + 0x10]
1416135bd: call 0x1428d9390
1416135c2: xor r14d, r14d
1416135c5: lea r15, [rbp + 0x10]
1416135c9: mov edx, 0x1b0
1416135ce: mov rcx, r15
1416135d1: mov r8d, 0x22
1416135d7: mov r9b, 1
1416135da: call 0x140bd4de0
1416135df: lea r12, [rbp + 0x68]
1416135e3: mov r8d, 4
1416135e9: mov rcx, r12
1416135ec: mov rdx, r15
1416135ef: call 0x140bd4f20
1416135f4: mov qword ptr [rsp + 0x30], r14
1416135f9: mov qword ptr [rsp + 0x28], rbx
1416135fe: mov qword ptr [rsp + 0x20], r12
141613603: lea r8, [rip + 0x166c9ed]
14161360a: lea rcx, [rbp - 0x40]
14161360e: mov r9d, 0x24
141613614: mov rdx, rdi
141613617: call 0x1416a2dc0
14161361c: cmp dword ptr [rbp - 0x40], 0xb
141613620: jne 0x141613689
141613622: mov rax, qword ptr [rbp - 0x18]
141613626: mov qword ptr [rsi + 0x20], rax
14161362a: movups xmm0, xmmword ptr [rbp - 0x38]
14161362e: movups xmm1, xmmword ptr [rbp - 0x28]
141613632: movups xmmword ptr [rsi + 0x10], xmm1
141613636: movups xmmword ptr [rsi], xmm0
141613639: mov rax, rsi
14161363c: add rsp, 0x100
141613643: pop rbx
141613644: pop rdi
141613645: pop rsi
141613646: pop r12
141613648: pop r14
14161364a: pop r15
14161364c: pop rbp
14161364d: ret 
14161364e: mov qword ptr [rbp + 0x10], 0
141613656: lea rax, [rip + 0x16b68eb]
14161365d: jmp 0x14161366e
14161365f: mov qword ptr [rbp + 0x10], 0
141613667: lea rax, [rip + 0x16b68f2]
14161366e: mov qword ptr [rsp + 0x20], rax
141613673: lea r8, [rip + 0x16b68c6]
14161367a: lea rdx, [rbp - 0x40]
14161367e: lea r9, [rbp + 0x10]
141613682: xor ecx, ecx
141613684: call 0x1428d9596
141613689: movups xmm0, xmmword ptr [rbp]
14161368d: movaps xmmword ptr [rbp + 0x50], xmm0
141613691: movups xmm0, xmmword ptr [rbp - 0x40]
141613695: movups xmm1, xmmword ptr [rbp - 0x30]
141613699: movups xmm2, xmmword ptr [rbp - 0x20]
14161369d: movups xmm3, xmmword ptr [rbp - 0x10]
1416136a1: movaps xmmword ptr [rbp + 0x40], xmm3
1416136a5: movaps xmmword ptr [rbp + 0x30], xmm2
1416136a9: movaps xmmword ptr [rbp + 0x20], xmm1
1416136ad: movaps xmmword ptr [rbp + 0x10], xmm0
1416136b1: lea rax, [rip + 0x166caf0]
1416136b8: mov qword ptr [rsp + 0x20], rax
1416136bd: lea rcx, [rip + 0x166b424]
1416136c4: lea r9, [rip + 0x166b3fd]
1416136cb: lea r8, [rbp + 0x10]
1416136cf: mov edx, 0x2b
1416136d4: call 0x1428d9760
1416136d9: ud2 
1416136db: nop dword ptr [rax + rax]
1416136e0: mov qword ptr [rsp + 0x10], rdx
1416136e5: push rbp
1416136e6: push r15
1416136e8: push r14
1416136ea: push r12
1416136ec: push rsi
1416136ed: push rdi
1416136ee: push rbx
1416136ef: sub rsp, 0x40
1416136f3: lea rbp, [rdx + 0x80]
1416136fa: lea rcx, [rbp + 0x10]
1416136fe: call 0x1407cea70
141613703: nop 
141613704: add rsp, 0x40
141613708: pop rbx
141613709: pop rdi
14161370a: pop rsi
14161370b: pop r12
14161370d: pop r14
14161370f: pop r15
141613711: pop rbp
141613712: ret 
