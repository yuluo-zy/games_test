0x140b3fe40 push rbp
0x140b3fe41 push r15
0x140b3fe43 push r14
0x140b3fe45 push rsi
0x140b3fe46 push rdi
0x140b3fe47 push rbx
0x140b3fe48 sub rsp, 0x268
0x140b3fe4f lea rbp, [rsp + 0x80]
0x140b3fe57 mov qword ptr [rbp + 0x1e0], 0xfffffffffffffffe
0x140b3fe62 mov r14, r8
0x140b3fe65 test r8, r8
0x140b3fe68 js 0x140b400d7
0x140b3fe6e mov rsi, r9
0x140b3fe71 je 0x140b3fea0
0x140b3fe73 mov rbx, rdx
0x140b3fe76 movzx eax, byte ptr [rip + 0x2af4d8c]
0x140b3fe7d mov edi, 1
0x140b3fe82 mov edx, 1
0x140b3fe87 mov rcx, r14
0x140b3fe8a call 0x140613c10
0x140b3fe8f test rax, rax
0x140b3fe92 je 0x140b400d9
0x140b3fe98 mov r15, rax
0x140b3fe9b mov rdx, rbx
0x140b3fe9e jmp 0x140b3fea6
0x140b3fea0 mov r15d, 1
0x140b3fea6 mov qword ptr [rbp + 0x1d8], r15
0x140b3fead mov rcx, r15
0x140b3feb0 mov r8, r14
0x140b3feb3 call 0x142923c00
0x140b3feb8 mov qword ptr [rbp + 0x150], r14
0x140b3febf mov qword ptr [rbp + 0x158], r15
0x140b3fec6 mov qword ptr [rbp + 0x160], r14
0x140b3fecd lea rcx, [rbp + 0x80]
0x140b3fed4 lea rdx, [rbp + 0x150]
0x140b3fedb mov qword ptr [rbp + 0x1d0], r14
0x140b3fee2 call 0x140d7bd90
0x140b3fee7 mov rax, qword ptr [rbp + 0x160]
0x140b3feee mov qword ptr [rbp + 0x140], rax
0x140b3fef5 movups xmm0, xmmword ptr [rbp + 0x150]
0x140b3fefc movaps xmmword ptr [rbp + 0x130], xmm0
0x140b3ff03 movups xmm0, xmmword ptr [rbp + 0x80]
0x140b3ff0a movaps xmmword ptr [rbp + 0x100], xmm0
0x140b3ff11 mov rax, qword ptr [rbp + 0x90]
0x140b3ff18 mov qword ptr [rbp + 0x110], rax
0x140b3ff1f mov qword ptr [rbp + 0x118], 0
0x140b3ff2a mov qword ptr [rbp + 0x120], 8
0x140b3ff35 mov qword ptr [rbp + 0x128], 0
0x140b3ff40 lea rcx, [rbp + 0x80]
0x140b3ff47 lea rdx, [rbp + 0x100]
0x140b3ff4e call 0x1409383b0
0x140b3ff53 cmp qword ptr [rbp + 0x80], 0
0x140b3ff5b je 0x140b400eb
0x140b3ff61 movups xmm0, xmmword ptr [rbp + 0xf0]
0x140b3ff68 movaps xmmword ptr [rbp + 0x1c0], xmm0
0x140b3ff6f movups xmm0, xmmword ptr [rbp + 0xe0]
0x140b3ff76 movaps xmmword ptr [rbp + 0x1b0], xmm0
0x140b3ff7d movups xmm0, xmmword ptr [rbp + 0xd0]
0x140b3ff84 movaps xmmword ptr [rbp + 0x1a0], xmm0
0x140b3ff8b movups xmm0, xmmword ptr [rbp + 0xc0]
0x140b3ff92 movaps xmmword ptr [rbp + 0x190], xmm0
0x140b3ff99 movups xmm0, xmmword ptr [rbp + 0x80]
0x140b3ffa0 movups xmm1, xmmword ptr [rbp + 0x90]
0x140b3ffa7 movups xmm2, xmmword ptr [rbp + 0xa0]
0x140b3ffae movups xmm3, xmmword ptr [rbp + 0xb0]
0x140b3ffb5 movaps xmmword ptr [rbp + 0x180], xmm3
0x140b3ffbc movaps xmmword ptr [rbp + 0x170], xmm2
0x140b3ffc3 movaps xmmword ptr [rbp + 0x160], xmm1
0x140b3ffca movaps xmmword ptr [rbp + 0x150], xmm0
0x140b3ffd1 mov rax, qword ptr [rbp + 0x100]
0x140b3ffd8 movups xmm0, xmmword ptr [rbp + 0x108]
0x140b3ffdf movups xmm1, xmmword ptr [rbp + 0x118]
0x140b3ffe6 movups xmm2, xmmword ptr [rbp + 0x128]
0x140b3ffed movups xmm3, xmmword ptr [rbp + 0x138]
0x140b3fff4 movaps xmmword ptr [rbp + 0xb0], xmm3
0x140b3fffb movaps xmmword ptr [rbp + 0xa0], xmm2
0x140b40002 movaps xmmword ptr [rbp + 0x90], xmm1
0x140b40009 movaps xmmword ptr [rbp + 0x80], xmm0
0x140b40010 mov rcx, rax
0x140b40013 neg rcx
0x140b40016 jo 0x140b40126
0x140b4001c mov rcx, qword ptr [rbp + 0x250]
0x140b40023 mov rdx, qword ptr [rbp + 0x240]
0x140b4002a mov qword ptr [rbp - 0x58], rax
0x140b4002e movaps xmm0, xmmword ptr [rbp + 0x80]
0x140b40035 movaps xmm1, xmmword ptr [rbp + 0x90]
0x140b4003c movaps xmm2, xmmword ptr [rbp + 0xa0]
0x140b40043 movaps xmm3, xmmword ptr [rbp + 0xb0]
0x140b4004a movups xmmword ptr [rbp - 0x50], xmm0
0x140b4004e movups xmmword ptr [rbp - 0x40], xmm1
0x140b40052 movups xmmword ptr [rbp - 0x30], xmm2
0x140b40056 movups xmmword ptr [rbp - 0x20], xmm3
0x140b4005a mov qword ptr [rbp - 0x10], rsi
0x140b4005e mov qword ptr [rbp - 8], rdx
0x140b40062 movaps xmm0, xmmword ptr [rbp + 0x150]
0x140b40069 movaps xmm1, xmmword ptr [rbp + 0x160]
0x140b40070 movaps xmm2, xmmword ptr [rbp + 0x170]
0x140b40077 movaps xmm3, xmmword ptr [rbp + 0x180]
0x140b4007e movups xmmword ptr [rbp], xmm0
0x140b40082 movups xmmword ptr [rbp + 0x10], xmm1
0x140b40086 movups xmmword ptr [rbp + 0x20], xmm2
0x140b4008a movups xmmword ptr [rbp + 0x30], xmm3
0x140b4008e movaps xmm0, xmmword ptr [rbp + 0x190]
0x140b40095 movups xmmword ptr [rbp + 0x40], xmm0
0x140b40099 movaps xmm0, xmmword ptr [rbp + 0x1a0]
0x140b400a0 movups xmmword ptr [rbp + 0x50], xmm0
0x140b400a4 movaps xmm0, xmmword ptr [rbp + 0x1b0]
0x140b400ab movups xmmword ptr [rbp + 0x60], xmm0
0x140b400af movaps xmm0, xmmword ptr [rbp + 0x1c0]
0x140b400b6 movups xmmword ptr [rbp + 0x70], xmm0
0x140b400ba lea rdx, [rbp - 0x58]
0x140b400be xor r8d, r8d
0x140b400c1 call 0x140936ef0
0x140b400c6 nop 
0x140b400c7 add rsp, 0x268
0x140b400ce pop rbx
0x140b400cf pop rdi
0x140b400d0 pop rsi
0x140b400d1 pop r14
0x140b400d3 pop r15
0x140b400d5 pop rbp
0x140b400d6 ret 
0x140b400d7 xor edi, edi
0x140b400d9 lea r8, [rip + 0x1f845a0] ; '0F�B\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00�\x00\x00\x00\x19\x00\x00\x00/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5'
0x140b400e0 mov rcx, rdi
0x140b400e3 mov rdx, r14
0x140b400e6 call 0x1428d8fe3
0x140b400eb mov rax, qword ptr [rbp + 0x88]
0x140b400f2 mov qword ptr [rbp + 0x150], rax
0x140b400f9 lea rax, [rip + 0x1f71190] ; "�\x11�B\x01\x00\x00\x00(\x00\x00\x00\x00\x00\x00\x00d\x00\x00\x007\x00\x00\x00�\x11�B\x01\x00\x00\x00(\x00\x00\x00\x00\x00\x00\x00�\x00\x00\x00\x16\x00\x00\x00Can't create secondary publisher"
0x140b40100 mov qword ptr [rsp + 0x20], rax
0x140b40105 lea rcx, [rip + 0x1f70af4] ; 'called `Result::unwrap()` on an `Err` value\x00\x00\x00\x00\x00��^@\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00q�@\x01\x00\x00\x00'
0x140b4010c lea r9, [rip + 0x1f70acd] ; '\x10f^@\x01\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00 ��@\x01\x00\x00\x00called `Result::unwrap()` on an `Err` value\x00\x00\x00\x00\x00'
0x140b40113 lea r8, [rbp + 0x150]
0x140b4011a mov edx, 0x2b
0x140b4011f call 0x1428d9760
0x140b40124 ud2 
0x140b40126 lea rcx, [rip + 0x1faee03] ; '\x08��B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00�\x00\x00\x00\x0e\x00\x00\x00\x08��B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00\x02\x01\x00\x00\x0e\x00\x00\x00struct DecoratorHistoryEditChang'