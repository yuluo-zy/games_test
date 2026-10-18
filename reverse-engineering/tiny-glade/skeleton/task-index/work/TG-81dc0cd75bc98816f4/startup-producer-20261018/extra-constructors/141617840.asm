141617840: push rbp
141617841: push r15
141617843: push r14
141617845: push r12
141617847: push rsi
141617848: push rdi
141617849: push rbx
14161784a: sub rsp, 0x100
141617851: lea rbp, [rsp + 0x80]
141617859: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141617861: mov rsi, rcx
141617864: test r8, r8
141617867: sete al
14161786a: cmp r9, 0x1389
141617871: setb cl
141617874: or cl, al
141617876: je 0x141617913
14161787c: mov r14, r9
14161787f: mov rbx, r8
141617882: mov rdi, rdx
141617885: lea r15, [rbp + 0x10]
141617889: mov edx, 0x27100
14161788e: mov rcx, r15
141617891: mov r8d, 0x22
141617897: mov r9b, 1
14161789a: call 0x140bd4de0
14161789f: lea r12, [rbp + 0x68]
1416178a3: mov r8d, 4
1416178a9: mov rcx, r12
1416178ac: mov rdx, r15
1416178af: call 0x140bd4f20
1416178b4: shl r14, 5
1416178b8: mov qword ptr [rsp + 0x30], r14
1416178bd: mov qword ptr [rsp + 0x28], rbx
1416178c2: mov qword ptr [rsp + 0x20], r12
1416178c7: lea r8, [rip + 0x166826a]
1416178ce: lea rcx, [rbp - 0x40]
1416178d2: mov r9d, 0xf
1416178d8: mov rdx, rdi
1416178db: call 0x1416a2dc0
1416178e0: cmp dword ptr [rbp - 0x40], 0xb
1416178e4: jne 0x141617974
1416178ea: mov rax, qword ptr [rbp - 0x18]
1416178ee: mov qword ptr [rsi + 0x20], rax
1416178f2: movups xmm0, xmmword ptr [rbp - 0x38]
1416178f6: movups xmm1, xmmword ptr [rbp - 0x28]
1416178fa: movups xmmword ptr [rsi + 0x10], xmm1
1416178fe: movups xmmword ptr [rsi], xmm0
141617901: add rsp, 0x100
141617908: pop rbx
141617909: pop rdi
14161790a: pop rsi
14161790b: pop r12
14161790d: pop r14
14161790f: pop r15
141617911: pop rbp
141617912: ret 
141617913: lea rax, [rip + 0x14865e3]
14161791a: mov qword ptr [rbp - 0x40], rax
14161791e: mov qword ptr [rbp - 0x38], 0x5e
141617926: lea rax, [rbp - 0x40]
14161792a: mov qword ptr [rbp + 0x68], rax
14161792e: lea rax, [rip - 0x150daa5]
141617935: mov qword ptr [rbp + 0x70], rax
141617939: lea rax, [rip + 0x1667e88]
141617940: mov qword ptr [rbp + 0x10], rax
141617944: mov qword ptr [rbp + 0x18], 1
14161794c: mov qword ptr [rbp + 0x30], 0
141617954: lea rax, [rbp + 0x68]
141617958: mov qword ptr [rbp + 0x20], rax
14161795c: mov qword ptr [rbp + 0x28], 1
141617964: lea rdx, [rip + 0x1667e6d]
14161796b: lea rcx, [rbp + 0x10]
14161796f: call 0x1428d9390
141617974: movups xmm0, xmmword ptr [rbp]
141617978: movaps xmmword ptr [rbp + 0x50], xmm0
14161797c: movups xmm0, xmmword ptr [rbp - 0x40]
141617980: movups xmm1, xmmword ptr [rbp - 0x30]
141617984: movups xmm2, xmmword ptr [rbp - 0x20]
141617988: movups xmm3, xmmword ptr [rbp - 0x10]
14161798c: movaps xmmword ptr [rbp + 0x40], xmm3
141617990: movaps xmmword ptr [rbp + 0x30], xmm2
141617994: movaps xmmword ptr [rbp + 0x20], xmm1
141617998: movaps xmmword ptr [rbp + 0x10], xmm0
14161799c: lea rax, [rip + 0x1668805]
1416179a3: mov qword ptr [rsp + 0x20], rax
1416179a8: lea rcx, [rip + 0x1667139]
1416179af: lea r9, [rip + 0x1667112]
1416179b6: lea r8, [rbp + 0x10]
1416179ba: mov edx, 0x2b
1416179bf: call 0x1428d9760
1416179c4: ud2 
1416179c6: nop word ptr cs:[rax + rax]
1416179d0: mov qword ptr [rsp + 0x10], rdx
1416179d5: push rbp
1416179d6: push r15
1416179d8: push r14
1416179da: push r12
1416179dc: push rsi
1416179dd: push rdi
1416179de: push rbx
1416179df: sub rsp, 0x40
1416179e3: lea rbp, [rdx + 0x80]
1416179ea: lea rcx, [rbp + 0x10]
1416179ee: call 0x1407cea70
1416179f3: nop 
1416179f4: add rsp, 0x40
1416179f8: pop rbx
1416179f9: pop rdi
1416179fa: pop rsi
1416179fb: pop r12
1416179fd: pop r14
1416179ff: pop r15
141617a01: pop rbp
141617a02: ret 
