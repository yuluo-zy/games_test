141615bb0: 55 push rbp
141615bb1: 4157 push r15
141615bb3: 4156 push r14
141615bb5: 4154 push r12
141615bb7: 56 push rsi
141615bb8: 57 push rdi
141615bb9: 53 push rbx
141615bba: 4881ec00010000 sub rsp, 0x100
141615bc1: 488dac2480000000 lea rbp, [rsp + 0x80]
141615bc9: 48c74578feffffff mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141615bd1: 4889ce mov rsi, rcx
141615bd4: 4d85c0 test r8, r8
141615bd7: 0f94c0 sete al
141615bda: 4981f901800000 cmp r9, 0x8001
141615be1: 0f92c1 setb cl
141615be4: 08c1 or cl, al
141615be6: 0f8497000000 je 0x141615c83
141615bec: 4d89ce mov r14, r9
141615bef: 4c89c3 mov rbx, r8
141615bf2: 4889d7 mov rdi, rdx
141615bf5: 4c8d7d10 lea r15, [rbp + 0x10]
141615bf9: ba00000800 mov edx, 0x80000
141615bfe: 4c89f9 mov rcx, r15
141615c01: 41b822000000 mov r8d, 0x22
141615c07: 41b101 mov r9b, 1
141615c0a: e8d1f15bff call 0x140bd4de0
141615c0f: 4c8d6568 lea r12, [rbp + 0x68]
141615c13: 41b804000000 mov r8d, 4
141615c19: 4c89e1 mov rcx, r12
141615c1c: 4c89fa mov rdx, r15
141615c1f: e8fcf25bff call 0x140bd4f20
141615c24: 49c1e604 shl r14, 4
141615c28: 4c89742430 mov qword ptr [rsp + 0x30], r14
141615c2d: 48895c2428 mov qword ptr [rsp + 0x28], rbx
141615c32: 4c89642420 mov qword ptr [rsp + 0x20], r12
141615c37: 4c8d053d9c6601 lea r8, [rip + 0x1669c3d]
141615c3e: 488d4dc0 lea rcx, [rbp - 0x40]
141615c42: 41b90b000000 mov r9d, 0xb
141615c48: 4889fa mov rdx, rdi
141615c4b: e870d10800 call 0x1416a2dc0
141615c50: 837dc00b cmp dword ptr [rbp - 0x40], 0xb
141615c54: 0f858a000000 jne 0x141615ce4
141615c5a: 488b45e8 mov rax, qword ptr [rbp - 0x18]
141615c5e: 48894620 mov qword ptr [rsi + 0x20], rax
141615c62: 0f1045c8 movups xmm0, xmmword ptr [rbp - 0x38]
141615c66: 0f104dd8 movups xmm1, xmmword ptr [rbp - 0x28]
141615c6a: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
141615c6e: 0f1106 movups xmmword ptr [rsi], xmm0
141615c71: 4881c400010000 add rsp, 0x100
141615c78: 5b pop rbx
141615c79: 5f pop rdi
141615c7a: 5e pop rsi
141615c7b: 415c pop r12
141615c7d: 415e pop r14
141615c7f: 415f pop r15
141615c81: 5d pop rbp
141615c82: c3 ret 
141615c83: 488d0574764801 lea rax, [rip + 0x1487674]
141615c8a: 488945c0 mov qword ptr [rbp - 0x40], rax
141615c8e: 48c745c835000000 mov qword ptr [rbp - 0x38], 0x35
141615c96: 488d45c0 lea rax, [rbp - 0x40]
141615c9a: 48894568 mov qword ptr [rbp + 0x68], rax
141615c9e: 488d05eb41affe lea rax, [rip - 0x150be15]
141615ca5: 48894570 mov qword ptr [rbp + 0x70], rax
141615ca9: 488d05189b6601 lea rax, [rip + 0x1669b18]
141615cb0: 48894510 mov qword ptr [rbp + 0x10], rax
141615cb4: 48c7451801000000 mov qword ptr [rbp + 0x18], 1
141615cbc: 48c7453000000000 mov qword ptr [rbp + 0x30], 0
141615cc4: 488d4568 lea rax, [rbp + 0x68]
141615cc8: 48894520 mov qword ptr [rbp + 0x20], rax
141615ccc: 48c7452801000000 mov qword ptr [rbp + 0x28], 1
141615cd4: 488d15fd9a6601 lea rdx, [rip + 0x1669afd]
141615cdb: 488d4d10 lea rcx, [rbp + 0x10]
141615cdf: e8ac362c01 call 0x1428d9390
141615ce4: 0f104500 movups xmm0, xmmword ptr [rbp]
141615ce8: 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
141615cec: 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
141615cf0: 0f104dd0 movups xmm1, xmmword ptr [rbp - 0x30]
141615cf4: 0f1055e0 movups xmm2, xmmword ptr [rbp - 0x20]
141615cf8: 0f105df0 movups xmm3, xmmword ptr [rbp - 0x10]
141615cfc: 0f295d40 movaps xmmword ptr [rbp + 0x40], xmm3
141615d00: 0f295530 movaps xmmword ptr [rbp + 0x30], xmm2
141615d04: 0f294d20 movaps xmmword ptr [rbp + 0x20], xmm1
141615d08: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
141615d0c: 488d0595a46601 lea rax, [rip + 0x166a495]
141615d13: 4889442420 mov qword ptr [rsp + 0x20], rax
141615d18: 488d0dc98d6601 lea rcx, [rip + 0x1668dc9]
141615d1f: 4c8d0da28d6601 lea r9, [rip + 0x1668da2]
141615d26: 4c8d4510 lea r8, [rbp + 0x10]
141615d2a: ba2b000000 mov edx, 0x2b
141615d2f: e82c3a2c01 call 0x1428d9760
141615d34: 0f0b ud2 
141615d36: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
141615d40: 4889542410 mov qword ptr [rsp + 0x10], rdx
141615d45: 55 push rbp
141615d46: 4157 push r15
141615d48: 4156 push r14
141615d4a: 4154 push r12
141615d4c: 56 push rsi
141615d4d: 57 push rdi
141615d4e: 53 push rbx
141615d4f: 4883ec40 sub rsp, 0x40
141615d53: 488daa80000000 lea rbp, [rdx + 0x80]
141615d5a: 488d4d10 lea rcx, [rbp + 0x10]
141615d5e: e80d8d1bff call 0x1407cea70
141615d63: 90 nop 
141615d64: 4883c440 add rsp, 0x40
141615d68: 5b pop rbx
141615d69: 5f pop rdi
141615d6a: 5e pop rsi
141615d6b: 415c pop r12
141615d6d: 415e pop r14
141615d6f: 415f pop r15
141615d71: 5d pop rbp
141615d72: c3 ret 
