14161ddb0: 55 push rbp
14161ddb1: 4157 push r15
14161ddb3: 4156 push r14
14161ddb5: 4154 push r12
14161ddb7: 56 push rsi
14161ddb8: 57 push rdi
14161ddb9: 53 push rbx
14161ddba: 4881ec30010000 sub rsp, 0x130
14161ddc1: 488dac2480000000 lea rbp, [rsp + 0x80]
14161ddc9: 48c785a8000000feffffff mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161ddd4: 4889d7 mov rdi, rdx
14161ddd7: 4889ce mov rsi, rcx
14161ddda: 4d85c0 test r8, r8
14161dddd: 7437 je 0x14161de16
14161dddf: 4c89cb mov rbx, r9
14161dde2: 4c89c8 mov rax, r9
14161dde5: 4883e003 and rax, 3
14161dde9: 488945c0 mov qword ptr [rbp - 0x40], rax
14161dded: 0f85fe010000 jne 0x14161dff1
14161ddf3: 4d89c7 mov r15, r8
14161ddf6: 4c89c0 mov rax, r8
14161ddf9: 4883e003 and rax, 3
14161ddfd: 488945c0 mov qword ptr [rbp - 0x40], rax
14161de01: 0f85fb010000 jne 0x14161e002
14161de07: 4883fb08 cmp rbx, 8
14161de0b: 0f838c020000 jae 0x14161e09d
14161de11: 4531e4 xor r12d, r12d
14161de14: eb30 jmp 0x14161de46
14161de16: 0fb605ec6d0102 movzx eax, byte ptr [rip + 0x2016dec]
14161de1d: bb04000000 mov ebx, 4
14161de22: b904000000 mov ecx, 4
14161de27: ba04000000 mov edx, 4
14161de2c: e8df5dfffe call 0x140613c10
14161de31: 4885c0 test rax, rax
14161de34: 0f84cd020000 je 0x14161e107
14161de3a: 4989c7 mov r15, rax
14161de3d: c70000000000 mov dword ptr [rax], 0
14161de43: 41b401 mov r12b, 1
14161de46: 4c89bd98000000 mov qword ptr [rbp + 0x98], r15
14161de4d: 4488a5a7000000 mov byte ptr [rbp + 0xa7], r12b
14161de54: 488d4d10 lea rcx, [rbp + 0x10]
14161de58: ba04000000 mov edx, 4
14161de5d: 41b822000000 mov r8d, 0x22
14161de63: 41b104 mov r9b, 4
14161de66: e8756f5bff call 0x140bd4de0
14161de6b: 4c89bd98000000 mov qword ptr [rbp + 0x98], r15
14161de72: 4488a5a7000000 mov byte ptr [rbp + 0xa7], r12b
14161de79: 4c8d7560 lea r14, [rbp + 0x60]
14161de7d: 488d5510 lea rdx, [rbp + 0x10]
14161de81: 41b804000000 mov r8d, 4
14161de87: 4c89f1 mov rcx, r14
14161de8a: e891705bff call 0x140bd4f20
14161de8f: 4c89bd98000000 mov qword ptr [rbp + 0x98], r15
14161de96: 4488a5a7000000 mov byte ptr [rbp + 0xa7], r12b
14161de9d: 48895c2430 mov qword ptr [rsp + 0x30], rbx
14161dea2: 4c897c2428 mov qword ptr [rsp + 0x28], r15
14161dea7: 4c89742420 mov qword ptr [rsp + 0x20], r14
14161deac: 4c8d05ed196601 lea r8, [rip + 0x16619ed]
14161deb3: 488d4dc0 lea rcx, [rbp - 0x40]
14161deb7: 41b90e000000 mov r9d, 0xe
14161debd: 4889fa mov rdx, rdi
14161dec0: e8fb4e0800 call 0x1416a2dc0
14161dec5: 837dc00b cmp dword ptr [rbp - 0x40], 0xb
14161dec9: 0f855d010000 jne 0x14161e02c
14161decf: 488b45e8 mov rax, qword ptr [rbp - 0x18]
14161ded3: 48898580000000 mov qword ptr [rbp + 0x80], rax
14161deda: 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
14161dede: 0f104dd8 movups xmm1, xmmword ptr [rbp - 0x28]
14161dee2: 0f294d70 movaps xmmword ptr [rbp + 0x70], xmm1
14161dee6: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
14161deea: 4584e4 test r12b, r12b
14161deed: 7413 je 0x14161df02
14161deef: ba04000000 mov edx, 4
14161def4: 41b804000000 mov r8d, 4
14161defa: 4c89f9 mov rcx, r15
14161defd: e81e5dfffe call 0x140613c20
14161df02: 488d4d60 lea rcx, [rbp + 0x60]
14161df06: 4889fa mov rdx, rdi
14161df09: e8c26d5bff call 0x140bd4cd0
14161df0e: 4885c0 test rax, rax
14161df11: 0f8475010000 je 0x14161e08c
14161df17: 483b5578 cmp rdx, qword ptr [rbp + 0x78]
14161df1b: 7259 jb 0x14161df76
14161df1d: 4889d1 mov rcx, rdx
14161df20: 4883e103 and rcx, 3
14161df24: 48894dc0 mov qword ptr [rbp - 0x40], rcx
14161df28: 7569 jne 0x14161df93
14161df2a: 4889c1 mov rcx, rax
14161df2d: 4883e103 and rcx, 3
14161df31: 48894dc0 mov qword ptr [rbp - 0x40], rcx
14161df35: 0f8587000000 jne 0x14161dfc2
14161df3b: 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
14161df42: 48894e30 mov qword ptr [rsi + 0x30], rcx
14161df46: 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
14161df4a: 0f284d70 movaps xmm1, xmmword ptr [rbp + 0x70]
14161df4e: 0f114e20 movups xmmword ptr [rsi + 0x20], xmm1
14161df52: 0f114610 movups xmmword ptr [rsi + 0x10], xmm0
14161df56: 48c1ea02 shr rdx, 2
14161df5a: 488906 mov qword ptr [rsi], rax
14161df5d: 48895608 mov qword ptr [rsi + 8], rdx
14161df61: 4889f0 mov rax, rsi
14161df64: 4881c430010000 add rsp, 0x130
14161df6b: 5b pop rbx
14161df6c: 5f pop rdi
14161df6d: 5e pop rsi
14161df6e: 415c pop r12
14161df70: 415e pop r14
14161df72: 415f pop r15
14161df74: 5d pop rbp
14161df75: c3 ret 
14161df76: 488d0d73226601 lea rcx, [rip + 0x1662273]
14161df7d: 4c8d05a4226601 lea r8, [rip + 0x16622a4]
14161df84: ba37000000 mov edx, 0x37
14161df89: e8a2b42b01 call 0x1428d9430
14161df8e: e991010000 jmp 0x14161e124
14161df93: 48c7451000000000 mov qword ptr [rbp + 0x10], 0
14161df9b: 488d05a6226601 lea rax, [rip + 0x16622a6]
14161dfa2: 4889442420 mov qword ptr [rsp + 0x20], rax
14161dfa7: 4c8d0592226601 lea r8, [rip + 0x1662292]
14161dfae: 488d55c0 lea rdx, [rbp - 0x40]
14161dfb2: 4c8d4d10 lea r9, [rbp + 0x10]
14161dfb6: 31c9 xor ecx, ecx
14161dfb8: e8d9b52b01 call 0x1428d9596
14161dfbd: e962010000 jmp 0x14161e124
14161dfc2: 48c7451000000000 mov qword ptr [rbp + 0x10], 0
14161dfca: 488d058f226601 lea rax, [rip + 0x166228f]
14161dfd1: 4889442420 mov qword ptr [rsp + 0x20], rax
14161dfd6: 4c8d0563226601 lea r8, [rip + 0x1662263]
14161dfdd: 488d55c0 lea rdx, [rbp - 0x40]
14161dfe1: 4c8d4d10 lea r9, [rbp + 0x10]
14161dfe5: 31c9 xor ecx, ecx
14161dfe7: e8aab52b01 call 0x1428d9596
14161dfec: e933010000 jmp 0x14161e124
14161dff1: 48c7451000000000 mov qword ptr [rbp + 0x10], 0
14161dff9: 488d0548bf6a01 lea rax, [rip + 0x16abf48]
14161e000: eb0f jmp 0x14161e011
14161e002: 48c7451000000000 mov qword ptr [rbp + 0x10], 0
14161e00a: 488d054fbf6a01 lea rax, [rip + 0x16abf4f]
14161e011: 4889442420 mov qword ptr [rsp + 0x20], rax
14161e016: 4c8d0523bf6a01 lea r8, [rip + 0x16abf23]
14161e01d: 488d55c0 lea rdx, [rbp - 0x40]
14161e021: 4c8d4d10 lea r9, [rbp + 0x10]
14161e025: 31c9 xor ecx, ecx
14161e027: e86ab52b01 call 0x1428d9596
14161e02c: 4489a594000000 mov dword ptr [rbp + 0x94], r12d
14161e033: 4c897db8 mov qword ptr [rbp - 0x48], r15
14161e037: 0f104500 movups xmm0, xmmword ptr [rbp]
14161e03b: 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
14161e03f: 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
14161e043: 0f104dd0 movups xmm1, xmmword ptr [rbp - 0x30]
14161e047: 0f1055e0 movups xmm2, xmmword ptr [rbp - 0x20]
14161e04b: 0f105df0 movups xmm3, xmmword ptr [rbp - 0x10]
14161e04f: 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
14161e053: 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
14161e057: 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
14161e05b: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
14161e05f: 488d0542216601 lea rax, [rip + 0x1662142]
14161e066: 4889442420 mov qword ptr [rsp + 0x20], rax
14161e06b: 488d0d760a6601 lea rcx, [rip + 0x1660a76]
14161e072: 4c8d0d4f0a6601 lea r9, [rip + 0x1660a4f]
14161e079: 4c8d4510 lea r8, [rbp + 0x10]
14161e07d: ba2b000000 mov edx, 0x2b
14161e082: e8d9b62b01 call 0x1428d9760
14161e087: e998000000 jmp 0x14161e124
14161e08c: 488d0d45216601 lea rcx, [rip + 0x1662145]
14161e093: e878b22b01 call 0x1428d9310
14161e098: e987000000 jmp 0x14161e124
14161e09d: 488d0523ff4701 lea rax, [rip + 0x147ff23]
14161e0a4: 488945c0 mov qword ptr [rbp - 0x40], rax
14161e0a8: 48c745c845000000 mov qword ptr [rbp - 0x38], 0x45
14161e0b0: 488d45c0 lea rax, [rbp - 0x40]
14161e0b4: 48894560 mov qword ptr [rbp + 0x60], rax
14161e0b8: 488d05d1bdaefe lea rax, [rip - 0x151422f]
14161e0bf: 48894568 mov qword ptr [rbp + 0x68], rax
14161e0c3: 488d05fe166601 lea rax, [rip + 0x16616fe]
14161e0ca: 48894510 mov qword ptr [rbp + 0x10], rax
14161e0ce: 48c7451801000000 mov qword ptr [rbp + 0x18], 1
14161e0d6: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
14161e0de: 488d4560 lea rax, [rbp + 0x60]
14161e0e2: 48894520 mov qword ptr [rbp + 0x20], rax
14161e0e6: 48c7452801000000 mov qword ptr [rbp + 0x28], 1
14161e0ee: c685a700000000 mov byte ptr [rbp + 0xa7], 0
14161e0f5: 488d15dc166601 lea rdx, [rip + 0x16616dc]
14161e0fc: 488d4d10 lea rcx, [rbp + 0x10]
14161e100: e88bb22b01 call 0x1428d9390
14161e105: eb1d jmp 0x14161e124
14161e107: c685a700000000 mov byte ptr [rbp + 0xa7], 0
14161e10e: 4c8d059b096601 lea r8, [rip + 0x166099b]
14161e115: b904000000 mov ecx, 4
14161e11a: ba04000000 mov edx, 4
14161e11f: e8bfae2b01 call 0x1428d8fe3
14161e124: 0f0b ud2 
14161e126: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14161e130: 4889542410 mov qword ptr [rsp + 0x10], rdx
14161e135: 55 push rbp
14161e136: 4157 push r15
14161e138: 4156 push r14
14161e13a: 4154 push r12
14161e13c: 56 push rsi
14161e13d: 57 push rdi
14161e13e: 53 push rbx
14161e13f: 4883ec40 sub rsp, 0x40
14161e143: 488daa80000000 lea rbp, [rdx + 0x80]
14161e14a: f685a700000001 test byte ptr [rbp + 0xa7], 1
14161e151: 7417 je 0x14161e16a
14161e153: 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
14161e15a: ba04000000 mov edx, 4
14161e15f: 41b804000000 mov r8d, 4
14161e165: e8b65afffe call 0x140613c20
14161e16a: 90 nop 
14161e16b: 4883c440 add rsp, 0x40
14161e16f: 5b pop rbx
14161e170: 5f pop rdi
14161e171: 5e pop rsi
14161e172: 415c pop r12
14161e174: 415e pop r14
14161e176: 415f pop r15
14161e178: 5d pop rbp
14161e179: c3 ret 
14161e17a: 660f1f440000 nop word ptr [rax + rax]
14161e180: 4889542410 mov qword ptr [rsp + 0x10], rdx
14161e185: 55 push rbp
14161e186: 4157 push r15
14161e188: 4156 push r14
14161e18a: 4154 push r12
14161e18c: 56 push rsi
14161e18d: 57 push rdi
14161e18e: 53 push rbx
14161e18f: 4883ec40 sub rsp, 0x40
14161e193: 488daa80000000 lea rbp, [rdx + 0x80]
14161e19a: 488d4d60 lea rcx, [rbp + 0x60]
14161e19e: e80d435cff call 0x140be24b0
14161e1a3: 90 nop 
14161e1a4: 4883c440 add rsp, 0x40
14161e1a8: 5b pop rbx
14161e1a9: 5f pop rdi
14161e1aa: 5e pop rsi
14161e1ab: 415c pop r12
14161e1ad: 415e pop r14
14161e1af: 415f pop r15
14161e1b1: 5d pop rbp
14161e1b2: c3 ret 
14161e1b3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14161e1c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14161e1c5: 55 push rbp
14161e1c6: 4157 push r15
14161e1c8: 4156 push r14
14161e1ca: 4154 push r12
14161e1cc: 56 push rsi
14161e1cd: 57 push rdi
14161e1ce: 53 push rbx
14161e1cf: 4883ec40 sub rsp, 0x40
14161e1d3: 488daa80000000 lea rbp, [rdx + 0x80]
14161e1da: 488b75b8 mov rsi, qword ptr [rbp - 0x48]
14161e1de: 4889b598000000 mov qword ptr [rbp + 0x98], rsi
14161e1e5: 8bbd94000000 mov edi, dword ptr [rbp + 0x94]
14161e1eb: 4088bda7000000 mov byte ptr [rbp + 0xa7], dil
14161e1f2: 488d4d10 lea rcx, [rbp + 0x10]
14161e1f6: e875081bff call 0x1407cea70
14161e1fb: 4889b598000000 mov qword ptr [rbp + 0x98], rsi
14161e202: 4088bda7000000 mov byte ptr [rbp + 0xa7], dil
14161e209: 4883c440 add rsp, 0x40
14161e20d: 5b pop rbx
14161e20e: 5f pop rdi
14161e20f: 5e pop rsi
14161e210: 415c pop r12
14161e212: 415e pop r14
14161e214: 415f pop r15
14161e216: 5d pop rbp
14161e217: c3 ret 
