1416146c0: push rbp
1416146c1: push r15
1416146c3: push r14
1416146c5: push r12
1416146c7: push rsi
1416146c8: push rdi
1416146c9: push rbx
1416146ca: sub rsp, 0x100
1416146d1: lea rbp, [rsp + 0x80]
1416146d9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1416146e1: mov rbx, r8
1416146e4: mov rdi, rdx
1416146e7: mov rsi, rcx
1416146ea: test r8, r8
1416146ed: je 0x14161477f
1416146f3: mov r14, r9
1416146f6: mov rax, r9
1416146f9: and rax, 0xf
1416146fd: mov qword ptr [rbp - 0x40], rax
141614701: jne 0x14161480b
141614707: mov rax, rbx
14161470a: and rax, 3
14161470e: mov qword ptr [rbp - 0x40], rax
141614712: jne 0x14161481c
141614718: cmp r14, 0x20
14161471c: jb 0x141614782
14161471e: lea rax, [rip + 0x1489d32]
141614725: mov qword ptr [rbp - 0x40], rax
141614729: mov qword ptr [rbp - 0x38], 0x55
141614731: lea rax, [rbp - 0x40]
141614735: mov qword ptr [rbp + 0x68], rax
141614739: lea rax, [rip - 0x150a8b0]
141614740: mov qword ptr [rbp + 0x70], rax
141614744: lea rax, [rip + 0x166b07d]
14161474b: mov qword ptr [rbp + 0x10], rax
14161474f: mov qword ptr [rbp + 0x18], 1
141614757: mov qword ptr [rbp + 0x30], 0
14161475f: lea rax, [rbp + 0x68]
141614763: mov qword ptr [rbp + 0x20], rax
141614767: mov qword ptr [rbp + 0x28], 1
14161476f: lea rdx, [rip + 0x166b062]
141614776: lea rcx, [rbp + 0x10]
14161477a: call 0x1428d9390
14161477f: xor r14d, r14d
141614782: lea r15, [rbp + 0x10]
141614786: mov edx, 0x10
14161478b: mov rcx, r15
14161478e: mov r8d, 0x22
141614794: mov r9b, 2
141614797: call 0x140bd4de0
14161479c: lea r12, [rbp + 0x68]
1416147a0: mov r8d, 4
1416147a6: mov rcx, r12
1416147a9: mov rdx, r15
1416147ac: call 0x140bd4f20
1416147b1: mov qword ptr [rsp + 0x30], r14
1416147b6: mov qword ptr [rsp + 0x28], rbx
1416147bb: mov qword ptr [rsp + 0x20], r12
1416147c0: lea r8, [rip + 0x166b159]
1416147c7: lea rcx, [rbp - 0x40]
1416147cb: mov r9d, 0x10
1416147d1: mov rdx, rdi
1416147d4: call 0x1416a2dc0
1416147d9: cmp dword ptr [rbp - 0x40], 0xb
1416147dd: jne 0x141614846
1416147df: mov rax, qword ptr [rbp - 0x18]
1416147e3: mov qword ptr [rsi + 0x20], rax
1416147e7: movups xmm0, xmmword ptr [rbp - 0x38]
1416147eb: movups xmm1, xmmword ptr [rbp - 0x28]
1416147ef: movups xmmword ptr [rsi + 0x10], xmm1
1416147f3: movups xmmword ptr [rsi], xmm0
1416147f6: mov rax, rsi
1416147f9: add rsp, 0x100
141614800: pop rbx
141614801: pop rdi
141614802: pop rsi
141614803: pop r12
141614805: pop r14
141614807: pop r15
141614809: pop rbp
14161480a: ret 
14161480b: mov qword ptr [rbp + 0x10], 0
141614813: lea rax, [rip + 0x16b572e]
14161481a: jmp 0x14161482b
14161481c: mov qword ptr [rbp + 0x10], 0
141614824: lea rax, [rip + 0x16b5735]
14161482b: mov qword ptr [rsp + 0x20], rax
141614830: lea r8, [rip + 0x16b5709]
141614837: lea rdx, [rbp - 0x40]
14161483b: lea r9, [rbp + 0x10]
14161483f: xor ecx, ecx
141614841: call 0x1428d9596
141614846: movups xmm0, xmmword ptr [rbp]
14161484a: movaps xmmword ptr [rbp + 0x50], xmm0
14161484e: movups xmm0, xmmword ptr [rbp - 0x40]
141614852: movups xmm1, xmmword ptr [rbp - 0x30]
141614856: movups xmm2, xmmword ptr [rbp - 0x20]
14161485a: movups xmm3, xmmword ptr [rbp - 0x10]
14161485e: movaps xmmword ptr [rbp + 0x40], xmm3
141614862: movaps xmmword ptr [rbp + 0x30], xmm2
141614866: movaps xmmword ptr [rbp + 0x20], xmm1
14161486a: movaps xmmword ptr [rbp + 0x10], xmm0
14161486e: lea rax, [rip + 0x166b933]
141614875: mov qword ptr [rsp + 0x20], rax
14161487a: lea rcx, [rip + 0x166a267]
141614881: lea r9, [rip + 0x166a240]
141614888: lea r8, [rbp + 0x10]
14161488c: mov edx, 0x2b
141614891: call 0x1428d9760
141614896: ud2 
141614898: nop dword ptr [rax + rax]
1416148a0: mov qword ptr [rsp + 0x10], rdx
1416148a5: push rbp
1416148a6: push r15
1416148a8: push r14
1416148aa: push r12
1416148ac: push rsi
1416148ad: push rdi
1416148ae: push rbx
1416148af: sub rsp, 0x40
1416148b3: lea rbp, [rdx + 0x80]
1416148ba: lea rcx, [rbp + 0x10]
1416148be: call 0x1407cea70
1416148c3: nop 
1416148c4: add rsp, 0x40
1416148c8: pop rbx
1416148c9: pop rdi
1416148ca: pop rsi
1416148cb: pop r12
1416148cd: pop r14
1416148cf: pop r15
1416148d1: pop rbp
1416148d2: ret 
