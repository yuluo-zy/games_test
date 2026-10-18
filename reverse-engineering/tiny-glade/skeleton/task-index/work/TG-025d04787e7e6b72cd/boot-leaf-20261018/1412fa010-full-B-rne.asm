1412fa010: 55 push rbp
1412fa011: 4157 push r15
1412fa013: 4156 push r14
1412fa015: 4155 push r13
1412fa017: 4154 push r12
1412fa019: 56 push rsi
1412fa01a: 57 push rdi
1412fa01b: 53 push rbx
1412fa01c: 4881ecf8000000 sub rsp, 0xf8
1412fa023: 488dac2480000000 lea rbp, [rsp + 0x80]
1412fa02b: 0f297d60 movaps xmmword ptr [rbp + 0x60], xmm7
1412fa02f: 0f297550 movaps xmmword ptr [rbp + 0x50], xmm6
1412fa033: 48c74548feffffff mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
1412fa03b: 4c8b22 mov r12, qword ptr [rdx]
1412fa03e: 4c8b7208 mov r14, qword ptr [rdx + 8]
1412fa042: 4d39f4 cmp r12, r14
1412fa045: 0f843a010000 je 0x1412fa185
1412fa04b: 4889d6 mov rsi, rdx
1412fa04e: 488b5210 mov rdx, qword ptr [rdx + 0x10]
1412fa052: 4c8b4618 mov r8, qword ptr [rsi + 0x18]
1412fa056: 4c8d4df8 lea r9, [rbp - 8]
1412fa05a: 0f28351f496301 movaps xmm6, xmmword ptr [rip + 0x163491f]
1412fa061: f30f103dab977801 movss xmm7, dword ptr [rip + 0x17897ab]
1412fa069: 4c89e0 mov rax, r12
1412fa06c: 0f1f4000 nop dword ptr [rax]
1412fa070: f30f1000 movss xmm0, dword ptr [rax]
1412fa074: 0f54c6 andps xmm0, xmm6
1412fa077: 0f2ef8 ucomiss xmm7, xmm0
1412fa07a: 7624 jbe 0x1412fa0a0
1412fa07c: f30f104008 movss xmm0, dword ptr [rax + 8]
1412fa081: 0f54c6 andps xmm0, xmm6
1412fa084: 0f2ef8 ucomiss xmm7, xmm0
1412fa087: 7617 jbe 0x1412fa0a0
1412fa089: 4883c030 add rax, 0x30
1412fa08d: 4c39f0 cmp rax, r14
1412fa090: 75de jne 0x1412fa070
1412fa092: e9eb000000 jmp 0x1412fa182
1412fa097: 660f1f840000000000 nop word ptr [rax + rax]
1412fa0a0: 4c8d6030 lea r12, [rax + 0x30]
1412fa0a4: 4c8926 mov qword ptr [rsi], r12
1412fa0a7: 4c8b12 mov r10, qword ptr [rdx]
1412fa0aa: 4989d5 mov r13, rdx
1412fa0ad: 488b5208 mov rdx, qword ptr [rdx + 8]
1412fa0b1: f049ff02 lock inc qword ptr [r10]
1412fa0b5: 0f8ee6000000 jle 0x1412fa1a1
1412fa0bb: 4c894538 mov qword ptr [rbp + 0x38], r8
1412fa0bf: 458b00 mov r8d, dword ptr [r8]
1412fa0c2: 0f1000 movups xmm0, xmmword ptr [rax]
1412fa0c5: 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
1412fa0c9: 0f105020 movups xmm2, xmmword ptr [rax + 0x20]
1412fa0cd: 0f2955d0 movaps xmmword ptr [rbp - 0x30], xmm2
1412fa0d1: 0f294dc0 movaps xmmword ptr [rbp - 0x40], xmm1
1412fa0d5: 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
1412fa0d9: 488955f0 mov qword ptr [rbp - 0x10], rdx
1412fa0dd: 410f115120 movups xmmword ptr [r9 + 0x20], xmm2
1412fa0e2: 410f114910 movups xmmword ptr [r9 + 0x10], xmm1
1412fa0e7: 4d89cf mov r15, r9
1412fa0ea: 410f1101 movups xmmword ptr [r9], xmm0
1412fa0ee: 48c7452800000000 mov qword ptr [rbp + 0x28], 0
1412fa0f6: 44894530 mov dword ptr [rbp + 0x30], r8d
1412fa0fa: 4c895540 mov qword ptr [rbp + 0x40], r10
1412fa0fe: 4c8955e8 mov qword ptr [rbp - 0x18], r10
1412fa102: 488b7910 mov rdi, qword ptr [rcx + 0x10]
1412fa106: 483b39 cmp rdi, qword ptr [rcx]
1412fa109: 7523 jne 0x1412fa12e
1412fa10b: 48c744242050000000 mov qword ptr [rsp + 0x20], 0x50
1412fa114: 41b801000000 mov r8d, 1
1412fa11a: 41b908000000 mov r9d, 8
1412fa120: 4889cb mov rbx, rcx
1412fa123: 4889fa mov rdx, rdi
1412fa126: e835f95a01 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412fa12b: 4889d9 mov rcx, rbx
1412fa12e: 488b4108 mov rax, qword ptr [rcx + 8]
1412fa132: 488d14bf lea rdx, [rdi + rdi*4]
1412fa136: 48c1e204 shl rdx, 4
1412fa13a: 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
1412fa13e: 0f104df8 movups xmm1, xmmword ptr [rbp - 8]
1412fa142: 0f105508 movups xmm2, xmmword ptr [rbp + 8]
1412fa146: 0f105d18 movups xmm3, xmmword ptr [rbp + 0x18]
1412fa14a: 0f106528 movups xmm4, xmmword ptr [rbp + 0x28]
1412fa14e: 0f11641040 movups xmmword ptr [rax + rdx + 0x40], xmm4
1412fa153: 0f115c1030 movups xmmword ptr [rax + rdx + 0x30], xmm3
1412fa158: 0f11541020 movups xmmword ptr [rax + rdx + 0x20], xmm2
1412fa15d: 0f114c1010 movups xmmword ptr [rax + rdx + 0x10], xmm1
1412fa162: 0f110410 movups xmmword ptr [rax + rdx], xmm0
1412fa166: 48ffc7 inc rdi
1412fa169: 48897910 mov qword ptr [rcx + 0x10], rdi
1412fa16d: 4d39f4 cmp r12, r14
1412fa170: 4c8b4538 mov r8, qword ptr [rbp + 0x38]
1412fa174: 4d89f9 mov r9, r15
1412fa177: 4c89ea mov rdx, r13
1412fa17a: 0f85e9feffff jne 0x1412fa069
1412fa180: eb03 jmp 0x1412fa185
1412fa182: 488906 mov qword ptr [rsi], rax
1412fa185: 0f287550 movaps xmm6, xmmword ptr [rbp + 0x50]
1412fa189: 0f287d60 movaps xmm7, xmmword ptr [rbp + 0x60]
1412fa18d: 4881c4f8000000 add rsp, 0xf8
1412fa194: 5b pop rbx
1412fa195: 5f pop rdi
1412fa196: 5e pop rsi
1412fa197: 415c pop r12
1412fa199: 415d pop r13
1412fa19b: 415e pop r14
1412fa19d: 415f pop r15
1412fa19f: 5d pop rbp
1412fa1a0: c3 ret 
1412fa1a1: 0f0b ud2 
1412fa1a3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1412fa1b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412fa1b5: 55 push rbp
1412fa1b6: 4157 push r15
1412fa1b8: 4156 push r14
1412fa1ba: 4155 push r13
1412fa1bc: 4154 push r12
1412fa1be: 56 push rsi
1412fa1bf: 57 push rdi
1412fa1c0: 53 push rbx
1412fa1c1: 4883ec48 sub rsp, 0x48
1412fa1c5: 488daa80000000 lea rbp, [rdx + 0x80]
1412fa1cc: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
1412fa1d1: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
1412fa1d6: 488b4540 mov rax, qword ptr [rbp + 0x40]
1412fa1da: f048ff08 lock dec qword ptr [rax]
1412fa1de: 7509 jne 0x1412fa1e9
1412fa1e0: 488d4de8 lea rcx, [rbp - 0x18]
1412fa1e4: e8474935ff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
1412fa1e9: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
1412fa1ee: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
1412fa1f3: 4883c448 add rsp, 0x48
1412fa1f7: 5b pop rbx
1412fa1f8: 5f pop rdi
1412fa1f9: 5e pop rsi
1412fa1fa: 415c pop r12
1412fa1fc: 415d pop r13
1412fa1fe: 415e pop r14
1412fa200: 415f pop r15
1412fa202: 5d pop rbp
1412fa203: c3 ret 
