1409f18f0: 55 push rbp
1409f18f1: 4156 push r14
1409f18f3: 56 push rsi
1409f18f4: 57 push rdi
1409f18f5: 53 push rbx
1409f18f6: 4881eca0000000 sub rsp, 0xa0
1409f18fd: 488dac2480000000 lea rbp, [rsp + 0x80]
1409f1905: 48c74518feffffff mov qword ptr [rbp + 0x18], 0xfffffffffffffffe
1409f190d: 48ff4220 inc qword ptr [rdx + 0x20]
1409f1911: 488b4218 mov rax, qword ptr [rdx + 0x18]
1409f1915: 4885c0 test rax, rax
1409f1918: 0f841d010000 je 0x1409f1a3b
1409f191e: 4889ce mov rsi, rcx
1409f1921: 488b4a10 mov rcx, qword ptr [rdx + 0x10]
1409f1925: 4c8d4901 lea r9, [rcx + 1]
1409f1929: 4c8b12 mov r10, qword ptr [rdx]
1409f192c: 4c8b5a08 mov r11, qword ptr [rdx + 8]
1409f1930: 31ff xor edi, edi
1409f1932: 4d39d1 cmp r9, r10
1409f1935: 490f43fa cmovae rdi, r10
1409f1939: 48f7df neg rdi
1409f193c: 4c8d0c39 lea r9, [rcx + rdi]
1409f1940: 49ffc1 inc r9
1409f1943: 4c894a10 mov qword ptr [rdx + 0x10], r9
1409f1947: 48ffc8 dec rax
1409f194a: 48894218 mov qword ptr [rdx + 0x18], rax
1409f194e: 48c1e106 shl rcx, 6
1409f1952: 498b040b mov rax, qword ptr [r11 + rcx]
1409f1956: 498b540b38 mov rdx, qword ptr [r11 + rcx + 0x38]
1409f195b: 488955d0 mov qword ptr [rbp - 0x30], rdx
1409f195f: 410f10440b28 movups xmm0, xmmword ptr [r11 + rcx + 0x28]
1409f1965: 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
1409f1969: 410f10440b18 movups xmm0, xmmword ptr [r11 + rcx + 0x18]
1409f196f: 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
1409f1973: 410f10440b08 movups xmm0, xmmword ptr [r11 + rcx + 8]
1409f1979: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
1409f197d: 4889c1 mov rcx, rax
1409f1980: 48f7d9 neg rcx
1409f1983: 0f80b2000000 jo 0x1409f1a3b
1409f1989: 488945d8 mov qword ptr [rbp - 0x28], rax
1409f198d: 0f2845a0 movaps xmm0, xmmword ptr [rbp - 0x60]
1409f1991: 0f284db0 movaps xmm1, xmmword ptr [rbp - 0x50]
1409f1995: 0f2855c0 movaps xmm2, xmmword ptr [rbp - 0x40]
1409f1999: 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
1409f199d: 0f114df0 movups xmmword ptr [rbp - 0x10], xmm1
1409f19a1: 0f115500 movups xmmword ptr [rbp], xmm2
1409f19a5: 488b45d0 mov rax, qword ptr [rbp - 0x30]
1409f19a9: 48894510 mov qword ptr [rbp + 0x10], rax
1409f19ad: 418b401c mov eax, dword ptr [r8 + 0x1c]
1409f19b1: 498b4810 mov rcx, qword ptr [r8 + 0x10]
1409f19b5: 8901 mov dword ptr [rcx], eax
1409f19b7: 498b38 mov rdi, qword ptr [r8]
1409f19ba: 488b5f30 mov rbx, qword ptr [rdi + 0x30]
1409f19be: 4c8b7740 mov r14, qword ptr [rdi + 0x40]
1409f19c2: 483b5f20 cmp rbx, qword ptr [rdi + 0x20]
1409f19c6: 7510 jne 0x1409f19d8
1409f19c8: 488d4f20 lea rcx, [rdi + 0x20]
1409f19cc: 488d1575872402 lea rdx, [rip + 0x2248775]
1409f19d3: e81810b7ff call 0x1405629f0
1409f19d8: 488b4728 mov rax, qword ptr [rdi + 0x28]
1409f19dc: 488d0c5b lea rcx, [rbx + rbx*2]
1409f19e0: 48c1e104 shl rcx, 4
1409f19e4: 48ba0000000000000080 movabs rdx, 0x8000000000000000
1409f19ee: 48891408 mov qword ptr [rax + rcx], rdx
1409f19f2: 48c744080800000000 mov qword ptr [rax + rcx + 8], 0
1409f19fb: 4c89740828 mov qword ptr [rax + rcx + 0x28], r14
1409f1a00: 48ffc3 inc rbx
1409f1a03: 48895f30 mov qword ptr [rdi + 0x30], rbx
1409f1a07: 48ff4740 inc qword ptr [rdi + 0x40]
1409f1a0b: 0f1045d8 movups xmm0, xmmword ptr [rbp - 0x28]
1409f1a0f: 0f104de8 movups xmm1, xmmword ptr [rbp - 0x18]
1409f1a13: 0f1055f8 movups xmm2, xmmword ptr [rbp - 8]
1409f1a17: 0f105d08 movups xmm3, xmmword ptr [rbp + 8]
1409f1a1b: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
1409f1a1f: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
1409f1a23: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
1409f1a27: 0f1106 movups xmmword ptr [rsi], xmm0
1409f1a2a: 4889f0 mov rax, rsi
1409f1a2d: 4881c4a0000000 add rsp, 0xa0
1409f1a34: 5b pop rbx
1409f1a35: 5f pop rdi
1409f1a36: 5e pop rsi
1409f1a37: 415e pop r14
1409f1a39: 5d pop rbp
1409f1a3a: c3 ret 
1409f1a3b: 488d0db63d0d02 lea rcx, [rip + 0x20d3db6]
1409f1a42: e8c978ee01 call 0x1428d9310
1409f1a47: cc int3 
1409f1a48: 0f1f840000000000 nop dword ptr [rax + rax]
1409f1a50: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409f1a55: 55 push rbp
1409f1a56: 4156 push r14
1409f1a58: 56 push rsi
1409f1a59: 57 push rdi
1409f1a5a: 53 push rbx
1409f1a5b: 4883ec20 sub rsp, 0x20
1409f1a5f: 488daa80000000 lea rbp, [rdx + 0x80]
1409f1a66: 488d4dd8 lea rcx, [rbp - 0x28]
1409f1a6a: e82134eaff call 0x140894e90
1409f1a6f: 90 nop 
1409f1a70: 4883c420 add rsp, 0x20
1409f1a74: 5b pop rbx
1409f1a75: 5f pop rdi
1409f1a76: 5e pop rsi
1409f1a77: 415e pop r14
1409f1a79: 5d pop rbp
1409f1a7a: c3 ret 
