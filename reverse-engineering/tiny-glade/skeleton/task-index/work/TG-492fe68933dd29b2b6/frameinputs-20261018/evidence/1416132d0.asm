1416132d0: push rbp
1416132d1: push r15
1416132d3: push r14
1416132d5: push r12
1416132d7: push rsi
1416132d8: push rdi
1416132d9: push rbx
1416132da: sub rsp, 0x100
1416132e1: lea rbp, [rsp + 0x80]
1416132e9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1416132f1: test r8, r8
1416132f4: je 0x1416133ee
1416132fa: mov r14, r9
1416132fd: mov rax, r9
141613300: and rax, 3
141613304: mov qword ptr [rbp + 0x18], rax
141613308: jne 0x1416133b3
14161330e: mov rbx, r8
141613311: mov rax, r8
141613314: and rax, 3
141613318: mov qword ptr [rbp + 0x18], rax
14161331c: jne 0x1416133c4
141613322: mov rdi, rdx
141613325: mov rsi, rcx
141613328: lea r15, [rbp - 0x40]
14161332c: mov rcx, r15
14161332f: mov rdx, r14
141613332: mov r8d, 0x22
141613338: mov r9b, 1
14161333b: call 0x140bd4de0
141613340: lea r12, [rbp + 0x68]
141613344: mov r8d, 4
14161334a: mov rcx, r12
14161334d: mov rdx, r15
141613350: call 0x140bd4f20
141613355: mov qword ptr [rsp + 0x30], r14
14161335a: mov qword ptr [rsp + 0x28], rbx
14161335f: mov qword ptr [rsp + 0x20], r12
141613364: lea r8, [rip + 0x166ca9d]
14161336b: lea rcx, [rbp + 0x18]
14161336f: mov r9d, 0x1b
141613375: mov rdx, rdi
141613378: call 0x1416a2dc0
14161337d: cmp dword ptr [rbp + 0x18], 0xb
141613381: jne 0x141613465
141613387: mov rax, qword ptr [rbp + 0x40]
14161338b: mov qword ptr [rsi + 0x20], rax
14161338f: movups xmm0, xmmword ptr [rbp + 0x20]
141613393: movups xmm1, xmmword ptr [rbp + 0x30]
141613397: movups xmmword ptr [rsi + 0x10], xmm1
14161339b: movups xmmword ptr [rsi], xmm0
14161339e: mov rax, rsi
1416133a1: add rsp, 0x100
1416133a8: pop rbx
1416133a9: pop rdi
1416133aa: pop rsi
1416133ab: pop r12
1416133ad: pop r14
1416133af: pop r15
1416133b1: pop rbp
1416133b2: ret 
1416133b3: mov qword ptr [rbp - 0x40], 0
1416133bb: lea rax, [rip + 0x16b6b86]
1416133c2: jmp 0x1416133d3
1416133c4: mov qword ptr [rbp - 0x40], 0
1416133cc: lea rax, [rip + 0x16b6b8d]
1416133d3: mov qword ptr [rsp + 0x20], rax
1416133d8: lea r8, [rip + 0x16b6b61]
1416133df: lea rdx, [rbp + 0x18]
1416133e3: lea r9, [rbp - 0x40]
1416133e7: xor ecx, ecx
1416133e9: call 0x1428d9596
1416133ee: lea rax, [rip + 0x148b3da]
1416133f5: mov qword ptr [rbp + 0x68], rax
1416133f9: mov qword ptr [rbp + 0x70], 0x40
141613401: lea rax, [rip + 0x166ca20]
141613408: mov qword ptr [rbp + 0x18], rax
14161340c: lea rax, [rip - 0x10dfb73]
141613413: mov qword ptr [rbp + 0x20], rax
141613417: lea rax, [rbp + 0x68]
14161341b: mov qword ptr [rbp + 0x28], rax
14161341f: lea rax, [rip - 0x1509596]
141613426: mov qword ptr [rbp + 0x30], rax
14161342a: lea rax, [rip + 0x166c2ff]
141613431: mov qword ptr [rbp - 0x40], rax
141613435: mov qword ptr [rbp - 0x38], 3
14161343d: mov qword ptr [rbp - 0x20], 0
141613445: lea rax, [rbp + 0x18]
141613449: mov qword ptr [rbp - 0x30], rax
14161344d: mov qword ptr [rbp - 0x28], 2
141613455: lea rdx, [rip + 0x166c32c]
14161345c: lea rcx, [rbp - 0x40]
141613460: call 0x1428d9390
141613465: movups xmm0, xmmword ptr [rbp + 0x58]
141613469: movaps xmmword ptr [rbp], xmm0
14161346d: movups xmm0, xmmword ptr [rbp + 0x18]
141613471: movups xmm1, xmmword ptr [rbp + 0x28]
141613475: movups xmm2, xmmword ptr [rbp + 0x38]
141613479: movups xmm3, xmmword ptr [rbp + 0x48]
14161347d: movaps xmmword ptr [rbp - 0x10], xmm3
141613481: movaps xmmword ptr [rbp - 0x20], xmm2
141613485: movaps xmmword ptr [rbp - 0x30], xmm1
141613489: movaps xmmword ptr [rbp - 0x40], xmm0
14161348d: lea rax, [rip + 0x166cd14]
141613494: mov qword ptr [rsp + 0x20], rax
141613499: lea rcx, [rip + 0x166b648]
1416134a0: lea r9, [rip + 0x166b621]
1416134a7: lea r8, [rbp - 0x40]
1416134ab: mov edx, 0x2b
1416134b0: call 0x1428d9760
1416134b5: ud2 
1416134b7: nop word ptr [rax + rax]
1416134c0: mov qword ptr [rsp + 0x10], rdx
1416134c5: push rbp
1416134c6: push r15
1416134c8: push r14
1416134ca: push r12
1416134cc: push rsi
1416134cd: push rdi
1416134ce: push rbx
1416134cf: sub rsp, 0x40
1416134d3: lea rbp, [rdx + 0x80]
1416134da: lea rcx, [rbp - 0x40]
1416134de: call 0x1407cea70
1416134e3: nop 
1416134e4: add rsp, 0x40
1416134e8: pop rbx
1416134e9: pop rdi
1416134ea: pop rsi
1416134eb: pop r12
1416134ed: pop r14
1416134ef: pop r15
1416134f1: pop rbp
1416134f2: ret 
