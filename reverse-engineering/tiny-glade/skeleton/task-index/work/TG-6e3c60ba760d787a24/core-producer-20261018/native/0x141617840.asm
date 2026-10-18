141617840: 55 push rbp
141617841: 4157 push r15
141617843: 4156 push r14
141617845: 4154 push r12
141617847: 56 push rsi
141617848: 57 push rdi
141617849: 53 push rbx
14161784a: 4881ec00010000 sub rsp, 0x100
141617851: 488dac2480000000 lea rbp, [rsp + 0x80]
141617859: 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141617861: 4889ce mov rsi, rcx
141617864: 4d85c0 test r8, r8
141617867: 0f94c0 sete al
14161786a: 4981f989130000 cmp r9, 0x1389
141617871: 0f92c1 setb cl
141617874: 08c1 or cl, al
141617876: 0f8497000000 je 0x141617913
14161787c: 4d89ce mov r14, r9
14161787f: 4c89c3 mov rbx, r8
141617882: 4889d7 mov rdi, rdx
141617885: 4c8d7d10 lea r15, [rbp + 0x10]
141617889: ba00710200 mov edx, 0x27100
14161788e: 4c89f9 mov rcx, r15
141617891: 41b822000000 mov r8d, 0x22
141617897: 41b101 mov r9b, 1
14161789a: e841d55bff call 0x140bd4de0
14161789f: 4c8d6568 lea r12, [rbp + 0x68]
1416178a3: 41b804000000 mov r8d, 4
1416178a9: 4c89e1 mov rcx, r12
1416178ac: 4c89fa mov rdx, r15
1416178af: e86cd65bff call 0x140bd4f20
1416178b4: 49c1e605 shl r14, 5
1416178b8: 4c89742430 mov qword ptr [rsp + 0x30], r14
1416178bd: 48895c2428 mov qword ptr [rsp + 0x28], rbx
1416178c2: 4c89642420 mov qword ptr [rsp + 0x20], r12
1416178c7: 4c8d056a826601 lea r8, [rip + 0x166826a]
1416178ce: 488d4dc0 lea rcx, [rbp - 0x40]
1416178d2: 41b90f000000 mov r9d, 0xf
1416178d8: 4889fa mov rdx, rdi
1416178db: e8e0b40800 call 0x1416a2dc0
1416178e0: 837dc00b cmp dword ptr [rbp - 0x40], 0xb
1416178e4: 0f858a000000 jne 0x141617974
1416178ea: 488b45e8 mov rax, qword ptr [rbp - 0x18]
1416178ee: 48894620 mov qword ptr [rsi + 0x20], rax
1416178f2: 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
1416178f6: 0f104dd8 movups xmm1, xmmword ptr [rbp - 0x28]
1416178fa: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
1416178fe: 0f1106 movups xmmword ptr [rsi], xmm0
141617901: 4881c400010000 add rsp, 0x100
141617908: 5b pop rbx
141617909: 5f pop rdi
14161790a: 5e pop rsi
14161790b: 415c pop r12
14161790d: 415e pop r14
14161790f: 415f pop r15
141617911: 5d pop rbp
141617912: c3 ret 
141617913: 488d05e3654801 lea rax, [rip + 0x14865e3]
14161791a: 488945c0 mov qword ptr [rbp - 0x40], rax
14161791e: 48c745c85e000000 mov qword ptr [rbp - 0x38], 0x5e
141617926: 488d45c0 lea rax, [rbp - 0x40]
14161792a: 48894568 mov qword ptr [rbp + 0x68], rax
14161792e: 488d055b25affe lea rax, [rip - 0x150daa5]
141617935: 48894570 mov qword ptr [rbp + 0x70], rax
141617939: 488d05887e6601 lea rax, [rip + 0x1667e88]
141617940: 48894510 mov qword ptr [rbp + 0x10], rax
141617944: 48c7451801000000 mov qword ptr [rbp + 0x18], 1
14161794c: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
141617954: 488d4568 lea rax, [rbp + 0x68]
141617958: 48894520 mov qword ptr [rbp + 0x20], rax
14161795c: 48c7452801000000 mov qword ptr [rbp + 0x28], 1
141617964: 488d156d7e6601 lea rdx, [rip + 0x1667e6d]
14161796b: 488d4d10 lea rcx, [rbp + 0x10]
14161796f: e81c1a2c01 call 0x1428d9390
141617974: 0f104500 movups xmm0, xmmword ptr [rbp]
141617978: 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
14161797c: 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
141617980: 0f104dd0 movups xmm1, xmmword ptr [rbp - 0x30]
141617984: 0f1055e0 movups xmm2, xmmword ptr [rbp - 0x20]
141617988: 0f105df0 movups xmm3, xmmword ptr [rbp - 0x10]
14161798c: 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
141617990: 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
141617994: 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
141617998: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
14161799c: 488d0505886601 lea rax, [rip + 0x1668805]
1416179a3: 4889442420 mov qword ptr [rsp + 0x20], rax
1416179a8: 488d0d39716601 lea rcx, [rip + 0x1667139]
1416179af: 4c8d0d12716601 lea r9, [rip + 0x1667112]
1416179b6: 4c8d4510 lea r8, [rbp + 0x10]
1416179ba: ba2b000000 mov edx, 0x2b
1416179bf: e89c1d2c01 call 0x1428d9760
1416179c4: 0f0b ud2 
1416179c6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1416179d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1416179d5: 55 push rbp
1416179d6: 4157 push r15
1416179d8: 4156 push r14
1416179da: 4154 push r12
1416179dc: 56 push rsi
1416179dd: 57 push rdi
1416179de: 53 push rbx
1416179df: 4883ec40 sub rsp, 0x40
1416179e3: 488daa80000000 lea rbp, [rdx + 0x80]
1416179ea: 488d4d10 lea rcx, [rbp + 0x10]
1416179ee: e87d701bff call 0x1407cea70
1416179f3: 90 nop 
1416179f4: 4883c440 add rsp, 0x40
1416179f8: 5b pop rbx
1416179f9: 5f pop rdi
1416179fa: 5e pop rsi
1416179fb: 415c pop r12
1416179fd: 415e pop r14
1416179ff: 415f pop r15
141617a01: 5d pop rbp
141617a02: c3 ret 
