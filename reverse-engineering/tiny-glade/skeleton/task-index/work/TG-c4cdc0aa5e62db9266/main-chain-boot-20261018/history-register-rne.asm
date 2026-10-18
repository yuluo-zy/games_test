1409438a0: push rbp
1409438a1: push r14
1409438a3: push rsi
1409438a4: push rdi
1409438a5: push rbx
1409438a6: sub rsp, 0x420
1409438ad: lea rbp, [rsp + 0x80]
1409438b5: mov qword ptr [rbp + 0x398], 0xfffffffffffffffe
1409438c0: mov rsi, rcx
1409438c3: lea r8, [rip + 0x216f076] ; 0x142ab2940 1029ab42010000002e00000000000000
1409438ca: lea rcx, [rbp + 0x10]
1409438ce: mov dl, 0xe
1409438d0: call 0x141738630
1409438d5: lea rcx, [rbp + 0xf0]
1409438dc: call 0x141737fb0
1409438e1: mov byte ptr [rbp + 0x2b0], 0x23
1409438e8: mov rcx, qword ptr [rip + 0x2ce8a81] ; 0x14362c370 10c362430100000040c3624301000000
1409438ef: lea r8, [rip + 0x231c46a] ; 0x142c5fd60 00000000000000000100000000000000
1409438f6: lea rdx, [rbp + 0x2b0]
1409438fd: call 0x140b218a0
140943902: mov r8, rdx
140943905: lea rcx, [rbp + 0xf0]
14094390c: mov rdx, rax
14094390f: call 0x141453000
140943914: movups xmm0, xmmword ptr [rbp + 0x150]
14094391b: movaps xmmword ptr [rbp + 0x380], xmm0
140943922: movups xmm0, xmmword ptr [rbp + 0x140]
140943929: movaps xmmword ptr [rbp + 0x370], xmm0
140943930: movups xmm0, xmmword ptr [rbp + 0x130]
140943937: movaps xmmword ptr [rbp + 0x360], xmm0
14094393e: movups xmm0, xmmword ptr [rbp + 0xf0]
140943945: movups xmm1, xmmword ptr [rbp + 0x100]
14094394c: movups xmm2, xmmword ptr [rbp + 0x110]
140943953: movups xmm3, xmmword ptr [rbp + 0x120]
14094395a: movaps xmmword ptr [rbp + 0x350], xmm3
140943961: movaps xmmword ptr [rbp + 0x340], xmm2
140943968: movaps xmmword ptr [rbp + 0x330], xmm1
14094396f: movaps xmmword ptr [rbp + 0x320], xmm0
140943976: lea r8, [rip + 0x216efdb] ; 0x142ab2958 1029ab42010000002e00000000000000
14094397d: lea rcx, [rbp - 0x60]
140943981: mov dl, 0x28
140943983: call 0x140ae3740
140943988: lea rcx, [rbp + 0xf0]
14094398f: call 0x14098ee10
140943994: movups xmm0, xmmword ptr [rbp + 0x150]
14094399b: movaps xmmword ptr [rbp + 0x310], xmm0
1409439a2: movups xmm0, xmmword ptr [rbp + 0x140]
1409439a9: movaps xmmword ptr [rbp + 0x300], xmm0
1409439b0: movups xmm0, xmmword ptr [rbp + 0x130]
1409439b7: movaps xmmword ptr [rbp + 0x2f0], xmm0
1409439be: movups xmm0, xmmword ptr [rbp + 0xf0]
1409439c5: movups xmm1, xmmword ptr [rbp + 0x100]
1409439cc: movups xmm2, xmmword ptr [rbp + 0x110]
1409439d3: movups xmm3, xmmword ptr [rbp + 0x120]
1409439da: movaps xmmword ptr [rbp + 0x2e0], xmm3
1409439e1: movaps xmmword ptr [rbp + 0x2d0], xmm2
1409439e8: movaps xmmword ptr [rbp + 0x2c0], xmm1
1409439ef: movaps xmmword ptr [rbp + 0x2b0], xmm0
1409439f6: lea rcx, [rbp + 0x80]
1409439fd: lea rdx, [rbp + 0x2b0]
140943a04: call 0x141737ae0
140943a09: mov byte ptr [rbp + 0x2b0], 0x29
140943a10: mov rcx, qword ptr [rip + 0x2ce8959] ; 0x14362c370 10c362430100000040c3624301000000
140943a17: lea r8, [rip + 0x231c342] ; 0x142c5fd60 00000000000000000100000000000000
140943a1e: lea rdi, [rbp + 0x2b0]
140943a25: mov rdx, rdi
140943a28: call 0x140b218a0
140943a2d: mov r8, rdx
140943a30: lea rbx, [rbp + 0x80]
140943a37: mov rcx, rbx
140943a3a: mov rdx, rax
140943a3d: call 0x141453000
140943a42: movups xmm0, xmmword ptr [rbp + 0xe0]
140943a49: movaps xmmword ptr [rbp + 0x2a0], xmm0
140943a50: movups xmm0, xmmword ptr [rbp + 0xd0]
140943a57: movaps xmmword ptr [rbp + 0x290], xmm0
140943a5e: movups xmm0, xmmword ptr [rbp + 0xc0]
140943a65: movaps xmmword ptr [rbp + 0x280], xmm0
140943a6c: movups xmm0, xmmword ptr [rbp + 0x80]
140943a73: movups xmm1, xmmword ptr [rbp + 0x90]
140943a7a: movups xmm2, xmmword ptr [rbp + 0xa0]
140943a81: movups xmm3, xmmword ptr [rbp + 0xb0]
140943a88: movaps xmmword ptr [rbp + 0x270], xmm3
140943a8f: movaps xmmword ptr [rbp + 0x260], xmm2
140943a96: movaps xmmword ptr [rbp + 0x250], xmm1
140943a9d: movaps xmmword ptr [rbp + 0x240], xmm0
140943aa4: movups xmm0, xmmword ptr [rbp + 0x10]
140943aa8: movups xmm1, xmmword ptr [rbp + 0x20]
140943aac: movups xmm2, xmmword ptr [rbp + 0x30]
140943ab0: movups xmm3, xmmword ptr [rbp + 0x40]
140943ab4: movaps xmmword ptr [rbp + 0xf0], xmm0
140943abb: movaps xmmword ptr [rbp + 0x100], xmm1
140943ac2: movaps xmmword ptr [rbp + 0x110], xmm2
140943ac9: movaps xmmword ptr [rbp + 0x120], xmm3
140943ad0: movups xmm0, xmmword ptr [rbp + 0x50]
140943ad4: movaps xmmword ptr [rbp + 0x130], xmm0
140943adb: movups xmm0, xmmword ptr [rbp + 0x60]
140943adf: movaps xmmword ptr [rbp + 0x140], xmm0
140943ae6: movups xmm0, xmmword ptr [rbp + 0x70]
140943aea: movaps xmmword ptr [rbp + 0x150], xmm0
140943af1: movaps xmm0, xmmword ptr [rbp + 0x380]
140943af8: movaps xmmword ptr [rbp + 0x1c0], xmm0
140943aff: movaps xmm0, xmmword ptr [rbp + 0x370]
140943b06: movaps xmmword ptr [rbp + 0x1b0], xmm0
140943b0d: movaps xmm0, xmmword ptr [rbp + 0x360]
140943b14: movaps xmmword ptr [rbp + 0x1a0], xmm0
140943b1b: movaps xmm0, xmmword ptr [rbp + 0x320]
140943b22: movaps xmm1, xmmword ptr [rbp + 0x330]
140943b29: movaps xmm2, xmmword ptr [rbp + 0x340]
140943b30: movaps xmm3, xmmword ptr [rbp + 0x350]
140943b37: movaps xmmword ptr [rbp + 0x190], xmm3
140943b3e: movaps xmmword ptr [rbp + 0x180], xmm2
140943b45: movaps xmmword ptr [rbp + 0x170], xmm1
140943b4c: movaps xmmword ptr [rbp + 0x160], xmm0
140943b53: movups xmm0, xmmword ptr [rbp]
140943b57: movaps xmmword ptr [rbp + 0x230], xmm0
140943b5e: movups xmm0, xmmword ptr [rbp - 0x10]
140943b62: movaps xmmword ptr [rbp + 0x220], xmm0
140943b69: movups xmm0, xmmword ptr [rbp - 0x20]
140943b6d: movaps xmmword ptr [rbp + 0x210], xmm0
140943b74: movups xmm0, xmmword ptr [rbp - 0x60]
140943b78: movups xmm1, xmmword ptr [rbp - 0x50]
140943b7c: movups xmm2, xmmword ptr [rbp - 0x40]
140943b80: movups xmm3, xmmword ptr [rbp - 0x30]
140943b84: movaps xmmword ptr [rbp + 0x200], xmm3
140943b8b: movaps xmmword ptr [rbp + 0x1f0], xmm2
140943b92: movaps xmmword ptr [rbp + 0x1e0], xmm1
140943b99: movaps xmmword ptr [rbp + 0x1d0], xmm0
140943ba0: lea rdx, [rbp + 0xf0]
140943ba7: mov rcx, rsi
140943baa: call 0x140b669c0
140943baf: lea r14, [rsi + 0x38]
140943bb3: movabs rdx, 0x8e1bd251962ba784
140943bbd: movabs r8, 0x6a782374c0e6529b
140943bc7: mov rcx, r14
140943bca: call 0x1400dd6f0
140943bcf: test rax, rax
140943bd2: je 0x140943c99
140943bd8: cmp qword ptr [rsi + 0x218], rdx
140943bdf: jbe 0x140943c99
140943be5: mov rax, qword ptr [rsi + 0x210]
140943bec: mov rcx, qword ptr [rax + rdx*8]
140943bf0: test rcx, rcx
140943bf3: je 0x140943c99
140943bf9: mov rax, qword ptr [rsi + 0x1e0]
140943c00: not rcx
140943c03: lea rcx, [rcx + rcx*2]
140943c07: shl rcx, 5
140943c0b: cmp qword ptr [rax + rcx + 0x20], 0
140943c11: je 0x140943c99
140943c17: add rax, rcx
140943c1a: mov rcx, qword ptr [rax + 0x10]
140943c1e: mov edx, dword ptr [rsi + 0x5b0]
140943c24: mov dword ptr [rax + 0x5c], edx
140943c27: call 0x1416a0b70
140943c2c: movabs rdx, 0x2557ffe8a7986afe
140943c36: movabs r8, 0x4932233c9b0e0737
140943c40: mov rcx, r14
140943c43: call 0x1400dd6f0
140943c48: test rax, rax
140943c4b: je 0x140943c80
140943c4d: cmp qword ptr [rsi + 0x218], rdx
140943c54: jbe 0x140943c80
140943c56: mov rax, qword ptr [rsi + 0x210]
140943c5d: mov rax, qword ptr [rax + rdx*8]
140943c61: test rax, rax
140943c64: je 0x140943c80
140943c66: mov rcx, qword ptr [rsi + 0x1e0]
140943c6d: not rax
140943c70: lea rax, [rax + rax*2]
140943c74: shl rax, 5
140943c78: cmp qword ptr [rcx + rax + 0x20], 0
140943c7e: jne 0x140943c88
140943c80: mov rcx, rsi
140943c83: call 0x141513380
140943c88: mov rax, rsi
140943c8b: add rsp, 0x420
140943c92: pop rbx
140943c93: pop rdi
140943c94: pop rsi
140943c95: pop r14
140943c97: pop rbp
140943c98: ret 
140943c99: lea rax, [rip + 0x21af020] ; 0x142af2cc0 626576795f6563733a3a736368656475
140943ca0: mov qword ptr [rbp + 0x2b0], rax
140943ca7: mov qword ptr [rbp + 0x2b8], 0x27
140943cb2: mov qword ptr [rbp + 0x80], rdi
140943cb9: lea rax, [rip - 0x839e30] ; 0x140109e90 4989d0488b01488b51084889c1e94ee8
140943cc0: mov qword ptr [rbp + 0x88], rax
140943cc7: lea rax, [rip + 0x21af122] ; 0x142af2df0 e72caf42010000001300000000000000
140943cce: mov qword ptr [rbp + 0xf0], rax
140943cd5: mov qword ptr [rbp + 0xf8], 2
140943ce0: mov qword ptr [rbp + 0x110], 0
140943ceb: mov qword ptr [rbp + 0x100], rbx
140943cf2: mov qword ptr [rbp + 0x108], 1
140943cfd: lea rdx, [rip + 0x21aefa4] ; 0x142af2ca8 462caf42010000005c00000000000000
140943d04: lea rcx, [rbp + 0xf0]
140943d0b: call 0x1428d9390
140943d10: int3 
140943d11: nop word ptr cs:[rax + rax]
140943d20: mov qword ptr [rsp + 0x10], rdx
140943d25: push rbp
140943d26: push r14
140943d28: push rsi
140943d29: push rdi
140943d2a: push rbx
140943d2b: sub rsp, 0x20
140943d2f: lea rbp, [rdx + 0x80]
140943d36: lea rcx, [rbp + 0x10]
140943d3a: call 0x140029880
140943d3f: nop 
140943d40: add rsp, 0x20
140943d44: pop rbx
140943d45: pop rdi
140943d46: pop rsi
140943d47: pop r14
140943d49: pop rbp
140943d4a: ret 
140943d4b: nop dword ptr [rax + rax]
140943d50: mov qword ptr [rsp + 0x10], rdx
140943d55: push rbp
140943d56: push r14
140943d58: push rsi
140943d59: push rdi
140943d5a: push rbx
140943d5b: sub rsp, 0x20
140943d5f: lea rbp, [rdx + 0x80]
140943d66: lea rcx, [rbp + 0xf0]
140943d6d: call 0x140029880
140943d72: nop 
140943d73: add rsp, 0x20
140943d77: pop rbx
140943d78: pop rdi
140943d79: pop rsi
140943d7a: pop r14
140943d7c: pop rbp
140943d7d: ret 
140943d7e: nop 
140943d80: mov qword ptr [rsp + 0x10], rdx
140943d85: push rbp
140943d86: push r14
140943d88: push rsi
140943d89: push rdi
140943d8a: push rbx
140943d8b: sub rsp, 0x20
140943d8f: lea rbp, [rdx + 0x80]
140943d96: lea rcx, [rbp + 0x320]
140943d9d: call 0x140029880
140943da2: nop 
140943da3: add rsp, 0x20
140943da7: pop rbx
140943da8: pop rdi
140943da9: pop rsi
140943daa: pop r14
140943dac: pop rbp
140943dad: ret 
140943dae: nop 
140943db0: mov qword ptr [rsp + 0x10], rdx
140943db5: push rbp
140943db6: push r14
140943db8: push rsi
140943db9: push rdi
140943dba: push rbx
140943dbb: sub rsp, 0x20
140943dbf: lea rbp, [rdx + 0x80]
140943dc6: lea rcx, [rbp - 0x60]
140943dca: call 0x140029880
140943dcf: nop 
140943dd0: add rsp, 0x20
140943dd4: pop rbx
140943dd5: pop rdi
140943dd6: pop rsi
140943dd7: pop r14
140943dd9: pop rbp
140943dda: ret 
140943ddb: nop dword ptr [rax + rax]
140943de0: mov qword ptr [rsp + 0x10], rdx
140943de5: push rbp
140943de6: push r14
140943de8: push rsi
140943de9: push rdi
140943dea: push rbx
140943deb: sub rsp, 0x20
140943def: lea rbp, [rdx + 0x80]
140943df6: lea rcx, [rbp + 0x80]
140943dfd: call 0x140029880
140943e02: nop 
140943e03: add rsp, 0x20
140943e07: pop rbx
140943e08: pop rdi
140943e09: pop rsi
140943e0a: pop r14
140943e0c: pop rbp
140943e0d: ret 
